// src/routes/journals.rs
use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::db::journal_repository::JournalRepository;
use crate::db::schema::init_db;
use crate::errors::SubmissionError;
use crate::models::journals::Journal;

#[derive(Template)]
#[template(path = "journals/current.html")]
struct CurrentJournalsTemplate {
    journals: Vec<Journal>,
}

#[derive(Template)]
#[template(path = "journals/past.html")]
struct PastJournalsTemplate {
    journals: Vec<Journal>,
}

#[derive(Template)]
#[template(path = "journals/details.html")]
struct JournalDetailTemplate {
    journal: Journal,
    id: i32,
}

#[derive(Template)]
#[template(path = "journals/journal.html")]
struct JournalTemplate {
    journals: Vec<Journal>,
}

#[derive(Deserialize)]
pub struct JournalQueryParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[get("/journals/current")]
pub async fn current_journals_handler() -> Result<HttpResponse, SubmissionError> {
    let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
    let repository = JournalRepository::new(conn);
    let journals = repository.get_latest_journals(10)?;

    Ok(HttpResponse::Ok().body(CurrentJournalsTemplate { journals }.render().unwrap()))
}

#[get("/journals/past")]
pub async fn past_journals_handler(
    query: web::Query<JournalQueryParams>,
) -> Result<HttpResponse, SubmissionError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
    let repository = JournalRepository::new(conn);
    let journals = repository.get_all_journals(limit, offset)?;

    Ok(HttpResponse::Ok().body(PastJournalsTemplate { journals }.render().unwrap()))
}

#[get("/journals/{id}")]
pub async fn journal_detail_handler(id: web::Path<i32>) -> Result<HttpResponse, SubmissionError> {
    let journal_id = id.into_inner();

    let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
    let repository = JournalRepository::new(conn);
    let journal = repository.get_journal_by_id(journal_id)?;

    Ok(HttpResponse::Ok().body(
        JournalDetailTemplate {
            journal,
            id: journal_id,
        }
        .render()
        .unwrap(),
    ))
}

#[get("/journal")]
pub async fn journal_handler(
    query: web::Query<JournalQueryParams>,
) -> Result<HttpResponse, SubmissionError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(12);
    let offset = (page - 1) * limit;

    let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
    let repository = JournalRepository::new(conn);
    let journals = repository.get_all_journals(limit, offset)?;

    Ok(HttpResponse::Ok().body(JournalTemplate { journals }.render().unwrap()))
}
