# Phase 5 Implementation Guide: Architecture Quick Reference

**Quick Access**: Jump directly to the details you need

---

## 1. Database Schema Overview

### New Tables (Phase 5)

```sql
training_templates
├─ id (PK)
├─ coach_id (FK → users)
├─ title
├─ content_type (drill|exercise|warmup|cooldown|quiz|homework)
├─ description
├─ duration_minutes
├─ created_by (FK → users)
├─ last_edited_by (FK → users, nullable)
├─ last_edited_at (datetime, nullable)
└─ is_public (boolean)

session_training_links
├─ id (PK)
├─ session_id (FK → sessions)
├─ training_template_id (FK → training_templates)
├─ order_index
└─ custom_notes (session-specific overrides)
```

### Migration Strategy

```
Legacy system (current):
  sessions → training_content (direct 1-to-many)

New system (Phase 5):
  sessions → session_training_links → training_templates
  
Migration path:
  1. Create training_templates with data from training_content
  2. Create session_training_links mapping sessions
  3. Update code to read from new tables
  4. Archive/drop old training_content table
```

---

## 2. Rust Model Structure

### File: `src/models/training_template.rs` (NEW)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingTemplate {
    pub id: i64,
    pub coach_id: i64,                      // Who the template belongs to
    pub title: String,
    pub content_type: ContentType,          // Reuse existing enum
    pub description: Option<String>,
    pub duration_minutes: Option<i32>,
    pub created_at: String,
    pub created_by: i64,                    // User ID who created
    pub last_edited_by: Option<i64>,        // User ID who last edited
    pub last_edited_at: Option<String>,
    pub is_public: bool,                    // Share across all
}

#[derive(Debug, Clone)]
pub struct TemplateAuditInfo {
    pub created_by_name: String,            // Coach display name
    pub last_edited_by_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTrainingLink {
    pub id: i64,
    pub session_id: i64,
    pub training_template_id: i64,
    pub order_index: i32,
    pub custom_notes: Option<String>,       // Hybrid: overrides
    pub template_data: Option<TrainingTemplate>, // Populated on fetch
}
```

### Update: `src/models/mod.rs`

```rust
// Add exports
pub use training_template::{TrainingTemplate, TemplateAuditInfo, SessionTrainingLink};
```

---

## 3. Repository Layer

### File: `src/db/repositories/training_template_repo.rs` (NEW)

```rust
pub struct TrainingTemplateRepository;

impl TrainingTemplateRepository {
    pub fn create(&self, template: TrainingTemplate, user_id: i64) -> Result<i64>;
    pub fn get_by_id(&self, id: i64) -> Result<TrainingTemplate>;
    pub fn list_all(&self) -> Result<Vec<TrainingTemplate>>;
    pub fn list_by_coach(&self, coach_id: i64) -> Result<Vec<TrainingTemplate>>;
    pub fn update(&self, template: &TrainingTemplate, edited_by: i64) -> Result<()>;
    pub fn delete(&self, id: i64) -> Result<()>;
    pub fn get_audit_info(&self, id: i64) -> Result<TemplateAuditInfo>;
    pub fn get_usage_count(&self, id: i64) -> Result<i64>; // How many sessions use it
}
```

**Key Methods Explained:**

| Method | Purpose | Sets |
|--------|---------|------|
| `create()` | New template | Sets `created_by=user_id`, `created_at=now` |
| `update()` | Edit existing | Sets `last_edited_by=user_id`, `last_edited_at=now` |
| `get_audit_info()` | Creator/editor names | Joins with users table |
| `get_usage_count()` | See reuse | Counts session_training_links |

### File: `src/db/repositories/session_training_link_repo.rs` (NEW)

```rust
pub struct SessionTrainingLinkRepository;

impl SessionTrainingLinkRepository {
    pub fn add_to_session(&self, session_id: i64, template_id: i64, order: i32) 
        -> Result<i64>; // Returns link ID
    pub fn get_for_session(&self, session_id: i64) -> Result<Vec<SessionTrainingLink>>;
    pub fn remove_from_session(&self, session_id: i64, template_id: i64) -> Result<()>;
    pub fn reorder_in_session(&self, session_id: i64, new_order: Vec<i64>) 
        -> Result<()>; // Reorder templates
    pub fn update_custom_notes(&self, link_id: i64, notes: &str) -> Result<()>;
}
```

---

## 4. UI Screen Variants

### Update: `src/ui/navigation.rs`

```rust
pub enum Screen {
    // Existing
    Home,
    Help,
    SessionList,
    SessionDetail(i64),
    SessionCreate,
    SessionEdit(i64),
    SessionDelete(i64),
    TrainingContentManagement,
    
    // NEW PHASE 5
    TrainingTemplates,
    TemplateDetail(i64),
    TemplateCreate,
    TemplateEdit(i64),
    TemplateDelete(i64),
    TrainingContentPicker,  // Browse templates to add
}
```

---

## 5. UI Screens to Create

### Screen 1: `TrainingTemplates` (List)
- Shows all templates with columns:
  - Title | Type | Duration | Created By | Updated
  - Highlight: Last edited date
- Keyboard: ↑↓ select, Enter detail, c create, e edit, d delete
- Coach only (menu item on Home)

### Screen 2: `TemplateDetail` 
- Shows template info:
  - Title, Type, Description, Duration
  - **Audit Block:**
    - Created by: [Name] on [Date]
    - Last edited by: [Name] on [Date]
  - Used in: [N] sessions (clickable to see which ones)
- Buttons: [Edit] [Delete] [Close]

### Screen 3: `TemplateCreate` / `TemplateEdit`
- Form fields:
  - Title (text input)
  - Content Type (dropdown)
  - Description (multiline)
  - Duration (number input)
- Auto-fills: created_by/created_at on create
- Auto-updates: last_edited_by/last_edited_at on save

### Screen 4: `TrainingContentPicker`
- Browse all templates when adding to session
- Search/filter by type
- Select template → add to session with order
- Can add custom notes for that session

---

## 6. Cursor Memory Implementation

### Update: `src/app.rs`

```rust
pub struct App {
    // ... existing fields ...
    
    // NEW: Session-aware cursor tracking
    pub screen_selection_history: HashMap<String, usize>,
}

impl App {
    /// Generate cache key from current screen
    fn get_screen_key(&self) -> String {
        match self.current_screen {
            Screen::SessionList => "session_list:global".to_string(),
            Screen::SessionDetail(id) => format!("session_detail:{}", id),
            Screen::TrainingTemplates => "templates:global".to_string(),
            _ => "default".to_string(),
        }
    }
    
    /// Before leaving screen: save position
    pub fn save_cursor_position(&mut self) {
        let key = self.get_screen_key();
        self.screen_selection_history.insert(key, self.selected_index);
    }
    
    /// After entering screen: restore position
    pub fn restore_cursor_position(&mut self) {
        let key = self.get_screen_key();
        self.selected_index = self.screen_selection_history.get(&key).copied().unwrap_or(0);
    }
}
```

### Update: `src/ui/app_ui.rs`

```rust
// In handle_key_event()

KeyCode::Esc | KeyCode::Backspace => {
    self.save_cursor_position();  // NEW: Save before leaving
    if self.current_screen == Screen::Home {
        self.should_quit = true;
    } else {
        self.current_screen = Screen::Home;
    }
}

KeyCode::Enter => {
    match self.current_screen {
        Screen::TrainingTemplates => {
            self.save_cursor_position();     // NEW: Save before detail
            let template_id = templates[self.selected_index].id;
            self.current_screen = Screen::TemplateDetail(template_id);
            self.restore_cursor_position();  // NEW: Restore in detail
        }
        _ => { /* existing logic */ }
    }
}
```

---

## 7. Key Implementation Order

### Week 1: Foundation
1. ✅ Database schema & migrations
2. ✅ Models (TrainingTemplate, SessionTrainingLink)
3. ✅ Repositories (template & link operations)
4. ✅ Basic unit tests

### Week 2: UI & Integration
5. ✅ Templates list screen
6. ✅ Template detail screen
7. ✅ Template create/edit forms
8. ✅ Navigation integration
9. ✅ Cursor memory implementation
10. ✅ Integration tests

### Week 3: Polish & Testing
11. ✅ Template picker (for sessions)
12. ✅ Data migration (training_content → templates)
13. ✅ Comprehensive testing
14. ✅ Documentation & examples
15. ✅ Git commits & release

---

## 8. Test File Organization

```
tests/
├── unit/
│   ├── templates/
│   │   ├── test_creation.rs
│   │   ├── test_audit.rs
│   │   ├── test_linking.rs
│   │   └── test_usage_tracking.rs
│   ├── cursor_memory/
│   │   ├── test_restore.rs
│   │   ├── test_context_aware.rs
│   │   └── test_multiple_sessions.rs
│   └── other/ (existing)
│
└── integration/
    ├── test_template_workflow.rs (create → use in session → view)
    ├── test_cursor_workflow.rs (navigate → remember → restore)
    └── other/ (existing)
```

---

## 9. Audit Tracking Examples

### Example 1: Template Creation
```
create_template("Backhand Drill", coach_alice)
  ↓
INSERT training_templates VALUES (
  id=42,
  coach_id=1,
  title="Backhand Drill",
  created_by=1,         ← Alice
  created_at="2026-01-29T10:00:00",
  last_edited_by=NULL,
  is_public=1
)
```

### Example 2: Template Edit
```
update_template(template_id=42, edited_by_coach_bob)
  ↓
UPDATE training_templates SET
  title="Backhand Drill - Advanced",
  last_edited_by=2,     ← Bob (different person!)
  last_edited_at="2026-02-15T14:30:00"
WHERE id=42
```

### Example 3: UI Display
```
Template: "Backhand Drill - Advanced"
Type: Drill | Duration: 25 min | Used in: 12 sessions

Created by: Alice Johnson       Jan 29, 2026
Last edited by: Bob Smith       Feb 15, 2026
                 ↑ Different coach! Shows evolution
```

---

## 10. Cursor Memory Examples

### Scenario 1: SessionList Navigation
```
1. Load SessionList (10 sessions)
   app.selected_index = 0

2. Select session #7
   app.selected_index = 7
   app.save_cursor_position()  // "session_list:global" → 7

3. Press Enter → SessionDetail(7)
   app.current_screen = SessionDetail(7)
   app.restore_cursor_position()  // No history yet, stays 0

4. Make changes...

5. Press Esc → back to SessionList
   app.save_cursor_position()  // "session_detail:7" → 0
   app.current_screen = SessionList
   app.restore_cursor_position()  // "session_list:global" → 7 ✓ RESTORED!
```

### Scenario 2: Context-Aware (Different Sessions)
```
1. SessionDetail(42) → select item #5
   app.save_cursor_position()  // "session_detail:42" → 5

2. Go back, navigate to SessionDetail(99) → select item #3
   app.save_cursor_position()  // "session_detail:99" → 3

3. Go back, navigate back to SessionDetail(42)
   app.restore_cursor_position()  // "session_detail:42" → 5 ✓
   (Session 42 remembers its position separately from Session 99)
```

---

## 11. Migration Strategy (Data)

```rust
// Run this once during database migration
fn migrate_training_content_to_templates() -> Result<()> {
    // For each existing training_content record:
    // 1. Create training_template (assume created by original session's creator)
    // 2. Create session_training_link mapping
    
    let conn = establish_connection();
    let old_content = fetch_all_training_content(&conn)?;
    
    for content in old_content {
        let session = fetch_session(&conn, content.session_id)?;
        
        let template = TrainingTemplate {
            coach_id: session.coach_id,
            title: content.title.clone(),
            content_type: content.content_type,
            description: content.description,
            duration_minutes: content.duration_minutes,
            created_by: session.created_by,
            created_at: session.created_at.clone(),
            last_edited_by: None,
            is_public: true,  // All templates are public
        };
        
        let template_id = create_template(&conn, &template)?;
        
        create_session_training_link(&conn, 
            content.session_id, 
            template_id, 
            content.order_index)?;
    }
    
    // After successful migration:
    // DROP TABLE training_content;
    
    Ok(())
}
```

---

## Quick Command Reference

```bash
# Check current line count (before Phase 5)
wc -l src/**/*.rs

# After implementing (rough estimate)
# +500 lines for templates (repos, models, screens)
# +100 lines for cursor memory (app.rs, app_ui.rs)
# +300 lines for tests

# Run tests with new structure
cargo test --lib templates
cargo test --lib cursor_memory
cargo test --test test_template_workflow

# Check test coverage
cargo tarpaulin --out Html
```

---

## Common Pitfalls to Avoid

| Pitfall | Solution |
|---------|----------|
| Forgetting to update `last_edited_by` on edit | Always pass `user_id` to update() function |
| Not setting order_index when creating links | Make order_index required param, validate uniqueness |
| Cursor memory interfering with other state | Use full screen key including context (session_detail:123 not just session_detail) |
| Losing position on certain transitions | Call save/restore on EVERY screen change |
| Test isolation issues | Each test should create fresh template with unique IDs |

---

**Quick Ref Version**: 1.0  
**For Details**: See [DESIGN.md](DESIGN.md#9-training-templates--cursor-memory-planned-features)  
**Detailed Plan**: See [PHASE_5_ROADMAP.md](PHASE_5_ROADMAP.md)
