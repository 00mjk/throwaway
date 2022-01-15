use validator::ValidationError;

const MISSING_EMAIL: &str = "missing_email";
const INVALID_EMAIL: &str = "invalid_email";
const INVALID_EMAIL_DOMAIN: &str = "invalid_email_domain";

pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.is_empty() {
        return Err(ValidationError::new(MISSING_EMAIL));
    }

    if !email.contains('@') {
        return Err(ValidationError::new(INVALID_EMAIL));
    }

    if email.contains(' ') {
        return Err(ValidationError::new(INVALID_EMAIL));
    }

    let email_parts: Vec<&str> = email.split('@').collect();
    if email_parts.len() != 2 {
        return Err(ValidationError::new(INVALID_EMAIL));
    }

    let domain = email_parts[1];
    if !domain.contains('.') {
        return Err(ValidationError::new(INVALID_EMAIL_DOMAIN));
    }

    if domain.contains('_') {
        return Err(ValidationError::new(INVALID_EMAIL_DOMAIN));
    }

    Ok(())
}
