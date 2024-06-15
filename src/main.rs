use clap::Parser;
use libs::{errors::AppResult, telemetry::init_telemetry};
use server::{commands::CommandArgs, Server};

mod api;
mod libs;
mod repository;
mod server;

#[tokio::main]
async fn main() -> AppResult<()> {
    let cfg = CommandArgs::parse();
    init_telemetry()?;
    Server::new(cfg).bootstrap().await
}
