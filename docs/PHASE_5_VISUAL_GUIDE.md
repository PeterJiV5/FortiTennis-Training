# Phase 5 Visual Architecture Diagrams

Quick visual reference for Phase 5 design

---

## 1. Training Templates Data Flow

### Current System (Before Phase 5)
```
Coach creates Session
  └─→ Session has Training Content (direct entries)
      └─→ Player views sessions
          └─→ Sees training content

PROBLEM: Content duplicated per session
If "Backhand Drill" is in 50 sessions, stored 50 times
Updating one affects only that session
```

### New System (Phase 5)
```
Coach creates Template
  ├─→ Global Template Library
  │    └─→ "Backhand Drill" created once
  │        ├─ Created by: Coach Alice
  │        ├─ Last edited by: Coach Alice
  │        └─ Used in: 50 sessions

Coach creates Session
  ├─→ References Templates (not duplicate)
  │    └─→ Session #5 uses Template #1 + custom notes
  │    └─→ Session #42 uses Template #1 (same template)
  │
  └─→ Player views Session
      └─→ Sees Template data (reads from single source)
```

---

## 2. Screen Navigation Hierarchy (Phase 5 Addition)

### Current (Phase 4)
```
HOME
├─ Help
├─ Manage Sessions / My Sessions
│   └─ Session Detail
│       ├─ Edit Session
│       ├─ Delete Session
│       └─ Training Content Management
```

### Phase 5: With Templates
```
HOME
├─ Help
├─ Manage Sessions / My Sessions (existing)
│   └─ Session Detail
│       ├─ Edit Session
│       ├─ Delete Session
│       ├─ Training Content Management
│       │   └─ Training Content Picker (NEW)
│       │       └─ Browse Templates
│       │           └─ Add to Session
├─ Training Templates (NEW - Coach only)
│   ├─ Templates List
│   │   └─ Templates Detail
│   │       ├─ Edit Template
│   │       ├─ Delete Template
│   │       └─ View Usage (which sessions use it?)
│   └─ Create Template (NEW)
```

---

## 3. Database Schema Diagram

### Tables Created (Phase 5)

```
EXISTING: sessions
├─ id (PK)
├─ title
├─ created_by
└─ ... other fields

            ↓ REFERENCES (many-to-many via link table)
            
NEW: training_templates
├─ id (PK)
├─ coach_id
├─ title
├─ content_type (drill|exercise|warmup|etc)
├─ created_by (FK → users)
├─ created_at
├─ last_edited_by (FK → users, nullable)
├─ last_edited_at (nullable)
└─ is_public (boolean)

            ↑ REFERENCED BY
            
NEW: session_training_links (junction table)
├─ id (PK)
├─ session_id (FK → sessions)
├─ training_template_id (FK → training_templates)
├─ order_index (position in session)
└─ custom_notes (session-specific overrides)
```

**Key Relationship:**
- One session → many training links
- One template → many training links
- Many sessions can share the same templates

---

## 4. Cursor Memory: Context Keys

### How Context Keys Work

```
Screen Type + Context ID = Storage Key

Examples:
┌─────────────────────────────────────────────┐
│ Screen::SessionList                         │
│ Key: "session_list:global"                  │
│ Position stored: #7 (which session selected)│
└─────────────────────────────────────────────┘

┌─────────────────────────────────────────────┐
│ Screen::SessionDetail(42)                   │
│ Key: "session_detail:42"                    │
│ Position stored: #3 (item in that session)  │
└─────────────────────────────────────────────┘

┌─────────────────────────────────────────────┐
│ Screen::TrainingTemplates                   │
│ Key: "templates:global"                     │
│ Position stored: #5 (which template sel.)   │
└─────────────────────────────────────────────┘
```

### Cursor Memory: Navigation Flow

```
User Journey:
┌────────────────────────────────────────┐
│ 1. In SessionList                      │
│    selected_index = 7                  │
│    ↓ save_cursor_position()            │
│    HashMap: ["session_list:global"→7]  │
└────────────────────────────────────────┘
              ↓ Enter → SessionDetail(42)
┌────────────────────────────────────────┐
│ 2. In SessionDetail                    │
│    selected_index = 3                  │
│    ↓ save_cursor_position()            │
│    HashMap: [..., "session_detail:42"→3]
└────────────────────────────────────────┘
              ↓ Esc → SessionList
┌────────────────────────────────────────┐
│ 3. Back to SessionList                 │
│    ↓ restore_cursor_position()         │
│    selected_index = 7 ✓ RESTORED!      │
│    HashMap: ["session_list:global"→7]  │
└────────────────────────────────────────┘
```

---

## 5. Audit Tracking: Who Created/Edited

### Display Format

```
Template: "Backhand Drill Pack"
├─ Type: Drill
├─ Duration: 25 minutes
├─ Used in: 12 sessions
│
├─ AUDIT TRAIL:
│  ├─ Created by: Alice Johnson       Jan 15, 2026 10:00 AM
│  └─ Last edited by: Bob Smith       Jan 28, 2026 2:30 PM
│
└─ (Different coaches shown = visible evolution)

When viewing this template, coaches see:
  ✓ Who created it originally
  ✓ Who last modified it (tracks all changes)
  ✓ When each change happened
```

### Template Edit Flow (Audit Update)

```
Coach Alice created "Backhand Drill" on Jan 15
  ├─ created_by: 1 (Alice's user_id)
  ├─ created_at: "2026-01-15T10:00:00"
  ├─ last_edited_by: NULL
  └─ last_edited_at: NULL

Coach Bob edits it on Jan 28
  ├─ created_by: 1 (unchanged - still Alice)
  ├─ created_at: "2026-01-15T10:00:00" (unchanged)
  ├─ last_edited_by: 2 (Bob's user_id)
  └─ last_edited_at: "2026-01-28T14:30:00"

Coach Charlie edits it on Jan 29
  ├─ created_by: 1 (still Alice - original creator)
  ├─ created_at: "2026-01-15T10:00:00" (still original)
  ├─ last_edited_by: 3 (Charlie's user_id)
  └─ last_edited_at: "2026-01-29T09:15:00"
```

---

## 6. Hybrid Binding: Template + Session Customization

### Scenario: Using Template in Multiple Sessions

```
Template Created:
┌─────────────────────────────────┐
│ ID: 1                           │
│ Title: "Backhand Drill"         │
│ Duration: 20 minutes           │
│ Type: Drill                     │
│ Description: "Basic technique"  │
└─────────────────────────────────┘
         ↓ Referenced by multiple sessions

Session #5 (Jan 10)
├─ Uses Template #1
├─ custom_notes: null (uses template as-is)
└─ Training Content Picker shows exact template

Session #42 (Jan 15)
├─ Uses Template #1
├─ custom_notes: "Focus on footwork"
└─ Training Content Picker shows template + note

Session #99 (Jan 20)
├─ Uses Template #1
├─ custom_notes: "Advanced - faster pace"
└─ Training Content Picker shows template + different note

BENEFIT: If template gets updated, all sessions see new version
BUT: Each session can add its own context via custom_notes
```

---

## 7. Testing Organization: Folder Structure

### Phase 5 Test Layout

```
tests/
├── unit/
│   ├── templates/                       (NEW)
│   │   ├── test_template_creation.rs
│   │   ├── test_template_audit.rs
│   │   ├── test_template_update.rs
│   │   ├── test_session_template_linking.rs
│   │   ├── test_template_picker.rs
│   │   └── test_template_usage_tracking.rs
│   │
│   ├── cursor_memory/                   (NEW)
│   │   ├── test_cursor_restore.rs
│   │   ├── test_context_aware.rs
│   │   ├── test_multiple_sessions.rs
│   │   └── test_navigation_flow.rs
│   │
│   ├── auth.rs (existing)
│   ├── db_connection.rs (existing)
│   ├── models.rs (existing)
│   └── ... others
│
├── integration/
│   ├── test_template_workflow.rs        (NEW)
│   │   ├── Create template
│   │   ├── Add to session
│   │   ├── View in UI
│   │   └── Modify template + check updates
│   │
│   ├── test_cursor_workflow.rs          (NEW)
│   │   ├── Navigate to multiple sessions
│   │   ├── Change selections
│   │   ├── Navigate back
│   │   └─ Verify positions restored
│   │
│   └── test_subscriptions.rs (existing)
```

---

## 8. Implementation Timeline: Visual Gantt

### Phase 5: 2-3 Week Timeline

```
WEEK 1 (Foundation)
│ ├─ Mon-Tue:   Database schema & migrations
│ ├─ Tue-Wed:   Models (TrainingTemplate, etc)
│ ├─ Wed-Thu:   Repositories (CRUD methods)
│ └─ Thu-Fri:   Unit tests for foundation
│
WEEK 2 (UI & Integration)
│ ├─ Mon-Tue:   Templates list screen
│ ├─ Tue-Wed:   Template detail & forms
│ ├─ Wed-Thu:   Navigation + key handlers
│ ├─ Thu-Fri:   Cursor memory implementation
│ └─ Fri:       Integration tests
│
WEEK 3 (Polish & Release)
│ ├─ Mon-Tue:   Template picker (for sessions)
│ ├─ Tue-Wed:   Data migration (legacy→templates)
│ ├─ Wed:       Comprehensive testing
│ ├─ Thu:       Documentation & examples
│ └─ Fri:       Git commits, ready for next phase
│
PARALLEL:
  Cursor memory can be done during template UI work
  (Independent feature, no dependencies)
```

---

## 9. Database Migration Process

### From Old to New (One-time Migration)

```
BEFORE: training_content table
┌──────────────┬────────────┬─────────┐
│ id           │ session_id │ title   │
├──────────────┼────────────┼─────────┤
│ 1            │ 5          │ Backhand│
│ 2            │ 5          │ Forehand│
│ 3            │ 42         │ Backhand│ ← duplicate
│ 4            │ 42         │ Serve   │
│ 5            │ 99         │ Backhand│ ← duplicate
└──────────────┴────────────┴─────────┘

MIGRATION PROCESS:
1. Create training_templates from unique content
2. Map training_content → templates
3. Create session_training_links to sessions
4. Archive old training_content

AFTER: New structure (no duplicates)
training_templates:
│ 1 │ Backhand drill (created by Alice)
│ 2 │ Forehand drill
│ 3 │ Serve technique

session_training_links:
│ session_5 → template_1 │
│ session_5 → template_2 │
│ session_42 → template_1 │ ← same template!
│ session_42 → template_3 │
│ session_99 → template_1 │ ← same template!

BENEFIT: Backhand drill stored once, used in 3 sessions
         Update it once = affects all 3 sessions
```

---

## 10. Cursor Memory: Multi-Session Example

### Walking Through: Different Sessions Retain Positions

```
START
app.screen_selection_history = {}

─────────────────────────────────────

STEP 1: In SessionList, select session #7
  app.current_screen = Screen::SessionList
  app.selected_index = 7
  app.save_cursor_position()
  
  screen_selection_history = {
    "session_list:global" → 7
  }

─────────────────────────────────────

STEP 2: Enter SessionDetail(5), select item #3
  app.current_screen = Screen::SessionDetail(5)
  app.selected_index = 3
  app.save_cursor_position()
  
  screen_selection_history = {
    "session_list:global" → 7,
    "session_detail:5" → 3
  }

─────────────────────────────────────

STEP 3: Go back to SessionList (restore #7)
  app.current_screen = Screen::SessionList
  app.restore_cursor_position()
  app.selected_index = 7 ✓ RESTORED
  
  screen_selection_history = {
    "session_list:global" → 7,
    "session_detail:5" → 3
  }

─────────────────────────────────────

STEP 4: Enter SessionDetail(99), select item #5
  app.current_screen = Screen::SessionDetail(99)
  app.selected_index = 5
  app.save_cursor_position()
  
  screen_selection_history = {
    "session_list:global" → 7,
    "session_detail:5" → 3,
    "session_detail:99" → 5
  }

─────────────────────────────────────

STEP 5: Back to SessionDetail(5) (restore #3)
  app.current_screen = Screen::SessionDetail(5)
  app.restore_cursor_position()
  app.selected_index = 3 ✓ RESTORED (not 5!)
  
  Different sessions have independent positions!

─────────────────────────────────────

STEP 6: Back to SessionList (restore #7)
  app.current_screen = Screen::SessionList
  app.restore_cursor_position()
  app.selected_index = 7 ✓ STILL RESTORED
```

---

## Summary Table: Phase 5 Features

| Feature | What It Solves | How It Works | Benefit |
|---------|---|---|---|
| **Templates** | Content duplication | Create once, use many | Update template = affects all sessions |
| **Audit Tracking** | Who changed what | creator/editor fields | Track evolution, accountability |
| **Hybrid Binding** | Rigid templates | Reference + custom notes | Flexibility per session |
| **Cursor Memory** | Lost position | HashMap per context | Smooth navigation UX |

---

**Document Version**: 1.0  
**Purpose**: Visual reference for Phase 5 design  
**For Details**: See [DESIGN.md Section 9](docs/DESIGN.md#9-training-templates--cursor-memory-planned-features)
