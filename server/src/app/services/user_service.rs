use sqlx::PgPool;
use uuid::Uuid;

use crate::app::{
    models::users::{UserResponse, ValidCreateUserRequest},
    repositories::user_repository,
    request_error::RequestResult,
};

pub async fn get_user(user_id: Uuid, pool: &PgPool) -> RequestResult<UserResponse> {
    user_repository::get(user_id, pool).await
}

// ? ? ?
pub async fn login_user() -> RequestResult<String> {
    Ok(String::new())
}

pub async fn create_user(user: ValidCreateUserRequest, pool: &PgPool) -> RequestResult<Uuid> {
    let email = user.email.as_ref();
    let password_hash = bcrypt::hash(user.password.as_ref(), bcrypt::DEFAULT_COST)?;

    user_repository::create(email, password_hash, pool).await
}

pub async fn delete_user(user_id: Uuid, pool: &PgPool) -> RequestResult<Uuid> {
    user_repository::delete(user_id, pool).await
}
