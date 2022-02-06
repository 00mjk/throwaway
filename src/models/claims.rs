use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::attributes::core::Attributes;

pub const ISS: &str = "throwaway";

/// <https://www.iana.org/assignments/jwt/jwt.xhtml#claims>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    // Issuer
    pub iss: String,

    // Expiration Time
    pub exp: i64,

    // Issued At
    pub iat: i64,

    // Subject
    pub sub: Uuid,

    // Attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
}
