use actix_web::{
    Error,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::{extensions::jwt_coding, models::users::UserEntity, request_error::RequestError},
    core::app_data::AppData,
};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,     // subject
    pub email: String, // user email
    pub iat: usize,    // issued at
    pub exp: usize,    // expiration time
}

impl From<UserEntity> for Claims {
    fn from(value: UserEntity) -> Self {
        let now = Utc::now();

        Self {
            sub: value.id,
            email: value.email,
            iat: now.timestamp() as usize,
            exp: (now + Duration::hours(24)).timestamp() as usize,
        }
    }
}

pub async fn verify_jwt(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // get app_data
    let app_data =
        req.app_data::<web::Data<AppData>>()
            .ok_or(RequestError::InternalServerError(
                "JWT middleware error: app_data initialize".into(),
            ))?;

    let error_str = "Extract token error";
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(RequestError::Unauthorized(error_str.into()))?
        .to_str()
        .map_err(|e| RequestError::Unauthorized(e.to_string()))?
        .strip_prefix("Bearer ")
        .filter(|t| !t.trim().is_empty())
        .ok_or(RequestError::Unauthorized(error_str.into()))?;

    let _data = jwt_coding::decode_jwt::<Claims>(token, &app_data.jwt_secret)?;

    next.call(req).await
}
