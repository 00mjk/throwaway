use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::{error, info};

use crate::config::Config;
use crate::errors::internal::ServerError;
use crate::models::secrets::database::DatabaseSecrets;

pub type DatabasePool = Pool<Postgres>;

pub async fn connect(config: &Config, database_secrets: &DatabaseSecrets) -> Result<DatabasePool, ServerError> {
    let dsn = if config.use_local {
        database_secrets.local_dsn()
    } else {
        database_secrets.dsn()
    };

    info!("Database DSN: {dsn}");

    PgPoolOptions::new()
        .max_connections(config.database_connections)
        .max_lifetime(Duration::from_secs(config.database_lifetime))
        .connect(&dsn)
        .await
        .map_err(|error| {
            error!("Failed to connect to Postgres: {error:#?}");
            ServerError::SqlxError(error)
        })
}
