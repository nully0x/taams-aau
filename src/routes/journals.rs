// src/routes/journals.rs
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;
use chrono::{DateTime, Datelike, Utc};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;

use crate::db::journal_repository::JournalRepository;
use crate::db::schema::init_db;
use crate::errors::SubmissionError;
use crate::models::journals::Journal;

#[derive(Template)]
#[template(path = "journals/details.html")]
struct JournalDetailTemplate {
    journal: Journal,
    id_string: String, // Changed from i32 to String
    is_admin: bool,
}

#[derive(Template)]
#[template(path = "journals/journal.html")]
struct JournalTemplate {
    journals: Vec<Journal>,
    archives: BTreeMap<i32, BTreeMap<i32, Vec<Journal>>>, // Grouped archives
    all_journals_json: String,
}

#[derive(Deserialize)]
pub struct JournalQueryParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub category: Option<String>,
}

#[get("/journals/{id}")]
pub async fn journal_detail_handler(
    id: web::Path<i32>,
    session: Session, // Add session parameter
) -> Result<HttpResponse, SubmissionError> {
    let journal_id = id.into_inner();

    let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
    let repository = JournalRepository::new(conn);
    let journal = repository.get_journal_by_id(journal_id)?;

    // Check if user is admin
    let is_admin = session
        .get::<i32>("admin_id")
        .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?
        .is_some();

    Ok(HttpResponse::Ok().body(
        JournalDetailTemplate {
            journal,
            id_string: journal_id.to_string(),
            is_admin, // Pass admin status to template
        }
        .render()
        .unwrap(),
    ))
}

#[get("/journal")]
pub async fn journal_handler() -> Result<HttpResponse, SubmissionError> {
    let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
    let repository = JournalRepository::new(conn);
    let journals = repository.get_all_journals(12, 0)?;

    Ok(HttpResponse::Ok().body(JournalTemplate { journals }.render().unwrap()))
}

#[get("/api/journals")]
pub async fn journal_api_handler(
    query: web::Query<JournalQueryParams>,
) -> Result<HttpResponse, SubmissionError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(12);
    let offset = (page - 1) * limit;
    let category = query.category.as_deref().unwrap_or("all");

    let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
    let repository = JournalRepository::new(conn);

    let journals = match category {
        "latest" => repository.get_latest_journals(limit)?,
        "current" => repository.get_current_edition(limit)?,
        "past" => repository.get_past_issues(limit, offset)?,
        _ => repository.get_all_journals(limit, offset)?,
    };

    Ok(HttpResponse::Ok().json(json!({
        "journals": journals,
        "hasMore": journals.len() == limit as usize
    })))
}

// Helper function to parse volume string
// Returns Option<(volume_number, issue_number)>
fn parse_volume_issue(volume_str: &str) -> Option<(i32, i32)> {
    // Use lazy_static! or once_cell for better performance if called often,
    // but simple Regex::new is fine here. Case-insensitive matching.
    let re = Regex::new(r"(?i)Vol\.\s*(\d+)\s*No\.\s*(\d+)").unwrap();
    if let Some(caps) = re.captures(volume_str) {
        // Try to parse captured groups as i32
        let vol_res = caps.get(1).and_then(|m| m.as_str().parse::<i32>().ok());
        let iss_res = caps.get(2).and_then(|m| m.as_str().parse::<i32>().ok());

        if let (Some(vol), Some(iss)) = (vol_res, iss_res) {
            Some((vol, iss))
        } else {
            log::warn!(
                "Failed to parse volume/issue numbers from captures in string: {}",
                volume_str
            );
            None // Parsing numbers failed
        }
    } else {
        log::warn!(
            "Regex did not match expected 'Vol. X No. Y' format in string: {}",
            volume_str
        );
        None // Regex didn't match
    }
}
