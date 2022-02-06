use serde::Serialize;

use crate::attributes::core::Attributes;

#[derive(Serialize, Debug)]
pub struct TokenInfoResponse {
    pub issued_at: String,
    pub expires_at: String,
    pub attributes: Option<Attributes>,
}
