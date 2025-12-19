pub mod apis;
pub mod app;
pub mod core;

use core::{app_config::AppConfig, app_data::AppData, app_error::AppResult};
use std::net::TcpListener;

pub async fn start() -> AppResult<()> {
    dotenvy::dotenv().ok();
    core::telemetry::init_logger("info");

    let config = AppConfig::configure()?;
    let jwt_secret = std::env::var("JWT_SECRET")?;

    let pool = core::database::connect(config.database.options()).await?;
    let lst = TcpListener::bind(config.app.addr())?;
    let app_data = AppData::builder()
        .with_pool(pool)
        .with_jwt_secret(jwt_secret)
        .build()?;

    core::server::run(lst, app_data).await?;

    Ok(())
}
