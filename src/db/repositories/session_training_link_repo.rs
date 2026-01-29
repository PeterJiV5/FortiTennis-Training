use crate::models::{SessionTrainingLink, SessionTrainingLinkWithTemplate, TrainingTemplate};
use crate::utils::error::Result;
use rusqlite::Connection;

pub struct SessionTrainingLinkRepository;

impl SessionTrainingLinkRepository {
    pub fn add_to_session(
        conn: &Connection,
        session_id: i64,
        template_id: i64,
        order_index: i32,
    ) -> Result<i64> {
        let mut stmt = conn.prepare(
            "INSERT INTO session_training_links (session_id, training_template_id, order_index) 
             VALUES (?, ?, ?)"
        )?;

        let id = stmt.insert(rusqlite::params![session_id, template_id, order_index])?;
        Ok(id as i64)
    }

    pub fn add_to_session_with_notes(
        conn: &Connection,
        session_id: i64,
        template_id: i64,
        order_index: i32,
        custom_notes: Option<&str>,
    ) -> Result<i64> {
        let mut stmt = conn.prepare(
            "INSERT INTO session_training_links (session_id, training_template_id, order_index, custom_notes) 
             VALUES (?, ?, ?, ?)"
        )?;

        let id = stmt.insert(rusqlite::params![session_id, template_id, order_index, custom_notes])?;
        Ok(id as i64)
    }

    pub fn get_for_session(conn: &Connection, session_id: i64) -> Result<Vec<SessionTrainingLink>> {
        let mut stmt = conn.prepare(
            "SELECT id, session_id, training_template_id, order_index, custom_notes 
             FROM session_training_links WHERE session_id = ? ORDER BY order_index"
        )?;

        let links = stmt.query_map(rusqlite::params![session_id], |row| {
            Ok(SessionTrainingLink {
                id: row.get(0)?,
                session_id: row.get(1)?,
                training_template_id: row.get(2)?,
                order_index: row.get(3)?,
                custom_notes: row.get(4)?,
            })
        })?;

        let mut results = Vec::new();
        for link in links {
            results.push(link?);
        }

        Ok(results)
    }

    pub fn get_for_session_with_templates(
        conn: &Connection,
        session_id: i64,
    ) -> Result<Vec<SessionTrainingLinkWithTemplate>> {
        let mut stmt = conn.prepare(
            "SELECT 
                stl.id, stl.session_id, stl.training_template_id, stl.order_index, stl.custom_notes,
                tt.id, tt.coach_id, tt.title, tt.content_type, tt.description, tt.duration_minutes,
                tt.created_at, tt.created_by, tt.last_edited_by, tt.last_edited_at, tt.is_public
             FROM session_training_links stl
             LEFT JOIN training_templates tt ON stl.training_template_id = tt.id
             WHERE stl.session_id = ? ORDER BY stl.order_index"
        )?;

        let links = stmt.query_map(rusqlite::params![session_id], |row| {
            let template = if let Ok(template_id) = row.get::<_, i64>(5) {
                Some(TrainingTemplate {
                    id: template_id,
                    coach_id: row.get(6)?,
                    title: row.get(7)?,
                    content_type: row.get(8)?,
                    description: row.get(9)?,
                    duration_minutes: row.get(10)?,
                    created_at: row.get(11)?,
                    created_by: row.get(12)?,
                    last_edited_by: row.get(13)?,
                    last_edited_at: row.get(14)?,
                    is_public: row.get::<_, i32>(15)? != 0,
                })
            } else {
                None
            };

            Ok(SessionTrainingLinkWithTemplate {
                link: SessionTrainingLink {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    training_template_id: row.get(2)?,
                    order_index: row.get(3)?,
                    custom_notes: row.get(4)?,
                },
                template,
            })
        })?;

        let mut results = Vec::new();
        for link in links {
            results.push(link?);
        }

        Ok(results)
    }

    pub fn remove_from_session(conn: &Connection, session_id: i64, template_id: i64) -> Result<()> {
        let mut stmt = conn.prepare(
            "DELETE FROM session_training_links WHERE session_id = ? AND training_template_id = ?"
        )?;
        stmt.execute(rusqlite::params![session_id, template_id])?;
        Ok(())
    }

    pub fn update_custom_notes(conn: &Connection, link_id: i64, notes: &str) -> Result<()> {
        let mut stmt = conn.prepare(
            "UPDATE session_training_links SET custom_notes = ? WHERE id = ?"
        )?;
        stmt.execute(rusqlite::params![notes, link_id])?;
        Ok(())
    }

    pub fn reorder_in_session(
        conn: &Connection,
        session_id: i64,
        template_ids: &[i64],
    ) -> Result<()> {
        for (order_index, template_id) in template_ids.iter().enumerate() {
            let mut stmt = conn.prepare(
                "UPDATE session_training_links SET order_index = ? WHERE session_id = ? AND training_template_id = ?"
            )?;
            stmt.execute(rusqlite::params![order_index as i32, session_id, template_id])?;
        }
        Ok(())
    }
}
