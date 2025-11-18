use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::request_error::{ReqResult, RequestError};

use super::domain;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub id: Option<Uuid>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

pub struct ValidUserRequest {
    pub id: Option<Uuid>,
    pub email: Option<domain::Email>,
    pub password: Option<domain::Password>,
    pub created_at: Option<DateTime<Utc>>,
}

impl TryFrom<UserRequest> for ValidUserRequest {
    type Error = RequestError;

    fn try_from(value: UserRequest) -> Result<Self, Self::Error> {
        let request = Self {
            id: value.id,
            email: try_opt(value.email)?,
            password: try_opt(value.password)?,
            created_at: value.created_at,
        };

        Ok(request)
    }
}

impl ValidUserRequest {
    pub fn extract_email(&self) -> ReqResult<domain::Email> {
        self.email
            .clone()
            .ok_or(RequestError::BadRequest("email is empty".into()))
    }

    pub fn extract_password(&self) -> ReqResult<domain::Password> {
        self.password
            .clone()
            .ok_or(RequestError::BadRequest("user password is empty".into()))
    }
}

fn try_opt<T, U, E>(opt: Option<T>) -> Result<Option<U>, E>
where
    T: TryInto<U, Error = E>,
{
    match opt {
        Some(v) => Ok(Some(v.try_into()?)),
        None => Ok(None),
    }
}
