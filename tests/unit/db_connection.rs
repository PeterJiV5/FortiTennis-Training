#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use tui_coach::db::connection::establish_connection;

    #[test]
    fn test_establish_connection() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db_path_str = db_path.to_str().unwrap();

        let conn = establish_connection(db_path_str).unwrap();
        
        // Test foreign key constraints are enabled
        let foreign_key_enabled: i32 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(foreign_key_enabled, 1, "Foreign key constraints should be enabled");
    }
}
