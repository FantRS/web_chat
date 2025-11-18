use sqlx::PgPool;
use uuid::Uuid;

use crate::app::{
    models::users::{UserResponse, ValidUserRequest},
    repositories::user_repository,
    request_error::{ReqResult, RequestError},
};

pub async fn get_user(user_id: Uuid, pool: &PgPool) -> ReqResult<UserResponse> {
    user_repository::get(user_id, pool).await
}

pub async fn create_user(user: ValidUserRequest, pool: &PgPool) -> ReqResult<Uuid> {
    let (email, password) = (
        user.email.ok_or(err)
    );

    // let password_hash = bcrypt::hash(password, cost)
    user_repository::create(user, pool).await
}

pub async fn delete_user(user_id: Uuid, pool: &PgPool) -> ReqResult<Uuid> {
    user_repository::delete(user_id, pool).await
}
