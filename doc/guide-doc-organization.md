# Documentation Organization Guide

**Purpose**: Explain how Cando-RS documentation is organized  
**Created**: 2025-01-23 (Session 69)  
**Status**: Reference Document

---

## The Confusion: Multiple "Phase" Numbering Systems

Cando-RS has **three different numbering systems** that were causing confusion:

### 1. WebUI Development Phases (`doc/phases/`)

Historical phases for WebUI development:
- **Phase 3**: J1939 Architecture
- **Phase 4**: Multi-Protocol Support
- **Phase 5**: Hardware Validation

**Location**: `doc/phases/`  
**Scope**: WebUI feature development  
**Status**: Historical record

---

### 2. Dynamic Registry Phases (`doc/dynamic-registry/`)

Implementation phases for message registry (Sessions 61-69):
- **Phase 1**: DynamicCanMessage trait
- **Phase 2**: MessageRegistry implementation
- **Phase 3**: Tool migration
- **Phase 4**: Virtual messages
- **Phase 5**: Performance & documentation

**Location**: `doc/dynamic-registry/`  
**Scope**: Dynamic registry feature  
**Status**: Complete (Phase 5 finished)

---

### 3. AI Work Sessions (`doc/sessions/`)

Chronological AI pairing sessions:
- SESSION-30, SESSION-34, SESSION-52, etc.
- Records of AI pair programming work
- Cross-cuts multiple features

**Location**: `doc/sessions/`  
**Scope**: Historical session records  
**Status**: Ongoing (Session 69 current)

---

## Current Documentation Structure

```
doc/
├── dynamic-registry/           # Dynamic registry implementation (Phases 1-5)
│   ├── README.md               # Index and overview
│   ├── MESSAGE-REGISTRY-IMPLEMENTATION-PLAN.md
│   ├── PHASE4B-VIRTUAL-MESSAGES-PLAN.md
│   ├── PHASE5-PERFORMANCE-ANALYSIS.md
│   ├── PHASE5-INVESTIGATION.md
│   ├── PHASE5-INTEGRATION-FINDINGS.md
│   └── VIRTUAL-MESSAGES-USER-GUIDE.md
│
├── phases/                     # WebUI development phases (historical)
│   ├── README.md
│   ├── PHASE3-J1939-ARCHITECTURE.md
│   ├── PHASE4-MULTI-PROTOCOL.md
│   └── PHASE5-HARDWARE-VALIDATION.md
│
├── sessions/                   # AI session records (chronological)
│   ├── README.md
│   ├── SESSION-30-*.md
│   ├── SESSION-52-*.md
│   └── SESSION-67-QUICK-START.md
│
├── build-system/               # Build system documentation
├── presentations/              # Sales/technical presentations
│
├── AI_CRITICAL_REMINDERS.md    # AI workflow rules
├── AI_PROJECT_STANDARDS.md     # Coding standards
├── CODE-COVERAGE.md            # Test coverage guidelines
├── GIT-WORKTREES-TUTORIAL.md   # Worktree infrastructure
└── GIT-WORKTREES-PRACTICAL-GUIDE.md
```

---

## Navigation Guide

### "I want to learn about virtual messages"

**Start here**: `doc/dynamic-registry/VIRTUAL-MESSAGES-USER-GUIDE.md`

This is the comprehensive user guide covering:
- What virtual messages are
- How to use them (CLI + API)
- Performance characteristics
- Troubleshooting

### "I want to understand the registry architecture"

**Start here**: `doc/dynamic-registry/MESSAGE-REGISTRY-IMPLEMENTATION-PLAN.md`

This is the complete 4-phase implementation plan with:
- Architectural decisions
- Phase-by-phase roadmap
- Performance requirements
- Success criteria

### "I want to see performance benchmarks"

**Start here**: `doc/dynamic-registry/PHASE5-PERFORMANCE-ANALYSIS.md`

Contains:
- 13 comprehensive benchmarks
- Performance baseline (1.2μs overhead)
- Bottleneck analysis
- Optimization opportunities

### "I want to understand WebUI phases"

**Start here**: `doc/phases/README.md`

Historical WebUI development phases (different from registry phases).

### "I want to see session history"

**Start here**: `doc/sessions/README.md`

Chronological AI pairing session records.

---

## How to Reference Docs

### In Code Comments

```rust
// See doc/dynamic-registry/VIRTUAL-MESSAGES-USER-GUIDE.md for usage examples
```

### In README

```markdown
See [Virtual Messages User Guide](doc/dynamic-registry/VIRTUAL-MESSAGES-USER-GUIDE.md)
```

### In RESUME.md

```markdown
**Phase 5 Documentation**: doc/dynamic-registry/PHASE5-*.md
```

---

## Adding New Documentation

### Feature-Specific Docs

Create subdirectory under `doc/`:
```bash
doc/
└── my-new-feature/
    ├── README.md
    ├── IMPLEMENTATION-PLAN.md
    └── USER-GUIDE.md
```

### Session Records

Add to `doc/sessions/`:
```bash
doc/sessions/SESSION-70-MY-WORK.md
```

### Project-Wide Guides

Add to `doc/` root:
```bash
doc/MY-WORKFLOW-GUIDE.md
```

---

## What Got Removed

### Temporary Session Documents

- `doc/GIT-BRANCH-SURGERY-COMPLETE.md` ❌ (served purpose, no longer needed)
- `doc/GIT-BRANCH-SURGERY-PLAN.md` ❌ (temporary planning doc)

These were created during Session 69 for git branch surgery but have no long-term value.

### What Stayed

- `doc/GIT-WORKTREES-TUTORIAL.md` ✅ (permanent infrastructure)
- `doc/GIT-WORKTREES-PRACTICAL-GUIDE.md` ✅ (permanent workflow guide)

These are permanent project infrastructure documentation.

---

## Key Takeaways

1. **Feature-specific docs go in subdirectories** (`doc/dynamic-registry/`, `doc/phases/`)
2. **Phases are feature-specific** (WebUI phases ≠ Registry phases)
3. **Sessions are chronological** (SESSION-30, SESSION-52, etc.)
4. **Common infrastructure in doc/ root** (AI guides, git workflows)

---

## Questions?

See `RESUME.md` for current project status or specific feature READMEs for detailed documentation.
