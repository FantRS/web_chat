use sqlx::{PgExecutor, Postgres};
use uuid::Uuid;

use crate::app::{
    models::users::{UserResponse, ValidUpdateUserRequest},
    request_error::{RequestError, RequestResult},
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
            .push_bind_unseparated(email.as_ref().to_owned());
    }

    if let Some(password) = user.password {
        separated
            .push("password = ")
            .push_bind_unseparated(password.as_ref().to_owned());
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

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use sqlx::PgPool;

    use crate::app::models::users::domain;

    use super::*;

    #[sqlx::test]
    async fn test_create(pool: PgPool) {
        let user_id = create("rost@gmail.com", "somepassword", &pool)
            .await
            .unwrap();

        let exp = expect!["b9718440-81e6-424e-9bb9-5ca4e24beab4"];
        exp.assert_eq(&user_id.to_string());
    }

    #[sqlx::test]
    async fn test_get(pool: PgPool) {
        let user_id = create("rost@gmail.com", "somepass", &pool).await.unwrap();
        let exp = expect!["4179a268-fc1b-4c40-8e11-971451e7d26e"];
        exp.assert_eq(&user_id.to_string());

        let user = get(user_id, &pool).await.unwrap();
        let exp = expect![[r#"
            UserResponse {
                id: 4179a268-fc1b-4c40-8e11-971451e7d26e,
                email: "rost@gmail.com",
                password: "somepass",
            }"#]];
        exp.assert_eq(&format!("{:#?}", user));
    }

    #[sqlx::test]
    async fn test_delete(pool: PgPool) {
        let user_id = create("rost@gmail.com", "somepass", &pool).await.unwrap();
        let exp = expect!["5682386b-6f73-4f5d-a5be-5d7747973823"];
        exp.assert_eq(&user_id.to_string());

        let deleted_user_id = delete(user_id, &pool).await.unwrap();
        let exp = expect!["5682386b-6f73-4f5d-a5be-5d7747973823"];
        exp.assert_eq(&deleted_user_id.to_string());

        let try_get_info = get(deleted_user_id, &pool).await;
        assert!(try_get_info.is_err());
    }

    #[sqlx::test]
    async fn test_patch(pool: PgPool) {
        let user_id = create("rost@gmail.com", "somepass", &pool).await.unwrap();
        let exp = expect!["9575ab6f-cac2-4844-ad89-9f27802c7bb8"];
        exp.assert_eq(&user_id.to_string());

        let user = get(user_id, &pool).await.unwrap();
        let exp = expect![[r#"
            UserResponse {
                id: 9575ab6f-cac2-4844-ad89-9f27802c7bb8,
                email: "rost@gmail.com",
                password: "somepass",
            }"#]];
        exp.assert_eq(&format!("{:#?}", user));

        let patch_info = ValidUpdateUserRequest {
            email: Some("updatedRost@gmail.com".to_string().try_into().unwrap()),
            password: Some("updatedPassword".to_string().try_into().unwrap()),
        };

        let patch_user_id = patch(user_id, patch_info, &pool).await.unwrap();
        let patch_user = get(patch_user_id, &pool).await.unwrap();
        let exp = expect![[r#"
            UserResponse {
                id: 9575ab6f-cac2-4844-ad89-9f27802c7bb8,
                email: "updatedRost@gmail.com",
                password: "updatedPassword",
            }"#]];
        exp.assert_eq(&format!("{:#?}", patch_user));
    }
}
