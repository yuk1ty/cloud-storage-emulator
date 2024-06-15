use libs::{errors::AppResult, telemetry::init_telemetry};
use routes::routes;
use snafu::ResultExt;
use tokio::net::TcpListener;

mod handler;
mod libs;
mod repository;
mod routes;

#[tokio::main]
async fn main() -> AppResult<()> {
    init_telemetry()?;

    tracing::info!("Starting the server on 0.0.0.0:4443 as http mode...");

    let router = routes();
    let listener = TcpListener::bind("0.0.0.0:4443")
        .await
        .whatever_context("Unexpected error has been occurred in constructing TcpListener")?;
    axum::serve(listener, router)
        .await
        .whatever_context("Failed to start the server!")
}
