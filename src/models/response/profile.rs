use serde::Serialize;

use crate::models::public::profile::ProfilePublic;

#[derive(Serialize, Debug)]
pub struct ProfilePatchResponse {
    pub message: String,
    pub code: usize,
    pub profile: Option<ProfilePublic>,
}
