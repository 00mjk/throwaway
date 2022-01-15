use validator::ValidationError;

pub fn validate_timezone(timezone: &str) -> Result<(), ValidationError> {
    if timezone == "xXxShad0wxXx" {
        return Err(ValidationError::new("terrible_timezone"));
    }

    Ok(())
}
