use rand_chacha::ChaCha20Rng;
use rand_core::{SeedableRng, RngCore};
use mithril::msp::{MspPk, MspMvk, MspSig, Msp};
use mithril::stm::Stake;
use mithril::atms::{Avk, Asig, AtmsError, MTValue};
use std::collections::HashSet;
use ark_bls12_377::Bls12_377;
use blake2::{Blake2b, Digest};
use mithril::merkle_tree::MTHashLeaf;
use ark_std::rand::prelude::IteratorRandom;

type C = Bls12_377;
type H = Blake2b;
type A = Msp<C>;
type F = <H as MTHashLeaf<MTValue<MspMvk<C>>>>::F;


fn main() {
    let total_nr_players = 10;
    let players = (0..total_nr_players);

    println!("Initialised.");
    println!("=======================");
    println!("Total players: {:?}", total_nr_players);
    println!();

    let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
    let mut msg = [0; 16];

    let mut parties_keypair = Vec::with_capacity(total_nr_players);
    let mut parties_pks = Vec::with_capacity(total_nr_players);
    for _ in 0..total_nr_players {
        let keypair = Msp::<C>::gen(&mut rng);
        parties_pks.push(keypair.1.clone());
        parties_keypair.push(keypair);
    }

    println!("Public keys generated (and broadcast): ");
    println!("-------------------------------------- ");
    for (index, key) in parties_pks.iter().enumerate() {
        println!("Key of party # {}: {:x}", index,
                 H::digest(&key.mvk.to_bytes()));
    }
    println!();

    // Epoch 1.
    // For the sake of the example, assume that we define the set of signers to be a
    // subset of the total players. We choose such a subset.
    let nr_parties_1 = 5;
    let threshold = 4;
    let qualified_signers = players.clone().choose_multiple(&mut rng, nr_parties_1);

    // This particular example does not consider `Stake`, and therefore gives `Stake = 1` for every
    // player.
    let mut qp_keys = Vec::with_capacity(nr_parties_1);
    for &qp in qualified_signers.iter() {
        qp_keys.push((parties_pks[qp].clone(), 1));
    }

    println!("Beggining of Epoch 1");
    println!("--------------------");
    println!("Nr signers: {}", nr_parties_1);
    println!("Threshold: {}", threshold);
    println!("Indices of qualified signers: {:?}", qualified_signers);

    println!("ATMs single key generation");
    println!();
    // With this data, we can generate the ATMs single key.
    let avk_key = Avk::<A, H>::new::<F>(&qp_keys, threshold).expect("We assume proofs of possession are valid.");


    // Now the parties can sign messages. No need of interaction.
    rng.fill_bytes(&mut msg);
    println!("Message to sign: {:?}", msg);
    let mut signatures = Vec::with_capacity(nr_parties_1);
    for &i in qualified_signers.iter() {
        signatures.push((parties_keypair[i].1.mvk, Msp::sig(&parties_keypair[i].0, &msg)));
    }
    println!();
    println!("Signature aggregation (may be performed by an untrusted party)");
    let aggr_sig = Asig::new(&avk_key, &signatures[..]);

    // aggregated signatures can be verified using the ATMs single key.
    assert!(aggr_sig.verify(&msg, &avk_key).is_ok());

    // A different epoch begins when the signers (or the stake) changes.
    // Beginning of epoch 2
    let nr_parties_2 = 7;
    let threshold = 5;
    let qualified_signers = players.choose_multiple(&mut rng, nr_parties_2);

    // This particular example does not consider `Stake`, and therefore gives `Stake = 1` for every
    // player.
    let mut qp_keys = Vec::with_capacity(nr_parties_2);
    for &qp in qualified_signers.iter() {
        qp_keys.push((parties_pks[qp].clone(), 1));
    }

    println!("Beggining of Epoch 2");
    println!("--------------------");
    println!("Nr signers: {}", nr_parties_2);
    println!("Threshold: {}", threshold);
    println!("Indices of qualified signers: {:?}", qualified_signers);

    println!("ATMs single key generation (needs to be recomputed)");
    println!();
    // With this data, we can generate the ATMs single key.
    let avk_key = Avk::<A, H>::new::<F>(&qp_keys, threshold).expect("We assume proofs of possession are valid.");


    // Now the parties can sign messages. No need of interaction.
    rng.fill_bytes(&mut msg);
    println!("Message to sign: {:?}", msg);
    let mut signatures = Vec::with_capacity(nr_parties_2);
    for i in qualified_signers {
        signatures.push((parties_keypair[i].1.mvk, Msp::sig(&parties_keypair[i].0, &msg)));
    }
    println!();
    println!("Signature aggregation (may be performed by an untrusted party).");
    let aggr_sig = Asig::new(&avk_key, &signatures[..]);

    // aggregated signatures can be verified using the ATMs single key.
    assert!(aggr_sig.verify(&msg, &avk_key).is_ok());
}