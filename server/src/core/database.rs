use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::core::app_error::{AppError, AppResult};

pub async fn establish_connection<S>(database_url: S) -> AppResult<PgPool>
where
    S: AsRef<str>,
{
    tracing::info!("establishing database connection");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_ref())
        .await?;

    let is_run_migrate: bool = std::env::var("MIGRATE_RUN")
        .unwrap_or("false".into())
        .parse()?;

    if is_run_migrate {
        tracing::info!("running migrations");

        let migrate_res = sqlx::migrate!("./migrations").run(&pool).await;

        if let Err(e) = migrate_res {
            tracing::error!("Migration failed: {}", e);
            return Err(AppError::MigrateError(e));
        }
    }

    Ok(pool)
}
