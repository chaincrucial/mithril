use thiserror::Error;

use crate::{
    crypto_helper::{
        key_decode_hex, ProtocolClosedKeyRegistration, ProtocolKeyRegistration,
        ProtocolStakeDistribution, ProtocolSignerVerificationKeySignature, OpCert, ProtocolSignerVerificationKey,
    },
    entities::{ProtocolParameters, SignerWithStake, StakeDistribution},
    StdResult,
};

pub struct SignerBuilder {
    protocol_parameters: ProtocolParameters,
    closed_key_registration: ProtocolClosedKeyRegistration,
}

#[derive(Debug, Error)]
pub enum SignerBuilderError {
    #[error("The list of signers must not be empty to create a signer builder.")]
    NoSignerRegistered,

    #[error("Invalid signers in stake distribution: '{0.party_id}'. It may be caused by an invalid Operational Certificate.")]
    InvalidSigner(SignerWithStake),
}

impl SignerBuilder {
    pub fn new(
        registered_signers: &[SignerWithStake],
        protocol_parameters: &ProtocolParameters,
    ) -> StdResult<Self> {
        let stake_distribution = registered_signers
            .iter()
            .map(|s| s.into())
            .collect::<ProtocolStakeDistribution>();
        let mut key_registration = ProtocolKeyRegistration::init(&stake_distribution);
        let mut total_signers = 0;

        for signer in registered_signers {
            let (operational_certificate, verification_key, kes_signature) = Self::decode_signer_keys(signer)?;
            let kes_period = signer.kes_period;
            key_registration
                .register(
                    Some(signer.party_id.to_owned()),
                    operational_certificate,
                    kes_signature,
                    kes_period,
                    verification_key,
                )
                .map_err(|e| ProtocolError::Core(e.to_string()))?;
            total_signers += 1;
        }
        match total_signers {
            0 => Ok(None),
            _ => {
                let closed_registration = key_registration.close();
            }
        };
        todo!()
    }

    fn decode_signer_keys(signer: &SignerWithStake) -> StdResult<(Option<OpCert>, ProtocolSignerVerificationKey, Option<ProtocolSignerVerificationKeySignature)> {
        let operational_certificate = match &signer.operational_certificate {
            Some(operational_certificate) => {
                key_decode_hex(operational_certificate)?
            }
            _ => None,
        };
        let verification_key =
            key_decode_hex(&signer.verification_key)?;
        let kes_signature = match &signer.verification_key_signature {
            Some(verification_key_signature) => {
                Some(key_decode_hex(verification_key_signature?))
            }
            _ => None,
        };
        
        Ok((operational_certificate, verification_key, kes_signature))
    }
}

pub struct SingleSigner;

pub struct MultiSigner;

#[cfg(test)]
mod test {
    use super::*;

    fn test() {
        // let multi_signer = SignerBuilder::new(stake_distrib).build_multi_signer(other_param);
    }
}
