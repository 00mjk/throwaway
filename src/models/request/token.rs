use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRequest {
    pub email: String,
    pub password: String,
}
