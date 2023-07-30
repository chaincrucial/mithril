use mithril_common::{
    entities::EpochSettings,
    messages::{EpochSettingsMessage, FromMessageAdapter},
};

/// Adapter to convert [EpochSettingsMessage] to [EpochSettings].
pub struct FromEpochSettingsAdapter;

impl FromMessageAdapter<EpochSettingsMessage, EpochSettings> for FromEpochSettingsAdapter {
    /// Method to convert.
    fn adapt(message: EpochSettingsMessage) -> EpochSettings {
        EpochSettings {
            epoch: message.epoch,
            protocol_parameters: message.protocol_parameters,
            next_protocol_parameters: message.next_protocol_parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_message() {
        let message = EpochSettingsMessage::dummy();
        let epoch = message.epoch;
        let epoch_settings = FromEpochSettingsAdapter::adapt(message);

        assert_eq!(epoch, epoch_settings.epoch);
    }
}
