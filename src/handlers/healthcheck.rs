use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

use crate::errors::core::ServerError;

pub fn routes() -> Router {
    Router::new().route("/health", get(healthcheck))
}

pub async fn healthcheck() -> Result<impl IntoResponse, ServerError> {
    Ok((
        StatusCode::OK,
        Json(json!({
            "status": 200,
            "message": "All good",
        })),
    ))
}
