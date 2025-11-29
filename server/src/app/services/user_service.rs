use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::app::{
    extensions::jwt_coding,
    middlewares::jwt::Claims,
    models::users::{
        UserResponse, ValidCreateUserRequest, ValidLoginUserRequest, ValidPatchUserRequest,
    },
    repositories::user_repository,
    request_error::{RequestError, RequestResult},
};

const DUMMY_HASH: &str =
    "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHR2YWx1ZQ$7H0vsXlY8UxxyW4TOHcDkME0mI0j6lf45i1HNrPzEzs";

pub async fn get_user(user_id: Uuid, pool: &PgPool) -> RequestResult<UserResponse> {
    user_repository::get(user_id, pool)
        .await
        .map(UserResponse::from)
}

pub async fn login_user(
    user: ValidLoginUserRequest,
    jwt_secret: &str,
    pool: &PgPool,
) -> RequestResult<String> {
    let sql_result = user_repository::get_by_email(user.email.as_ref(), pool).await;

    let password_hash = sql_result
        .as_ref()
        .map(|entity| entity.password.as_ref())
        .unwrap_or(DUMMY_HASH);

    let is_verified = PasswordHash::new(password_hash)
        .map(|hash| {
            Argon2::default()
                .verify_password(user.password.as_ref().as_bytes(), &hash)
                .is_ok()
        })
        .unwrap_or(false);

    if is_verified {
        let claims = Claims::from(sql_result.unwrap());
        let token = jwt_coding::encode_jwt(claims, jwt_secret)?;
        Ok(token)
    } else {
        Err(RequestError::Unauthorized(
            "Invalid email or password".into(),
        ))
    }
}

pub async fn create_user(user: ValidCreateUserRequest, pool: &PgPool) -> RequestResult<Uuid> {
    let email = user.email.as_ref();
    let password_hash = generate_password_hash(user.password.as_ref().as_bytes())?;

    user_repository::create(email, password_hash.as_str(), pool).await
}

pub async fn patch_user(
    user_id: Uuid,
    mut user: ValidPatchUserRequest,
    pool: &PgPool,
) -> RequestResult<Uuid> {
    if user.is_empty() {
        return Err(RequestError::BadRequest("user info is empty".into()));
    }

    if let Some(password) = user.password {
        let password_hash = generate_password_hash(password.as_ref().as_bytes())?;

        user.password = Some(password_hash.to_string().try_into()?);
    }

    user_repository::patch(user_id, user, pool).await
}

pub async fn delete_user(user_id: Uuid, pool: &PgPool) -> RequestResult<Uuid> {
    user_repository::delete(user_id, pool).await
}

fn generate_password_hash(password: &[u8]) -> RequestResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password, &salt)?
        .to_string();

    Ok(password_hash)
}
