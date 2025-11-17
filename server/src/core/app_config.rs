use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::postgres::PgConnectOptions;

use crate::core::app_error::AppResult;

#[derive(Deserialize)]
pub struct AppConfig {
    #[serde(flatten)]
    pub app: AppSettings,
    #[serde(flatten)]
    pub database: DatabaseSettings,
}

impl AppConfig {
    pub fn configure() -> AppResult<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        config.try_deserialize().map_err(From::from)
    }
}

#[serde_as]
#[derive(Deserialize)]
pub struct DatabaseSettings {
    #[serde(rename = "postgres_user")]
    user: String,
    #[serde(rename = "postgres_password")]
    password: String,
    #[serde(rename = "postgres_host")]
    host: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "postgres_port")]
    port: u16,
    #[serde(rename = "postgres_db")]
    db_name: String,
}

impl DatabaseSettings {
    pub fn options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .username(&self.user)
            .password(&self.password)
            .host(&self.host)
            .port(self.port)
            .database(&self.db_name)
    }
}

#[serde_as]
#[derive(Deserialize)]
pub struct AppSettings {
    #[serde(rename = "server_host")]
    host: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "server_port")]
    port: u16,
}

impl AppSettings {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
