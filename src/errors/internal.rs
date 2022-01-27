use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

// FIXME: Plan out error types ahead of time, DatabaseErrors, CacheErrors, RepositoryErrors, ServiceErrors ...
// FIXME: Grep across the axum-examples project for ideas "enum.*Error"
// FIXME: Don't make every error take an input, log out core details.
// FIXME: Errors translate to response codes, so they're user facing

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("An error has occurred")]
    GenericError,

    #[error("Validation: {0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Serde: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("SQL: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Argon: {0}")]
    ArgonPasswordError(#[from] argon2::password_hash::Error),

    #[error("JWT: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Typed Header: {0}")]
    AxumTypedHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),

    #[error("Extension: {0}")]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),

    #[error("Form: {0}")]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error("Json: {0}")]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error("Profile: {0}")]
    ProfileError(#[from] crate::errors::profile::ProfileError),

    #[error("Token: {0}")]
    TokenError(#[from] crate::errors::token::TokenError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{:?}]", self);
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::ProfileError(error) => {
                return error.into_response();
            }
            ServerError::TokenError(error) => {
                return error.into_response();
            }
            _ => {
                error!("Internal server error: {self:#?}");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        let body = Json(json!({
            "status": status.as_str(),
            "message": message,
        }));

        (status, body).into_response()
    }
}
