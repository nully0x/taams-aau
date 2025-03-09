// src/routes/journals.rs
use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;

// --- Current Journals ---
#[derive(Template)]
#[template(path = "journals/current.html")]
struct CurrentJournalsTemplate {}

#[get("/journals/current")]
pub async fn current_journals_handler() -> impl Responder {
    HttpResponse::Ok().body(CurrentJournalsTemplate {}.render().unwrap())
}

// --- Past Journals ---
#[derive(Template)]
#[template(path = "journals/past.html")]
struct PastJournalsTemplate {}

#[get("/journals/past")]
pub async fn past_journals_handler() -> impl Responder {
    HttpResponse::Ok().body(PastJournalsTemplate {}.render().unwrap())
}

// --- Journal Detail (Placeholder - will need a database later) ---
#[derive(Template)]
#[template(path = "journals/details.html")]
struct JournalDetailTemplate {
    id: i32, // Placeholder ID
}
#[get("/journals/{id}")]
pub async fn journal_detail_handler(id: web::Path<i32>) -> impl Responder {
    let journal_id = id.into_inner();
    HttpResponse::Ok().body(JournalDetailTemplate { id: journal_id }.render().unwrap())
}

#[derive(Template)]
#[template(path = "journals/journal.html")]
struct Journal {}

#[get("/journal")]
pub async fn journal_handler() -> impl Responder {
    HttpResponse::Ok().body(Journal {}.render().unwrap())
}
