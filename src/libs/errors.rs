use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Errors {
    #[snafu(whatever, display("{message}"))]
    Whatever { message: String },
}

pub type AppResult<T, E = snafu::Whatever> = Result<T, E>;
