use actix_web::{get, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingTemplate {}

#[get("/")]
pub async fn landing_handler() -> impl Responder {
    HttpResponse::Ok().body(LandingTemplate {}.render().unwrap())
}
