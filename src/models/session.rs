use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

use super::user::SkillLevel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_date: Option<NaiveDate>,
    pub scheduled_time: Option<NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub skill_level: Option<SkillLevel>,
    pub created_by: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Drill,
    Exercise,
    Warmup,
    Cooldown,
}

impl ContentType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "drill" => Some(ContentType::Drill),
            "exercise" => Some(ContentType::Exercise),
            "warmup" => Some(ContentType::Warmup),
            "cooldown" => Some(ContentType::Cooldown),
            _ => None,
        }
    }

	pub fn as_str(&self) -> &str {
		match self {
			ContentType::Drill => "drill",
			ContentType::Exercise => "exercise",
			ContentType::Warmup => "warmup",
			ContentType::Cooldown => "cooldown",
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingContent {
    pub id: i64,
    pub session_id: i64,
    pub content_type: ContentType,
    pub title: String,
    pub description: Option<String>,
    pub duration_minutes: Option<i32>,
    pub order_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Completed,
    Cancelled,
}

impl SubscriptionStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "active" => Some(SubscriptionStatus::Active),
            "completed" => Some(SubscriptionStatus::Completed),
            "cancelled" => Some(SubscriptionStatus::Cancelled),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Completed => "completed",
            SubscriptionStatus::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: i64,
    pub user_id: i64,
    pub session_id: i64,
    pub subscribed_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: SubscriptionStatus,
    pub notes: Option<String>,
}