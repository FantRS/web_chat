use sqlx::PgExecutor;
use uuid::Uuid;

use crate::app::{models::users::UserResponse, request_error::RequestResult};

pub async fn get<'c, E>(user_id: Uuid, exec: E) -> RequestResult<UserResponse>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as!(
        UserResponse,
        "SELECT id, email, password 
            FROM users 
            WHERE id = $1",
        user_id
    )
    .fetch_one(exec)
    .await
    .map_err(From::from)
}

pub async fn get_by<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

pub async fn create<'c, S1, S2, E>(email: S1, password: S2, exec: E) -> RequestResult<Uuid>
where
    S1: AsRef<str>,
    S2: AsRef<str>,
    E: PgExecutor<'c>,
{
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

pub async fn delete<'c, E>(user_id: Uuid, exec: E) -> RequestResult<Uuid>
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
