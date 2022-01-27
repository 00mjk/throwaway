use anyhow::Result;
use futures::future::try_join_all;
use tracing::error;

use crate::errors::internal::ServerError;
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
    // NOTE: Where do we handle the mapping then? Answer: The handler.
    pub async fn perform_healthcheck(&self) -> Result<HealthcheckResponse, ServerError> {
        let pg_database_exists = self
            .healthcheck_repository
            .table_exists("pg_catalog", "pg_database");

        let pg_index_exists = self
            .healthcheck_repository
            .table_exists("pg_catalog", "pg_index");

        let pg_statistic_exists = self
            .healthcheck_repository
            .table_exists("pg_catalog", "pg_statistic");

        let results = try_join_all([pg_database_exists, pg_index_exists, pg_statistic_exists]).await;
        match results {
            // FIXME: Don't build response here...
            Ok(output) => {
                Ok(HealthcheckResponse {
                    ok: true,
                    database: HealthDatabaseResponse {
                        tables: output.into_iter().collect(),
                    },
                })
            }
            Err(error) => {
                error!("Healthcheck error: {error:#?}");
                Err(ServerError::GenericError)
            }
        }
    }
}
