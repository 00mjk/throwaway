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

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    AxumTypedHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),

    #[error(transparent)]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    ProfileError(#[from] crate::errors::profile::ProfileError),

    #[error(transparent)]
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
