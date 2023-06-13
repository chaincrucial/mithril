use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use mithril_common::certificate_chain::{
    CertificateRetriever, CertificateVerifier, CertificateVerifierError,
};
use mithril_common::crypto_helper::ProtocolGenesisVerifier;
use mithril_common::digesters::{ImmutableDigester, ImmutableDigesterError};
use mithril_common::entities::{Beacon, Certificate, ProtocolMessage, ProtocolParameters};
use mockall::mock;

mock! {
    pub DigesterImpl { }

    #[async_trait]
    impl ImmutableDigester for DigesterImpl {
        async fn compute_digest(
            &self,
            dirpath: &Path,
            beacon: &Beacon,
        ) -> Result<String, ImmutableDigesterError>;
    }
}

mock! {
    pub CertificateVerifierImpl { }

    #[async_trait]
    impl CertificateVerifier for CertificateVerifierImpl {
        fn verify_multi_signature(
            &self,
            message: &[u8],
            multi_signature: &str,
            aggregate_verification_key: &str,
            protocol_parameters: &ProtocolParameters,
        ) -> Result<(), CertificateVerifierError>;

        async fn verify_genesis_certificate(
            &self,
            certificate: &Certificate,
            genesis_verifier: &ProtocolGenesisVerifier,
        ) -> Result<(), CertificateVerifierError>;

        async fn verify_standard_certificate(
            &self,
            certificate: &Certificate,
            certificate_retriever: Arc<dyn CertificateRetriever>,
        ) -> Result<Option<Certificate>, CertificateVerifierError>;

        async fn verify_certificate(
            &self,
            certificate: &Certificate,
            certificate_retriever: Arc<dyn CertificateRetriever>,
            genesis_verifier: &ProtocolGenesisVerifier,
        ) -> Result<Option<Certificate>, CertificateVerifierError>;

        async fn verify_certificate_chain(
            &self,
            certificate: Certificate,
            certificate_retriever: Arc<dyn CertificateRetriever>,
            genesis_verifier: &ProtocolGenesisVerifier,
        ) -> Result<(), CertificateVerifierError>;

        fn verify_protocol_message(
            &self,
            protocol_message: &ProtocolMessage,
            certificate: &Certificate,
        ) -> bool;
    }
}
