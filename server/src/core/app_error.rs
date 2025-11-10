pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("DatabaseError. Context: {0}")]
    DatabaseError(String),

    #[error("ParseError. Context: {0}")]
    ParseError(String),

    #[error("IoError. Context: {0}")]
    IoError(#[from] std::io::Error),

    #[error("VarError. Context: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("BuildConfigError. Context: {0}")]
    BuildConfigError(String),
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(value.to_string())
    }
}

impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::DatabaseError(value.to_string())
    }
}

impl From<std::str::ParseBoolError> for AppError {
    fn from(value: std::str::ParseBoolError) -> Self {
        Self::ParseError(value.to_string())
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParseError(value.to_string())
    }
}

impl From<std::num::ParseFloatError> for AppError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::ParseError(value.to_string())
    }
}
