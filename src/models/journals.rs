use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Journal {
    pub id: Option<i32>,
    pub title: String,
    pub authors: String,
    pub abstract_text: String,
    pub keywords: String,
    pub volume: String,
    pub pages: String,
    pub publication_date: DateTime<Utc>,
    pub pdf_url: String,
    pub created_at: Option<DateTime<Utc>>,
}

impl Journal {
    pub fn new(
        title: String,
        authors: String,
        abstract_text: String,
        keywords: String,
        volume: String,
        pages: String,
        publication_date: DateTime<Utc>,
        pdf_url: String,
    ) -> Self {
        Self {
            id: None,
            title,
            authors,
            abstract_text,
            keywords,
            volume,
            pages,
            publication_date,
            pdf_url,
            created_at: None,
        }
    }
}
