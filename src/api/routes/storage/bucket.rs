use axum::{routing::get, Router};

use crate::api::handlers::storage::bucket::list_buckets;

pub fn bucket_routes() -> Router {
    Router::new().route("/b", get(list_buckets))
}
