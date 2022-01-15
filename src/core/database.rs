use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

use crate::config::Config;
use crate::core::errors::ServerError;
use crate::models::secrets::database::DatabaseSecrets;

pub type DatabasePool = Pool<Postgres>;

pub async fn connect(config: &Config, database_secrets: &DatabaseSecrets) -> Result<DatabasePool, ServerError> {
    let dsn = if config.use_local {
        database_secrets.local_dsn()
    } else {
        database_secrets.dsn()
    };

    info!("Database DSN: {}", &dsn);

    PgPoolOptions::new()
        .max_connections(config.database_connections)
        .max_lifetime(Duration::from_secs(config.database_lifetime))
        .connect(&dsn)
        .await
        .map_err(|err| ServerError::ConnectingToDatabase(err.to_string()))
}
