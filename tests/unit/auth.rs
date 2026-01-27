#[cfg(test)]
mod tests {
    use chrono::Utc;
    use tui_coach::auth::UserContext;
    use tui_coach::models::{User, UserRole, SkillLevel};

    #[test]
    fn test_user_context_creation() {
        let user = User {
            id: 1,
            username: "coach_john".to_string(),
            display_name: "John Coach".to_string(),
            role: UserRole::Coach,
            skill_level: Some(SkillLevel::Advanced),
            goals: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let context = UserContext::new(user.clone());
        assert_eq!(context.user.id, 1);
        assert_eq!(context.user.username, "coach_john");
    }

    #[test]
    fn test_user_context_username() {
        let user = User {
            id: 1,
            username: "test_username".to_string(),
            display_name: "Test User".to_string(),
            role: UserRole::Player,
            skill_level: None,
            goals: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let context = UserContext::new(user);
        assert_eq!(context.username(), "test_username");
    }

    #[test]
    fn test_user_context_is_coach() {
        let coach = User {
            id: 1,
            username: "coach".to_string(),
            display_name: "Coach".to_string(),
            role: UserRole::Coach,
            skill_level: Some(SkillLevel::Advanced),
            goals: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let context = UserContext::new(coach);
        assert!(context.is_coach());
        assert!(!context.is_player());
    }

    #[test]
    fn test_user_context_is_player() {
        let player = User {
            id: 2,
            username: "player".to_string(),
            display_name: "Player".to_string(),
            role: UserRole::Player,
            skill_level: Some(SkillLevel::Beginner),
            goals: Some("Improve".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let context = UserContext::new(player);
        assert!(!context.is_coach());
        assert!(context.is_player());
    }

    #[test]
    fn test_user_context_clone() {
        let user = User {
            id: 1,
            username: "clone_test".to_string(),
            display_name: "Clone Test".to_string(),
            role: UserRole::Coach,
            skill_level: Some(SkillLevel::Advanced),
            goals: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let context1 = UserContext::new(user);
        let context2 = context1.clone();

        assert_eq!(context1.username(), context2.username());
        assert_eq!(context1.is_coach(), context2.is_coach());
    }
}
