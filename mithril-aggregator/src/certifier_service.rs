//! ## Certifier Service
//!
//! This service is responsible of [OpenMessage] cycle of life. It creates open
//! messages and turn them into [Certificate]. To do so, it registers
//! single signatures and deal with the multi_signer for aggregate signature
//! creation.
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use mithril_common::certificate_chain::CertificateVerifier;
use mithril_common::crypto_helper::{key_encode_hex, ProtocolGenesisVerifier, PROTOCOL_VERSION};
use mithril_common::entities::{
    Certificate, CertificateMetadata, Epoch, ProtocolMessage, SignedEntityType, SingleSignatures,
};
use mithril_common::StdResult;
use slog::Logger;
use slog_scope::{debug, error, info, warn};
use thiserror::Error;
use tokio::sync::RwLock;

use crate::database::provider::{
    CertificateRepository, OpenMessageRecord, OpenMessageRepository,
    OpenMessageWithSingleSignaturesRecord, SingleSignatureRepository,
};
use crate::entities::OpenMessage;
use crate::MultiSigner;

#[cfg(test)]
use mockall::automock;

/// Errors dedicated to the CertifierService.
#[derive(Debug, Error)]
pub enum CertifierServiceError {
    /// OpenMessage not found.
    #[error("The open message was not found for beacon {0:?}.")]
    NotFound(SignedEntityType),

    /// The open message is already certified, no more single signatures may be
    /// attached to it nor be certified again.
    #[error("Open message for beacon {0:?} already certified.")]
    AlreadyCertified(SignedEntityType),

    /// The given beacon is older than the current open message for this type.
    #[error("Given beacon {0:?} is older than the current open message beacon.")]
    BeaconTooOld(SignedEntityType),

    /// The given OpenMessage already exists
    #[error("An open message already exist for this beacon {0:?}, cannot create another one.")]
    OpenMessageAlreadyExists(SignedEntityType),

    /// No parent certificate could be found, this certifier cannot create genesis certificates.
    #[error(
        "No parent certificate could be found, this certifier cannot create genesis certificates."
    )]
    NoParentCertificateFound,

    /// Codec error.
    #[error("codec error: '{0}'")]
    Codec(String),
}

/// ## CertifierService
///
/// This service manages the open message and their beacon transitions. It can
/// ultimately transform open messages into certificates.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait CertifierService: Sync + Send {
    /// Inform the certifier I have detected a new epoch, it may clear its state
    /// and prepare the new signature round. If the given Epoch is equal or less
    /// than the previous informed Epoch, nothing is done.
    async fn inform_epoch(&self, epoch: Epoch) -> StdResult<()>;

    /// Add a new single signature for the open message at the given beacon. If
    /// the open message does not exist or the open message has been certified
    /// since then, an error is returned.
    async fn register_single_signature(
        &self,
        signed_entity_type: &SignedEntityType,
        signature: &SingleSignatures,
    ) -> StdResult<()>;

    /// Create an open message at the given beacon. If the open message does not
    /// exist or exists at an older beacon, the older open messages are cleared
    /// along with their associated single signatures and the new open message
    /// is created. If the message already exists, an error is returned.
    async fn create_open_message(
        &self,
        signed_entity_type: &SignedEntityType,
        protocol_message: &ProtocolMessage,
    ) -> StdResult<OpenMessage>;

    /// Return the open message at the given Beacon. If the message does not
    /// exist, None is returned.
    async fn get_open_message(
        &self,
        signed_entity_type: &SignedEntityType,
    ) -> StdResult<Option<OpenMessage>>;

    /// Create a certificate if possible. If the pointed open message does
    /// not exist or has been already certified, an error is raised. If a multi
    /// signature is created then the flag `is_certified` of the open
    /// message is set to true. The Certificate is created.
    /// If the stake quorum of the single signatures is
    /// not reached for the multisignature to be created, the certificate is not
    /// created and None is returned. If the certificate can be created, the
    /// list of the registered signers for the given epoch is used.
    async fn create_certificate(
        &self,
        signed_entity_type: &SignedEntityType,
    ) -> StdResult<Option<Certificate>>;
}

/// Mithril CertifierService implementation
pub struct MithrilCertifierService {
    open_message_repository: Arc<OpenMessageRepository>,
    single_signature_repository: Arc<SingleSignatureRepository>,
    certificate_repository: Arc<CertificateRepository>,
    certificate_verifier: Arc<dyn CertificateVerifier>,
    genesis_verifier: Arc<ProtocolGenesisVerifier>,
    multi_signer: Arc<RwLock<dyn MultiSigner>>,
    current_epoch: Arc<RwLock<Epoch>>,
    _logger: Logger,
}

impl MithrilCertifierService {
    /// instantiate the service
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open_message_repository: Arc<OpenMessageRepository>,
        single_signature_repository: Arc<SingleSignatureRepository>,
        certificate_repository: Arc<CertificateRepository>,
        certificate_verifier: Arc<dyn CertificateVerifier>,
        genesis_verifier: Arc<ProtocolGenesisVerifier>,
        multi_signer: Arc<RwLock<dyn MultiSigner>>,
        current_epoch: Epoch,
        logger: Logger,
    ) -> Self {
        Self {
            open_message_repository,
            single_signature_repository,
            certificate_repository,
            multi_signer,
            certificate_verifier,
            genesis_verifier,
            current_epoch: Arc::new(RwLock::new(current_epoch)),
            _logger: logger,
        }
    }

    async fn get_open_message_record(
        &self,
        signed_entity_type: &SignedEntityType,
    ) -> StdResult<Option<OpenMessageWithSingleSignaturesRecord>> {
        debug!(
            "CertifierService::get_open_message_record(signed_entity_type: {signed_entity_type:?})"
        );

        self.open_message_repository
            .get_open_message_with_single_signatures(signed_entity_type)
            .await
    }
}

#[async_trait]
impl CertifierService for MithrilCertifierService {
    async fn inform_epoch(&self, epoch: Epoch) -> StdResult<()> {
        debug!("CertifierService::inform_epoch(epoch: {epoch:?})");
        let mut current_epoch = self.current_epoch.write().await;

        if epoch <= *current_epoch {
            debug!("CertifierService::inform_epoch: given epoch ({epoch:?}) is older than current epoch ({}), ignoring", *current_epoch);

            return Ok(());
        }
        let nb = self
            .open_message_repository
            .clean_epoch(*current_epoch)
            .await?;
        info!("MithrilCertifierService: Got a new Epoch: {epoch:?}. Cleaned {nb} open messages along with their single signatures.");
        *current_epoch = epoch;

        Ok(())
    }

    async fn register_single_signature(
        &self,
        signed_entity_type: &SignedEntityType,
        signature: &SingleSignatures,
    ) -> StdResult<()> {
        debug!("CertifierService::register_single_signature(signed_entity_type: {signed_entity_type:?}, single_signatures: {signature:?}");
        let open_message = self
            .get_open_message_record(signed_entity_type)
            .await?
            .ok_or_else(|| {
                warn!("CertifierService::register_single_signature: OpenMessage not found for type {signed_entity_type:?}.");
                CertifierServiceError::NotFound(signed_entity_type.clone())
            })?;

        if open_message.is_certified {
            warn!("CertifierService::register_single_signature: open message {signed_entity_type:?} is already certified, cannot register single signature.");

            return Err(CertifierServiceError::AlreadyCertified(signed_entity_type.clone()).into());
        }

        let multi_signer = self.multi_signer.read().await;
        multi_signer
            .verify_single_signature(&open_message.protocol_message, signature)
            .await?;

        let single_signature = self
            .single_signature_repository
            .create_single_signature(signature, &open_message.into())
            .await?;
        info!("CertifierService::register_single_signature: created pool '{}' single signature for {signed_entity_type:?}.", single_signature.signer_id);
        debug!("CertifierService::register_single_signature: created single signature for open message ID='{}'.", single_signature.open_message_id);

        Ok(())
    }

    async fn create_open_message(
        &self,
        signed_entity_type: &SignedEntityType,
        protocol_message: &ProtocolMessage,
    ) -> StdResult<OpenMessage> {
        debug!("CertifierService::create_open_message(signed_entity_type: {signed_entity_type:?}, protocol_message: {protocol_message:?})");
        let current_epoch = self.current_epoch.read().await;
        let open_message = self
            .open_message_repository
            .create_open_message(*current_epoch, signed_entity_type, protocol_message)
            .await?;
        info!("CertifierService::create_open_message: created open message for {signed_entity_type:?}");
        debug!(
            "CertifierService::create_open_message: created open message ID='{}'",
            open_message.open_message_id
        );

        Ok(open_message.into())
    }

    async fn get_open_message(
        &self,
        signed_entity_type: &SignedEntityType,
    ) -> StdResult<Option<OpenMessage>> {
        debug!("CertifierService::get_open_message(signed_entity_type: {signed_entity_type:?})");

        let open_message = self
            .open_message_repository
            .get_open_message_with_single_signatures(signed_entity_type)
            .await?
            .map(|record| record.into());

        Ok(open_message)
    }

    async fn create_certificate(
        &self,
        signed_entity_type: &SignedEntityType,
    ) -> StdResult<Option<Certificate>> {
        debug!("CertifierService::create_certificate(signed_entity_type: {signed_entity_type:?})");
        let open_message_record = self
            .get_open_message_record(signed_entity_type)
            .await?
            .ok_or_else(|| {
                warn!("CertifierService::create_certificate: OpenMessage not found for type {signed_entity_type:?}.");
                CertifierServiceError::NotFound(signed_entity_type.clone())
            })?;
        let open_message: OpenMessage = open_message_record.clone().into();

        if open_message.is_certified {
            warn!("CertifierService::create_certificate: open message {signed_entity_type:?} is already certified, cannot create certificate.");

            return Err(CertifierServiceError::AlreadyCertified(signed_entity_type.clone()).into());
        }

        let multi_signer = self.multi_signer.write().await;
        let signature = multi_signer.create_multi_signature(&open_message).await?;

        if signature.is_some() {
            info!("CertifierService::create_certificate: multi-signature created for open message {signed_entity_type:?}");
        } else {
            debug!("CertifierService::create_certificate: No multi-signature could be created for open message {signed_entity_type:?}");

            return Ok(None);
        }
        let signature = signature.unwrap();
        let signer_ids = open_message.get_signers_id();
        let signers = multi_signer
            .get_signers_with_stake()
            .await?
            .into_iter()
            .filter(|signer| signer_ids.contains(&signer.party_id))
            .collect::<Vec<_>>();

        let protocol_version = PROTOCOL_VERSION.to_string();
        let initiated_at = format!("{:?}", open_message.created_at);
        let sealed_at = format!("{:?}", Utc::now());
        let metadata = CertificateMetadata::new(
            protocol_version,
            // TODO remove this multi_signer call ↓
            multi_signer
                .get_protocol_parameters()
                .await?
                .unwrap()
                .into(),
            initiated_at,
            sealed_at,
            signers,
        );
        let multi_signature = key_encode_hex(signature).map_err(CertifierServiceError::Codec)?;
        let parent_certificate_hash = self
            .certificate_repository
            .get_master_certificate_for_epoch(open_message.epoch)
            .await?
            .map(|cert| cert.hash)
            .ok_or_else(|| Box::new(CertifierServiceError::NoParentCertificateFound))?;

        let certificate = Certificate::new(
            parent_certificate_hash,
            // TODO: remove this multi_signer call ↓
            multi_signer.get_current_beacon().await.unwrap(),
            metadata,
            open_message.protocol_message.clone(),
            multi_signer
                .compute_stake_distribution_aggregate_verification_key()
                .await?
                .unwrap(),
            multi_signature,
            "".to_string(),
        );

        self.certificate_verifier
            .verify_certificate(
                &certificate,
                self.certificate_repository.clone(),
                &self.genesis_verifier,
            )
            .await?;

        let certificate = self
            .certificate_repository
            .create_certificate(certificate)
            .await?;

        let mut open_message_certified: OpenMessageRecord = open_message_record.into();
        open_message_certified.is_certified = true;
        self.open_message_repository
            .update_open_message(&open_message_certified)
            .await?;

        Ok(Some(certificate))
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        dependency_injection::DependenciesBuilder, multi_signer::MockMultiSigner, Configuration,
    };
    use mithril_common::{
        entities::{Beacon, ProtocolMessagePartKey},
        test_utils::{MithrilFixture, MithrilFixtureBuilder},
    };

    use super::*;

    async fn setup_certifier_service(
        fixture: &MithrilFixture,
        epochs_with_signers: &[Epoch],
    ) -> MithrilCertifierService {
        let configuration = Configuration::new_sample();
        let mut dependency_builder = DependenciesBuilder::new(configuration);
        let connection = dependency_builder.get_sqlite_connection().await.unwrap();

        let dependency_manager = dependency_builder
            .build_dependency_container()
            .await
            .unwrap();
        dependency_manager
            .init_state_from_fixture(fixture, epochs_with_signers)
            .await;

        let open_message_repository = Arc::new(OpenMessageRepository::new(connection.clone()));
        let single_signature_repository =
            Arc::new(SingleSignatureRepository::new(connection.clone()));
        let certificate_repository = Arc::new(CertificateRepository::new(connection));
        let certificate_verifier = dependency_builder.get_certificate_verifier().await.unwrap();
        let genesis_verifier = dependency_builder.get_genesis_verifier().await.unwrap();
        let multi_signer = dependency_builder.get_multi_signer().await.unwrap();
        let logger = dependency_builder.get_logger().await.unwrap();

        MithrilCertifierService::new(
            open_message_repository,
            single_signature_repository,
            certificate_repository,
            certificate_verifier,
            genesis_verifier,
            multi_signer,
            Epoch(0),
            logger,
        )
    }

    #[tokio::test]
    async fn should_not_clean_epoch_when_inform_same_epoch() {
        let beacon = Beacon::new("devnet".to_string(), 1, 1);
        let signed_entity_type = SignedEntityType::CardanoImmutableFilesFull(beacon.clone());
        let protocol_message = ProtocolMessage::new();
        let epoch = beacon.epoch;
        let epochs_with_signers = (1..=5).map(Epoch).collect::<Vec<_>>();
        let fixture = MithrilFixtureBuilder::default().with_signers(5).build();
        let mut certifier_service = setup_certifier_service(&fixture, &epochs_with_signers).await;
        certifier_service.current_epoch = Arc::new(RwLock::new(epoch));
        certifier_service
            .create_open_message(&signed_entity_type, &protocol_message)
            .await
            .unwrap();
        certifier_service.inform_epoch(epoch).await.unwrap();
        let open_message = certifier_service
            .get_open_message(&signed_entity_type)
            .await
            .unwrap();
        assert!(open_message.is_some());
    }

    #[tokio::test]
    async fn should_clean_epoch_when_inform_new_epoch() {
        let beacon = Beacon::new("devnet".to_string(), 1, 1);
        let signed_entity_type = SignedEntityType::CardanoImmutableFilesFull(beacon.clone());
        let protocol_message = ProtocolMessage::new();
        let epoch = beacon.epoch;
        let epochs_with_signers = (1..=5).map(Epoch).collect::<Vec<_>>();
        let fixture = MithrilFixtureBuilder::default().with_signers(5).build();
        let mut certifier_service = setup_certifier_service(&fixture, &epochs_with_signers).await;
        certifier_service.current_epoch = Arc::new(RwLock::new(epoch));
        certifier_service
            .create_open_message(&signed_entity_type, &protocol_message)
            .await
            .unwrap();
        certifier_service.inform_epoch(epoch + 1).await.unwrap();
        let open_message = certifier_service
            .get_open_message(&signed_entity_type)
            .await
            .unwrap();
        assert!(open_message.is_none());
    }

    #[tokio::test]
    async fn should_register_valid_single_signature() {
        let beacon = Beacon::new("devnet".to_string(), 3, 1);
        let signed_entity_type = SignedEntityType::CardanoImmutableFilesFull(beacon.clone());
        let protocol_message = ProtocolMessage::new();
        let epoch = beacon.epoch;
        let epochs_with_signers = (1..=5).map(Epoch).collect::<Vec<_>>();
        let fixture = MithrilFixtureBuilder::default().with_signers(1).build();
        let mut certifier_service = setup_certifier_service(&fixture, &epochs_with_signers).await;
        certifier_service.current_epoch = Arc::new(RwLock::new(epoch));
        certifier_service
            .multi_signer
            .write()
            .await
            .update_current_beacon(beacon.clone())
            .await
            .unwrap();

        certifier_service
            .create_open_message(&signed_entity_type, &protocol_message)
            .await
            .unwrap();

        let mut signatures = Vec::new();
        for signer_fixture in fixture.signers_fixture() {
            if let Some(signature) = signer_fixture.sign(&protocol_message) {
                signatures.push(signature);
            }
        }
        certifier_service
            .register_single_signature(&signed_entity_type, &signatures[0])
            .await
            .unwrap();
        let open_message = certifier_service
            .get_open_message(&signed_entity_type)
            .await
            .unwrap()
            .unwrap();
        assert!(!open_message.single_signatures.is_empty());
    }

    #[tokio::test]
    async fn should_not_register_invalid_single_signature() {
        let beacon = Beacon::new("devnet".to_string(), 3, 1);
        let signed_entity_type = SignedEntityType::CardanoImmutableFilesFull(beacon.clone());
        let mut protocol_message = ProtocolMessage::new();
        let epoch = beacon.epoch;
        let epochs_with_signers = (1..=5).map(Epoch).collect::<Vec<_>>();
        let fixture = MithrilFixtureBuilder::default().with_signers(1).build();
        let mut certifier_service = setup_certifier_service(&fixture, &epochs_with_signers).await;
        certifier_service.current_epoch = Arc::new(RwLock::new(epoch));
        certifier_service
            .multi_signer
            .write()
            .await
            .update_current_beacon(beacon.clone())
            .await
            .unwrap();

        certifier_service
            .create_open_message(&signed_entity_type, &protocol_message)
            .await
            .unwrap();

        protocol_message.set_message_part(
            ProtocolMessagePartKey::SnapshotDigest,
            "snapshot-digest-123".to_string(),
        );

        let mut signatures = Vec::new();
        for signer_fixture in fixture.signers_fixture() {
            if let Some(signature) = signer_fixture.sign(&protocol_message) {
                signatures.push(signature);
            }
        }
        certifier_service
            .register_single_signature(&signed_entity_type, &signatures[0])
            .await
            .expect_err("register_single_signature should fail");
    }

    #[tokio::test]
    async fn should_not_register_single_signature_for_certified_open_message() {
        let beacon = Beacon::new("devnet".to_string(), 3, 1);
        let signed_entity_type = SignedEntityType::CardanoImmutableFilesFull(beacon.clone());
        let protocol_message = ProtocolMessage::new();
        let epoch = beacon.epoch;
        let epochs_with_signers = (1..=5).map(Epoch).collect::<Vec<_>>();
        let fixture = MithrilFixtureBuilder::default().with_signers(1).build();
        let mut certifier_service = setup_certifier_service(&fixture, &epochs_with_signers).await;
        certifier_service.current_epoch = Arc::new(RwLock::new(epoch));
        let mut open_message = certifier_service
            .open_message_repository
            .create_open_message(beacon.epoch, &signed_entity_type, &protocol_message)
            .await
            .unwrap();
        open_message.is_certified = true;
        certifier_service
            .open_message_repository
            .update_open_message(&open_message)
            .await
            .unwrap();

        let mut signatures = Vec::new();
        for signer_fixture in fixture.signers_fixture() {
            if let Some(signature) = signer_fixture.sign(&protocol_message) {
                signatures.push(signature);
            }
        }
        certifier_service
            .register_single_signature(&signed_entity_type, &signatures[0])
            .await
            .expect_err("register_single_signature should fail");
    }

    #[tokio::test]
    async fn should_create_certificate_when_multi_signature_produced() {
        let beacon = Beacon::new("devnet".to_string(), 3, 1);
        let signed_entity_type = SignedEntityType::CardanoImmutableFilesFull(beacon.clone());
        let protocol_message = ProtocolMessage::new();
        let epoch = beacon.epoch;
        let epochs_with_signers = (1..=5).map(Epoch).collect::<Vec<_>>();
        let fixture = MithrilFixtureBuilder::default().with_signers(3).build();
        let mut certifier_service = setup_certifier_service(&fixture, &epochs_with_signers).await;
        certifier_service.current_epoch = Arc::new(RwLock::new(epoch));
        certifier_service
            .multi_signer
            .write()
            .await
            .update_current_beacon(beacon.clone())
            .await
            .unwrap();

        certifier_service
            .create_open_message(&signed_entity_type, &protocol_message)
            .await
            .unwrap();

        let genesis_beacon = Beacon {
            epoch: beacon.epoch - 1,
            ..beacon.clone()
        };
        let genesis_certificate = fixture.create_genesis_certificate(&genesis_beacon);
        certifier_service
            .certificate_repository
            .create_certificate(genesis_certificate)
            .await
            .unwrap();

        let mut signatures = Vec::new();
        for signer_fixture in fixture.signers_fixture() {
            if let Some(signature) = signer_fixture.sign(&protocol_message) {
                signatures.push(signature);
            }
        }
        for signature in signatures {
            certifier_service
                .register_single_signature(&signed_entity_type, &signature)
                .await
                .expect("register_single_signature should not fail");
        }

        let create_certificate_result = certifier_service
            .create_certificate(&signed_entity_type)
            .await
            .unwrap();
        assert!(create_certificate_result.is_some());

        let certificate_created = create_certificate_result.unwrap();
        certifier_service
            .certificate_verifier
            .verify_certificate(
                &certificate_created,
                certifier_service.certificate_repository.clone(),
                &certifier_service.genesis_verifier,
            )
            .await
            .unwrap();

        let open_message = certifier_service
            .get_open_message(&signed_entity_type)
            .await
            .unwrap()
            .unwrap();
        assert!(open_message.is_certified);
    }

    #[tokio::test]
    async fn should_not_create_certificate_for_open_message_not_created() {
        let beacon = Beacon::new("devnet".to_string(), 1, 1);
        let signed_entity_type = SignedEntityType::CardanoImmutableFilesFull(beacon.clone());
        let epoch = beacon.epoch;
        let epochs_with_signers = (1..=5).map(Epoch).collect::<Vec<_>>();
        let fixture = MithrilFixtureBuilder::default().with_signers(5).build();
        let mut certifier_service = setup_certifier_service(&fixture, &epochs_with_signers).await;
        certifier_service.current_epoch = Arc::new(RwLock::new(epoch));
        certifier_service
            .create_certificate(&signed_entity_type)
            .await
            .expect_err("create_certificate should fail");
    }

    #[tokio::test]
    async fn should_not_create_certificate_for_open_message_already_certified() {
        let beacon = Beacon::new("devnet".to_string(), 1, 1);
        let signed_entity_type = SignedEntityType::CardanoImmutableFilesFull(beacon.clone());
        let protocol_message = ProtocolMessage::new();
        let epoch = beacon.epoch;
        let epochs_with_signers = (1..=5).map(Epoch).collect::<Vec<_>>();
        let fixture = MithrilFixtureBuilder::default().with_signers(5).build();
        let mut certifier_service = setup_certifier_service(&fixture, &epochs_with_signers).await;
        certifier_service.current_epoch = Arc::new(RwLock::new(epoch));
        certifier_service
            .open_message_repository
            .create_open_message(epoch, &signed_entity_type, &protocol_message)
            .await
            .unwrap();
        certifier_service
            .create_certificate(&signed_entity_type)
            .await
            .expect_err("create_certificate should fail");
    }

    #[tokio::test]
    async fn should_not_create_certificate_when_no_multi_signature_produced() {
        let mut mock_multi_signer = MockMultiSigner::new();
        mock_multi_signer
            .expect_create_multi_signature()
            .return_once(move |_| Ok(None));
        let beacon = Beacon::new("devnet".to_string(), 1, 1);
        let signed_entity_type = SignedEntityType::CardanoImmutableFilesFull(beacon.clone());
        let protocol_message = ProtocolMessage::new();
        let epoch = beacon.epoch;
        let epochs_with_signers = (1..=5).map(Epoch).collect::<Vec<_>>();
        let fixture = MithrilFixtureBuilder::default().with_signers(5).build();
        let mut certifier_service = setup_certifier_service(&fixture, &epochs_with_signers).await;
        certifier_service.current_epoch = Arc::new(RwLock::new(epoch));
        certifier_service.multi_signer = Arc::new(RwLock::new(mock_multi_signer));
        certifier_service
            .create_open_message(&signed_entity_type, &protocol_message)
            .await
            .unwrap();
        let create_certificate_result = certifier_service
            .create_certificate(&signed_entity_type)
            .await
            .unwrap();
        assert!(create_certificate_result.is_none());
    }
}