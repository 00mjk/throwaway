use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

use crate::core::errors::ServerError;

#[derive(Clone)]
pub struct PasswordService {
    argon2: Argon2<'static>,
}

impl PasswordService {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    pub async fn hash(&self, password: &str) -> Result<String, ServerError> {
        let salt = SaltString::generate(OsRng);

        let password_hash: PasswordHash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap();

        Ok(password_hash.to_string())
    }

    pub async fn verify(&self, password: &str, password_hash: &str) -> bool {
        let parsed_hash = PasswordHash::new(password_hash).unwrap();

        self.argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

impl Default for PasswordService {
    fn default() -> Self {
        Self::new()
    }
}
