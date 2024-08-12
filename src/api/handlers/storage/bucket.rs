use std::convert::Infallible;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use axum_garde::WithValidation;
use tracing::instrument;

use crate::{
    api::models::{
        bucket::{
            BucketResponse, GetBucketParams, InsertBucket, InsertBucketParams, UpdateBucket,
            UpdateBucketParams,
        },
        ListResponse,
    },
    flows::bucket::{create_new_bucket, find_bucket, list, update_existing_bucket},
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
pub async fn insert_bucket(
    State(storage): State<Storage>,
    Query(_params): Query<InsertBucketParams>,
    WithValidation(req): WithValidation<Json<InsertBucket>>,
) -> AppResult<Json<BucketResponse>, Errors> {
    create_new_bucket(storage, req.into_inner())
        .await
        .map(BucketResponse::from)
        .map(Json)
}

#[instrument(skip(storage))]
pub async fn update_bucket(
    State(storage): State<Storage>,
    Path(bucket): Path<String>,
    Query(_params): Query<UpdateBucketParams>,
    WithValidation(req): WithValidation<Json<UpdateBucket>>,
) -> AppResult<Json<BucketResponse>, Errors> {
    update_existing_bucket(storage, bucket, req.into_inner())
        .await
        .map(BucketResponse::from)
        .map(Json)
}
