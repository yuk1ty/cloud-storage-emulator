use std::convert::Infallible;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use axum_garde::WithValidation;
use tracing::instrument;

use crate::{
    api::models::{
        bucket::{BucketResponse, CreateBucket, GetBucketParams},
        ListResponse,
    },
    flows::bucket::{create_new_bucket, find_bucket, list},
    libs::errors::{AppResult, Errors},
    storage::Storage,
};

#[instrument(skip(storage))]
pub async fn list_buckets(
    State(storage): State<Storage>,
) -> AppResult<Json<ListResponse<BucketResponse>>, Infallible> {
    list(storage).await.map(ListResponse::from).map(Json)
}

#[instrument(skip(storage))]
pub async fn get_bucket(
    Path(bucket): Path<String>,
    State(storage): State<Storage>,
    Query(params): Query<GetBucketParams>,
) -> AppResult<Json<Option<BucketResponse>>, Infallible> {
    find_bucket(storage, bucket)
        .await
        .map(|result| result.map(BucketResponse::from))
        .map(Json)
}

#[instrument(skip(storage))]
pub async fn create_bucket(
    State(storage): State<Storage>,
    WithValidation(req): WithValidation<Json<CreateBucket>>,
) -> AppResult<Json<BucketResponse>, Errors> {
    create_new_bucket(storage, req.into_inner())
        .await
        .map(BucketResponse::from)
        .map(Json)
}
