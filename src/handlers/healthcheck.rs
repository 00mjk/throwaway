use axum::extract::Extension;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};

use crate::errors::internal::ServerError;
use crate::models::response::healthcheck::HealthcheckResponse;
use crate::services::healthcheck::HealthcheckService;

pub fn routes() -> Router {
    Router::new().route("/health", get(healthcheck))
}

pub async fn healthcheck(
    Extension(healthcheck_service): Extension<HealthcheckService>,
) -> Result<(StatusCode, Json<HealthcheckResponse>), ServerError> {
    let healthcheck_response = healthcheck_service
        .perform_healthcheck()
        .await?;

    Ok((StatusCode::OK, Json(healthcheck_response)))
}
