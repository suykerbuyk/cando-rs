# AI Project Standards and Context Directives

**Purpose**: This document establishes permanent guidelines for AI assistants working on the Cando-RS project. These directives ensure consistency, maintainability, and project organization across development sessions.

**Status**: Authoritative - All AI assistants MUST follow these guidelines  
**Last Updated**: 2024-11-03

---

## 📋 Table of Contents

- [File Organization Standards](#file-organization-standards)
- [Development Workflow](#development-workflow)
- [Testing Requirements](#testing-requirements)
- [Documentation Standards](#documentation-standards)
- [Code Quality Standards](#code-quality-standards)
- [CAN Hardware Constraints](#can-hardware-constraints)
- [Context Restoration Protocol](#context-restoration-protocol)

---

## 📁 File Organization Standards

### Root Directory Rules

**ONLY the following file types are allowed in the project root:**

1. **Core Project Files**:
   - `README.md` - Project overview and documentation
   - `Cargo.toml` - Workspace configuration
   - `Cargo.lock` - Dependency lock file
   - `Makefile` - Build system automation
   - `.gitignore` - Git exclusions

2. **AI Context Restoration** (Special Exception):
   - `RESUME.md` - AI context restoration entry point
   - `commit.msg` - Git commit message template/staging

3. **Required Configuration**:
   - `.zed/`, `.vscode/` - IDE configuration directories
   - Build artifacts (`target/`) - ignored by git

**EVERYTHING ELSE MUST BE ORGANIZED** into appropriate subdirectories.

### Script Organization

**Location**: `scripts/`

All executable scripts MUST be placed in subdirectories under `scripts/`:

```
scripts/
├── integration/        # Integration test scripts (validate_*.sh)
├── dev-tools/         # Development utilities (clear-diagnostics.sh, etc.)
├── benchmarking/      # Performance testing scripts
└── utilities/         # General-purpose utilities
```

**Rules**:

- ✅ All `.sh`, `.py`, `.rb` scripts go here
- ✅ Create descriptive subdirectories for logical grouping
- ✅ Scripts should be executable (`chmod +x`)
- ❌ NEVER place scripts in project root
- ❌ NEVER place scripts in `doc/` directory

### Documentation Organization

**Location**: `doc/`

All documentation MUST be organized into feature/topic-specific subdirectories:

```
doc/
├── AI_CONTEXT_RESTORATION.md          # Master context document
├── AI_PROJECT_STANDARDS.md            # This file - project standards
├── AI-HOW-TO-ADD-J1939-MESSAGES.md   # Implementation guides
├── AI-HOW-TO-ADD-J1939-73-DIAGS.md   # Implementation guides
│
├── session-restoration/               # Historical session notes
│   ├── COMPLETE-SUMMARY.md
│   ├── SESSION-RESUMPTION.md
│   └── resume-j1939-73-build-resolution.md
│
├── j1939-batch-implementation/        # Feature-specific docs
│   ├── batch1_implementation_guide.md
│   ├── batch1_progress.csv
│   └── batch_progress.txt
│
├── metadata-enhancement/              # Feature development docs
│   ├── MANPAGE-COMPLETE.md
│   └── DUMP-MESSAGES-FIX.md
│
├── codegen-development/               # Code generation docs
│   ├── MAKEFILE-CODEGEN-TARGETS.md
│   └── dbc.analyzer.output.txt
│
├── doctest-fixes/                     # Quality improvement docs
│   ├── START_HERE_DOCTEST_FIXES.md
│   └── doc-sync.txt
│
└── validation-history/                # Test/validation records
    ├── VALIDATION_SUMMARY.md
    └── SAFETY-AUDIT-COMPLETE.md
```

**Rules for Persistent Documentation**:

- ✅ **ALL persistent project documentation** MUST be placed in `doc/` directory
- ✅ **Group related documents** into logically named subdirectories
- ✅ **Use descriptive subdirectory names** that reflect the topic/feature:
  - `doc/testing/` - Testing methodology, test completion reports, validation docs
  - `doc/j1939-batch-implementation/` - J1939 message implementation tracking
  - `doc/session-restoration/` - Historical session notes and context
  - `doc/codegen-development/` - Code generation system documentation
- ✅ **Create subdirectories** when you have 3+ related documents on same topic
- ✅ **Consolidate related documents** (don't create single-file directories unless justified)
- ✅ **Use consistent naming** within subdirectories (e.g., `feature-complete.md`, `feature-summary.md`)

**Examples of Good Organization**:

```
doc/testing/                                    # Testing-related documentation
├── phase3-complete.md                          # Phase 3 completion summary
├── physical-j1939-integration-complete.md      # Implementation details
├── physical-j1939-integration-summary.md       # Quick reference
└── physical-j1939-integration-session-start.md # Planning document

doc/j1939-batch-implementation/                 # J1939 implementation tracking
├── batch1_implementation_guide.md
├── batch1_progress.csv
└── batch_progress.txt
```

**What Belongs in doc/ Subdirectories**:

- ✅ Feature implementation completion reports
- ✅ Phase/milestone documentation
- ✅ Technical decision records
- ✅ Implementation guides and how-tos (feature-specific)
- ✅ Session planning and restoration documents
- ✅ Validation and audit reports
- ✅ Progress tracking documents

**Exceptions (Root-Level doc/ Files)**:

- `AI_CONTEXT_RESTORATION.md` - Master project architecture (referenced by resume.md)
- `AI_PROJECT_STANDARDS.md` - This file, project-wide standards
- `AI-HOW-TO-*.md` - General implementation guides (cross-cutting concerns)

**Rules (Strict)**:

- ❌ NEVER place persistent documentation in project root
- ❌ NEVER place documentation in `scripts/` or code directories
- ❌ NEVER mix temporary/generated docs with persistent docs
- ❌ NEVER create deeply nested subdirectories (keep to 1-2 levels max)

### Data Files Organization

**Generated/Temporary Data**:

- Test outputs: `target/` (ignored by git)
- Benchmark reports: `benchmarks/reports/`
- CAN dumps for analysis: `live.can.dumps/`
- Build artifacts: `target/` (ignored by git)

**Rules**:

- ✅ Temporary data goes in `target/` or gets specific subdirectory
- ✅ Important reference data (e.g., CAN logs) gets descriptive directory
- ✅ Distinguish persistent docs (doc/) from temporary output (target/, benchmarks/)
- ❌ NEVER commit temporary output files to git
- ❌ NEVER leave `.out`, `.log`, `.tmp` files in root
- ❌ NEVER place persistent documentation in temporary data directories

---

## 🔧 Development Workflow

### Before Making Changes

1. **Validate current state**: `make tier1` (must pass 100%)
2. **Read context documents** (in order):
   - `doc/AI_CONTEXT_RESTORATION.md` - Full project understanding
   - `doc/AI-HOW-TO-ADD-J1939-MESSAGES.md` - If adding J1939 messages
   - `doc/AI-HOW-TO-ADD-J1939-73-DIAGS.md` - If adding diagnostics
3. **Check git status**: Understand working directory state
4. **Review last 5 commits**: `git log -5` - Understand recent changes

### During Development

1. **Follow established patterns**: Use existing code as templates
2. **Test incrementally**: Build and test after each logical unit
3. **Document as you go**: Update relevant docs when changing behavior
4. **Maintain zero warnings**: `cargo build` must be clean
5. **Preserve test coverage**: Tests must remain at 100% pass rate
6. **Use tee for build/test logging**: Log all build and test output for debugging

   ```bash
   # Create logs directory if it doesn't exist
   mkdir -p logs

   # Log ALL build output while showing it in real-time
   cargo build --workspace 2>&1 | tee logs/build-$(date +%Y%m%d-%H%M%S).log
   cargo build -p cando-webui 2>&1 | tee logs/build-webui-$(date +%Y%m%d-%H%M%S).log

   # Log ALL test output
   cargo test --workspace 2>&1 | tee logs/test-$(date +%Y%m%d-%H%M%S).log
   cargo test -p cando-webui 2>&1 | tee logs/test-webui-$(date +%Y%m%d-%H%M%S).log
   make tier1 2>&1 | tee logs/tier1-$(date +%Y%m%d-%H%M%S).log
   make tier2 2>&1 | tee logs/tier2-$(date +%Y%m%d-%H%M%S).log

   # Monitor in another terminal
   tail -f logs/build-*.log
   tail -f logs/test-*.log
   ```

   **When to use**: For all significant builds/tests during development

   **Benefits**: Detect hangs, review full output, debug failures, keep history

   **Exception**: Quick iteration checks (`cargo check`, `cargo clippy`) may skip logging

### After Making Changes

1. **Run tier1 validation**: `make tier1` (must pass 48/48 tests)
2. **Run tier2 validation**: `make tier2` (must pass 22/22 tests, vcan only)
3. **Check for file clutter**: Run organization audit (see below)
4. **Update documentation**: Reflect changes in relevant docs
5. **Stage organized commits**: Clean git history with descriptive messages

### File Organization Audit

**Before committing, run this check**:

```bash
# Check for clutter in root
ls -1 *.md *.sh *.txt *.json *.csv *.out 2>/dev/null | \
  grep -v "README.md\|resume.md\|commit.msg"

# Should output nothing if properly organized
# If files are listed, organize them per this document
```

### AI Assistant Workflow (CRITICAL)

**These practices are MANDATORY for all AI assistants working on this project.**

#### Git Staging Practices

**AI assistants are authorized and expected to stage files**:

```bash
# Stage newly created or modified implementation files
git add <file1> <file2> <file3>

# Check what's staged
git status
git diff --cached --stat
```

**Rules**:

- ✅ **DO** stage all files you create or modify as part of a logical unit of work
- ✅ **DO** stage documentation updates alongside code changes
- ✅ **DO** verify staged changes with `git diff --cached`
- ✅ **DO** organize files properly before staging (scripts/, doc/, etc.)
- ✅ **DO** prepare commit messages in `commit.msg` file
- ❌ **DO NOT** stage unrelated changes
- ❌ **DO NOT** stage temporary files (.log, .out, .tmp)
- ❌ **DO NOT** commit without user review and approval

**⛔ CRITICAL: AI ASSISTANTS MUST NEVER EXECUTE `git commit` ⛔**

**Commit Authority**:

- **ONLY THE USER** may execute `git commit`
- **AI MUST** stage files with `git add`
- **AI MUST** prepare `commit.msg` file with complete commit message
- **AI MUST** report what is staged and ready for user review
- **AI MUST NEVER** execute: `git commit`, `git commit -m`, `git commit -F`, etc.
- **USER REVIEWS** staged changes and commit message
- **USER EXECUTES** `git commit -F commit.msg` when satisfied

**Workflow**:

1. AI stages files: `git add <files>`
2. AI updates `commit.msg` with complete message
3. AI reports: "Files staged and ready for your review"
4. User reviews: `git diff --cached`, `cat commit.msg`
5. User commits: `git commit -F commit.msg` (or makes changes)

**Best Practice**: Stage files incrementally as you complete logical units of work, not all at once at the end.

#### Resume.md and Context Document Updates

**At the completion of each stage or deliverable unit of work**:

1. **Update RESUME.md** (MANDATORY):
   - Mark completed tasks with ✅
   - Update status indicators (⏸️ NEXT → ✅ COMPLETE)
   - Add implementation metrics (lines added, time taken, tests passing)
   - Document what was accomplished

#### Thread Context Switch Procedure (MANDATORY)

**Purpose**: Enable seamless context restoration when switching AI threads or starting new sessions, with incremental git commits as progress snapshots.

**When to Execute**: When ready to commit a unit of work and switch context (end of session, before break, after completing a phase).

**CRITICAL**: This is a **5-step procedure** that must be executed **in order**. Each step is required. The human will ALWAYS do the final `git commit` - the AI prepares everything and presents it for review.

---

### Step 1: Create/Update Feature-Specific Documentation

**One Feature = One Document in `doc/`**

For each feature branch or major task, create a single document that serves as the **durable record** of all work:

**Document Name**: `doc/[FEATURE-NAME]-[PURPOSE].md`

- Examples: `doc/CLIPPY-WARNINGS-SIZED-TYPES.md`, `doc/SNAKE-CASE-FIELD-NAMES.md`
- Use descriptive names that match the branch purpose
- This document lives throughout the entire feature development

**What Goes in This Document**:

1. **Investigation findings** - What was discovered and why
2. **Architectural decisions** - Choices made and rationale
3. **Implementation approach** - How the solution works
4. **Current progress** - What's done, what's in progress, what's left
5. **Technical details** - Code changes, patterns, algorithms
6. **Lessons learned** - Process insights, gotchas, best practices
7. **Open questions** - Blockers, decisions needed, uncertainties

**What This Document Is**:

- ✅ The single source of truth for this feature/branch
- ✅ Updated incrementally as work progresses
- ✅ Long-lived (exists until feature complete and merged)
- ✅ Detailed technical documentation
- ✅ Context restoration guide for next AI session

**What This Document Is NOT**:

- ❌ A temporary investigation document (it's permanent)
- ❌ A session summary (those go in RESUME.md)
- ❌ Rewritten each commit (it's updated/appended)

**Example Structure**:

```markdown
# Clippy Warnings: Sized Types Implementation

**Branch**: `fix/clippy-warnings-codegen`
**Status**: 🟡 In Progress - Phase 2 of 3

## Investigation Summary

[What we discovered about the root cause]

## Architectural Decisions

[Why we chose sized types over u64 everywhere]

## Implementation Approach

[How the code generator was modified]

## Current Progress

- ✅ Phase 1: Investigation and design
- 🟡 Phase 2: Code generator updates (in progress)
- ⏸️ Phase 3: Consumer code updates

## Technical Details

[Specifics of implementation]

## Open Questions

[What's blocking or unclear]
```

---

### Step 2: Update RESUME.md (High-Level Status)

**RESUME.md contains HIGH-LEVEL status only.** It points to feature documents for details.

**Update the "CURRENT STATUS" section**:

1. **Branch status** - What branch, what phase, what's blocking
2. **Brief summary** - High-level overview (2-3 sentences)
3. **Reference to feature doc** - Point to `doc/FEATURE-NAME.md` for details
4. **Next actions** - What should be done next

**Keep it concise** - Details live in the feature document, not RESUME.md.

**Example**:

```markdown
## ⚡ CURRENT STATUS

### Fix Branch: clippy-warnings-codegen 🟡 IN PROGRESS

**Branch**: `fix/clippy-warnings-codegen`
**Status**: 🟡 Phase 2 - Code generator updates complete, consumer code needs fixing

**Summary**: Investigating clippy warnings led to discovery that code generator
was using u64 for all fields. Implemented sized types (u8, u16, u32) based on
actual bit widths. Generator updated, all protocols regenerated. Now fixing 446
type mismatches in consumer code.

**Details**: See `doc/CLIPPY-WARNINGS-SIZED-TYPES.md` for complete investigation
and implementation details.

**Next Steps**: Fix type mismatches in J1939 diagnostic helpers.
```

**DO NOT create separate SESSION-YYYY-MM-DD.md files** - session summaries are temporary and belong in git commit messages, not permanent documentation.

---

### Step 3: Stage ALL Relevant Files

**CRITICAL**: You MUST stage all modified/created files before presenting to user.

```bash
# Check what files changed
git status

# Stage implementation files (all modified source code)
git add <modified-source-files>

# Stage generated code (if regenerated)
git add <generated-files>

# Stage test files (all modified tests)
git add <test-files>

# Stage documentation (RESUME.md + any completion docs)
git add RESUME.md
git add doc/<new-completion-docs>

# Stage configuration changes (if any)
git add dbc/.checksums.json  # or other config files

# Verify what's staged
git status --short
# Should show "M  " or "A  " (staged) for all relevant files
# Should NOT show " M " (unstaged modifications)

# Review staged changes
git diff --staged --stat
```

**DO NOT stage `commit.msg`** - it should remain untracked.

**Common mistake**: Forgetting to stage files. The user should see:

```
M  RESUME.md
A  doc/NEW-DOC.md
M  src/main.rs
```

NOT:

```
 M RESUME.md        <-- WRONG! Space before M means unstaged
 M doc/NEW-DOC.md   <-- WRONG! Not staged
```

---

### Step 4: Create commit.msg File (Temporary Snapshot Message)

Write a comprehensive commit message that captures THIS commit's snapshot.

**File**: `commit.msg` (in project root, untracked)

**CRITICAL UNDERSTANDING**:

- ✅ `commit.msg` is **TEMPORARY** - rewritten for each context switch
- ✅ It captures the **snapshot** of work being committed right now
- ✅ It is **NEVER staged** or tracked in git
- ✅ After commit, it can be overwritten for the next commit
- ❌ It is NOT a permanent record (that's in the feature doc)

**Format** (Conventional Commits):

```
type(scope): Brief summary of this commit (50 chars max)

Detailed description of what THIS COMMIT includes.

## What Was Accomplished in This Session
- Completed investigation of X
- Implemented Y changes
- Updated Z documentation

## Files Modified
- cando-codegen/src/generator.rs: Added sized type logic
- cando-messages/src/generated/*.rs: Regenerated with new types
- doc/CLIPPY-WARNINGS-SIZED-TYPES.md: Updated with progress

## Testing Status
- ✅ All unit tests passing (872/872)
- ⚠️ 446 type mismatches in consumer code (expected, next phase)

## Current Progress
- ✅ Phase 1: Investigation complete
- ✅ Phase 2: Generator updates complete
- ⏸️ Phase 3: Consumer code updates (next session)

## Next Actions
See doc/CLIPPY-WARNINGS-SIZED-TYPES.md for:
- List of files needing type fixes
- Approach for fixing diagnostic helpers
- Estimated 2-3 hours remaining

Branch: fix/clippy-warnings-codegen
Refs: #clippy-warnings
```

**Verification**:

```bash
git status | grep commit.msg
# Should show: "Untracked files"
# Should NOT be in "Changes to be committed"
```

**DO NOT stage commit.msg!** It stays untracked.

---

### Step 5: Present to User and STOP

**Present a summary showing**:

1. **What was accomplished** (brief overview)
2. **Files staged** (output of `git status --short`)
3. **Commit message** (mention it's in `commit.msg` ready for review)
4. **Explicit question**: "Would you like to review and commit these changes?"

**Example presentation**:

```markdown
## ✅ Thread Context Switch Complete

### What Was Accomplished

- Implemented Feature X across Y files
- All Z tests passing (100%)
- Created completion documentation

### Files Staged (N files)

Documentation (3 files):

- RESUME.md
- doc/FEATURE-COMPLETE.md
- doc/FEATURE-INVESTIGATION.md

Implementation (5 files):

- src/main.rs
- src/module.rs
  ...

Test files (2 files):

- tests/integration_test.rs
  ...

### Commit Message

Created in `commit.msg` and ready for your review.

**Ready for your review!** You can:

1. Review staged changes: `git diff --staged`
2. Review commit message: `cat commit.msg`
3. When ready: `git commit -F commit.msg`
```

**STOP HERE and WAIT for user's explicit approval.**

User will review and run: `git commit -F commit.msg`

---

### Common Mistakes to Avoid

❌ **Don't**: Create commit without user approval
❌ **Don't**: Stage commit.msg file  
❌ **Don't**: Leave files unstaged (must stage BEFORE presenting)
❌ **Don't**: Put detailed technical info in RESUME.md (use feature doc)
❌ **Don't**: Create temporary investigation documents (use feature doc)
❌ **Don't**: Skip updating the feature document

✅ **Do**: Maintain ONE feature document in doc/ for the entire branch
✅ **Do**: Keep RESUME.md high-level, pointing to feature docs for details
✅ **Do**: Stage ALL relevant files before presenting
✅ **Do**: Create commit.msg but leave it untracked (rewritten each commit)
✅ **Do**: Present summary and wait for approval
✅ **Do**: Let user run the final `git commit` command

---

### Why This Procedure Matters

**Document Hierarchy**:

- **Feature doc** (`doc/FEATURE-NAME.md`): Durable technical record, updated throughout feature
- **RESUME.md**: High-level status, points to feature docs
- **commit.msg**: Temporary snapshot message, rewritten each commit

**Benefits**:

- No scattered temporary documents to maintain
- Clear single source of truth for each feature
- Easy context restoration (read feature doc + RESUME.md)
- User sees exactly what's being committed
- Clean commit history with detailed messages
- **Human approval required**: User maintains control of git history
- **Complete context**: Next AI can restore context from RESUME.md + completion docs
- **Clean commit history**: Well-documented commits with comprehensive messages
  - Update "Next Steps" or "Current Task" sections
  - Add references to new documentation created

2. **Update dependent documentation** (as needed):
   - If architecture changed: Update `doc/AI_CONTEXT_RESTORATION.md`
   - If new patterns emerged: Update implementation guides
   - If new features added: Update README.md
   - If testing approach changed: Update relevant test documentation

3. **Create completion documents** (for significant work):
   - Location: Appropriate subdirectory in `doc/`
   - Naming: Descriptive, feature-specific name
   - Content: Implementation details, decisions, metrics, lessons learned
   - Purpose: Future context restoration and knowledge preservation

**Example resume.md update pattern**:

```markdown
## Current Task: Feature X Implementation ✅ COMPLETE

**Status**: ✅ COMPLETE (was: ⏸️ NEXT)
**Time**: 2 hours (estimated 3-4 hours)
**Files Modified**: 4 files, +1,229 lines

### Implementation Steps:

1. Step 1: Import infrastructure ✅ COMPLETE (was: ⏸️ PENDING)
2. Step 2: Adapt test function ✅ COMPLETE
3. Step 3: Add messages ✅ COMPLETE
4. Step 4: Documentation ✅ COMPLETE

**Key Documentation**:

- 📊 `doc/feature/implementation-complete.md` (649 lines) ✨ NEW
- 📊 `doc/feature/implementation-summary.md` (319 lines) ✨ NEW
```

#### Commit Message Management

**At the completion of each stage or deliverable unit of work**:

**Rewrite `commit.msg`** to reflect incremental progress achieved:

1. **Format**: Plain text (not markdown) - suitable for `git commit -F commit.msg`
2. **Structure**:

   ```
   feat: Brief summary of what was accomplished

   Detailed explanation of the changes, why they were made,
   and what problem they solve.

   Changes:
   --------
   1. File/Component A
      - Specific change 1
      - Specific change 2

   2. File/Component B
      - Specific change 1

   Technical Details:
   ------------------
   - Implementation approach
   - Key design decisions
   - Performance considerations

   Files Modified:
   ---------------
   M path/to/file1.rs (+XX lines)
   A path/to/file2.md (+XXX lines)

   Testing:
   --------
   - All tier1 tests passing (48/48)
   - Zero warnings
   ```

3. **Commit Type Prefixes**:
   - `feat:` - New features or capabilities
   - `fix:` - Bug fixes
   - `docs:` - Documentation only changes
   - `refactor:` - Code restructuring without behavior change
   - `test:` - Adding or modifying tests
   - `chore:` - Maintenance tasks (dependencies, build config)

4. **Content Requirements**:
   - ✅ Clear, descriptive summary line (<72 characters preferred)
   - ✅ Explain **what** changed and **why** (not just how)
   - ✅ Include metrics (lines changed, tests passing, time taken)
   - ✅ List all modified files with change counts
   - ✅ Document technical decisions and trade-offs
   - ✅ Plain text format (no markdown formatting like bold, italics, tables)
   - ❌ Don't use markdown syntax (use plain text alternatives)
   - ❌ Don't include emojis in commit messages (save for documentation)

**Convention**: Use `commit.msg` (not `commit.md`) for actual git commits.

#### Commit Review Documentation (COMMIT-READY.md)

**At the completion of each stage or deliverable unit of work**:

**Create or update a `COMMIT-READY.md` file** in the appropriate doc/ subdirectory:

1. **Purpose**: Provide a structured review summary for commit approval
   - Historical record of what was reviewed and committed
   - Snapshot of the completion state
   - Checklist to verify everything is ready
   - Reference for future similar commits

2. **Location**: Place in feature-specific doc/ subdirectory
   - Example: `doc/webui/COMMIT-READY.md`
   - Example: `doc/testing/COMMIT-READY.md`
   - Co-located with other feature documentation

3. **Update Strategy**:
   - Create initially when staging first commit for a feature
   - Update with each subsequent commit in the feature branch
   - Maintains historical record of incremental progress

4. **Required Sections**:

   ```markdown
   # [Feature Name] - Commit Review Summary

   **Date**: YYYY-MM-DD
   **Branch**: feature-name
   **Status**: ✅ Ready for Commit Review

   ## What's Being Committed

   - Brief overview of changes

   ## Files Staged for Commit

   ### New Files (X files, Y lines)

   - List of new files with line counts

   ### Modified Files (X files)

   - List of modified files with change summary

   ## Documentation Overview

   - Summary of each major documentation file's purpose
   - Key sections and contents

   ## Key Decisions/Changes

   - Major architectural decisions
   - Technology choices
   - Design principles applied

   ## Review Checklist

   - [ ] All documentation files are well-organized
   - [ ] Changes follow project standards
   - [ ] Commit message is comprehensive
   - [ ] Git status is clean

   ## Metrics

   - Lines added/modified
   - Time spent
   - Files created

   ## Next Steps After Commit

   - What to do after this commit
   - Next phase or task
   ```

5. **Benefits**:
   - **For Users**: Easy-to-review summary before commit approval
   - **For Future AI**: Historical context of what was committed and why
   - **For Team**: Clear record of incremental progress
   - **For Auditing**: Traceable decision-making process

6. **Best Practices**:
   - ✅ Update with each commit (don't create fresh each time)
   - ✅ Include metrics and time estimates
   - ✅ Link to related documentation
   - ✅ Provide clear next steps
   - ✅ Make checklists actionable
   - ❌ Don't duplicate content from other docs (summarize instead)
   - ❌ Don't make it too verbose (keep it scannable)

**Example workflow**:

```bash
# First commit in feature branch
# Create doc/feature/COMMIT-READY.md with initial state
git add doc/feature/COMMIT-READY.md

# Second commit in same feature
# Update doc/feature/COMMIT-READY.md with new state
# Shows progression: "Commit 2 of 3" or similar
git add doc/feature/COMMIT-READY.md

# This provides a living document of the feature's progress
```

**Historical Value**:

- Future developers can see what was reviewed at each stage
- AI assistants can learn from past commit patterns
- Provides template for similar features
- Documents decision-making process over time

#### Completion Criteria for a "Stage" or "Deliverable Unit"

**A stage is complete when**:

1. **Implementation**:
   - ✅ All planned code changes implemented
   - ✅ All files properly organized (no root clutter)
   - ✅ All files staged with `git add`

2. **Quality**:
   - ✅ `make tier1` passes 100% (48/48 tests)
   - ✅ Zero compiler warnings
   - ✅ Zero clippy warnings
   - ✅ Code follows established patterns

3. **Documentation**:
   - ✅ `resume.md` updated with completion status
   - ✅ Completion document created (if significant work)
   - ✅ Related documentation updated (if needed)
   - ✅ `commit.msg` written with full details
   - ✅ `COMMIT-READY.md` created or updated in feature doc/ subdirectory

4. **Review Ready**:
   - ✅ `git status` shows clean staged changes
   - ✅ `git diff --cached` shows logical, related changes
   - ✅ Ready for user review and approval

**Example completion checklist**:

```bash
# Verify implementation quality
make tier1                          # Must pass 100%
cargo build --workspace             # Zero warnings
git status                          # Check staged files

# Verify documentation updates
grep "✅ COMPLETE" resume.md        # Task marked complete
ls doc/feature/                     # Completion docs exist
cat commit.msg | head -20           # Commit message ready

# Final check
git diff --cached --stat            # Review changes
```

#### Best Practices (Inferred from Project Patterns)

1. **Incremental Progress**:
   - Work in small, testable increments
   - Stage and document each increment
   - Update `commit.msg` as you progress (not just at the end)

2. **Context Preservation**:
   - Assume future AI or developers will need full context
   - Document **why** decisions were made, not just what
   - Capture metrics (time, effort, alternatives considered)
   - Link related documentation

3. **Quality First**:
   - Never compromise on test coverage
   - Never ignore warnings
   - Never skip validation steps
   - Clean up as you go (don't leave TODOs)

4. **Communication**:
   - Keep user informed of progress
   - Ask clarifying questions early
   - Confirm understanding before major changes
   - Report blockers immediately

5. **Efficiency**:
   - Reuse proven patterns and infrastructure
   - Don't reinvent solutions that exist
   - Leverage established frameworks and helpers
   - Learn from previous completion documents

6. **Organization**:
   - Follow the file organization standards strictly
   - Create proper subdirectories for new features
   - Keep naming consistent with existing patterns
   - Clean up temporary files immediately

7. **Git Hygiene**:
   - Stage related changes together
   - Don't mix unrelated changes
   - Verify staged files before declaring complete
   - Keep commits focused and atomic

#### Workflow Summary

**Every completion cycle**:

```bash
# 1. Complete implementation
make tier1                              # Validate quality

# 2. Organize and stage
git add <files>                         # Stage changes
git status                              # Verify

# 3. Update documentation
# - Edit resume.md (mark ✅ COMPLETE, add metrics)
# - Create completion docs if significant
# - Update dependent docs if needed

# 4. Create/update commit review document
# - Create or update doc/<feature>/COMMIT-READY.md
# - Include all required sections (files staged, metrics, checklist)
# - Provide structured review summary
# - Stage the COMMIT-READY.md file

# 5. Write commit message
# - Update commit.msg with full details
# - Plain text format, comprehensive

# 6. Final verification
git diff --cached --stat                # Review
cat commit.msg | head -20               # Verify message

# 7. Report to user
# - Summarize what was accomplished
# - Show staged files and stats
# - Confirm ready for commit
```

**User commits when ready**: `git commit -F commit.msg`

---

## 🧪 Testing Requirements

### Mandatory Test Execution

**Before ANY commit**:

```bash
make tier1    # 48 tests, <10 minutes, 100% pass required
```

**Before major features or releases**:

```bash
make tier2    # 22 tests, <30 minutes, 100% pass required
```

### Test Coverage Standards

- ✅ All new code MUST have unit tests. We strive for 80% code coverage.
- ✅ All new features MUST have integration tests. We strive for 100% coverage.
- ✅ Doc-tests MUST be executable (no `#[ignore]` without justification)
- ✅ Test pass rate MUST be 100%
- ✅ Zero compiler warnings required
- ✅ Zero clippy warnings required

### Test Organization

```
cando-messages/
├── src/           # Implementation with inline unit tests
└── tests/         # Integration tests
    ├── *_roundtrip.rs          # Message encoding/decoding
    ├── *_comprehensive.rs      # Feature-specific tests
    └── *_integration.rs        # Cross-component tests

scripts/integration/
├── lib/
│   └── config_helpers.sh       # Configuration query functions
├── validate_*.sh               # Tier 1 & 2 test scripts
├── integration_test_*_config.sh  # Config-driven test scripts
└── tier2_*.sh                  # Full-stack integration
```

**🚨 CRITICAL: Test Script Configuration Requirements 🚨**

ALL test scripts MUST follow configuration-driven testing pattern:

- ❌ NO hardcoded device IDs, ports, interfaces, or any configuration values
- ✅ ALL values MUST come from `.yaml` configuration files
- ✅ Use `cando-cfg` tool and `config_helpers.sh` library
- ✅ Scripts MUST accept config file and environment as parameters

See detailed requirements in [Configuration-Driven Testing Pattern](#configuration-driven-testing-pattern-mandatory) section below.

### Code Coverage Standards

**Established**: 2024-01-21 (Session 63)  
**Status**: Active - Different standards for hand-written vs generated code

#### Coverage Targets

**Hand-Written Code: 80%+ line coverage**

All hand-written code in the following modules should maintain at least 80% test coverage:

- `cando-messages/src/common.rs` - Core types and utilities
- `cando-messages/src/metadata.rs` - Metadata structures  
- `cando-messages/src/encoder.rs` - Encoding/decoding logic
- `cando-messages/src/lib.rs` - Library integration
- `cando-messages/src/j1939/` - J1939 helper modules (non-generated)
- `cando-codegen/src/` - Code generation logic
- `cando-core/src/` - Core utilities

**Generated Code: 20-40% line coverage (acceptable)**

Auto-generated protocol files have lower coverage expectations:

- `cando-messages/src/generated/j1939.rs` - 200+ messages
- `cando-messages/src/generated/j1939_73.rs` - 50+ diagnostic messages
- `cando-messages/src/generated/emp_j1939.rs` - 9 EMP messages
- `cando-messages/src/generated/hvpc.rs` - 4 HVPC messages
- `cando-messages/src/generated/udc.rs` - 11 UDC messages

**Rationale for Generated Code**:
1. Uniform implementation - all follow identical patterns
2. Generator itself is tested, ensuring correct output
3. Sample testing validates patterns work correctly
4. Testing every generated message provides diminishing returns

#### Running Coverage Analysis

```bash
# Install coverage tool (one-time)
cargo install cargo-llvm-cov

# Run coverage for cando-messages
cargo llvm-cov --package cando-messages --all-targets

# Generate HTML report
cargo llvm-cov --package cando-messages --all-targets --html
open target/llvm-cov/html/index.html

# Generate LCOV for CI integration
cargo llvm-cov --package cando-messages --lcov --output-path coverage.lcov
```

#### What to Test

**✅ DO TEST:**
- Static types (DeviceId, Percentage, MotorSpeed)
- Validation logic and error paths
- Edge cases and boundary values
- Type conversions and trait implementations
- Helper functions and utilities
- Error message clarity

**❌ DON'T TEST:**
- Every generated message individually
- Auto-generated struct definitions
- Generated field accessors
- Third-party library code

#### Coverage Documentation

See `doc/CODE-COVERAGE.md` for comprehensive coverage guidelines, rationale, and examples.

### Configuration-Driven Testing Pattern (MANDATORY)

**Established**: 2024-12-19 (Phase 2: Configuration-Driven Testing)  
**Updated**: 2025-01-15 (Enhanced with strict prohibitions and WebUI examples)  
**Status**: Project-wide standard for ALL test scripts

**📚 Complete Reference**: See **[CONFIGURATION-GUIDE.md](CONFIGURATION-GUIDE.md)** for comprehensive examples of:

- Starting WebUI with configuration
- Starting simulators (by environment, by device name, with CLI overrides)
- Multi-device startup patterns
- Device name scoping rules (environment-scoped, not globally unique)
- Configuration precedence hierarchy
- All command-line patterns and examples

**🚨 ABSOLUTE RULE: NO HARDCODED VALUES IN TEST SCRIPTS 🚨**

Test scripts MUST NEVER contain hardcoded values for:

- ❌ Device IDs (e.g., `0x82`, `0x8A`)
- ❌ CAN interfaces (e.g., `can0`, `vcan0`)
- ❌ WebSocket ports (e.g., `10752`, `10756`)
- ❌ Device types (e.g., `fan`, `pump`, `test-ecu`)
- ❌ Device variants (e.g., `fan`, `pump`)
- ❌ Protocol modes (e.g., `j1939`, `emp`, `hybrid`)
- ❌ HTTP ports (e.g., `8080`)
- ❌ ANY configuration parameter

**Design Pattern:**

All test scripts MUST use configuration-driven approach with cando configuration
files (`.yaml`) as the ONLY source of truth. This applies to:

- `cando.yaml` - Unified configuration with multiple environments
- Use `--environment` flag to select test environment (e.g., `tier2-virtual`, `webui-simple`)
- All device configurations in single file with environment-based selection

Scripts query configuration using `cando-cfg` tool and `config_helpers.sh` library.

**OLD PATTERN (DEPRECATED - Do Not Use):**

```bash
# ❌ WRONG: Hardcoded parameters scattered in scripts
DEVICE_ID="0x8B"
WEBSOCKET_PORT="10756"
CAN_INTERFACE="vcan0"

j1939-simulator --interface "$CAN_INTERFACE" \
                --device-id "$DEVICE_ID" \
                --websocket-port "$WEBSOCKET_PORT"
```

**Problems with old pattern:**

- Configuration duplicated across multiple scripts
- Parameters drift out of sync with testing topology
- Difficult to maintain when configurations change
- Port conflicts from scattered port assignments
- No single source of truth
- Error-prone environment switching

**NEW PATTERN (MANDATORY - Use This):**

```bash
# ✅ CORRECT: Configuration-driven approach
source scripts/integration/lib/config_helpers.sh
load_test_config "cando.yaml" "tier2-virtual"

# All parameters come from config
start_simulator_by_name "J1939 Test ECU"
```

**Benefits of new pattern:**

- Single source of truth: `cando.yaml` (with environment selection)
- Zero hardcoded device IDs, ports, or interfaces
- Environment-based configuration selection
- Automatic device discovery
- Self-documenting test scenarios
- Easy environment switching
- Port conflicts eliminated via validation
- Maintainable and extensible

**Configuration File Structure:**

```toml
# cando.yaml - Single source of truth for all configurations

environments:
  tier2-virtual:
    friendly_name: "Tier 2 Virtual CAN Testing"
    enabled: true
    devices:
      j1939_test_ecu:
        friendly_name: "J1939 Test ECU (Virtual)"
        type: j1939
        device_id: "0x8B"
        interface: vcan0
        protocol: j1939
        websocket_port: 10756
        enabled: true
      emp_test_device:
        friendly_name: "EMP Test Device"
        type: emp
        device_id: "0x82"
        interface: vcan0
        protocol: j1939
        enabled: true

  webui-simple:
    friendly_name: "WebUI Simple Test Environment"
    enabled: true
    devices:
      test_fan:
        friendly_name: "EMP Test Fan"
        type: emp
        device_id: "0x82"
        variant: fan
        interface: vcan0
        protocol: j1939
        websocket_port: 10754
        enabled: true
```

</text>

<old_text line=1029>
**Helper Functions (Use These):**

```bash
# Configuration loading
load_test_config "cando.yaml" "tier2-virtual"

# Device queries
get_device_info "J1939 Test ECU"      # Full device JSON
get_device_port "J1939 Test ECU"      # WebSocket port
get_device_id "J1939 Test ECU"        # Device ID
get_device_interface "J1939 Test ECU" # CAN interface

# Environment queries
list_environment_devices               # All devices in environment
get_environment_interface              # Environment CAN interface

# Simulator management
start_simulator_by_name "J1939 Test ECU"
stop_simulator "$PID"
```

**Helper Functions (Use These):**

```bash
# Configuration loading
load_test_config "cando.yaml" "tier2-virtual"

# Device queries
get_device_info "J1939 Test ECU"      # Full device JSON
get_device_port "J1939 Test ECU"      # WebSocket port
get_device_id "J1939 Test ECU"        # Device ID
get_device_interface "J1939 Test ECU" # CAN interface

# Environment queries
list_environment_devices               # All devices in environment
get_environment_interface              # Environment CAN interface

# Simulator management
start_simulator_by_name "J1939 Test ECU"
stop_simulator "$PID"
```

**Implementation Guidelines:**

1. **Configuration First (MANDATORY)**
   - ALL test parameters MUST be defined in `cando.yaml` configuration file
   - Use `--environment` flag to select test environment (e.g., `tier2-virtual`, `webui-simple`, `physical-lab`)
   - All environments defined in single consolidated `cando.yaml` file
   - Include ALL device parameters: ID, interface, type, variant, ports, protocol
   - Use descriptive device names (e.g., "J1939 Test ECU", "Test Fan")

2. **Use Helper Functions (REQUIRED)**
   - ALWAYS source `config_helpers.sh` at start of test script
   - ALWAYS use `load_test_config()` before any device access
   - Query ALL device info via helper functions
   - NEVER hardcode device parameters in script
   - NEVER duplicate configuration values from TOML into bash variables

3. **Environment Selection (REQUIRED)**
   - ALL test scripts MUST support `--environment` or `$ENVIRONMENT` variable
   - Default to appropriate environment (`tier2-virtual`, `webui-simple`, etc.)
   - Use `tier2-physical` for hardware testing
   - Pass environment to all configuration queries

4. **Discovery Over Enumeration (BEST PRACTICE)**
   - Use `list_environment_devices` to discover devices
   - Iterate over discovered devices, don't hardcode list
   - Scripts automatically adapt to config changes
   - No "magic" device names in scripts

5. **Prohibitions (ABSOLUTE)**
   - ❌ NEVER write: `DEVICE_ID="0x82"` in a test script
   - ❌ NEVER write: `INTERFACE="vcan0"` in a test script
   - ❌ NEVER write: `PORT="10752"` in a test script
   - ❌ NEVER write: `TYPE="fan"` in a test script
   - ❌ NEVER write: `PROTOCOL="j1939"` in a test script
   - ✅ ALWAYS use: `DEVICE_ID=$(get_device_id "Test Fan")`
   - ✅ ALWAYS use: `INTERFACE=$(get_device_interface "Test Fan")`
   - ✅ ALWAYS use: `VARIANT=$(get_device_variant "Test Fan")`

6. **Parallel Implementation (MIGRATION)**
   - When migrating existing scripts, create new config-driven version
   - Keep old script for comparison during transition
   - Replace old script only after validation complete

**🔑 CRITICAL: All Cando Binaries Use Direct Configuration Loading**

**Key Architectural Principle**: All Cando binaries (WebUI, simulators, tools) use the **same configuration pattern** via `CommonSimulatorArgs` from `cando-simulator-common`. They read `cando.yaml` directly - **NO query-then-pass pattern needed**.

**Configuration Precedence (Built into all binaries)**:

1. CLI arguments (highest priority)
2. Environment variables
3. Device-specific config from TOML (via `--device-name`)
4. Environment-scoped device (via `--environment`)
5. Global defaults from TOML
6. Built-in defaults (lowest priority)

**Device Name Scoping**: Device names are **environment-scoped** (unique within an environment, not globally unique).

**Example Test Script Structure:**

```bash
#!/bin/bash
# Config-driven test script - binaries read config directly
#
# Usage:
#   ./test_script.sh [CONFIG_FILE] [ENVIRONMENT]
#
# Examples:
#   ./test_script.sh cando.yaml tier2-virtual
#   ./test_script.sh cando.yaml webui-simple

set -e

# Accept command-line arguments with sensible defaults
CONFIG_FILE="${1:-cando.yaml}"
ENVIRONMENT="${2:-webui-simple}"

# ✅ CORRECT: Simulator reads config directly (like WebUI does)
cargo run -p emp-simulator -- \
    --config "${CONFIG_FILE}" \
    --environment "${ENVIRONMENT}" \
    --no-console &
SIMULATOR_PID=$!

# Start WebUI with same pattern
cargo run -p cando-webui -- \
    --cando-config "${CONFIG_FILE}" \
    --environment "${ENVIRONMENT}" &
WEBUI_PID=$!

# Run tests...
# ... test logic here ...

# Cleanup
kill ${SIMULATOR_PID} ${WEBUI_PID} 2>/dev/null || true
```

**Multi-Device Test Example:**

```bash
#!/bin/bash
# Start multiple devices from same environment

CONFIG_FILE="cando.yaml"
ENVIRONMENT="webui-simple"

# Start WebUI
cargo run -p cando-webui -- \
    --cando-config "${CONFIG_FILE}" \
    --environment "${ENVIRONMENT}" &
WEBUI_PID=$!

# Start Test Fan (by device name - most specific)
cargo run -p emp-simulator -- \
    --config "${CONFIG_FILE}" \
    --device-name "Test Fan" \
    --no-console &
FAN_PID=$!

# Start Test Pump (by device name)
cargo run -p emp-simulator -- \
    --config "${CONFIG_FILE}" \
    --device-name "Test Pump" \
    --no-console &
PUMP_PID=$!

# Wait for user testing...
echo "Press Enter to stop all services..."
read

# Cleanup
kill ${WEBUI_PID} ${FAN_PID} ${PUMP_PID} 2>/dev/null || true
```

**❌ DEPRECATED PATTERN - DO NOT USE:**

```bash
# WRONG: Query-then-pass pattern (violates direct config loading)
DEVICE_ID=$(get_device_id "Test Fan")
INTERFACE=$(get_device_interface "Test Fan")
VARIANT=$(get_device_variant "Test Fan")

emp-simulator --device-id ${DEVICE_ID} \
              --device-type ${VARIANT} \
              --interface ${INTERFACE}

# This bypasses the configuration precedence system and creates
# unnecessary indirection. Simulators should read config directly.
```

**✅ CORRECT PATTERN - USE THIS:**

```bash
# Simulator reads config directly (same as WebUI)
emp-simulator --config cando.yaml --device-name "Test Fan"

# Or by environment (takes first device)
emp-simulator --config cando.yaml --environment webui-simple

# With CLI overrides (precedence: CLI > Config > Defaults)
emp-simulator --config cando.yaml --device-name "Test Fan" --interface can0
```

**When to Use Helper Functions:**

Helper functions (`config_helpers.sh`) are ONLY for:

- **Validation**: Verify config file exists and is valid
- **Discovery**: List available devices/environments dynamically
- **Assertions**: Test that specific devices are configured
- **Display**: Show configuration to user for confirmation

**NOT for:**

- ❌ Extracting values to pass to binaries (binaries read config directly)
- ❌ Duplicating configuration data into bash variables
- ❌ Creating intermediate query-then-pass workflows

**Reference Documentation:**

For complete examples and all startup patterns, see:

- **[CONFIGURATION-GUIDE.md](CONFIGURATION-GUIDE.md)** - Comprehensive configuration usage guide
  - Starting WebUI (multiple patterns)
  - Starting Simulators (by environment, by device name, with overrides)
  - Multi-device startup examples
  - Device name scoping rules
  - CLI override precedence

**Makefile Integration:**

```makefile
# Config-driven test targets
tier2-config: build-all setup-can-privileges
	CONFIG_FILE=$${CONFIG_FILE:-cando.yaml} \
	ENVIRONMENT=$${ENVIRONMENT:-tier2-virtual} \
	./scripts/integration/integration_test_all_protocols_config.sh

tier2-config-virtual: build-all setup-can-privileges
	CONFIG_FILE=cando.yaml \
	ENVIRONMENT=tier2-virtual \
	./scripts/integration/integration_test_all_protocols_config.sh
```

**When to Use This Pattern:**

- ✅ **MUST use** for ALL integration test scripts in `scripts/integration/`
- ✅ **MUST use** for ALL tier1/tier2 validation tests
- ✅ **MUST use** for ALL WebUI test scripts
- ✅ **MUST use** for ALL simulator test scripts
- ✅ **MUST use** for ALL long-lived testing features
- ✅ **MUST use** when test topology changes frequently
- ✅ **MUST use** when multiple test environments needed
- ⚠️ **MAY skip** for trivial throwaway debug scripts (< 20 lines, temporary)
- ⚠️ **MAY skip** for CLI tool tests that explicitly test command-line argument parsing
- ❌ **NEVER skip** for any script committed to the repository

**Migration Path:**

When updating existing test scripts:

1. Create config entries in `cando.yaml` (add devices and environments as needed)
2. Create new `*_config.sh` version of test script
3. Use helper functions to query config
4. Validate new script works correctly
5. Keep old script for comparison
6. Replace old script after validation complete

**Documentation:**

See `doc/testing/CONFIGURATION-DRIVEN-TESTING.md` for comprehensive
implementation guide, migration examples, best practices, and troubleshooting.

### CAN Hardware Constraints

**CRITICAL**: This development environment has **NO physical CAN hardware**.

**Testing MUST use**:

- ✅ Virtual CAN (`vcan0`) - Software-only CAN interface
- ✅ Unit tests with mock CAN sockets
- ✅ Simulator-based integration tests

**NEVER**:

- ❌ Assume physical CAN hardware availability
- ❌ Hardcode physical interface names (can0, can1)
- ❌ Require real CAN devices for testing
- ❌ Skip validation due to "missing hardware"

**Tier 2 Testing**:

- Creates and uses `vcan0` virtual interface
- All simulators bind to `vcan0`
- All tests run without physical hardware
- WebSocket APIs allow external integration if needed

---

## 📚 Documentation Standards

### AI Context Documents

**These live in `doc/` and are CRITICAL for AI context restoration**:

1. `AI_CONTEXT_RESTORATION.md` - Master architectural overview
2. `AI_PROJECT_STANDARDS.md` - This file, project standards
3. `AI-HOW-TO-ADD-J1939-MESSAGES.md` - Implementation guide
4. `AI-HOW-TO-ADD-J1939-73-DIAGS.md` - Diagnostic guide

**Rules**:

- ✅ Keep these updated with architectural changes
- ✅ Use these as single source of truth
- ✅ Reference these in `resume.md` for AI context loading
- ❌ NEVER create conflicting documentation
- ❌ NEVER let these become stale

### Code Documentation

**Inline Documentation**:

````rust
/// Brief one-line summary
///
/// Detailed explanation of the function/struct
///
/// # Example
///
/// ```
/// use cando_messages::j1939::DM01;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let msg = DM01::decode_real(0x18FECA00, &[0xFF; 8])?;
/// assert_eq!(msg.device_id, DeviceId::Device8A);
/// # Ok(())
/// # }
/// ```
pub fn decode_real(...) -> Result<...> {
````

**Requirements**:

- ✅ All public APIs have doc comments
- ✅ All doc examples are executable (verified by `cargo test --doc`)
- ✅ Use `# ` prefix for hidden test setup lines
- ✅ Examples show realistic usage patterns

### README Updates

When adding features that affect user-facing functionality:

1. Update `README.md` with new capabilities
2. Add usage examples to appropriate sections
3. Update feature lists and statistics
4. Maintain consistency with existing format

---

## 🎯 Code Quality Standards

### Zero Tolerance Policy

**NEVER commit code with**:

- ❌ Compiler warnings
- ❌ Failing tests
- ❌ Ignored tests without documented justification
- ❌ Clippy warnings (at default level)
- ❌ Formatting violations

### Quality Gates

**Every commit must pass**:

```bash
cargo build --workspace          # Zero warnings
cargo test --workspace           # 100% pass rate
cargo clippy -- -D warnings      # Zero clippy warnings
make tier1                       # 48/48 tests passing
```

**Before major releases**:

```bash
make tier2                       # 22/22 full-stack tests
cargo doc --no-deps              # Documentation builds
```

### Code Formatting

- ✅ Use `rustfmt` with default settings
- ✅ Maximum line length: 100 characters (rustfmt default)
- ✅ Consistent indentation (4 spaces, enforced by rustfmt)

### Error Handling

- ✅ Use `Result<T, E>` for fallible operations
- ✅ Custom error types in `cando-messages/src/common/error.rs`
- ✅ Meaningful error messages with context
- ❌ NEVER use `.unwrap()` in production code
- ❌ NEVER silently ignore errors

---

## 🔄 Context Restoration Protocol

### MANDATORY: Single-Document Context Restoration

**CRITICAL PRINCIPLE**: Context restoration must use a SINGLE self-contained document.

**Why**: Human brains struggle with 3+ dependent variables. Multiple documents requiring cross-referencing create cognitive load and increase the chance of missing critical information.

**Standard Pattern**: `RESUME-<feature>.md` or `resume-<branch-name>.md` in project root

### For AI Assistants Starting a New Session

**Step 1: Read ONE Primary Context Document**:

- Read the single resume document top-to-bottom
- Everything essential must be in that ONE document
- Optional references to detailed docs are okay, but NOT required reading
- The resume document must be complete and self-contained

**Example Resume Document Names**:

- `RESUME-PHASE2D.md` - Current work resumption
- `resume-tier2-investigation.md` - Branch/feature context
- `RESUME.md` - General project resumption

**Step 2: Read Mandatory Standards** (only these):

1. `doc/AI_CRITICAL_REMINDERS.md` - Absolute workflow rules
2. `doc/AI_PROJECT_STANDARDS.md` - This document

**Step 3: Validate Current State**:

```bash
# Check git status
git status
git log -5

# Validate tests pass
make tier1
```

**Step 4: Confirm Understanding**:

- Summarize project state to user
- Identify any issues or concerns
- Ask clarifying questions if needed

### When Creating Context Restoration Documents

**MANDATORY Requirements**:

1. **Single Document Rule**
   - Create ONE consolidated resume document
   - Everything essential must be in this document
   - Read top-to-bottom, linear, no jumping between files
   - Optional references to detailed docs are acceptable but NOT dependencies

2. **Location**: Project root (for active work) or `doc/session-restoration/` (for historical reference)

3. **Naming**:
   - Active work: `RESUME-<feature>.md` (e.g., `RESUME-PHASE2D.md`)
   - Branch work: `resume-<branch-name>.md`
   - Historical: `doc/session-restoration/YYYY-MM-DD-<feature>-session.md`

4. **Content Structure** (all in ONE document):

   ```markdown
   # Title - Complete Context Restoration

   **Current Situation**: What's done, what's next

   ## IMMEDIATE ACTIONS

   - What to do right now (commands to run)

   ## WHAT WAS ACCOMPLISHED

   - Complete context of work done
   - Key decisions made
   - Files created/modified

   ## KEY DESIGN DECISIONS

   - Important architectural choices
   - Rationale for each decision

   ## IMPORTANT FILES

   - List of files and their purposes

   ## CRITICAL REMINDERS

   - Workflow rules (git, testing, etc.)

   ## COMPLETION CRITERIA

   - What defines "done"

   ## WHAT TO DO RIGHT NOW

   - Immediate next steps
   - Commands to execute

   ## ADDITIONAL DOCUMENTATION (Optional)

   - References to detailed docs if needed
   - But reader should NOT need to read these
   ```

5. **Anti-Pattern to AVOID**:
   - ❌ "Read these 5 documents in order"
   - ❌ "See doc X for details on Y"
   - ❌ Multiple dependent resume files
   - ❌ "Context is split across multiple files"
6. **Correct Pattern**:
   - ✅ Single document with all essential information
   - ✅ Linear reading, top-to-bottom
   - ✅ Optional pointers to detailed docs at end
   - ✅ Reader can start working after reading ONE file

7. **Reference Updates**:
   - If creating historical record, optionally note in `resume.md`
   - But primary context must be self-contained in the single document

### Rationale

Human cognitive limits require simplification. Multiple dependent documents force:

- Context switching between files
- Mental tracking of dependencies
- Higher chance of missing critical information
- Increased cognitive load

Solution: ONE self-contained document that can be read linearly from top to bottom.

---

## 🚨 Common Pitfalls to Avoid

### File Organization

- ❌ Creating temporary files in root directory
- ❌ Leaving debug output files uncommitted and cluttering workspace
- ❌ Not organizing feature documents into subdirectories

### Testing

- ❌ Skipping tier1 validation before commits
- ❌ Assuming physical CAN hardware is available
- ❌ Ignoring test failures as "not important"

### Development

- ❌ Making changes without understanding existing patterns
- ❌ Ignoring compiler warnings as "minor issues"
- ❌ Skipping documentation updates after code changes

### Context Management

- ❌ Not reading context documents before starting work
- ❌ Creating conflicting documentation
- ❌ Forgetting to update `doc/AI_CONTEXT_RESTORATION.md` after major changes

---

## ✅ Pre-Commit Checklist

Before committing ANY code, verify:

- [ ] `make tier1` passes (48/48 tests, 100% success)
- [ ] Zero compiler warnings (`cargo build --workspace`)
- [ ] Zero clippy warnings (`cargo clippy`)
- [ ] Root directory is clean (only allowed files present)
- [ ] New documentation is properly organized in `doc/`
- [ ] New scripts are in appropriate `scripts/` subdirectory
- [ ] Context documents updated if architecture changed
- [ ] `resume.md` updated with completion status and metrics
- [ ] `COMMIT-READY.md` created or updated in feature doc/ subdirectory
- [ ] `commit.msg` written with comprehensive details
- [ ] Git commit message is descriptive and follows conventions

---

## 📖 Quick Reference

### File Placement Decision Tree

```
Is it a script (.sh, .py)?
  → YES: scripts/<appropriate-subdir>/

Is it documentation (.md, .txt)?
  → YES: Is it README.md, resume.md, commit.msg, or AI_*.md?
    → YES: Keep in root
    → NO: Is it persistent project documentation?
      → YES: doc/<logical-subdir>/
        Examples:
        - Testing docs → doc/testing/
        - Feature completion → doc/<feature-name>/
        - Session notes → doc/session-restoration/
      → NO: Is it temporary/generated?
        → YES: target/ or appropriate temp directory

Is it data/output (.json, .csv, .out, .log)?
  → YES: Is it persistent reference data?
    → YES: Create/use descriptive directory (e.g., live.can.dumps/)
    → NO: Is it test/benchmark output?
      → YES: target/ (gitignored) or benchmarks/reports/

Is it core project file (Cargo.toml, Makefile, .gitignore)?
  → YES: Keep in root

OTHERWISE:
  → Find or create appropriate subdirectory
  → If in doubt, ask about proper location
```

### Documentation Subdirectory Creation Guidelines

**When to create a new subdirectory in doc/**:

1. **Topic has 3+ related documents**: Create subdirectory
   - Example: `doc/testing/` for phase3-complete.md, physical-integration-\*.md, etc.

2. **Feature-specific documentation**: Use feature name
   - Example: `doc/j1939-batch-implementation/` for J1939 tracking docs

3. **Recurring document pattern**: Create category subdirectory
   - Example: `doc/session-restoration/` for session notes

**Naming Conventions**:

- Use lowercase with hyphens: `feature-name/` not `Feature_Name/`
- Be descriptive but concise: `testing/` not `test-docs-and-validation/`
- Match feature/component names when applicable
- Use plural for categories: `validation-history/` not `validation-histories/`

**Anti-patterns to Avoid**:

- ❌ Creating subdirectory for single document (unless it's clearly the first of a series)
- ❌ Generic names like `misc/`, `docs/`, `files/` (be specific)
- ❌ Deeply nested structures: `doc/a/b/c/file.md` (keep flat)
- ❌ Mixing unrelated documents in same subdirectory

### Key Commands

```bash
# Validate current state (with logging)
make tier1 2>&1 | tee logs/tier1-$(date +%Y%m%d-%H%M%S).log
make tier2 2>&1 | tee logs/tier2-$(date +%Y%m%d-%H%M%S).log

# Build and test (with logging)
cargo build --workspace 2>&1 | tee logs/build-$(date +%Y%m%d-%H%M%S).log
cargo build -p <crate> 2>&1 | tee logs/build-<crate>-$(date +%Y%m%d-%H%M%S).log
cargo test --workspace 2>&1 | tee logs/test-$(date +%Y%m%d-%H%M%S).log
cargo test -p <crate> 2>&1 | tee logs/test-<crate>-$(date +%Y%m%d-%H%M%S).log

# Quick checks (logging optional for iteration)
cargo clippy                  # Lint check
cargo check                   # Fast compilation check

# Code generation
make codegen-generate         # Regenerate from DBC files
make codegen-status           # Check generation status
```

# Organization audit

ls -1 _.md _.sh _.txt _.json _.csv _.out 2>/dev/null | \
 grep -v "README.md\|resume.md\|commit.msg"

# Should output nothing if organized properly

```

---

## 🎯 Summary

**Three Golden Rules for AI Assistants**:

1. **Keep Root Clean**: Only `README.md`, `resume.md`, `commit.msg`, and core build files
2. **Organize Everything**: Scripts → `scripts/`, Docs → `doc/`, Data → specific subdirs
3. **Test Everything**: `make tier1` must pass 100% before any commit

**Before ANY work**:
- Read context docs
- Validate with `make tier1`
- Check git status

**After ANY work**:
- Organize files properly
- Run `make tier1` (and `make tier2` for major changes)
- Update documentation

**Remember**: This project has NO physical CAN hardware. All testing uses `vcan0`.

---

**Document Status**: ✅ AUTHORITATIVE
**Compliance**: MANDATORY for all AI assistants
**Updates**: Should be rare; reflects permanent project standards
```
