//src/routes/submissions.rs
use actix_web::{get, HttpResponse, Responder};
use askama::Template;

//src/routes/submissions.rs
#[derive(Template)]
#[template(path = "submissions/submit.html")]
struct SubmissionsTemplate {}

#[get("/submit")]
pub async fn submit_paper_handler() -> impl Responder {
    HttpResponse::Ok().body(SubmissionsTemplate {}.render().unwrap())
}
