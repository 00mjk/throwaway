use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ProfilePublic {
    pub name: String,
    pub email: String,
    pub country: String,
    pub timezone: String,
}
