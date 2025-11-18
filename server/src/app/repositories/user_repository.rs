use sqlx::PgExecutor;
use uuid::Uuid;

use crate::app::request_error::RequestResult;

pub async fn get<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

pub async fn get_by<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

pub async fn create<'c, E>(email: String, password: String, exec: E) -> RequestResult<Uuid>
where
    E: PgExecutor<'c>,
{
    sqlx::query_scalar!(
        "INSERT INTO users (email, password) 
            VALUES ($1, $2) 
            RETURNING id",
        email,
        password
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

pub async fn delete<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}
