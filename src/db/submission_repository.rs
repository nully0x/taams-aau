use crate::{errors::SubmissionError, models::submission::Submission};
use chrono::{DateTime, NaiveDateTime, Utc}; // Add chrono
use rusqlite::{params, Connection, Result as RusqliteResult}; // Specify RusqliteResult

pub struct SubmissionRepository {
    conn: Connection,
}

impl SubmissionRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    // --- map_row_to_submission helper ---
    fn map_row_to_submission(row: &rusqlite::Row) -> RusqliteResult<Submission> {
        let created_at_str: Option<String> = row.get(7)?;

        let created_at = created_at_str.and_then(|s| {
            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .ok()
                .map(|dt| DateTime::<Utc>::from_utc(dt, Utc))
        });

        Ok(Submission {
            id: Some(row.get(0)?),
            full_name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
            title: row.get(4)?,
            abstract_text: row.get(5)?,
            pdf_url: row.get(6)?,
            created_at,
        })
    }

    // --- save_submission remains the same ---
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
                submission.pdf_url, // Assumes pdf_url in Submission struct is the desired path
            ],
        );

        match result {
            Ok(_) => Ok(self.conn.last_insert_rowid()),
            Err(e) => Err(SubmissionError::DatabaseError(e.to_string())),
        }
    }

    // --- Add get_all_submissions ---
    pub fn get_all_submissions(&self) -> Result<Vec<Submission>, SubmissionError> {
        let mut stmt = self
            .conn
            .prepare(
                // Added created_at to SELECT if your table has it and you map it
                "SELECT id, full_name, email, phone, title, abstract_text, pdf_url, created_at
             FROM submissions ORDER BY created_at DESC",
            )
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let submission_iter = stmt
            .query_map([], Self::map_row_to_submission) // No params needed here
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let submissions: Result<Vec<Submission>, _> = submission_iter
            .map(|res| res.map_err(|e| SubmissionError::DatabaseError(e.to_string())))
            .collect();

        submissions
    }

    // --- Add get_submission_by_id (needed for download/details) ---
    pub fn get_submission_by_id(&self, id: i32) -> Result<Submission, SubmissionError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, full_name, email, phone, title, abstract_text, pdf_url, created_at
              FROM submissions WHERE id = ?1",
            )
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        stmt.query_row(params![id], Self::map_row_to_submission)
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    SubmissionError::NotFound(format!("Submission with ID {} not found", id))
                }
                _ => SubmissionError::DatabaseError(e.to_string()),
            })
    }

    pub fn get_recent_submissions(&self, limit: i32) -> Result<Vec<Submission>, SubmissionError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, full_name, email, phone, title, abstract_text, pdf_url, created_at
                     FROM submissions
                     ORDER BY created_at DESC
                     LIMIT ?1",
            )
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let submission_iter = stmt
            .query_map([limit], Self::map_row_to_submission)
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let submissions: Result<Vec<Submission>, _> = submission_iter
            .map(|res| res.map_err(|e| SubmissionError::DatabaseError(e.to_string())))
            .collect();

        submissions
    }
}
