use sqlx::PgExecutor;
use uuid::Uuid;

use crate::app::{
    models::profiles::{ProfileEntity, ValidCreateProfileRequest},
    request_error::RequestResult,
};

pub async fn get_by_user_id<'c, E>(user_id: Uuid, exec: E) -> RequestResult<ProfileEntity>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as!(
        ProfileEntity,
        "SELECT * FROM profiles 
            WHERE user_id = $1",
        user_id
    )
    .fetch_one(exec)
    .await
    .map_err(From::from)
}

pub async fn create<'c, E>(
    user_id: Uuid,
    profile: ValidCreateProfileRequest,
    exec: E,
) -> RequestResult<Uuid>
where
    E: PgExecutor<'c>,
{
    sqlx::query_scalar!(
        "INSERT INTO profiles (username, user_id, age, about_me) 
            VALUES ($1, $2, $3, $4) 
            RETURNING id",
        profile.username.as_ref(),
        user_id,
        profile.age.as_ref(),
        profile.about_me.as_ref()
    )
    .fetch_one(exec)
    .await
    .map_err(From::from)
}
