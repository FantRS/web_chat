use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

