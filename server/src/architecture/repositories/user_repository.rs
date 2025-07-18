use crate::models::user::{User, UserInfo};

use sqlx::{Error as SqlxError, PgPool};

pub async fn get_user(username: &String, pool: &PgPool) -> Result<User, SqlxError> {
    sqlx::query_as::<_, User>(
        "SELECT * FROM users 
        WHERE username = $1",
    )
    .bind(username)
    .fetch_one(pool)
    .await
}

pub async fn add_user(user: &UserInfo, pool: &PgPool) -> Result<User, SqlxError> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash, fk_role_id) 
        VALUES ($1, $2, (SELECT id FROM roles WHERE role_name = $3))
        RETURNING *",
    )
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(&user.role_name)
    .fetch_one(pool)
    .await
}
