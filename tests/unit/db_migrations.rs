#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use tui_coach::db::connection::establish_connection;
    use tui_coach::db::migrations::run_migrations;
    use tui_coach::utils::Result;

    #[test]
    fn test_run_migrations() -> Result<()> {
        let dir = tempdir().map_err(|e| tui_coach::utils::error::AppError::Io(e))?;
        let db_path = dir.path().join("test.db");
        let db_path_str = db_path.to_str().unwrap();
        let conn = establish_connection(db_path_str)?;

        // Should not panic here
        run_migrations(&conn)?;

        // Verify tables exist by querying table count
        let table_count: i32 = conn
            .query_row("SELECT COUNT(*) FROM sqlite_master WHERE type='table'", [], |row| row.get(0))?;
        assert!(table_count > 0, "No tables were created");

        // Check if specific tables exist
        let mut stmt = conn.prepare(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('users', 'sessions', 'subscriptions')"
        )?;
        let required_tables: i32 = stmt.query_row([], |row| row.get(0))?;
        assert_eq!(required_tables, 3, "Not all required tables were created");

        Ok(())
    }
}
