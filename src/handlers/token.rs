use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};

use crate::errors::core::ServerError;
use crate::middleware::basic_authentication::BasicAuthenticationLayer;
use crate::middleware::token_authentication::TokenAuthenticationLayer;
use crate::models::database::profile::Profile;
use crate::models::response::token::TokenResponse;
use crate::TokenService;

pub fn routes() -> Router {
    let basic_auth_routes = Router::new()
        .route("/token", post(token_post))
        .route_layer(BasicAuthenticationLayer::default());

    let token_auth_routes = Router::new()
        .route("/token/test", post(token_test_post))
        .route_layer(TokenAuthenticationLayer::default());

    Router::new()
        .merge(basic_auth_routes)
        .merge(token_auth_routes)
}

pub async fn token_post(
    Extension(token_service): Extension<TokenService>,
    Extension(profile): Extension<Profile>,
) -> Result<impl IntoResponse, ServerError> {
    // Generate token
    let token = token_service.generate(profile.profile_id)?;
    let token_response = TokenResponse {
        token,
    };

    // Return
    Ok((StatusCode::CREATED, Json(token_response)))
}

pub async fn token_test_post() -> Result<impl IntoResponse, ServerError> {
    Ok(StatusCode::CREATED)
}
