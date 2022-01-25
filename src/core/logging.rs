use anyhow::Result;
use tracing::error;
use tracing::subscriber::set_global_default;
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::errors::internal::ServerError;

#[allow(clippy::cognitive_complexity)]
pub fn init() -> Result<()> {
    if let Err(error) = LogTracer::init() {
        error!("Failed to initialize log tracer: {error:#?}");
        return Err(ServerError::GenericError.into());
    }

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap();

    let stdout_subscriber = tracing_subscriber::fmt::layer();

    let subscriber = Registry::default()
        .with(env_filter)
        .with(ErrorLayer::default())
        .with(stdout_subscriber);

    if let Err(error) = set_global_default(subscriber) {
        error!("Failed to initialize log subscriber: {error:#?}");
        return Err(ServerError::GenericError.into());
    }

    Ok(())
}
