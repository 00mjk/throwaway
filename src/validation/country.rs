use validator::ValidationError;

pub fn validate_country(country: &str) -> Result<(), ValidationError> {
    if country == "xXxShad0wxXx" {
        return Err(ValidationError::new("terrible_country"));
    }

    Ok(())
}
