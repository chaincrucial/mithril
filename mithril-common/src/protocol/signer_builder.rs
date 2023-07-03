use anyhow::{anyhow, Context, Result};
use mithril_stm::stm::StmParameters;
use thiserror::Error;

use crate::{
    crypto_helper::{
        key_decode_hex, OpCert, ProtocolAggregateVerificationKey, ProtocolAggregationError,
        ProtocolClerk, ProtocolClosedKeyRegistration, ProtocolKeyRegistration,
        ProtocolMultiSignature, ProtocolSignerVerificationKey,
        ProtocolSignerVerificationKeySignature, ProtocolSingleSignature, ProtocolStakeDistribution,
    },
    entities::{ProtocolMessage, ProtocolParameters, SignerWithStake, SingleSignatures},
};

pub struct SignerBuilder {
    protocol_parameters: ProtocolParameters,
    closed_key_registration: ProtocolClosedKeyRegistration,
}

#[derive(Debug, Error)]
pub enum SignerBuilderError {
    /// Error raised when the given list of signers to the builder is empty
    #[error("The list of signers must not be empty to create a signer builder.")]
    EmptySigners,
    #[error("The list of signers must not be empty to create a signer builder.")]
    NoSignerRegistered,
    // dedicated error or anyhow is enough ?
    // #[error("Invalid signers in stake distribution: '{0}'. It may be caused by an invalid Operational Certificate.")]
    // InvalidSigner(PartyId),
}

struct SignerKeys {
    pub operational_certificate: Option<OpCert>,
    pub verification_key: ProtocolSignerVerificationKey,
    pub kes_signature: Option<ProtocolSignerVerificationKeySignature>,
}

impl SignerKeys {
    fn from_signer_with_stake(signer: &SignerWithStake) -> Result<Self> {
        let operational_certificate = match &signer.operational_certificate {
            Some(operational_certificate) => key_decode_hex(operational_certificate)
                .map_err(|e| anyhow!(e))
                .with_context(|| format!("Could not decode operational certificate"))?,
            _ => None,
        };
        let verification_key = key_decode_hex(&signer.verification_key)
            .map_err(|e| anyhow!(e))
            .with_context(|| format!("Could not decode verification key"))?;
        let kes_signature = match &signer.verification_key_signature {
            Some(verification_key_signature) => Some(
                key_decode_hex(verification_key_signature)
                    .map_err(|e| anyhow!(e))
                    .with_context(|| format!("Could not decode verification key signature"))?,
            ),
            _ => None,
        };

        Ok(Self {
            operational_certificate,
            verification_key,
            kes_signature,
        })
    }
}

impl SignerBuilder {
    /// [SignerBuilder] constructor.
    pub fn new(
        registered_signers: &[SignerWithStake],
        protocol_parameters: &ProtocolParameters,
    ) -> Result<Self> {
        let stake_distribution = registered_signers
            .iter()
            .map(|s| s.into())
            .collect::<ProtocolStakeDistribution>();
        let mut key_registration = ProtocolKeyRegistration::init(&stake_distribution);
        let mut total_signers = 0;

        for signer in registered_signers {
            let signer_keys = SignerKeys::from_signer_with_stake(signer).with_context(|| {
                format!(
                    "Invalid signers in stake distribution: '{}'",
                    signer.party_id
                )
            })?;
            let kes_period = signer.kes_period;
            key_registration
                .register(
                    Some(signer.party_id.to_owned()),
                    signer_keys.operational_certificate,
                    signer_keys.kes_signature,
                    kes_period,
                    signer_keys.verification_key,
                )
                .with_context(|| {
                    format!("Registration failed for signer: '{}'", signer.party_id)
                })?;
            total_signers += 1;
        }

        match total_signers {
            0 => Err(SignerBuilderError::NoSignerRegistered.into()),
            _ => {
                let closed_registration = key_registration.close();
                Ok(Self {
                    protocol_parameters: protocol_parameters.clone(),
                    closed_key_registration: closed_registration,
                })
            }
        }
    }

    pub fn build_multi_signer(&self) -> MultiSigner {
        let stm_parameters = self.protocol_parameters.clone().into();
        let clerk =
            ProtocolClerk::from_registration(&stm_parameters, &self.closed_key_registration);

        MultiSigner {
            protocol_clerk: clerk,
            protocol_parameters: stm_parameters,
        }
    }
}

pub struct MultiSigner {
    protocol_clerk: ProtocolClerk,
    protocol_parameters: StmParameters,
}

impl MultiSigner {
    pub fn aggregate_single_signatures(
        &self,
        single_signatures: Vec<SingleSignatures>,
        protocol_message: ProtocolMessage,
    ) -> Result<Option<ProtocolMultiSignature>> {
        let protocol_signatures: Vec<ProtocolSingleSignature> = single_signatures
            .iter()
            .filter_map(|single_signature| single_signature.to_protocol_signature().ok())
            .collect::<Vec<_>>();

        match self.protocol_clerk.aggregate(
            &protocol_signatures,
            protocol_message.compute_hash().as_bytes(),
        ) {
            Ok(multi_signature) => Ok(Some(multi_signature)),
            Err(ProtocolAggregationError::NotEnoughSignatures(_actual, _expected)) => {
                // todo: log ?
                // warn!("Could not compute multi-signature: Not enough signatures. Got only {} out of {}.", actual, expected);
                Ok(None)
            }
            Err(err) => {
                Err(anyhow!(err)).with_context(|| "Error while aggregating single signatures")
            }
        }
    }

    pub fn compute_aggregate_verification_key(&self) -> ProtocolAggregateVerificationKey {
        self.protocol_clerk.compute_avk()
    }

    pub fn verify_single_signature(
        &self,
        message: &ProtocolMessage,
        single_signature: &SingleSignatures,
    ) -> Result<()> {
        let protocol_signature = single_signature
            .to_protocol_signature()
            .map_err(|e| anyhow!(e))
            .with_context(|| {
                format!(
                    "Error while decoding single signature for party: '{}'",
                    single_signature.party_id
                )
            })?;

        let avk = self.compute_aggregate_verification_key();

        // If there is no reg_party, then we simply received a signature from a non-registered
        // party, and we can ignore the request.
        let (vk, stake) = self
            .protocol_clerk
            .get_reg_party(&protocol_signature.signer_index)
            .ok_or_else(|| {
                anyhow!(format!(
                    "Unregistered party: '{}'",
                    single_signature.party_id
                ))
            })?;

        protocol_signature
            .verify(
                &self.protocol_parameters,
                &vk,
                &stake,
                &avk,
                message.compute_hash().as_bytes(),
            )
            .map_err(|e| anyhow!(e))
            .with_context(|| {
                format!(
                    "Invalid signature for party: '{}'",
                    single_signature.party_id
                )
            })?;

        Ok(())
    }
}

pub struct SingleSigner;

#[cfg(test)]
mod test {
    use super::*;

    fn test() {
        // let multi_signer = SignerBuilder::new(stake_distrib).build_multi_signer(other_param);
    }
}
