#[cfg(test)]
mod subscription_tests {
    use tui_coach::db::{establish_connection, run_migrations};
    use tui_coach::db::repositories::{SessionRepository, SubscriptionRepository};

    fn setup_test_db_with_session() -> (rusqlite::Connection, i64, i64) {
        use std::path::PathBuf;
        use std::fs;
        
        // Create a persistent temp file instead of using tempdir
        let temp_db_path = PathBuf::from(format!("/tmp/test_sub_{}.db", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()));
        
        // Ensure clean state
        let _ = fs::remove_file(&temp_db_path);
        
        let conn = establish_connection(temp_db_path.to_str().unwrap()).unwrap();
        run_migrations(&conn).unwrap();
        
        // Insert test coach
        conn.execute(
            "INSERT INTO users (username, display_name, role, created_at, updated_at)
             VALUES (?, ?, ?, datetime('now'), datetime('now'))",
            ["coach", "Test Coach", "coach"],
        ).unwrap();
        let coach_id = conn.last_insert_rowid();
        
        // Insert test player
        conn.execute(
            "INSERT INTO users (username, display_name, role, skill_level, created_at, updated_at)
             VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))",
            ["alice", "Alice Smith", "player", "beginner"],
        ).unwrap();
        let player_id = conn.last_insert_rowid();
        
        (conn, coach_id, player_id)
    }

    #[test]
    fn test_create_subscription() {
        let (conn, coach_id, player_id) = setup_test_db_with_session();
        
        // Create a session
        let session_id = SessionRepository::create(
            &conn,
            "Test Session",
            None,
            None,
            None,
            None,
            None,
            coach_id,
        ).unwrap();
        
        // Create subscription
        let subscription_id = SubscriptionRepository::create(&conn, player_id, session_id).unwrap();
        
        assert!(subscription_id > 0);
    }

    #[test]
    fn test_is_subscribed() {
        let (conn, coach_id, player_id) = setup_test_db_with_session();
        
        let session_id = SessionRepository::create(
            &conn,
            "Test Session",
            None,
            None,
            None,
            None,
            None,
            coach_id,
        ).unwrap();
        
        // Not subscribed initially
        assert!(!SubscriptionRepository::is_subscribed(&conn, player_id, session_id).unwrap());
        
        // Subscribe
        SubscriptionRepository::create(&conn, player_id, session_id).unwrap();
        
        // Now subscribed
        assert!(SubscriptionRepository::is_subscribed(&conn, player_id, session_id).unwrap());
    }

    #[test]
    fn test_find_by_user_and_session() {
        let (conn, coach_id, player_id) = setup_test_db_with_session();
        
        let session_id = SessionRepository::create(
            &conn,
            "Test Session",
            None,
            None,
            None,
            None,
            None,
            coach_id,
        ).unwrap();
        
        // No subscription initially
        let result = SubscriptionRepository::find_by_user_and_session(&conn, player_id, session_id).unwrap();
        assert!(result.is_none());
        
        // Create subscription
        SubscriptionRepository::create(&conn, player_id, session_id).unwrap();
        
        // Now found
        let result = SubscriptionRepository::find_by_user_and_session(&conn, player_id, session_id).unwrap();
        assert!(result.is_some());
        
        let subscription = result.unwrap();
        assert_eq!(subscription.user_id, player_id);
        assert_eq!(subscription.session_id, session_id);
    }

    #[test]
    fn test_find_by_user() {
        let (conn, coach_id, player_id) = setup_test_db_with_session();
        
        // Create multiple sessions
        let session1 = SessionRepository::create(&conn, "Session 1", None, None, None, None, None, coach_id).unwrap();
        let session2 = SessionRepository::create(&conn, "Session 2", None, None, None, None, None, coach_id).unwrap();
        
        // Subscribe to both
        SubscriptionRepository::create(&conn, player_id, session1).unwrap();
        SubscriptionRepository::create(&conn, player_id, session2).unwrap();
        
        // Find all user subscriptions
        let subscriptions = SubscriptionRepository::find_by_user(&conn, player_id).unwrap();
        assert_eq!(subscriptions.len(), 2);
    }

    #[test]
    fn test_find_by_session() {
        let (conn, coach_id, player_id) = setup_test_db_with_session();
        
        // Insert second player
        conn.execute(
            "INSERT INTO users (username, display_name, role, skill_level, created_at, updated_at)
             VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))",
            ["bob", "Bob Jones", "player", "intermediate"],
        ).unwrap();
        let player2_id = conn.last_insert_rowid();
        
        let session_id = SessionRepository::create(&conn, "Test Session", None, None, None, None, None, coach_id).unwrap();
        
        // Both players subscribe
        SubscriptionRepository::create(&conn, player_id, session_id).unwrap();
        SubscriptionRepository::create(&conn, player2_id, session_id).unwrap();
        
        // Find all subscriptions for session
        let subscriptions = SubscriptionRepository::find_by_session(&conn, session_id).unwrap();
        assert_eq!(subscriptions.len(), 2);
    }

    #[test]
    fn test_mark_completed() {
        let (conn, coach_id, player_id) = setup_test_db_with_session();
        
        let session_id = SessionRepository::create(&conn, "Test Session", None, None, None, None, None, coach_id).unwrap();
        let subscription_id = SubscriptionRepository::create(&conn, player_id, session_id).unwrap();
        
        // Not completed initially
        let subscription = SubscriptionRepository::find_by_user_and_session(&conn, player_id, session_id).unwrap().unwrap();
        assert!(subscription.completed_at.is_none());
        
        // Mark as completed
        SubscriptionRepository::mark_completed(&conn, subscription_id).unwrap();
        
        // Now completed
        let subscription = SubscriptionRepository::find_by_user_and_session(&conn, player_id, session_id).unwrap().unwrap();
        assert!(subscription.completed_at.is_some());
    }

    #[test]
    fn test_delete_subscription() {
        let (conn, coach_id, player_id) = setup_test_db_with_session();
        
        let session_id = SessionRepository::create(&conn, "Test Session", None, None, None, None, None, coach_id).unwrap();
        let subscription_id = SubscriptionRepository::create(&conn, player_id, session_id).unwrap();
        
        // Exists
        assert!(SubscriptionRepository::is_subscribed(&conn, player_id, session_id).unwrap());
        
        // Delete
        SubscriptionRepository::delete(&conn, subscription_id).unwrap();
        
        // No longer exists
        assert!(!SubscriptionRepository::is_subscribed(&conn, player_id, session_id).unwrap());
    }

    #[test]
    fn test_delete_by_user_and_session() {
        let (conn, coach_id, player_id) = setup_test_db_with_session();
        
        let session_id = SessionRepository::create(&conn, "Test Session", None, None, None, None, None, coach_id).unwrap();
        SubscriptionRepository::create(&conn, player_id, session_id).unwrap();
        
        // Exists
        assert!(SubscriptionRepository::is_subscribed(&conn, player_id, session_id).unwrap());
        
        // Delete by user and session
        SubscriptionRepository::delete_by_user_and_session(&conn, player_id, session_id).unwrap();
        
        // No longer exists
        assert!(!SubscriptionRepository::is_subscribed(&conn, player_id, session_id).unwrap());
    }

    #[test]
    fn test_unique_constraint() {
        let (conn, coach_id, player_id) = setup_test_db_with_session();
        
        let session_id = SessionRepository::create(&conn, "Test Session", None, None, None, None, None, coach_id).unwrap();
        
        // First subscription succeeds
        SubscriptionRepository::create(&conn, player_id, session_id).unwrap();
        
        // Second subscription to same session should fail (unique constraint)
        let result = SubscriptionRepository::create(&conn, player_id, session_id);
        assert!(result.is_err());
    }
}