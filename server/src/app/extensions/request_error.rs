use std::borrow::Cow;

use actix_web::{
    HttpResponse, ResponseError,
    http::{StatusCode, header::ContentType},
};

pub type ReqResult<T> = Result<T, RequestError>;

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("400 Bad Request: {0}")]
    BadRequest(String),

    #[error("401 Unauthorized: {0}")]
    Unauthorized(String),

    #[error("403 Forbidden: {0}")]
    Forbidden(String),

    #[error("404 Not Found: {0}")]
    NotFound(String),

    #[error("405 Method Not Allowed: {0}")]
    MethodNotAllowed(String),

    #[error("409 Conflict: {0}")]
    Conflict(String),

    #[error("422 Unprocessable Entity: {0}")]
    UnprocessableEntity(String),

    #[error("500 Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("501 Not Implemented: {0}")]
    NotImplemented(String),

    #[error("502 Bad Gateway: {0}")]
    BadGateway(String),

    #[error("503 Service Unavailable: {0}")]
    ServiceUnavailable(String),
}

impl ResponseError for RequestError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            RequestError::BadRequest(_) => StatusCode::BAD_REQUEST,
            RequestError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            RequestError::Forbidden(_) => StatusCode::FORBIDDEN,
            RequestError::NotFound(_) => StatusCode::NOT_FOUND,
            RequestError::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            RequestError::Conflict(_) => StatusCode::CONFLICT,
            RequestError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,

            RequestError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RequestError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
            RequestError::BadGateway(_) => StatusCode::BAD_GATEWAY,
            RequestError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl From<sqlx::Error> for RequestError {
    fn from(error: sqlx::Error) -> Self {
        match &error {
            sqlx::Error::RowNotFound => RequestError::NotFound(error.to_string()),
            sqlx::Error::Database(db_error) => {
                let db_code = db_error.code().unwrap_or_default();
                let db_error = db_code.to_string();

                match db_code.as_ref() {
                    "23502" => RequestError::BadRequest(db_error), // спроба впихнути NULL
                    "23503" => RequestError::BadRequest(db_error), // неіснуючий елемент
                    "23505" => RequestError::Conflict(db_error),   // дублікат значення
                    _ => RequestError::InternalServerError(db_error),
                }
            }
            _ => RequestError::InternalServerError(error.to_string()),
        }
    }
}
