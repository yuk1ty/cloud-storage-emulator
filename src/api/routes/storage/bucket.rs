use axum::{routing::get, Router};

use crate::{api::handlers::storage::bucket::list_buckets, storage::Storage};

pub fn bucket_routes() -> Router<Storage> {
    Router::new().route("/b", get(list_buckets))
}
