use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Config: {0}")]
    Config(String),

    #[error("Logging: {0}")]
    Logging(String),
    #[error("Logging subscriber: {0}")]
    LoggingSubscriber(String),

    #[error("Connecting to database: {0}")]
    ConnectingToDatabase(String),
    #[error("Migrating database: {0}")]
    DatabaseMigration(String),

    #[error("Connecting to cache: {0}")]
    ConnectingToCache(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServerError::Config(message)
            | ServerError::Logging(message)
            | ServerError::LoggingSubscriber(message)
            | ServerError::ConnectingToDatabase(message)
            | ServerError::ConnectingToCache(message)
            | ServerError::Internal(message)
            | ServerError::DatabaseMigration(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            ServerError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            ServerError::ValidationError(value) => {
                let message = format!("Input validation error: [{:?}]", value).replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(json!({
            "status": status.as_str(),
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
