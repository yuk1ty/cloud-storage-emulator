use axum::{extract::FromRef, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::{libs::errors::Errors, storage::Storage};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CloudStorageErrorResponse {
    pub status_code: u16,
    pub error_message: String,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        match self {
            Errors::AlreadyExists { message } => (
                StatusCode::CONFLICT,
                Json(CloudStorageErrorResponse {
                    status_code: StatusCode::CONFLICT.as_u16(),
                    error_message: message,
                }),
            )
                .into_response(),
            Errors::FailedToWriteStorage { id, message } => {
                tracing::error!(err.message = %message, id);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Errors::Whatever { message } => {
                tracing::error!(err.message = %message, "Unexpected error happened");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

// This is needed to work with `axum_garde`.
impl FromRef<Storage> for () {
    fn from_ref(_: &Storage) -> Self {}
}
