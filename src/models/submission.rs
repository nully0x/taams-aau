use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Submission {
    pub id: Option<i32>,

    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub full_name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(
        min = 10,
        max = 15,
        message = "Phone number must be between 10-15 digits"
    ))]
    #[validate(regex(path = "PHONE_REGEX", message = "Invalid phone number format"))]
    pub phone: String,

    #[validate(length(min = 10, message = "Title must be at least 10 characters"))]
    pub title: String,

    #[validate(length(min = 100, message = "Abstract must be at least 100 characters"))]
    pub abstract_text: String,

    pub pdf_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    pub id: Option<i32>,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    pub title: String,
    pub abstract_text: String,
    pub pdf_url: String,
}

impl Submission {
    pub fn new(
        full_name: String,
        email: String,
        phone: String,
        title: String,
        abstract_text: String,
        pdf_url: String,
    ) -> Self {
        Self {
            id: None,
            full_name,
            email,
            phone,
            title,
            abstract_text,
            pdf_url,
        }
    }

    pub fn validate_submission(&self) -> Result<(), Vec<ValidationResponse>> {
        if let Err(errors) = self.validate() {
            let validation_errors = errors
                .field_errors()
                .iter()
                .map(|(field, error_vec)| ValidationResponse {
                    field: field.to_string(),
                    message: error_vec[0].message.clone().unwrap_or_default().to_string(),
                })
                .collect();
            return Err(validation_errors);
        }
        Ok(())
    }
}
