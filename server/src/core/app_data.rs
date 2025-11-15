use sqlx::PgPool;

use crate::core::app_error::{AppError, AppResult};

#[derive(Clone)]
pub struct AppData {
    pool: PgPool,
}

impl AppData {
    pub fn builder() -> AppDataBuilder {
        AppDataBuilder::default()
    }
}

#[derive(Default)]
pub struct AppDataBuilder {
    pool: Option<PgPool>,
}

impl AppDataBuilder {
    pub fn build(self) -> AppResult<AppData> {
        let pool = self
            .pool
            .ok_or(AppError::Other("AppData building error".to_string()))?;

        let app_data = AppData { pool };

        Ok(app_data)
    }

    pub fn with_pool(mut self, pool: PgPool) -> Self {
        self.pool = Some(pool);
        self
    }
}
