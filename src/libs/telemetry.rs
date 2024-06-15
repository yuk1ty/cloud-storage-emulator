use snafu::ResultExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use super::errors::AppResult;

pub fn init_telemetry() -> AppResult<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    let subscriber = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()
        .whatever_context("Failed to initialize tracing_subscriber")?;
    Ok(())
}
