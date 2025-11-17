use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};

use crate::core::app_error::AppResult;

pub async fn connect(options: PgConnectOptions) -> AppResult<PgPool> {
    tracing::info!("establishing database connection");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    let is_run_migrate: bool = std::env::var("MIGRATE_RUN")
        .unwrap_or("false".into())
        .parse()?;

    if is_run_migrate {
        tracing::info!("running migrations");
        sqlx::migrate!("./migrations").run(&pool).await?;
    }

    Ok(pool)
}
