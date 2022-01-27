use tracing::subscriber::set_global_default;
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::filter::FromEnvError;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::errors::internal::ServerError;

#[allow(clippy::cognitive_complexity)]
pub fn init() -> Result<(), ServerError> {
    LogTracer::init().map_err(ServerError::LogTracerError)?;

    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .map_err(|err| ServerError::LogEnvError(FromEnvError::from(err)))?;

    let stdout_subscriber = tracing_subscriber::fmt::layer();

    let subscriber = Registry::default()
        .with(env_filter)
        .with(ErrorLayer::default())
        .with(stdout_subscriber);

    set_global_default(subscriber).map_err(ServerError::LogSubscriberError)?;

    Ok(())
}
