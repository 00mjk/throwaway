use axum::body::Body;
use axum::http::{HeaderValue, Request};
use axum::response::IntoResponse;
use axum_extra::middleware::Next;

use crate::VERSION;

pub async fn version_header_middleware(request: Request<Body>, next: Next<Body>) -> impl IntoResponse {
    let mut response = next.run(request).await;

    response
        .headers_mut()
        .insert("X-Version", HeaderValue::from_static(VERSION));

    response
}
