use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    api::handlers::storage::bucket::{create_bucket, list_buckets},
    storage::Storage,
};

pub fn bucket_routes() -> Router<Storage> {
    Router::new()
        .route("/b", get(list_buckets))
        .route("/b", post(create_bucket))
}
