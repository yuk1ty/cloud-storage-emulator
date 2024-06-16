use std::convert::Infallible;

use axum::{extract::State, Json};
use tracing::instrument;

use crate::{
    api::models::{bucket::BucketResponse, ListResponse},
    flows::bucket::list,
    libs::errors::AppResult,
    storage::Storage,
};

#[instrument(skip(storage))]
pub async fn list_buckets(
    State(storage): State<Storage>,
) -> AppResult<Json<ListResponse<BucketResponse>>, Infallible> {
    list(storage).await.map(ListResponse::from).map(Json)
}
