use actix_web::{get, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "admin/upload.html")]
struct AdminTemplate {}

#[get("/admin/upload")]
pub async fn upload_journal_handler() -> impl Responder {
    HttpResponse::Ok().body(AdminTemplate {}.render().unwrap())
}
