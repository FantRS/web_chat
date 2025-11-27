use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use sqlx::PgPool;
use uuid::Uuid;

use crate::app::{
    models::users::{UserResponse, ValidCreateUserRequest, ValidUpdateUserRequest},
    repositories::user_repository,
    request_error::{RequestError, RequestResult},
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

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(user.password.as_ref().as_bytes(), &salt)?
        .to_string();

    user_repository::create(email, password_hash, pool).await
}

pub async fn patch_user(
    user_id: Uuid,
    mut user: ValidUpdateUserRequest,
    pool: &PgPool,
) -> RequestResult<Uuid> {
    if user.is_empty() {
        return Err(RequestError::BadRequest("user info is empty".into()));
    }

    if let Some(password) = user.password {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_ref().as_bytes(), &salt)?
            .to_string();

        user.password = password_hash.to_string().try_into().ok();
    }

    user_repository::patch(user_id, user, pool).await
}

pub async fn delete_user(user_id: Uuid, pool: &PgPool) -> RequestResult<Uuid> {
    user_repository::delete(user_id, pool).await
}
