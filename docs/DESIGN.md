# Tennis Training TUI App - Project Documentation

## Executive Summary

A multi-user TUI application for tennis training management where coaches create and manage training sessions, and players subscribe to and track their progress. Built with Rust and Ratatui, accessible via SSH.

**Target Users:** 1 coach + ~10-18 players (max 20 total users)  
**Primary Use Case:** Centralized training plan management with coach-player workflow  
**Tech Stack:** Rust, Ratatui, SSH (via russh), SQLite

---

## 1. Product Overview

### 1.1 User Roles

**Coach (Admin)**
- Create, edit, and delete training sessions
- Assign sessions to specific players or groups
- View all players' progress and completion status
- Manage player accounts
- Create session templates for reuse

**Player (Standard User)**
- View assigned sessions
- Subscribe to available sessions
- Mark sessions as completed
- Track personal training history
- View session details (drills, homework, quizzes)

### 1.2 Core Concepts

**Session**: A single training unit containing:
- Date/time schedule
- Training content (drills, exercises)
- Quizzes (knowledge checks)
- Homework (practice assignments)
- Target skill level
- Goals/objectives

**Subscription**: Player enrollment in a session, tracking:
- Subscription date
- Completion status
- Quiz results
- Notes

---

## 2. Feature List

### 2.1 MVP Features (Version 1.0)

**Authentication & User Management**
- ✓ SSH-based authentication with forced commands
- ✓ User role system (Coach/Player)
- ✓ Basic user profile (name, skill level, goals)

**Session Management (Coach)**
- ✓ Create training session with all fields
- ✓ Edit existing sessions
- ✓ Delete sessions
- ✓ List all sessions
- ✓ View session details
- ✓ Assign sessions to specific players

**Session Viewing (Player)**
- ✓ View assigned sessions
- ✓ View available sessions (browse catalog)
- ✓ Subscribe to sessions
- ✓ View session details
- ✓ Mark session as completed

**Session Tracking**
- ✓ View personal session history
- ✓ Track completion status
- ✓ Basic progress indicators

**Data Persistence**
- ✓ SQLite database for all data
- ✓ User data persistence
- ✓ Session and subscription tracking

### 2.2 Version 2.0 Features

**Enhanced Session Management**
- Session templates (save/reuse common session structures)
- Clone/duplicate sessions
- Bulk assign sessions to multiple players
- Session categories/tags
- Search and filter sessions

**Progress Tracking**
- Quiz scoring system
- Homework submission tracking
- Progress analytics per player
- Completion streaks
- Personal statistics dashboard

**Coach Tools**
- View all players' progress at a glance
- Generate progress reports
- Session attendance tracking
- Player performance comparisons

**Player Experience**
- Calendar view of sessions
- Reminders for upcoming sessions
- Personal notes on sessions
- Rate/feedback on sessions

### 2.3 Future Enhancements

**Advanced Features**
- Multi-week training programs (session series)
- Skill progression trees
- Automated plan generation based on skill level
- Export training history (CSV, PDF)
- Session sharing between coaches
- Player groups/teams management

**Social Features**
- Leaderboards (opt-in)
- Shared achievements
- Player-to-player messaging
- Group sessions/challenges

**Integration**
- Calendar export (iCal)
- Mobile companion app
- Web dashboard view
- Integration with fitness trackers

---

## 3. System Architecture

### 3.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Client Side                         │
│                    (User's SSH Client)                      │
└────────────────────────┬────────────────────────────────────┘
                         │ SSH Connection
                         │
┌────────────────────────▼────────────────────────────────────┐
│                      SSH Server Layer                       │
│                     (russh or custom)                       │
│  ┌─────────────────────────────────────────────────────┐    │
│  │  Authentication & Session Management                │    │
│  │  - authorized_keys mapping                          │    │
│  │  - User identification via forced command           │    │
│  └─────────────────────────────────────────────────────┘    │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                    Application Layer                        │
│                   (Rust TUI Application)                    │
│  ┌──────────────────┐  ┌──────────────────┐                 │
│  │  UI Layer        │  │  Business Logic  │                 │
│  │  (Ratatui)       │◄─┤  Layer           │                 │
│  │  - Screens       │  │  - Session mgmt  │                 │
│  │  - Components    │  │  - User mgmt     │                 │
│  │  - Navigation    │  │  - Subscription  │                 │
│  └──────────────────┘  └─────────┬────────┘                 │
│                                  │                          │
│                         ┌────────▼─────────┐                │
│                         │  Data Access     │                │
│                         │  Layer           │                │
│                         │  - Models        │                │
│                         │  - Repositories  │                │
│                         └────────┬─────────┘                │
└──────────────────────────────────┼──────────────────────────┘
                                   │
                         ┌─────────▼─────────┐
                         │  SQLite Database  │
                         │  - users.db       │
                         │  - sessions.db    │
                         └───────────────────┘
```

### 3.2 Module Structure

```
tennis-tui/
├── Cargo.toml
├── README.md
├── data/                      # SQLite databases
│   └── tennis.db
├── src/
│   ├── main.rs               # Entry point, CLI args parsing
│   ├── app.rs                # Main app state and event loop
│   ├── auth/
│   │   ├── mod.rs
│   │   └── user.rs           # User authentication & context
│   ├── db/
│   │   ├── mod.rs
│   │   ├── connection.rs     # SQLite connection pool
│   │   ├── migrations.rs     # Database schema migrations
│   │   └── repositories/
│   │       ├── mod.rs
│   │       ├── user_repo.rs
│   │       ├── session_repo.rs
│   │       └── subscription_repo.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── session.rs
│   │   ├── subscription.rs
│   │   ├── quiz.rs
│   │   └── homework.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── session_service.rs
│   │   ├── user_service.rs
│   │   └── subscription_service.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── app_ui.rs         # Main UI orchestrator
│   │   ├── screens/
│   │   │   ├── mod.rs
│   │   │   ├── home.rs
│   │   │   ├── session_list.rs
│   │   │   ├── session_detail.rs
│   │   │   ├── session_form.rs
│   │   │   ├── player_list.rs
│   │   │   └── progress.rs
│   │   ├── components/
│   │   │   ├── mod.rs
│   │   │   ├── header.rs
│   │   │   ├── footer.rs
│   │   │   ├── table.rs
│   │   │   ├── form.rs
│   │   │   └── modal.rs
│   │   └── navigation.rs
│   └── utils/
│       ├── mod.rs
│       ├── error.rs          # Custom error types
│       └── config.rs         # App configuration
└── tests/
    ├── integration/
    └── fixtures/
```

### 3.3 Technology Stack

**Core**
- **Language**: Rust (2021 edition)
- **TUI Framework**: Ratatui (0.26+)
- **Terminal Backend**: crossterm

**Data & Persistence**
- **Database**: SQLite (via rusqlite)
- **Migrations**: refinery or diesel-migrations

**Authentication & SSH**
- **SSH Access**: System SSH with authorized_keys forced commands
- **User Context**: Passed via CLI argument (`--user=username`)

**Development Tools**
- **Testing**: cargo test, proptest for property-based testing
- **Logging**: tracing + tracing-subscriber
- **Error Handling**: thiserror, anyhow

---

## 4. Data Models

### 4.1 Database Schema

```sql
-- Users table
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    display_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('coach', 'player')),
    skill_level TEXT CHECK(skill_level IN ('beginner', 'intermediate', 'advanced')),
    goals TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Sessions table
CREATE TABLE sessions (
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
);

-- Training content (drills/exercises)
CREATE TABLE training_content (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    content_type TEXT NOT NULL CHECK(content_type IN ('drill', 'exercise', 'warmup', 'cooldown')),
    title TEXT NOT NULL,
    description TEXT,
    duration_minutes INTEGER,
    order_index INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

-- Quizzes
CREATE TABLE quizzes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    question TEXT NOT NULL,
    correct_answer TEXT NOT NULL,
    options TEXT, -- JSON array of options
    order_index INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

-- Homework
CREATE TABLE homework (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    due_date DATE,
    order_index INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

-- Subscriptions (player enrollment in sessions)
CREATE TABLE subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    session_id INTEGER NOT NULL,
    subscribed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    status TEXT NOT NULL DEFAULT 'active' CHECK(status IN ('active', 'completed', 'cancelled')),
    notes TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    UNIQUE(user_id, session_id)
);

-- Quiz responses
CREATE TABLE quiz_responses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER NOT NULL,
    quiz_id INTEGER NOT NULL,
    answer TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL,
    answered_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (subscription_id) REFERENCES subscriptions(id) ON DELETE CASCADE,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id) ON DELETE CASCADE
);

-- Homework submissions
CREATE TABLE homework_submissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subscription_id INTEGER NOT NULL,
    homework_id INTEGER NOT NULL,
    notes TEXT,
    submitted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (subscription_id) REFERENCES subscriptions(id) ON DELETE CASCADE,
    FOREIGN KEY (homework_id) REFERENCES homework(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_sessions_created_by ON sessions(created_by);
CREATE INDEX idx_sessions_date ON sessions(scheduled_date);
CREATE INDEX idx_subscriptions_user ON subscriptions(user_id);
CREATE INDEX idx_subscriptions_session ON subscriptions(session_id);
CREATE INDEX idx_training_content_session ON training_content(session_id);
```

### 4.2 Rust Data Models

```rust
// models/user.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Coach,
    Player,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillLevel {
    Beginner,
    Intermediate,
    Advanced,
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

// models/session.rs
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct TrainingContent {
    pub id: i64,
    pub session_id: i64,
    pub content_type: ContentType,
    pub title: String,
    pub description: Option<String>,
    pub duration_minutes: Option<i32>,
    pub order_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Drill,
    Exercise,
    Warmup,
    Cooldown,
}

// models/subscription.rs
#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: i64,
    pub user_id: i64,
    pub session_id: i64,
    pub subscribed_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: SubscriptionStatus,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Active,
    Completed,
    Cancelled,
}
```

---

## 5. User Flows

### 5.1 Coach Workflow

```
┌─────────────┐
│   Login     │
│  (via SSH)  │
└──────┬──────┘
       │
       ▼
┌─────────────────┐
│   Home Screen   │
│  [Coach View]   │
│                 │
│  1. Sessions    │◄────────┐
│  2. Players     │         │
│  3. Analytics   │         │
│  4. Exit        │         │
└────┬───┬────────┘         │
     │   │                  │
     │   └──────────┐       │
     │              │       │
     ▼              ▼       │
┌──────────┐  ┌──────────┐  │
│ Sessions │  │ Players  │  │
│  List    │  │   List   │  │
└────┬─────┘  └──────────┘  │
     │                      │
     ├─► Create Session ────┤
     ├─► Edit Session ──────┤
     ├─► Delete Session ────┤
     ├─► Assign to Player ──┤
     └─► View Details ──────┘
```

### 5.2 Player Workflow

```
┌─────────────┐
│   Login     │
│  (via SSH)  │
└──────┬──────┘
       │
       ▼
┌─────────────────┐
│   Home Screen   │
│  [Player View]  │
│                 │
│  1. My Sessions │◄────────┐
│  2. Browse      │         │
│  3. History     │         │
│  4. Exit        │         │
└────┬───┬────────┘         │
     │   │                  │
     │   └──────────┐       │
     │              │       │
     ▼              ▼       │
┌──────────┐  ┌──────────┐  │
│   My     │  │  Browse  │  │
│ Sessions │  │ Sessions │  │
└────┬─────┘  └────┬─────┘  │
     │             │        │
     ├─► View ─────┼────────┤
     ├─► Complete ─┤        │
     └─► Subscribe ┘        │
            │               │
            ▼               │
       ┌──────────┐         │
       │ Session  │         │
       │ Details  │─────────┘
       │          │
       │ - Info   │
       │ - Drills │
       │ - Quiz   │
       │ - HW     │
       └──────────┘
```

---

## 6. Implementation Phases

### Phase 1: Foundation (Week 1-2)
- Set up Rust project structure
- Implement SQLite database with migrations
- Create data models and repositories
- Basic CLI argument parsing (`--user` flag)
- Simple TUI skeleton with Ratatui

**Deliverable**: App launches, connects to DB, displays placeholder UI

### Phase 2: Core Session Management (Week 3-4)
- Coach: Create/edit/delete sessions (basic fields only)
- Coach: View session list
- Player: View assigned sessions list
- Basic navigation between screens
- Session detail view

**Deliverable**: Coaches can manage sessions, players can view them

### Phase 3: Subscription System (Week 5)
- Player: Subscribe to sessions
- Player: Mark sessions complete
- Database relationships working
- Status tracking

**Deliverable**: Full workflow from session creation to completion

### Phase 4: Rich Content (Week 6-7)
- Add training content (drills, exercises)
- Add quizzes to sessions
- Add homework to sessions
- Enhanced session detail view
- Form validation

**Deliverable**: Sessions have full content structure

### Phase 5: Polish & MVP Launch (Week 8)
- Error handling and validation
- User feedback messages
- Help screens
- Logging
- Testing
- Documentation

**Deliverable**: Production-ready MVP

### Phase 6: V2 Features (Post-MVP)
- Session templates
- Progress analytics
- Search/filter
- Calendar view
- Export functionality

---

## 7. Development Guidelines

### 7.1 Code Style
- Follow Rust standard style (rustfmt)
- Use clippy for linting
- Meaningful variable names, avoid abbreviations
- Document public APIs with doc comments
- Keep functions small and focused

### 7.2 Error Handling
- Use `Result<T, E>` for fallible operations
- Create custom error types with `thiserror`
- Use `anyhow::Result` for application-level errors
- Never use `.unwrap()` in production code
- Provide helpful error messages to users

### 7.3 Testing Strategy
- Unit tests for business logic
- Integration tests for database operations
- Fixture data for testing
- Test both coach and player workflows
- Aim for >70% code coverage on core logic

### 7.4 Security Considerations
- Never store passwords in database (SSH handles auth)
- Validate all user input
- Use parameterized queries (prevent SQL injection)
- Sanitize data before display
- Role-based access control for all operations

---

## 8. Deployment Guide

### 8.1 Server Setup

```bash
# Create system user for the app
sudo useradd -m -s /bin/bash tennis-tui

# Create directory structure
sudo -u tennis-tui mkdir -p /home/tennis-tui/{bin,data,logs}

# Set permissions
sudo chmod 700 /home/tennis-tui/.ssh
```

### 8.2 Build & Deploy

```bash
# Build release binary
cargo build --release

# Copy to server
scp target/release/tennis-tui user@server:/home/tennis-tui/bin/

# Set executable
ssh user@server "chmod +x /home/tennis-tui/bin/tennis-tui"

# Initialize database
ssh user@server "/home/tennis-tui/bin/tennis-tui --init-db"
```

### 8.3 User Provisioning

```bash
# Generate SSH key for a user (on your machine)
ssh-keygen -t ed25519 -f alice_key -C "alice" -N ""

# Add to authorized_keys on server
cat >> /home/tennis-tui/.ssh/authorized_keys << 'EOF'
command="/home/tennis-tui/bin/tennis-tui --user=alice",no-pty,no-port-forwarding,no-X11-forwarding ssh-ed25519 AAAAC3... alice
EOF

# Send private key to Alice securely
```

### 8.4 Seed Initial Data

```sql
-- Create coach account
INSERT INTO users (username, display_name, role) 
VALUES ('coach', 'Coach John', 'coach');

-- Create player accounts  
INSERT INTO users (username, display_name, role, skill_level, goals)
VALUES 
  ('alice', 'Alice Smith', 'player', 'beginner', 'Improve serve'),
  ('bob', 'Bob Jones', 'player', 'intermediate', 'Tournament prep');
```

---

## 9. Configuration

### 9.1 Config File (`config.toml`)

```toml
[database]
path = "/home/tennis-tui/data/tennis.db"

[logging]
level = "info"
path = "/home/tennis-tui/logs/tennis-tui.log"

[app]
max_sessions_per_page = 20
session_cache_ttl = 300  # seconds
```

### 9.2 Environment Variables

```bash
# Optional overrides
export TENNIS_DB_PATH="/custom/path/tennis.db"
export TENNIS_LOG_LEVEL="debug"
```

---

## 10. Monitoring & Maintenance

### 10.1 Logging

- Log all authentication attempts
- Log database operations (errors)
- Log user actions (session creation, subscription)
- Rotate logs weekly

### 10.2 Backups

```bash
# Daily backup script
#!/bin/bash
DATE=$(date +%Y%m%d)
sqlite3 /home/tennis-tui/data/tennis.db ".backup /home/tennis-tui/backups/tennis-$DATE.db"
find /home/tennis-tui/backups -name "tennis-*.db" -mtime +30 -delete
```

### 10.3 Monitoring Checklist

- Disk space for database growth
- Log file sizes
- Active user connections
- Database integrity (`PRAGMA integrity_check`)

---

## 11. Future Technical Considerations

### 11.1 Scalability
If user base grows beyond 50 users:
- Consider PostgreSQL instead of SQLite
- Add connection pooling
- Implement caching layer (Redis)
- Horizontal scaling with load balancer

### 11.2 Feature Flags
Use feature flags for:
- A/B testing new features
- Gradual rollouts
- Emergency feature disable

### 11.3 API Layer
For future mobile/web apps:
- REST API with actix-web or axum
- GraphQL with async-graphql
- JWT authentication
- Rate limiting

---

## Appendix A: Useful Crates

```toml
[dependencies]
# TUI
ratatui = "0.26"
crossterm = "0.27"

# Database
rusqlite = { version = "0.31", features = ["bundled"] }
refinery = { version = "0.8", features = ["rusqlite"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Date/Time
chrono = "0.4"

# Error Handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# CLI
clap = { version = "4.5", features = ["derive"] }

# Utilities
uuid = { version = "1.7", features = ["v4"] }

[dev-dependencies]
proptest = "1.4"
tempfile = "3.10"
```

## Appendix B: Quick Start Commands

```bash
# Clone and build
git clone <repo> && cd tennis-tui
cargo build --release

# Run locally (dev mode)
cargo run -- --user=coach

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt --check

# Generate docs
cargo doc --open
```

---

**Document Version**: 1.0  
**Last Updated**: January 2026  
**Maintained By**: [Your Name]
