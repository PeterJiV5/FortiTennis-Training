mod auth;
mod db;
mod models;
mod ui;
mod utils;

use clap::Parser;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use tracing_subscriber;

use auth::UserContext;
use db::{establish_connection, run_migrations};
use models::{User, UserRole};
use ui::App;
use utils::AppError;

#[derive(Parser, Debug)]
#[command(name = "tennis-tui")]
#[command(about = "Tennis Training TUI Application", long_about = None)]
struct Args {
    // Username to authenticate as
    #[arg(long)]
    user: Option<String>,

    /// Path to SQLite database
    #[arg(long, default_value = "data/tennis.db")]
    db_path: String,

    /// Initialize database with sample data
    #[arg(long)]
    init_db: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Args::parse();

    // Establish database connection
    let conn = establish_connection(&args.db_path)?;

    // Run migrations
    run_migrations(&conn)?;

    // Init database with sample data if requested
    if args.init_db {
        init_sample_data(&conn)?;
        println!("Database initialized with sample data.");
        return Ok(());
    }

    // User is required for running the application
    let username = args.user.ok_or_else(|| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Error: --user is required",
        )) as Box<dyn std::error::Error>
    })?;

    // Load user from database
    let user = load_user(&conn, &username)?;
    let user_context = UserContext::new(user);

    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create and run app
    let mut app = App::new(user_context, args.db_path);
    let res = app.run(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn load_user(conn: &rusqlite::Connection, username: &str) -> Result<User, AppError> {
    eprintln!("DEBUG: Attempting to load user: {}", username);

    let mut stmt = conn.prepare(
        "SELECT id, username, display_name, role, skill_level, goals, created_at, updated_at
         FROM users WHERE username = ?",
    )?;

    let user = stmt.query_row([username], |row| {
        let role_str: String = row.get(3)?;
        eprintln!("DEBUG: role_str = {}", role_str);

        let skill_level_str: Option<String> = row.get(4)?;
        eprintln!("DEBUG: skill_level_str = {:?}", skill_level_str);

        let role = match UserRole::from_str(&role_str) {
            Some(r) => {
                eprintln!("DEBUG: Parsed role successfully: {:?}", r);
                r
            }
            None => {
                eprintln!("DEBUG: Failed to parse role from: '{}'", role_str);
                return Err(rusqlite::Error::InvalidQuery);
            }
        };

        let skill_level = skill_level_str.and_then(|s| models::SkillLevel::from_str(&s));
        eprintln!("DEBUG: skill_level parsed: {:?}", skill_level);

        let created_at_str: String = row.get(6)?;
        eprintln!("DEBUG: created_at_str = {}", created_at_str);
        let updated_at_str: String = row.get(7)?;
        eprintln!("DEBUG: updated_at_str = {}", updated_at_str);

        // Parse SQLite datetime format (YYYY-MM-DD HH:MM:SS)
        let created_at =
            chrono::NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
                .map(|dt| {
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
                })
                .unwrap_or_else(|_| chrono::Utc::now());
        eprintln!("DEBUG: created_at parsed successfully");

        let updated_at =
            chrono::NaiveDateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S")
                .map(|dt| {
                    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
                })
                .unwrap_or_else(|_| chrono::Utc::now());
        eprintln!("DEBUG: updated_at parsed successfully");

        eprintln!("DEBUG: About to create User struct");

        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            display_name: row.get(2)?,
            role,
            skill_level,
            goals: row.get(5)?,
            created_at,
            updated_at,
        })
    })?;

    Ok(user)
}

fn init_sample_data(conn: &rusqlite::Connection) -> Result<(), AppError> {
    // Insert coach
    conn.execute(
        "INSERT OR IGNORE INTO users (username, display_name, role, created_at, updated_at)
         VALUES (?, ?, ?, datetime('now'), datetime('now'))",
        ["coach", "Coach Peter", "coach"],
    )?;

    // Insert sample players
    conn.execute(
        "INSERT OR IGNORE INTO users (username, display_name, role, skill_level, goals, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        ["alice", "Alice Smith", "player", "beginner", "Improve serve and backhand"],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO users (username, display_name, role, skill_level, goals, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        ["bob", "Bob Jones", "player", "intermediate", "Prepare for tournament"],
    )?;

    Ok(())
}
