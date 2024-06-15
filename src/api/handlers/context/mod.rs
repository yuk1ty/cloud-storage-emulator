use axum::{http::StatusCode, response::IntoResponse};

use crate::libs::errors::Errors;

impl IntoResponse for Errors {
    fn into_response(self) -> axum::response::Response {
        match self {
            Errors::Whatever { message } => {
                tracing::error!(err.message = %message, "Unexpected error happened");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
