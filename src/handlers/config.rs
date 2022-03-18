use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::errors::core::ServerError;
use crate::models::response::config::ConfigResponse;
use crate::Config;

#[must_use]
pub fn routes() -> Router {
    Router::new().route("/config", get(config_get))
}

pub async fn config_get(Extension(config): Extension<Config>) -> Result<impl IntoResponse, ServerError> {
    let response = ConfigResponse {
        config,
    };

    Ok((StatusCode::OK, Json(response)))
}
