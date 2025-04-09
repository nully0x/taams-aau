use crate::models::response::ValidationResponse;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    pub id: Option<i32>,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub title: String,
    pub abstract_text: String,
    pub pdf_url: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl Submission {
    pub fn new(
        full_name: String,
        email: String,
        phone: String,
        title: String,
        abstract_text: String,
        pdf_url: String,
        created_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: None,
            full_name,
            email,
            phone,
            title,
            abstract_text,
            pdf_url,
            created_at,
        }
    }

    fn is_valid_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }

    pub fn validate_submission(&self) -> Result<(), Vec<ValidationResponse>> {
        let mut validation_errors = Vec::new();

        if self.full_name.is_empty() {
            validation_errors.push(ValidationResponse {
                field: "full_name".to_string(),
                message: "Name cannot be empty".to_string(),
            });
        }

        if !Self::is_valid_email(&self.email) {
            validation_errors.push(ValidationResponse {
                field: "email".to_string(),
                message: "Invalid email address".to_string(),
            });
        }

        if self.phone.len() < 10 || self.phone.len() > 15 {
            validation_errors.push(ValidationResponse {
                field: "phone".to_string(),
                message: "Phone number must be between 10-15 digits".to_string(),
            });
        }

        if self.title.len() < 10 {
            validation_errors.push(ValidationResponse {
                field: "title".to_string(),
                message: "Title must be at least 10 characters".to_string(),
            });
        }

        if self.abstract_text.len() < 100 {
            validation_errors.push(ValidationResponse {
                field: "abstract_text".to_string(),
                message: "Abstract must be at least 100 characters".to_string(),
            });
        }

        if validation_errors.is_empty() {
            Ok(())
        } else {
            Err(validation_errors)
        }
    }

    pub fn pdf_filename(&self) -> Option<String> {
        Path::new(&self.pdf_url)
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(|s| s.to_string())
    }

    pub fn formatted_date(&self) -> String {
        self.created_at
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "N/A".to_string())
    }
}
