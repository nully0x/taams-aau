use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum SubmissionError {
    #[display(fmt = "Database error: {}", _0)]
    DatabaseError(String),

    #[display(fmt = "Storage error: {}", _0)]
    StorageError(String),

    #[display(fmt = "Validation error: {}", _0)]
    ValidationError(String),

    #[display(fmt = "File processing error: {}", _0)]
    FileProcessingError(String),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl ResponseError for SubmissionError {
    fn error_response(&self) -> HttpResponse {
        match self {
            SubmissionError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "VALIDATION_ERROR".to_string(),
                    message: msg.to_string(),
                })
            }
            SubmissionError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: msg.to_string(),
                })
            }
            SubmissionError::StorageError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "STORAGE_ERROR".to_string(),
                    message: msg.to_string(),
                })
            }
            SubmissionError::FileProcessingError(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "FILE_PROCESSING_ERROR".to_string(),
                    message: msg.to_string(),
                })
            }
        }
    }
}
