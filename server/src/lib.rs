pub mod app;
pub mod core;
pub mod telemetry;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use core::app_error::AppResult;

pub async fn run(lst: TcpListener) -> AppResult<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
    })
    .listen(lst)?
    .run()
    .await?;

    Ok(())
}
