use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const ISS: &str = "throwaway";

/// <https://www.iana.org/assignments/jwt/jwt.xhtml#claims>
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Issuer
    pub iss: String,

    // Expiration Time
    pub exp: i64,

    // Issued At
    pub iat: i64,

    // Subject
    pub sub: Uuid,
}
