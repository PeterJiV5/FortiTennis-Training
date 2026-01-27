#[cfg(test)]
mod tests {
    use chrono::{Utc, NaiveDate, NaiveTime};
    use tui_coach::models::{
        Session, ContentType, TrainingContent, Subscription, SubscriptionStatus, SkillLevel,
    };

    #[test]
    fn test_session_creation() {
        let now = Utc::now();
        let session = Session {
            id: 1,
            title: "Beginner Fundamentals".to_string(),
            description: Some("Learn the basics".to_string()),
            scheduled_date: Some(NaiveDate::from_ymd_opt(2025, 3, 15).unwrap()),
            scheduled_time: Some(NaiveTime::from_hms_opt(10, 30, 0).unwrap()),
            duration_minutes: Some(60),
            skill_level: Some(SkillLevel::Beginner),
            created_by: 1,
            created_at: now,
            updated_at: now,
        };

        assert_eq!(session.id, 1);
        assert_eq!(session.title, "Beginner Fundamentals");
        assert_eq!(session.created_by, 1);
        assert_eq!(session.duration_minutes, Some(60));
    }

    #[test]
    fn test_session_minimal() {
        let now = Utc::now();
        let session = Session {
            id: 1,
            title: "Quick Drill".to_string(),
            description: None,
            scheduled_date: None,
            scheduled_time: None,
            duration_minutes: None,
            skill_level: None,
            created_by: 1,
            created_at: now,
            updated_at: now,
        };

        assert_eq!(session.description, None);
        assert_eq!(session.scheduled_date, None);
        assert_eq!(session.duration_minutes, None);
    }

    #[test]
    fn test_content_type_from_str_drill() {
        assert_eq!(ContentType::from_str("drill"), Some(ContentType::Drill));
    }

    #[test]
    fn test_content_type_from_str_exercise() {
        assert_eq!(
            ContentType::from_str("exercise"),
            Some(ContentType::Exercise)
        );
    }

    #[test]
    fn test_content_type_from_str_warmup() {
        assert_eq!(ContentType::from_str("warmup"), Some(ContentType::Warmup));
    }

    #[test]
    fn test_content_type_from_str_cooldown() {
        assert_eq!(
            ContentType::from_str("cooldown"),
            Some(ContentType::Cooldown)
        );
    }

    #[test]
    fn test_content_type_from_str_case_insensitive() {
        assert_eq!(ContentType::from_str("DRILL"), Some(ContentType::Drill));
        assert_eq!(ContentType::from_str("Exercise"), Some(ContentType::Exercise));
        assert_eq!(ContentType::from_str("WARMUP"), Some(ContentType::Warmup));
    }

    #[test]
    fn test_content_type_from_str_invalid() {
        assert_eq!(ContentType::from_str("invalid"), None);
        assert_eq!(ContentType::from_str(""), None);
    }

    #[test]
    fn test_content_type_as_str() {
        assert_eq!(ContentType::Drill.as_str(), "drill");
        assert_eq!(ContentType::Exercise.as_str(), "exercise");
        assert_eq!(ContentType::Warmup.as_str(), "warmup");
        assert_eq!(ContentType::Cooldown.as_str(), "cooldown");
    }

    #[test]
    fn test_training_content_creation() {
        let content = TrainingContent {
            id: 1,
            session_id: 1,
            content_type: ContentType::Drill,
            title: "Serve Practice".to_string(),
            description: Some("Practice your serve".to_string()),
            duration_minutes: Some(15),
            order_index: 1,
        };

        assert_eq!(content.id, 1);
        assert_eq!(content.session_id, 1);
        assert_eq!(content.order_index, 1);
        assert_eq!(content.title, "Serve Practice");
    }

    #[test]
    fn test_subscription_status_from_str_active() {
        assert_eq!(
            SubscriptionStatus::from_str("active"),
            Some(SubscriptionStatus::Active)
        );
    }

    #[test]
    fn test_subscription_status_from_str_completed() {
        assert_eq!(
            SubscriptionStatus::from_str("completed"),
            Some(SubscriptionStatus::Completed)
        );
    }

    #[test]
    fn test_subscription_status_from_str_cancelled() {
        assert_eq!(
            SubscriptionStatus::from_str("cancelled"),
            Some(SubscriptionStatus::Cancelled)
        );
    }

    #[test]
    fn test_subscription_status_from_str_case_insensitive() {
        assert_eq!(
            SubscriptionStatus::from_str("ACTIVE"),
            Some(SubscriptionStatus::Active)
        );
        assert_eq!(
            SubscriptionStatus::from_str("Completed"),
            Some(SubscriptionStatus::Completed)
        );
        assert_eq!(
            SubscriptionStatus::from_str("CANCELLED"),
            Some(SubscriptionStatus::Cancelled)
        );
    }

    #[test]
    fn test_subscription_status_from_str_invalid() {
        assert_eq!(SubscriptionStatus::from_str("invalid"), None);
    }

    #[test]
    fn test_subscription_status_as_str() {
        assert_eq!(SubscriptionStatus::Active.as_str(), "active");
        assert_eq!(SubscriptionStatus::Completed.as_str(), "completed");
        assert_eq!(SubscriptionStatus::Cancelled.as_str(), "cancelled");
    }

    #[test]
    fn test_subscription_creation_active() {
        let now = Utc::now();
        let subscription = Subscription {
            id: 1,
            user_id: 1,
            session_id: 1,
            subscribed_at: now,
            completed_at: None,
            status: SubscriptionStatus::Active,
            notes: Some("Looking forward to this session".to_string()),
        };

        assert_eq!(subscription.id, 1);
        assert_eq!(subscription.status, SubscriptionStatus::Active);
        assert_eq!(subscription.completed_at, None);
    }

    #[test]
    fn test_subscription_creation_completed() {
        let now = Utc::now();
        let subscription = Subscription {
            id: 2,
            user_id: 1,
            session_id: 1,
            subscribed_at: now,
            completed_at: Some(now),
            status: SubscriptionStatus::Completed,
            notes: None,
        };

        assert_eq!(subscription.status, SubscriptionStatus::Completed);
        assert!(subscription.completed_at.is_some());
    }

    #[test]
    fn test_subscription_status_equality() {
        assert_eq!(SubscriptionStatus::Active, SubscriptionStatus::Active);
        assert_ne!(SubscriptionStatus::Active, SubscriptionStatus::Completed);
    }
}
