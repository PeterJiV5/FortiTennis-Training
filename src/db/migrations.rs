use rusqlite::Connection;
use crate::utils::Result;

pub fn run_migrations(conn: &Connection) -> Result<()> {
	// Create users table
	conn.execute(
		"CREATE TABLE IF NOT EXISTS users (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			username TEXT UNIQUE NOT NULL,
			display_name TEXT NOT NULL,
			role TEXT NOT NULL CHECK(role IN ('coach', 'player')),
			skill_level TEXT CHECK(skill_level IN ('beginner', 'intermediate', 'advanced')),
			goals TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)",
		[]
	)?;

	// Create sessions table
	conn.execute(
		"CREATE TABLE IF NOT EXISTS sessions (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			title TEXT NOT NULL,
			description TEXT,
			scheduled_date DATE,
			scheduled_time TIME,
			duration_minutes INTEGER,
			skill_level TEXT,
			created_by INTEGER NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			FOREIGN KEY (created_by) REFERENCES users(id)
		)",
		[]
	)?;

	// Create training content table
	conn.execute(
		"CREATE TABLE IF NOT EXISTS training_content (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			session_id INTEGER NOT NULL,
			content_type TEXT NOT NULL CHECK(content_type IN ('drill', 'exercise', 'warmup', 'cooldown')),
			title TEXT NOT NULL,
			description TEXT,
			duration_minutes INTEGER,
			order_index INTEGER NOT NULL,
			FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
		)",
		[]
	)?;

	// create quizzes table
	conn.execute(
		"CREATE TABLE IF NOT EXISTS quizzes (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			session_id INTEGER NOT NULL,
			question TEXT NOT NULL,
			correct_answer TEXT NOT NULL,
			options TEXT,
			order_index INTEGER NOT NULL,
			FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
		)",
		[]
	)?;

	// create homework table
	conn.execute(
		"CREATE TABLE IF NOT EXISTS homework (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			session_id INTEGER NOT NULL,
			title TEXT NOT NULL,
			description TEXT NOT NULL,
			due_date DATE,
			order_index INTEGER NOT NULL,
			FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
		)",
		[]
	)?;

	// create subscriptions table
	conn.execute(
		"CREATE TABLE IF NOT EXISTS subscriptions (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			user_id INTEGER NOT NULL,
			session_id INTEGER NOT NULL,
			subscribed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			completed_at DATETIME,
			status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'completed', 'cancelled')),
			notes TEXT,
			FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
			FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
			UNIQUE(user_id, session_id)
		)",
		[]
	)?;

	// create quiz responses table
	conn.execute(
		"CREATE TABLE IF NOT EXISTS quiz_responses (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			subscription_id INTEGER NOT NULL,
			quiz_id INTEGER NOT NULL,
			answer TEXT NOT NULL,
			is_correct BOOLEAN NOT NULL,
			answered_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			FOREIGN KEY (subscription_id) REFERENCES subscriptions(id) ON DELETE CASCADE,
			FOREIGN KEY (quiz_id) REFERENCES quizzes(id) ON DELETE CASCADE
		)",
		[]
	)?;

	// create homework submissions table
	conn.execute(
		"CREATE TABLE IF NOT EXISTS homework_submissions (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			subscription_id INTEGER NOT NULL,
			homework_id INTEGER NOT NULL,
			notes TEXT,
			submitted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			FOREIGN KEY (subscription_id) REFERENCES subscriptions(id) ON DELETE CASCADE,
			FOREIGN KEY (homework_id) REFERENCES homework(id) ON DELETE CASCADE
		)",
		[]
	)?;

	// create indexes
	conn.execute(
		"CREATE INDEX IF NOT EXISTS idx_sessions_created_by ON sessions(created_by)",
		[],
	)?;
	conn.execute(
		"CREATE INDEX IF NOT EXISTS idx_sessions_date ON sessions(scheduled_date)",
		[],
	)?;
	conn.execute(
		"CREATE INDEX IF NOT EXISTS idx_subscriptions_user ON subscriptions(user_id)",
		[],
	)?;
	conn.execute(
		"CREATE INDEX IF NOT EXISTS idx_subscriptions_session ON subscriptions(session_id)",
		[],
	)?;
	conn.execute(
		"CREATE INDEX IF NOT EXISTS idx_training_content_session ON training_content(session_id)",
		[],
	)?;
	conn.execute(
		"CREATE INDEX IF NOT EXISTS idx_quizzes_session ON quizzes(session_id)",
		[],
	)?;
	conn.execute(
		"CREATE INDEX IF NOT EXISTS idx_homework_session ON homework(session_id)",
		[],
	)?;

	Ok(())
}