use commands::{CommandArgs, Protocol};
use snafu::ResultExt;
use tokio::net::TcpListener;

use crate::{api::routes::routes, libs::errors::AppResult};

pub mod commands;

pub struct Server {
    cfg: CommandArgs,
}

impl Server {
    pub fn new(cfg: CommandArgs) -> Self {
        // TODO: Change args to specific `Config` stuff
        Self { cfg }
    }

    pub async fn bootstrap(&self) -> AppResult<()> {
        tracing::debug!(server.args = ?self.cfg, "Bootstrapping the server with given configuration");

        let host = self.cfg.host.clone().unwrap_or("0.0.0.0".into());
        let port = self.cfg.port.unwrap_or(8000);
        let scheme = self.cfg.scheme.clone().unwrap_or(Protocol::Http);

        tracing::info!(
            server.cfg.host=%host,
            server.cfg.port=%port,
            server.cfg.mode=%scheme,
            "Starting server..."
        );

        let router = routes();
        let listener = TcpListener::bind(format!("{host}:{port}"))
            .await
            .whatever_context("Unexpected error has been occurred in constructing TcpListener")?;
        axum::serve(listener, router)
            .await
            .whatever_context("Failed to start the server!")
    }
}
