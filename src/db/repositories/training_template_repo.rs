use crate::models::{TemplateAuditInfo, TrainingTemplate};
use crate::utils::error::Result;
use rusqlite::Connection;

pub struct TrainingTemplateRepository;

impl TrainingTemplateRepository {
    pub fn create(conn: &Connection, template: &TrainingTemplate, created_by: i64) -> Result<i64> {
        let mut stmt = conn.prepare(
            "INSERT INTO training_templates (coach_id, title, content_type, description, 
             duration_minutes, created_by, is_public, created_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)"
        )?;

        let id = stmt.insert(rusqlite::params![
            template.coach_id,
            &template.title,
            &template.content_type,
            template.description,
            template.duration_minutes,
            created_by,
            template.is_public
        ])?;

        Ok(id as i64)
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<TrainingTemplate> {
        let mut stmt = conn.prepare(
            "SELECT id, coach_id, title, content_type, description, duration_minutes, 
                    created_at, created_by, last_edited_by, last_edited_at, is_public 
             FROM training_templates WHERE id = ?"
        )?;

        let template = stmt.query_row(rusqlite::params![id], |row| {
            Ok(TrainingTemplate {
                id: row.get(0)?,
                coach_id: row.get(1)?,
                title: row.get(2)?,
                content_type: row.get(3)?,
                description: row.get(4)?,
                duration_minutes: row.get(5)?,
                created_at: row.get(6)?,
                created_by: row.get(7)?,
                last_edited_by: row.get(8)?,
                last_edited_at: row.get(9)?,
                is_public: row.get::<_, i32>(10)? != 0,
            })
        })?;

        Ok(template)
    }

    pub fn list_all(conn: &Connection) -> Result<Vec<TrainingTemplate>> {
        let mut stmt = conn.prepare(
            "SELECT id, coach_id, title, content_type, description, duration_minutes, 
                    created_at, created_by, last_edited_by, last_edited_at, is_public 
             FROM training_templates WHERE is_public = 1 ORDER BY created_at DESC"
        )?;

        let templates = stmt.query_map([], |row| {
            Ok(TrainingTemplate {
                id: row.get(0)?,
                coach_id: row.get(1)?,
                title: row.get(2)?,
                content_type: row.get(3)?,
                description: row.get(4)?,
                duration_minutes: row.get(5)?,
                created_at: row.get(6)?,
                created_by: row.get(7)?,
                last_edited_by: row.get(8)?,
                last_edited_at: row.get(9)?,
                is_public: row.get::<_, i32>(10)? != 0,
            })
        })?;

        let mut results = Vec::new();
        for template in templates {
            results.push(template?);
        }

        Ok(results)
    }

    pub fn list_by_coach(conn: &Connection, coach_id: i64) -> Result<Vec<TrainingTemplate>> {
        let mut stmt = conn.prepare(
            "SELECT id, coach_id, title, content_type, description, duration_minutes, 
                    created_at, created_by, last_edited_by, last_edited_at, is_public 
             FROM training_templates WHERE coach_id = ? ORDER BY created_at DESC"
        )?;

        let templates = stmt.query_map(rusqlite::params![coach_id], |row| {
            Ok(TrainingTemplate {
                id: row.get(0)?,
                coach_id: row.get(1)?,
                title: row.get(2)?,
                content_type: row.get(3)?,
                description: row.get(4)?,
                duration_minutes: row.get(5)?,
                created_at: row.get(6)?,
                created_by: row.get(7)?,
                last_edited_by: row.get(8)?,
                last_edited_at: row.get(9)?,
                is_public: row.get::<_, i32>(10)? != 0,
            })
        })?;

        let mut results = Vec::new();
        for template in templates {
            results.push(template?);
        }

        Ok(results)
    }

    pub fn update(conn: &Connection, template: &TrainingTemplate, edited_by: i64) -> Result<()> {
        let mut stmt = conn.prepare(
            "UPDATE training_templates 
             SET title = ?, content_type = ?, description = ?, duration_minutes = ?, 
                 last_edited_by = ?, last_edited_at = CURRENT_TIMESTAMP 
             WHERE id = ?"
        )?;

        stmt.execute(rusqlite::params![
            &template.title,
            &template.content_type,
            template.description,
            template.duration_minutes,
            edited_by,
            template.id
        ])?;

        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        let mut stmt = conn.prepare("DELETE FROM training_templates WHERE id = ?")?;
        stmt.execute(rusqlite::params![id])?;
        Ok(())
    }

    pub fn get_audit_info(conn: &Connection, template_id: i64) -> Result<TemplateAuditInfo> {
        let mut stmt = conn.prepare(
            "SELECT 
                (SELECT display_name FROM users WHERE id = training_templates.created_by) as created_by_name,
                (SELECT display_name FROM users WHERE id = training_templates.last_edited_by) as edited_by_name
             FROM training_templates WHERE id = ?"
        )?;

        let audit_info = stmt.query_row(rusqlite::params![template_id], |row| {
            Ok(TemplateAuditInfo {
                created_by_name: row.get(0)?,
                last_edited_by_name: row.get(1)?,
            })
        })?;

        Ok(audit_info)
    }

    pub fn get_usage_count(conn: &Connection, template_id: i64) -> Result<i64> {
        let mut stmt = conn.prepare(
            "SELECT COUNT(*) FROM session_training_links WHERE training_template_id = ?"
        )?;

        let count = stmt.query_row(rusqlite::params![template_id], |row| {
            row.get(0)
        })?;

        Ok(count)
    }
}
