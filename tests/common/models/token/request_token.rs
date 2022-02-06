use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenRequest {
    pub lifespan: usize,
    pub attributes: String,
}
