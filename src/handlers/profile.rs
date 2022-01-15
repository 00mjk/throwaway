use axum::extract::Extension;
use axum::http::StatusCode;
use axum::routing::patch;
use axum::{Json, Router};

use crate::core::errors::ServerError;
use crate::extractors::token_claims::TokenClaims;
use crate::extractors::validated_json::ValidatedJson;
use crate::models::claims::Claims;
use crate::models::request::profile::ProfilePatchRequest;
use crate::models::response::profile::ProfilePatchResponse;
use crate::services::profile::ProfileService;
use crate::PasswordService;

pub fn routes() -> Router {
    Router::new().route("/profile", patch(profile_patch))
}

pub async fn profile_patch(
    TokenClaims(claims): TokenClaims<Claims>,
    ValidatedJson(profile_patch_request): ValidatedJson<ProfilePatchRequest>,
    Extension(profile_service): Extension<ProfileService>,
    Extension(password_service): Extension<PasswordService>,
) -> Result<(StatusCode, Json<ProfilePatchResponse>), ServerError> {
    let profile_id = claims.sub;
    let mut profile = profile_service.read(profile_id).await?;

    let mut should_update = false;

    if let Some(name) = profile_patch_request.name {
        if name != profile.name {
            profile.name = name;
            should_update = true;
        }
    }

    if let Some(email) = profile_patch_request.email {
        if email != profile.email {
            profile.email = email;
            should_update = true;
        }
    }

    if let Some(password) = profile_patch_request.password {
        // HOW TO CHECK HAS CHANGED?
        let password_hash = password_service
            .hash(&password.clone())
            .await?;

        profile.password = password_hash;
        should_update = true;
    }

    if let Some(country) = profile_patch_request.country {
        if country != profile.country {
            profile.country = country;
            should_update = true;
        }
    }

    if let Some(timezone) = profile_patch_request.timezone {
        if timezone != profile.timezone {
            profile.timezone = timezone;
            should_update = true;
        }
    }

    if should_update {
        profile_service.update(profile).await?;
    }

    let response = ProfilePatchResponse {
        name: None,
        email: None,
        password: None,
        country: None,
        timezone: None,
    };

    Ok((StatusCode::OK, Json(response)))
}
