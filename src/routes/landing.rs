use actix_web::{get, HttpResponse, Responder};
use askama::Template;
use crate::config::get_journal_config;

#[derive(Template)]
#[template(path = "landing.html")]
struct LandingTemplate {
    journal_name: String,
    journal_field: String,
}

#[get("/")]
pub async fn landing_handler() -> impl Responder {
    let config = get_journal_config();
    let template = LandingTemplate {
        journal_name: config.name,
        journal_field: config.field,
    };

    HttpResponse::Ok().body(template.render().unwrap())
}
