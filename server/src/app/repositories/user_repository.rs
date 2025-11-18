use sqlx::PgExecutor;
use uuid::Uuid;

use crate::app::{
    models::users::{UserResponse, ValidUserRequest},
    request_error::{ReqResult, RequestError},
};

pub async fn get<'c, E>(user_id: Uuid, exec: E) -> ReqResult<UserResponse>
where
    E: PgExecutor<'c>,
{
    let user = sqlx::query_as!(
        UserResponse,
        "SELECT id, email, password 
            FROM users 
            WHERE id = $1",
        user_id
    )
    .fetch_one(exec)
    .await?;

    Ok(user)
}

pub async fn get_by<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

pub async fn create<'c, E>(user_req: ValidUserRequest, exec: E) -> ReqResult<Uuid>
where
    E: PgExecutor<'c>,
{
    let (email, password) = (
        user_req
            .email
            .ok_or(RequestError::BadRequest("invalid email address".into()))?,
        user_req
            .password
            .ok_or(RequestError::BadRequest("invalid user password".into()))?,
    );

    sqlx::query_scalar!(
        "INSERT INTO users (email, password) 
            VALUES ($1, $2) 
            RETURNING id",
        email.as_ref(),
        password.as_ref()
    )
    .fetch_one(exec)
    .await
    .map_err(From::from)
}

pub async fn update<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

pub async fn delete<'c, E>(user_id: Uuid, exec: E) -> ReqResult<Uuid>
where
    E: PgExecutor<'c>,
{
    sqlx::query_scalar!(
        "DELETE FROM users 
            WHERE id = $1 
            RETURNING id",
        user_id
    )
    .fetch_one(exec)
    .await
    .map_err(From::from)
}
