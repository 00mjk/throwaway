use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct VersionResponse {
    pub version: String,
}
