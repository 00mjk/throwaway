use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::{Duration, TimeZone, Utc};

use crate::errors::core::ServerError;
use crate::extractors::validated_json::ValidatedJson;
use crate::middleware::basic_authentication::BasicAuthenticationLayer;
use crate::middleware::token_authentication::TokenAuthenticationLayer;
use crate::models::claims::Claims;
use crate::models::database::profile::Profile;
use crate::models::request::token::TokenRequest;
use crate::models::response::token::TokenResponse;
use crate::models::response::token_info::TokenInfoResponse;
use crate::TokenService;

#[must_use]
pub fn routes() -> Router {
    let basic_auth_routes = Router::new()
        .route("/token", post(token_post))
        .route_layer(BasicAuthenticationLayer::default());

    let token_auth_routes = Router::new()
        .route("/token/test", post(token_test_get))
        .route("/token/info", get(token_info_get))
        .route_layer(TokenAuthenticationLayer::default());

    Router::new()
        .merge(basic_auth_routes)
        .merge(token_auth_routes)
}

pub async fn token_post(
    ValidatedJson(token_request): ValidatedJson<TokenRequest>,
    Extension(token_service): Extension<TokenService>,
    Extension(profile): Extension<Profile>,
) -> Result<impl IntoResponse, ServerError> {
    let lifespan_duration = Duration::seconds(token_request.lifespan as i64);
    let token = token_service.generate(profile.profile_id, lifespan_duration, token_request.attributes)?;

    let token_response = TokenResponse {
        token,
    };

    Ok((StatusCode::CREATED, Json(token_response)))
}

pub async fn token_test_get() -> Result<impl IntoResponse, ServerError> {
    Ok(StatusCode::CREATED)
}

pub async fn token_info_get(Extension(claims): Extension<Claims>) -> Result<impl IntoResponse, ServerError> {
    let token_info_response = TokenInfoResponse {
        issued_at: Utc
            .timestamp(claims.iat, 0)
            .to_rfc3339(),
        expires_at: Utc
            .timestamp(claims.exp, 0)
            .to_rfc3339(),
        attributes: claims.attributes,
    };

    Ok((StatusCode::CREATED, Json(token_info_response)))
}
