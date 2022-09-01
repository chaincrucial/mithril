//! Test data builders for Mithril Core types, for testing purpose.

use super::{genesis::*, key_encode_hex, types::*};
use crate::{
    entities::{Certificate, Epoch, PartyId, ProtocolMessage, ProtocolMessagePartKey, Stake},
    fake_data,
};

use crate::certificate_chain::CertificateGenesisProducer;
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};
use std::collections::HashMap;
use std::{cmp::min, sync::Arc};

/// Instantiate a [ProtocolMessage] using fake data, use this for tests only.
pub fn setup_message() -> ProtocolMessage {
    let mut protocol_message = ProtocolMessage::new();
    protocol_message.set_message_part(
        ProtocolMessagePartKey::SnapshotDigest,
        "message_to_sign_123".to_string(),
    );
    protocol_message.set_message_part(
        ProtocolMessagePartKey::NextAggregateVerificationKey,
        "next-avk-123".to_string(),
    );
    protocol_message
}

/// Instantiate a [ProtocolParameters], use this for tests only.
pub fn setup_protocol_parameters() -> ProtocolParameters {
    ProtocolParameters {
        m: 100,
        k: 5,
        phi_f: 0.65,
    }
}

/// Instantiate a list of protocol signers, use this for tests only.
pub fn setup_signers(
    total: u64,
    protocol_parameters: &ProtocolParameters,
) -> Vec<(
    ProtocolPartyId,
    ProtocolStake,
    ProtocolSignerVerificationKey,
    ProtocolSigner,
    ProtocolInitializer,
)> {
    let stake_seed = [0u8; 32];
    let mut stake_rng = ChaCha20Rng::from_seed(stake_seed);
    let signers = (0..total)
        .into_iter()
        .map(|party_id| {
            let stake = 1 + stake_rng.next_u64() % 999;
            let party_id = format!("{:<032}", party_id);
            (party_id as PartyId, stake as Stake)
        })
        .collect::<Vec<_>>();
    setup_signers_from_parties(&signers, protocol_parameters)
}

/// Instantiate a list of protocol signers based on the given [PartyId] / [Stake] pairs, use this for tests only.
pub fn setup_signers_from_parties(
    party_with_stake: &[(PartyId, Stake)],
    protocol_parameters: &ProtocolParameters,
) -> Vec<(
    ProtocolPartyId,
    ProtocolStake,
    ProtocolSignerVerificationKey,
    ProtocolSigner,
    ProtocolInitializer,
)> {
    let signers = party_with_stake
        .iter()
        .map(|(party_id, stake)| {
            let protocol_initializer_seed: [u8; 32] = party_id.as_bytes()[..32].try_into().unwrap();
            let mut protocol_initializer_rng = ChaCha20Rng::from_seed(protocol_initializer_seed);
            let protocol_initializer: ProtocolInitializer = ProtocolInitializer::setup(
                *protocol_parameters,
                *stake,
                &mut protocol_initializer_rng,
            );
            (
                party_id.clone() as ProtocolPartyId,
                *stake as ProtocolStake,
                protocol_initializer,
            )
        })
        .collect::<Vec<_>>();

    let mut key_registration = ProtocolKeyRegistration::init();
    signers.iter().for_each(|(_, stake, protocol_initializer)| {
        key_registration
            .register(*stake, protocol_initializer.verification_key())
            .expect("key registration should have succeeded");
    });
    let closed_key_registration = key_registration.close();
    signers
        .into_iter()
        .map(|(party_id, stake, protocol_initializer)| {
            (
                party_id,
                stake,
                protocol_initializer.verification_key(),
                protocol_initializer
                    .clone()
                    .new_signer(closed_key_registration.clone())
                    .unwrap(),
                protocol_initializer,
            )
        })
        .collect::<_>()
}

/// Instantiate a Genesis Signer and its associated Verifier
pub fn setup_genesis() -> (ProtocolGenesisSigner, ProtocolGenesisVerifier) {
    let genesis_signer = ProtocolGenesisSigner::create_test_genesis_signer();
    let genesis_verifier = genesis_signer.create_genesis_verifier();
    (genesis_signer, genesis_verifier)
}

/// Instantiate a certificate chain, use this for tests only.
pub fn setup_certificate_chain(
    total_certificates: u64,
    certificates_per_epoch: u64,
) -> (Vec<Certificate>, ProtocolGenesisVerifier) {
    let genesis_signer = ProtocolGenesisSigner::create_test_genesis_signer();
    let genesis_verifier = genesis_signer.create_genesis_verifier();
    let genesis_producer = CertificateGenesisProducer::new(Arc::new(genesis_signer));
    let protocol_parameters = setup_protocol_parameters();
    let mut epochs = (1..total_certificates + 2)
        .into_iter()
        .map(|i| match certificates_per_epoch {
            0 => panic!("expected at least 1 certificate per epoch"),
            1 => Epoch(i),
            _ => Epoch(i / certificates_per_epoch + 1),
        })
        .collect::<Vec<_>>();
    let signers_by_epoch = epochs
        .clone()
        .into_iter()
        .map(|epoch| {
            (
                epoch,
                setup_signers(min(2 + epoch.0, 5), &protocol_parameters),
            )
        })
        .collect::<HashMap<_, _>>();
    let clerk_for_signers = |signers: &[(_, _, _, ProtocolSigner, _)]| -> ProtocolClerk {
        let first_signer = &signers.first().unwrap().3;
        ProtocolClerk::from_signer(first_signer)
    };
    let avk_for_signers =
        |signers: &[(_, _, _, ProtocolSigner, _)]| -> ProtocolAggregateVerificationKey {
            let clerk = clerk_for_signers(signers);
            clerk.compute_avk()
        };
    let avk_encode =
        |avk: &ProtocolAggregateVerificationKey| -> String { key_encode_hex(avk).unwrap() };
    epochs.pop();
    let certificates = epochs
        .into_iter()
        .enumerate()
        .map(|(i, epoch)| {
            let immutable_file_number = i as u64 * 10;
            let digest = format!("digest{}", i);
            let certificate_hash = format!("certificate_hash-{}", i);
            let signers = signers_by_epoch.get(&epoch).unwrap();
            let next_signers = signers_by_epoch.get(&(epoch + 1)).unwrap();
            let avk = avk_for_signers(signers);
            let next_avk = avk_for_signers(next_signers);
            let mut fake_certificate = fake_data::certificate(certificate_hash);
            fake_certificate.beacon.epoch = epoch;
            fake_certificate.beacon.immutable_file_number = immutable_file_number;
            fake_certificate
                .protocol_message
                .set_message_part(ProtocolMessagePartKey::SnapshotDigest, digest);
            fake_certificate.protocol_message.set_message_part(
                ProtocolMessagePartKey::NextAggregateVerificationKey,
                avk_encode(&next_avk),
            );
            fake_certificate.aggregate_verification_key = avk_encode(&avk);
            fake_certificate.signed_message = fake_certificate.protocol_message.compute_hash();
            fake_certificate.previous_hash = "".to_string();
            match i {
                0 => {
                    fake_certificate = genesis_producer
                        .create_genesis_certificate(
                            fake_certificate.metadata.protocol_parameters,
                            fake_certificate.beacon,
                            next_avk,
                        )
                        .unwrap()
                }
                _ => {
                    let mut single_signatures = Vec::new();
                    signers.iter().for_each(|(_, _, _, protocol_signer, _)| {
                        if let Some(signature) =
                            protocol_signer.sign(fake_certificate.signed_message.as_bytes())
                        {
                            single_signatures.push(signature);
                        }
                    });
                    let clerk = clerk_for_signers(signers);
                    let multi_signature = clerk
                        .aggregate(
                            &single_signatures,
                            fake_certificate.signed_message.as_bytes(),
                        )
                        .unwrap();
                    fake_certificate.multi_signature = key_encode_hex(&multi_signature).unwrap();
                    fake_certificate.genesis_signature = "".to_string()
                }
            }
            fake_certificate
        })
        .collect::<Vec<Certificate>>();
    let mut certificates_new: Vec<Certificate> = Vec::new();
    certificates
        .iter()
        .enumerate()
        .for_each(|(i, certificate)| {
            let mut certificate_new = certificate.clone();
            if i > 0 {
                if let Some(previous_certificate) = certificates_new.get(i - 1) {
                    certificate_new.previous_hash = previous_certificate.compute_hash();
                }
            }
            certificate_new.hash = certificate_new.compute_hash();
            certificates_new.push(certificate_new);
        });
    certificates_new.reverse();
    (certificates_new, genesis_verifier)
}
