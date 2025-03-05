use actix_files as fs;
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::info;

mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    // Configure env_logger programmatically
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let host = "0.0.0.0";
    let port = 8080;

    info!("Starting server on http://{}:{}...", host, port); // Log a message
    HttpServer::new(|| {
        App::new()
            // Serve static files.
            .service(
                fs::Files::new("/static", "./src/static")
                    .show_files_listing()
                    .use_last_modified(true)
                    .prefer_utf8(true)
                    .disable_content_disposition(),
            )
            // Register ALL your routes here:
            .service(routes::landing::landing_handler)
            .service(routes::journals::current_journals_handler)
            .service(routes::journals::past_journals_handler)
            .service(routes::journals::journal_detail_handler)
            .service(routes::admin::upload_journal_handler)
            .service(routes::about::about_handler)
            .service(routes::submissions::submit_paper_handler)
            .service(routes::editorial::editorial_board_handler)
            .service(routes::call_for_papers::call_for_papers_handler)
    })
    .bind((host, port))?
    .run()
    .await
}
