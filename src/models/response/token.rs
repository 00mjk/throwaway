use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TokenResponse {
    pub token: String,
}
