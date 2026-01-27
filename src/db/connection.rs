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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_establish_connection() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db_path_str = db_path.to_str().unwrap();

        let conn = establish_connection(db_path_str).unwrap();
        assert!(conn.is_ok());

        // Test foreign key constraints
        let foreign_key_enabled: i32 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(foreign_key_enabled, 1);
    }
}
