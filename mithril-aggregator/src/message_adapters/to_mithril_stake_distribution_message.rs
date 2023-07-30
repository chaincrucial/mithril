use mithril_common::entities::{MithrilStakeDistribution, SignedEntity};
use mithril_common::messages::{
    MithrilStakeDistributionMessage, SignerWithStakeMessagePart, ToMessageAdapter,
};

/// Adapter to convert [MithrilStakeDistribution] to [MithrilStakeDistributionMessage] instances
pub struct ToMithrilStakeDistributionMessageAdapter;

impl ToMessageAdapter<SignedEntity<MithrilStakeDistribution>, MithrilStakeDistributionMessage>
    for ToMithrilStakeDistributionMessageAdapter
{
    /// Method to trigger the conversion
    fn adapt(from: SignedEntity<MithrilStakeDistribution>) -> MithrilStakeDistributionMessage {
        MithrilStakeDistributionMessage {
            epoch: from.artifact.epoch,
            signers_with_stake: SignerWithStakeMessagePart::from_signers(
                from.artifact.signers_with_stake,
            ),
            hash: from.artifact.hash,
            certificate_hash: from.certificate_id,
            created_at: from.created_at,
            protocol_parameters: from.artifact.protocol_parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use mithril_common::{
        entities::{Epoch, SignedEntityType},
        test_utils::fake_data,
    };

    use super::*;

    #[test]
    fn adapt_ok() {
        let mithril_stake_distribution = MithrilStakeDistribution {
            epoch: Epoch(1),
            signers_with_stake: fake_data::signers_with_stakes(2),
            hash: "hash-123".to_string(),
            protocol_parameters: fake_data::protocol_parameters(),
        };
        let signed_entity = SignedEntity {
            signed_entity_id: "id-1234".to_string(),
            signed_entity_type: SignedEntityType::MithrilStakeDistribution(Epoch(1)),
            certificate_id: "cert-hash-123".to_string(),
            artifact: mithril_stake_distribution,
            created_at: DateTime::<Utc>::default(),
        };
        let mithril_stake_distribution_message_expected = MithrilStakeDistributionMessage {
            epoch: Epoch(1),
            signers_with_stake: SignerWithStakeMessagePart::from_signers(
                fake_data::signers_with_stakes(2),
            ),
            hash: "hash-123".to_string(),
            certificate_hash: "cert-hash-123".to_string(),
            created_at: DateTime::<Utc>::default(),
            protocol_parameters: fake_data::protocol_parameters(),
        };
        let mithril_stake_distribution_message =
            ToMithrilStakeDistributionMessageAdapter::adapt(signed_entity);

        assert_eq!(
            mithril_stake_distribution_message_expected,
            mithril_stake_distribution_message
        );
    }
}
