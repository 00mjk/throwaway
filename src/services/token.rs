use chrono::{Duration, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

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

    pub fn generate(&self, profile_id: Uuid) -> String {
        let now = Utc::now();
        let duration = Duration::minutes(15);
        let expiration = now + duration;

        let claims = Claims {
            iss: ISS.to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
            sub: profile_id,
        };

        let encoding_key = EncodingKey::from_secret(self.jwt_password.as_bytes());
        let token = encode(&Header::default(), &claims, &encoding_key);

        token.unwrap()
    }

    pub fn decode(&self, token: &str) -> Claims {
        let validation = Validation {
            iss: Some(ISS.to_string()),
            ..Validation::default()
        };

        let decoding_key = DecodingKey::from_secret(self.jwt_password.as_bytes());
        let token_data = decode::<Claims>(token, &decoding_key, &validation);

        return match token_data {
            Ok(token) => token.claims,
            Err(err) => {
                match err.kind() {
                    ErrorKind::InvalidToken => {
                        panic!("Token is invalid");
                    }
                    ErrorKind::InvalidIssuer => {
                        panic!("Issuer is invalid");
                    }
                    _ => {
                        panic!("Some other errors");
                    }
                }
            }
        };
    }
}
