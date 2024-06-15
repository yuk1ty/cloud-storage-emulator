use axum::{routing::get, Router};
use storage::bucket::bucket_routes;

use crate::handler::health::health_check;

mod storage;

pub fn routes() -> Router {
    let hc_router = Router::new().route("/hc", get(health_check));
    let storage_router = Router::new().merge(bucket_routes());
    Router::new()
        .merge(hc_router)
        .nest("/storage/v1", storage_router)
}
