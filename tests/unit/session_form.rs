#[cfg(test)]
mod tests {
    use tui_coach::ui::session_form::{SessionForm, FormField};

    #[test]
    fn test_session_form_creation() {
        let form = SessionForm::new();
        assert_eq!(form.title, "");
        assert_eq!(form.description, "");
        assert_eq!(form.scheduled_date, "");
        assert_eq!(form.scheduled_time, "");
        assert_eq!(form.duration_minutes, "");
        assert_eq!(form.skill_level, "beginner");
        assert_eq!(form.focus_field, FormField::Title);
    }

    #[test]
    fn test_add_char_to_various_fields() {
        let mut form = SessionForm::new();
        
        // Add to title
        form.add_char('T');
        form.add_char('e');
        form.add_char('s');
        form.add_char('t');
        assert_eq!(form.title, "Test");
        
        // Switch to description and add
        form.next_field();
        assert_eq!(form.focus_field, FormField::Description);
        form.add_char('D');
        form.add_char('e');
        assert_eq!(form.description, "De");
    }

    #[test]
    fn test_field_navigation() {
        let mut form = SessionForm::new();
        assert_eq!(form.focus_field, FormField::Title);
        
        form.next_field();
        assert_eq!(form.focus_field, FormField::Description);
        
        form.next_field();
        assert_eq!(form.focus_field, FormField::ScheduledDate);
        
        form.prev_field();
        assert_eq!(form.focus_field, FormField::Description);
    }

    #[test]
    fn test_backspace_on_different_fields() {
        let mut form = SessionForm::new();
        form.title = "Hello".to_string();
        form.backspace();
        assert_eq!(form.title, "Hell");
        
        form.focus_field = FormField::DurationMinutes;
        form.duration_minutes = "90".to_string();
        form.backspace();
        assert_eq!(form.duration_minutes, "9");
    }

    #[test]
    fn test_validation_title_required() {
        let form = SessionForm::new();
        assert!(form.validate().is_err());
        
        let mut form = SessionForm::new();
        form.title = "T".to_string(); // Too short
        assert!(form.validate().is_err());
        
        let mut form = SessionForm::new();
        form.title = "Valid Title".to_string();
        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_validation_title_length() {
        let mut form = SessionForm::new();
        form.title = "a".repeat(101); // Too long
        assert!(form.validate().is_err());
        
        let mut form = SessionForm::new();
        form.title = "a".repeat(100); // Max valid length
        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_validation_description_length() {
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.description = "a".repeat(501);
        assert!(form.validate().is_err());
        
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.description = "a".repeat(500);
        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_validation_date_format() {
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.scheduled_date = "2026-02-15".to_string();
        assert!(form.validate().is_ok());
        
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.scheduled_date = "2026022015".to_string(); // Missing dashes
        assert!(form.validate().is_err());
        
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.scheduled_date = "2026-02".to_string(); // Not enough parts
        assert!(form.validate().is_err());
    }

    #[test]
    fn test_validation_time_format() {
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.scheduled_time = "14:30".to_string();
        assert!(form.validate().is_ok());
        
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.scheduled_time = "1430".to_string(); // Missing colon
        assert!(form.validate().is_err());
        
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.scheduled_time = "14:30:00".to_string(); // Too many parts
        assert!(form.validate().is_err());
    }

    #[test]
    fn test_validation_duration_range() {
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.duration_minutes = "4".to_string(); // Too short
        assert!(form.validate().is_err());
        
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.duration_minutes = "481".to_string(); // Too long
        assert!(form.validate().is_err());
        
        let mut form = SessionForm::new();
        form.title = "Title".to_string();
        form.duration_minutes = "90".to_string();
        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_skill_level_cycling() {
        let mut form = SessionForm::new();
        assert_eq!(form.skill_level, "beginner");
        
        form.cycle_skill_level_forward();
        assert_eq!(form.skill_level, "intermediate");
        
        form.cycle_skill_level_forward();
        assert_eq!(form.skill_level, "advanced");
        
        form.cycle_skill_level_forward();
        assert_eq!(form.skill_level, "beginner");
    }

    #[test]
    fn test_duration_only_accepts_digits() {
        let mut form = SessionForm::new();
        form.focus_field = FormField::DurationMinutes;
        
        form.add_char('1');
        form.add_char('2');
        form.add_char('0');
        form.add_char('a'); // Should not be added
        form.add_char('5'); // Should be added
        
        assert_eq!(form.duration_minutes, "1205");
    }

    #[test]
    fn test_as_db_values() {
        let mut form = SessionForm::new();
        form.title = "Test Session".to_string();
        form.description = "A test session".to_string();
        form.scheduled_date = "2026-02-15".to_string();
        form.scheduled_time = "14:30".to_string();
        form.duration_minutes = "90".to_string();
        form.skill_level = "intermediate".to_string();
        
        let (title, desc, date, time, duration, skill) = form.as_db_values();
        
        assert_eq!(title, "Test Session");
        assert_eq!(desc, "A test session");
        assert_eq!(date, Some("2026-02-15".to_string()));
        assert_eq!(time, Some("14:30".to_string()));
        assert_eq!(duration, Some(90));
        assert_eq!(skill, "intermediate");
    }

    #[test]
    fn test_as_db_values_with_optional_fields() {
        let form = SessionForm::new();
        let (title, desc, date, time, duration, skill) = form.as_db_values();
        
        assert_eq!(title, "");
        assert_eq!(desc, "");
        assert_eq!(date, None);
        assert_eq!(time, None);
        assert_eq!(duration, None);
        assert_eq!(skill, "beginner");
    }
}
