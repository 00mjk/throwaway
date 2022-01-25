use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Profile not found")]
    NotFound,

    #[error("Profile could not be updated")]
    UpdateFailure,
}

impl IntoResponse for ProfileError {
    fn into_response(self) -> Response {
        let status = match self {
            ProfileError::NotFound => (StatusCode::NOT_FOUND),
            ProfileError::UpdateFailure => (StatusCode::BAD_REQUEST),
        };

        let body = Json(json!({
            "status": status.as_str(),
            "message": format!("{self}"),
        }));

        (status, body).into_response()
    }
}
