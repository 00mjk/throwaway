use anyhow::Error;
use axum::http::Request;
use hyper::body::to_bytes;
use hyper::{Body, Client, Method};
use rand_core::{OsRng, RngCore};
use serde_json::{from_slice, json, Value};

#[tokio::test]
async fn test_e2e() -> Result<(), Error> {
    let random_id = OsRng.next_u64();
    let client = Client::new();

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
        .uri("http://0.0.0.0:8000/register")
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(Body::from(register_request_body.to_string()))?;

    let register_response = client.request(register_request).await?;
    let register_response_bytes = to_bytes(register_response.into_body()).await?;

    let register_response_body: Value = from_slice(&register_response_bytes)?;
    println!("Register Response: {register_response_body:#?}");

    let profile_id = register_response_body["profile_id"]
        .as_str()
        .unwrap();
    assert!(!profile_id.is_empty());

    // Part 2: Fetch Token
    let token_request_body: Value = json!({
        "email": email,
        "password": password,
    });

    println!("Token Request: {token_request_body:#?}");
    let token_request = Request::builder()
        .uri("http://0.0.0.0:8000/token")
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(Body::from(token_request_body.to_string()))?;

    let token_response = client.request(token_request).await?;
    let token_response_bytes = to_bytes(token_response.into_body()).await?;

    let token_response_body: Value = from_slice(&token_response_bytes)?;
    println!("Token Response: {token_response_body:#?}");

    let token = token_response_body["token"]
        .as_str()
        .unwrap();
    assert!(!token.is_empty());

    // Part 3: Patch Profile
    let patch_request_body: Value = json!({
        "name": "Updated",
    });

    println!("Patch Request: {patch_request_body:#?}");
    let patch_request = Request::builder()
        .uri("http://0.0.0.0:8000/profile")
        .method(Method::PATCH)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::from(patch_request_body.to_string()))?;

    let patch_response = client.request(patch_request).await?;
    let patch_response_bytes = to_bytes(patch_response.into_body()).await?;

    let patch_response_body: Value = from_slice(&patch_response_bytes)?;
    println!("Patch Response: {patch_response_body:#?}");

    let patch_response_country = patch_response_body["profile"]["name"]
        .as_str()
        .unwrap();
    assert_eq!(patch_response_country, "Updated");

    Ok(())
}
