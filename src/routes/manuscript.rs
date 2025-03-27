use actix_web::{get, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "manuscript/manuscript.html")]
struct ManuscriptTemplate {}

#[get("/manuscript")]
pub async fn manuscript_guide() -> impl Responder {
    HttpResponse::Ok().body(ManuscriptTemplate {}.render().unwrap())
}
