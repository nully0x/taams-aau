use crate::errors::SubmissionError;
use crate::models::journals::Journal;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{error, info};
use rusqlite::{params, Connection, Result as RusqliteResult};
use std::fs; // Import fs for file deletion
use std::path::Path; // Import Path

pub struct JournalRepository {
    conn: Connection,
}

impl JournalRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    fn map_row_to_journal(row: &rusqlite::Row) -> RusqliteResult<Journal> {
        let timestamp: i64 = row.get(7)?;
        let created_at_str: Option<String> = row.get(9)?;
        let pdf_filename: String = row.get(8)?; // Get filename from DB

        let naive_dt = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
        let publication_date = DateTime::<Utc>::from_utc(naive_dt, Utc);

        let created_at = match created_at_str {
            Some(s) => NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .ok()
                .map(|dt| DateTime::<Utc>::from_utc(dt, Utc)),
            None => None,
        };

        Ok(Journal {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            authors: row.get(2)?,
            abstract_text: row.get(3)?,
            keywords: row.get(4)?,
            volume: row.get(5)?,
            pages: row.get(6)?,
            publication_date,
            pdf_url: pdf_filename, // Store only filename in the struct field now
            created_at,
        })
    }

    pub fn save_journal(&self, journal: &Journal) -> Result<i64, SubmissionError> {
        let result = self.conn.execute(
            "INSERT INTO journals (title, authors, abstract_text, keywords, volume, pages, publication_date, pdf_url)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                journal.title,
                journal.authors,
                journal.abstract_text,
                journal.keywords,
                journal.volume,
                journal.pages,
                journal.publication_date.timestamp(),
                journal.pdf_url,
            ],
        );

        match result {
            Ok(_) => Ok(self.conn.last_insert_rowid()),
            Err(e) => Err(SubmissionError::DatabaseError(e.to_string())),
        }
    }

    pub fn get_journal_by_id(&self, id: i32) -> Result<Journal, SubmissionError> {
        let mut stmt = self.conn.prepare(
                "SELECT id, title, authors, abstract_text, keywords, volume, pages, publication_date, pdf_url, created_at
                 FROM journals WHERE id = ?1"
            ).map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journal = stmt
            .query_row(params![id], Self::map_row_to_journal)
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    SubmissionError::NotFound(format!("Journal with ID {} not found", id))
                }
                _ => SubmissionError::DatabaseError(e.to_string()),
            })?;

        Ok(journal)
    }

    pub fn get_all_journals(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Journal>, SubmissionError> {
        let mut stmt = self.conn.prepare(
                "SELECT id, title, authors, abstract_text, keywords, volume, pages, publication_date, pdf_url, created_at
                 FROM journals ORDER BY publication_date DESC LIMIT ?1 OFFSET ?2"
            ).map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journal_iter = stmt
            .query_map(params![limit, offset], Self::map_row_to_journal)
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journals: Result<Vec<Journal>, _> = journal_iter
            .map(|res| res.map_err(|e| SubmissionError::DatabaseError(e.to_string())))
            .collect();

        journals
    }

    pub fn get_latest_journals(&self, limit: i32) -> Result<Vec<Journal>, SubmissionError> {
        let mut stmt = self.conn.prepare(
                    "SELECT id, title, authors, abstract_text, keywords, volume, pages, publication_date, pdf_url, created_at
                     FROM journals
                     ORDER BY publication_date DESC
                     LIMIT ?1"
                ).map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journal_iter = stmt
            .query_map(params![limit], Self::map_row_to_journal)
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journals: Result<Vec<Journal>, _> = journal_iter
            .map(|res| res.map_err(|e| SubmissionError::DatabaseError(e.to_string())))
            .collect();

        journals
    }

    pub fn get_current_edition(&self, limit: i32) -> Result<Vec<Journal>, SubmissionError> {
        let mut stmt = self.conn.prepare(
                    "SELECT id, title, authors, abstract_text, keywords, volume, pages, publication_date, pdf_url, created_at
                     FROM journals
                     WHERE volume = (SELECT volume FROM journals ORDER BY publication_date DESC LIMIT 1)
                     LIMIT ?1"
                ).map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journal_iter = stmt
            .query_map(params![limit], Self::map_row_to_journal)
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journals: Result<Vec<Journal>, _> = journal_iter
            .map(|res| res.map_err(|e| SubmissionError::DatabaseError(e.to_string())))
            .collect();

        journals
    }

    pub fn get_past_issues(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Journal>, SubmissionError> {
        let mut stmt = self.conn.prepare(
                    "SELECT id, title, authors, abstract_text, keywords, volume, pages, publication_date, pdf_url, created_at
                     FROM journals
                     WHERE volume != (SELECT volume FROM journals ORDER BY publication_date DESC LIMIT 1)
                     ORDER BY publication_date DESC
                     LIMIT ?1 OFFSET ?2"
                ).map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journal_iter = stmt
            .query_map(params![limit, offset], Self::map_row_to_journal)
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journals: Result<Vec<Journal>, _> = journal_iter
            .map(|res| res.map_err(|e| SubmissionError::DatabaseError(e.to_string())))
            .collect();

        journals
    }

    pub fn delete_journal_by_id(&self, id: i32) -> Result<(), SubmissionError> {
        // 1. Get the journal details first to find the PDF filename
        let journal = self.get_journal_by_id(id)?; // This now returns NotFound error if ID doesn't exist

        // 2. Attempt to delete the database record
        let rows_affected = self
            .conn
            .execute("DELETE FROM journals WHERE id = ?1", params![id])
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        // Check if the record was actually deleted
        if rows_affected == 0 {
            // This case should ideally be caught by get_journal_by_id, but added as a safeguard
            return Err(SubmissionError::NotFound(format!(
                "Journal with ID {} could not be deleted (already removed?)",
                id
            )));
        }

        info!("Successfully deleted journal record with ID: {}", id);

        // 3. Construct the full path to the PDF file
        // Assuming pdf_url stored in the Journal struct is just the filename now
        let pdf_path = Path::new("./data/uploads").join(&journal.pdf_url);

        // 4. Attempt to delete the file
        match fs::remove_file(&pdf_path) {
            Ok(_) => {
                info!("Successfully deleted PDF file: {:?}", pdf_path);
                Ok(())
            }
            Err(e) => {
                // Log the error but don't necessarily fail the entire operation
                // if the DB record was deleted. The file might have been manually removed earlier.
                error!(
                    "Failed to delete PDF file {:?}: {}. DB record was deleted.",
                    pdf_path, e
                );
                // Depending on requirements, you might want to return an error here instead.
                // For now, we consider DB deletion the primary success indicator.
                Err(SubmissionError::StorageError(format!(
                    "Journal record deleted, but failed to remove file {:?}: {}",
                    pdf_path, e
                )))
            }
        }
    }

    // New method to get all journals without pagination for archive grouping
    pub fn get_all_journals_for_archive(&self) -> Result<Vec<Journal>, SubmissionError> {
        let mut stmt = self.conn.prepare(
                   "SELECT id, title, authors, abstract_text, keywords, volume, pages, publication_date, pdf_url, created_at
                    FROM journals ORDER BY publication_date DESC" // Order by date remains useful
               ).map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journal_iter = stmt
            .query_map([], Self::map_row_to_journal) // No parameters for limit/offset
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let journals: Result<Vec<Journal>, _> = journal_iter
            .map(|res| res.map_err(|e| SubmissionError::DatabaseError(e.to_string())))
            .collect();

        journals
    }
}
