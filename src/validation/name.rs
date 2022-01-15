use validator::ValidationError;

pub fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name == "xXxShad0wxXx" {
        return Err(ValidationError::new("terrible_name"));
    }

    Ok(())
}
