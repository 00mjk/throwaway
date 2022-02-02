use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateProfile {
    pub name: String,
    pub email: String,
    pub password: String,
    pub country: String,
    pub timezone: String,
}

impl CreateProfile {
    pub fn new() -> Self {
        let id = OsRng.next_u64();

        Self {
            name: format!("Test {id}"),
            email: format!("test-{id}@domain.test"),
            password: "test-password".to_string(),
            country: "UK".to_string(),
            timezone: "GMT".to_string(),
        }
    }
}
