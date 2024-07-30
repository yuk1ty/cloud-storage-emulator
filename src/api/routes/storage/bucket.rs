use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    api::handlers::storage::bucket::{get_bucket, insert_bucket, list_buckets},
    storage::Storage,
};

pub fn bucket_routes() -> Router<Storage> {
    Router::new()
        .route("/b", get(list_buckets))
        .route("/b/:bucket", get(get_bucket))
        .route("/b", post(insert_bucket))
}
