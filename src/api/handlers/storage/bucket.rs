use axum::http::StatusCode;
use tracing::instrument;

use crate::libs::errors::{AppResult, Errors};

#[instrument]
pub async fn list_buckets() -> AppResult<StatusCode, Errors> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}
