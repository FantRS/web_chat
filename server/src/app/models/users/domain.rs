use crate::app::request_error::RequestError;

#[derive(Debug, Clone)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let is_valid = validator::ValidateEmail::validate_email(&value);

        if is_valid {
            return Err(RequestError::BadRequest(
                "invalid email address".to_string(),
            ));
        }

        Ok(Email(value))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Password(String);

impl TryFrom<String> for Password {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let is_valid = !value.is_empty() || value.len() < 256;

        if !is_valid {
            return Err(RequestError::BadRequest(
                "invalid user password".to_string(),
            ));
        }

        Ok(Password(value))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
