use rusqlite::Connection;

pub fn init_db() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("submissions.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS submissions (
            id INTEGER PRIMARY KEY,
            full_name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone TEXT NOT NULL,
            title TEXT NOT NULL,
            abstract TEXT NOT NULL,
            pdf_url TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    Ok(conn)
}
