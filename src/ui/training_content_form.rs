use crate::models::ContentType;

#[derive(Debug, Clone)]
pub struct TrainingContentForm {
    pub title: String,
    pub description: String,
    pub duration_minutes: String,
    pub content_type: ContentType,
    pub focus_field: FormField,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormField {
    Title,
    Description,
    DurationMinutes,
    ContentType,
}

impl TrainingContentForm {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            duration_minutes: String::new(),
            content_type: ContentType::Drill,
            focus_field: FormField::Title,
        }
    }

    pub fn next_field(&mut self) {
        self.focus_field = match self.focus_field {
            FormField::Title => FormField::Description,
            FormField::Description => FormField::DurationMinutes,
            FormField::DurationMinutes => FormField::ContentType,
            FormField::ContentType => FormField::Title,
        };
    }

    pub fn prev_field(&mut self) {
        self.focus_field = match self.focus_field {
            FormField::Title => FormField::ContentType,
            FormField::Description => FormField::Title,
            FormField::DurationMinutes => FormField::Description,
            FormField::ContentType => FormField::DurationMinutes,
        };
    }

    pub fn add_char(&mut self, c: char) {
        match self.focus_field {
            FormField::Title => self.title.push(c),
            FormField::Description => self.description.push(c),
            FormField::DurationMinutes => {
                if c.is_numeric() {
                    self.duration_minutes.push(c);
                }
            }
            FormField::ContentType => {
                // Don't allow direct character input for content type
            }
        }
    }

    pub fn backspace(&mut self) {
        match self.focus_field {
            FormField::Title => {
                self.title.pop();
            }
            FormField::Description => {
                self.description.pop();
            }
            FormField::DurationMinutes => {
                self.duration_minutes.pop();
            }
            FormField::ContentType => {
                // Cycle backwards
                self.content_type = match self.content_type {
                    ContentType::Drill => ContentType::Cooldown,
                    ContentType::Exercise => ContentType::Drill,
                    ContentType::Warmup => ContentType::Exercise,
                    ContentType::Cooldown => ContentType::Warmup,
                };
            }
        }
    }

    pub fn cycle_content_type_forward(&mut self) {
        self.content_type = match self.content_type {
            ContentType::Drill => ContentType::Exercise,
            ContentType::Exercise => ContentType::Warmup,
            ContentType::Warmup => ContentType::Cooldown,
            ContentType::Cooldown => ContentType::Drill,
        };
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Title is required".to_string());
        }
        if self.title.len() < 2 {
            return Err("Title must be at least 2 characters".to_string());
        }
        if self.title.len() > 100 {
            return Err("Title must be less than 100 characters".to_string());
        }

        if self.description.len() > 500 {
            return Err("Description must be less than 500 characters".to_string());
        }

        if !self.duration_minutes.is_empty() {
            let duration: u32 = self
                .duration_minutes
                .parse()
                .map_err(|_| "Duration must be a number".to_string())?;
            if duration < 1 || duration > 480 {
                return Err("Duration must be between 1 and 480 minutes".to_string());
            }
        }

        Ok(())
    }

    pub fn as_db_values(&self) -> (String, String, Option<i32>, String) {
        (
            self.title.clone(),
            self.description.clone(),
            if self.duration_minutes.is_empty() {
                None
            } else {
                self.duration_minutes.parse().ok()
            },
            self.content_type.as_str().to_string(),
        )
    }

    pub fn clear(&mut self) {
        self.title.clear();
        self.description.clear();
        self.duration_minutes.clear();
        self.content_type = ContentType::Drill;
        self.focus_field = FormField::Title;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_creation() {
        let form = TrainingContentForm::new();
        assert_eq!(form.title, "");
        assert_eq!(form.description, "");
        assert_eq!(form.duration_minutes, "");
        assert_eq!(form.content_type, ContentType::Drill);
    }

    #[test]
    fn test_add_char() {
        let mut form = TrainingContentForm::new();
        form.add_char('F');
        form.add_char('o');
        form.add_char('r');
        assert_eq!(form.title, "For");
    }

    #[test]
    fn test_field_navigation() {
        let mut form = TrainingContentForm::new();
        assert_eq!(form.focus_field, FormField::Title);
        form.next_field();
        assert_eq!(form.focus_field, FormField::Description);
        form.next_field();
        assert_eq!(form.focus_field, FormField::DurationMinutes);
        form.prev_field();
        assert_eq!(form.focus_field, FormField::Description);
    }

    #[test]
    fn test_validate_title_required() {
        let form = TrainingContentForm::new();
        let result = form.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_lowercase()
            .contains("required"));
    }

    #[test]
    fn test_validate_duration_range() {
        let mut form = TrainingContentForm::new();
        form.title = "Test".to_string();
        form.duration_minutes = "500".to_string();
        let result = form.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_cycle_content_type() {
        let mut form = TrainingContentForm::new();
        assert_eq!(form.content_type, ContentType::Drill);
        form.cycle_content_type_forward();
        assert_eq!(form.content_type, ContentType::Exercise);
        form.cycle_content_type_forward();
        assert_eq!(form.content_type, ContentType::Warmup);
    }

    #[test]
    fn test_as_db_values() {
        let mut form = TrainingContentForm::new();
        form.title = "Forehand Drill".to_string();
        form.description = "Practice forehand".to_string();
        form.duration_minutes = "30".to_string();

        let (title, desc, duration, content_type) = form.as_db_values();
        assert_eq!(title, "Forehand Drill");
        assert_eq!(desc, "Practice forehand");
        assert_eq!(duration, Some(30));
        assert_eq!(content_type, "drill");
    }
}
