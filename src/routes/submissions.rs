use actix_multipart::Multipart;
use actix_web::{get, post, HttpResponse, Responder};
use askama::Template;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use uuid::Uuid;

use crate::db::schema::init_db;
use crate::db::submission_repository::SubmissionRepository;
use crate::errors::SubmissionError;
use crate::models::response::SubmissionResponse;
use crate::models::submission::Submission;

#[derive(Template)]
#[template(path = "submissions/submit.html")]
struct SubmissionsTemplate {}

#[get("/submit")]
pub async fn submit_paper_handler() -> impl Responder {
    HttpResponse::Ok().body(SubmissionsTemplate {}.render().unwrap())
}

#[post("/submit")]
pub async fn process_submission(mut payload: Multipart) -> Result<HttpResponse, SubmissionError> {
    let mut full_name = None;
    let mut email = None;
    let mut phone = None;
    let mut title = None;
    let mut abstract_text = None;
    let mut pdf_filename = None;
    let mut created_at = None;

    // Process the multipart form
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().ok_or_else(|| {
            SubmissionError::ValidationError("Content disposition not found".to_string())
        })?;

        let name = content_disposition
            .get_name()
            .ok_or_else(|| SubmissionError::ValidationError("Field name not found".to_string()))?;

        match name {
            "full_name" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                full_name = Some(value);
            }
            "email" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                email = Some(value);
            }
            "phone" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                phone = Some(value);
            }
            "title" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                title = Some(value);
            }
            "abstract_text" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                abstract_text = Some(value);
            }
            "pdf" => {
                let uuid = Uuid::new_v4();
                let file_name = format!("{}.pdf", uuid.to_string());
                let file_path = format!("./data/uploads/{}", file_name);

                // Create the file
                let mut f = std::fs::File::create(&file_path)
                    .map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;

                // Write file content
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    f.write_all(&data)
                        .map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                }

                pdf_filename = Some(file_name);
            }
            _ => {
                // Skip other fields
                while let Some(_) = field.next().await {}
            }
        }
    }

    // Validate all required fields are present
    let full_name = full_name.ok_or(SubmissionError::ValidationError(
        "Full name is required".to_string(),
    ))?;
    let email = email.ok_or(SubmissionError::ValidationError(
        "Email is required".to_string(),
    ))?;
    let phone = phone.ok_or(SubmissionError::ValidationError(
        "Phone is required".to_string(),
    ))?;
    let title = title.ok_or(SubmissionError::ValidationError(
        "Title is required".to_string(),
    ))?;
    let abstract_text = abstract_text.ok_or(SubmissionError::ValidationError(
        "Abstract is required".to_string(),
    ))?;
    let pdf_url = pdf_filename.ok_or(SubmissionError::ValidationError(
        "PDF file is required".to_string(),
    ))?;

    // Create submission object
    let submission = Submission::new(
        full_name,
        email,
        phone,
        title,
        abstract_text,
        format!("./data/uploads/{}", pdf_url),
        created_at,
    );

    // Validate submission
    submission.validate_submission()?;

    // Save to database
    let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
    let repository = SubmissionRepository::new(conn);
    let submission_id = repository.save_submission(&submission)?;

    Ok(HttpResponse::Ok().json(SubmissionResponse {
        success: true,
        submission_id: submission_id as i32,
        message: "Submission uploaded successfully".to_string(),
    }))
}
