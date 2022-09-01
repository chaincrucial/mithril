use mithril_common::digesters::ImmutableFileObserver;
use mithril_common::entities::{Signer, SignerWithStake};
use slog::Drain;
use slog_scope::debug;
use std::error::Error as StdError;
use std::{path::PathBuf, sync::Arc, time::Duration};
use thiserror::Error;

use mithril_common::crypto_helper::{key_encode_hex, tests_setup};
use mithril_common::{
    chain_observer::FakeObserver,
    digesters::{DumbImmutableDigester, DumbImmutableFileObserver},
    entities::{Beacon, Epoch},
    store::{adapter::MemoryAdapter, StakeStore, StakeStorer},
    BeaconProviderImpl,
};
use mithril_signer::{
    CertificateHandler, Config, MithrilSingleSigner, ProtocolInitializerStore,
    ProtocolInitializerStorer, SignerRunner, SignerServices, SignerState, StateMachine,
};

use super::FakeAggregator;

type Result<T> = std::result::Result<T, TestError>;

#[derive(Debug, Error)]
pub enum TestError {
    #[error("Assertion failed: {0}")]
    AssertFailed(String),
    #[error("Subsystem error: {0:?}")]
    SubsystemError(#[from] Box<dyn StdError + Sync + Send>),
    #[error("Value error: {0}")]
    ValueError(String),
}

pub struct StateMachineTester {
    state_machine: StateMachine,
    immutable_observer: Arc<DumbImmutableFileObserver>,
    chain_observer: Arc<FakeObserver>,
    certificate_handler: Arc<FakeAggregator>,
    protocol_initializer_store: Arc<ProtocolInitializerStore>,
    stake_store: Arc<StakeStore>,
    comment_no: u32,
    _logs_guard: slog_scope::GlobalLoggerGuard,
}

impl StateMachineTester {
    pub async fn init() -> Self {
        let config = Config {
            aggregator_endpoint: "http://0.0.0.0:8000".to_string(),
            cardano_cli_path: PathBuf::new(),
            cardano_node_socket_path: PathBuf::new(),
            db_directory: PathBuf::new(),
            network: "devnet".to_string(),
            network_magic: Some(42),
            party_id: "99999999999999999999999999999999".to_string(),
            run_interval: 5000,
            data_stores_directory: PathBuf::new(),
        };

        let decorator = slog_term::PlainDecorator::new(slog_term::TestStdoutWriter);
        let drain = slog_term::CompactFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logs_guard =
            slog_scope::set_global_logger(slog::Logger::root(Arc::new(drain), slog::o!()));

        let immutable_observer = Arc::new(DumbImmutableFileObserver::new());
        immutable_observer.shall_return(Some(1)).await;
        let chain_observer = Arc::new(FakeObserver::new(Some(Beacon {
            epoch: Epoch(1),
            immutable_file_number: 1,
            network: "devnet".to_string(),
        })));
        let beacon_provider = Arc::new(BeaconProviderImpl::new(
            chain_observer.clone(),
            immutable_observer.clone(),
            config.get_network().unwrap(),
        ));
        let certificate_handler = Arc::new(FakeAggregator::new(beacon_provider.clone()));
        let digester = Arc::new(DumbImmutableDigester::new("DIGEST", true));
        let protocol_initializer_store = Arc::new(ProtocolInitializerStore::new(Box::new(
            MemoryAdapter::new(None).unwrap(),
        )));
        let single_signer = Arc::new(MithrilSingleSigner::new(config.party_id.clone()));
        let stake_store = Arc::new(StakeStore::new(Box::new(MemoryAdapter::new(None).unwrap())));

        let services = SignerServices {
            certificate_handler: certificate_handler.clone(),
            beacon_provider: beacon_provider.clone(),
            chain_observer: chain_observer.clone(),
            digester: digester.clone(),
            protocol_initializer_store: protocol_initializer_store.clone(),
            single_signer: single_signer.clone(),
            stake_store: stake_store.clone(),
        };
        // set up stake distribution
        let protocol_parameters = tests_setup::setup_protocol_parameters();
        let mut signers: Vec<SignerWithStake> =
            tests_setup::setup_signers(10, &protocol_parameters)
                .into_iter()
                .map(|(party_id, stake, key, _, _)| SignerWithStake {
                    party_id,
                    stake,
                    verification_key: key_encode_hex(key).unwrap(),
                })
                .collect();
        signers.push(SignerWithStake {
            party_id: "99999999999999999999999999999999".to_string(),
            stake: 999,
            verification_key: "".to_string(),
        });

        chain_observer.set_signers(signers).await;

        let runner = Box::new(SignerRunner::new(config, services));

        let state_machine = StateMachine::new(
            SignerState::Unregistered(None),
            runner,
            Duration::from_secs(5),
        );

        StateMachineTester {
            state_machine,
            immutable_observer,
            chain_observer,
            certificate_handler,
            protocol_initializer_store,
            stake_store,
            comment_no: 0,
            _logs_guard: logs_guard,
        }
    }

    fn assert(&mut self, condition: bool, description: String) -> Result<&mut Self> {
        if !condition {
            Err(TestError::AssertFailed(description))
        } else {
            Ok(self)
        }
    }
    /// trigger a cycle in the state machine
    async fn cycle(&mut self) -> Result<&mut Self> {
        self.state_machine.cycle().await?;

        Ok(self)
    }

    /// cycle the state machine and test the resulting state
    pub async fn cycle_registered(&mut self) -> Result<&mut Self> {
        self.cycle().await?;
        self.assert(
            self.state_machine.get_state().is_registered(),
            format!(
                "state machine is in {} state (Registered was expected)",
                self.state_machine.get_state()
            ),
        )
    }

    /// cycle the state machine and test the resulting state
    pub async fn cycle_signed(&mut self) -> Result<&mut Self> {
        self.cycle().await?;

        self.assert(
            self.state_machine.get_state().is_signed(),
            format!(
                "state machine is in {} state (Signed was expected)",
                self.state_machine.get_state()
            ),
        )
    }

    /// cycle the state machine and test the resulting state
    pub async fn cycle_unregistered(&mut self) -> Result<&mut Self> {
        self.cycle().await?;

        self.assert(
            self.state_machine.get_state().is_unregistered(),
            format!(
                "state machine is in {} state (Unregistered was expected)",
                self.state_machine.get_state()
            ),
        )
    }

    /// make the aggregator send the epoch settings from now on
    pub async fn aggregator_send_epoch_settings(&mut self) -> &mut Self {
        self.certificate_handler.release_epoch_settings().await;
        self
    }

    /// check there is a protocol initializer for the given Epoch
    pub async fn check_protocol_initializer(&mut self, epoch: Epoch) -> Result<&mut Self> {
        let maybe_protocol_initializer = self
            .protocol_initializer_store
            .get_protocol_initializer(epoch)
            .await
            .map_err(|e| TestError::SubsystemError(e.into()))?;

        self.assert(maybe_protocol_initializer.is_some(), format!(
                "there should be a protocol intializer in store for Epoch {}, here is the last 3 in store: {:?}",
                epoch,
                self.protocol_initializer_store
                    .get_last_protocol_initializer(2)
                    .await
                    .unwrap(),
            ))
    }

    /// check there is a stake for the given Epoch
    pub async fn check_stake_store(&mut self, epoch: Epoch) -> Result<&mut Self> {
        let maybe_stakes = self
            .stake_store
            .get_stakes(epoch)
            .await
            .map_err(|e| TestError::SubsystemError(e.into()))?;

        self.assert(maybe_stakes.is_some(), format!(
            "there should be stake distribution saved for {}, here is the last 3 in the store: {:?}",
            epoch,
            self.stake_store.get_last_stakes(3).await.unwrap(),
        ))
    }

    /// increase the immutable file number in the dumb beacon provider
    pub async fn increase_immutable(&mut self, increment: u64, expected: u64) -> Result<&mut Self> {
        let immutable_number = self
            .immutable_observer
            .get_last_immutable_number()
            .await
            .map_err(|e| TestError::SubsystemError(e.into()))?;
        let new_immutable = immutable_number + increment;
        self.assert(
            expected == new_immutable,
            format!(
                "expected to increase immutable number up to {}, got {}",
                expected, new_immutable
            ),
        )?;
        self.immutable_observer
            .shall_return(Some(new_immutable))
            .await;

        Ok(self)
    }

    /// increase the epoch in the chain observer
    pub async fn increase_epoch(&mut self, expected: u64) -> Result<&mut Self> {
        let new_epoch = self
            .chain_observer
            .next_epoch()
            .await
            .ok_or_else(|| TestError::ValueError("no epoch returned".to_string()))?;

        self.assert(
            expected == new_epoch,
            format!(
                "Epoch increased by 1 to {} ({} expected)",
                new_epoch, expected
            ),
        )
    }

    /// add a comment in the logs
    pub fn comment(&mut self, comment: &str) -> &mut Self {
        self.comment_no += 1;
        debug!("COMMENT {:02} 💬 {}", self.comment_no, comment);

        self
    }

    /// register the signer in the certificate handler
    pub async fn register_signers(&mut self, count: u64) -> Result<&mut Self> {
        let protocol_parameters = tests_setup::setup_protocol_parameters();
        for (party_id, _stake, verification_key, _signer, _protocol_initializer) in
            tests_setup::setup_signers(count, &protocol_parameters)
        {
            let signer = Signer {
                party_id,
                verification_key: key_encode_hex(verification_key)
                    .map_err(|e| TestError::SubsystemError(e.into()))?,
            };
            self.certificate_handler
                .register_signer(&signer)
                .await
                .map_err(|e| TestError::SubsystemError(e.into()))?;
        }

        Ok(self)
    }
}
