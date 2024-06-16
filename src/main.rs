use clap::Parser;
use libs::{errors::AppResult, telemetry::init_telemetry};
use server::{commands::CommandArgs, Server};

mod api;
mod flows;
mod kernel;
mod libs;
mod repositories;
mod server;
mod storage;

#[tokio::main]
async fn main() -> AppResult<()> {
    let cfg = CommandArgs::parse();
    init_telemetry()?;
    Server::new(cfg).bootstrap().await
}
