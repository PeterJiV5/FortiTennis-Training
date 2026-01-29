use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingTemplate {
    pub id: i64,
    pub coach_id: i64,
    pub title: String,
    pub content_type: String, // 'drill', 'exercise', 'warmup', 'cooldown', 'quiz', 'homework'
    pub description: Option<String>,
    pub duration_minutes: Option<i32>,
    pub created_at: String,
    pub created_by: i64,
    pub last_edited_by: Option<i64>,
    pub last_edited_at: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Clone)]
pub struct TemplateAuditInfo {
    pub created_by_name: String,
    pub last_edited_by_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTrainingLink {
    pub id: i64,
    pub session_id: i64,
    pub training_template_id: i64,
    pub order_index: i32,
    pub custom_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTrainingLinkWithTemplate {
    pub link: SessionTrainingLink,
    pub template: Option<TrainingTemplate>,
}
