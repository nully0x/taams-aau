use crate::{errors::SubmissionError, models::admin::Admin};
use rusqlite::{params, Connection, OptionalExtension, Result as RusqliteResult};

pub struct AdminRepository {
    conn: Connection,
}

impl AdminRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    // Find admin by email
    pub fn find_admin_by_email(&self, email: &str) -> Result<Option<Admin>, SubmissionError> {
        self.conn
            .query_row(
                "SELECT id, email, password_hash FROM admins WHERE email = ?1",
                params![email],
                |row| {
                    Ok(Admin {
                        id: row.get(0)?,
                        email: row.get(1)?,
                        password_hash: row.get(2)?,
                    })
                },
            )
            .optional() // Makes it return Option<Admin>
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))
    }

    // Create admin (used for seeding)
    pub fn create_admin(&self, email: &str, password_hash: &str) -> Result<i64, SubmissionError> {
        self.conn
            .execute(
                "INSERT INTO admins (email, password_hash) VALUES (?1, ?2)",
                params![email, password_hash],
            )
            .map(|_| self.conn.last_insert_rowid()) // Return the ID
            .map_err(|e| SubmissionError::DatabaseError(e.to_string()))
    }
}
