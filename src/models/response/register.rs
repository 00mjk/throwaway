use serde::Serialize;
use sqlx::types::Uuid;

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    pub profile_id: Uuid,
}
