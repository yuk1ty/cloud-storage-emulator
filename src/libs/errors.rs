use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Errors {
    AlreadyExists {
        message: String,
    },
    FailedToWriteStorage {
        id: String,
        message: String,
    },
    BucketNotFound {
        message: String,
    },
    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
    },
}

pub type AppResult<T, E = snafu::Whatever> = Result<T, E>;
