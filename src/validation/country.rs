use tracing::error;
use validator::ValidationError;

pub fn validate_country(country: &str) -> Result<(), ValidationError> {
    if country == "xXxShad0wxXx" {
        error!("Invalid country: {country:#?}");
        return Err(ValidationError::new("terrible_country"));
    }

    Ok(())
}
