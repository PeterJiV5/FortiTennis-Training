use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Coach,
    Player,
}

impl UserRole {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "coach" => Some(UserRole::Coach),
            "player" => Some(UserRole::Player),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Coach => "coach",
            UserRole::Player => "player",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
}

impl SkillLevel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "beginner" => Some(SkillLevel::Beginner),
            "intermediate" => Some(SkillLevel::Intermediate),
            "advanced" => Some(SkillLevel::Advanced),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SkillLevel::Beginner => "beginner",
            SkillLevel::Intermediate => "intermediate",
            SkillLevel::Advanced => "advanced",
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub role: UserRole,
    pub skill_level: Option<SkillLevel>,
    pub goals: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn is_coach(&self) -> bool {
        self.role == UserRole::Coach
    }

    pub fn is_player(&self) -> bool {
        self.role == UserRole::Player
    }
}
