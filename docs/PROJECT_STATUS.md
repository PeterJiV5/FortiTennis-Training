# Tennis Training TUI App - Project Status

**Last Updated:** January 27, 2026  
**Current Version:** 0.2.0 (Phase 2 Complete)  
**Project Status:** ✅ On Track

---

## Executive Summary

A multi-user terminal-based (TUI) application for tennis training management built with Rust and Ratatui. The application allows coaches to create and manage training sessions while players can view and subscribe to sessions. Access is controlled via SSH with user-specific authentication.

**Target Scale:** 1 coach + ~10-18 players (max 20 users)  
**Deployment:** Local PC server with SSH access  
**Tech Stack:** Rust, Ratatui, SQLite, SSH (forced commands)

---

## Completed Phases

### ✅ Phase 1: Foundation (Complete)

**Deliverables:**
- [x] Rust project structure established
- [x] SQLite database with complete schema
- [x] Database migrations system
- [x] Data models (User, Session, Subscription, Training Content)
- [x] CLI argument parsing with `--user` flag
- [x] Basic TUI skeleton with Ratatui
- [x] User authentication via SSH forced commands
- [x] Role-based display (Coach vs Player)

**Files Created:**
```
tennis-tui/
├── Cargo.toml
├── data/
│   └── tennis.db (SQLite database)
├── src/
│   ├── main.rs
│   ├── auth/
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── db/
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   └── migrations.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── session.rs
│   ├── ui/
│   │   ├── mod.rs
│   │   └── app_ui.rs
│   └── utils/
│       ├── mod.rs
│       └── error.rs
```

**Key Features:**
- User context loaded from database based on SSH username
- Personalized welcome screens for coaches and players
- Clean error handling with custom error types
- Foreign key constraints enabled in SQLite

---

### ✅ Phase 2: Core Session Management (Complete)

**Deliverables:**
- [x] Session repository with CRUD operations
- [x] Session list view (role-specific)
- [x] Session detail view
- [x] Navigation system between screens
- [x] Keyboard controls (arrow keys, number keys, Enter, Esc)
- [x] Coach: View all created sessions
- [x] Player: View all available sessions
- [x] Session data loaded from database

**New Files:**
```
src/
├── db/
│   └── repositories/
│       ├── mod.rs
│       └── session_repo.rs
└── ui/
    └── navigation.rs
```

**Key Features:**
- Screen-based navigation (Home, SessionList, SessionDetail, SessionCreate)
- Interactive list with cursor selection
- Role-specific features (coaches can create sessions)
- Real-time database queries
- Empty state handling
- Contextual keyboard shortcuts

**Current Keyboard Controls:**
```
Home Screen:
  [1] - Navigate to Home
  [2] - Navigate to Sessions
  [q] - Quit application

Session List:
  ↑/↓ or j/k - Navigate list
  [Enter]     - View session details
  [c]         - Create session (coach only)
  [Esc]       - Back to home
  [q]         - Quit

Session Detail:
  [Esc] - Back to list
  [q]   - Quit
```

---

## Current Capabilities

### Coach Workflow
1. ✅ Log in via SSH as coach
2. ✅ View home screen with personalized greeting
3. ✅ Navigate to "Manage Sessions"
4. ✅ View list of all created sessions
5. ✅ Select and view session details
6. ✅ See session information (title, date, time, duration, skill level, description)
7. 🚧 Create new sessions (placeholder screen only)
8. ❌ Edit sessions
9. ❌ Delete sessions
10. ❌ Assign sessions to players

### Player Workflow
1. ✅ Log in via SSH as player
2. ✅ View home screen with skill level and goals
3. ✅ Navigate to "My Sessions"
4. ✅ View list of available sessions
5. ✅ Select and view session details
6. ❌ Subscribe to sessions
7. ❌ Mark sessions as complete
8. ❌ View training history

---

## Database Schema (Implemented)

### Tables Created
- ✅ `users` - User accounts (coach/player)
- ✅ `sessions` - Training sessions
- ✅ `training_content` - Drills, exercises, warmups, cooldowns
- ✅ `quizzes` - Knowledge check questions
- ✅ `homework` - Practice assignments
- ✅ `subscriptions` - Player enrollment in sessions
- ✅ `quiz_responses` - Player quiz answers
- ✅ `homework_submissions` - Homework completion tracking

### Indexes Created
- ✅ `idx_sessions_created_by`
- ✅ `idx_sessions_date`
- ✅ `idx_subscriptions_user`
- ✅ `idx_subscriptions_session`
- ✅ `idx_training_content_session`

---

## Code Quality

### Testing
- ✅ Unit tests for models (user roles, skill levels, enums)
- ✅ Integration tests for database operations
- ✅ Test coverage for SessionRepository CRUD operations
- ✅ Temporary database fixtures for testing

### Error Handling
- ✅ Custom `AppError` type with `From` implementations
- ✅ Result type alias for cleaner error propagation
- ✅ Proper error handling in database operations
- ✅ User-friendly error messages

### Code Organization
- ✅ Clean separation of concerns (models, db, ui, auth, utils)
- ✅ Repository pattern for database access
- ✅ Modular screen rendering
- ✅ Reusable components (header, footer)

---

## Known Issues

### Fixed Issues
- ✅ Typo in UserRole::from_str ("caoch" → "coach")
- ✅ Quit key not working (fixed with KeyEventKind filtering)
- ✅ Database datetime parsing (RFC3339 → SQLite format)
- ✅ Type mismatch in establish_connection (Path → str)
- ✅ CLI args requiring --user for --init-db

### Current Issues
- None reported

---

## Dependencies

```toml
[dependencies]
ratatui = "0.26"           # TUI framework
crossterm = "0.27"         # Terminal backend
rusqlite = "0.31"          # SQLite database
serde = "1.0"              # Serialization
serde_json = "1.0"         # JSON support
chrono = "0.4"             # Date/time handling
thiserror = "1.0"          # Error types
anyhow = "1.0"             # Error handling
clap = "4.5"               # CLI parsing
tracing = "0.1"            # Logging
tracing-subscriber = "0.3" # Log subscriber

[dev-dependencies]
tempfile = "3.10"          # Temporary files for tests
```

---

## How to Use (Current State)

### Initial Setup
```bash
# Clone and build
cargo build --release

# Initialize database with sample data
cargo run --release -- --init-db

# This creates:
# - Coach account (username: coach)
# - Two player accounts (alice, bob)
```

### Add Sample Sessions
```bash
sqlite3 data/tennis.db "INSERT INTO sessions (title, description, scheduled_date, scheduled_time, duration_minutes, skill_level, created_by, created_at, updated_at) VALUES ('Forehand Fundamentals', 'Practice basic forehand technique', '2026-02-01', '10:00:00', 90, 'beginner', 1, datetime('now'), datetime('now'));"

sqlite3 data/tennis.db "INSERT INTO sessions (title, description, scheduled_date, scheduled_time, duration_minutes, skill_level, created_by, created_at, updated_at) VALUES ('Serve Practice', 'Work on serve mechanics', '2026-02-03', '14:00:00', 60, 'intermediate', 1, datetime('now'), datetime('now'));"
```

### Run the Application
```bash
# As coach
cargo run --release -- --user=coach

# As player
cargo run --release -- --user=alice
```

### Run Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_create_session
```

---

## Next Steps (Phase 3)

### Phase 3: Subscription System (Planned)
**Goal:** Allow players to subscribe to sessions and track completion

**Features to Implement:**
- [ ] Player: Subscribe to sessions from session list
- [ ] Player: Unsubscribe from sessions
- [ ] Player: Mark sessions as completed
- [ ] Player: Filter sessions (My Sessions vs All Sessions)
- [ ] Coach: View subscription status for sessions
- [ ] Coach: See which players are enrolled
- [ ] Database: Subscription CRUD operations
- [ ] UI: Subscription status indicators
- [ ] UI: Completion status indicators

**Estimated Time:** 1-2 weeks

---

## Deployment Guide (For Phase 1-2)

### Server Setup
```bash
# Create system user
sudo useradd -m -s /bin/bash tennis-tui

# Create directory structure
sudo -u tennis-tui mkdir -p /home/tennis-tui/{bin,data,logs}

# Build and deploy
cargo build --release
scp target/release/tennis-tui user@server:/home/tennis-tui/bin/
ssh user@server "chmod +x /home/tennis-tui/bin/tennis-tui"
```

### SSH User Setup (Approach 3)
```bash
# On server, add to /home/tennis-tui/.ssh/authorized_keys:
command="/home/tennis-tui/bin/tennis-tui --user=alice",no-pty,no-port-forwarding ssh-ed25519 AAAA... alice
command="/home/tennis-tui/bin/tennis-tui --user=bob",no-pty,no-port-forwarding ssh-ed25519 AAAA... bob
```

### Users Connect
```bash
# Users connect via SSH with their key
ssh tennis-tui@yourserver.com

# TUI launches automatically
```

---

## Project Statistics

### Lines of Code (Approximate)
- Source code: ~1,500 lines
- Tests: ~400 lines
- Documentation: ~800 lines

### Files
- Source files: 15
- Test files: 4
- Documentation: 2

### Database
- Tables: 8
- Indexes: 5
- Sample users: 3 (1 coach, 2 players)

---

## Team & Contact

**Developer:** [Your Name]  
**Project Start:** January 26, 2026  
**Current Phase:** Phase 2 Complete  
**Next Milestone:** Phase 3 - Subscription System

---

## Resources

### Documentation
- [Main Project Documentation](PROJECT_DOCUMENTATION.md)
- [Ratatui Documentation](https://docs.rs/ratatui)
- [Rusqlite Documentation](https://docs.rs/rusqlite)

### Useful Commands
```bash
# View database
sqlite3 data/tennis.db

# Check schema
sqlite3 data/tennis.db ".schema"

# View users
sqlite3 data/tennis.db "SELECT * FROM users;"

# View sessions
sqlite3 data/tennis.db "SELECT * FROM sessions;"

# Format code
cargo fmt

# Lint code
cargo clippy

# Build documentation
cargo doc --open
```

---

## Change Log

### v0.2.0 - Phase 2 Complete (January 27, 2026)
- ✅ Added session repository with full CRUD
- ✅ Implemented session list view
- ✅ Implemented session detail view
- ✅ Added navigation system
- ✅ Added keyboard controls for navigation
- ✅ Role-based session filtering (coach sees own, player sees all)
- ✅ Empty state handling
- ✅ Integration tests for database operations

### v0.1.0 - Phase 1 Complete (January 26, 2026)
- ✅ Initial project setup
- ✅ Database schema and migrations
- ✅ User authentication via CLI
- ✅ Basic TUI with home screen
- ✅ Role-based display
- ✅ Sample data initialization

---

**Status:** Ready for Phase 3 development  
**Blockers:** None  
**Risk Level:** Low
