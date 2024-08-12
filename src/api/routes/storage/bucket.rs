use axum::{
    routing::{get, patch, post, put},
    Router,
};

use crate::{
    api::handlers::storage::bucket::{get_bucket, insert_bucket, list_buckets, update_bucket},
    storage::Storage,
};

pub fn bucket_routes() -> Router<Storage> {
    Router::new()
        .route("/b", get(list_buckets))
        .route("/b/:bucket", get(get_bucket))
        .route("/b", post(insert_bucket))
        // Google doesn't recommend this method but for just in case.
        .route("/b/:bucket", put(update_bucket))
        .route("/b/:bucket", patch(update_bucket))
}
