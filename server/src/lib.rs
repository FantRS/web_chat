pub mod apis;
pub mod app;
pub mod core;
pub mod telemetry;

use core::{app_config::AppConfig, app_data::AppData, app_error::AppResult, database, server};
use std::net::TcpListener;

pub async fn start() -> AppResult<()> {
    dotenvy::dotenv().ok();
    telemetry::init_logger("info");

    let config = AppConfig::configure()?;
    let jwt_secret = std::env::var("JWT_SECRET")?;

    let pool = database::connect(config.database.options()).await?;
    let lst = TcpListener::bind(config.app.addr())?;
    let app_data = AppData::builder()
        .with_pool(pool)
        .with_jwt_secret(jwt_secret)
        .build()?;

    server::run(lst, app_data).await?;

    Ok(())
}
