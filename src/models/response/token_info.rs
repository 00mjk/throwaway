use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct TokenInfoResponse {
    pub issued_at: String,
    pub expires_at: String,
}
