use crate::app::request_error::RequestError;

use super::domain;

pub struct CreateProfileRequest {
    pub username: String,
    pub age: i32,
    pub about_me: String,
}

pub struct ValidCreateProfileRequest {
    pub username: domain::Username,
    pub age: domain::Age,
    pub about_me: domain::AboutMe,
}

impl TryFrom<CreateProfileRequest> for ValidCreateProfileRequest {
    type Error = RequestError;

    fn try_from(value: CreateProfileRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            username: value.username.try_into()?,
            age: value.age.try_into()?,
            about_me: value.about_me.try_into()?,
        })
    }
}
