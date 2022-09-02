use std::error::Error as StdError;
use std::fs;
use std::sync::Arc;

use mithril_common::{
    chain_observer::{CardanoCliChainObserver, CardanoCliRunner, ChainObserver},
    digesters::{CardanoImmutableDigester, ImmutableDigester, ImmutableFileSystemObserver},
    store::{adapter::SQLiteAdapter, StakeStore},
    BeaconProvider, BeaconProviderImpl,
};

use crate::{
    certificate_handler::CertificateHandler, single_signer::SingleSigner,
    CertificateHandlerHTTPClient, Config, MithrilSingleSigner, ProtocolInitializerStore,
    ProtocolInitializerStorer,
};

type StakeStoreService = Arc<StakeStore>;
type CertificateHandlerService = Arc<dyn CertificateHandler>;
type ChainObserverService = Arc<dyn ChainObserver>;
type DigesterService = Arc<dyn ImmutableDigester>;
type SingleSignerService = Arc<dyn SingleSigner>;
type BeaconProviderService = Arc<dyn BeaconProvider>;
type ProtocolInitializerStoreService = Arc<dyn ProtocolInitializerStorer>;

/// The ServiceBuilder is intended to manage Services instance creation.
/// The goal of this is to put all this code out of the way of business code.
pub trait ServiceBuilder {
    /// Create a SignerService instance.
    fn build(&self) -> Result<SignerServices, Box<dyn StdError>>;
}

/// Create a SignerService instance for Production environment.
pub struct ProductionServiceBuilder<'a> {
    config: &'a Config,
}

impl<'a> ProductionServiceBuilder<'a> {
    /// Create a new production service builder.
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

impl<'a> ServiceBuilder for ProductionServiceBuilder<'a> {
    /// Build a Services for the Production environment.
    fn build(&self) -> Result<SignerServices, Box<dyn StdError>> {
        if !self.config.data_stores_directory.exists() {
            fs::create_dir_all(self.config.data_stores_directory.clone())
                .map_err(|e| format!("Could not create data stores directory: {:?}", e))?;
        }

        let sqlite_db_path = Some(self.config.data_stores_directory.join("signer.sqlite3"));
        let protocol_initializer_store = Arc::new(ProtocolInitializerStore::new(Box::new(
            SQLiteAdapter::new("protocol_initializer", sqlite_db_path.clone())?,
        )));
        let single_signer = Arc::new(MithrilSingleSigner::new(self.config.party_id.clone()));
        let certificate_handler = Arc::new(CertificateHandlerHTTPClient::new(
            self.config.aggregator_endpoint.clone(),
        ));
        let digester = Arc::new(CardanoImmutableDigester::new(
            self.config.db_directory.clone(),
            slog_scope::logger(),
        ));
        let stake_store = Arc::new(StakeStore::new(Box::new(SQLiteAdapter::new(
            "stake",
            sqlite_db_path,
        )?)));
        let chain_observer = Arc::new(CardanoCliChainObserver::new(Box::new(
            CardanoCliRunner::new(
                self.config.cardano_cli_path.clone(),
                self.config.cardano_node_socket_path.clone(),
                self.config.get_network()?,
            ),
        )));
        let beacon_provider = Arc::new(BeaconProviderImpl::new(
            chain_observer.clone(),
            Arc::new(ImmutableFileSystemObserver::new(&self.config.db_directory)),
            self.config.get_network()?.to_owned(),
        ));

        let services = SignerServices {
            beacon_provider,
            certificate_handler,
            chain_observer,
            digester,
            single_signer,
            stake_store,
            protocol_initializer_store,
        };

        Ok(services)
    }
}

/// This structure groups all the services required by the state machine.
pub struct SignerServices {
    /// Beacon provider service
    pub beacon_provider: BeaconProviderService,

    /// Stake store service
    pub stake_store: StakeStoreService,

    /// Certificate handler service
    pub certificate_handler: CertificateHandlerService,

    /// Chain Observer service
    pub chain_observer: ChainObserverService,

    /// Digester service
    pub digester: DigesterService,

    /// SingleSigner service
    pub single_signer: SingleSignerService,

    /// ProtocolInitializer store
    pub protocol_initializer_store: ProtocolInitializerStoreService,
}
