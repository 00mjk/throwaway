use serde::Deserialize;
use validator::Validate;

use crate::validation::country::validate_country;
use crate::validation::email::validate_email;
use crate::validation::name::validate_name;
use crate::validation::password::validate_password;
use crate::validation::timezone::validate_timezone;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(custom = "validate_name")]
    pub name: String,

    #[validate(custom = "validate_email")]
    pub email: String,

    #[validate(custom = "validate_password")]
    pub password: String,

    #[validate(custom = "validate_country")]
    pub country: String,

    #[validate(custom = "validate_timezone")]
    pub timezone: String,
}
