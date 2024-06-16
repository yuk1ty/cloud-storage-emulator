use commands::CommandArgs;
use snafu::ResultExt;
use tokio::net::TcpListener;

use crate::{api::routes::routes, libs::errors::AppResult, storage::Storage};

pub mod commands;
pub mod context;

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

        let CommandArgs { host, port, scheme } = &self.cfg;

        tracing::info!(
            server.cfg.host=%host,
            server.cfg.port=%port,
            server.cfg.mode=%scheme,
            "Starting server..."
        );

        let router = routes().with_state(Storage::new());
        let listener = TcpListener::bind(format!("{host}:{port}"))
            .await
            .whatever_context("Unexpected error has been occurred in constructing TcpListener")?;
        axum::serve(listener, router)
            .await
            .whatever_context("Failed to start the server!")
    }
}
