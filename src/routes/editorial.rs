use actix_web::{get, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "editorial/board.html")]
struct EditorialTemplate {}

#[get("/editorial-board")]
pub async fn editorial_board_handler() -> impl Responder {
    HttpResponse::Ok().body(EditorialTemplate {}.render().unwrap())
}
