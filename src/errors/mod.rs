use crate::models::response::ValidationResponse;
use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum SubmissionError {
    DatabaseError(String),
    StorageError(String),
    ValidationError(String),
    FileProcessingError(String),
}

// Implement Display manually instead of using derive
impl fmt::Display for SubmissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubmissionError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            SubmissionError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            SubmissionError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            SubmissionError::FileProcessingError(msg) => {
                write!(f, "File processing error: {}", msg)
            }
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

// Implement std::error::Error
impl std::error::Error for SubmissionError {}

impl From<Vec<ValidationResponse>> for SubmissionError {
    fn from(errors: Vec<ValidationResponse>) -> Self {
        // Join all validation messages into a single string
        let message = errors
            .iter()
            .map(|e| format!("{}: {}", e.field, e.message))
            .collect::<Vec<String>>()
            .join("; ");

        SubmissionError::ValidationError(message)
    }
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

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            SubmissionError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            SubmissionError::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            SubmissionError::StorageError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            SubmissionError::FileProcessingError(_) => actix_web::http::StatusCode::BAD_REQUEST,
        }
    }
}
