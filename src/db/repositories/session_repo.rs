use crate::models::{Session, SkillLevel};
use crate::utils::Result;
use chrono::{NaiveDate, NaiveTime};
use rusqlite::Connection;

pub struct SessionRepository;

impl SessionRepository {
    /// Get all sessions
    pub fn find_all(conn: &Connection) -> Result<Vec<Session>> {
        let mut stmt = conn.prepare(
            "SELECT id, title, description, scheduled_date, scheduled_time,
                    duration_minutes, skill_level, created_by, created_at, updated_at
             FROM sessions
             ORDER BY scheduled_date DESC, created_at DESC",
        )?;

        let sessions = stmt
            .query_map([], |row| Self::map_row(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    /// Get sessions by coach (creator)
    pub fn find_by_coach(conn: &Connection, coach_id: i64) -> Result<Vec<Session>> {
        let mut stmt = conn.prepare(
            "SELECT id, title, description, scheduled_date, scheduled_time,
                    duration_minutes, skill_level, created_by, created_at, updated_at
             FROM sessions
             WHERE created_by = ?
             ORDER BY scheduled_date DESC, created_at DESC",
        )?;

        let sessions = stmt
            .query_map([coach_id], |row| Self::map_row(row))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(sessions)
    }

    /// Create a new session
    pub fn create(
        conn: &Connection,
        title: &str,
        description: Option<&str>,
        scheduled_date: Option<NaiveDate>,
        scheduled_time: Option<NaiveTime>,
        duration_minutes: Option<i32>,
        skill_level: Option<&SkillLevel>,
        created_by: i64,
    ) -> Result<i64> {
        let skill_level_str = skill_level.map(|s| s.as_str());
        let scheduled_date_str = scheduled_date.map(|d| d.format("%Y-%m-%d").to_string());
        let scheduled_time_str = scheduled_time.map(|t| t.format("%H:%M:%S").to_string());

        conn.execute(
            "INSERT INTO sessions (title, description, scheduled_date, scheduled_time,
                                   duration_minutes, skill_level, created_by, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
            rusqlite::params![
                title,
                description,
                scheduled_date_str,
                scheduled_time_str,
                duration_minutes,
                skill_level_str,
                created_by,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Get a single session by ID
    pub fn find_by_id(conn: &Connection, id: i64) -> Result<Option<Session>> {
        let mut stmt = conn.prepare(
            "SELECT id, title, description, scheduled_date, scheduled_time,
                    duration_minutes, skill_level, created_by, created_at, updated_at
             FROM sessions
             WHERE id = ?",
        )?;

        let session = stmt.query_row([id], |row| Self::map_row(row));

        match session {
            Ok(s) => Ok(Some(s)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Delete a session
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM sessions WHERE id = ?", [id])?;
        Ok(())
    }

    // Helper to map a row to a Session
    fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Session> {
        let skill_level_str: Option<String> = row.get(6)?;
        let skill_level = skill_level_str.and_then(|s| SkillLevel::from_str(&s));

        let scheduled_date_str: Option<String> = row.get(3)?;
        let scheduled_date =
            scheduled_date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

        let scheduled_time_str: Option<String> = row.get(4)?;
        let scheduled_time =
            scheduled_time_str.and_then(|s| NaiveTime::parse_from_str(&s, "%H:%M:%S").ok());

        let created_at_str: String = row.get(8)?;
        let created_at =
            chrono::NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
                .map(|dt| {
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
                })
                .unwrap_or_else(|_| chrono::Utc::now());

        let updated_at_str: String = row.get(9)?;
        let updated_at =
            chrono::NaiveDateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S")
                .map(|dt| {
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
                })
                .unwrap_or_else(|_| chrono::Utc::now());

        Ok(Session {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            scheduled_date,
            scheduled_time,
            duration_minutes: row.get(5)?,
            skill_level,
            created_by: row.get(7)?,
            created_at,
            updated_at,
        })
    }
}
