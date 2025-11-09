use sqlx::PgPool;

#[derive(Clone)]
pub struct AppData {
    pool: PgPool,
}

impl AppData {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
