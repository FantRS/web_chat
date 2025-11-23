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

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

pub struct ValidUpdateUserRequest {
    pub email: Option<domain::Email>,
    pub password: Option<domain::Password>,
}

impl TryFrom<UpdateUserRequest> for ValidUpdateUserRequest {
    type Error = RequestError;

    fn try_from(value: UpdateUserRequest) -> Result<Self, Self::Error> {
        let update_info = Self {
            email: value.email.map(domain::Email::try_from).transpose()?,
            password: value.password.map(domain::Password::try_from).transpose()?,
        };

        Ok(update_info)
    }
}

impl ValidUpdateUserRequest {
    pub fn is_empty(&self) -> bool {
        self.email.is_none() && self.password.is_none()
    }
}
