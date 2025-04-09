use crate::{
    db::{admin_repository::AdminRepository, schema::init_db},
    errors::SubmissionError,
    utils::security::verify_password,
};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use askama::Template;
use log::{error, info, warn};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "admin/login.html")]
struct LoginTemplate {
    error: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginFormData {
    email: String,
    password: String,
}

// Show Login Form
#[get("/admin/login")]
pub async fn show_login_form(session: Session) -> impl Responder {
    // ... (same as before) ...
    if session.get::<i32>("admin_id").unwrap_or(None).is_some() {
        return HttpResponse::Found()
            .append_header(("Location", "/admin/dashboard"))
            .finish();
    }
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(LoginTemplate { error: None }.render().unwrap_or_else(|e| {
            error!("Login template render error: {}", e);
            "Error rendering login page.".to_string()
        }))
}

// Process Login
#[post("/admin/login")]
pub async fn login(
    session: Session,
    form: web::Form<LoginFormData>,
) -> Result<HttpResponse, SubmissionError> {
    // --- Clone the necessary data BEFORE the closure ---
    let email_clone = form.email.clone();
    let password_clone = form.password.clone(); // Also clone password for the second block

    // --- Use the cloned email in the first web::block ---
    let result = web::block(move || {
        let conn = init_db().map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;
        let admin_repo = AdminRepository::new(conn);
        // Use the owned clone inside the closure
        admin_repo.find_admin_by_email(&email_clone)
    })
    .await
    .map_err(|e| SubmissionError::DatabaseError(format!("Blocking error: {}", e)))??; // Handle blocking error and inner DB error

    match result {
        Some(admin) => {
            let stored_hash = admin.password_hash.clone();
            // --- Use the cloned password in the second web::block ---
            let match_result = web::block(move || verify_password(&password_clone, &stored_hash))
                .await
                .map_err(|e| {
                    SubmissionError::InternalError(format!(
                        "Password verification task failed: {}",
                        e
                    ))
                })?; // Handle blocking error for verify

            match match_result {
                Ok(true) => {
                    session.insert("admin_id", admin.id).map_err(|e| {
                        SubmissionError::StorageError(format!("Session insert error: {}", e))
                    })?;
                    session.renew();
                    // Use the original email (or the clone) for logging
                    info!("Admin login successful for email: {}", form.email);
                    Ok(HttpResponse::Found()
                        .append_header(("Location", "/admin/dashboard"))
                        .finish())
                }
                Ok(false) => {
                    warn!(
                        "Admin login failed (wrong password) for email: {}",
                        form.email
                    );
                    Ok(HttpResponse::Unauthorized()
                        .content_type("text/html; charset=utf-8")
                        .body(
                            LoginTemplate {
                                error: Some("Invalid email or password.".to_string()),
                            }
                            .render()
                            .unwrap_or_else(|e| {
                                error!("Login template render error: {}", e);
                                "Error rendering login page.".to_string()
                            }),
                        ))
                }
                Err(e) => {
                    error!(
                        "Password verification error for email {}: {}",
                        form.email, e
                    );
                    Err(SubmissionError::HashingError(format!(
                        "Password verification failed: {}",
                        e
                    )))
                }
            }
        }
        None => {
            warn!("Admin login failed (email not found): {}", form.email);
            Ok(HttpResponse::Unauthorized()
                .content_type("text/html; charset=utf-8")
                .body(
                    LoginTemplate {
                        error: Some("Invalid email or password.".to_string()),
                    }
                    .render()
                    .unwrap_or_else(|e| {
                        error!("Login template render error: {}", e);
                        "Error rendering login page.".to_string()
                    }),
                ))
        }
    }
}

// Logout Handler
#[post("/logout")]
// --- End change ---
pub async fn logout(session: Session) -> impl Responder {
    let admin_id_result = session.get::<i32>("admin_id");
    session.purge();
    match admin_id_result {
        Ok(Some(id)) => info!("Admin logout successful for ID: {}", id),
        Ok(None) => info!("Admin logout successful (no ID found in session)."),
        Err(e) => warn!("Error reading admin_id during logout: {}", e),
    };
    info!("Redirecting to login page after logout.");
    HttpResponse::Found()
        .append_header(("Location", "/admin/login")) // Redirect to absolute login path
        .finish()
}
