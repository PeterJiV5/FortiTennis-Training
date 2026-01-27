use rusqlite::Connection;
use crate::models::{Subscription, SubscriptionStatus};
use crate::utils::Result;

pub struct SubscriptionRepository;

impl SubscriptionRepository {
    /// Subscribe a user to a session
    pub fn create(conn: &Connection, user_id:i64, session_id: i64) -> Result<i64> {
        conn.execute(
            "INSERT INTO subscriptions (user_id, session_id, status, subscribed_at)
             VALUES (?, ?, 'active', datetime('now'))",
             rusqlite::params![user_id, session_id],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Check if a user is subscribed to a session
    pub fn is_subscribed(conn: &Connection, user_id: i64, session_id: i64) -> Result<bool> {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subscriptions
             WHERE user_id = ? AND session_id = ?",
             rusqlite::params![user_id, session_id],
             |row| row.get(0),
        )?;

        Ok(count > 0)
    }

    /// Get a subscription by user and session
    pub fn find_by_user_and_session(conn: &Connection, user_id: i64, session_id: i64) -> Result<Option<Subscription>> {
        match conn.query_row(
            "SELECT id, user_id, session_id, subscribed_at, completed_at, status, notes
             FROM subscriptions
             WHERE user_id = ? AND session_id = ?",
            rusqlite::params![user_id, session_id],
            Self::map_row,
        ) {
            Ok(sub) => Ok(Some(sub)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get all subscriptions for a user
    pub fn find_by_user(conn: &Connection, user_id: i64) -> Result<Vec<Subscription>> {
        let mut stmt = conn.prepare(
            "SELECT id, user_id, session_id, subscribed_at, completed_at, status, notes
             FROM subscriptions
             WHERE user_id = ?
             ORDER BY subscribed_at DESC",
        )?;

        let subscriptions_iter = stmt.query_map([user_id], |row| Self::map_row(row))?;

        let mut subscriptions = Vec::new();
        for subscription_result in subscriptions_iter {
            subscriptions.push(subscription_result?)
        }

        Ok(subscriptions)
    }

    /// Get all subscriptions for a session
    pub fn find_by_session(conn: &Connection, session_id: i64) -> Result<Vec<Subscription>> {
        let mut stmt = conn.prepare(
            "SELECT id, user_id, session_id, subscribed_at, completed_at, status, notes
             FROM subscriptions
             WHERE session_id = ?
             ORDER BY subscribed_at DESC",
        )?;

        let subscriptions_iter = stmt.query_map([session_id], |row| Self::map_row(row))?;

        let mut subscriptions = Vec::new();
        for subscription_result in subscriptions_iter {
            subscriptions.push(subscription_result?)
        }

        Ok(subscriptions)
    }

    /// Makr a subscription as completed
    pub fn mark_completed(conn: &Connection, subscription_id: i64) -> Result<()> {
        conn.execute(
            "UPDATE subscriptions
             SET completed_at = datetime('now'), status = 'completed'
             WHERE id = ?",
             [subscription_id],
        )?;

        Ok(())
    }

    /// Unsubscribe (delete subscription)
    pub fn delete(conn: &Connection, subscription_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM subscriptions
             WHERE id = ?",
             [subscription_id],
        )?;

        Ok(())
    }

    /// Delete by user and session
    pub fn delete_by_user_and_session(conn: &Connection, user_id: i64, session_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM subscriptions
             WHERE user_id = ? AND session_id = ?",
             rusqlite::params![user_id, session_id],
        )?;
        Ok(())
    }

    // Helper to map a row to Subscription
    fn map_row(row: &rusqlite::Row) -> rusqlite::Result<Subscription> {
        let status_str: String = row.get(5)?;
        let status = SubscriptionStatus::from_str(&status_str)
            .ok_or_else(|| rusqlite::Error::InvalidQuery)?;

        let subscribed_at_str: String = row.get(3)?;
        let subscribed_at = 
            chrono::NaiveDateTime::parse_from_str(&subscribed_at_str, "%Y-%m-%d %H:%M:%S")
                .map(|dt| {
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
                })
                .unwrap_or_else(|_| chrono::Utc::now());

        let completed_at_str: Option<String> = row.get(4)?;
        let completed_at = completed_at_str.and_then(|s| {
            chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                .map(|dt| {
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
                })
                .ok()
        });

        Ok(Subscription {
            id: row.get(0)?,
            user_id: row.get(1)?,
            session_id: row.get(2)?,
            subscribed_at,
            completed_at,
            status,
            notes: row.get(6)?,
        })
    }
}