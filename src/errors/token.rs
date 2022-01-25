use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Token is invalid")]
    InvalidToken,

    #[error("Token issuer is invalid")]
    InvalidIssuer,

    #[error("Token has expired")]
    ExpiredToken,

    #[error("Token unexpected error")]
    Generic,
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let status = match self {
            TokenError::InvalidToken => (StatusCode::NOT_FOUND),
            TokenError::InvalidIssuer | TokenError::ExpiredToken | TokenError::Generic => (StatusCode::BAD_REQUEST),
        };

        let body = Json(json!({
            "status": status.as_str(),
            "message": format!("{self}"),
        }));

        (status, body).into_response()
    }
}
