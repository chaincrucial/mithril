//! This module contains a struct to exchange snapshot information with the Aggregator

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use indicatif::ProgressBar;
use mithril_common::{
    entities::Snapshot,
    messages::{SnapshotListItemMessage, SnapshotListMessage, SnapshotMessage},
    StdResult,
};
use slog_scope::warn;
use thiserror::Error;

use super::AggregatorClient;

/// Error for the Snapshot client
#[derive(Error, Debug)]
pub enum SnapshotClientError {
    /// Download location does not work
    #[error("Could not find a working download location for the snapshot digest '{digest}', tried location: {{'{locations}'}}.")]
    NoWorkingLocation {
        /// given digest
        digest: String,

        /// list of locations tried
        locations: String,
    },
}

/// Aggregator client for the snapshot artifact
pub struct SnapshotClient {
    http_client: Arc<dyn AggregatorClient>,
}

impl SnapshotClient {
    /// constructor
    pub fn new(http_client: Arc<dyn AggregatorClient>) -> Self {
        Self { http_client }
    }

    /// Return a list of available snapshots
    pub async fn list(&self) -> StdResult<Vec<SnapshotListItemMessage>> {
        let url = "artifact/snapshots";
        let response = self.http_client.get_content(url).await?;
        let items = serde_json::from_str::<SnapshotListMessage>(&response)?;

        Ok(items)
    }

    /// Return a snapshot based on the given digest (list to get the digests)
    pub async fn show(&self, digest: &str) -> StdResult<SnapshotMessage> {
        let url = format!("artifact/snapshot/{}", digest);
        let response = self.http_client.get_content(&url).await?;
        let message = serde_json::from_str::<SnapshotMessage>(&response)?;

        Ok(message)
    }

    /// Download the snapshot identified by the given snapshot in the given directory
    pub async fn download(
        &self,
        snapshot: &Snapshot,
        download_dir: &Path,
        progress_bar: ProgressBar,
    ) -> StdResult<PathBuf> {
        let filepath = PathBuf::new()
            .join(download_dir)
            .join(format!("snapshot-{}.tar.gz", snapshot.digest));

        for url in snapshot.locations.as_slice() {
            if self.http_client.probe(url).await.is_ok() {
                match self
                    .http_client
                    .download(url, &filepath, progress_bar)
                    .await
                {
                    Ok(()) => return Ok(filepath),
                    Err(e) => {
                        warn!("Failed downloading snapshot from '{url}' Error: {e}.");
                        return Err(e.into());
                    }
                };
            }
        }

        let locations = snapshot.locations.join(", ");

        Err(SnapshotClientError::NoWorkingLocation {
            digest: snapshot.digest.clone(),
            locations,
        }
        .into())
    }
}
