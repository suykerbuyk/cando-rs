# AI Workflow Guide for Cando-RS

**Purpose**: Consolidated workflow guide for AI assistants working on the Cando-RS project  
**Status**: Authoritative - All AI assistants MUST follow these guidelines  
**Last Updated**: 2024-12-24

---

## 🚨 QUICK REFERENCE - CRITICAL RULES

**Before ANY work, remember these rules:**

1. ❌ **NEVER commit without explicit approval** - "commit it", "yes commit", "go ahead"
2. ✅ **ALWAYS log commands with tee** - `cargo test 2>&1 | tee logs/test_$(date +%Y%m%d_%H%M%S).log`
3. ✅ **PAUSE at 75% token budget** - Stop at 150K tokens to prepare context handoff
4. ✅ **ONLY user declares "done"** - Never assume work is complete
5. ❌ **NEVER stage commit.msg** - Create it, but leave untracked
6. ✅ **Investigation first, implementation after approval**
7. ✅ **Update RESUME.md** - Avoid creating superfluous documents
8. ✅ **Zero warnings policy** - Must maintain across all packages
9. ✅ **Session Closeout required** - Clean up ephemeral docs before declaring "done"

---

##  🤝 PAIR PROGRAMMING PRINCIPLES

### The AI's Role: Expert Implementation
- Expert coder with deep technical knowledge
- Rapid analysis and systematic investigation
- Implements solutions after architectural approval
- Writes comprehensive, maintainable code

### The Human's Role: Architectural Vision
- Context across entire project history
- Understanding of long-term maintainability goals
- Knowledge of project principles and design patterns
- Ensures changes align with whole system

### Critical Anti-Pattern: Premature Implementation

❌ **NEVER: Jump to coding without investigation**

**Why This Harms**:
- Loses project maintainability
- Violates shared principles
- Creates technical debt
- Fixes symptoms instead of root causes

✅ **ALWAYS: Investigate → Analyze → Present → Get Approval → Implement**

---

## 🔍 INVESTIGATION-FIRST WORKFLOW

### Step 1: Investigate Thoroughly
- Gather all relevant information
- Test hypotheses systematically
- Use logging for reproducibility
- Document findings as you go

### Step 2: Analyze Root Causes
- Don't stop at symptoms - find the real problem
- Consider multiple hypotheses
- Verify with tests and evidence
- Understand full scope of issue

### Step 3: Present Findings
- Create comprehensive summary
- Explain what you found and why
- Present multiple solution options
- Recommend approach with reasoning
- **STOP AND WAIT FOR APPROVAL**

### Step 4: Get Architectural Approval
- Human reviews findings and recommendations
- Human considers project-wide implications
- Human approves approach or suggests alternatives
- Only proceed when explicitly approved

### Step 5: Implement the Solution
- Follow the approved approach
- Maintain project standards and patterns
- Write maintainable, well-documented code
- Test thoroughly

---

## 🎯 SOURCE OF TRUTH HIERARCHY

When resolving conflicts, follow this priority:

### 1. 🥇 HIGHEST: Live Hardware Capture Logs
**Location**: `live.can.dumps/*.log`  
**Why**: Reality cannot be wrong. Real CAN messages from actual hardware are ultimate ground truth.

### 2. 🥈 SECOND: DBC Files
**Location**: `dbc/*.dbc`  
**Why**: Assumed correct after validation against hardware captures.

### 3. 🥉 THIRD: Vendor Documentation
**Location**: `doc/*Idd-*.md`  
**Why**: Reference for understanding intent, but may contain errors.

### 4. LOWEST: Test Expectations
**Why**: Tests are code. Code has bugs. Tests can be wrong.

---

## 📁 FILE ORGANIZATION STANDARDS

### Root Directory Rules

**ONLY these in project root:**
1. Core: `README.md`, `Cargo.toml`, `Cargo.lock`, `Makefile`, `.gitignore`
2. AI context: `RESUME.md`, `CLAUDE.md`
3. Configuration: `cando.yaml`
4. Temporary (untracked): `commit.msg`

### Script Organization

All scripts in `scripts/` subdirectories:
- `scripts/integration/` - Integration tests
- `scripts/dev-tools/` - Development utilities
- `scripts/testing/` - Test harness scripts

### Documentation Organization

All docs in `doc/` with topic subdirectories:
- `doc/webui/` - WebUI phase docs
- `doc/sessions/` - Historical sessions
- `doc/phases/` - Phase overviews

---

## 🔧 GIT WORKFLOW

### Staging Files (AI Authorized)

```bash
git add <file1> <file2>
git status
git diff --cached --stat
```

**Rules**:
- ✅ Stage all files you create/modify
- ✅ Stage documentation with code
- ❌ Never stage `commit.msg`
- ❌ Never stage temporary files

### Commit Authority

**⛔ AI MUST NEVER EXECUTE `git commit` ⛔**

**Workflow**:
1. AI stages files: `git add <files>`
2. AI creates `commit.msg`
3. AI reports: "Files staged, ready for review"
4. User reviews: `git diff --cached`
5. User commits: `git commit -F commit.msg`

### commit.msg Format

```
type(scope): Brief summary (50 chars)

Detailed description.

## What Was Accomplished
- Item 1
- Item 2

## Testing Status
- ✅ 437/437 tests passing

Branch: feature/name
```

---

## 🧪 TESTING REQUIREMENTS

**All commands MUST use tee:**

```bash
cargo test 2>&1 | tee logs/test_$(date +%Y%m%d_%H%M%S).log
make tier1 2>&1 | tee logs/tier1_$(date +%Y%m%d_%H%M%S).log
```

**Requirements**:
- Zero warnings policy
- 100% test pass rate
- No regressions
- Configuration-driven (no hardcoded values)

---

## 📝 DOCUMENTATION STRATEGY

**❌ DO NOT CREATE:**
- Session summaries as separate files
- Per-session status documents
- Temporary progress trackers

**✅ ALWAYS:**
- Update RESUME.md with status
- Update ONE canonical feature document
- Consolidate context in existing documents

**Question Before Creating**:
- Is this >500 lines of substantial content?
- Will this be referenced for weeks/months?
- Does RESUME.md already cover this?

---

## 🔄 CONTEXT SWITCH PROCEDURE

**5-step procedure executed in order:**

### Step 1: Update Feature Document
One feature = one document in `doc/`

### Step 2: Update RESUME.md
Add session summary to "Recent Sessions"

### Step 3: Stage ALL Relevant Files
```bash
git add src/main.rs doc/FEATURE.md RESUME.md
git status --short
```

### Step 4: Create commit.msg
Write comprehensive message (leave untracked)

### Step 5: Present and STOP
Show summary, staged files, commit message
**WAIT** for user approval

---

## 🧹 SESSION CLOSEOUT PROCEDURE

**Call Name**: "Session Closeout" or "Perform closeout"

### The Principle

**A feature is NOT done until ephemeral artifacts are cleaned up.**

During implementation, we generate temporary files, debug tools, investigation documents, 
test scripts, and other artifacts. Before declaring work complete, these MUST be properly 
handled to maintain project hygiene and documentation quality.

### Why This Matters

- **Prevents documentation clutter** - Keeps `doc/` navigable and purposeful
- **Reduces noise in git status** - Clear signal vs noise
- **Maintains project standards** - Root directory stays clean
- **Preserves valuable context** - Consolidates insights, archives history
- **Enables future AI assistants** - Clear, consolidated documentation

### When to Perform Session Closeout

**REQUIRED before declaring work "done":**
- Feature implementation complete
- Tests passing
- Code committed and merged
- User says "this is done" or equivalent

**FORBIDDEN:**
- During active development
- Before user approval of work
- When context switch is needed urgently

### Closeout Checklist

**Step 1: Identify Ephemeral Artifacts**

Run `git status` and categorize untracked files:

```bash
git status --short
```

Look for:
- Investigation/findings documents in root or `doc/`
- Session-specific summaries
- Test scripts created for this session only
- Debug tools (like viewport indicators, diagnostic scripts)
- Backup files (*.backup, *.bak, *.old)
- Temporary dependencies (node_modules/, package.json for one-off use)
- Screenshot captures and test output
- Proof-of-concept code

**Step 2: Delete Temporary/Debug Files**

Files with NO long-term value:
- Backup files created during session
- Debug/diagnostic tools added temporarily
- node_modules/ if not needed ongoing
- Test screenshots (unless documenting bugs)
- Proof-of-concept scripts

```bash
rm -rf node_modules/ package-lock.json package.json
rm *.backup test_*.js capture_*.js
git status  # Verify cleanup
```

**Step 3: Consolidate Documentation**

Multiple small docs → ONE canonical document:
- Merge investigation findings into main feature doc
- Combine session summaries into comprehensive guide
- Update existing docs rather than creating new ones

**Example:**
```
SESSION-49-FIX-WEBUI-SCALING-SUMMARY.md (292 lines)
++ SESSION-49-FIX-WEBUI-SCALING-INVESTIGATION.md (513 lines)
++ SESSION-49-FIX-WEBUI-SCALING-IMPLEMENTATION.md (640 lines)
→ doc/webui/WEBUI-RESPONSIVE-DESIGN.md (consolidated)
```

**Naming Convention for Session Docs:**
- Format: `SESSION-NN-TYPE-FEATURE-NAME-PURPOSE.md`
- NN = Session number
- TYPE = FIX, FEAT, REFACTOR, etc.
- FEATURE-NAME = Brief feature/component identifier
- PURPOSE = SUMMARY, INVESTIGATION, IMPLEMENTATION, etc.
- Example: `SESSION-49-FIX-WEBUI-SCALING-SUMMARY.md`

**Step 4: Archive Session-Specific Documents**

Historical context with permanent value:
- Move to `doc/sessions/SESSION-XX-NAME.md`
- Update `doc/sessions/README.md` with entry
- Cross-reference from main docs

**Step 5: Keep Substantial Reference Docs**

Documents that stay in main `doc/`:
- >500 lines of substantial technical content
- Will be referenced for weeks/months
- Serves as canonical reference
- Not session-specific

**Step 6: Update RESUME.md**

Remove references to ephemeral files:
- Update "Files Created" sections
- Remove debug/temporary tool mentions
- Link to consolidated/archived docs instead

**Step 7: Verify Clean State**

```bash
git status --short
# Should show only:
# - Legitimate new features/scripts with ongoing value
# - Documentation that meets "keep" criteria
# - Configuration files for permanent features
```

### Decision Tree: Keep, Consolidate, Archive, or Delete?

**DELETE if:**
- Temporary debug/diagnostic tool
- Backup file
- Dependencies for one-off testing
- No long-term value

**CONSOLIDATE if:**
- Multiple docs covering same topic
- Session summaries that should be in main doc
- Investigation findings that belong in feature doc

**ARCHIVE if:**
- Session-specific historical context
- Useful for understanding decisions
- Referenced in commit messages
- >200 lines of substantial content

**KEEP if:**
- >500 lines of substantial reference material
- Canonical documentation for feature
- Ongoing utility (scripts, tests)
- Referenced by multiple other docs

### Invocation

When user says:
- "Perform session closeout"
- "Clean up ephemeral docs"
- "Do the closeout procedure"

You should:
1. Run through the 7-step checklist above
2. Present deletion/consolidation/archival plan
3. **WAIT for approval**
4. Execute approved plan
5. Verify clean `git status`
6. Update RESUME.md

### Example Session Closeout

**Before:**
```
?? SESSION-49-FIX-WEBUI-SCALING-SUMMARY.md
?? doc/webui/SESSION-49-FIX-WEBUI-SCALING-INVESTIGATION.md
?? doc/webui/SESSION-49-FIX-WEBUI-SCALING-IMPLEMENTATION.md
?? capture_screenshots.js
?? test_layout_quick.js
?? test_layout_multi_res.js
?? node_modules/
?? package.json
?? cando-webui/static/css/style.css.backup
?? start_test_env.sh
```

**After:**
```
?? doc/webui/WEBUI-RESPONSIVE-DESIGN.md (consolidated from 3 docs)
?? tests/webui/widget-dimensions.spec.js (permanent test)
?? playwright.config.js (permanent test infrastructure)
```

**Deleted:**
- capture_screenshots.js (debug tool)
- test_layout_quick.js (temporary test)
- test_layout_multi_res.js (temporary test)
- node_modules/ (one-off dependency)
- package.json (one-off)
- style.css.backup (backup file)
- start_test_env.sh (redundant with existing test scripts)
- SESSION-49-FIX-WEBUI-SCALING-SUMMARY.md (consolidated)

**Consolidated:**
- Three SESSION-49-FIX-WEBUI-SCALING-* docs → doc/webui/WEBUI-RESPONSIVE-DESIGN.md

### Anti-Patterns

❌ **DON'T:**
- Delete docs before user declares work "done"
- Archive everything (creates archive bloat)
- Keep everything (creates clutter)
- Rush closeout during active development
- Delete without user approval

✅ **DO:**
- Wait for explicit "done" signal
- Be ruthless with temporary files
- Consolidate aggressively
- Preserve valuable context
- Get approval before deletion

---

## 📊 TOKEN BUDGET MANAGEMENT

- **0-75% (0-150K)**: Continue normally
- **75-85% (150K-170K)**: Finish task, then pause
- **85%+ (170K+)**: STOP immediately

**At 75%**: ASK user to continue or switch context

---

## ✅ PRE-COMMIT CHECKLIST

- [ ] Investigated before implementing?
- [ ] Got approval for approach?
- [ ] Ran tests with tee logging?
- [ ] All tests passing?
- [ ] Build clean (zero warnings)?
- [ ] Updated RESUME.md?
- [ ] Staged all relevant files?
- [ ] Created commit.msg (not staged)?
- [ ] Asking for approval (not assuming)?

## ✅ SESSION COMPLETION CHECKLIST

**Before declaring "done":**

- [ ] Code committed and merged to main?
- [ ] All tests passing?
- [ ] User declared work "done"?
- [ ] Performed session closeout?
- [ ] Ephemeral docs deleted/consolidated/archived?
- [ ] Git status clean (only intentional files)?
- [ ] RESUME.md updated with session summary?

---

## 🎯 KEY PHRASES

**User wants commit:**
- "commit it"
- "yes, commit"
- "go ahead"

**User NOT ready:**
- "looks good" (without "commit")
- "tests pass"

**When in doubt: ASK!**

---

**For detailed procedures**: See individual sections above  
**For current status**: See RESUME.md  
**For quick start**: See CLAUDE.md

