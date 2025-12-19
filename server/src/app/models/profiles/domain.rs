use crate::app::request_error::RequestError;

const MAX_USERNAME_LENGTH: usize = 256;
const MIN_AGE: i32 = 12;
const MAX_AGE: i32 = 120;
const MAX_ABOUT_ME_LENGTH: usize = 1024;

#[derive(Debug, Clone)]
pub struct Username(String);

impl TryFrom<String> for Username {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(RequestError::BadRequest("Username is empty".into()));
        }
        if value.len() >= MAX_USERNAME_LENGTH {
            return Err(RequestError::BadRequest("Username is too long".into()));
        }
        if !value.chars().all(char::is_alphanumeric) {
            return Err(RequestError::BadRequest(
                "Username must be alphanumeric".into(),
            ));
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Age(i32);

impl TryFrom<i32> for Age {
    type Error = RequestError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < MIN_AGE || value > MAX_AGE {
            return Err(RequestError::BadRequest("Invalid age".into()));
        }

        Ok(Self(value))
    }
}

impl AsRef<i32> for Age {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct AboutMe(String);

impl TryFrom<String> for AboutMe {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > MAX_ABOUT_ME_LENGTH {
            return Err(RequestError::BadRequest(
                "Profile description is too long".into(),
            ));
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for AboutMe {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
