# 🚨 CRITICAL REMINDERS - READ FIRST 🚨

**Created**: 2024-11-05  
**Purpose**: Prevent violations of git and logging standards  
**Priority**: MANDATORY - Read before any work

---

## 🤝 AI/HUMAN PAIR PROGRAMMING PRINCIPLES

**Core Understanding**: This project requires a partnership where both AI and human bring unique, complementary strengths.

### The AI's Role: Expert Implementation

**Strengths**:

- Expert coder with deep technical knowledge
- Better than any human at writing correct, idiomatic code
- Can rapidly analyze code patterns and identify issues
- Excellent at systematic investigation and testing

**Responsibilities**:

- Investigate problems thoroughly BEFORE implementing fixes
- Present findings and action plans for review
- Implement solutions only after architectural approval
- Write comprehensive, maintainable code

### The Human's Role: Architectural Vision

**Strengths**:

- Context that envelopes the entire project across many days and iterations
- Understanding of long-term maintainability goals
- Knowledge of project principles and design patterns
- Ability to see how changes affect the whole system

**Responsibilities**:

- Guide architectural decisions
- Approve implementation approaches
- Maintain project-wide consistency
- Ensure changes align with long-term goals

### Critical Anti-Pattern: Premature Implementation

**❌ NEVER: Jump to coding short-term fixes without investigation**

**Why This Harms the Project**:

- Loses project maintainability
- Violates clear, shared, common principles across the codebase
- Creates technical debt
- Misses root causes in favor of symptoms
- May fix the wrong problem entirely

**✅ ALWAYS: Investigate → Analyze → Present → Get Approval → Implement**

### The Investigation-First Workflow

**Step 1: Investigate Thoroughly**

- Gather all relevant information
- Test hypotheses systematically
- Use logging to make investigations reproducible
- Document findings as you go

**Step 2: Analyze Root Causes**

- Don't stop at symptoms - find the real problem
- Consider multiple hypotheses
- Verify with tests and evidence
- Understand the full scope of the issue

**Step 3: Present Findings**

- Create comprehensive summary document
- Explain what you found and why
- Present multiple solution options
- Recommend an approach with reasoning
- **STOP AND WAIT FOR APPROVAL**

**Step 4: Get Architectural Approval**

- Human reviews findings and recommendations
- Human considers project-wide implications
- Human approves approach or suggests alternatives
- Only proceed when explicitly approved

**Step 5: Implement the Solution**

- Follow the approved approach
- Maintain project standards and patterns
- Write maintainable, well-documented code
- Test thoroughly

### Real Example: tier2 Investigation (2024-11-12)

**❌ What Would Have Been Wrong**:

- See simulators failing to start
- Immediately fix duplicate `no_console` field
- Commit the fix
- **Result**: Miss underlying J1939 decode bug, create incomplete solution

**✅ What Actually Happened**:

1. Investigated simulator startup failures
2. Fixed duplicate field issue
3. Re-ran tests with logging
4. Discovered J1939 field swapping bug
5. Traced through entire stack systematically
6. Created comprehensive investigation document
7. Presented findings and recommendations
8. **Waited for approval before implementing J1939 fix**
9. **Result**: Complete understanding, proper solution, maintainable fix

### Key Principle: Trust the Process

**AI Temptation**: "I see the bug, I can fix it in 5 minutes, let me just code it"

**Reality**:

- That 5-minute fix may not be the right fix
- You may not understand the full context
- The human may have critical information about why things are the way they are
- Quick fixes accumulate into unmaintainable code

**Better Approach**:

- Spend 30 minutes investigating thoroughly
- Document findings clearly
- Present to human for review
- Get approval
- Implement the RIGHT fix that maintains project principles

**Result**: Maintainable, coherent codebase that serves the project for years

---

## ✅ COMPLETION AND REVIEW AUTHORITY - CRITICAL

**The Golden Rule**: Nothing is ever "done" until the human explicitly says it is done.

**Why This Matters**:

- The human has architectural understanding that spans the entire project history
- The human has the curiosity and context to find edge cases AI cannot
- The human understands long-term maintainability implications
- The human sees how changes affect the whole system
- "All tests pass" ≠ "Implementation is correct"

**The AI's Responsibility**:

- Implement thoroughly and test rigorously
- Strive to acheive at least 80% unit test code coverage.
- A code change is not finished until there are no diagnostic warnings or clippy warnings.
- Present results clearly with context
- **ALWAYS WAIT** for explicit human approval before considering work complete
- If human finds issues, investigate root causes without defensiveness
- Understand that human feedback = better code, not criticism

**The Human's Authority**:

- Final decision on whether work is complete
- Right to find and report issues at any time
- Responsibility to articulate why something isn't done
- Authority to request changes, improvements, or alternative approaches

**Real Example - What NOT to do**:
❌ "I implemented the feature, all tests pass, work is done" (followed by: "Actually there are 7 issues...")

**Real Example - What TO do**:
✅ "I've implemented the feature, all tests pass, here's what I did. Please review for architectural correctness."

### ⚠️ CRITICAL: False Victory Laps and Premature "Done" Declarations

**The Problem**: Far too often, work is declared "done" based on false assumptions, creating mountains of faulty code built on incorrect foundations.

**The Pattern to AVOID**:

1. AI implements something
2. AI declares it "done" because tests pass
3. User discovers fundamental issues
4. Massive rework required, time wasted, technical debt created

**The CORRECT Pattern**:

1. **Document FIRST** - Ensure documentation is accurate before building code
2. **Implement** - Build according to accurate documentation
3. **Test thoroughly** - Verify behavior matches expectations
4. **PROVE it works** - Demonstrate functionality to user
5. **WAIT for user agreement** - User declares "done", not AI

**Critical Rules**:

- ❌ NEVER declare work "done" - only the user can declare completion
- ❌ NEVER assume tests passing = implementation correct
- ❌ NEVER build code before documentation is verified
- ✅ ALWAYS get documentation approved before implementation
- ✅ ALWAYS prove functionality works before asking for review
- ✅ ALWAYS wait for explicit user declaration of "done"

**Why This Matters**:

- Prevents building on false assumptions
- Saves massive rework time
- Maintains code quality and architectural integrity
- Ensures symbiotic development (WebUI + Simulator together)
- Avoids technical debt from rushed implementations

**Real-World Impact**:
When AI declares something "done" prematurely:

- User wastes time reviewing incomplete work
- Follow-on work builds on faulty foundation
- Rework cascades through multiple layers
- Project loses momentum and trust

**The Standard**:
Documentation accuracy → Implementation → Thorough testing → User verification → User declares "done"

---

## 📚 DOCUMENTATION STRATEGY - NO SUPERFLUOUS ARTIFACTS

**The Problem**: Too many ephemeral summary documents create clutter and confusion.

- Session summaries become outdated
- Multiple documents contradict each other
- Nobody can figure out which is authoritative
- Historical artifacts are useless after one or two commits

**The Solution**:

- **ONLY create documents that serve the long-term architecture**
- Architecture decisions → Long-lived docs (e.g., `FEATURE_NAME_IMPLEMENTATION_PLAN.md`)
- Ephemeral state → Update `RESUME.md` instead
- Session summaries → DELETE after commit (don't commit them)

**Examples of Documents To AVOID**:

- ❌ Session completion summaries
- ❌ "Status overview" documents from today's work
- ❌ Numbered phase completion documents (e.g., "Phase7-Complete-Report.md")
- ❌ Temporary progress trackers
- ❌ Multiple versions of the same spec

**Examples of Documents To KEEP**:

- ✅ Architecture decisions (e.g., why voltage is implemented this way)
- ✅ Protocol specifications (e.g., J1939 message definitions)
- ✅ Hardware reference docs (e.g., UDC IDD)
- ✅ Implementation guides for future work
- ✅ `RESUME.md` with current project state

**Rule**: If it won't still be useful 3 months from now, don't commit it.

---

## 🎯 RESUME.md AS SINGLE SOURCE OF TRUTH - CRITICAL

**The Principle**: `RESUME.md` is THE authoritative source for AI context restoration.

**What RESUME.md Contains**:

1. Current project status and branch state
2. What has been completed and what remains
3. Architecture decisions (with references to detailed docs)
4. File locations and structure
5. Ephemeral state (what was just done, what's in progress)
6. Links to long-lived architecture documents
7. Git commit messages documents specific code changes per session, RESUME.md documents feature or fix status across sessions.
8. RESUME.md references other documents for indepth details. Keep RESUME.md lean, let it reference other documents for in depth details.

**What RESUME.md Does NOT Replace**:

- Detailed implementation guides (those live in architecture docs)
- Protocol specifications (those live in spec docs)
- Hardware documentation (reference external sources)
- Code comments (those live in code)

**The Reconstruction Process**:

- Read `RESUME.md` to understand current state
- Follow references to architecture/specification docs for details
- Use git commit history to see decision rationale
- Combine these three sources = complete project understanding

**Ephemeral State Management**:

- After each AI session, update `RESUME.md` with what was done
- When work moves to a new phase, update `RESUME.md` status
- Git commit message + RESUME.md update = full record of what happened
- This approach means the next AI thread can restore complete context
- **RESUME.md history (git log of RESUME.md changes) IS the decision log**

**What This Prevents**:

- Multiple conflicting documentation artifacts
- Lost context when moving to new AI sessions
- Outdated summaries lying around
- Confusion about what is authoritative

**What This Enables**:

- Clean, organized documentation
- Easy context restoration between AI threads
- Git history that tells a coherent story
- RESUME.md as a living document that grows with the project

---

## 🎯 SOURCE OF TRUTH HIERARCHY - RESOLVING CONFLICTS

**When decoding/encoding issues arise, ALWAYS follow this priority:**

### 1. 🥇 HIGHEST: Live Hardware Capture Logs

**Location**: `live.can.dumps/*.log`

**Why**: Reality cannot be wrong. Real CAN messages from actual hardware are the ultimate ground truth.

**Example**: `live.can.dumps/udc-log-2025-12-05.log` shows actual bytes sent by GE UDC hardware.

**Usage**:

- Validate DBC correctness
- Verify test expectations
- Resolve vendor documentation discrepancies

### 2. 🥈 SECOND: DBC Files

**Location**: `dbc/*.dbc`

**Why**: Assumed correct after validation against hardware captures. Defines message structures and bit positions.

**Critical**: If decoder output doesn't match hardware capture, the DBC is WRONG, not the hardware.

**Example Discovery**: Status_1_Report hardware sends status bits in byte 0, but original DBC showed them in byte 1. DBC was corrected to match hardware.

### 3. 🥉 THIRD: Vendor Documentation

**Location**: `doc/UdcIdd-*.md`, `doc/HvpcIdd-*.md`

**Why**: Reference for understanding intent and field descriptions.

**WARNING**: May contain errors!

- Vendor doc showed Status_1_Report status bits in "Byte 2" (1-indexed)
- Actual hardware sends them in byte 0 (0-indexed)
- Vendor byte numbering was off by one

**Usage**:

- Understand field meanings
- Reference value descriptions
- Check parameter ranges
- **Always verify against hardware**

### 4. LOWEST: Test Expectations

**Why**: Tests are code. Code has bugs. Tests can be wrong.

**Rule**: If tests fail after DBC regeneration, assume tests are wrong, not the generated code.

**Action**: Update test expectations to match hardware reality, never modify DBC to match incorrect tests.

### Real Example from Session

**Conflict**: Test expected `lvVoltage = 6.2V` from specific bytes
**Vendor Doc**: Said status bits in "Byte 2"
**DBC**: Defined lvVoltage at bit 8, status at bit 0
**Hardware**: `19FF1C59#D8003E00007C00` decoded to lvVoltage = 0V, status in byte 0

**Resolution**:

- Hardware was correct
- Test expectation was wrong (fixed to expect 0V)
- Vendor doc byte numbering was wrong (noted in comments)
- DBC was validated correct after matching hardware

---

## ⚠️ ABSOLUTE RULES - NO EXCEPTIONS

</thinking>

### Rule #1: NEVER RUN GIT COMMANDS WITHOUT APPROVAL

**FORBIDDEN without explicit user approval:**

```bash
git commit
git push
git checkout (when switching branches)
```

**The user MUST explicitly say:**

- "commit it"
- "yes, commit"
- "go ahead and commit"
- "stage and commit"

**"The code is done" or "tests pass" is NOT approval to commit!**

---

### Rule #2: ALWAYS LOG LONG-RUNNING COMMANDS

**EVERY test, build, or long-running command MUST use tee:**

❌ **WRONG:**

```bash
cargo test -p cando-webui
cargo build
make tier1
```

✅ **CORRECT:**

```bash
cargo test -p cando-webui 2>&1 | tee logs/context_$(date +%Y%m%d_%H%M%S).log
cargo build 2>&1 | tee logs/build_$(date +%Y%m%d_%H%M%S).log
make tier1 2>&1 | tee logs/tier1_$(date +%Y%m%d_%H%M%S).log
```

**Log filename format:** `logs/<context>_$(date +%Y%m%d_%H%M%S).log`

Examples:

- `logs/phase7-tests_20241105_143022.log`
- `logs/manual-testing-replay_20241105_150033.log`
- `logs/build-release_20241105_151544.log`

---

### Rule #3: NEVER STAGE commit.msg FILE

**ALWAYS create commit.msg, but NEVER stage it:**

❌ **WRONG:**

```bash
git add commit.msg  # NEVER DO THIS!
git add .           # NEVER DO THIS! (stages commit.msg)
```

✅ **CORRECT:**

```bash
# Create commit.msg file
echo "feat: Add new feature" > commit.msg

# Stage only the actual code/doc files
git add Makefile doc/BUILD-SYSTEM.md

# Commit using the message file
git commit -F commit.msg

# commit.msg stays untracked - that's correct!
```

**WHY:** commit.msg is a temporary file for the current commit only. It should never be tracked in git.

**PROCESS:**

1. Create commit.msg with detailed message
2. Do NOT add hours of work to the accomplishments.
3. Do NOT leave any records or notes that anything was done by an AI.
4. Show it to user
5. Stage ONLY the actual changed files (never commit.msg)
6. Use `git commit -F commit.msg` to apply the message
7. Leave commit.msg untracked

---

### Rule #4: PAUSE AT 75% TOKEN BUDGET

**MANDATORY: Monitor token usage and pause when approaching limits**

**At 75% token usage (150,000 tokens out of 200,000):**

- ⏸️ STOP current work immediately
- Present current progress summary
- **ASK**: "We've used 75% of our token budget (150K tokens). Should I continue or prepare for context switch?"
- **WAIT** for user decision

**WHY:** Need to reserve tokens for:

- Updating RESUME.md with current status
- Updating implementation documents (e.g., FIELD-NAME-SNAKE-CASE-CONVERSION.md)
- Staging files if approved
- Creating commit.msg with detailed message
- Proper context handoff to next AI session
- Avoiding incomplete work without documentation

**Token Budget Guidelines:**

- 0-75% (0-150K): Continue working normally
- 75-85% (150K-170K): Finish current small task, then pause
- 85%+ (170K+): STOP immediately and prepare for context switch

**When in doubt about token usage: ASK for guidance!**

**Better to pause early than run out of tokens mid-documentation!**

---

### Rule #5: AVOID SUPERFLUOUS DOCUMENTATION - USE RESUME.md

**MANDATORY: Keep context in RESUME.md, not scattered across many files**

**❌ DO NOT CREATE:**

- Short-lived session summaries
- Per-session status documents
- Temporary progress tracking files
- Duplicate context documents
- "Session-DATE.md" files

**✅ ALWAYS:**

- Update RESUME.md with current status
- Update the ONE canonical implementation plan document (e.g., GE-UDC-IMPLEMENTATION-PLAN.md)
- Keep investigation findings in feature-specific docs only when substantial (>500 lines)
- Consolidate context in existing documents

**WHY:**

- Prevents document clutter
- Single source of truth for AI context restoration
- Easier to maintain and find information
- Reduces cognitive load for next AI session
- Follows project documentation consolidation principles

**Document Lifecycle:**

1. **RESUME.md**: Always current, always updated, primary AI entry point
2. **Implementation Plans**: One per feature, updated as phases complete
3. **Investigation Docs**: Created only for substantial analysis (like PHASE4-UTILITY-INVESTIGATION.md)
4. **Session Summaries**: Avoid unless feature spans multiple days and needs historical record

**Examples:**

❌ **WRONG - Creating unnecessary documents:**

```bash
# Don't create these:
doc/GE-UDC-SESSION-2025-12-03-MORNING.md
doc/GE-UDC-STATUS-UPDATE.md
doc/PHASE4-PROGRESS.md
```

✅ **CORRECT - Update existing documents:**

```bash
# Update these instead:
RESUME.md  # Always update status here
doc/GE-UDC-IMPLEMENTATION-PLAN.md  # Update phases
doc/PHASE4-UTILITY-INVESTIGATION.md  # Already exists, substantial
```

**Question to Ask Before Creating a New Doc:**

- Is this >500 lines of substantial technical content?
- Will this be referenced for weeks/months, not hours?
- Does RESUME.md or existing docs already cover this?
- Am I duplicating information?

**If answers are NO/NO/YES/YES → Don't create the document! Update RESUME.md instead.**

---

## 📋 MANDATORY WORKFLOW: Implementation to Commit

### Phase 2: Commit Preparation (Only After User Says YES)

1. Create `commit.msg` with detailed message
2. Show the commit message to user
3. Show `git diff --stat` summary

### Phase 1: Implementation & Verification (AI Does This)

1. Implement the code changes
2. Run tests **WITH TEE LOGGING**
3. Check for warnings **WITH TEE LOGGING**
4. Create documentation

### 🛑 STOP POINT 1: Present Results

**AI MUST:**

- Show summary of what was implemented
- Show test results
- Show files modified/created
- **ASK**: "Implementation complete. Would you like to review before preparing a commit?"
- **WAIT** for user response

### Phase 2: Commit Preparation (Only After User Says YES)

1. Create `commit.msg` with detailed message
2. Show the commit message to user
3. Show `git diff --stat` summary

### 🛑 STOP POINT 2: Commit Approval

**AI MUST ASK:**
"I've prepared the commit message (shown above). Would you like me to stage and commit?"

**AI MUST WAIT** for explicit approval:

- "YES"
- "commit"
- "commit it"
- "go ahead"
- "stage and commit"

### Phase 3: Git Operations (Only After Explicit Approval)

**NOW and ONLY NOW:**

```bash
git add <actual-files>       # Stage ONLY the code/doc files
                             # NEVER include commit.msg in this!
git commit -F commit.msg     # Use message file (but don't stage it)
```

**CRITICAL:** The `git add` command should list specific files or use `git add -u` for tracked files only. NEVER `git add .` which would stage commit.msg!

---

## 🔴 REAL VIOLATIONS THAT HAPPENED

### Violation #1: Auto-Committing After Tests Pass

**What I did wrong:**

```bash
# Tests pass
cargo test -p cando-webui  # ✓ OK
# Create docs  # ✓ OK
# Create commit.msg  # ✓ OK
git add <files>  # ❌ NO APPROVAL!
git commit -F commit.msg  # ❌ NO APPROVAL!
```

**What I should have done:**

```bash
# Tests pass
cargo test -p cando-webui 2>&1 | tee logs/...  # ✓ With logging
# Create docs  # ✓ OK
# Create commit.msg  # ✓ OK
# STOP AND ASK USER  # ✓ REQUIRED
# WAIT for "yes, commit"
# THEN run git commands
```

### Violation #2: Running Tests Without Logging

**What I did wrong:**

```bash
cargo test -p cando-webui | grep "test result"  # ❌ No tee!
```

**What I should have done:**

```bash
cargo test -p cando-webui 2>&1 | tee logs/phase7-tests_$(date +%Y%m%d_%H%M%S).log | tail -40
```

**Even if I only show a summary, the FULL output must be logged!**

---

## ✅ CORRECT WORKFLOW EXAMPLE

**Scenario: Implementing a new feature**

```markdown
**Me:** "I've implemented Feature X. Running tests now..."

[runs: cargo test 2>&1 | tee logs/feature-x-tests_20241105_140022.log]

**Me:** "✅ Feature X implementation complete:

- All 95 tests pass (was 87, +8 new)
- Zero warnings
- Files modified: 2 files (+150 lines)

**Would you like to review the changes before I prepare a commit?**"

**User:** "Yes, show me"

**Me:** [shows git diff summary and prepares commit.msg]

**Me:** "I've prepared this commit message: [shows commit.msg]

**Would you like me to stage and commit these changes?**"

**User:** "Yes, commit it"

**Me:** [NOW runs git add and git commit]
```

---

## 🧠 MENTAL CHECKLIST BEFORE ANY GIT COMMAND

Before typing `git add`, `git commit`, `git push`:

- [ ] Did the user explicitly say "commit this"?
- [ ] Did I show them what I'm committing?
- [ ] Did I get clear "YES" or "COMMIT" in their last message?

**If ANY answer is NO → DO NOT RUN GIT COMMANDS**

---

## 🧠 MENTAL CHECKLIST BEFORE ANY TEST/BUILD COMMAND

Before typing `cargo test`, `cargo build`, `make tier1`:

- [ ] Does the command include `| tee logs/<context>_$(date +%Y%m%d_%H%M%S).log`?
- [ ] Is the log filename descriptive?
- [ ] Am I capturing stderr with `2>&1`?

**If ANY answer is NO → FIX THE COMMAND FIRST**

---

## 📖 WHERE TO FIND THESE RULES

**Primary source:** `doc/AI_PROJECT_STANDARDS.md`

**Relevant sections:**

- "Git Staging Practices" (lines ~239-280)
- "Development Workflow" (lines ~182-215)
- "Before Making Changes" (lines ~172-180)

**Quote from standards:**

> "NEVER stage files or create commits without explicit user approval"

**Quote from user:**

> "no more violations of git actions please"

---

## 🎯 KEY PHRASES TO LISTEN FOR

**User wants commit:**

- "commit it"
- "yes, commit"
- "go ahead"
- "stage and commit"
- "commit this"
- "looks good, commit"

**User does NOT want commit yet:**

- "looks good" (without "commit")
- "nice"
- "great"
- "that's correct"
- "tests pass"
- "implementation complete"

**When in doubt: ASK!**

---

## 💾 LOG FILE NAMING CONVENTIONS

**Format:** `logs/<phase>-<action>-<subcontext>_YYYYMMDD_HHMMSS.log`

**Examples:**

- `logs/phase7-initial-tests_20241105_143022.log`
- `logs/phase7-final-workspace-tests_20241105_150033.log`
- `logs/manual-testing-replay-mode_20241105_151544.log`
- `logs/manual-testing-browser-check_20241105_152010.log`

**Context should describe:**

- What phase/task you're working on
- What you're testing (tests, build, tier1, etc.)
- Any sub-context (initial, final, retry, etc.)

---

## 🎓 LESSON LEARNED

**User's feedback:**

> "Why are you still doing the git commits without my review?"

**This document exists because I made mistakes.**  
**Don't repeat them.**  
**Always stop and ask before git operations.**  
**Always log long-running commands.**

---

## 🧪 SIMULATOR TESTING PITFALLS - CRITICAL

### Rule #6: ALWAYS USE --no-console FLAG FOR AUTOMATED TESTING

**MANDATORY: All automated tests and scripts must use `--no-console` when running simulators**

**The Problem**: Simulators have interactive console mode enabled by default, which:
- Blocks automated tests waiting for input
- Causes test scripts to hang indefinitely
- Prevents CI/CD integration
- Makes debugging difficult due to unresponsive processes

**❌ WRONG - Missing --no-console:**

```/dev/null/integration_test.rs
let simulator_process = Command::new("cargo")
    .args(&[
        "run",
        "--bin",
        "hvpc-simulator",
        "--",
        "--interface",
        "vcan0",
        "--websocket-port",
        "9001",
        "--device-id",
        "100",
    ])
    .spawn()?;
```

**✅ CORRECT - With --no-console:**

```/dev/null/integration_test.rs
let simulator_process = Command::new("cargo")
    .args(&[
        "run",
        "--bin",
        "hvpc-simulator",
        "--",
        "--interface",
        "vcan0",
        "--websocket-port",
        "9001",
        "--device-id",
        "100",
        "--no-console",  // CRITICAL for automated testing
    ])
    .spawn()?;
```

**Where to Add --no-console**:
1. **Integration tests** (`**/tests/integration_test.rs`)
2. **Test scripts** (`scripts/test_*.sh`, `scripts/compare_*.sh`)
3. **CI/CD pipelines** (`.github/workflows/*.yml`)
4. **Any automated simulator launch**

**When NOT to use --no-console**:
- Manual testing where you want interactive control
- Development/debugging sessions
- Situations where you need to manually send commands via console

**Mental Checklist Before Writing Simulator Test Code**:
- [ ] Am I starting a simulator in an automated context?
- [ ] Did I include `--no-console` flag?
- [ ] Will this test run in CI/CD or scripts?

**This is a FREQUENT violation that happens repeatedly. Always check simulator startup code!**

---

## ⚡ QUICK REFERENCE

**Before implementing:**

1. Read AI_PROJECT_STANDARDS.md
2. Read this document
3. Understand the workflow

**During implementation:**

1. Code changes
2. Tests WITH TEE LOGGING
3. Create docs
4. STOP - ask for review

**Before committing:**

1. Create commit.msg
2. Show to user
3. STOP - ask for approval
4. WAIT for "commit"
5. THEN git operations

**Every session:**

1. Read this document first
2. Read AI_PROJECT_STANDARDS.md
3. Follow the workflow
4. No shortcuts!

---

**Remember: Better to ask twice than commit once without approval!** 🚨
