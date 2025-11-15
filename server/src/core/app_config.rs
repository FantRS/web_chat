pub struct AppConfig {
    app: AppSettings,
    database: DatabaseSettings,
}

pub struct DatabaseSettings {
    user: String,
    password: String,
    host: String,
    port: u16,
    db_name: String,
}

impl DatabaseSettings {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }
}

pub struct AppSettings {
    host: String,
    port: u16,
}
