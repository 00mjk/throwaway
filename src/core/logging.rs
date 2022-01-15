use tracing::subscriber::set_global_default;
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::core::errors::ServerError;

pub fn init() -> Result<(), ServerError> {
    LogTracer::init().map_err(|err| ServerError::Logging(err.to_string()))?;

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let stdout_subscriber = tracing_subscriber::fmt::layer();

    let subscriber = Registry::default()
        .with(env_filter)
        .with(ErrorLayer::default())
        .with(stdout_subscriber);

    set_global_default(subscriber).map_err(|err| ServerError::LoggingSubscriber(err.to_string()))?;

    Ok(())
}
