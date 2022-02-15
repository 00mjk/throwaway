use sqlx::migrate;
use sqlx::migrate::Migrator;

use crate::{DatabasePool, ServerError};

// Migrations embedded within the binary
static MIGRATOR: Migrator = migrate!("./sql/migrations");

pub async fn migrate(database_deployment: &DatabasePool) -> Result<(), ServerError> {
    MIGRATOR
        .run(database_deployment)
        .await
        .map_err(ServerError::SqlxMigrationError)?;

    Ok(())
}
