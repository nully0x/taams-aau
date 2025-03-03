// src/routes/about.rs
use actix_web::{get, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

#[get("/about")]
pub async fn about_handler() -> impl Responder {
    HttpResponse::Ok().body(AboutTemplate {}.render().unwrap())
}
