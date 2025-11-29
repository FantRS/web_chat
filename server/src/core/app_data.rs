use sqlx::PgPool;

use crate::core::app_error::{AppError, AppResult};

#[derive(Clone)]
pub struct AppData {
    pub pool: PgPool,
    pub jwt_secret: String,
}

impl AppData {
    pub fn builder() -> AppDataBuilder {
        AppDataBuilder::default()
    }
}

#[derive(Default)]
pub struct AppDataBuilder {
    pool: Option<PgPool>,
    jwt_secret: Option<String>,
}

impl AppDataBuilder {
    pub fn build(self) -> AppResult<AppData> {
        let app_data = AppData {
            pool: self.pool.ok_or(AppError::MissingDatabasePool)?,
            jwt_secret: self.jwt_secret.ok_or(AppError::MissingJwtSecret)?,
        };

        Ok(app_data)
    }

    pub fn with_pool(mut self, pool: PgPool) -> Self {
        self.pool = Some(pool);
        self
    }

    pub fn with_jwt_secret(mut self, secret: String) -> Self {
        self.jwt_secret = Some(secret);
        self
    }
}
