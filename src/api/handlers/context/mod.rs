use axum::{extract::FromRef, http::StatusCode, response::IntoResponse};

use crate::{libs::errors::Errors, storage::Storage};

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        match self {
            Errors::AlreadyExists { message } => {
                tracing::error!(err.message = %message);
                StatusCode::CONFLICT.into_response()
            }
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
    fn from_ref(_: &Storage) -> Self {
        ()
    }
}
