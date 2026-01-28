# Phase 4 - Extended Coach Features Roadmap

**Current Status:** Phase 4A ✅ COMPLETE  
**Date:** January 27, 2026

---

## Overview

Phase 4 extends the MVP with advanced session management and content creation capabilities. Each phase is designed to be delivered independently so coaches get value incrementally.

---

## ✅ Phase 4A: Session Creation Form (COMPLETE)

**Status:** Live and tested  
**Delivery Date:** January 27, 2026  
**Test Coverage:** 13 unit tests, all passing

### What's Implemented
- Interactive form with 6 input fields
- Tab/Shift+Tab for field navigation
- Arrow keys for skill level cycling
- Real-time input validation:
  - Title: 3-100 characters
  - Description: 0-500 characters (optional)
  - Date: YYYY-MM-DD format
  - Time: HH:MM format
  - Duration: 5-480 minutes
  - Skill Level: beginner/intermediate/advanced
- Direct database save with error handling
- Return to session list with visual feedback

### User Workflow
```
Coach presses [c] on session list
  → Session creation form opens
  → Fields: Title, Description, Date, Time, Duration, Skill Level
  → [Tab] to move between fields
  → [Shift+Tab] to go back
  → [↑/↓] to cycle skill levels
  → [Backspace] to delete
  → [Enter] to save
  → Message: "Session created successfully" or error details
  → Returns to session list
```

### Files Created
- `src/ui/session_form.rs` (195 lines) - Form state and validation
- `tests/unit/session_form.rs` (218 lines) - 13 comprehensive tests

### Files Modified
- `src/ui/mod.rs` - Export SessionForm
- `src/ui/app_ui.rs` - Form rendering and integration
- `tests/unit/mod.rs` - Include session form tests

---

## 🚧 Phase 4B: Session Editing & Deletion (PLANNED)

**Estimated Duration:** 1 week  
**Complexity:** Medium  
**Dependencies:** Phase 4A (complete)

### What to Build

#### 1. Session Edit Form
- Pre-populate form with existing session data
- Same validation rules as creation form
- Mark unsaved changes visually
- Confirm before saving

```
Keyboard:
  [e] - Edit selected session
  [Enter] - Save changes
  [Esc] - Cancel without saving
```

#### 2. Session Deletion
- Confirmation dialog before deletion
- Show affected subscriptions count
- Soft delete option (archive) vs hard delete

```
Keyboard:
  [d] - Delete selected session
  [y] - Confirm deletion
  [n] - Cancel
```

#### 3. Subscription View
- Display count of subscribed players for each session
- Show list of subscribed players in detail view
- Optional: player filtering/search

### Database Changes
- Potentially add `archived_at` field for soft deletes
- No schema breaking changes

### Tests Needed
- Form pre-population validation
- Update/edit database operations
- Deletion cascade handling
- Confirmation dialog logic

### Implementation Steps
1. Create `src/ui/session_edit_form.rs` (similar to session_form.rs)
2. Extend `SessionRepository` with update operations
3. Add delete confirmation dialog UI
4. Update app_ui.rs with edit/delete key handlers
5. Add 8-10 integration tests for edit/delete operations
6. Update keyboard controls documentation

---

## 🚧 Phase 4C: Training Content Management (PLANNED)

**Estimated Duration:** 2 weeks  
**Complexity:** High  
**Dependencies:** Phase 4A, 4B recommended but not required

### What to Build

#### 1. Content Types Management
- Drills: Exercises with techniques
- Quizzes: Knowledge checks
- Homework: Practice assignments

#### 2. Drill Management
- Create drill form (name, description, technique, difficulty)
- Attach drills to sessions
- Players see drills in session detail
- Edit/delete drills (if not yet completed by players)

```
In Session Detail:
  [a] - Add content to session
  → Select content type (Drill/Quiz/Homework)
  → Select or create content
  → Confirm attachment
```

#### 3. Quiz System
- Create quiz form (title, questions)
- Question types: multiple choice, short answer
- Assign to sessions
- Players take quizzes, get immediate feedback
- Track scores

#### 4. Homework System
- Create homework form (title, description, due date)
- Attach to sessions
- Players submit responses
- Coach grades submissions
- Track completion status

### UI Changes
- New screens: Content Library, Quiz Taker, Homework Reviewer
- Session detail expanded to show content
- Homework submission tracker

### Database Changes
- Use existing `training_content`, `quizzes`, `homework`, `quiz_responses`, `homework_submissions` tables
- Add schema updates if needed (due dates, grading)

### Files to Create
- `src/models/drill.rs` - Drill model
- `src/models/quiz.rs` - Quiz model
- `src/models/question.rs` - Quiz question model
- `src/db/repositories/drill_repo.rs`
- `src/db/repositories/quiz_repo.rs`
- `src/db/repositories/homework_repo.rs`
- `src/ui/content_form.rs`
- `src/ui/quiz_screen.rs`
- `src/ui/homework_screen.rs`

### Tests Needed
- 30+ unit tests for content models and repos
- 10+ integration tests for content workflows
- Quiz logic and scoring
- Homework submission tracking

### Implementation Priority
1. Drill management (simplest, high value)
2. Homework system (medium complexity)
3. Quiz system (most complex, interactive features)

---

## 🚧 Phase 4D: Enhanced Player Features (PLANNED)

**Estimated Duration:** 1-2 weeks  
**Complexity:** Medium  
**Dependencies:** Phases 4A-4C would enhance this significantly

### What to Build

#### 1. Training History View
- Player page showing all past sessions
- Filter by date range, skill level
- Mark historical sessions as reviewed

#### 2. Progress Statistics
- Total sessions completed
- Skills practiced breakdown (pie chart)
- Time invested in training
- Completion rate over time

#### 3. Calendar View
- Month/week view of upcoming sessions
- Highlight subscribed sessions
- Show session details in calendar cells
- Quick subscribe/unsubscribe from calendar

#### 4. Personal Notes
- Add notes to sessions (personal reminders)
- Attach files/links to sessions
- Share notes with coach (optional)

#### 5. Session Reminders
- Set reminders for upcoming sessions
- Notifications (if TUI supports)
- Email reminders (if available)

### UI Changes
- New "Statistics" screen
- New "Calendar" view option
- "Notes" tab in session detail
- Reminder settings panel

### No Database Changes Required
- Notes can use existing `description` field or new `player_notes` column

### Tests Needed
- 15+ unit tests for calculations (progress, statistics)
- 5+ integration tests for filtering and views

### Implementation Priority
1. Training history view (essential, quick wins)
2. Progress statistics (visual impact, motivating)
3. Calendar view (nice to have, better UX)
4. Personal notes (convenience feature)
5. Reminders (last, least critical)

---

## Implementation Sequence Recommendation

```
Timeline:
Week 1-2:  Phase 4B (Edit/Delete) - Core functionality
Week 3-4:  Phase 4C (Content) - Drill management first
Week 5-6:  Phase 4C (Content) - Homework and quiz systems
Week 7-8:  Phase 4D (Player features) - History and stats
Week 9:    Phase 4D (Player features) - Calendar and notes
Week 10:   Polish, documentation, optimization
```

---

## Success Criteria

### Phase 4B Complete When:
- [ ] Sessions can be edited and deleted
- [ ] Coaches can see subscription counts
- [ ] All edit/delete operations tested (10+ tests)
- [ ] Keyboard controls updated
- [ ] No regressions in existing features

### Phase 4C Complete When:
- [ ] Drills fully manageable (CRUD)
- [ ] Quizzes functional with scoring
- [ ] Homework submission tracking works
- [ ] Content visible in session details
- [ ] 30+ content-related tests passing
- [ ] Players can interact with all content types

### Phase 4D Complete When:
- [ ] Training history viewable by players
- [ ] Progress statistics calculated and displayed
- [ ] Calendar view implemented
- [ ] Personal notes functional
- [ ] All player-facing features tested
- [ ] 20+ player feature tests passing

---

## Remaining Features Count

| Phase | Features | Est. Tests | Lines of Code |
|-------|----------|-----------|---------------|
| 4B    | 3        | 10+       | 300-400       |
| 4C    | 3 types  | 30+       | 800-1000      |
| 4D    | 5        | 20+       | 400-500       |
| **Total** | **11** | **60+** | **1500-1900** |

---

## Notes

- Each phase is independently valuable and can be deployed
- Phases 4B and 4C can be done in parallel
- Phase 4D depends on having content (benefits from 4C)
- All phases maintain MVP stability
- Test coverage target: 90% of new code
- Code review and documentation after each phase
