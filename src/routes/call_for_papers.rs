use actix_web::{get, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "call_for_papers/info.html")]
struct CallForPapersTemplate {}

#[get("/call-for-papers")]
pub async fn call_for_papers_handler() -> impl Responder {
    HttpResponse::Ok().body(CallForPapersTemplate {}.render().unwrap())
}
