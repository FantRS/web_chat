use sqlx::{PgExecutor, Postgres};
use uuid::Uuid;

use crate::app::{
    models::users::{UserResponse, ValidUpdateUserRequest},
    request_error::RequestResult,
};

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

pub async fn patch<'c, E>(
    user_id: Uuid,
    user: ValidUpdateUserRequest,
    exec: E,
) -> RequestResult<Uuid>
where
    E: PgExecutor<'c>,
{
    let mut query_builder = sqlx::QueryBuilder::<Postgres>::new("UPDATE users SET ");
    let mut separated = query_builder.separated(", ");

    if let Some(email) = user.email {
        separated
            .push("email = ")
            .push_bind(email.as_ref().to_owned());
    }

    if let Some(password) = user.password {
        separated
            .push("password = ")
            .push_bind(password.as_ref().to_owned());
    }

    separated.push("updated_at = now()");

    query_builder
        .push(" WHERE id = ")
        .push_bind(user_id)
        .push(" RETURNING id");

    query_builder
        .build_query_scalar()
        .fetch_one(exec)
        .await
        .map_err(From::from)
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
