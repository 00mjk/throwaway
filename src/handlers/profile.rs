use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::patch;
use axum::{Json, Router};
use tracing::instrument;

use crate::errors::core::ServerError;
use crate::extractors::validated_json::ValidatedJson;
use crate::middleware::token_authentication::TokenAuthenticationLayer;
use crate::models::database::profile::Profile;
use crate::models::request::profile::ProfilePatchRequest;
use crate::models::response::profile::ProfilePatchResponse;
use crate::services::profile::ProfileService;

#[must_use]
pub fn routes() -> Router {
    Router::new()
        .route("/profile", patch(profile_patch))
        .route_layer(TokenAuthenticationLayer::default())
}

// TODO: How does PATCH work with passwords and redacted fields?
// TODO: How to prevent a PATCH being abused as a PUT? Just check request only 1 field?
// FIXME: Can we remove the printing of errs but having 'err' in instrument instead?
#[instrument(skip(profile_service), level = "info", err)]
pub async fn profile_patch(
    ValidatedJson(profile_patch_request): ValidatedJson<ProfilePatchRequest>,
    Extension(profile_service): Extension<ProfileService>,
    Extension(profile): Extension<Profile>,
) -> Result<impl IntoResponse, ServerError> {
    // FIXME: Should this logic be pushed into a service?
    return match profile_patch_request {
        ProfilePatchRequest {
            name: Some(name), ..
        } => {
            let updated_profile = profile_service
                .update_name(profile.profile_id, name)
                .await?;

            let response = ProfilePatchResponse {
                // FIXME: Where do we keep static strings like this? "constants"
                message: "Updated Profile name".to_string(),
                code: 200,
                profile: Some(updated_profile.to_public()),
            };

            Ok((StatusCode::OK, Json(response)))
        }
        ProfilePatchRequest {
            email: Some(email), ..
        } => {
            let updated_profile = profile_service
                .update_email(profile.profile_id, email)
                .await?;

            let response = ProfilePatchResponse {
                message: "Updated Profile email".to_string(),
                code: 200,
                profile: Some(updated_profile.to_public()),
            };

            Ok((StatusCode::OK, Json(response)))
        }
        // ProfilePatchRequest {
        //     country: Some(country),
        //     ..
        // } => {
        //     if country == profile.country {
        //         let response = ProfilePatchResponse {
        //             message: "Country already set".to_string(),
        //             code: 409,
        //             profile: None,
        //         };
        //
        //         return Ok((StatusCode::CONFLICT, Json(response)));
        //     }
        //
        //     profile.country = country.clone();
        //     profile_service
        //         .update(profile.clone())
        //         .await?;
        //
        //     let response = ProfilePatchResponse {
        //         message: "Updated Country password".to_string(),
        //         code: 200,
        //         profile: Some(profile.to_public()),
        //     };
        //
        //     Ok((StatusCode::OK, Json(response)))
        // }
        // ProfilePatchRequest {
        //     timezone: Some(timezone),
        //     ..
        // } => {
        //     if timezone == profile.timezone {
        //         let response = ProfilePatchResponse {
        //             message: "Timezone already set".to_string(),
        //             code: 409,
        //             profile: None,
        //         };
        //
        //         return Ok((StatusCode::CONFLICT, Json(response)));
        //     }
        //
        //     profile.timezone = timezone.clone();
        //     profile_service
        //         .update(profile.clone())
        //         .await?;
        //
        //     let response = ProfilePatchResponse {
        //         message: "Updated Timezone password".to_string(),
        //         code: 200,
        //         profile: Some(profile.to_public()),
        //     };
        //
        //     Ok((StatusCode::OK, Json(response)))
        // }
        _ => {
            // This could be a scenario when a user tries to patch multiple fields (or no fields).
            // TODO: Can we validate patches at the extractor level? Ensure validated json with only 1 field set e.g.
            // TODO: Can we just return an Err here?
            let response = ProfilePatchResponse {
                message: "Invalid request".to_string(),
                code: 422,
                profile: None,
            };

            Ok((StatusCode::UNPROCESSABLE_ENTITY, Json(response)))
        }
    };
}
