use axum::{routing::get, Router};

use crate::handler::health::health_check;

pub fn routes() -> Router {
    let hc_router = Router::new().route("/hc", get(health_check));
    let storage_router = Router::new();
    Router::new()
        .merge(hc_router)
        .nest("/storage/v", storage_router)
}
