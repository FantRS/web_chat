use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::core::{app_data::AppData, app_error::AppResult};

pub async fn run(lst: TcpListener, app_data: AppData) -> AppResult<()> {
    tracing::info!("running server");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(app_data.clone()))
    })
    .listen(lst)?
    .run()
    .await?;

    Ok(())
}
