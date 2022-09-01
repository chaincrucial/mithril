use crate::dependency::{
    CertificatePendingStoreWrapper, CertificateStoreWrapper, MultiSignerWrapper,
    SnapshotStoreWrapper,
};
use crate::{Configuration, DependencyManager};
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

/// With snapshot store middleware
pub fn with_snapshot_store(
    dependency_manager: Arc<DependencyManager>,
) -> impl Filter<Extract = (SnapshotStoreWrapper,), Error = Infallible> + Clone {
    warp::any().map(move || dependency_manager.snapshot_store.clone())
}

/// With certificate store middleware
pub fn with_certificate_store(
    dependency_manager: Arc<DependencyManager>,
) -> impl Filter<Extract = (CertificateStoreWrapper,), Error = Infallible> + Clone {
    warp::any().map(move || dependency_manager.certificate_store.clone())
}

/// With certificate pending store
pub(crate) fn with_certificate_pending_store(
    dependency_manager: Arc<DependencyManager>,
) -> impl Filter<Extract = (CertificatePendingStoreWrapper,), Error = Infallible> + Clone {
    warp::any().map(move || dependency_manager.certificate_pending_store.clone())
}

/// With multi signer middleware
pub fn with_multi_signer(
    dependency_manager: Arc<DependencyManager>,
) -> impl Filter<Extract = (MultiSignerWrapper,), Error = Infallible> + Clone {
    warp::any().map(move || dependency_manager.multi_signer.clone())
}

/// With config middleware
pub fn with_config(
    dependency_manager: Arc<DependencyManager>,
) -> impl Filter<Extract = (Configuration,), Error = Infallible> + Clone {
    warp::any().map(move || dependency_manager.config.clone())
}
