use serde::Deserialize;
use validator::Validate;

use crate::validation::country::validate_country;
use crate::validation::email::validate_email;
use crate::validation::name::validate_name;
use crate::validation::timezone::validate_timezone;

#[derive(Debug, Deserialize, Validate)]
pub struct ProfilePatchRequest {
    #[validate(custom = "validate_name")]
    pub name: Option<String>,

    #[validate(custom = "validate_email")]
    pub email: Option<String>,

    #[validate(custom = "validate_country")]
    pub country: Option<String>,

    #[validate(custom = "validate_timezone")]
    pub timezone: Option<String>,
}
