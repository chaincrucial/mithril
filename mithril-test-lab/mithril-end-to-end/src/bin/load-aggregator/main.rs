use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::Context;
use async_recursion::async_recursion;
use clap::Parser;

use indicatif::{ProgressBar, ProgressDrawTarget};
use mithril_common::{
    digesters::{
        CardanoImmutableDigester, DummyImmutableDb, DummyImmutablesDbBuilder, ImmutableDigester,
    },
    entities::{
        Beacon, Epoch, PartyId, ProtocolMessage, ProtocolMessagePartKey, ProtocolParameters,
        SignedEntityType, Signer, SingleSignatures,
    },
    messages::{
        CertificateListItemMessage, EpochSettingsMessage, MithrilStakeDistributionListItemMessage,
        RegisterSignatureMessage, RegisterSignerMessage, SnapshotListItemMessage,
    },
    test_utils::{MithrilFixture, MithrilFixtureBuilder},
    StdResult,
};

use mithril_end_to_end::{Aggregator, BftNode};
use reqwest::StatusCode;
use serde::Deserialize;
use slog::Level;
use slog_scope::{debug, info, warn};
use thiserror::Error;
use tokio::{
    select,
    task::JoinSet,
    time::{sleep, Instant},
};

macro_rules! spin_while_waiting {
    ($block:block, $timeout:expr, $wait_message:expr, $timeout_message:expr) => {{
        info!("⇄ {}", $wait_message);
        let progress_bar = ProgressBar::new_spinner().with_message($wait_message);

        let spinner = async move {
            loop {
                progress_bar.tick();
                sleep(Duration::from_millis(50)).await;
            }
        };
        let probe = async move { $block };

        select! {
        _ = spinner => Err(String::new().into()),
        _ = sleep($timeout) => Err($timeout_message.into()),
        res = probe => res
        }
    }};
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Registering signer party_id={party_id}, expected HTTP code {expected_http_code} got {got_http_code} with the message: {error_message}.")]
    SignerRegistrationError {
        party_id: PartyId,
        expected_http_code: u32,
        got_http_code: u32,
        error_message: String,
    },
    #[error("Registering signatures for party_id={party_id}, expected HTTP code {expected_http_code} got {got_http_code} with the message: {error_message}.")]
    SignaturesRegistrationError {
        party_id: PartyId,
        expected_http_code: u32,
        got_http_code: u32,
        error_message: String,
    },
}

fn init_logger(opts: &MainOpts) -> slog_scope::GlobalLoggerGuard {
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = slog::LevelFilter::new(drain, opts.log_level()).fuse();

    slog_scope::set_global_logger(slog::Logger::root(Arc::new(drain), slog::o!()))
}

/// Generate signer data
pub fn generate_signer_data(
    number_of_signers: usize,
    protocol_parameters: ProtocolParameters,
) -> MithrilFixture {
    MithrilFixtureBuilder::default()
        .with_signers(number_of_signers)
        .with_protocol_parameters(protocol_parameters)
        .build()
}

/// Generate signer registration message
pub fn generate_register_signer_message(
    signers: &[Signer],
    epoch: Epoch,
) -> Vec<RegisterSignerMessage> {
    signers
        .iter()
        .cloned()
        .map(|signer| RegisterSignerMessage {
            epoch: Some(epoch),
            party_id: signer.party_id,
            verification_key: signer.verification_key.to_json_hex().unwrap(),
            verification_key_signature: signer.verification_key_signature,
            operational_certificate: signer.operational_certificate,
            kes_period: signer.kes_period,
        })
        .collect::<Vec<_>>()
}

/// Generate register signature message
pub fn generate_register_signature_message(
    signatures: &[SingleSignatures],
    signed_entity_type: SignedEntityType,
) -> Vec<RegisterSignatureMessage> {
    signatures
        .iter()
        .map(|s| RegisterSignatureMessage {
            signed_entity_type: Some(signed_entity_type.clone()),
            party_id: s.party_id.clone(),
            signature: s.signature.clone().to_json_hex().unwrap(),
            won_indexes: s.won_indexes.clone(),
        })
        .collect::<Vec<_>>()
}

/// Wait for http response until timeout
pub async fn wait_for_http_response(url: &str, timeout: Duration, message: &str) -> StdResult<()> {
    spin_while_waiting!(
        {
            while reqwest::get(url).await.is_err() {
                sleep(Duration::from_millis(300)).await;
            }
            Ok(())
        },
        timeout,
        message.to_owned(),
        format!("Aggregator did not get a response after {timeout:?} from '{url}'")
    )
}

/// Wait for a given epoch in the epoch settings until timeout
pub async fn wait_for_epoch_settings_at_epoch(
    aggregator: &Aggregator,
    timeout: Duration,
    epoch: Epoch,
) -> StdResult<()> {
    let url = &format!("{}/epoch-settings", aggregator.endpoint());
    spin_while_waiting!(
        {
            while let Ok(response) = reqwest::get(url).await {
                match response.status() {
                    StatusCode::OK => {
                        let epoch_settings = response.json::<EpochSettingsMessage>().await.unwrap();

                        if epoch_settings.epoch >= epoch {
                            break;
                        }
                        sleep(Duration::from_millis(300)).await
                    }
                    s if s.is_server_error() => {
                        warn!(
                            "Server error while waiting for the Aggregator, http code: {}",
                            s
                        );
                        break;
                    }
                    _ => sleep(Duration::from_millis(300)).await,
                }
            }
            Ok(())
        },
        timeout,
        format!("Waiting for epoch {epoch}"),
        format!("Aggregator did not get a response after {timeout:?} from '{url}'")
    )
}

/// Wait for pending certificate
pub async fn wait_for_pending_certificate(
    aggregator: &Aggregator,
    timeout: Duration,
) -> StdResult<()> {
    let url = &format!("{}/certificate-pending", aggregator.endpoint());
    spin_while_waiting!(
        {
            while let Ok(response) = reqwest::get(url).await {
                match response.status() {
                    StatusCode::OK => {
                        break;
                    }
                    s if s.is_server_error() => {
                        warn!(
                            "Server error while waiting for the Aggregator, http code: {}",
                            s
                        );
                        break;
                    }
                    _ => sleep(Duration::from_millis(300)).await,
                }
            }
            Ok(())
        },
        timeout,
        format!("Waiting for pending certificate"),
        format!("Aggregator did not get a response after {timeout:?} from '{url}'")
    )
}

#[async_recursion]
async fn request_first_list_item_with_expected_size<I>(
    url: &str,
    expected_size: usize,
) -> Result<I, String>
where
    for<'a> I: Deserialize<'a> + Sync + Send + Clone,
{
    sleep(Duration::from_millis(300)).await;

    match reqwest::get(url).await {
        Ok(response) => match response.status() {
            StatusCode::OK => match response.json::<Vec<I>>().await.as_deref() {
                Ok(list) if list.len() == expected_size => Ok(list.first().unwrap().clone()),
                Ok(list) if list.len() > expected_size => Err(format!(
                    "Invalid size, expected {expected_size}, got {}",
                    list.len()
                )),
                Ok(_) => request_first_list_item_with_expected_size::<I>(url, expected_size).await,
                Err(err) => Err(format!("Invalid list body : {err}")),
            },
            s if s.is_server_error() => {
                let message = format!(
                    "Server error while waiting for the Aggregator, http code: {}",
                    s
                );
                warn!("{message}");
                Err(message)
            }
            _ => request_first_list_item_with_expected_size::<I>(url, expected_size).await,
        },
        Err(err) => Err(format!("Request to `{url}` failed: {err}")),
    }
}

/// Precompute all signers single signatures for the given fixture
pub async fn precompute_mithril_stake_distribution_signatures(
    signers_fixture: &MithrilFixture,
    timeout: Duration,
) -> StdResult<Vec<SingleSignatures>> {
    spin_while_waiting!(
        {
            let signers_fixture = signers_fixture.clone();
            let signatures = tokio::task::spawn_blocking(move || -> Vec<SingleSignatures> {
                let mithril_stake_distribution_message = {
                    let mut message = ProtocolMessage::new();
                    message.set_message_part(
                    mithril_common::entities::ProtocolMessagePartKey::NextAggregateVerificationKey,
                    signers_fixture.compute_and_encode_avk(),
                );

                    message
                };

                signers_fixture.sign_all(&mithril_stake_distribution_message)
            })
            .await?;

            Ok(signatures)
        },
        timeout,
        format!("Precompute signatures for MithrilStakeDistribution signed entity"),
        format!("Precomputing signatures timeout after {timeout:?}")
    )
}

/// Compute all signers single signatures for the given fixture
pub async fn compute_immutable_files_signatures(
    immutable_db: &DummyImmutableDb,
    epoch: Epoch,
    signers_fixture: &MithrilFixture,
    timeout: Duration,
) -> StdResult<(Beacon, Vec<SingleSignatures>)> {
    spin_while_waiting!(
        {
            let beacon = Beacon::new(
                "devnet".to_string(),
                *epoch,
                // Minus one because the last immutable isn't "finished"
                immutable_db.last_immutable_number().unwrap() - 1,
            );
            let digester = CardanoImmutableDigester::new(None, slog_scope::logger());
            let digest = digester.compute_digest(&immutable_db.dir, &beacon).await?;
            let signers_fixture = signers_fixture.clone();

            let signatures = tokio::task::spawn_blocking(move || -> Vec<SingleSignatures> {
                let mithril_stake_distribution_message = {
                    let mut message = ProtocolMessage::new();
                    message.set_message_part(ProtocolMessagePartKey::SnapshotDigest, digest);
                    message.set_message_part(
                        ProtocolMessagePartKey::NextAggregateVerificationKey,
                        signers_fixture.compute_and_encode_avk(),
                    );

                    message
                };

                signers_fixture.sign_all(&mithril_stake_distribution_message)
            })
            .await?;

            Ok((beacon, signatures))
        },
        timeout,
        format!("Precompute signatures for CardanoImmutableFiles signed entity"),
        format!("Precomputing signatures timeout after {timeout:?}")
    )
}

/// Wait for the given number of certificates, return the latest certificate
pub async fn wait_for_certificates(
    aggregator: &Aggregator,
    total: usize,
    timeout: Duration,
) -> StdResult<CertificateListItemMessage> {
    let url = &format!("{}/certificates", aggregator.endpoint());
    spin_while_waiting!(
        {
            request_first_list_item_with_expected_size::<CertificateListItemMessage>(url, total)
                .await
                .map_err(|e| e.into())
        },
        timeout,
        format!("Waiting for certificates"),
        format!("Aggregator did not get a response after {timeout:?} from '{url}'")
    )
}

/// Wait for Mithril Stake Distribution artifacts
pub async fn wait_for_mithril_stake_distribution_artifacts(
    aggregator: &Aggregator,
    timeout: Duration,
) -> StdResult<MithrilStakeDistributionListItemMessage> {
    let url = &format!(
        "{}/artifact/mithril-stake-distributions",
        aggregator.endpoint()
    );
    spin_while_waiting!(
        {
            request_first_list_item_with_expected_size::<MithrilStakeDistributionListItemMessage>(
                url, 1,
            )
            .await
            .map_err(|e| e.into())
        },
        timeout,
        format!("Waiting for mithril stake distribution artifacts"),
        format!("Aggregator did not get a response after {timeout:?} from '{url}'")
    )
}

/// Wait for Cardano Immutable Files artifacts
pub async fn wait_for_immutable_files_artifacts(
    aggregator: &Aggregator,
    timeout: Duration,
) -> StdResult<SnapshotListItemMessage> {
    let url = &format!("{}/artifact/snapshots", aggregator.endpoint());
    spin_while_waiting!(
        {
            request_first_list_item_with_expected_size::<SnapshotListItemMessage>(url, 1)
                .await
                .map_err(|e| e.into())
        },
        timeout,
        format!("Waiting for immutable files artifacts"),
        format!("Aggregator did not get a response after {timeout:?} from '{url}'")
    )
}

pub async fn register_signers_to_aggregator(
    aggregator: &Aggregator,
    signers: &[Signer],
    epoch: Epoch,
) -> StdResult<usize> {
    let register_messages = generate_register_signer_message(signers, epoch);

    let mut join_set: JoinSet<StdResult<()>> = JoinSet::new();
    let progress_bar = ProgressBar::with_draw_target(
        Some(register_messages.len() as u64),
        ProgressDrawTarget::stdout(),
    );

    let http_client = reqwest::Client::new();

    for register in register_messages {
        let endpoint = aggregator.endpoint();
        let http_request = http_client
            .post(format!("{}/register-signer", endpoint))
            .json(&register);

        join_set.spawn(async move {
            let response = http_request.send().await.unwrap();

            match response.status() {
                StatusCode::CREATED => Ok(()),
                status => Err(LoadError::SignerRegistrationError {
                    expected_http_code: 201,
                    got_http_code: status.as_u16() as u32,
                    party_id: register.party_id,
                    error_message: response.text().await.unwrap(),
                }
                .into()),
            }
        });
    }
    let mut errors = 0;

    while let Some(res) = join_set.join_next().await {
        let res = res.expect("Tokio task join failed!");
        progress_bar.inc(1);

        if res.is_err() {
            warn!("Signer Registration error caught: {res:?}");
            errors += 1;
        }
    }

    Ok(errors)
}

pub async fn register_signatures_to_aggregator(
    aggregator: &Aggregator,
    signatures: &[SingleSignatures],
    signed_entity_type: SignedEntityType,
) -> StdResult<usize> {
    let register_messages = generate_register_signature_message(signatures, signed_entity_type);

    let mut join_set: JoinSet<StdResult<()>> = JoinSet::new();
    let progress_bar = ProgressBar::with_draw_target(
        Some(register_messages.len() as u64),
        ProgressDrawTarget::stdout(),
    );

    let http_client = reqwest::Client::new();

    for register in register_messages {
        let endpoint = aggregator.endpoint();
        let http_request = http_client
            .post(format!("{}/register-signatures", endpoint))
            .json(&register);

        join_set.spawn(async move {
            let response = http_request.send().await.unwrap();

            match response.status() {
                StatusCode::CREATED => Ok(()),
                status => Err(LoadError::SignaturesRegistrationError {
                    expected_http_code: 201,
                    got_http_code: status.as_u16() as u32,
                    party_id: register.party_id,
                    error_message: response.text().await.unwrap(),
                }
                .into()),
            }
        });
    }
    let mut errors = 0;

    while let Some(res) = join_set.join_next().await {
        let res = res.expect("Tokio task join failed!");
        progress_bar.inc(1);

        if res.is_err() {
            warn!("Signer Signature Registration error caught: {res:?}");
            errors += 1;
        }
    }

    Ok(errors)
}

pub fn write_stake_distribution(
    mock_stake_distribution_file_path: &Path,
    signers_fixture: &MithrilFixture,
) {
    let mock_stake_distribution_file = File::create(mock_stake_distribution_file_path).unwrap();
    serde_json::to_writer(
        &mock_stake_distribution_file,
        &signers_fixture.cardano_cli_stake_distribution(),
    )
    .expect("Writing the stake distribution into a file for the mock cardano cli failed");
}

pub fn write_epoch(mock_epoch_file_path: &Path, epoch: Epoch) {
    let mock_epoch_file = File::create(mock_epoch_file_path).unwrap();
    write!(&mock_epoch_file, "{}", *epoch)
        .expect("Writing the epoch into a file for the mock cardano cli failed");
    debug!("New Epoch: {epoch}");
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct MainOpts {
    /// Location of the Cardano CLI binary
    #[arg(short, long)]
    pub cardano_cli_path: PathBuf,

    /// Temporary location for logs, databases etc.
    #[arg(short, long)]
    pub temporary_path: Option<PathBuf>,

    /// Path of the Aggregator binary
    #[arg(short, long, default_value = "./target/debug")]
    pub aggregator_dir: PathBuf,

    /// Number of concurrent signers
    #[arg(long, default_value = "20")]
    pub num_signers: usize,

    /// Mithril technical Era
    #[arg(long, default_value = "thales")]
    pub mithril_era: String,

    /// Aggregator HTTP port
    #[arg(short = 'p', long, default_value = "8080")]
    server_port: u32,

    /// Log level
    #[arg(short='v', action = clap::ArgAction::Count)]
    verbose: u8,
}

impl MainOpts {
    /// get log level from parameters
    pub fn log_level(&self) -> Level {
        match self.verbose {
            0 => Level::Warning,
            1 => Level::Info,
            2 => Level::Debug,
            _ => Level::Trace,
        }
    }
}

#[derive(Debug)]
pub struct AggregatorParameters {
    server_port: u32,
    bft_node: BftNode,
    cardano_cli_path: PathBuf,
    work_dir: PathBuf,
    bin_dir: PathBuf,
    mithril_era: String,
}

impl AggregatorParameters {
    fn new(opts: &MainOpts, immutable_db_path: &Path) -> StdResult<Self> {
        let bft_node = BftNode {
            db_path: immutable_db_path.to_path_buf(),
            socket_path: PathBuf::new(),
        };
        let tmp_dir = opts
            .temporary_path
            .as_ref()
            .cloned()
            .unwrap_or_else(|| std::env::temp_dir().join("load-aggregator"));

        if tmp_dir.exists() {
            std::fs::remove_dir_all(&tmp_dir).with_context(|| {
                format!(
                    "Could not remove existing temp directory '{}'.",
                    tmp_dir.display()
                )
            })?;
        }
        std::fs::create_dir_all(&tmp_dir)
            .with_context(|| format!("Could not create temp directory '{}'.", tmp_dir.display()))?;

        let cardano_cli_path = {
            if !opts.cardano_cli_path.exists() {
                Err(format!(
                    "Given cardano-cli path does not exist: {}",
                    opts.cardano_cli_path.display()
                ))?
            }

            opts.cardano_cli_path.canonicalize().with_context(|| {
                format!(
                    "Could not canonicalize path to the cardano-cli, path: {}",
                    opts.cardano_cli_path.display()
                )
            })?
        };

        let aggregator_parameters = AggregatorParameters {
            bft_node,
            bin_dir: opts.aggregator_dir.clone(),
            cardano_cli_path,
            server_port: opts.server_port,
            work_dir: tmp_dir,
            mithril_era: opts.mithril_era.clone(),
        };

        Ok(aggregator_parameters)
    }

    fn mock_stake_distribution_file_path(&self) -> PathBuf {
        self.work_dir.join("stake_distribution.json")
    }

    fn mock_epoch_file_path(&self) -> PathBuf {
        self.work_dir.join("epoch.txt")
    }
}

/// Bootstrap an aggregator and made it compute its genesis certificate
async fn bootstrap_aggregator(
    args: &AggregatorParameters,
    signers_fixture: &MithrilFixture,
    current_epoch: &mut Epoch,
) -> StdResult<Aggregator> {
    info!(">> Launch Aggregator");
    let mut aggregator = Aggregator::new(
        args.server_port as u64,
        &args.bft_node,
        &args.cardano_cli_path,
        &args.work_dir,
        &args.bin_dir,
        &args.mithril_era,
    )
    .unwrap();

    write_epoch(&args.mock_epoch_file_path(), *current_epoch);
    write_stake_distribution(&args.mock_stake_distribution_file_path(), signers_fixture);

    // Extremely large interval since for the two following start only the http_server part
    // of the aggregator is relevant since we need to send signer registrations.
    aggregator.change_run_interval(Duration::from_secs(20000));
    aggregator.set_mock_cardano_cli_file_path(
        &args.mock_stake_distribution_file_path(),
        &args.mock_epoch_file_path(),
    );
    aggregator.set_protocol_parameters(&signers_fixture.protocol_parameters());

    info!(
        ">> Starting the aggregator with a large run interval to call the http_server\
    without being bothered by the state machine cycles"
    );
    aggregator.serve().unwrap();
    wait_for_http_response(
        &format!("{}/epoch-settings", aggregator.endpoint()),
        Duration::from_secs(10),
        "Waiting for the aggregator to start",
    )
    .await?;

    info!(">> Send the Signer Key Registrations payloads for the genesis signers");
    let errors =
        register_signers_to_aggregator(&aggregator, &signers_fixture.signers(), *current_epoch + 1)
            .await?;
    assert_eq!(0, errors);
    aggregator.stop().await.unwrap();

    info!(">> Move one epoch forward in order to issue the genesis certificate");
    *current_epoch += 1;
    write_epoch(&args.mock_epoch_file_path(), *current_epoch);

    info!(">> Restarting the aggregator still with a large run interval");
    aggregator.serve().unwrap();
    wait_for_http_response(
        &format!("{}/epoch-settings", aggregator.endpoint()),
        Duration::from_secs(10),
        "Waiting for the aggregator to start",
    )
    .await?;

    info!(">> Send the Signer Key Registrations payloads for next genesis signers");
    let errors =
        register_signers_to_aggregator(&aggregator, &signers_fixture.signers(), *current_epoch + 1)
            .await?;
    assert_eq!(0, errors);
    aggregator.stop().await.unwrap();

    {
        info!(">> Compute genesis certificate");
        let mut genesis_aggregator = Aggregator::copy_configuration(&aggregator);
        genesis_aggregator
            .bootstrap_genesis()
            .await
            .expect("Genesis aggregator should be able to bootstrap genesis");
    }

    info!(">> Restart aggregator with a normal run interval");
    aggregator.change_run_interval(Duration::from_secs(3));
    aggregator.serve().unwrap();

    wait_for_http_response(
        &format!("{}/epoch-settings", aggregator.endpoint()),
        Duration::from_secs(10),
        "Waiting for the aggregator to restart",
    )
    .await?;

    info!(">> Aggregator bootrapped");

    Ok(aggregator)
}

struct Timing {
    phase: String,
    duration: Duration,
}

struct Reporter {
    number_of_signers: usize,
    timings: Vec<Timing>,
    current_timing: Option<(String, Instant)>,
}

impl Reporter {
    fn new(number_of_signers: usize) -> Self {
        Self {
            number_of_signers,
            timings: vec![],
            current_timing: None,
        }
    }

    fn start(&mut self, phase: &str) {
        self.current_timing = Some((phase.to_owned(), Instant::now()));
    }

    fn stop(&mut self) {
        match &self.current_timing {
            Some((phase, instant)) => {
                let timing = Timing {
                    phase: phase.clone(),
                    duration: instant.elapsed(),
                };

                self.timings.push(timing);
                self.current_timing = None;
            }
            None => (),
        }
    }

    fn print_report(&self) {
        println!("number_of_signers\t{}", self.number_of_signers);
        println!("phase\tduration/ms");
        for t in &self.timings {
            println!("{}\t{}", t.phase, t.duration.as_millis());
        }
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> StdResult<()> {
    let opts = MainOpts::parse();
    let mut reporter: Reporter = Reporter::new(opts.num_signers);
    reporter.start("stress tests bootstrap");
    // configure a dummy immutable db
    let mut immutable_db = DummyImmutablesDbBuilder::new("load-tester")
        .with_immutables(&[1, 2, 3])
        .append_immutable_trio()
        .build();

    let _logger_guard = init_logger(&opts);
    let args = AggregatorParameters::new(&opts, &immutable_db.dir)?;
    let mut current_epoch = Epoch(1);
    let protocol_parameters = ProtocolParameters::new(2422, 20973, 0.20);
    info!(">> Starting stress test with options: {opts:?}");

    reporter.start("stress bootstrap");
    info!(">> Creation of the Signer Key Registrations payloads");
    let signers_fixture = generate_signer_data(opts.num_signers, protocol_parameters);

    let mithril_stake_distribution_signatures = precompute_mithril_stake_distribution_signatures(
        &signers_fixture,
        Duration::from_secs(180),
    )
    .await?;

    let mut aggregator = bootstrap_aggregator(&args, &signers_fixture, &mut current_epoch).await?;
    reporter.stop();

    info!(">> Move one epoch forward in order to start creating certificates");
    current_epoch += 1;
    write_epoch(&args.mock_epoch_file_path(), current_epoch);
    wait_for_epoch_settings_at_epoch(&aggregator, Duration::from_secs(10), current_epoch).await?;

    info!(">> Send the Signer Key Registrations payloads");
    reporter.start("signers registration");
    let errors =
        register_signers_to_aggregator(&aggregator, &signers_fixture.signers(), current_epoch + 1)
            .await?;
    reporter.stop();
    assert_eq!(0, errors);

    info!(">> Wait for pending certificate to be available");
    wait_for_pending_certificate(&aggregator, Duration::from_secs(30)).await?;

    info!(
        ">> Send the Signer Signatures payloads for MithrilStakeDistribution({:?})",
        current_epoch
    );
    reporter.start("signatures registration");
    let errors = register_signatures_to_aggregator(
        &aggregator,
        &mithril_stake_distribution_signatures,
        SignedEntityType::MithrilStakeDistribution(current_epoch),
    )
    .await?;
    reporter.stop();
    assert_eq!(0, errors);

    info!(">> Wait for certificates to be available...");
    wait_for_certificates(&aggregator, 1, Duration::from_secs(30)).await?;

    info!(">> Wait for artifacts to be available...");
    wait_for_mithril_stake_distribution_artifacts(&aggregator, Duration::from_secs(30)).await?;

    info!(">> Add new immutable file");
    immutable_db.add_immutable_file();

    info!(">> Wait for pending certificate to be available");
    wait_for_pending_certificate(&aggregator, Duration::from_secs(30)).await?;

    info!(">> Compute the immutable files signature");
    let (current_beacon, immutable_files_signatures) = compute_immutable_files_signatures(
        &immutable_db,
        current_epoch,
        &signers_fixture,
        Duration::from_secs(30),
    )
    .await
    .unwrap();

    info!(
        ">> Send the Signer Signatures payloads for CardanoImmutableFiles({:?})",
        current_beacon
    );
    reporter.start("signatures registration");
    let errors = register_signatures_to_aggregator(
        &aggregator,
        &immutable_files_signatures,
        SignedEntityType::CardanoImmutableFilesFull(current_beacon),
    )
    .await?;
    reporter.stop();
    assert_eq!(0, errors);

    info!(">> Wait for certificates to be available...");
    wait_for_certificates(&aggregator, 2, Duration::from_secs(30)).await?;

    info!(">> Wait for artifacts to be available...");
    wait_for_immutable_files_artifacts(&aggregator, Duration::from_secs(30)).await?;

    info!(">> Display execution timings:");
    reporter.print_report();

    info!(">> All steps executed successfully, stopping all tasks...");
    aggregator.stop().await.unwrap();

    Ok(())
}
