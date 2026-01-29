use crate::models::{ContentType, TrainingContent};
use crate::utils::Result;
use rusqlite::Connection;
use rusqlite::OptionalExtension;

pub struct TrainingContentRepository;

impl TrainingContentRepository {
    /// Get all training content for a session
    pub fn find_by_session(conn: &Connection, session_id: i64) -> Result<Vec<TrainingContent>> {
        let mut stmt = conn.prepare(
            "SELECT id, session_id, content_type, title, description, duration_minutes, order_index
             FROM training_content
             WHERE session_id = ?
             ORDER BY order_index ASC",
        )?;

        let content = stmt.query_map([session_id], |row| Self::map_row(row))?;
        let content: Vec<TrainingContent> = content
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| crate::utils::AppError::Database(e))?;

        Ok(content)
    }

    /// Get a single training content by ID
    pub fn find_by_id(conn: &Connection, id: i64) -> Result<Option<TrainingContent>> {
        let mut stmt = conn.prepare(
            "SELECT id, session_id, content_type, title, description, duration_minutes, order_index
             FROM training_content
             WHERE id = ?",
        )?;

        let result = stmt
            .query_row([id], |row| Self::map_row(row))
            .optional()
            .map_err(|e| crate::utils::AppError::Database(e))?;

        Ok(result)
    }

    /// Create new training content
    pub fn create(
        conn: &Connection,
        session_id: i64,
        content_type: &ContentType,
        title: &str,
        description: Option<&str>,
        duration_minutes: Option<i32>,
        order_index: i32,
    ) -> Result<i64> {
        conn.execute(
            "INSERT INTO training_content (session_id, content_type, title, description, duration_minutes, order_index)
             VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                session_id,
                content_type.as_str(),
                title,
                description,
                duration_minutes,
                order_index,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update training content
    pub fn update(
        conn: &Connection,
        id: i64,
        content_type: &ContentType,
        title: &str,
        description: Option<&str>,
        duration_minutes: Option<i32>,
        order_index: i32,
    ) -> Result<()> {
        conn.execute(
            "UPDATE training_content
             SET content_type = ?, title = ?, description = ?, duration_minutes = ?, order_index = ?
             WHERE id = ?",
            rusqlite::params![
                content_type.as_str(),
                title,
                description,
                duration_minutes,
                order_index,
                id,
            ],
        )?;

        Ok(())
    }

    /// Delete training content by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM training_content WHERE id = ?", [id])?;
        Ok(())
    }

    /// Delete all content for a session
    pub fn delete_by_session(conn: &Connection, session_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM training_content WHERE session_id = ?",
            [session_id],
        )?;
        Ok(())
    }

    /// Map a database row to TrainingContent struct
    fn map_row(row: &rusqlite::Row) -> rusqlite::Result<TrainingContent> {
        let content_type_str: String = row.get(2)?;
        let content_type = ContentType::from_str(&content_type_str).unwrap_or(ContentType::Drill);

        Ok(TrainingContent {
            id: row.get(0)?,
            session_id: row.get(1)?,
            content_type,
            title: row.get(3)?,
            description: row.get(4)?,
            duration_minutes: row.get(5)?,
            order_index: row.get(6)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connection::establish_connection;

    #[test]
    fn test_create_training_content() {
        let conn = establish_connection(":memory:").expect("Failed to create in-memory DB");
        crate::db::migrations::run_migrations(&conn).expect("Failed to run migrations");

        // Create a test session first
        let result = conn.execute(
            "INSERT INTO users (username, display_name, role) VALUES ('coach', 'Coach', 'coach')",
            [],
        );
        assert!(result.is_ok());

        let result = conn.execute(
            "INSERT INTO sessions (title, created_by, created_at, updated_at) 
             VALUES ('Test Session', 1, datetime('now'), datetime('now'))",
            [],
        );
        assert!(result.is_ok());

        // Create training content
        let id = TrainingContentRepository::create(
            &conn,
            1,
            &ContentType::Drill,
            "Forehand Drill",
            Some("Practice forehand strokes"),
            Some(30),
            0,
        )
        .expect("Failed to create training content");

        assert!(id > 0);

        // Verify it was created
        let content = TrainingContentRepository::find_by_id(&conn, id)
            .expect("Failed to find content")
            .expect("Content not found");

        assert_eq!(content.title, "Forehand Drill");
        assert_eq!(content.content_type, ContentType::Drill);
        assert_eq!(content.duration_minutes, Some(30));
    }

    #[test]
    fn test_find_by_session() {
        let conn = establish_connection(":memory:").expect("Failed to create in-memory DB");
        crate::db::migrations::run_migrations(&conn).expect("Failed to run migrations");

        // Setup
        conn.execute(
            "INSERT INTO users (username, display_name, role) VALUES ('coach', 'Coach', 'coach')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO sessions (title, created_by, created_at, updated_at) 
             VALUES ('Test Session', 1, datetime('now'), datetime('now'))",
            [],
        )
        .unwrap();

        // Create multiple content items
        TrainingContentRepository::create(
            &conn,
            1,
            &ContentType::Warmup,
            "Warm-up",
            None,
            Some(10),
            0,
        )
        .unwrap();

        TrainingContentRepository::create(
            &conn,
            1,
            &ContentType::Drill,
            "Main Drill",
            None,
            Some(40),
            1,
        )
        .unwrap();

        TrainingContentRepository::create(
            &conn,
            1,
            &ContentType::Cooldown,
            "Cool-down",
            None,
            Some(10),
            2,
        )
        .unwrap();

        // Retrieve all
        let items = TrainingContentRepository::find_by_session(&conn, 1)
            .expect("Failed to find items");

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].order_index, 0);
        assert_eq!(items[1].order_index, 1);
        assert_eq!(items[2].order_index, 2);
    }

    #[test]
    fn test_update_training_content() {
        let conn = establish_connection(":memory:").expect("Failed to create in-memory DB");
        crate::db::migrations::run_migrations(&conn).expect("Failed to run migrations");

        // Setup
        conn.execute(
            "INSERT INTO users (username, display_name, role) VALUES ('coach', 'Coach', 'coach')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO sessions (title, created_by, created_at, updated_at) 
             VALUES ('Test Session', 1, datetime('now'), datetime('now'))",
            [],
        )
        .unwrap();

        let id = TrainingContentRepository::create(
            &conn,
            1,
            &ContentType::Drill,
            "Original Title",
            None,
            Some(30),
            0,
        )
        .unwrap();

        // Update
        TrainingContentRepository::update(
            &conn,
            id,
            &ContentType::Exercise,
            "Updated Title",
            Some("Updated description"),
            Some(45),
            0,
        )
        .unwrap();

        // Verify
        let content = TrainingContentRepository::find_by_id(&conn, id)
            .unwrap()
            .unwrap();

        assert_eq!(content.title, "Updated Title");
        assert_eq!(content.content_type, ContentType::Exercise);
        assert_eq!(content.duration_minutes, Some(45));
        assert_eq!(content.description, Some("Updated description".to_string()));
    }

    #[test]
    fn test_delete_training_content() {
        let conn = establish_connection(":memory:").expect("Failed to create in-memory DB");
        crate::db::migrations::run_migrations(&conn).expect("Failed to run migrations");

        // Setup
        conn.execute(
            "INSERT INTO users (username, display_name, role) VALUES ('coach', 'Coach', 'coach')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO sessions (title, created_by, created_at, updated_at) 
             VALUES ('Test Session', 1, datetime('now'), datetime('now'))",
            [],
        )
        .unwrap();

        let id = TrainingContentRepository::create(
            &conn,
            1,
            &ContentType::Drill,
            "Test",
            None,
            None,
            0,
        )
        .unwrap();

        // Delete
        TrainingContentRepository::delete(&conn, id).unwrap();

        // Verify
        let result = TrainingContentRepository::find_by_id(&conn, id)
            .unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_delete_by_session() {
        let conn = establish_connection(":memory:").expect("Failed to create in-memory DB");
        crate::db::migrations::run_migrations(&conn).expect("Failed to run migrations");

        // Setup
        conn.execute(
            "INSERT INTO users (username, display_name, role) VALUES ('coach', 'Coach', 'coach')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO sessions (title, created_by, created_at, updated_at) 
             VALUES ('Test Session', 1, datetime('now'), datetime('now'))",
            [],
        )
        .unwrap();

        // Create multiple items
        TrainingContentRepository::create(&conn, 1, &ContentType::Drill, "Item 1", None, None, 0).unwrap();
        TrainingContentRepository::create(&conn, 1, &ContentType::Drill, "Item 2", None, None, 1).unwrap();

        // Delete all for session
        TrainingContentRepository::delete_by_session(&conn, 1).unwrap();

        // Verify
        let items = TrainingContentRepository::find_by_session(&conn, 1).unwrap();
        assert_eq!(items.len(), 0);
    }
}
