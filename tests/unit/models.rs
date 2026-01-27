#[cfg(test)]
mod tests {
    use chrono::Utc;
    use tui_coach::models::{User, UserRole, SkillLevel};

    #[test]
    fn test_user_role_from_str_coach() {
        let role = UserRole::from_str("coach");
        assert_eq!(role, Some(UserRole::Coach));
    }

    #[test]
    fn test_user_role_from_str_player() {
        let role = UserRole::from_str("player");
        assert_eq!(role, Some(UserRole::Player));
    }

    #[test]
    fn test_user_role_from_str_case_insensitive() {
        assert_eq!(UserRole::from_str("COACH"), Some(UserRole::Coach));
        assert_eq!(UserRole::from_str("Coach"), Some(UserRole::Coach));
        assert_eq!(UserRole::from_str("PLAYER"), Some(UserRole::Player));
        assert_eq!(UserRole::from_str("Player"), Some(UserRole::Player));
    }

    #[test]
    fn test_user_role_from_str_invalid() {
        let role = UserRole::from_str("invalid");
        assert_eq!(role, None);
    }

    #[test]
    fn test_user_role_as_str() {
        assert_eq!(UserRole::Coach.as_str(), "coach");
        assert_eq!(UserRole::Player.as_str(), "player");
    }

    #[test]
    fn test_skill_level_from_str_beginner() {
        assert_eq!(SkillLevel::from_str("beginner"), Some(SkillLevel::Beginner));
    }

    #[test]
    fn test_skill_level_from_str_intermediate() {
        assert_eq!(
            SkillLevel::from_str("intermediate"),
            Some(SkillLevel::Intermediate)
        );
    }

    #[test]
    fn test_skill_level_from_str_advanced() {
        assert_eq!(SkillLevel::from_str("advanced"), Some(SkillLevel::Advanced));
    }

    #[test]
    fn test_skill_level_from_str_case_insensitive() {
        assert_eq!(SkillLevel::from_str("BEGINNER"), Some(SkillLevel::Beginner));
        assert_eq!(
            SkillLevel::from_str("INTERMEDIATE"),
            Some(SkillLevel::Intermediate)
        );
        assert_eq!(SkillLevel::from_str("ADVANCED"), Some(SkillLevel::Advanced));
    }

    #[test]
    fn test_skill_level_from_str_invalid() {
        assert_eq!(SkillLevel::from_str("invalid"), None);
    }

    #[test]
    fn test_skill_level_as_str() {
        assert_eq!(SkillLevel::Beginner.as_str(), "beginner");
        assert_eq!(SkillLevel::Intermediate.as_str(), "intermediate");
        assert_eq!(SkillLevel::Advanced.as_str(), "advanced");
    }

    #[test]
    fn test_user_is_coach() {
        let user = User {
            id: 1,
            username: "john_coach".to_string(),
            display_name: "John Coach".to_string(),
            role: UserRole::Coach,
            skill_level: Some(SkillLevel::Advanced),
            goals: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(user.is_coach());
        assert!(!user.is_player());
    }

    #[test]
    fn test_user_is_player() {
        let user = User {
            id: 2,
            username: "jane_player".to_string(),
            display_name: "Jane Player".to_string(),
            role: UserRole::Player,
            skill_level: Some(SkillLevel::Intermediate),
            goals: Some("Improve serve".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(!user.is_coach());
        assert!(user.is_player());
    }

    #[test]
    fn test_user_creation() {
        let now = Utc::now();
        let user = User {
            id: 1,
            username: "test_user".to_string(),
            display_name: "Test User".to_string(),
            role: UserRole::Player,
            skill_level: Some(SkillLevel::Beginner),
            goals: Some("Learn tennis".to_string()),
            created_at: now,
            updated_at: now,
        };

        assert_eq!(user.id, 1);
        assert_eq!(user.username, "test_user");
        assert_eq!(user.display_name, "Test User");
        assert_eq!(user.role, UserRole::Player);
        assert_eq!(user.skill_level, Some(SkillLevel::Beginner));
        assert_eq!(user.goals, Some("Learn tennis".to_string()));
    }

    #[test]
    fn test_user_with_no_skill_level() {
        let user = User {
            id: 1,
            username: "test_user".to_string(),
            display_name: "Test User".to_string(),
            role: UserRole::Player,
            skill_level: None,
            goals: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(user.skill_level, None);
        assert_eq!(user.goals, None);
    }
}
