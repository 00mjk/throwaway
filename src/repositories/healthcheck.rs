use tracing::error;

use crate::database::DatabasePool;
use crate::errors::internal::ServerError;
use crate::Cache;

#[derive(Clone)]
pub struct HealthcheckRepository {
    pub database: DatabasePool,
    pub cache: Cache,
}

impl HealthcheckRepository {
    pub const fn new(database: DatabasePool, cache: Cache) -> Self {
        Self {
            database,
            cache,
        }
    }

    // No idea why this is flagging the linter up, assuming it's a library bug?
    #[allow(clippy::missing_panics_doc)]
    pub async fn table_exists(&self, table_schema: &str, table_name: &str) -> Result<(String, bool), ServerError> {
        let identifier = format!("{table_schema}.{table_name}");
        let cache_key = format!("table_exists_{identifier}");
        if let Some(result) = self.cache.get(&cache_key).await {
            return Ok((identifier, result));
        }

        // language=sql
        let query = sqlx::query!(
            r#"
            SELECT EXISTS(
              SELECT 1
              FROM information_schema.tables
              WHERE table_schema = $1 AND table_name = $2
            )
            "#,
            table_schema,
            table_name,
        );

        match query.fetch_one(&self.database).await {
            Ok(output) => {
                let exists: bool = output.exists.unwrap();

                self.cache
                    .set(&cache_key, exists, 60)
                    .await;

                Ok((identifier, exists))
            }
            Err(error) => {
                error!("Could not find existing Profile by email: {error:#?}");
                Err(ServerError::GenericError)
            }
        }
    }
}
