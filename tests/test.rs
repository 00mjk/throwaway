use std::error::Error;
use std::net::{SocketAddr, TcpListener};

use axum::http::Request;
use base64::encode;
use hyper::body::to_bytes;
use hyper::{Body, Client, Method};
use rand_core::{OsRng, RngCore};
use serde_json::{from_slice, json, Value};
use throwaway::build_app;

#[tokio::test]
async fn test_integration() -> Result<(), Box<dyn Error>> {
    let random_id = OsRng.next_u64();
    let client = Client::new();

    // Start API
    let app = build_app().await?;

    let socket_address = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(socket_address)?;

    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // Part 1: Register
    let email = format!("test-{random_id}@domain.test");
    let password = "test-password";

    let register_request_body: Value = json!({
        "name": "Test",
        "email": email,
        "password": password,
        "country": "UK",
        "timezone": "GMT"
    });

    println!("Register Request: {register_request_body:#?}");
    let register_request = Request::builder()
        .uri(format!("http://{address}/register"))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(Body::from(register_request_body.to_string()))?;

    let register_response = client.request(register_request).await?;
    let register_response_headers = register_response.headers();
    println!("Register Response Headers: {register_response_headers:#?}");

    let register_response_bytes = to_bytes(register_response.into_body()).await?;

    let register_response_body: Value = from_slice(&register_response_bytes)?;
    println!("Register Response: {register_response_body:#?}");

    let profile_id = register_response_body["profile_id"]
        .as_str()
        .unwrap();
    assert!(!profile_id.is_empty(), "Profile ID is empty");

    // Part 2: Fetch Token
    let basic_authorization = format!("{email}:{password}");
    let basic_authorization_encoded = encode(format!("{email}:{password}"));

    println!("Token Request Headers: {basic_authorization:#?} -> {basic_authorization_encoded:#?}");
    let token_request = Request::builder()
        .uri(format!("http://{address}/token"))
        .method(Method::POST)
        .header("Authorization", format!("Basic {basic_authorization_encoded}"))
        .header("Content-Type", "application/json")
        .body(Body::empty())?;

    let token_response = client.request(token_request).await?;
    let token_response_headers = token_response.headers();
    println!("Token Response Headers: {token_response_headers:#?}");

    let token_response_bytes = to_bytes(token_response.into_body()).await?;

    let token_response_body: Value = from_slice(&token_response_bytes)?;
    println!("Token Response: {token_response_body:#?}");

    let token_authorization = token_response_body["token"]
        .as_str()
        .unwrap();
    assert!(!token_authorization.is_empty(), "Token is empty");

    // Part 3: Patch Profile
    let patch_request_body: Value = json!({
        "name": "Updated",
    });

    println!("Patch Request: {patch_request_body:#?}");
    let patch_request = Request::builder()
        .uri(format!("http://{address}/profile"))
        .method(Method::PATCH)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token_authorization}"))
        .body(Body::from(patch_request_body.to_string()))?;

    let patch_response = client.request(patch_request).await?;
    let patch_response_headers = patch_response.headers();
    println!("Patch Response Headers: {patch_response_headers:#?}");

    let patch_response_bytes = to_bytes(patch_response.into_body()).await?;

    let patch_response_body: Value = from_slice(&patch_response_bytes)?;
    println!("Patch Response: {patch_response_body:#?}");

    let patch_response_country = patch_response_body["profile"]["name"]
        .as_str()
        .unwrap();
    assert_eq!(patch_response_country, "Updated", "Patch request failed");

    Ok(())
}
