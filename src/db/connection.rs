use crate::utils::Result;
use rusqlite::Connection;
use std::path::Path;

pub fn establish_connection(db_path: &str) -> Result<Connection> {
    let db_path = Path::new(db_path);
    // Create data directory if it doesn't exist
    if let Some(parent) = Path::new(db_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(db_path)?;

    // Enable foreign key constraints
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    Ok(conn)
}
