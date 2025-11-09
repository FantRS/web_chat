pub mod app;
pub mod core;
pub mod telemetry;

use core::{app_data::AppData, app_error::AppResult, database, server};
use std::net::TcpListener;

pub async fn start() -> AppResult<()> {
    dotenvy::dotenv().ok();
    telemetry::init_logger("info");

    let database_url = std::env::var("DATABASE_URL")?;

    let pool = database::establish_connection(database_url).await?;
    let lst = TcpListener::bind("127.0.0.1:8080")?;
    let app_data = AppData::new(pool);

    server::run(lst, app_data).await?;

    Ok(())
}
