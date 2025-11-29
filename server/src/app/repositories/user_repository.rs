use sqlx::{PgExecutor, Postgres};
use uuid::Uuid;

use crate::app::{
    models::users::{UserEntity, ValidUpdateUserRequest},
    request_error::RequestResult,
};

pub async fn get<'c, E>(user_id: Uuid, exec: E) -> RequestResult<UserEntity>
where
    E: PgExecutor<'c>,
{
    sqlx::query_as!(
        UserEntity,
        "SELECT *
            FROM users 
            WHERE id = $1",
        user_id
    )
    .fetch_one(exec)
    .await
    .map_err(From::from)
}

pub async fn get_by_email<'c, S, E>(email: S, exec: E) -> RequestResult<UserEntity>
where
    E: PgExecutor<'c>,
    S: AsRef<str>,
{
    sqlx::query_as!(
        UserEntity,
        "SELECT * FROM users 
            WHERE email = $1",
        email.as_ref()
    )
    .fetch_one(exec)
    .await
    .map_err(From::from)
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

    use super::*;

    #[sqlx::test]
    async fn test_create(pool: PgPool) {
        let user_id = create("rost@gmail.com", "somepassword", &pool)
            .await
            .unwrap();

        let exp = expect!["a936afc8-6079-477a-98af-5c8c8620f67b"];
        exp.assert_eq(&user_id.to_string());
    }

    #[sqlx::test]
    async fn test_get(pool: PgPool) {
        let user_id = create("rost@gmail.com", "somepass", &pool).await.unwrap();
        let exp = expect!["3d162d80-1916-43b0-9824-d45f29f31fd0"];
        exp.assert_eq(&user_id.to_string());

        let user = get(user_id, &pool).await.unwrap();
        let exp = expect![[r#"
            UserEntity {
                id: 3d162d80-1916-43b0-9824-d45f29f31fd0,
                email: "rost@gmail.com",
                password: "somepass",
                created_at: 2025-11-29T20:18:52.012315Z,
                updated_at: 2025-11-29T20:18:52.012315Z,
            }"#]];
        exp.assert_eq(&format!("{:#?}", user));
    }

    #[sqlx::test]
    async fn test_get_by_email(pool: PgPool) {
        let email = "rost@gmail.com";

        let user_id = create(email, "somepass", &pool).await.unwrap();
        let exp = expect!["217a7634-cd89-4e3a-b58e-d10ff460323c"];
        exp.assert_eq(&user_id.to_string());

        let user = get_by_email(email, &pool).await.unwrap();
        let exp = expect![[r#"
            UserEntity {
                id: 217a7634-cd89-4e3a-b58e-d10ff460323c,
                email: "rost@gmail.com",
                password: "somepass",
                created_at: 2025-11-29T20:18:52.001497Z,
                updated_at: 2025-11-29T20:18:52.001497Z,
            }"#]];
        exp.assert_eq(&format!("{:#?}", user));
    }

    #[sqlx::test]
    async fn test_delete(pool: PgPool) {
        let user_id = create("rost@gmail.com", "somepass", &pool).await.unwrap();
        let exp = expect!["c1fc0c91-c2fa-439a-bae7-82793f051be6"];
        exp.assert_eq(&user_id.to_string());

        let deleted_user_id = delete(user_id, &pool).await.unwrap();
        let exp = expect!["c1fc0c91-c2fa-439a-bae7-82793f051be6"];
        exp.assert_eq(&deleted_user_id.to_string());

        let try_get_info = get(deleted_user_id, &pool).await;
        assert!(try_get_info.is_err());
    }

    #[sqlx::test]
    async fn test_patch(pool: PgPool) {
        let user_id = create("rost@gmail.com", "somepass", &pool).await.unwrap();
        let exp = expect!["7949b2f2-ddac-477f-820d-b83edd5c6651"];
        exp.assert_eq(&user_id.to_string());

        let user = get(user_id, &pool).await.unwrap();
        let exp = expect![[r#"
            UserEntity {
                id: 7949b2f2-ddac-477f-820d-b83edd5c6651,
                email: "rost@gmail.com",
                password: "somepass",
                created_at: 2025-11-29T20:18:52.027833Z,
                updated_at: 2025-11-29T20:18:52.027833Z,
            }"#]];
        exp.assert_eq(&format!("{:#?}", user));

        let patch_info = ValidUpdateUserRequest {
            email: Some("updatedRost@gmail.com".to_string().try_into().unwrap()),
            password: Some("updatedPassword".to_string().try_into().unwrap()),
        };

        let patch_user_id = patch(user_id, patch_info, &pool).await.unwrap();
        let patch_user = get(patch_user_id, &pool).await.unwrap();
        let exp = expect![[r#"
            UserEntity {
                id: 7949b2f2-ddac-477f-820d-b83edd5c6651,
                email: "updatedRost@gmail.com",
                password: "updatedPassword",
                created_at: 2025-11-29T20:18:52.027833Z,
                updated_at: 2025-11-29T20:18:52.064844Z,
            }"#]];
        exp.assert_eq(&format!("{:#?}", patch_user));
    }
}
