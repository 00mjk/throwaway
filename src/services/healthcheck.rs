use futures::future::try_join_all;

use crate::core::errors::ServerError;
use crate::models::response::healthcheck::{HealthDatabaseResponse, HealthcheckResponse};
use crate::repositories::healthcheck::HealthcheckRepository;

#[derive(Clone)]
pub struct HealthcheckService {
    healthcheck_repository: HealthcheckRepository,
}

impl HealthcheckService {
    pub const fn new(healthcheck_repository: HealthcheckRepository) -> Self {
        Self {
            healthcheck_repository,
        }
    }

    // NOTE: Should a service return a response model?
    // NOTE: Probably not.
    // NOTE: Where do we handle the mapping then?
    pub async fn perform_healthcheck(&self) -> HealthcheckResponse {
        let pg_database_exists = self
            .healthcheck_repository
            .table_exists("pg_catalog", "pg_database");

        let pg_index_exists = self
            .healthcheck_repository
            .table_exists("pg_catalog", "pg_index");

        let pg_statistic_exists = self
            .healthcheck_repository
            .table_exists("pg_catalog", "pg_statistic");

        let results = try_join_all([pg_database_exists, pg_index_exists, pg_statistic_exists])
            .await
            .map_err(|err| ServerError::NotFound(err.to_string()))
            .unwrap();

        HealthcheckResponse {
            ok: true,
            database: HealthDatabaseResponse {
                tables: results.into_iter().collect(),
            },
        }
    }
}
