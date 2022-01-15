use axum::extract::Extension;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};

use crate::core::errors::ServerError;
use crate::models::request::token::TokenRequest;
use crate::models::response::token::TokenResponse;
use crate::{ProfileService, TokenService};

pub fn routes() -> Router {
    Router::new().route("/token", post(token_post))
}

pub async fn token_post(
    Json(token_request): Json<TokenRequest>,
    Extension(profile_service): Extension<ProfileService>,
    Extension(token_service): Extension<TokenService>,
) -> Result<(StatusCode, Json<TokenResponse>), ServerError> {
    // Verify email and password are valid.
    let (valid_credentials, profile) = profile_service
        .verify_credentials(token_request.email, token_request.password)
        .await?;

    if !valid_credentials {
        return Err(ServerError::Internal("Invalid password".to_string()));
    }

    // Generate token
    let token = token_service.generate(profile.profile_id);
    let token_response = TokenResponse {
        token,
    };

    // Return
    Ok((StatusCode::CREATED, Json(token_response)))
}
