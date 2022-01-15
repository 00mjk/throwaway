use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ProfilePatchResponse {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub country: Option<String>,
    pub timezone: Option<String>,
}
