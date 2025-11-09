use server::{core::app_error::AppResult, telemetry};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenvy::dotenv().ok();
    telemetry::init_logger("debug");

    let lst = TcpListener::bind("127.0.0.1:8080")?;

    server::run(lst).await
}
