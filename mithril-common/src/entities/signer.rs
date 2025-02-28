use crate::{
    crypto_helper::{KESPeriod, ProtocolSignerVerificationKey},
    entities::{HexEncodedOpCert, HexEncodedVerificationKeySignature, PartyId, Stake},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Signer represents a signing participant in the network
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Signer {
    /// The unique identifier of the signer
    // TODO: Should be removed once the signer certification is fully deployed
    pub party_id: PartyId,

    /// The public key used to authenticate signer signature
    pub verification_key: ProtocolSignerVerificationKey,

    /// The encoded signer 'Mithril verification key' signature (signed by the Cardano node KES secret key)
    // TODO: Option should be removed once the signer certification is fully deployed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_key_signature: Option<HexEncodedVerificationKeySignature>,

    /// The encoded operational certificate of stake pool operator attached to the signer node
    // TODO: Option should be removed once the signer certification is fully deployed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_certificate: Option<HexEncodedOpCert>,

    /// The kes period used to compute the verification key signature
    // TODO: This kes period shoud not be used as is and should probably be within an allowed range of kes period for the epoch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kes_period: Option<KESPeriod>,
}

impl PartialEq for Signer {
    fn eq(&self, other: &Self) -> bool {
        self.party_id.eq(&other.party_id)
    }
}

impl Signer {
    /// Signer factory
    pub fn new(
        party_id: PartyId,
        verification_key: ProtocolSignerVerificationKey,
        verification_key_signature: Option<HexEncodedVerificationKeySignature>,
        operational_certificate: Option<HexEncodedOpCert>,
        kes_period: Option<KESPeriod>,
    ) -> Signer {
        Signer {
            party_id,
            verification_key,
            verification_key_signature,
            operational_certificate,
            kes_period,
        }
    }

    /// Computes the hash of Signer
    pub fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.party_id.as_bytes());
        hasher.update(self.verification_key.to_json_hex().unwrap().as_bytes());

        if let Some(verification_key_signature) = &self.verification_key_signature {
            hasher.update(verification_key_signature.as_bytes());
        }
        if let Some(operational_certificate) = &self.operational_certificate {
            hasher.update(operational_certificate.as_bytes());
        }
        hex::encode(hasher.finalize())
    }
}

impl From<SignerWithStake> for Signer {
    fn from(other: SignerWithStake) -> Self {
        Signer::new(
            other.party_id,
            other.verification_key,
            other.verification_key_signature,
            other.operational_certificate,
            other.kes_period,
        )
    }
}

/// Signer represents a signing party in the network (including its stakes)
#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct SignerWithStake {
    /// The unique identifier of the signer
    // TODO: Should be removed once the signer certification is fully deployed
    pub party_id: PartyId,

    /// The public key used to authenticate signer signature
    pub verification_key: ProtocolSignerVerificationKey,

    /// The encoded signer 'Mithril verification key' signature (signed by the Cardano node KES secret key)
    // TODO: Option should be removed once the signer certification is fully deployed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_key_signature: Option<HexEncodedVerificationKeySignature>,

    /// The encoded operational certificate of stake pool operator attached to the signer node
    // TODO: Option should be removed once the signer certification is fully deployed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_certificate: Option<HexEncodedOpCert>,

    /// The kes period used to compute the verification key signature
    // TODO: This kes period shoud not be used as is and should probably be within an allowed range of kes period for the epoch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kes_period: Option<KESPeriod>,

    /// The signer stake
    pub stake: Stake,
}

impl PartialEq for SignerWithStake {
    fn eq(&self, other: &Self) -> bool {
        self.party_id.eq(&other.party_id)
    }
}

impl PartialOrd for SignerWithStake {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.party_id.partial_cmp(&other.party_id)
    }
}

impl Ord for SignerWithStake {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.party_id.cmp(&other.party_id)
    }
}

impl SignerWithStake {
    /// SignerWithStake factory
    pub fn new(
        party_id: PartyId,
        verification_key: ProtocolSignerVerificationKey,
        verification_key_signature: Option<HexEncodedVerificationKeySignature>,
        operational_certificate: Option<HexEncodedOpCert>,
        kes_period: Option<KESPeriod>,
        stake: Stake,
    ) -> SignerWithStake {
        SignerWithStake {
            party_id,
            verification_key,
            verification_key_signature,
            operational_certificate,
            kes_period,
            stake,
        }
    }

    /// Turn a [Signer] into a [SignerWithStake].
    pub fn from_signer(signer: Signer, stake: Stake) -> Self {
        Self {
            party_id: signer.party_id,
            verification_key: signer.verification_key,
            verification_key_signature: signer.verification_key_signature,
            operational_certificate: signer.operational_certificate,
            kes_period: signer.kes_period,
            stake,
        }
    }

    /// Computes the hash of SignerWithStake
    pub fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.party_id.as_bytes());
        hasher.update(self.verification_key.to_json_hex().unwrap().as_bytes());

        if let Some(verification_key_signature) = &self.verification_key_signature {
            hasher.update(verification_key_signature.as_bytes());
        }

        if let Some(operational_certificate) = &self.operational_certificate {
            hasher.update(operational_certificate.as_bytes());
        }
        hasher.update(self.stake.to_be_bytes());
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_keys, MithrilFixtureBuilder};

    use super::*;

    #[test]
    fn test_stake_signers_from_into() {
        let verification_key = MithrilFixtureBuilder::default()
            .with_signers(1)
            .build()
            .signers_with_stake()[0]
            .verification_key
            .clone();
        let signer_expected =
            Signer::new("1".to_string(), verification_key.clone(), None, None, None);
        let signer_with_stake =
            SignerWithStake::new("1".to_string(), verification_key, None, None, None, 100);

        let signer_into: Signer = signer_with_stake.into();
        assert_eq!(signer_expected, signer_into);
    }

    #[test]
    fn test_signer_compute_hash() {
        const HASH_EXPECTED: &str =
            "02778791113dcd8647b019366e223bfe3aa8a054fa6d9d1918b6b669de485f1c";

        assert_eq!(
            HASH_EXPECTED,
            Signer::new(
                "1".to_string(),
                fake_keys::signer_verification_key()[3].try_into().unwrap(),
                None,
                None,
                None,
            )
            .compute_hash()
        );
        assert_ne!(
            HASH_EXPECTED,
            Signer::new(
                "0".to_string(),
                fake_keys::signer_verification_key()[3].try_into().unwrap(),
                None,
                None,
                None
            )
            .compute_hash()
        );
        assert_ne!(
            HASH_EXPECTED,
            Signer::new(
                "1".to_string(),
                fake_keys::signer_verification_key()[0].try_into().unwrap(),
                None,
                None,
                None
            )
            .compute_hash()
        );
    }

    #[test]
    fn test_signer_with_stake_compute_hash() {
        const EXPECTED_HASH: &str =
            "9a832baccd04aabfc419f57319e3831a1655a95bf3bf5ed96a1167d1e81b5085";
        let signers = MithrilFixtureBuilder::default()
            .with_signers(2)
            .build()
            .signers_with_stake();
        let signer = signers[0].clone();

        assert_eq!(EXPECTED_HASH, signer.compute_hash());

        {
            let mut signer_different_party_id = signer.clone();
            signer_different_party_id.party_id = "whatever".to_string();

            assert_ne!(EXPECTED_HASH, signer_different_party_id.compute_hash());
        }
        {
            let mut signer_different_verification_key = signer.clone();
            signer_different_verification_key.verification_key =
                signers[1].verification_key.clone();

            assert_ne!(
                EXPECTED_HASH,
                signer_different_verification_key.compute_hash()
            );
        }
        {
            let mut signer_different_stake = signer.clone();
            signer_different_stake.stake += 20;

            assert_ne!(EXPECTED_HASH, signer_different_stake.compute_hash());
        }
    }
}
