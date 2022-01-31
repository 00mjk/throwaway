use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use tower::layer::layer_fn;

use crate::errors::core::ServerError;
use crate::middleware::basic_authentication::BasicAuthentication;
use crate::models::database::profile::Profile;
use crate::models::response::token::TokenResponse;
use crate::TokenService;

pub fn routes() -> Router {
    Router::new()
        .route("/token", post(token_post))
        .route_layer(layer_fn(|inner| {
            BasicAuthentication {
                inner,
            }
        }))
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
