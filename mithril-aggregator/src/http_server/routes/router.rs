use crate::http_server::routes::{
    certificate_routes, epoch_routes, signatures_routes, signer_routes, snapshot_routes,
};
use crate::http_server::SERVER_BASE_PATH;
use crate::DependencyManager;
use std::sync::Arc;
use warp::http::Method;
use warp::Filter;

/// Routes
pub fn routes(
    dependency_manager: Arc<DependencyManager>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS]);

    warp::any().and(warp::path(SERVER_BASE_PATH)).and(
        certificate_routes::routes(dependency_manager.clone())
            .or(snapshot_routes::routes(dependency_manager.clone()))
            .or(signer_routes::routes(dependency_manager.clone()))
            .or(signatures_routes::routes(dependency_manager.clone()))
            .or(epoch_routes::routes(dependency_manager))
            .with(cors),
    )
}
