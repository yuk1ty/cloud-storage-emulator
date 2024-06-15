use axum::http::StatusCode;

use crate::libs::errors::{AppResult, Errors};

pub async fn list_buckets() -> AppResult<StatusCode, Errors> {
    Ok(StatusCode::NOT_IMPLEMENTED)
}
