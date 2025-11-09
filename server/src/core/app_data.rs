use sqlx::PgPool;

pub struct AppData {
    pool: PgPool,
}
