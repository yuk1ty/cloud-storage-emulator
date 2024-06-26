use axum::{routing::get, Router};
use storage::bucket::bucket_routes;

use crate::storage::Storage;

use super::handlers::health::health_check;

pub mod storage;

pub fn routes() -> Router<Storage> {
    let hc_router = Router::new().route("/hc", get(health_check));
    let storage_router = Router::new().merge(bucket_routes());
    Router::new()
        .merge(hc_router)
        .nest("/storage/v1", storage_router)
}
