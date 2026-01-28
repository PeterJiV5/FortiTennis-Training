#[cfg(test)]
mod tests {
    use tui_coach::ui::session_edit_form::{SessionEditForm, FormField};
    use tui_coach::models::{Session, SkillLevel};
    use chrono::{NaiveDate, NaiveTime, Utc};

    fn create_test_session() -> Session {
        Session {
            id: 1,
            title: "Advanced Tennis Drill".to_string(),
            description: Some("Focused on serve techniques".to_string()),
            scheduled_date: NaiveDate::parse_from_str("2026-02-01", "%Y-%m-%d").ok(),
            scheduled_time: NaiveTime::parse_from_str("10:30", "%H:%M").ok(),
            duration_minutes: Some(60),
            skill_level: Some(SkillLevel::Advanced),
            created_by: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_session_edit_form_from_session() {
        let session = create_test_session();
        let form = SessionEditForm::from_session(&session);

        assert_eq!(form.title, "Advanced Tennis Drill");
        assert_eq!(form.description, "Focused on serve techniques");
        assert_eq!(form.scheduled_date, "2026-02-01");
        assert_eq!(form.scheduled_time, "10:30");
        assert_eq!(form.duration_minutes, "60");
        assert_eq!(form.skill_level, "advanced");
        assert_eq!(form.focus_field, FormField::Title);
    }

    #[test]
    fn test_session_edit_form_from_session_with_optional_fields() {
        let mut session = create_test_session();
        session.description = None;
        session.scheduled_date = None;
        session.scheduled_time = None;
        session.duration_minutes = None;
        session.skill_level = None;

        let form = SessionEditForm::from_session(&session);

        assert_eq!(form.title, "Advanced Tennis Drill");
        assert_eq!(form.description, "");
        assert_eq!(form.scheduled_date, "");
        assert_eq!(form.scheduled_time, "");
        assert_eq!(form.duration_minutes, "");
        assert_eq!(form.skill_level, "beginner");
    }

    #[test]
    fn test_edit_form_field_navigation() {
        let session = create_test_session();
        let mut form = SessionEditForm::from_session(&session);

        assert_eq!(form.focus_field, FormField::Title);

        form.next_field();
        assert_eq!(form.focus_field, FormField::Description);

        form.next_field();
        assert_eq!(form.focus_field, FormField::ScheduledDate);

        form.prev_field();
        assert_eq!(form.focus_field, FormField::Description);

        form.prev_field();
        assert_eq!(form.focus_field, FormField::Title);
    }

    #[test]
    fn test_edit_form_add_char() {
        let session = create_test_session();
        let mut form = SessionEditForm::from_session(&session);

        form.add_char('!');
        assert_eq!(form.title, "Advanced Tennis Drill!");

        form.next_field(); // Move to Description
        form.add_char(' ');
        form.add_char('N');
        assert_eq!(form.description, "Focused on serve techniques N");
    }

    #[test]
    fn test_edit_form_backspace() {
        let session = create_test_session();
        let mut form = SessionEditForm::from_session(&session);

        form.backspace();
        assert_eq!(form.title, "Advanced Tennis Dril");

        form.backspace();
        assert_eq!(form.title, "Advanced Tennis Dri");
    }

    #[test]
    fn test_edit_form_skill_level_cycling() {
        let session = create_test_session();
        let mut form = SessionEditForm::from_session(&session);

        assert_eq!(form.skill_level, "advanced");

        form.next_field(); // Description
        form.next_field(); // ScheduledDate
        form.next_field(); // ScheduledTime
        form.next_field(); // DurationMinutes
        form.next_field(); // SkillLevel

        form.cycle_skill_level_forward();
        assert_eq!(form.skill_level, "beginner");

        form.cycle_skill_level_forward();
        assert_eq!(form.skill_level, "intermediate");

        form.cycle_skill_level_forward();
        assert_eq!(form.skill_level, "advanced");
    }

    #[test]
    fn test_edit_form_validate_title_required() {
        let session = create_test_session();
        let mut form = SessionEditForm::from_session(&session);

        form.title = String::new();
        let result = form.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Title is required"));
    }

    #[test]
    fn test_edit_form_validate_duration_range() {
        let session = create_test_session();
        let mut form = SessionEditForm::from_session(&session);

        form.duration_minutes = "3".to_string();
        let result = form.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("5 and 480"));

        form.duration_minutes = "500".to_string();
        let result = form.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("5 and 480"));
    }

    #[test]
    fn test_edit_form_as_db_values() {
        let session = create_test_session();
        let form = SessionEditForm::from_session(&session);

        let (title, description, date, time, duration, skill_level) = form.as_db_values();
        assert_eq!(title, "Advanced Tennis Drill");
        assert_eq!(description, "Focused on serve techniques");
        assert_eq!(date, Some("2026-02-01".to_string()));
        assert_eq!(time, Some("10:30".to_string()));
        assert_eq!(duration, Some(60));
        assert_eq!(skill_level, "advanced");
    }

    #[test]
    fn test_edit_form_as_db_values_with_empty_optional() {
        let session = create_test_session();
        let mut form = SessionEditForm::from_session(&session);

        form.description = String::new();
        form.scheduled_date = String::new();
        form.scheduled_time = String::new();
        form.duration_minutes = String::new();

        let (title, description, date, time, duration, skill_level) = form.as_db_values();
        assert_eq!(title, "Advanced Tennis Drill");
        assert_eq!(description, "");
        assert_eq!(date, None);
        assert_eq!(time, None);
        assert_eq!(duration, None);
        assert_eq!(skill_level, "advanced");
    }
}
