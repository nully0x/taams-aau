use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Journal {
    pub id: Option<i32>,
    pub title: String,
    pub authors: String,
    pub abstract_text: String,
    pub keywords: String,
    pub volume_number: i32,
    pub issue_number: Option<i32>,
    pub is_special_edition: bool,
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
        volume_number: i32,
        issue_number: Option<i32>,
        is_special_edition: bool,
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
            volume_number,
            issue_number,
            is_special_edition,
            pages,
            publication_date,
            pdf_url,
            created_at: None,
        }
    }
    pub fn id_string(&self) -> String {
        self.id.map_or_else(String::new, |id| id.to_string())
    }

    pub fn pdf_url(&self) -> String {
        format!("data/uploads/{}", self.pdf_url)
    }

    // Helper to display volume/issue nicely
    pub fn volume_issue_display(&self) -> String {
        if self.is_special_edition {
            format!("Vol. {} Special Edition", self.volume_number)
        } else {
            format!(
                "Vol. {} No. {}",
                self.volume_number,
                self.issue_number.unwrap_or(0)
            )
        }
    }
}
