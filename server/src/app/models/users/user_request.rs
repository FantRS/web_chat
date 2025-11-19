use serde::{Deserialize, Serialize};

use crate::app::request_error::RequestError;

use super::domain;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

pub struct ValidCreateUserRequest {
    pub email: domain::Email,
    pub password: domain::Password,
}

impl TryFrom<CreateUserRequest> for ValidCreateUserRequest {
    type Error = RequestError;

    fn try_from(value: CreateUserRequest) -> Result<Self, Self::Error> {
        let valid_user = Self {
            email: value.email.try_into()?,
            password: value.password.try_into()?,
        };

        Ok(valid_user)
    }
}
