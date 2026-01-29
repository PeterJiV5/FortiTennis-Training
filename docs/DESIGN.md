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

### 3.3 Visual Screen Hierarchy

The application uses a hierarchical screen navigation model with the Home screen as the root:

```
┌─────────────────────────────────────────────────────────────┐
│                         HOME SCREEN (Root)                  │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ > Help                 (Menu Item - Yellow, Bold)     │   │
│  │   Manage Sessions / My Sessions (Depending on role)  │   │
│  │                                                      │   │
│  │ Controls: ↑↓ Navigate | Enter Select | q Quit | ? Help│   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
  │                                    │
  │                                    └──────────────────┐
  │                                                       │
  │                                    ┌──────────────────▼──────────────────┐
  │                                    │                                     │
  ▼                                    ▼                                     │
┌─────────────────────┐     ┌──────────────────────────────────────┐      │
│   HELP SCREEN       │     │   SESSION LIST SCREEN               │      │
│ (Accessible from    │     │ (Coach: Manage Sessions)            │      │
│  all screens via    │     │ (Player: My Sessions)               │      │
│  [?] key or menu)   │     │                                    │      │
│                     │     │ ↑↓ Navigate | Enter Select         │      │
│                     │     │ c/C Create | e/E Edit              │      │
│                     │     │ d/D Delete | s/S Subscribe         │      │
│                     │     │ f/F Filter (Player) | q Quit       │      │
│                     │     │ Esc Go Back | ? Help               │      │
│                     │     │                                    │      │
│                     │     │ Navigation paths:                 │      │
│                     │     │   ↓ Enter                          │      │
└─────────────────────┘     │   └────► SESSION DETAIL            │      │
         ▲                  │           │                         │      │
         │                  │           ├─► SESSION EDIT         │      │
         │                  │           ├─► SESSION DELETE       │      │
         │                  │           └─► TRAINING CONTENT     │      │
         │                  │                                    │      │
         │                  └────────────────────────────────────┘      │
         │                                                               │
         │                                                               │
         └───────────────────────────────────────────────────────────────┘
                        (Esc from any screen returns to Home)

┌────────────────────────────────────────────────────────────┐
│  SESSION DETAIL SCREEN                                     │
│  ├─ View session info (date, time, level, etc.)           │
│  ├─ View training content list                            │
│  ├─ (Coach) Edit session: e/E → SESSION EDIT SCREEN       │
│  ├─ (Coach) Delete session: d/D → CONFIRM & delete        │
│  ├─ (Coach) Add training content: t/T → TRAINING CONTENT  │
│  ├─ (Player) Mark complete: m/M                           │
│  └─ Navigate back: Esc, q or Back button                  │
└────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────┐
│  TRAINING CONTENT MANAGEMENT                              │
│  ├─ View training items (warmup, drills, homework, etc.)  │
│  ├─ Add new training: [Create] form                       │
│  ├─ Edit existing: Navigation and edit forms              │
│  ├─ Delete: With confirmation                            │
│  └─ Return to Session Detail: Esc or back button          │
└────────────────────────────────────────────────────────────┘

Key Navigation Rules:
- **Home** is the root of all navigation
- **Help** is accessible from any screen via [?] key
- **Esc/Backspace** returns to Home from any screen (or quits if already on Home)
- **q key** quits the entire application from any screen
- **Enter key** selects the highlighted menu item on Home screen
- **↑↓ arrow keys** (or j/k) navigate menus and lists
```

### 3.4 Technology Stack

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

---

## 9. Training Templates & Cursor Memory (Planned Features)

### 9.1 Training Templates Architecture

**Goal**: Reduce training content duplication by enabling coaches to create reusable templates that multiple sessions can reference.

**Confirmed Design Decisions:**
- **Implementation Strategy**: Option A - Migrate all existing training_content into templates + links (cleaner architecture, requires migration)
- **Template Scope**: Global (all coaches can see templates), with creator/editor tracking
- **Session Binding**: Hybrid (templates referenced by sessions, with per-session overrides allowed)

#### 9.1.1 Database Schema Changes

```sql
-- NEW: Training Templates (global library)
CREATE TABLE training_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    coach_id INTEGER NOT NULL,              -- Who created it
    title TEXT NOT NULL,                    -- e.g., "Backhand Drill Pack"
    content_type TEXT NOT NULL CHECK(content_type IN ('drill', 'exercise', 'warmup', 'cooldown', 'quiz', 'homework')),
    description TEXT,
    duration_minutes INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by INTEGER NOT NULL,            -- Explicit creator
    last_edited_by INTEGER,                 -- Last editor
    last_edited_at DATETIME,
    is_public BOOLEAN DEFAULT 1,            -- Shared across all sessions
    FOREIGN KEY (coach_id) REFERENCES users(id),
    FOREIGN KEY (created_by) REFERENCES users(id),
    FOREIGN KEY (last_edited_by) REFERENCES users(id)
);

-- NEW: Session-Template Links (junction table)
CREATE TABLE session_training_links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id INTEGER NOT NULL,
    training_template_id INTEGER NOT NULL,
    order_index INTEGER NOT NULL,
    custom_notes TEXT,                      -- Session-specific overrides/notes
    UNIQUE(session_id, order_index),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
    FOREIGN KEY (training_template_id) REFERENCES training_templates(id) ON DELETE CASCADE
);

-- MODIFY: training_content (keep for legacy, populate from templates at runtime)
-- OR MIGRATE: Move all data to training_templates and update references
-- Decision: MIGRATE (cleaner, Option A)

-- Migration Strategy:
-- 1. Create training_templates with data from training_content
-- 2. Create session_training_links mapping sessions to templates
-- 3. Archive original training_content table
-- 4. Update code to read from templates
```

#### 9.1.2 Data Models

```rust
// src/models/training_template.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingTemplate {
    pub id: i64,
    pub coach_id: i64,                      // Global creator
    pub title: String,
    pub content_type: ContentType,
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
    pub created_by_name: String,            // Coach name who created
    pub last_edited_by_name: Option<String>, // Coach name who last edited
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTrainingLink {
    pub id: i64,
    pub session_id: i64,
    pub training_template_id: i64,
    pub order_index: i32,
    pub custom_notes: Option<String>,       // Hybrid: per-session overrides
    pub template_data: Option<TrainingTemplate>, // Populated on read
}
```

#### 9.1.3 Repository Layer

```rust
// src/db/repositories/training_template_repo.rs
impl TrainingTemplateRepository {
    fn create_template(&self, template: &TrainingTemplate) -> Result<i64>;
    fn get_template_by_id(&self, id: i64) -> Result<TrainingTemplate>;
    fn list_all_templates(&self) -> Result<Vec<TrainingTemplate>>;
    fn list_templates_by_coach(&self, coach_id: i64) -> Result<Vec<TrainingTemplate>>;
    fn update_template(&self, template: &TrainingTemplate, edited_by: i64) -> Result<()>;
    fn delete_template(&self, id: i64) -> Result<()>;
    fn get_audit_info(&self, template_id: i64) -> Result<TemplateAuditInfo>;
}

// src/db/repositories/session_training_link_repo.rs
impl SessionTrainingLinkRepository {
    fn add_template_to_session(&self, session_id: i64, template_id: i64, order: i32) -> Result<()>;
    fn get_templates_for_session(&self, session_id: i64) -> Result<Vec<SessionTrainingLink>>;
    fn remove_template_from_session(&self, session_id: i64, template_id: i64) -> Result<()>;
    fn reorder_templates(&self, session_id: i64, new_order: Vec<i64>) -> Result<()>;
}
```

#### 9.1.4 UI Screen Hierarchy

```
┌──────────────────────────────────────────────────────────────┐
│                         HOME SCREEN                          │
├──────────────────────────────────────────────────────────────┤
│ > Help                                                       │
│   Manage Sessions / My Sessions                             │
│   Training Templates (NEW - Coach only)                    │
│                                                              │
│ Controls: ↑↓ Navigate | Enter Select | q Quit | ? Help     │
└──────────────────────────────────────────────────────────────┘
           │                    │
           │                    ├─────────────────┐
           │                    │                 │
           ▼                    ▼                 ▼
        [HELP]        [SESSIONS LIST]   [TRAINING TEMPLATES]
                           │                    │
                           ▼                    ├──► TEMPLATE DETAIL
                      [SESSION DETAIL]         │     ├─ View audit info
                           │                   │     ├─ Edit template
                           ├──► [TRAINING      │     └─ Delete
                           │     CONTENT PICKER]
                           │     - Browse all   ├──► CREATE TEMPLATE
                           │       templates    │
                           │     - Select &     └──► EDIT TEMPLATE
                           │       add to       
                           │       session
                           │
                           └──► [EDIT SESSION]
                                (now refs templates)

NEW SCREENS (Pseudo-code):
- TRAINING_TEMPLATES: List all templates with creator/editor info
- TRAINING_TEMPLATE_DETAIL: View single template, audit trail, options to edit/delete
- TRAINING_TEMPLATE_CREATE: Form to create new template
- TRAINING_TEMPLATE_EDIT: Form to edit existing template (updates created_by/last_edited_by)
- TRAINING_CONTENT_PICKER: Browse templates when adding to session (search, filter by type)
```

#### 9.1.5 Program Structure

```
src/
├── db/
│   └── repositories/
│       ├── training_template_repo.rs (NEW)
│       └── session_training_link_repo.rs (NEW)
├── models/
│   └── training_template.rs (NEW)
├── services/
│   └── template_service.rs (NEW)
│       ├── create_template()
│       ├── add_template_to_session()
│       ├── update_template_references()
│       └── migrate_legacy_content()
└── ui/
    ├── screens/
    │   ├── templates_list.rs (NEW)
    │   ├── template_detail.rs (NEW)
    │   ├── template_form.rs (NEW)
    │   ├── template_picker.rs (NEW)
    │   └── template_audit.rs (NEW)
    └── navigation.rs (MODIFIED - add new Screen variants)

tests/
├── unit/
│   ├── templates.rs (NEW)
│   └── session_training_links.rs (NEW)
└── integration/
    └── test_template_workflow.rs (NEW)
```

#### 9.1.6 Audit Tracking UI

When viewing a template, display:
```
┌─────────────────────────────────────────────────────┐
│ Training Template: "Backhand Drill Pack"             │
├─────────────────────────────────────────────────────┤
│                                                     │
│ Type: Drill              Duration: 20 minutes       │
│                                                     │
│ Created by: Coach Alice         Jan 15, 2026       │
│ Last edited by: Coach Alice     Jan 28, 2026       │
│                                                     │
│ Description:                                       │
│ Advanced backhand technique drills with footwork   │
│                                                     │
│ Used in: 12 sessions                               │
│                                                     │
│ [Edit] [Delete] [View Usage] [Back]               │
└─────────────────────────────────────────────────────┘
```

---

### 9.2 Cursor Position Memory (Session-Aware)

**Goal**: Remember user's selection position per screen context, providing smoother navigation experience.

**Confirmed Design Decisions:**
- **Memory Scope**: Session-aware (different position for each session context)
- **Persistence**: In-memory only (survives only current app session)
- **Behavior**: Auto-focus last selected item when navigating back to a previously visited screen

#### 9.2.1 Architecture

```rust
// src/app.rs - Screen context tracking

// New field in App struct:
pub struct App {
    // ... existing fields ...
    
    /// Tracks cursor position per screen + context
    /// Key format: "screen_type:context_id"
    /// Examples:
    ///   "session_list:global" -> position in coach's session list
    ///   "session_detail:123" -> position in session 123's content list
    ///   "templates:global" -> position in templates list
    pub screen_selection_history: HashMap<String, usize>,
}

impl App {
    /// Generate cache key based on current screen and context
    fn get_screen_key(&self) -> String {
        match self.current_screen {
            Screen::SessionList => "session_list:global".to_string(),
            Screen::SessionDetail(session_id) => format!("session_detail:{}", session_id),
            Screen::TrainingTemplates => "templates:global".to_string(),
            Screen::TemplateDetail(template_id) => format!("template_detail:{}", template_id),
            _ => "default:global".to_string(),
        }
    }
    
    /// Save current cursor position when leaving a screen
    fn save_cursor_position(&mut self) {
        let key = self.get_screen_key();
        self.screen_selection_history.insert(key, self.selected_index);
    }
    
    /// Restore cursor position when entering a screen
    fn restore_cursor_position(&mut self) {
        let key = self.get_screen_key();
        self.selected_index = self.screen_selection_history
            .get(&key)
            .copied()
            .unwrap_or(0);
    }
}
```

#### 9.2.2 Key Handler Integration

```rust
// src/ui/app_ui.rs - Updated key handlers

match key.code {
    KeyCode::Esc | KeyCode::Backspace => {
        // When navigating away, save current position
        self.save_cursor_position();
        
        if self.current_screen == Screen::Home {
            self.should_quit = true;
        } else {
            self.current_screen = Screen::Home;
            self.home_menu_selected_index = 0;
        }
    }
    
    KeyCode::Enter => {
        // When entering a detail screen, restore saved position later
        // Navigate to new screen
        self.current_screen = Screen::SessionDetail(session_id);
        
        // After screen change, restore will be called in the render loop
    }
}

// In handle_screen_transition():
fn handle_screen_transition(&mut self, new_screen: Screen) {
    self.save_cursor_position();  // Save current before leaving
    self.current_screen = new_screen;
    self.restore_cursor_position(); // Restore for new screen
}
```

#### 9.2.3 UI Behavior Examples

**Example 1: Session List Navigation**
```
1. View SessionList with 20 sessions, select item #7 (my session)
2. Press Enter → navigates to SessionDetail
3. View SessionDetail for 10 minutes
4. Press Esc → back to SessionList
   → Cursor automatically positioned at item #7 ✓
```

**Example 2: Template Detail with Session Context**
```
1. In SessionDetail #42, view training content
2. Open TRAINING_CONTENT_PICKER to browse 50 templates
3. Select and add template at position #23
4. Close picker → back to SessionDetail
   → Content list cursor at #23 (just added) ✓

5. Later, go back to SessionDetail #42
   → Content list shows cursor at #23 (remembered) ✓

6. Switch to SessionDetail #99 (different session)
   → Content list for #99 starts at position 0 (fresh context) ✓
```

#### 9.2.4 Implementation Structure

```
src/
├── app.rs (MODIFIED)
│   ├── Add screen_selection_history: HashMap<String, usize>
│   ├── Add get_screen_key() method
│   ├── Add save_cursor_position() method
│   ├── Add restore_cursor_position() method
│   └── Add handle_screen_transition() method
│
└── ui/
    └── app_ui.rs (MODIFIED)
        ├── Update Esc/Backspace handler to call save_cursor_position()
        ├── Update Enter handler to call handle_screen_transition()
        └── Update render loops to restore position on screen entry

tests/
└── unit/
    └── cursor_memory.rs (NEW)
        ├── test_session_list_cursor_restore()
        ├── test_context_aware_positions()
        ├── test_multiple_sessions_independent_positions()
        └── test_position_persists_through_cycle()
```

#### 9.2.5 Test Examples

```rust
// tests/unit/cursor_memory.rs

#[test]
fn test_session_list_cursor_restore() {
    let mut app = App::new(user_context, db_path);
    
    // Load sessions, select item 7
    app.load_sessions();
    app.selected_index = 7;
    app.save_cursor_position();
    
    // Navigate to detail
    app.current_screen = Screen::SessionDetail(123);
    app.restore_cursor_position();
    
    // Navigate back
    app.current_screen = Screen::SessionList;
    app.restore_cursor_position();
    
    assert_eq!(app.selected_index, 7); // ✓ Position restored
}

#[test]
fn test_context_aware_positions() {
    let mut app = App::new(user_context, db_path);
    
    // View SessionDetail #42, position at 5
    app.current_screen = Screen::SessionDetail(42);
    app.selected_index = 5;
    app.save_cursor_position();
    
    // View SessionDetail #99, position at 3
    app.current_screen = Screen::SessionDetail(99);
    app.selected_index = 3;
    app.save_cursor_position();
    
    // Back to SessionDetail #42
    app.current_screen = Screen::SessionDetail(42);
    app.restore_cursor_position();
    assert_eq!(app.selected_index, 5); // ✓ Different context remembers separately
    
    // To SessionDetail #99
    app.current_screen = Screen::SessionDetail(99);
    app.restore_cursor_position();
    assert_eq!(app.selected_index, 3); // ✓ Each session has own position
}
```

---

### 9.3 Future Feature: Session & Training Content Diff

**Status**: Planned (not implemented)  
**Goal**: Enable coaches to compare and track changes between sessions and their templates over time

**High-level Concept**:
- Show what's changed in a template since a session was created
- Highlight modifications to training content between sessions
- Display audit history of template edits with before/after diffs
- Help identify when to update sessions after template changes

**Example Use Case**:
```
Coach Alice created Session A using "Backhand Drill" template on Jan 10
Coach Bob edited "Backhand Drill" template on Jan 20 (added warm-up step)

When Coach Alice views Session A, show:
"⚠ This session uses 1 outdated template version
 Template 'Backhand Drill' was modified on Jan 20
 [View Changes] [Update Session] [Ignore]"
```

**Planned Implementation**:
- Add timestamp tracking to training_templates (created_at, updated_at)
- Add migration history table to track template version changes
- Create diff rendering UI showing before/after content
- Add comparison screen accessible from SessionDetail
- Optional auto-update capability for sessions

---

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
