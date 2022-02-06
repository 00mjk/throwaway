use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("[Validation]: {0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("[Serde]: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("[SQL]: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("[SQL Migration]: {0}")]
    SqlxMigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("[Cache]: {0}")]
    CacheError(#[from] deadpool_redis::CreatePoolError),

    #[error("[Env Config]: {0}")]
    EnvConfigError(#[from] envconfig::Error),

    #[error("[Log Env]: {0}")]
    LogEnvError(#[from] tracing_subscriber::filter::FromEnvError),

    #[error("[Log Tracer]: {0}")]
    LogTracerError(#[from] tracing::log::SetLoggerError),

    #[error("[Log Subscriber]: {0}")]
    LogSubscriberError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error("[Argon]: {0}")]
    ArgonPasswordError(#[from] argon2::password_hash::Error),

    #[error("[JWT]: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("[Vault Client Config]: {0}")]
    VaultClientConfigError(#[from] vaultrs::client::VaultClientSettingsBuilderError),

    #[error("[Vault Client]: {0}")]
    VaultClientError(#[from] vaultrs::error::ClientError),

    #[error("[Hyper]: {0}")]
    HyperError(#[from] hyper::Error),

    #[error("[Axum Header]: {0}")]
    AxumTypedHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),

    #[error("[Axum Extension]: {0}")]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),

    #[error("[Axum Form]: {0}")]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),

    #[error("[Axum Json]: {0}")]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error("[Credentials]: {0}")]
    CredentialsError(#[from] crate::errors::credentials::CredentialsError),

    #[error("[Profile]: {0}")]
    ProfileError(#[from] crate::errors::profile::ProfileError),

    #[error("Token: {0}")]
    TokenError(#[from] crate::errors::token::TokenError),

    /// FIXME: This should only be used short term, and replaced with proper error handling in time
    #[error("WIP Error")]
    WIPError,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServerError::ValidationError(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            ServerError::CredentialsError(error) => {
                return error.into_response();
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
