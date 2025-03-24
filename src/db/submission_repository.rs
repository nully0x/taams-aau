use crate::errors::SubmissionError;
use crate::models::submission::Submission;
use rusqlite::{params, Connection, Result};

pub struct SubmissionRepository {
    conn: Connection,
}

impl SubmissionRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn save_submission(&self, submission: &Submission) -> Result<i64, SubmissionError> {
        let result = self.conn.execute(
            "INSERT INTO submissions (full_name, email, phone, title, abstract_text, pdf_url)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                submission.full_name,
                submission.email,
                submission.phone,
                submission.title,
                submission.abstract_text,
                submission.pdf_url,
            ],
        );

        match result {
            Ok(_) => Ok(self.conn.last_insert_rowid()),
            Err(e) => Err(SubmissionError::DatabaseError(e.to_string())),
        }
    }
}
