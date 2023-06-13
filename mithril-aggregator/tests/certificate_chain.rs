mod test_extensions;

use mithril_aggregator::{Configuration, VerificationKeyStorer};
use mithril_common::{
    chain_observer::ChainObserver, entities::ProtocolParameters, test_utils::MithrilFixtureBuilder,
};
use test_extensions::{utilities::get_test_dir, RuntimeTester, SignedEntityTypeDiscriminants};

#[tokio::test]
async fn certificate_chain() {
    let protocol_parameters = ProtocolParameters {
        k: 5,
        m: 100,
        phi_f: 0.95,
    };
    let configuration = Configuration {
        protocol_parameters: protocol_parameters.clone(),
        data_stores_directory: get_test_dir("certificate_chain").join("aggregator.sqlite3"),
        ..Configuration::new_sample()
    };
    let mut tester = RuntimeTester::build(configuration).await;
    let observer = tester.observer.clone();

    comment!("Create signers & declare stake distribution");
    let fixture = MithrilFixtureBuilder::default()
        .with_signers(5)
        .with_protocol_parameters(protocol_parameters.clone())
        .build();
    let signers = fixture.signers_fixture();
    let mut signers_with_stake = fixture.signers_with_stake();
    tester
        .chain_observer
        .set_signers(signers_with_stake.clone())
        .await;
    tester
        .deps_builder
        .build_dependency_container()
        .await
        .unwrap()
        .prepare_for_genesis(
            signers_with_stake.clone(),
            signers_with_stake.clone(),
            &protocol_parameters,
        )
        .await;
    let mut current_epoch = tester
        .chain_observer
        .get_current_epoch()
        .await
        .unwrap()
        .unwrap();

    comment!("Boostrap the genesis certificate, {:?}", current_epoch);
    tester.register_genesis_certificate(&signers).await.unwrap();

    comment!("Increase immutable number");
    tester.increase_immutable_number().await.unwrap();

    comment!("Start the runtime state machine & send send first single signatures");
    cycle!(tester, "ready");
    cycle!(tester, "signing");
    tester.register_signers(&signers).await.unwrap();
    cycle_err!(tester, "signing");
    let signed_entity_type = observer
        .get_current_signed_entity_type(SignedEntityTypeDiscriminants::MithrilStakeDistribution)
        .await
        .unwrap();
    tester
        .send_single_signatures(&signed_entity_type, &signers)
        .await
        .unwrap();

    comment!("The state machine should issue a certificate for the MithrilStakeDistribution");
    cycle!(tester, "ready");
    let (last_certificates, snapshots) =
        tester.get_last_certificates_and_snapshots().await.unwrap();
    assert_eq!((2, 0), (last_certificates.len(), snapshots.len()));

    comment!("The state machine should get back to signing to sign CardanoImmutableFilesFull");
    tester.increase_immutable_number().await.unwrap();
    cycle!(tester, "signing");
    let signed_entity_type = observer
        .get_current_signed_entity_type(SignedEntityTypeDiscriminants::CardanoImmutableFilesFull)
        .await
        .unwrap();
    tester
        .send_single_signatures(&signed_entity_type, &signers)
        .await
        .unwrap();
    comment!("The state machine should issue a certificate for the CardanoImmutableFilesFull");
    cycle!(tester, "ready");
    let (last_certificates, snapshots) =
        tester.get_last_certificates_and_snapshots().await.unwrap();
    assert_eq!((3, 1), (last_certificates.len(), snapshots.len()));

    comment!(
        "Increase immutable number to do a second CardanoImmutableFilesFull certificate for this epoch, {:?}",
        current_epoch
    );
    tester.increase_immutable_number().await.unwrap();
    cycle!(tester, "signing");
    let signed_entity_type = observer
        .get_current_signed_entity_type(SignedEntityTypeDiscriminants::CardanoImmutableFilesFull)
        .await
        .unwrap();
    tester
        .send_single_signatures(&signed_entity_type, &signers)
        .await
        .unwrap();
    cycle!(tester, "ready");
    let (last_certificates, snapshots) =
        tester.get_last_certificates_and_snapshots().await.unwrap();
    assert_eq!((4, 2), (last_certificates.len(), snapshots.len()));
    assert_eq!(
        (
            last_certificates[0].beacon.immutable_file_number,
            last_certificates[0].beacon.epoch,
        ),
        (
            last_certificates[1].beacon.immutable_file_number + 1,
            last_certificates[1].beacon.epoch,
        ),
        "Only the immutable_file_number should have changed"
    );
    assert_eq!(
        &last_certificates[0].previous_hash, &last_certificates[3].hash,
        "A new certificate on the same epoch should be linked to the first certificate of the current epoch"
    );

    comment!("Change stake distribution");
    for (i, signer) in signers_with_stake.iter_mut().enumerate() {
        signer.stake += (i * 1000) as u64;
    }
    let new_signers = tester
        .update_stake_distribution(signers_with_stake)
        .await
        .unwrap();

    comment!(
        "Increase epoch, triggering stake distribution update, Next epoch: {:?}",
        current_epoch + 1
    );
    let new_epoch = tester.increase_epoch().await.unwrap();
    current_epoch = new_epoch;
    cycle!(tester, "idle");
    cycle!(tester, "ready");
    cycle!(tester, "signing");

    comment!(
        "Checking that no signers are registered for the next epoch since they did not register"
    );
    let next_epoch_verification_keys = tester
        .deps_builder
        .get_verification_key_store()
        .await
        .unwrap()
        .get_verification_keys(new_epoch + 1)
        .await
        .expect("get_verification_keys should not fail");
    assert_eq!(
        None, next_epoch_verification_keys,
        "After a new epoch no signers should be registered for the next epoch"
    );

    comment!(
        "Signers register & send signatures, the new certificate should be link to the first of the previous epoch"
    );
    tester.register_signers(&new_signers).await.unwrap();
    let signed_entity_type = observer
        .get_current_signed_entity_type(SignedEntityTypeDiscriminants::MithrilStakeDistribution)
        .await
        .unwrap();
    tester
        .send_single_signatures(&signed_entity_type, &signers)
        .await
        .unwrap();
    cycle!(tester, "ready");
    let (last_certificates, snapshots) =
        tester.get_last_certificates_and_snapshots().await.unwrap();
    assert_eq!((5, 2), (last_certificates.len(), snapshots.len()));
    assert_eq!(
        (
            last_certificates[0].beacon.immutable_file_number,
            last_certificates[0].beacon.epoch,
        ),
        (
            last_certificates[1].beacon.immutable_file_number,
            last_certificates[1].beacon.epoch + 1,
        ),
        "Only the epoch should have changed"
    );
    assert_eq!(
        &last_certificates[0].previous_hash, &last_certificates[4].hash,
        "The new epoch certificate should be linked to the first certificate of the previous epoch"
    );
    assert_eq!(
        last_certificates[0].metadata.get_stake_distribution(),
        last_certificates[2].metadata.get_stake_distribution(),
        "The stake distribution update should only be taken into account at the next epoch",
    );

    comment!(
        "Increase epoch & immutable, stake distribution updated at {} will now be signed into the certificate chain, allowing it to be used in the next epoch, Next epoch: {:?}",
        current_epoch,
        current_epoch + 1
    );
    current_epoch = tester.increase_epoch().await.unwrap();
    tester.increase_immutable_number().await.unwrap();
    cycle!(tester, "idle");
    cycle!(tester, "ready");
    tester.register_signers(&signers).await.unwrap();
    cycle!(tester, "signing");

    let signed_entity_type = observer
        .get_current_signed_entity_type(SignedEntityTypeDiscriminants::MithrilStakeDistribution)
        .await
        .unwrap();
    tester
        .send_single_signatures(&signed_entity_type, &signers)
        .await
        .unwrap();
    cycle!(tester, "ready");

    comment!(
        "Increase epoch & immutable, stake distribution updated at {} should be signed in the new certificate, Next epoch: {:?}",
        current_epoch - 1,
        current_epoch + 1
    );
    current_epoch = tester.increase_epoch().await.unwrap();
    tester.increase_immutable_number().await.unwrap();
    cycle!(tester, "idle");
    cycle!(tester, "ready");
    tester.register_signers(&signers).await.unwrap();
    cycle!(tester, "signing");
    let signed_entity_type = observer
        .get_current_signed_entity_type(SignedEntityTypeDiscriminants::MithrilStakeDistribution)
        .await
        .unwrap();

    tester
        .send_single_signatures(&signed_entity_type, &new_signers)
        .await
        .unwrap();
    cycle!(tester, "ready");

    let (last_certificates, snapshots) =
        tester.get_last_certificates_and_snapshots().await.unwrap();
    assert_eq!((7, 2), (last_certificates.len(), snapshots.len()));
    assert_eq!(
        (
            last_certificates[0].beacon.immutable_file_number,
            last_certificates[0].beacon.epoch,
        ),
        (
            last_certificates[1].beacon.immutable_file_number + 1,
            last_certificates[1].beacon.epoch + 1,
        ),
        "Both the epoch & immutable file number should have change"
    );

    assert_eq!(
        &last_certificates[0].previous_hash, &last_certificates[1].hash,
        "The new epoch certificate should be linked to the first certificate of the previous epoch"
    );
    assert_ne!(
        last_certificates[0].metadata.get_stake_distribution(),
        last_certificates[2].metadata.get_stake_distribution(),
        "The stake distribution update should have been applied for this epoch",
    );

    comment!(
        "Increase immutable number to do a CardanoImmutableFilesFull certificate for this epoch, {:?}",
        current_epoch
    );
    tester.increase_immutable_number().await.unwrap();
    cycle!(tester, "signing");

    let signed_entity_type = observer
        .get_current_signed_entity_type(SignedEntityTypeDiscriminants::CardanoImmutableFilesFull)
        .await
        .unwrap();
    tester
        .send_single_signatures(&signed_entity_type, &new_signers)
        .await
        .unwrap();
    cycle!(tester, "ready");

    comment!(
        "A CardanoImmutableFilesFull, linked to the MithrilStakeDistribution of the current epoch, should have been created, {:?}",
        current_epoch
    );
    let (last_certificates, snapshots) =
        tester.get_last_certificates_and_snapshots().await.unwrap();
    assert_eq!((8, 3), (last_certificates.len(), snapshots.len()));
    assert_eq!(
        &last_certificates[0].previous_hash, &last_certificates[1].hash,
        "The CardanoImmutableFilesFull certificate should be linked to the MithrilStakeDistribution certificate of the current epoch"
    );
}
