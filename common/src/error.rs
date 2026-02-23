use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFoundError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Conflict: {0}")]
    ConflictError(String),

    #[error("Unauthorized: {0}")]
    UnauthorizedError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),

    #[error("Database error: {0}")]
    SeaOrmError(sea_orm::DbErr),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Pub/Sub error: {0}")]
    PubSubError(String),

    #[error("Serialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        match err {
            sea_orm::DbErr::RecordNotFound(msg) => AppError::NotFoundError(msg),
            sea_orm::DbErr::Query(sea_orm::RuntimeErr::SqlxError(e))
                if e.as_database_error()
                    .is_some_and(|e| e.is_unique_violation()) =>
            {
                AppError::ConflictError(e.as_database_error().unwrap().message().to_string())
            }
            _ => AppError::SeaOrmError(err),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFoundError(e) => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::ValidationError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::ConflictError(e) => (StatusCode::CONFLICT, e.to_string()),
            AppError::UnauthorizedError(e) => (StatusCode::UNAUTHORIZED, e.to_string()),
            AppError::InternalServerError(e) => {
                tracing::error!("Internal server error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            AppError::SeaOrmError(e) => {
                tracing::error!("SeaORM error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            AppError::InvalidConfiguration(e) => {
                tracing::error!("Invalid configuration");
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            AppError::PubSubError(e) => {
                tracing::error!("Pub/Sub error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            AppError::SerdeJsonError(e) => {
                tracing::error!("Serialization error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
        };

        let body = Json(ErrorResponse {
            error: status.to_string(),
            message,
        });

        (status, body).into_response()
    }
}
