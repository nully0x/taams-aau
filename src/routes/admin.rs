use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};
use actix_web::{delete, get, post, web, Error as ActixError, HttpRequest, HttpResponse}; // Keep ActixError
use askama::Template;
use chrono::{DateTime, NaiveDate, Utc};
use futures::StreamExt;
use log::{debug, error, warn};
use serde_json::json;
use std::path::PathBuf; // Use PathBuf

use crate::{
    db::{
        journal_repository::JournalRepository, schema::init_db,
        submission_repository::SubmissionRepository,
    },
    errors::SubmissionError,
    models::{journals::Journal, response::UploadResponse, submission::Submission},
    utils, // Import the utils module
};

type AuthResult = Result<i32, HttpResponse>;

fn check_authentication(session: &Session) -> AuthResult {
    match session.get::<i32>("admin_id") {
        Ok(Some(admin_id)) => Ok(admin_id),
        _ => {
            warn!("Unauthorized access attempt to admin route.");
            Err(HttpResponse::Found()
                .append_header(("Location", "/admin/login")) // Redirect path
                .finish())
        }
    }
}

// --- Templates ---
#[derive(Template)]
#[template(path = "admin/index.html")]
struct AdminDashboardTemplate {
    current_page: &'static str,
    recent_submissions: Vec<Submission>,
}

#[derive(Template)]
#[template(path = "admin/upload.html")]
struct AdminUploadTemplate {
    current_page: &'static str,
}

#[derive(Template)]
#[template(path = "admin/submitted.html")]
struct AdminSubmissionsTemplate {
    submissions: Vec<Submission>,
    current_page: &'static str,
    title: &'static str,
}

#[derive(Template)]
#[template(path = "admin/login.html")]
struct AdminLoginTemplate {
    error: Option<String>,
}

// --- Handlers ---

#[get("/login")]
pub async fn admin_login_form_handler() -> HttpResponse {
    // No auth check needed to view the login form
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            AdminLoginTemplate { error: None }
                .render()
                .unwrap_or_else(|e| {
                    error!("Login template render error: {}", e);
                    "Error rendering login page.".to_string()
                }),
        )
}

#[get("/dashboard")]
pub async fn admin_dashboard_handler(session: Session) -> Result<HttpResponse, ActixError> {
    match check_authentication(&session) {
        Ok(_admin_id) => {
            // Get recent submissions
            let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
            let sub_repo = SubmissionRepository::new(conn);
            let recent_submissions = sub_repo.get_recent_submissions(10)?;

            let template = AdminDashboardTemplate {
                current_page: "dashboard",
                recent_submissions,
            };

            Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(template.render().map_err(|e| {
                    error!("Dashboard template render error: {}", e);
                    actix_web::error::ErrorInternalServerError("Template error")
                })?))
        }
        Err(redirect) => Ok(redirect),
    }
}

#[get("/upload")]
pub async fn upload_journal_handler(session: Session) -> Result<HttpResponse, ActixError> {
    match check_authentication(&session) {
        Ok(_) => Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            // Pass the current page identifier
            .body(
                AdminUploadTemplate {
                    current_page: "upload",
                }
                .render()
                .map_err(|e| {
                    error!("Upload template render error: {}", e);
                    actix_web::error::ErrorInternalServerError("Template error")
                })?,
            )),
        Err(redirect) => Ok(redirect),
    }
}

#[post("/upload")]
pub async fn process_upload(
    session: Session,
    mut payload: Multipart,
) -> Result<HttpResponse, ActixError> {
    // Return ActixError
    match check_authentication(&session) {
        Ok(_) => {
            // Use block level async, map SubmissionError to ActixError at the end
            let result: Result<HttpResponse, SubmissionError> = async move {
                let mut title: Option<String> = None;
                let mut authors: Option<String> = None;
                let mut abstract_text: Option<String> = None;
                let mut keywords: Option<String> = None;
                let mut volume: Option<String> = None;
                let mut pages: Option<String> = None;
                let mut publication_date: Option<String> = None;
                let mut pdf_filename: Option<String> = None;

                while let Some(field_result) = payload.next().await {
                    let mut field = field_result.map_err(|e| {
                        SubmissionError::FileProcessingError(format!("Multipart error: {}", e))
                    })?;
                    let content_disposition =
                        field.content_disposition().cloned().ok_or_else(|| {
                            SubmissionError::ValidationError(
                                "Content disposition missing".to_string(),
                            )
                        })?; // Clone disposition

                    let name = content_disposition.get_name().ok_or_else(|| {
                        SubmissionError::ValidationError("Field name missing".to_string())
                    })?;

                    match name {
                        "title" => title = Some(utils::read_field(field).await?),
                        "authors" => authors = Some(utils::read_field(field).await?),
                        "abstract_text" => abstract_text = Some(utils::read_field(field).await?),
                        "keywords" => keywords = Some(utils::read_field(field).await?),
                        "volume" => volume = Some(utils::read_field(field).await?),
                        "pages" => pages = Some(utils::read_field(field).await?),
                        "publication_date" => {
                            publication_date = Some(utils::read_field(field).await?)
                        }
                        "pdf" => pdf_filename = Some(utils::save_uploaded_file(field).await?),
                        _ => {
                            // Drain ignored fields explicitly
                            while field.next().await.is_some() {}
                        }
                    }
                }

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
                let publication_date_str = publication_date.ok_or(
                    SubmissionError::ValidationError("Publication date is required".to_string()),
                )?;
                let pdf_url = pdf_filename.ok_or(SubmissionError::ValidationError(
                    "PDF file is required".to_string(),
                ))?;

                let naive_date = NaiveDate::parse_from_str(&publication_date_str, "%Y-%m-%d")
                    .map_err(|_| {
                        SubmissionError::ValidationError(
                            "Invalid publication date format".to_string(),
                        )
                    })?;
                let publication_datetime =
                    DateTime::<Utc>::from_utc(naive_date.and_hms_opt(0, 0, 0).unwrap(), Utc);

                let journal = Journal::new(
                    title,
                    authors,
                    abstract_text,
                    keywords,
                    volume,
                    pages,
                    publication_datetime,
                    pdf_url,
                );

                let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
                let repository = JournalRepository::new(conn);
                let journal_id = repository.save_journal(&journal)?; // This returns Result<_, SubmissionError>

                Ok(HttpResponse::Ok().json(UploadResponse {
                    success: true,
                    journal_id: journal_id as i32,
                    message: "Journal uploaded successfully".to_string(),
                }))
            }
            .await; // await the result of the inner async block

            result.map_err(ActixError::from) // Map SubmissionError -> ActixError if it's Err
        }
        Err(redirect) => Ok(redirect), // Return the redirect HttpResponse if auth failed
    }
}

#[delete("/journals/{id}")]
pub async fn delete_journal_handler(
    session: Session,
    id: web::Path<i32>,
) -> Result<HttpResponse, ActixError> {
    match check_authentication(&session) {
        Ok(_) => {
            let result: Result<HttpResponse, SubmissionError> = async move {
                let journal_id = id.into_inner();
                debug!("Attempting to delete journal with ID: {}", journal_id);

                let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
                let repository = JournalRepository::new(conn);
                repository.delete_journal_by_id(journal_id)?; // Returns Result<(), SubmissionError>

                Ok(HttpResponse::Ok().json(json!({
                    "success": true,
                    "message": format!("Journal with ID {} deleted successfully", journal_id)
                })))
            }
            .await; // Await the inner async block

            result.map_err(ActixError::from) // Map SubmissionError -> ActixError
        }
        Err(redirect) => Ok(redirect), // Return the redirect HttpResponse
    }
}

#[get("/submissions")]
pub async fn admin_submissions_handler(session: Session) -> Result<HttpResponse, ActixError> {
    match check_authentication(&session) {
        Ok(_) => {
            let result: Result<HttpResponse, SubmissionError> = async move {
                let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
                let sub_repo = SubmissionRepository::new(conn);
                let submissions = sub_repo.get_all_submissions()?;

                // Pass the current page identifier
                let template = AdminSubmissionsTemplate {
                    submissions,
                    current_page: "submissions",
                    title: "Admin Submissions",
                };
                Ok(HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(template.render().map_err(|e| {
                        error!("Submissions template render error: {}", e);
                        SubmissionError::FileProcessingError(format!("Template error: {}", e))
                    })?))
            }
            .await;
            result.map_err(ActixError::from)
        }
        Err(redirect) => Ok(redirect),
    }
}

#[get("/submissions/{id}/download")]
pub async fn download_submission_handler(
    session: Session,
    req: HttpRequest, // Need request to build absolute paths if needed, but NamedFile handles relative
    id: web::Path<i32>,
) -> Result<HttpResponse, ActixError> {
    match check_authentication(&session) {
        Ok(_) => {
            let submission_id = id.into_inner();
            debug!(
                "Attempting to download submission PDF for ID: {}",
                submission_id
            );

            let result: Result<NamedFile, SubmissionError> = async move {
                let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
                let sub_repo = SubmissionRepository::new(conn);
                let submission = sub_repo.get_submission_by_id(submission_id)?;

                // pdf_url in submission should be like "./data/uploads/uuid.pdf"
                let file_path = PathBuf::from(&submission.pdf_url);

                // Extract filename for content disposition
                let filename = file_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("submission.pdf") // Fallback filename
                    .to_string();

                // Attempt to open the file
                let named_file = NamedFile::open_async(&file_path).await.map_err(|io_err| {
                    error!(
                        "Failed to open submission file {:?} for ID {}: {}",
                        file_path, submission_id, io_err
                    );
                    // Map IO error to NotFound or InternalError appropriately
                    if io_err.kind() == std::io::ErrorKind::NotFound {
                        SubmissionError::NotFound(format!(
                            "Submission file not found for ID {}",
                            submission_id
                        ))
                    } else {
                        SubmissionError::StorageError(format!(
                            "Error opening submission file: {}",
                            io_err
                        ))
                    }
                })?;

                // Set headers for download
                Ok(named_file.set_content_disposition(ContentDisposition {
                    disposition: DispositionType::Attachment, // Force download
                    parameters: vec![DispositionParam::Filename(filename)],
                }))
            }
            .await; // await the inner block

            // Map SubmissionError to ActixError OR directly return NamedFile response
            match result {
                Ok(named_file) => Ok(named_file.into_response(&req)), // Convert NamedFile to HttpResponse
                Err(e) => Err(ActixError::from(e)), // Convert SubmissionError to ActixError
            }
        }
        Err(redirect) => Ok(redirect),
    }
}
