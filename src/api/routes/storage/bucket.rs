use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    api::handlers::storage::bucket::{create_bucket, get_bucket, list_buckets},
    storage::Storage,
};

pub fn bucket_routes() -> Router<Storage> {
    Router::new()
        .route("/b", get(list_buckets))
        .route("/b/:bucket", get(get_bucket))
        .route("/b", post(create_bucket))
}
