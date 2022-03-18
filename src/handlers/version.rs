use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

use crate::errors::core::ServerError;
use crate::models::response::version::VersionResponse;
use crate::VERSION;

#[must_use]
pub fn routes() -> Router {
    Router::new().route("/version", get(version_get))
}

pub async fn version_get() -> Result<impl IntoResponse, ServerError> {
    let response = VersionResponse {
        version: VERSION.to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}
