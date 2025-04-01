use actix_multipart::Multipart;
use actix_web::{get, post, HttpResponse, Responder};
use askama::Template;
use chrono::{DateTime, NaiveDate, Utc};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use uuid::Uuid;

use crate::db::journal_repository::JournalRepository;
use crate::db::schema::init_db;
use crate::errors::SubmissionError;
use crate::models::journals::Journal;
use crate::models::response::UploadResponse;

#[derive(Template)]
#[template(path = "admin/upload.html")]
struct AdminTemplate {}

#[get("/admin/upload")]
pub async fn upload_journal_handler() -> impl Responder {
    HttpResponse::Ok().body(AdminTemplate {}.render().unwrap())
}

#[post("/admin/upload")]
pub async fn process_upload(mut payload: Multipart) -> Result<HttpResponse, SubmissionError> {
    let mut title = None;
    let mut authors = None;
    let mut abstract_text = None;
    let mut keywords = None;
    let mut volume = None;
    let mut pages = None;
    let mut publication_date = None;
    let mut pdf_filename = None;

    // Process the multipart form
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().ok_or_else(|| {
            SubmissionError::ValidationError("Content disposition not found".to_string())
        })?;

        let name = content_disposition
            .get_name()
            .ok_or_else(|| SubmissionError::ValidationError("Field name not found".to_string()))?;

        match name {
            "title" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                title = Some(value);
            }
            "authors" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                authors = Some(value);
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
            "keywords" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                keywords = Some(value);
            }
            "volume" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                volume = Some(value);
            }
            "pages" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                pages = Some(value);
            }
            "publication_date" => {
                let mut value = String::new();
                while let Some(chunk) = field.next().await {
                    let data =
                        chunk.map_err(|e| SubmissionError::FileProcessingError(e.to_string()))?;
                    value.push_str(std::str::from_utf8(&data).unwrap_or(""));
                }
                publication_date = Some(value);
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
    let title = title.ok_or(SubmissionError::ValidationError(
        "Title is required".to_string(),
    ))?;
    let authors = authors.ok_or(SubmissionError::ValidationError(
        "Authors are required".to_string(),
    ))?;
    let abstract_text = abstract_text.ok_or(SubmissionError::ValidationError(
        "Abstract is required".to_string(),
    ))?;
    let keywords = keywords.ok_or(SubmissionError::ValidationError(
        "Keywords are required".to_string(),
    ))?;
    let volume = volume.ok_or(SubmissionError::ValidationError(
        "Volume is required".to_string(),
    ))?;
    let pages = pages.ok_or(SubmissionError::ValidationError(
        "Pages are required".to_string(),
    ))?;
    let publication_date_str = publication_date.ok_or(SubmissionError::ValidationError(
        "Publication date is required".to_string(),
    ))?;
    let pdf_url = pdf_filename.ok_or(SubmissionError::ValidationError(
        "PDF file is required".to_string(),
    ))?;

    // Parse publication date
    let naive_date =
        NaiveDate::parse_from_str(&publication_date_str, "%Y-%m-%d").map_err(|_| {
            SubmissionError::ValidationError("Invalid publication date format".to_string())
        })?;
    let publication_datetime =
        DateTime::<Utc>::from_utc(naive_date.and_hms_opt(0, 0, 0).unwrap(), Utc);

    // Create a Journal object
    let journal = Journal::new(
        title,
        authors,
        abstract_text,
        keywords,
        volume,
        pages,
        publication_datetime,
        format!("./data/uploads/{}", pdf_url),
    );

    // Save to database
    let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
    let repository = JournalRepository::new(conn);
    let journal_id = repository.save_journal(&journal)?;

    Ok(HttpResponse::Ok().json(UploadResponse {
        success: true,
        journal_id: journal_id as i32,
        message: "Journal uploaded successfully".to_string(),
    }))
}
