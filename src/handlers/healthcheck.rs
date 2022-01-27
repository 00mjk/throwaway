use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

use crate::errors::internal::ServerError;

pub fn routes() -> Router {
    Router::new().route("/health", get(healthcheck))
}

pub async fn healthcheck() -> Result<StatusCode, ServerError> {
    Ok(StatusCode::OK)
}
