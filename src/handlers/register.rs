use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use crate::errors::core::ServerError;
use crate::extractors::validated_json::ValidatedJson;
use crate::models::request::register::RegisterRequest;
use crate::models::response::register::RegisterResponse;
use crate::services::profile::ProfileService;

#[must_use]
pub fn routes() -> Router {
    Router::new().route("/register", post(register_post))
}

pub async fn register_post(
    ValidatedJson(register_request): ValidatedJson<RegisterRequest>,
    Extension(profile_service): Extension<ProfileService>,
) -> Result<impl IntoResponse, ServerError> {
    let profile_id = profile_service
        .create(register_request)
        .await?;

    let register_response = RegisterResponse {
        profile_id,
    };

    Ok((StatusCode::CREATED, Json(register_response)))
}
