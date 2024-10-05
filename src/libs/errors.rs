use eyre::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("{message}")]
    AlreadyExists { message: String },
    #[error("Failed to write {id}: {message}")]
    FailedToWriteStorage { id: String, message: String },
    #[error("Bucket not found: {message}")]
    BucketNotFound { message: String },
}

pub type AppResult<T, E = eyre::Report> = Result<T, E>;
