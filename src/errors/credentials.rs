use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CredentialsError {
    #[error("Credentials is invalid")]
    InvalidCredentials,
}

impl IntoResponse for CredentialsError {
    fn into_response(self) -> Response {
        let status = match self {
            CredentialsError::InvalidCredentials => (StatusCode::NOT_FOUND),
        };

        let body = Json(json!({
            "status": status.as_str(),
            "message": format!("{self}"),
        }));

        (status, body).into_response()
    }
}
