use validator::ValidationError;

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password == "xXxShad0wxXx" {
        return Err(ValidationError::new("terrible_password"));
    }

    Ok(())
}
