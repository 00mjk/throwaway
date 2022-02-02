use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PatchProfile {
    pub name: Option<String>,
    pub email: Option<String>,
    pub country: Option<String>,
    pub timezone: Option<String>,
}
