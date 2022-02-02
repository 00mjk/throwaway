use axum::body::Body;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use hyper::body::to_bytes;
use serde_json::{from_slice, Value};
use tracing::debug;

pub struct APIResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Value,
}

impl APIResponse {
    pub async fn new(response: Response<Body>) -> Self {
        let status = response.status();
        debug!("Response Status: {status:#?}");

        let headers = response.headers().clone();
        debug!("Response Headers: {headers:#?}");

        let body_bytes = to_bytes(response.into_body())
            .await
            .unwrap();

        let body: Value = from_slice(&body_bytes).unwrap();
        debug!("Response Body: {body:#?}");

        Self {
            status,
            headers,
            body,
        }
    }
}
