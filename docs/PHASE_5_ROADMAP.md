# Phase 5: Training Templates & Cursor Memory Roadmap

**Status**: Designed, Not Started  
**Target Start**: After Phase 4C (Menu navigation) completes  
**Estimated Duration**: 2-3 weeks  
**Created**: January 29, 2026

---

## Overview

This phase addresses two major UX improvements:

1. **Training Templates**: Enable coaches to create reusable training content libraries, reducing duplication when multiple sessions use the same drills/exercises
2. **Cursor Memory**: Remember user's selection position per screen context for smoother navigation

Both features have been fully designed and documented in [DESIGN.md Section 9](DESIGN.md#9-training-templates--cursor-memory-planned-features).

---

## Feature 1: Training Templates System

### Why This Matters

**Current Problem:**
- Training content is duplicated across sessions
- A "Backhand Drill" might be manually entered 50 times
- Updating content across sessions is error-prone

**Solution:**
- Global template library (created once, used many times)
- Audit tracking (see who created/edited)
- Flexible binding (reference templates, optionally override per-session)

### Architecture Decisions (CONFIRMED)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Migration Strategy | Option A: Full migration | Cleaner codebase, templates as primary source |
| Template Scope | Global + Audit | All coaches see templates, with creator/editor tracking |
| Session Binding | Hybrid | Sessions reference templates; can add custom notes |
| Test Organization | Separate folders | `/tests/unit/templates/` for better organization |

### Detailed Design

See [DESIGN.md Section 9.1](DESIGN.md#911-training-templates-architecture) for:
- Complete database schema
- Data models (`TrainingTemplate`, `SessionTrainingLink`)
- Repository interfaces
- Full UI screen hierarchy
- Program structure
- Audit tracking UI mockup

### Implementation Phases

#### Phase 5.1a: Database & Models (1-2 days)
```
Priority: CRITICAL - foundation for everything else

Tasks:
  [ ] Create migration: training_templates table
  [ ] Create migration: session_training_links table
  [ ] Create models/training_template.rs
  [ ] Create models/training_template_audit.rs
  [ ] Add to models/mod.rs exports
  [ ] Create integration tests for schema
```

#### Phase 5.1b: Repositories (1-2 days)
```
Priority: HIGH - data access layer

Tasks:
  [ ] Create db/repositories/training_template_repo.rs
  [ ] Implement: create, read, update, delete, list operations
  [ ] Create db/repositories/session_training_link_repo.rs
  [ ] Implement: add_to_session, remove_from_session, get_for_session
  [ ] Add audit timestamp tracking
  [ ] Unit tests in tests/unit/templates/
```

#### Phase 5.1c: Services & Logic (2-3 days)
```
Priority: HIGH - business logic

Tasks:
  [ ] Create services/template_service.rs
  [ ] Implement template CRUD operations
  [ ] Implement session-template linking
  [ ] Create migration function: legacy training_content → templates
  [ ] Add template usage tracking (which sessions use which templates)
  [ ] Unit tests in tests/unit/templates/
```

#### Phase 5.1d: UI Screens (3-4 days)
```
Priority: HIGH - user interaction

New screens:
  [ ] TRAINING_TEMPLATES: List all templates (coach only from Home)
  [ ] TEMPLATE_DETAIL: View template with audit info
  [ ] TEMPLATE_CREATE: Form to create new template
  [ ] TEMPLATE_EDIT: Form to edit existing template
  [ ] TRAINING_CONTENT_PICKER: Browse templates when adding to session
  [ ] Update TRAINING_CONTENT_MANAGEMENT: Use templates instead of direct content

Tasks:
  [ ] Create ui/screens/templates_list.rs
  [ ] Create ui/screens/template_detail.rs
  [ ] Create ui/screens/template_form.rs
  [ ] Create ui/screens/template_picker.rs
  [ ] Update ui/navigation.rs with new Screen variants
  [ ] Update ui/app_ui.rs with new key handlers (template management)
  [ ] Render methods for audit info display
```

#### Phase 5.1e: Integration & Migration (1-2 days)
```
Priority: CRITICAL - make it work end-to-end

Tasks:
  [ ] Add template screens to render dispatch
  [ ] Implement key handlers for template navigation
  [ ] Create migration: transfer training_content → templates
  [ ] Test full workflow: create template → use in session → view
  [ ] Handle backward compatibility if needed
  [ ] Integration tests in tests/integration/
```

### Testing Strategy

```
tests/unit/templates/
├── test_template_creation.rs
├── test_template_update.rs
├── test_template_audit.rs
├── test_session_template_linking.rs
└── test_template_picker.rs

tests/integration/
└── test_template_workflow.rs (full workflow: create template → session → view)

Coverage Target: 85%+
```

### Key Behaviors to Test

```rust
// Template creation with audit
let template = create_template("Backhand Drill", coach_id, user_id);
assert_eq!(template.created_by, user_id);
assert_eq!(template.coach_id, coach_id);

// Template editing updates last_edited_by
update_template(&template, different_user_id);
template = fetch_template(template.id);
assert_eq!(template.last_edited_by, Some(different_user_id));

// Session can reference multiple templates
add_template_to_session(session_id, template1_id, order=0);
add_template_to_session(session_id, template2_id, order=1);
let links = get_templates_for_session(session_id);
assert_eq!(links.len(), 2);

// Hybrid: template with session-specific notes
add_with_notes(session_id, template_id, "Focus on footwork");
link = get_link(session_id, template_id);
assert_eq!(link.custom_notes, Some("Focus on footwork"));
```

---

## Feature 2: Cursor Position Memory

### Why This Matters

**Current Problem:**
- Select item #7 in SessionList
- Navigate to SessionDetail
- Come back to SessionList
- Cursor is at position #0 (lost your place)

**Solution:**
- Remember selection per screen + context
- Auto-restore when navigating back
- Different positions for different sessions (context-aware)

### Architecture Decisions (CONFIRMED)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Memory Scope | Session-aware | Different position per session context |
| Persistence | In-memory only | No file I/O, lost on app exit (simpler) |
| Implementation | HashMap with string keys | "session_list:global", "session_detail:123" |

### Implementation (1-2 days, can do in parallel with Templates)

#### Phase 5.2a: Core Implementation (1 day)
```
Tasks:
  [ ] Add screen_selection_history: HashMap<String, usize> to App struct
  [ ] Implement get_screen_key() method
  [ ] Implement save_cursor_position() method
  [ ] Implement restore_cursor_position() method
  [ ] Implement handle_screen_transition() method
```

#### Phase 5.2b: Integration (1 day)
```
Tasks:
  [ ] Update Esc/Backspace handler to save position
  [ ] Update Enter handler to trigger transitions
  [ ] Update all screen navigation to call handlers
  [ ] Test with multiple sessions
```

### Testing Strategy

```
tests/unit/cursor_memory/
├── test_session_list_restore.rs
├── test_context_aware_positions.rs
├── test_multiple_sessions_independent.rs
└── test_position_persists_through_navigation.rs
```

### Key Behaviors to Test

```rust
// Save and restore within same context
app.current_screen = Screen::SessionList;
app.selected_index = 7;
app.save_cursor_position();
app.selected_index = 0;
app.restore_cursor_position();
assert_eq!(app.selected_index, 7);

// Different sessions have independent positions
app.current_screen = Screen::SessionDetail(42);
app.selected_index = 5;
app.save_cursor_position();

app.current_screen = Screen::SessionDetail(99);
app.selected_index = 3;
app.save_cursor_position();

app.current_screen = Screen::SessionDetail(42);
app.restore_cursor_position();
assert_eq!(app.selected_index, 5); // Not 3

// SessionList is global context (no session ID)
app.current_screen = Screen::SessionList;
app.selected_index = 7;
app.save_cursor_position();
// ... view some sessions ...
app.current_screen = Screen::SessionList;
app.restore_cursor_position();
assert_eq!(app.selected_index, 7);
```

---

## Documentation Updates

The following documentation has been pre-written and is ready:

✅ [DESIGN.md Section 9.1](DESIGN.md#911-training-templates-architecture) - Complete templates specification  
✅ [DESIGN.md Section 9.2](DESIGN.md#92-cursor-position-memory-session-aware) - Complete cursor memory specification  
✅ [DESIGN.md Section 9.3](DESIGN.md#93-future-feature-session--training-content-diff) - Diff feature future plans  
✅ [PROJECT_STATUS.md Phase 5](PROJECT_STATUS.md#phase-5-training-templates--cursor-memory-planned---not-started) - Timeline & checklist

No additional documentation needed during implementation - just reference these sections.

---

## Implementation Checklist

### Pre-Implementation
- [x] Architecture designed and confirmed
- [x] Database schema finalized
- [x] UI flows documented
- [x] Test strategy defined
- [x] Program structure defined

### During Implementation
- [ ] All unit tests written (85%+ coverage)
- [ ] All integration tests written
- [ ] Documentation updated with examples
- [ ] Backward compatibility verified
- [ ] Performance tested (with realistic data)
- [ ] Code reviewed

### Post-Implementation
- [ ] All 90+ tests passing
- [ ] No compiler warnings
- [ ] Changelog updated
- [ ] Git commits organized with clear messages
- [ ] Features working end-to-end
- [ ] Ready for next phase

---

## Estimated Effort Breakdown

| Component | Estimate | Notes |
|-----------|----------|-------|
| Templates DB & Models | 1-2 days | Foundation layer |
| Templates Repositories | 1-2 days | Data access |
| Templates Services | 2-3 days | Business logic |
| Templates UI | 3-4 days | Most complex part |
| Templates Integration | 1-2 days | Putting it together |
| Cursor Memory | 1-2 days | Can overlap with templates |
| **Total** | **2-3 weeks** | Including testing & docs |

### Parallelization Opportunities
- Cursor Memory can be implemented in parallel with Templates (independent features)
- UI development can overlap with Repository development
- Testing can begin as soon as Models are created

---

## Success Criteria

### Phase 5.1: Training Templates ✅ COMPLETE
- [x] Coaches can create reusable templates
- [x] Templates display creator/editor metadata
- [x] Sessions can reference templates (hybrid with overrides)
- [x] Template library accessible from Home screen
- [x] All tests passing (90+)
- [x] No duplicate training content across sessions
- [x] Documentation complete

### Phase 5.2: Cursor Memory ✅ COMPLETE
- [x] Selection position remembered per screen context
- [x] Different sessions have independent positions
- [x] Position restored when navigating back
- [x] All tests passing (90+)
- [x] Smooth UX (no more losing your place)
- [x] Documentation complete

---

## Next Phase (Phase 6)

After Phase 5 is complete and all tests pass:

**Phase 6: Session & Training Content Diff** (Planned)
- Compare templates to sessions
- Show what changed since session creation
- Audit history with diffs
- Help coaches identify outdated sessions

See [DESIGN.md Section 9.3](DESIGN.md#93-future-feature-session--training-content-diff) for initial concept.

---

**Document Version**: 1.0  
**Created**: January 29, 2026  
**Last Updated**: January 29, 2026
