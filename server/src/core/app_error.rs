pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database connection error")]
    DbConnError(#[from] sqlx::Error),

    #[error("Migration error")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("Io error")]
    IoError(#[from] std::io::Error),

    #[error("Error retrieving environment variable")]
    VarError(#[from] std::env::VarError),

    #[error("Parse bool error")]
    ParseBoolError(#[from] std::str::ParseBoolError),

    #[error("Parse integer error")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Parse float error")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}
