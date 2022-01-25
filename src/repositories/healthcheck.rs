use anyhow::Result;

use crate::database::DatabasePool;
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

    pub async fn table_exists(&self, table_schema: &str, table_name: &str) -> Result<(String, bool)> {
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

        let result: bool = query
            .fetch_one(&self.database)
            .await
            .unwrap()
            .exists
            .unwrap();

        self.cache
            .set(&cache_key, result, 60)
            .await;

        Ok((identifier, result))
    }
}
