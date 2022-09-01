#![doc = include_str!("../README.md")]

use mithril_aggregator::{
    AggregatorConfig, AggregatorRunner, AggregatorRuntime, CertificatePendingStore,
    CertificateStore, Config, DependencyManager, GzipSnapshotter, MultiSignerImpl,
    ProtocolParametersStore, ProtocolParametersStorer, Server, SingleSignatureStore,
    VerificationKeyStore,
};
use mithril_common::certificate_chain::MithrilCertificateVerifier;
use mithril_common::chain_observer::{CardanoCliRunner, ChainObserver};
use mithril_common::crypto_helper::{key_decode_hex, ProtocolGenesisVerifier};
use mithril_common::digesters::{CardanoImmutableDigester, ImmutableFileSystemObserver};
use mithril_common::store::adapter::JsonFileStoreAdapter;
use mithril_common::store::StakeStore;
use mithril_common::BeaconProviderImpl;

use clap::Parser;
use config::{Map, Source, Value, ValueKind};
use mithril_common::entities::Epoch;
use slog::{Drain, Level, Logger};
use slog_scope::debug;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, fs};
use tokio::sync::RwLock;
use tokio::time::Duration;

/// Node args
#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// Server listening IP
    #[clap(long, default_value = "0.0.0.0")]
    server_ip: String,

    /// Server listening port
    #[clap(long, default_value_t = 8080)]
    server_port: u16,

    /// Verbosity level
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,

    /// Run mode
    #[clap(short, long, default_value = "dev")]
    run_mode: String,

    /// Directory to snapshot
    #[clap(long, default_value = "/db")]
    db_directory: PathBuf,

    /// Directory to store snapshot
    /// Defaults to work folder
    #[clap(long, default_value = ".")]
    snapshot_directory: PathBuf,
}

impl Args {
    fn log_level(&self) -> Level {
        match self.verbose {
            0 => Level::Warning,
            1 => Level::Info,
            2 => Level::Debug,
            _ => Level::Trace,
        }
    }

    fn build_logger(&self) -> Logger {
        let drain = slog_bunyan::new(std::io::stdout())
            .set_pretty(false)
            .build()
            .fuse();
        let drain = slog::LevelFilter::new(drain, self.log_level()).fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        Logger::root(Arc::new(drain), slog::o!())
    }
}

impl Source for Args {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, config::ConfigError> {
        let mut result = Map::new();
        let uri = "clap arguments".to_string();

        let server_url = format!("http://{}:{}/", &self.server_ip, &self.server_port);
        result.insert(
            "server_url".to_string(),
            Value::new(Some(&uri), ValueKind::from(server_url)),
        );
        result.insert(
            "db_directory".to_string(),
            Value::new(
                Some(&uri),
                ValueKind::from(self.db_directory.to_str().unwrap().to_string()),
            ),
        );
        result.insert(
            "snapshot_directory".to_string(),
            Value::new(
                Some(&uri),
                ValueKind::from(self.snapshot_directory.to_str().unwrap().to_string()),
            ),
        );

        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load args
    let args = Args::parse();
    let _guard = slog_scope::set_global_logger(args.build_logger());

    // Load config
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| args.run_mode.clone());
    let config: Config = config::Config::builder()
        .add_source(config::File::with_name(&format!("./config/{}.json", run_mode)).required(false))
        .add_source(config::Environment::default().separator("__"))
        .add_source(args.clone())
        .build()
        .map_err(|e| format!("configuration build error: {}", e))?
        .try_deserialize()
        .map_err(|e| format!("configuration deserialize error: {}", e))?;
    debug!("Started"; "run_mode" => &run_mode, "config" => format!("{:?}", config));

    // Init dependencies
    let snapshot_store = config.build_snapshot_store()?;

    let snapshot_uploader = config.build_snapshot_uploader();
    let certificate_pending_store = Arc::new(CertificatePendingStore::new(Box::new(
        JsonFileStoreAdapter::new(config.data_stores_directory.join("pending_cert_db"))?,
    )));
    let certificate_store = Arc::new(CertificateStore::new(Box::new(JsonFileStoreAdapter::new(
        config.data_stores_directory.join("cert_db"),
    )?)));
    let verification_key_store = Arc::new(VerificationKeyStore::new(Box::new(
        JsonFileStoreAdapter::new(config.data_stores_directory.join("verification_key_db"))?,
    )));
    let stake_store = Arc::new(StakeStore::new(Box::new(JsonFileStoreAdapter::new(
        config.data_stores_directory.join("stake_db"),
    )?)));
    let single_signature_store = Arc::new(SingleSignatureStore::new(Box::new(
        JsonFileStoreAdapter::new(config.data_stores_directory.join("single_signature_db"))?,
    )));
    let protocol_parameters_store = Arc::new(ProtocolParametersStore::new(Box::new(
        JsonFileStoreAdapter::new(config.data_stores_directory.join("protocol_parameters_db"))?,
    )));
    let multi_signer = Arc::new(RwLock::new(MultiSignerImpl::new(
        verification_key_store.clone(),
        stake_store.clone(),
        single_signature_store.clone(),
        protocol_parameters_store.clone(),
    )));
    let chain_observer = Arc::new(
        mithril_common::chain_observer::CardanoCliChainObserver::new(Box::new(
            CardanoCliRunner::new(
                config.cardano_cli_path.clone(),
                config.cardano_node_socket_path.clone(),
                config.get_network()?,
            ),
        )),
    );
    let immutable_file_observer = Arc::new(ImmutableFileSystemObserver::new(&config.db_directory));
    let beacon_provider = Arc::new(BeaconProviderImpl::new(
        chain_observer.clone(),
        immutable_file_observer.clone(),
        config.get_network()?,
    ));
    let digester = Arc::new(CardanoImmutableDigester::new(
        config.db_directory.clone(),
        slog_scope::logger(),
    ));
    let certificate_verifier = Arc::new(MithrilCertificateVerifier::new(slog_scope::logger()));
    let genesis_verification_key = key_decode_hex(&config.genesis_verification_key)?;
    let genesis_verifier = Arc::new(ProtocolGenesisVerifier::from_verification_key(
        genesis_verification_key,
    ));

    // Snapshotter - Ensure its ongoing snapshot directory exist
    let ongoing_snapshot_directory = config.snapshot_directory.join("pending_snapshot");
    if !ongoing_snapshot_directory.exists() {
        fs::create_dir(&ongoing_snapshot_directory)
            .expect("Pending snapshot directory creation failure");
    }
    let snapshotter = Arc::new(GzipSnapshotter::new(
        config.db_directory.clone(),
        ongoing_snapshot_directory,
    ));

    // Init dependency manager
    let dependency_manager = DependencyManager {
        config: config.clone(),
        snapshot_store: snapshot_store.clone(),
        snapshot_uploader: snapshot_uploader.clone(),
        multi_signer: multi_signer.clone(),
        certificate_pending_store: certificate_pending_store.clone(),
        certificate_store: certificate_store.clone(),
        verification_key_store: verification_key_store.clone(),
        stake_store: stake_store.clone(),
        single_signature_store: single_signature_store.clone(),
        protocol_parameters_store: protocol_parameters_store.clone(),
        chain_observer: chain_observer.clone(),
        beacon_provider: beacon_provider.clone(),
        immutable_file_observer,
        digester,
        snapshotter,
        certificate_verifier,
        genesis_verifier,
    };
    let dependency_manager = Arc::new(dependency_manager);
    let network = config.get_network()?;

    // todo: Genesis ?
    do_first_launch_initialization_if_needed(
        chain_observer,
        protocol_parameters_store,
        config.clone(),
    )
    .await?;

    // Start snapshot uploader
    let runtime_dependencies = dependency_manager.clone();
    let handle = tokio::spawn(async move {
        let config =
            AggregatorConfig::new(config.run_interval, network, &config.db_directory.clone());
        let mut runtime = AggregatorRuntime::new(
            Duration::from_millis(config.interval),
            None,
            Arc::new(AggregatorRunner::new(config, runtime_dependencies.clone())),
        )
        .await
        .unwrap();
        runtime.run().await
    });

    // Start REST server
    println!("Starting server...");
    println!("Press Ctrl+C to stop...");
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };
    let http_server = Server::new(args.server_ip, args.server_port, dependency_manager.clone());
    http_server.start(shutdown_signal).await;

    handle.abort();

    println!("Exiting...");
    Ok(())
}

async fn do_first_launch_initialization_if_needed(
    chain_observer: Arc<dyn ChainObserver>,
    protocol_parameters_store: Arc<ProtocolParametersStore>,
    config: Config,
) -> Result<(), Box<dyn Error>> {
    // TODO: Remove that when we hande genesis certificate
    let (work_epoch, epoch_to_sign) = match chain_observer
        .get_current_epoch()
        .await?
        .ok_or("Can't retrieve current epoch")?
    {
        Epoch(0) => (Epoch(0), Epoch(1)),
        epoch => (
            epoch.offset_to_signer_retrieval_epoch()?,
            epoch.offset_to_next_signer_retrieval_epoch()?,
        ),
    };

    if protocol_parameters_store
        .get_protocol_parameters(work_epoch)
        .await?
        .is_none()
    {
        debug!("First launch, will use the configured protocol parameters for the current and next epoch certificate");

        for epoch in [work_epoch, epoch_to_sign] {
            protocol_parameters_store
                .save_protocol_parameters(epoch, config.protocol_parameters.clone())
                .await?;
        }
    }

    Ok(())
}
