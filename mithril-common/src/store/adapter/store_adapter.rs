use async_trait::async_trait;
use thiserror::Error;

/// [StoreAdapter] related errors
#[derive(Debug, Error)]
pub enum AdapterError {
    /// Generic [StoreAdapter] error.
    #[error("something wrong happened: {0}")]
    GeneralError(String),

    /// Error raised when the store initialization fails.
    #[error("problem creating the repository: {0}")]
    InitializationError(Box<dyn std::error::Error + Sync + Send>),

    /// Error raised when the opening of a IO stream fails.
    #[error("problem opening the IO stream: {0}")]
    OpeningStreamError(Box<dyn std::error::Error + Sync + Send>),

    /// Error raised when the parsing of a IO stream fails.
    #[error("problem parsing the IO stream: {0}")]
    ParsingDataError(Box<dyn std::error::Error + Sync + Send>),

    /// Error raised if a writting operation fails.
    #[error("problem writing on the adapter: {0}")]
    MutationError(Box<dyn std::error::Error + Sync + Send>),
}

/// Represent a way to store Key/Value pair data.
#[async_trait]
pub trait StoreAdapter: Sync + Send {
    /// The key type
    type Key;

    /// The record type
    type Record;

    /// Store the given `record`.
    async fn store_record(
        &mut self,
        key: &Self::Key,
        record: &Self::Record,
    ) -> Result<(), AdapterError>;

    /// Get the record stored using the given `key`.
    async fn get_record(&self, key: &Self::Key) -> Result<Option<Self::Record>, AdapterError>;

    /// Check if a record exist for the given `key`.
    async fn record_exists(&self, key: &Self::Key) -> Result<bool, AdapterError>;

    /// Get the last `n` records in the store
    async fn get_last_n_records(
        &self,
        how_many: usize,
    ) -> Result<Vec<(Self::Key, Self::Record)>, AdapterError>;

    /// remove values from store
    ///
    /// if the value exists it is returned by the adapter otherwise None is returned
    async fn remove(&mut self, key: &Self::Key) -> Result<Option<Self::Record>, AdapterError>;
}