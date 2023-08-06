use crate::crypto_helper::{ProtocolGenesisSignature, ProtocolMultiSignature};
use crate::entities::{
    Beacon, CertificateMetadata, HexEncodedAgregateVerificationKey, ProtocolMessage,
};
use std::cmp::Ordering;

use sha2::{Digest, Sha256};

/// The signature of a [Certificate]
#[derive(Clone, Debug)]
pub enum CertificateSignature {
    /// Genesis signature created from the original stake distribution
    /// aka GENESIS_SIG(AVK(-1))
    GenesisSignature(ProtocolGenesisSignature),

    /// STM multi signature created from a quorum of single signatures from the signers
    /// aka MULTI_SIG(H(MSG(p,n) || AVK(n-1)))
    MultiSignature(ProtocolMultiSignature),
}

/// Certificate represents a Mithril certificate embedding a Mithril STM multisignature
#[derive(Clone, Debug)]
pub struct Certificate {
    /// Hash of the current certificate
    /// Computed from the other fields of the certificate
    /// aka H(Cp,n))
    pub hash: String,

    /// Hash of the previous certificate in the chain
    /// This is either the hash of the first certificate of the epoch in the chain
    /// Or the first certificate of the previous epoch in the chain (if the certificate is the first of its epoch)
    /// aka H(FC(n))
    pub previous_hash: String,

    /// Mithril beacon on the Cardano chain
    /// aka BEACON(p,n)
    pub beacon: Beacon,

    /// Certificate metadata
    /// aka METADATA(p,n)
    pub metadata: CertificateMetadata,

    /// Structured message that is used to created the signed message
    /// aka MSG(p,n) U AVK(n-1)
    pub protocol_message: ProtocolMessage,

    /// Message that is signed by the signers
    /// aka H(MSG(p,n) || AVK(n-1))
    pub signed_message: String,

    /// Aggregate verification key
    /// The AVK used to sign during the current epoch
    /// aka AVK(n-2)
    pub aggregate_verification_key: HexEncodedAgregateVerificationKey,

    /// Certificate signature
    pub signature: CertificateSignature,
}

impl Certificate {
    /// Certificate factory
    pub fn new(
        previous_hash: String,
        beacon: Beacon,
        metadata: CertificateMetadata,
        protocol_message: ProtocolMessage,
        aggregate_verification_key: HexEncodedAgregateVerificationKey,
        signature: CertificateSignature,
    ) -> Certificate {
        let signed_message = protocol_message.compute_hash();
        let mut certificate = Certificate {
            hash: "".to_string(),
            previous_hash,
            beacon,
            metadata,
            protocol_message,
            signed_message,
            aggregate_verification_key,
            signature,
        };
        certificate.hash = certificate.compute_hash();
        certificate
    }

    /// Computes the hash of a Certificate
    pub fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.beacon.compute_hash().as_bytes());
        hasher.update(self.metadata.compute_hash().as_bytes());
        hasher.update(self.protocol_message.compute_hash().as_bytes());
        hasher.update(self.signed_message.as_bytes());
        hasher.update(self.aggregate_verification_key.as_bytes());
        match &self.signature {
            CertificateSignature::GenesisSignature(signature) => {
                hasher.update(signature.to_bytes_hex());
            }
            CertificateSignature::MultiSignature(signature) => {
                hasher.update(&signature.to_json_hex().unwrap());
            }
        };
        hex::encode(hasher.finalize())
    }

    /// Tell if the certificate is a genesis certificate
    pub fn is_genesis(&self) -> bool {
        matches!(self.signature, CertificateSignature::GenesisSignature(_))
    }

    /// Return true if the certificate is chaining into itself (meaning that its hash and previous
    /// hash are equal).
    pub fn is_chaining_to_itself(&self) -> bool {
        self.hash == self.previous_hash
    }
}

impl PartialEq for Certificate {
    fn eq(&self, other: &Self) -> bool {
        self.beacon.eq(&other.beacon) && self.hash.eq(&other.hash)
    }
}

impl PartialOrd for Certificate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Order by beacon first then per hash
        match self.beacon.partial_cmp(&other.beacon) {
            Some(ordering) if ordering == Ordering::Equal => self.hash.partial_cmp(&other.hash),
            Some(other) => Some(other),
            // Beacons may be not comparable (most likely because the network isn't the same) in
            // that case we can still order per hash
            None => self.hash.partial_cmp(&other.hash),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entities::{ProtocolMessagePartKey, ProtocolParameters, SignerWithStake},
        test_utils::fake_keys,
    };
    use chrono::{DateTime, Duration, Utc};

    fn get_signers_with_stake() -> Vec<SignerWithStake> {
        vec![
            SignerWithStake::new(
                "1".to_string(),
                fake_keys::signer_verification_key()[1].try_into().unwrap(),
                None,
                None,
                None,
                10,
            ),
            SignerWithStake::new(
                "2".to_string(),
                fake_keys::signer_verification_key()[2].try_into().unwrap(),
                None,
                None,
                None,
                20,
            ),
        ]
    }

    fn get_protocol_message() -> ProtocolMessage {
        let mut protocol_message = ProtocolMessage::new();
        protocol_message.set_message_part(
            ProtocolMessagePartKey::SnapshotDigest,
            "snapshot-digest-123".to_string(),
        );
        protocol_message.set_message_part(
            ProtocolMessagePartKey::NextAggregateVerificationKey,
            "next-avk-123".to_string(),
        );

        protocol_message
    }

    #[test]
    fn test_certificate_compute_hash() {
        const HASH_EXPECTED: &str =
            "255d59cef74aae5bc2e83e87612dea41309551dde0c770d46bf6607971bb9765";

        let initiated_at = DateTime::parse_from_rfc3339("2024-02-12T13:11:47.0123043Z")
            .unwrap()
            .with_timezone(&Utc);
        let sealed_at = initiated_at + Duration::seconds(100);

        let certificate = Certificate::new(
            "previous_hash".to_string(),
            Beacon::new("testnet".to_string(), 10, 100),
            CertificateMetadata::new(
                "0.1.0".to_string(),
                ProtocolParameters::new(1000, 100, 0.123),
                initiated_at,
                sealed_at,
                get_signers_with_stake(),
            ),
            get_protocol_message(),
            "aggregate_verification_key".to_string(),
            CertificateSignature::MultiSignature(
                fake_keys::multi_signature()[0].try_into().unwrap(),
            ),
        );

        assert_eq!(HASH_EXPECTED, certificate.compute_hash());

        assert_ne!(
            HASH_EXPECTED,
            Certificate {
                previous_hash: "previous_hash-modified".to_string(),
                ..certificate.clone()
            }
            .compute_hash(),
        );

        assert_ne!(
            HASH_EXPECTED,
            Certificate {
                beacon: Beacon::new("testnet-modified".to_string(), 10, 100),
                ..certificate.clone()
            }
            .compute_hash(),
        );

        assert_ne!(
            HASH_EXPECTED,
            Certificate {
                metadata: CertificateMetadata {
                    protocol_version: "0.1.0-modified".to_string(),
                    ..certificate.metadata.clone()
                },
                ..certificate.clone()
            }
            .compute_hash(),
        );

        assert_ne!(
            HASH_EXPECTED,
            Certificate {
                protocol_message: {
                    let mut protocol_message_modified = certificate.protocol_message.clone();
                    protocol_message_modified.set_message_part(
                        ProtocolMessagePartKey::NextAggregateVerificationKey,
                        "next-avk-456".to_string(),
                    );

                    protocol_message_modified
                },
                ..certificate.clone()
            }
            .compute_hash(),
        );

        assert_ne!(
            HASH_EXPECTED,
            Certificate {
                aggregate_verification_key: "aggregate_verification_key-modified".to_string(),
                ..certificate.clone()
            }
            .compute_hash(),
        );

        assert_ne!(
            HASH_EXPECTED,
            Certificate {
                signature: CertificateSignature::MultiSignature(
                    fake_keys::multi_signature()[1].try_into().unwrap()
                ),
                ..certificate.clone()
            }
            .compute_hash(),
        );
    }

    #[test]
    fn test_genesis_certificate_compute_hash() {
        const HASH_EXPECTED: &str =
            "bbb265e74082896873d3fbe568e4b0118ddcf9a63b4f4b369b92773439e80159";

        let initiated_at = DateTime::parse_from_rfc3339("2024-02-12T13:11:47.0123043Z")
            .unwrap()
            .with_timezone(&Utc);
        let sealed_at = initiated_at + Duration::seconds(100);

        let genesis_certificate = Certificate::new(
            "previous_hash".to_string(),
            Beacon::new("testnet".to_string(), 10, 100),
            CertificateMetadata::new(
                "0.1.0".to_string(),
                ProtocolParameters::new(1000, 100, 0.123),
                initiated_at,
                sealed_at,
                get_signers_with_stake(),
            ),
            get_protocol_message(),
            "aggregate_verification_key".to_string(),
            CertificateSignature::GenesisSignature(
                fake_keys::genesis_signature()[0].try_into().unwrap(),
            ),
        );

        assert_eq!(HASH_EXPECTED, genesis_certificate.compute_hash());

        assert_ne!(
            HASH_EXPECTED,
            Certificate {
                signature: CertificateSignature::GenesisSignature(
                    fake_keys::genesis_signature()[1].try_into().unwrap()
                ),
                ..genesis_certificate.clone()
            }
            .compute_hash(),
        );
    }
}
