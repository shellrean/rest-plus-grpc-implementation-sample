use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tonic::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::Database(msg) => {
                tracing::error!("database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("500"),
                    String::from("internal server database error"),
                )
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, String::from("404"), msg),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, String::from("400"), msg),
        };

        let body = ErrorDetail { code, message };

        (status, Json(body)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Database(value.to_string())
    }
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Database(msg) => Status::internal(msg),
            AppError::NotFound(msg) => Status::not_found(msg),
            AppError::InvalidInput(msg) => Status::invalid_argument(msg),
        }
    }
}
