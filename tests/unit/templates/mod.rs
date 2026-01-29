#[cfg(test)]
mod template_creation {
    use tui_coach::db::connection::establish_connection;
    use tui_coach::db::repositories::TrainingTemplateRepository;
    use tui_coach::models::TrainingTemplate;
    use tempfile::TempDir;

    fn setup() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let db_str = db_path.to_str().unwrap().to_string();

        // Initialize database
        let conn = establish_connection(&db_str).unwrap();
        conn.execute_batch(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                display_name TEXT NOT NULL,
                role TEXT NOT NULL,
                skill_level TEXT,
                goals TEXT,
                created_at DATETIME,
                updated_at DATETIME
            );
            INSERT INTO users VALUES (1, 'coach1', 'Coach Alice', 'coach', NULL, NULL, '2026-01-01', '2026-01-01');"
        ).unwrap();

        (temp_dir, db_str)
    }

    #[test]
    fn test_create_template() {
        let (_temp_dir, db_path) = setup();
        let conn = establish_connection(&db_path).unwrap();

        // Create migration table
        conn.execute_batch(
            "CREATE TABLE training_templates (
                id INTEGER PRIMARY KEY,
                coach_id INTEGER,
                title TEXT,
                content_type TEXT,
                description TEXT,
                duration_minutes INTEGER,
                created_at DATETIME,
                created_by INTEGER,
                last_edited_by INTEGER,
                last_edited_at DATETIME,
                is_public BOOLEAN
            );"
        ).unwrap();

        let template = TrainingTemplate {
            id: 0,
            coach_id: 1,
            title: "Backhand Drill".to_string(),
            content_type: "drill".to_string(),
            description: Some("Basic backhand technique".to_string()),
            duration_minutes: Some(20),
            created_at: "2026-01-29".to_string(),
            created_by: 1,
            last_edited_by: None,
            last_edited_at: None,
            is_public: true,
        };

        let id = TrainingTemplateRepository::create(&conn, &template, 1).unwrap();
        assert!(id > 0);

        let retrieved = TrainingTemplateRepository::get_by_id(&conn, id).unwrap();
        assert_eq!(retrieved.title, "Backhand Drill");
        assert_eq!(retrieved.created_by, 1);
        assert_eq!(retrieved.coach_id, 1);
    }

    #[test]
    fn test_template_audit_tracking() {
        let (_temp_dir, db_path) = setup();
        let conn = establish_connection(&db_path).unwrap();

        conn.execute_batch(
            "CREATE TABLE training_templates (
                id INTEGER PRIMARY KEY,
                coach_id INTEGER,
                title TEXT,
                content_type TEXT,
                description TEXT,
                duration_minutes INTEGER,
                created_at DATETIME,
                created_by INTEGER,
                last_edited_by INTEGER,
                last_edited_at DATETIME,
                is_public BOOLEAN
            );
            INSERT INTO users VALUES (2, 'coach2', 'Coach Bob', 'coach', NULL, NULL, '2026-01-01', '2026-01-01');"
        ).unwrap();

        let template = TrainingTemplate {
            id: 0,
            coach_id: 1,
            title: "Test Template".to_string(),
            content_type: "exercise".to_string(),
            description: None,
            duration_minutes: None,
            created_at: "2026-01-29".to_string(),
            created_by: 1,
            last_edited_by: None,
            last_edited_at: None,
            is_public: true,
        };

        let id = TrainingTemplateRepository::create(&conn, &template, 1).unwrap();
        let mut retrieved = TrainingTemplateRepository::get_by_id(&conn, id).unwrap();

        // Verify initial state
        assert_eq!(retrieved.created_by, 1);
        assert!(retrieved.last_edited_by.is_none());

        // Update template
        retrieved.title = "Updated Template".to_string();
        TrainingTemplateRepository::update(&conn, &retrieved, 2).unwrap();

        // Verify update
        let updated = TrainingTemplateRepository::get_by_id(&conn, id).unwrap();
        assert_eq!(updated.title, "Updated Template");
        assert_eq!(updated.created_by, 1); // Still original creator
        assert_eq!(updated.last_edited_by, Some(2)); // New editor
    }
}
