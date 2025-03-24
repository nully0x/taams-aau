use crate::errors::SubmissionError;
use crate::models::journals::Journal;
use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::{params, Connection, Result};

pub struct JournalRepository {
    conn: Connection,
}

impl JournalRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
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
            .query_row(params![id], |row| {
                let timestamp: i64 = row.get(7)?;
                let created_at_str: Option<String> = row.get(9)?;

                let naive_dt = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
                let publication_date = DateTime::<Utc>::from_utc(naive_dt, Utc);

                let created_at = created_at_str.and_then(|s| {
                    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                        .ok()
                        .map(|dt| DateTime::<Utc>::from_utc(dt, Utc))
                });

                Ok(Journal {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    authors: row.get(2)?,
                    abstract_text: row.get(3)?,
                    keywords: row.get(4)?,
                    volume: row.get(5)?,
                    pages: row.get(6)?,
                    publication_date,
                    pdf_url: row.get(8)?,
                    created_at,
                })
            })
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

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
            .query_map(params![limit, offset], |row| {
                let timestamp: i64 = row.get(7)?;
                let created_at_str: Option<String> = row.get(9)?;

                let naive_dt = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
                let publication_date = DateTime::<Utc>::from_utc(naive_dt, Utc);

                let created_at = created_at_str.and_then(|s| {
                    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                        .ok()
                        .map(|dt| DateTime::<Utc>::from_utc(dt, Utc))
                });

                Ok(Journal {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    authors: row.get(2)?,
                    abstract_text: row.get(3)?,
                    keywords: row.get(4)?,
                    volume: row.get(5)?,
                    pages: row.get(6)?,
                    publication_date,
                    pdf_url: row.get(8)?,
                    created_at,
                })
            })
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))?;

        let mut journals = Vec::new();
        for journal in journal_iter {
            journals.push(journal.map_err(|e| SubmissionError::DatabaseError(e.to_string()))?);
        }

        Ok(journals)
    }

    pub fn get_latest_journals(&self, limit: i32) -> Result<Vec<Journal>, SubmissionError> {
        self.get_all_journals(limit, 0)
    }
}
