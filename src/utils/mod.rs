use actix_multipart::Field;
use actix_web::web::Bytes;
use futures::{StreamExt, TryStreamExt};
use log::info;
use std::fs;
use std::io::Write; // Import Write trait
use std::path::Path;
use uuid::Uuid; // Import Uuid

use crate::errors::SubmissionError; // Assuming SubmissionError is in scope

pub mod security;

pub fn ensure_upload_dir() -> std::io::Result<()> {
    let upload_dir = Path::new("./data/uploads");
    if !upload_dir.exists() {
        info!("Creating uploads directory...");
        fs::create_dir_all(upload_dir)?;
    }
    Ok(())
}

// Helper to read text fields from multipart
pub async fn read_field(mut field: Field) -> Result<String, SubmissionError> {
    let mut bytes = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk
            .map_err(|e| SubmissionError::FileProcessingError(format!("Chunk error: {}", e)))?;
        bytes.extend_from_slice(&data);
    }
    String::from_utf8(bytes)
        .map_err(|e| SubmissionError::FileProcessingError(format!("Invalid UTF-8: {}", e)))
}

// Helper to save uploaded files
pub async fn save_uploaded_file(mut field: Field) -> Result<String, SubmissionError> {
    let content_disposition = field.content_disposition().ok_or_else(|| {
        SubmissionError::ValidationError("Content disposition not found".to_string())
    })?;
    let original_filename = content_disposition.get_filename().unwrap_or("unknown.bin"); // Use .bin as generic default
    let extension = Path::new(original_filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("bin"); // Default extension

    let uuid = Uuid::new_v4();
    let file_name = format!("{}.{}", uuid, extension);
    let file_path_str = format!("./data/uploads/{}", file_name);
    let file_path = Path::new(&file_path_str);

    // Ensure directory exists (might be redundant if ensure_upload_dir is called at startup)
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            SubmissionError::StorageError(format!("Failed to create upload dir: {}", e))
        })?;
    }

    let mut file = fs::File::create(file_path).map_err(|e| {
        SubmissionError::FileProcessingError(format!(
            "Failed to create file {}: {}",
            file_path_str, e
        ))
    })?;

    while let Some(chunk) = field.next().await {
        let data = chunk
            .map_err(|e| SubmissionError::FileProcessingError(format!("Chunk error: {}", e)))?;
        file.write_all(&data).map_err(|e| {
            SubmissionError::FileProcessingError(format!(
                "Failed to write to file {}: {}",
                file_path_str, e
            ))
        })?;
    }

    Ok(file_name) // Return only the generated filename
}
