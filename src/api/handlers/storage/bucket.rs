use axum::Json;
use tracing::instrument;

use crate::{
    api::models::{bucket::BucketResponse, Kind, ListKind, ListResponse},
    libs::errors::{AppResult, Errors},
};

#[instrument]
pub async fn list_buckets() -> AppResult<Json<ListResponse<BucketResponse>>, Errors> {
    Ok(Json(ListResponse {
        kind: ListKind::Buckets,
        items: vec![BucketResponse {
            kind: Kind::Bucket,
            ..Default::default()
        }],
        prefixes: vec![],
    }))
}
