# Phase 5 Architecture: Summary & Next Steps

**Date**: January 29, 2026  
**Status**: ✅ Design Complete - Ready for Implementation  
**Duration to Implement**: 2-3 weeks  

---

## What Was Designed

### Feature 1: Training Templates System
**Problem Solved**: Reduce duplicate training content across sessions

**How It Works**:
1. **Global Template Library**: Coaches create reusable training templates (drills, exercises, etc.)
2. **Sessions Reference Templates**: Instead of copying content, sessions link to templates
3. **Hybrid Binding**: Sessions can customize templates with per-session notes
4. **Audit Tracking**: See who created and last edited each template

**Database**:
- NEW: `training_templates` table (with creator/editor metadata)
- NEW: `session_training_links` table (junction, with custom notes)
- MIGRATE: Existing training_content → templates

**New UI Screens** (4):
- Templates List (browse all templates)
- Template Detail (view + audit info)
- Template Create/Edit (forms)
- Training Content Picker (when adding to sessions)

### Feature 2: Cursor Position Memory
**Problem Solved**: Selection position resets when navigating back

**How It Works**:
1. **Session-Aware Storage**: Different positions remembered for different session contexts
2. **Auto-Restore**: Navigating back to SessionDetail #42 shows your last selection in that session
3. **Independent Per Session**: SessionDetail #42 and #99 remember separate positions
4. **In-Memory Only**: Saved during app session, lost on exit (simple, no persistence)

**Examples**:
- Select item #7 in SessionList → go to detail → come back → at item #7 ✓
- In SessionDetail #42, select item #5 → do stuff → back → at item #5 ✓
- Switch to SessionDetail #99 → still at item #5? No! Different context, starts fresh ✓

---

## Your Architecture Choices (CONFIRMED)

### Templates

| Choice | Your Selection | Why It Matters |
|--------|---|---|
| **Migration Strategy** | Option A (Full) | Cleaner code, templates as single source of truth |
| **Template Visibility** | Global (all coaches) | Share drills across team, track who created/edited each |
| **Session Binding** | Hybrid | Flexibility: reference templates but override per-session |
| **Test Organization** | Separate folders | `/tests/unit/templates/` keeps related tests together |

### Cursor Memory

| Choice | Your Selection | Why It Matters |
|--------|---|---|
| **Memory Scope** | Session-aware | Each session remembers its own position independently |
| **Persistence** | In-memory only | Simpler (no file I/O), fresh start each app session |
| **Storage Method** | HashMap keys | "session_detail:123" format handles all screen types |

---

## What's Documented (Ready to Use)

### 1. **DESIGN.md** (Added Section 9: 1,321 lines total)
- **9.1 Training Templates** (complete spec):
  - Database schema with all fields
  - Data models (Rust structs)
  - Repository interfaces (all methods)
  - UI screen hierarchy with diagrams
  - Program structure (file organization)
  - Audit tracking examples
  
- **9.2 Cursor Memory** (complete spec):
  - HashMap implementation
  - Key handler integration
  - Multi-session examples
  - Test examples
  
- **9.3 Future: Diff Feature** (concept planning)

### 2. **PHASE_5_ROADMAP.md** (NEW - Implementation timeline)
- 2-3 week timeline breakdown
- 5 implementation phases with daily estimates
- Testing strategy
- Success criteria
- Parallelization opportunities

### 3. **PHASE_5_IMPLEMENTATION_GUIDE.md** (NEW - Developer quick ref)
- Database schema overview
- Exact Rust struct definitions (copy-paste ready)
- Repository method signatures
- Screen descriptions and flows
- Code examples (migration, cursor memory, audit)
- Common pitfalls to avoid
- Test file organization

### 4. **PROJECT_STATUS.md** (Updated)
- Phase 5 section with full checklist
- Phase 6 preview (Diff feature)
- Realistic timeline estimates

---

## Implementation Roadmap (When You're Ready)

### Week 1: Foundation
- [ ] Database schema & migrations
- [ ] Models (TrainingTemplate, SessionTrainingLink)
- [ ] Repositories (CRUD operations)
- [ ] Unit tests for foundation

### Week 2: UI & Integration  
- [ ] Templates list screen
- [ ] Template detail screen
- [ ] Template create/edit forms
- [ ] Navigation integration
- [ ] Cursor memory implementation
- [ ] Integration tests

### Week 3: Polish & Release
- [ ] Template picker (for sessions)
- [ ] Data migration (training_content → templates)
- [ ] Comprehensive testing
- [ ] Documentation examples
- [ ] Git organization

---

## Key Implementation Details (Copy-Paste Ready)

### Database Schema
```sql
training_templates (
  id, coach_id, title, content_type, description, 
  duration_minutes, created_by, last_edited_by, 
  last_edited_at, is_public
)

session_training_links (
  id, session_id, training_template_id, order_index, custom_notes
)
```

### Rust Models (in src/models/training_template.rs)
```rust
TrainingTemplate {
  id, coach_id, title, content_type, description,
  duration_minutes, created_at, created_by,
  last_edited_by, last_edited_at, is_public
}

SessionTrainingLink {
  id, session_id, training_template_id, 
  order_index, custom_notes, template_data
}
```

### Cursor Memory (in src/app.rs)
```rust
pub struct App {
  screen_selection_history: HashMap<String, usize>
}

// Key format: "screen_type:context"
// Examples: "session_list:global", "session_detail:123"
```

---

## What Was NOT Implemented (By Your Request)

### ✗ No Code Changes Yet
- This is design and documentation only
- Ready to start Phase 5 implementation whenever you're ready

### ✗ Diff Feature (Phase 6)
- Documented as future work in DESIGN.md Section 9.3
- Will track when templates change vs session creation
- Can compare template versions
- After Phase 5 complete

---

## Files You Should Review

**Start Here:**
1. [PHASE_5_IMPLEMENTATION_GUIDE.md](docs/PHASE_5_IMPLEMENTATION_GUIDE.md) - 5-10 min read, get the gist
2. [PHASE_5_ROADMAP.md](docs/PHASE_5_ROADMAP.md) - Timeline & task breakdown

**Deep Dives:**
3. [DESIGN.md Section 9](docs/DESIGN.md#9-training-templates--cursor-memory-planned-features) - Architecture & decisions
4. [PROJECT_STATUS.md Phase 5](docs/PROJECT_STATUS.md#phase-5-training-templates--cursor-memory-planned---not-started) - Status & progress

---

## Next Steps

### Option 1: Start Implementation (2-3 weeks)
```
When ready:
1. Create database migration file
2. Implement models and repositories
3. Build UI screens
4. Write comprehensive tests
5. Git commit and tag Phase 5
```

### Option 2: Continue Current Work (Recommended)
- Phase 4C is complete (menu navigation ✅)
- Phase 5 is fully designed
- Can work on other features or bug fixes
- Start Phase 5 when ready

### Option 3: Review & Refine
- Read the documentation
- Ask questions or suggest changes
- Adjust design before implementing
- Different approach to any aspect

---

## What Each Document Contains

| Document | Purpose | Length | Use Case |
|----------|---------|--------|----------|
| **DESIGN.md Section 9** | Complete architecture spec | ~400 lines | Reference during implementation |
| **PHASE_5_ROADMAP.md** | Implementation timeline | ~350 lines | Project planning & tracking |
| **PHASE_5_IMPLEMENTATION_GUIDE.md** | Developer quick reference | ~450 lines | Copy-paste code, test organization |
| **PROJECT_STATUS.md Phase 5** | Status & checklist | ~50 lines | Progress tracking |

---

## Summary of Your Decisions

```
✅ Templates: Migrate all existing content → new templates table
✅ Templates: Global, but track creator/editor per template
✅ Templates: Hybrid binding (reference + per-session customization)
✅ Templates: Separate test folders for better organization

✅ Cursor: Remember position per session context (session-aware)
✅ Cursor: In-memory only (simpler, no persistence file)
✅ Cursor: Different positions for different sessions

✅ Future: Add diff feature (Phase 6) to compare template changes
```

---

## Questions Before Starting?

Consider:
1. **Parallel Work**: Cursor memory can be done while templates are being built
2. **Testing**: Comprehensive test structure ready to use
3. **Database**: Migration needed for existing training_content
4. **UI**: 4 new screens, detailed mockups ready
5. **Timeline**: 2-3 weeks with normal pace

---

**Status**: Ready for implementation whenever you choose to start  
**All materials prepared**: Yes ✅  
**Code ready to implement**: Yes ✅  
**Tests designed**: Yes ✅  
**Documentation complete**: Yes ✅  

**Next Action**: Start Phase 5 implementation or continue other work?

---

*Document created: January 29, 2026*  
*Total documentation added: 1,321 lines*  
*Files created: 2 (PHASE_5_ROADMAP.md, PHASE_5_IMPLEMENTATION_GUIDE.md)*  
*Files modified: 2 (DESIGN.md, PROJECT_STATUS.md)*  
*Code changes: 0 (design and docs only)*
