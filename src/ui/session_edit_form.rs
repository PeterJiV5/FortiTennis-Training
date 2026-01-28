use crate::models::Session;

/// Session edit form state - pre-populated with existing session data
#[derive(Debug, Clone)]
pub struct SessionEditForm {
    pub title: String,
    pub description: String,
    pub scheduled_date: String,
    pub scheduled_time: String,
    pub duration_minutes: String,
    pub skill_level: String,
    pub focus_field: FormField,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormField {
    Title,
    Description,
    ScheduledDate,
    ScheduledTime,
    DurationMinutes,
    SkillLevel,
}

impl SessionEditForm {
    /// Create a new edit form pre-populated with session data
    pub fn from_session(session: &Session) -> Self {
        Self {
            title: session.title.clone(),
            description: session.description.clone().unwrap_or_default(),
            scheduled_date: session
                .scheduled_date
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
            scheduled_time: session
                .scheduled_time
                .map(|t| t.format("%H:%M").to_string())
                .unwrap_or_default(),
            duration_minutes: session
                .duration_minutes
                .map(|d| d.to_string())
                .unwrap_or_default(),
            skill_level: session
                .skill_level
                .as_ref()
                .map(|s| s.as_str().to_string())
                .unwrap_or_else(|| "beginner".to_string()),
            focus_field: FormField::Title,
        }
    }

    /// Move focus to next field
    pub fn next_field(&mut self) {
        self.focus_field = match self.focus_field {
            FormField::Title => FormField::Description,
            FormField::Description => FormField::ScheduledDate,
            FormField::ScheduledDate => FormField::ScheduledTime,
            FormField::ScheduledTime => FormField::DurationMinutes,
            FormField::DurationMinutes => FormField::SkillLevel,
            FormField::SkillLevel => FormField::Title,
        };
    }

    /// Move focus to previous field
    pub fn prev_field(&mut self) {
        self.focus_field = match self.focus_field {
            FormField::Title => FormField::SkillLevel,
            FormField::Description => FormField::Title,
            FormField::ScheduledDate => FormField::Description,
            FormField::ScheduledTime => FormField::ScheduledDate,
            FormField::DurationMinutes => FormField::ScheduledTime,
            FormField::SkillLevel => FormField::DurationMinutes,
        };
    }

    /// Add character to current field
    pub fn add_char(&mut self, c: char) {
        match self.focus_field {
            FormField::Title => self.title.push(c),
            FormField::Description => self.description.push(c),
            FormField::ScheduledDate => self.scheduled_date.push(c),
            FormField::ScheduledTime => self.scheduled_time.push(c),
            FormField::DurationMinutes => {
                // Only allow digits
                if c.is_numeric() {
                    self.duration_minutes.push(c);
                }
            }
            FormField::SkillLevel => {
                // Don't allow direct character input for skill level
            }
        }
    }

    /// Remove last character from current field
    pub fn backspace(&mut self) {
        match self.focus_field {
            FormField::Title => {
                self.title.pop();
            }
            FormField::Description => {
                self.description.pop();
            }
            FormField::ScheduledDate => {
                self.scheduled_date.pop();
            }
            FormField::ScheduledTime => {
                self.scheduled_time.pop();
            }
            FormField::DurationMinutes => {
                self.duration_minutes.pop();
            }
            FormField::SkillLevel => {
                // Handle skill level cycling
                self.skill_level = match self.skill_level.as_str() {
                    "beginner" => "advanced".to_string(),
                    "intermediate" => "beginner".to_string(),
                    "advanced" => "intermediate".to_string(),
                    _ => "beginner".to_string(),
                };
            }
        }
    }

    /// Cycle skill level forward
    pub fn cycle_skill_level_forward(&mut self) {
        self.skill_level = match self.skill_level.as_str() {
            "beginner" => "intermediate".to_string(),
            "intermediate" => "advanced".to_string(),
            "advanced" => "beginner".to_string(),
            _ => "beginner".to_string(),
        };
    }

    /// Validate form fields
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("Title is required".to_string());
        }
        if self.title.len() < 3 {
            return Err("Title must be at least 3 characters".to_string());
        }
        if self.title.len() > 100 {
            return Err("Title must be less than 100 characters".to_string());
        }

        if self.description.len() > 500 {
            return Err("Description must be less than 500 characters".to_string());
        }

        if !self.scheduled_date.is_empty() {
            // Validate date format YYYY-MM-DD
            if !self.scheduled_date.contains('-') {
                return Err("Date format should be YYYY-MM-DD".to_string());
            }
            let parts: Vec<&str> = self.scheduled_date.split('-').collect();
            if parts.len() != 3 {
                return Err("Date format should be YYYY-MM-DD".to_string());
            }
        }

        if !self.scheduled_time.is_empty() {
            // Validate time format HH:MM
            if !self.scheduled_time.contains(':') {
                return Err("Time format should be HH:MM".to_string());
            }
            let parts: Vec<&str> = self.scheduled_time.split(':').collect();
            if parts.len() != 2 {
                return Err("Time format should be HH:MM".to_string());
            }
        }

        if !self.duration_minutes.is_empty() {
            let duration: u32 = self.duration_minutes.parse()
                .map_err(|_| "Duration must be a number".to_string())?;
            if duration < 5 || duration > 480 {
                return Err("Duration must be between 5 and 480 minutes".to_string());
            }
        }

        Ok(())
    }

    /// Get all fields as a tuple for database update
    pub fn as_db_values(&self) -> (String, String, Option<String>, Option<String>, Option<i32>, String) {
        (
            self.title.clone(),
            self.description.clone(),
            if self.scheduled_date.is_empty() { None } else { Some(self.scheduled_date.clone()) },
            if self.scheduled_time.is_empty() { None } else { Some(self.scheduled_time.clone()) },
            if self.duration_minutes.is_empty() { None } else { self.duration_minutes.parse().ok() },
            self.skill_level.clone(),
        )
    }
}
