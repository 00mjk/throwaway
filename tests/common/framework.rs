use std::net::SocketAddr;

use anyhow::Error;
use axum::http::Request;
use base64::encode;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper::{Body, Method};
use serde_json::to_string;
use tracing::log::debug;

use crate::common::models::response::APIResponse;
use crate::common::server::start_test_server;
use crate::{CreateProfile, PatchProfile};

pub struct Framework {
    client: Client<HttpConnector>,
    address: SocketAddr,
}

impl Framework {
    pub async fn new() -> Self {
        let address = start_test_server().await;

        Self {
            client: Client::new(),
            address,
        }
    }

    pub fn endpoint(&self, endpoint: &str) -> String {
        let address = self.address;
        format!("http://{address}{endpoint}")
    }

    pub async fn create_profile(&self, profile: &CreateProfile) -> Result<APIResponse, Error> {
        let body: String = to_string(&profile)?;
        debug!("Create Profile Request Body: {body:#?}");

        let request = Request::builder()
            .uri(self.endpoint("/register"))
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(Body::from(body))?;

        let response = self.client.request(request).await?;
        let result = APIResponse::new(response).await;
        Ok(result)
    }

    pub async fn patch_profile(&self, token: &str, profile: &PatchProfile) -> Result<APIResponse, Error> {
        let body: String = to_string(&profile)?;
        debug!("Patch Profile Request Body: {body:#?}");

        let request = Request::builder()
            .uri(self.endpoint("/profile"))
            .method(Method::PATCH)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {token}"))
            .body(Body::from(body))?;

        let response = self.client.request(request).await?;
        let result = APIResponse::new(response).await;
        Ok(result)
    }

    pub async fn request_token(&self, email: &str, password: &str) -> Result<APIResponse, Error> {
        let encoded_authorization = encode(format!("{email}:{password}"));
        let basic_authorization = format!("Basic {encoded_authorization}");
        debug!("Request Token Request Headers: {basic_authorization:#?}");

        let request = Request::builder()
            .uri(self.endpoint("/token"))
            .method(Method::POST)
            .header("Authorization", basic_authorization)
            .header("Content-Type", "application/json")
            .body(Body::empty())?;

        let response = self.client.request(request).await?;
        let result = APIResponse::new(response).await;
        Ok(result)
    }
}
