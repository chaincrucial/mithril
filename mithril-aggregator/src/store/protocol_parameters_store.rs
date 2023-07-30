use async_trait::async_trait;
use tokio::sync::RwLock;

use mithril_common::entities::{Epoch, ProtocolParameters};
use mithril_common::store::{adapter::StoreAdapter, StoreError, StorePruner};

type Adapter = Box<dyn StoreAdapter<Key = Epoch, Record = ProtocolParameters>>;

/// Store and get [protocol parameters][ProtocolParameters] for given epoch.
#[async_trait]
pub trait ProtocolParametersStorer: Sync + Send {
    /// Save the given `ProtocolParameter` for the given [Epoch].
    async fn save_protocol_parameters(
        &self,
        epoch: Epoch,
        protocol_parameters: ProtocolParameters,
    ) -> Result<Option<ProtocolParameters>, StoreError>;

    /// Get the saved `ProtocolParameter` for the given [Epoch] if any.
    async fn get_protocol_parameters(
        &self,
        epoch: Epoch,
    ) -> Result<Option<ProtocolParameters>, StoreError>;
}
/// `ProtocolParameter` store.
pub struct ProtocolParametersStore {
    adapter: RwLock<Adapter>,
    retention_len: Option<usize>,
}

impl ProtocolParametersStore {
    /// Create an instance of `ProtocolParameterStore`.
    pub fn new(adapter: Adapter, retention_len: Option<usize>) -> Self {
        Self {
            adapter: RwLock::new(adapter),
            retention_len,
        }
    }
}

#[async_trait]
impl StorePruner for ProtocolParametersStore {
    type Key = Epoch;
    type Record = ProtocolParameters;

    fn get_adapter(
        &self,
    ) -> &RwLock<Box<dyn StoreAdapter<Key = Self::Key, Record = Self::Record>>> {
        &self.adapter
    }

    fn get_max_records(&self) -> Option<usize> {
        self.retention_len
    }

    /// Pruning is deactivated on this store.
    async fn prune(&self) -> Result<(), StoreError> {
        Ok(())
    }
}

#[async_trait]
impl ProtocolParametersStorer for ProtocolParametersStore {
    async fn save_protocol_parameters(
        &self,
        epoch: Epoch,
        protocol_parameters: ProtocolParameters,
    ) -> Result<Option<ProtocolParameters>, StoreError> {
        let previous_protocol_parameters = self.adapter.read().await.get_record(&epoch).await?;
        self.adapter
            .write()
            .await
            .store_record(&epoch, &protocol_parameters)
            .await?;
        self.prune().await?;

        Ok(previous_protocol_parameters)
    }

    async fn get_protocol_parameters(
        &self,
        epoch: Epoch,
    ) -> Result<Option<ProtocolParameters>, StoreError> {
        let record = self.adapter.read().await.get_record(&epoch).await?;

        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mithril_common::{store::adapter::MemoryAdapter, test_utils::fake_data};

    fn setup_protocol_parameters(nb_epoch: u64) -> Vec<(Epoch, ProtocolParameters)> {
        let mut values: Vec<(Epoch, ProtocolParameters)> = Vec::new();

        for epoch in 1..=nb_epoch {
            let mut protocol_parameters = fake_data::protocol_parameters();
            protocol_parameters.k += epoch;
            values.push((Epoch(epoch), protocol_parameters));
        }
        values
    }

    fn init_store(nb_epoch: u64, retention_len: Option<usize>) -> ProtocolParametersStore {
        let values = setup_protocol_parameters(nb_epoch);

        let values = if !values.is_empty() {
            Some(values)
        } else {
            None
        };
        let adapter: MemoryAdapter<Epoch, ProtocolParameters> = MemoryAdapter::new(values).unwrap();

        ProtocolParametersStore::new(Box::new(adapter), retention_len)
    }

    #[tokio::test]
    async fn save_key_in_empty_store() {
        let protocol_parameters = setup_protocol_parameters(1);
        let store = init_store(0, None);
        let res = store
            .save_protocol_parameters(protocol_parameters[0].0, protocol_parameters[0].1.clone())
            .await
            .unwrap();

        assert!(res.is_none());
    }

    #[tokio::test]
    async fn update_protocol_parameters_in_store() {
        let protocol_parameters = setup_protocol_parameters(2);
        let store = init_store(1, None);
        let res = store
            .save_protocol_parameters(protocol_parameters[0].0, protocol_parameters[1].1.clone())
            .await
            .unwrap();

        assert_eq!(protocol_parameters[0].1, res.unwrap());
    }

    #[tokio::test]
    async fn get_protocol_parameters_for_empty_epoch() {
        let store = init_store(2, None);
        let res = store.get_protocol_parameters(Epoch(0)).await.unwrap();

        assert!(res.is_none());
    }

    #[tokio::test]
    async fn get_protocol_parameters_for_existing_epoch() {
        let store = init_store(2, None);
        let res = store.get_protocol_parameters(Epoch(1)).await.unwrap();

        assert!(res.is_some());
    }

    #[tokio::test]
    async fn test_retention_length() {
        let store = init_store(2, Some(2));
        let protocol_parameters = setup_protocol_parameters(3);
        let _ = store
            .save_protocol_parameters(Epoch(3), protocol_parameters[2].1.clone())
            .await
            .unwrap();
        assert!(store
            .get_protocol_parameters(Epoch(1))
            .await
            .unwrap()
            .is_some());
        let res = store.get_protocol_parameters(Epoch(2)).await.unwrap();
        assert!(res.is_some());
    }

    #[tokio::test]
    async fn test_retention_length_on_update() {
        let store = init_store(4, Some(2));
        let mut protocol_parameters = store
            .get_protocol_parameters(Epoch(1))
            .await
            .unwrap()
            .expect("There should be a protocol parameter for this epoch");
        protocol_parameters.k += 1;
        store
            .save_protocol_parameters(Epoch(1), protocol_parameters)
            .await
            .unwrap();
        assert!(store
            .get_protocol_parameters(Epoch(4))
            .await
            .unwrap()
            .is_some());
        assert!(store
            .get_protocol_parameters(Epoch(3))
            .await
            .unwrap()
            .is_some());
        assert!(store
            .get_protocol_parameters(Epoch(1))
            .await
            .unwrap()
            .is_some());
        assert!(store
            .get_protocol_parameters(Epoch(2))
            .await
            .unwrap()
            .is_some());
    }
}
