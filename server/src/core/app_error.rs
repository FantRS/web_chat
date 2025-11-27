pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("DBConnection. Context: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("MigrateError. Context: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("IoError. Context: {0}")]
    IoError(#[from] std::io::Error),

    #[error("VarError. Context: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("ConfigError. Context: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("ParseBoolError. Context: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),

    #[error("ParseIntError. Context: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("ParseFloatError. Context: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("OtherError. Context: {0}")]
    Other(String),
}

