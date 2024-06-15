use axum::{routing::get, Router};

use crate::handler::storage::bucket::list_buckets;

pub fn bucket_routes() -> Router {
    Router::new().route("/b", get(list_buckets))
}
