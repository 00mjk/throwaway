use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::attributes::core::Attributes;
use crate::errors::core::ServerError;
use crate::models::claims::{Claims, ISS};
use crate::models::secrets::jwt::JwtSecrets;

#[derive(Clone)]
pub struct TokenService {
    jwt_password: String,
}

impl TokenService {
    pub fn new(jwt_secrets: &JwtSecrets) -> Self {
        Self {
            jwt_password: jwt_secrets.password.clone(),
        }
    }

    pub fn generate(
        &self,
        profile_id: Uuid,
        lifespan: Duration,
        attributes: Option<Attributes>,
    ) -> Result<String, ServerError> {
        let now = Utc::now();
        let expiration: DateTime<Utc> = now + lifespan;

        let claims = Claims {
            iss: ISS.to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
            sub: profile_id,
            attributes,
        };

        let encoding_key = EncodingKey::from_secret(self.jwt_password.as_bytes());
        let token = encode(&Header::default(), &claims, &encoding_key);

        token.map_err(ServerError::JwtError)
    }

    pub fn decode(&self, token: &str) -> Result<Claims, ServerError> {
        let validation = Validation {
            iss: Some(ISS.to_string()),
            ..Validation::default()
        };

        let decoding_key = DecodingKey::from_secret(self.jwt_password.as_bytes());
        let token_data = decode::<Claims>(token, &decoding_key, &validation);

        match token_data {
            Ok(token) => Ok(token.claims),
            Err(error) => Err(ServerError::JwtError(error)),
        }
    }
}
