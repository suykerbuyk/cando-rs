# Cando-RS Script Documentation

**Last Updated**: 2025-01-15  
**Total Scripts**: 31  
**Status**: Complete reference for all project scripts

---

## 📋 Table of Contents

- [Overview](#overview)
- [Quick Reference](#quick-reference)
- [Script Categories](#script-categories)
  - [Root Scripts](#root-scripts)
  - [Development Tools](#development-tools)
  - [Integration Testing](#integration-testing)
  - [Core Libraries](#core-libraries)
  - [Library Test Scripts](#library-test-scripts)
  - [Standalone Test Scripts](#standalone-test-scripts)
  - [Utilities](#utilities)
- [Usage Patterns](#usage-patterns)
- [Best Practices](#best-practices)
- [Removed Scripts](#removed-scripts)

---

## Overview

The Cando-RS project includes 31 shell scripts organized into logical categories for testing, development, and deployment. All scripts follow project standards and prefer Rust tools (cansend-rs, candump-rs, cando-cfg) over external dependencies.

### Organization Structure

```
scripts/
├── dev-tools/              # Development utilities (3 scripts)
├── integration/            # Integration testing (9 scripts)
│   ├── configs/            # Test configuration (1 script)
│   └── lib/                # Reusable libraries (4 scripts)
├── packaging/              # Distribution tools (1 script)
├── testing/                # Test utilities (1 script)
└── [root]                  # Standalone scripts (12 scripts)
```

### Key Principles

- ✅ **Rust-First**: All scripts prefer native Rust tools (cansend-rs, candump-rs, cando-cfg)
- ✅ **Backward Compatible**: Fallback to system tools when Rust versions unavailable
- ✅ **Configuration-Driven**: Use cando.toml as single source of truth
- ✅ **Comprehensive Logging**: All test scripts use tee for full output capture
- ✅ **Error Detection**: Clear error messages with actionable guidance

---

## Quick Reference

### Most Common Scripts

```bash
# Integration Testing
make tier1                                          # Tier 1 validation (48 tests)
make tier2                                          # Tier 2 full integration (56 tests)
./scripts/integration/integration_test_all_protocols.sh
./scripts/integration/integration_test_physical_can.sh

# Development
./scripts/follow-log.sh                             # Tail latest log
./scripts/dev-tools/clear-diagnostics.sh            # Clear rust-analyzer cache
./scripts/dev-tools/detect_codegen_change.sh        # Check codegen drift

# Testing Utilities
./scripts/test_canid_fixes.sh                       # Test CAN ID recognition
./scripts/test_emp_simulator.sh                     # EMP simulator demo

# Setup
./scripts/set_can_privileges.sh all                 # Setup CAN permissions
```

---

## Script Categories

### Root Scripts

#### scripts/follow-log.sh

**Purpose**: Quick utility to tail the most recent log file

**Usage**:
```bash
./scripts/follow-log.sh
```

**Implementation**: Simple 2-line script using `tail -f logs/$(ls -t logs/ | head -1)`

**Dependencies**: None

**Used In**: Development workflow (manual use)

**Lines**: 2

---

#### scripts/generate-man.sh

**Purpose**: Generate man pages for all Cando-RS workspace binaries

**Usage**:
```bash
./scripts/generate-man.sh
```

**Features**:
- Auto-discovers all workspace binaries
- Generates man pages from Rust clap definitions
- Outputs to `man/` directory
- Color-coded logging
- Error handling for missing binaries

**Dependencies**: Workspace binaries with clap-based CLI

**Used In**: Packaging pipeline, Makefile (`build-manpages` target)

**Lines**: ~150

**Output**: `man/*.1` files

---

#### scripts/install-man.sh

**Purpose**: Install generated man pages to system or user directories

**Usage**:
```bash
# System installation (requires sudo)
./scripts/install-man.sh

# User installation
./scripts/install-man.sh --user
```

**Features**:
- System install: `/usr/local/share/man/man1`
- User install: `~/.local/share/man/man1`
- Creates directories if needed
- Validates man pages exist
- Updates man database

**Dependencies**: `generate-man.sh` output

**Used In**: Deployment, system integration

**Lines**: ~100

---

### Development Tools

#### scripts/dev-tools/clear-diagnostics.sh

**Purpose**: Clear diagnostic caches to resolve stubborn language server issues

**Usage**:
```bash
./scripts/dev-tools/clear-diagnostics.sh
```

**Actions**:
1. Kills rust-analyzer processes
2. Clears rust-analyzer cache (`~/.cache/rust-analyzer`)
3. Clears Zed IDE cache (`~/.cache/zed`)
4. Runs `cargo clean`
5. Removes `target/`, `.cargo/`, `Cargo.lock`
6. Performs clean rebuild

**Dependencies**: cargo

**When to Use**: 
- Language server shows stale diagnostics
- Phantom errors that don't match actual code
- After major refactoring
- J1939-73 diagnostic implementation issues

**Used In**: Development troubleshooting

**Lines**: ~50

**⚠️ Warning**: Removes build artifacts and forces full rebuild

---

#### scripts/dev-tools/detect_codegen_change.sh

**Purpose**: Detect code generation algorithm changes for CI/CD

**Usage**:
```bash
# Check all protocols
./scripts/dev-tools/detect_codegen_change.sh

# Strict mode (exit 1 on any changes)
./scripts/dev-tools/detect_codegen_change.sh --strict

# CI-friendly output
./scripts/dev-tools/detect_codegen_change.sh --ci
```

**Exit Codes**:
- `0`: All protocols clean (no changes)
- `1`: Algorithm evolution detected (breaking changes possible)
- `2`: Script error (tool not found, build failure)

**Features**:
- Wraps cando-codegen detect-changes command
- CI/CD integration friendly
- Actionable output
- Color-coded results (disabled in CI)

**Dependencies**: cando-codegen binary

**Used In**: Makefile (`tier2` target), CI/CD pipelines

**Lines**: ~150

---
- Color-coded output (✓/⚠/✗)
- Error and warning counts
- Exit code indicates overall health

**Dependencies**: cargo, git, workspace structure

**Used In**: AI workflow, new thread context restoration

**Lines**: ~250

---

### Integration Testing

#### scripts/integration/integration_test_all_protocols.sh

**Purpose**: Main tier2 integration test suite for all protocols (J1939, EMP, HVPC, UDC)

**Usage**:
```bash
# Via Makefile (recommended)
make tier2

# Direct execution
./scripts/integration/integration_test_all_protocols.sh

# With custom environment
ENVIRONMENT=tier2-physical ./scripts/integration/integration_test_all_protocols.sh
```

**Features**:
- Tests all 4 protocols (J1939, EMP, HVPC, UDC)
- Configuration-driven (cando.toml)
- Dynamic device discovery
- Message validation
- State persistence testing
- WebSocket integration
- Error detection
- Performance tracking

**Dependencies**:
- cando-cfg
- config_helpers.sh
- simulator_helpers.sh
- websocket_helpers.sh
- candump-rs (preferred) or candump
- All 4 simulators

**Used In**: `make tier2` (56 tests)

**Lines**: 1,498

**Target Time**: < 30 minutes

**Migration**: Updated in cando-cfg Phase 3a with full configuration integration

---

#### scripts/integration/integration_test_physical_can.sh

**Purpose**: Integration testing for physical CAN hardware

**Usage**:
```bash
# Via Makefile
make tier2-physical

# Direct execution
./scripts/integration/integration_test_physical_can.sh
```

**Features**:
- Physical CAN interface testing (can0, can1)
- Hardware loopback validation
- EMP motor control testing
- J1939 message validation
- Multi-interface coordination
- Real hardware timing

**Dependencies**:
- Physical CAN hardware (2 interfaces)
- cando-cfg
- config_helpers.sh
- simulator_helpers.sh
- Proper CAN setup (see set_can_privileges.sh)

**Hardware Requirements**:
- 2 CAN interfaces (can0, can1)
- Loopback connection or real devices
- CAP_NET_RAW capability

**Used In**: `make tier2-physical`

**Lines**: 1,130

**Target Time**: < 12 minutes

**Migration**: Updated in cando-cfg Phase 3b

---

#### scripts/integration/validate_all_protocols.sh

**Purpose**: Tier 1 integration testing framework (repository-independent, no hardware)

**Usage**:
```bash
# Via Makefile (recommended)
make tier1

# Direct execution
./scripts/integration/validate_all_protocols.sh
```

**Features**:
- GitHub Actions compatible
- No hardware dependencies
- Pure software validation
- Protocol encoding/decoding
- Message structure validation
- Fast execution

**Dependencies**: Workspace binaries only (no external tools)

**Used In**: `make tier1` (part of 48 tests), CI/CD

**Lines**: ~500

**Target Time**: < 10 minutes

**⚠️ Note**: NOT redundant with integration_test_all_protocols.sh - different purpose (tier1 vs tier2)

---

#### scripts/integration/validate_dump_messages.sh

**Purpose**: Tier 1 integration testing for dump-messages metadata flags

**Usage**:
```bash
# Via Makefile
make validate-dump-messages

# Direct execution
./scripts/integration/validate_dump_messages.sh
```

**Tests**:
- `dump-messages --comments` flag
- `dump-messages --enums` flag
- `dump-messages --full` flag
- Metadata extraction
- DBC comment parsing
- Enumeration display

**Dependencies**: dump-messages binary

**Used In**: `make tier1`, `make validate`, `make tier2`

**Lines**: ~300

**Target Time**: < 2 minutes

---

#### scripts/integration/test_phase5b.sh

**Purpose**: Message verification tests (Phase 5b validation)

**Usage**:
```bash
./scripts/integration/test_phase5b.sh
```

**Tests**:
- WebSocket message tracking
- wait_for_message functionality
- Deterministic verification
- Message history tracking
- Timing validation

**Dependencies**:
- cando-cfg
- config_helpers.sh
- simulator_helpers.sh
- websocket_helpers.sh
- rust-websocket-query

**Lines**: 301

**Migration**: Updated in cando-cfg Phase 3b

---

#### scripts/integration/test_phase5c.sh

**Purpose**: Sequence tests (Phase 5c validation)

**Usage**:
```bash
./scripts/integration/test_phase5c.sh
```

**Tests**:
- Message sequence validation
- Multi-step workflows
- State transitions
- Timing sequences
- Error recovery

**Dependencies**:
- cando-cfg
- config_helpers.sh
- simulator_helpers.sh

**Lines**: 389

**Migration**: Updated in cando-cfg Phase 3b

---

#### scripts/integration/test_phase5d.sh

**Purpose**: Infrastructure delay optimization tests (Phase 5d validation)

**Usage**:
```bash
./scripts/integration/test_phase5d.sh
```

**Tests**:
- Simulator startup timing
- candump-rs readiness detection
- CAN bus settle time
- Infrastructure latency
- Performance optimization validation

**Features**:
- Test-and-retry with exponential backoff
- CAN ID filtering for readiness detection
- Dynamic timing adjustment

**Dependencies**:
- cando-cfg
- config_helpers.sh
- simulator_helpers.sh
- candump-rs

**Lines**: 383

**Migration**: Updated in cando-cfg Phase 3a

**Performance**: Validates 40% improvement (80s → 48s)

---

#### scripts/integration/phase8_hardware_validation.sh

**Purpose**: Phase 8 physical hardware testing (deferred until DeviceId newtype refactor)

**Status**: ⏸️ DEFERRED - Future work

**Planned Features**:
- External J1939 equipment testing
- Full compliance certification
- Multi-device network testing (253+ devices)
- Real-world J1939 environment validation

**Dependencies**: DeviceId newtype refactor completion

**Lines**: TBD (not yet implemented)

**When**: After DeviceId newtype refactor (see DEVICEID-REFACTOR-PLAN.md)

---

#### scripts/integration/configs/test_config.sh

**Purpose**: Integration test configuration parameters

**Content**:
- Execution time targets (tier1, tier2)
- Performance regression thresholds
- Test iteration counts
- Protocol-specific settings
- Timing parameters

**Usage**: Sourced by integration test scripts

**Lines**: ~50

**⚠️ Note**: Consolidated into cando.toml (Session 19)

---

### Core Libraries

#### scripts/integration/lib/config_helpers.sh

**Purpose**: Configuration query library (cando-cfg integration - Phase 1)

**Usage**:
```bash
source scripts/integration/lib/config_helpers.sh
load_test_config "cando.toml"
PORT=$(get_device_port "j1939-sim")
```

**Functions** (13 total):
- `load_test_config()` - Load TOML configuration
- `get_device_port()` - Get WebSocket port
- `get_device_id()` - Get device ID
- `get_device_type()` - Get device type
- `get_device_interface()` - Get CAN interface
- `list_environments()` - List available environments
- `get_environment_interface()` - Get environment's CAN interface
- `get_environment_devices()` - List devices in environment
- `is_hardware_environment()` - Check if physical hardware
- `is_environment_enabled()` - Check if environment enabled
- `get_simulator_ports_array()` - Populate port array
- `get_simulator_device_ids_array()` - Populate device ID array
- `get_environment_info_arrays()` - Populate both arrays efficiently

**Key Features**:
- Zero Python dependencies
- Native Rust binary (cando-cfg)
- 17x faster than old Python approach (35ms → 2ms)
- Shell-parseable output
- Single source of truth (cando.toml)

**Dependencies**: cando-cfg binary

**Used By**: All integration test scripts

**Lines**: 998

**Test Coverage**: 19/20 tests passing (95%)

**Documentation**: doc/CANDO-CFG-PHASE1-COMPLETE.md

---

#### scripts/integration/lib/simulator_helpers.sh

**Purpose**: Simulator management library (cando-cfg integration - Phase 2)

**Usage**:
```bash
source scripts/integration/lib/simulator_helpers.sh
start_all_simulators_for_environment "tier2-virtual"
wait_for_all_simulators_ready "${SIMULATOR_PIDS[@]}"
cleanup_simulators
```

**Functions** (12 total):

**Array Building**:
- `get_simulator_ports_array()` - Build port arrays from config
- `get_simulator_device_ids_array()` - Build device ID arrays
- `get_environment_info_arrays()` - Build both at once

**Simulator Control**:
- `start_simulator()` - Start single simulator
- `start_all_simulators_for_environment()` - Start all in environment
- `start_all_simulators()` - Start all (current environment)
- `wait_for_all_simulators_ready()` - Wait for WebSocket readiness
- `stop_simulator_by_pid()` - Stop single simulator
- `stop_all_simulators()` - Stop all simulators
- `cleanup_simulators()` - Cleanup for trap handlers

**Utilities**:
- `verify_simulator_binaries()` - Check binaries exist
- `print_simulator_summary()` - Show simulator status

**Key Features**:
- Environment-aware design
- Dynamic configuration loading
- No hardcoded values
- Comprehensive error handling
- Eliminates ~229 lines of duplicate code

**Dependencies**: cando-cfg, config_helpers.sh

**Used By**: All integration test scripts

**Lines**: 739

**Test Coverage**: 15/15 tests passing (100%)

**Documentation**: doc/CANDO-CFG-PHASE2-COMPLETE.md

---

#### scripts/integration/lib/validation_functions.sh

**Purpose**: Validation library functions for protocol-specific testing

**Usage**:
```bash
source scripts/integration/lib/validation_functions.sh
log_info "Starting test"
record_validation_result "test_name" "PASS" "250ms"
```

**Functions**:
- `log_info()`, `log_success()`, `log_warning()`, `log_error()` - Colored logging
- `record_validation_result()` - Record test result with metadata
- Various validation helpers

**Key Features**:
- Consistent output formatting
- Test result tracking
- Color-coded messages
- Timestamp logging

**Dependencies**: None (pure bash)

**Used By**: Integration test scripts

**Lines**: ~200

---

#### scripts/integration/lib/websocket_helpers.sh

**Purpose**: WebSocket helper functions using rust-websocket-query

**Usage**:
```bash
source scripts/integration/lib/websocket_helpers.sh
detect_rust_websocket_query
STATE=$(query_simulator_state 8082)
VALUE=$(extract_state_field 8082 "field_name")
validate_field_value 8082 "field_name" "expected_value"
```

**Functions**:
- `detect_rust_websocket_query()` - Locate rust-websocket-query binary
- `query_simulator_state()` - Query simulator via WebSocket
- `extract_state_field()` - Extract specific field from state
- `validate_field_value()` - Validate field matches expected value
- Device lookup helpers (9 functions for Phase 5d)

**Key Features**:
- Native Rust tool (rust-websocket-query)
- JSON parsing with jq
- Error handling
- Timeout support
- Auto-detection of tool location

**Dependencies**: rust-websocket-query, jq

**Used By**: Integration test scripts, message verification tests

**Lines**: ~300

---

### Library Test Scripts

#### scripts/integration/test_config_helpers.sh

**Purpose**: Unit tests for config_helpers.sh library

**Usage**:
```bash
./scripts/integration/test_config_helpers.sh
```

**Tests**:
- All 13 functions in config_helpers.sh
- Configuration loading
- Device queries
- Environment queries
- Array population
- Error handling

**Test Coverage**: 19/20 tests passing (95%)

**Dependencies**: cando-cfg, config_helpers.sh

**Lines**: ~200

---

#### scripts/integration/test_simulator_helpers.sh

**Purpose**: Unit tests for simulator_helpers.sh library

**Usage**:
```bash
./scripts/integration/test_simulator_helpers.sh
```

**Tests**:
- All 12 functions in simulator_helpers.sh
- Simulator startup/shutdown
- Array building
- Environment handling
- Cleanup logic

**Test Coverage**: 15/15 tests passing (100%)

**Dependencies**: cando-cfg, simulator_helpers.sh

**Lines**: ~200

---

#### scripts/integration/test_websocket_helpers.sh

**Purpose**: Unit tests for websocket_helpers.sh library

**Usage**:
```bash
./scripts/integration/test_websocket_helpers.sh
```

**Tests**:
- WebSocket query functions
- State extraction
- Field validation
- Error handling
- Timeout behavior

**Process**:
1. Starts J1939 simulator
2. Tests WebSocket queries
3. Sends CAN message
4. Verifies state updates
5. Cleans up

**Dependencies**: J1939 simulator, websocket_helpers.sh

**Lines**: ~150

---

#### scripts/integration/test_device_lookup_helpers.sh

**Purpose**: Unit tests for Phase 5d device lookup helper functions

**Usage**:
```bash
./scripts/integration/test_device_lookup_helpers.sh
```

**Tests**:
- 9 new device lookup functions in websocket_helpers.sh
- Configuration-based device queries
- Port lookup
- Device ID lookup
- Interface queries

**Dependencies**: cando.toml, rust-can-util, jq

**Lines**: ~200

---

### Standalone Test Scripts

#### scripts/test_canid_fixes.sh

**Purpose**: Focused integration test for CAN ID fixes in all 4 simulators

**Usage**:
```bash
./scripts/test_canid_fixes.sh
```

**Tests**:
- J1939: Messages with corrected BASE_CAN_ID values
- EMP: MCM_MotorCommandMessage recognition
- HVPC: HVPC_Command recognition
- UDC: UDC_Command recognition

**Features**:
- Uses test device ID 0x0F (DeviceId::Device0F)
- Error detection (prevents false positives)
- Source filtering validation
- Message recognition verification

**Test Validation**:
1. Expected "Received" pattern appears
2. No error/panic messages in logs

**Dependencies**: All 4 simulators, cansend-rs (preferred) or cansend

**Lines**: ~400

**CAN Tool**: ✅ Migrated to prefer cansend-rs (2025-01-15)

**Exit Codes**:
- `0`: All tests passed
- `1`: One or more tests failed

---

#### scripts/test_emp_message_tracking.sh

**Purpose**: Manual test for EMP WebSocket message tracking (WebSocket ACK Phase)

**Usage**:
```bash
./scripts/test_emp_message_tracking.sh
```

**Verifies**:
1. EMP simulator receives CAN messages
2. Messages recorded in message history
3. WebSocket WaitForMessage queries work correctly
4. MessageReceived response has found=true

**Expected Behavior**:
- Send MCM_MotorCommandMessage via CAN
- Query WebSocket for message reception
- Receive MessageReceived with 11-12ms response time

**Dependencies**: EMP simulator, rust-websocket-query, cansend-rs

**Lines**: ~150

**Purpose**: Manual validation of WebSocket ACK feature

---

#### scripts/test_emp_simulator.sh

**Purpose**: EMP simulator demonstration/test script

**Usage**:
```bash
./scripts/test_emp_simulator.sh
```

**Features**:
- Sets up vcan interface
- Starts EMP simulator
- Demonstrates functionality
- Tests message sending
- Validates responses

**CAN Tool**: ✅ Already migrated (detects and prefers cansend-rs)

**Dependencies**: EMP simulator, cansend-rs (preferred) or cansend

**Lines**: ~200

**Purpose**: Demo and manual testing tool

---

#### scripts/test_single_j1939.sh

**Purpose**: Single J1939 message test with detailed diagnostics

**Usage**:
```bash
# Test EEC12 message
./scripts/test_single_j1939.sh EEC12

# Test with custom device ID
./scripts/test_single_j1939.sh EEC12 0x8A

# Test ETC5 message
./scripts/test_single_j1939.sh ETC5
```

**Features**:
- One message at a time
- Full verbose output
- Detailed diagnostics
- Encoder/decoder debugging
- State inspection

**Supported Messages**: EEC12, ETC5 (configurable)

**Dependencies**: J1939 simulator

**Lines**: ~150

**Purpose**: Debugging tool for J1939 message issues

---

### Utilities

#### scripts/j1939_progress.sh

**Purpose**: J1939 implementation progress dashboard

**Usage**:
```bash
./scripts/j1939_progress.sh
```

**Displays**:
- Current implementation count
- Target (50% = 1,073 messages)
- Baseline (3 messages)
- Progress percentage
- Messages remaining

**Calculation**: Tracks messages in `cando-messages/src/j1939_impl.rs`

**Constants**:
- Total J1939 messages: 2,146
- Target messages: 1,073 (50%)
- Baseline: 3

**Dependencies**: None (analyzes source files)

**Lines**: ~100

**Purpose**: Track progress toward 50% J1939 coverage goal

---

#### scripts/set_can_privileges.sh

**Purpose**: Permanently grant CAP_NET_RAW to cando-rs binaries for unprivileged CAN access

**Usage**:
```bash
# Complete setup (capabilities + CAN interfaces)
sudo ./scripts/set_can_privileges.sh all

# Set capabilities only
sudo ./scripts/set_can_privileges.sh caps

# Setup CAN interfaces only
sudo ./scripts/set_can_privileges.sh can

# Setup systemd network configuration
sudo ./scripts/set_can_privileges.sh setup

# Setup udev rules
sudo ./scripts/set_can_privileges.sh udev
```

**Actions**:
- Sets `cap_net_raw+eip` on all workspace binaries
- Configures systemd network for CAN interfaces
- Creates udev rules for automatic interface setup
- Validates configuration

**Requirements**:
- libcap2-bin (`sudo apt install libcap2-bin`)
- sudo access
- Run after `cargo build --release`

**Dependencies**: libcap2-bin, systemd (optional), udev (optional)

**Used In**: `make setup-can-all`, `make setup-can-privileges`

**Lines**: ~300

**⚠️ Important**: Must re-run after rebuilding binaries

---

#### scripts/packaging/generate-completions.sh

**Purpose**: Generate shell completions for all Cando-RS binaries

**Usage**:
```bash
# Generate for default target (x86_64-unknown-linux-musl)
./scripts/packaging/generate-completions.sh

# Generate for specific target
./scripts/packaging/generate-completions.sh aarch64-unknown-linux-gnu
```

**Generates**:
- Bash completions → `target/completions/bash/`
- Zsh completions → `target/completions/zsh/`
- Fish completions → `target/completions/fish/`

**Requirements**:
- Binaries must be built first
- Each binary should support clap-based completion generation
- Binary must implement `--generate-completion` flag

**Dependencies**: Workspace binaries with clap completion support

**Used In**: Packaging pipeline, distribution

**Lines**: ~150

---

#### scripts/testing/generate-test-frames.sh

**Purpose**: Generate various CAN frames for testing candump-rs

**Usage**:
```bash
# Generate all test frames
./scripts/testing/generate-test-frames.sh vcan0 all

# Generate specific test mode
./scripts/testing/generate-test-frames.sh vcan0 basic
./scripts/testing/generate-test-frames.sh vcan0 ascii
./scripts/testing/generate-test-frames.sh vcan0 filter
./scripts/testing/generate-test-frames.sh vcan0 stress
```

**Test Modes**:
- `basic`: Standard CAN frames
- `ascii`: ASCII-printable data patterns
- `filter`: Various CAN IDs for filter testing
- `stress`: High-volume message generation
- `all`: All of the above

**CAN Tool**: ✅ Already migrated (detects and prefers cansend-rs)

**Dependencies**: cansend-rs (preferred) or cansend

**Lines**: ~200

**Purpose**: candump-rs testing and validation

---

## Usage Patterns

### Integration Testing Workflow

```bash
# Standard development workflow
make tier1                                          # Quick validation (10 min)
make tier2                                          # Full integration (30 min)

# Manual testing
./scripts/integration/integration_test_all_protocols.sh
./scripts/integration/integration_test_physical_can.sh

# Test specific phases
./scripts/integration/test_phase5b.sh               # Message verification
./scripts/integration/test_phase5c.sh               # Sequence tests
./scripts/integration/test_phase5d.sh               # Infrastructure tests
```

### Configuration Management

```bash
# Using config_helpers.sh
source scripts/integration/lib/config_helpers.sh
load_test_config "cando.toml"

# Query configuration
PORT=$(get_device_port "j1939-sim")
DEVICE_ID=$(get_device_id "j1939-sim")
INTERFACE=$(get_device_interface "j1939-sim")

# Environment discovery
list_environments
IS_HARDWARE=$(is_hardware_environment "tier2-physical")
```

### Simulator Management

```bash
# Using simulator_helpers.sh
source scripts/integration/lib/simulator_helpers.sh

# Start simulators
start_all_simulators_for_environment "tier2-virtual"
wait_for_all_simulators_ready "${SIMULATOR_PIDS[@]}"

# Cleanup
trap cleanup_simulators EXIT
```

### WebSocket Queries

```bash
# Using websocket_helpers.sh
source scripts/integration/lib/websocket_helpers.sh

# Query simulator state
STATE=$(query_simulator_state 8082)

# Extract and validate field
VALUE=$(extract_state_field 8082 "field_name")
validate_field_value 8082 "field_name" "expected_value"
```

### CAN Tool Detection

```bash
# Pattern used in all CAN-using scripts
detect_cansend() {
    if command -v cansend-rs >/dev/null 2>&1; then
        echo "cansend-rs"
    elif [[ -x "$WORKSPACE_DIR/target/release/cansend-rs" ]]; then
        echo "$WORKSPACE_DIR/target/release/cansend-rs"
    elif command -v cansend >/dev/null 2>&1; then
        echo "cansend"
    else
        return 1
    fi
}

CANSEND_CMD=$(detect_cansend) || { echo "Error: cansend not found"; exit 1; }
$CANSEND_CMD "$interface" "$message"
```

---

## Best Practices

### Writing New Scripts

1. **Add Purpose Header**:
   ```bash
   #!/bin/bash
   # Script Name and Purpose
   # Detailed description of what this script does
   ```

2. **Use Configuration Libraries**:
   ```bash
   source scripts/integration/lib/config_helpers.sh
   source scripts/integration/lib/simulator_helpers.sh
   ```

3. **Prefer Rust Tools**:
   - Use cansend-rs over cansend
   - Use candump-rs over candump
   - Use cando-cfg for configuration
   - Implement fallback for compatibility

4. **Implement Error Handling**:
   ```bash
   set -euo pipefail  # Exit on error, undefined vars, pipe failures
   ```

5. **Use Color-Coded Logging**:
   ```bash
   GREEN='\033[0;32m'
   RED='\033[0;31m'
   NC='\033[0m'
   log_success() { echo -e "${GREEN}✓${NC} $*"; }
   ```

6. **Capture Full Output**:
   ```bash
   ./script.sh 2>&1 | tee logs/script_$(date +%Y%m%d_%H%M%S).log
   ```

### Script Organization

- **Root scripts**: Utilities used from project root
- **dev-tools/**: Development utilities
- **integration/**: All integration testing
- **integration/lib/**: Reusable libraries
- **packaging/**: Distribution and deployment
- **testing/**: Test utilities

### Maintenance Guidelines

1. **Update SCRIPTS.md** when adding/modifying scripts
2. **Document dependencies** clearly in script headers
3. **Add to appropriate tier tests** if applicable
4. **Test thoroughly** before committing
5. **Follow naming conventions** (lowercase, underscores)

---

## Removed Scripts

### Obsolete Scripts Removed (2025-01-15)

During the script consolidation effort, the following obsolete scripts were removed:

#### scripts/run_integration_test.sh

**Removed**: 2025-01-15  
**Reason**: Old EMP-only integration test superseded by integration_test_all_protocols.sh  
**Lines**: ~100  
**Last Modified**: Ancient (Phase 5 timeframe)  
**Replacement**: Use `scripts/integration/integration_test_all_protocols.sh` or `make tier2`

#### scripts/integration/integration_test_all_protocols_config.sh

**Removed**: 2025-01-15  
**Reason**: Incomplete 455-line stub created as proof-of-concept, never fully implemented  
**Lines**: 455  
**Last Modified**: Phase 2 (pre-cando-cfg)  
**Replacement**: Use `scripts/integration/integration_test_all_protocols.sh` (1,498 lines, fully updated)  
**Note**: Main `tier2` target always used the correct script. Stub was only referenced by obsolete tier2-config* Makefile targets (also removed).

#### scripts/validate_doctest_state.sh

**Removed**: 2025-01-15  
**Reason**: Historic validation script for completed doctest fixes project  
**Lines**: ~150  
**Last Modified**: Ancient (doctest fixes phase)  
**Replacement**: Doctest fixes are complete, no replacement needed

#### scripts/validate_enhanced_emp_testing.sh

**Removed**: 2025-01-15  
**Reason**: Historic validation script for completed Phase 2B EMP integration  
**Lines**: ~200  
**Last Modified**: Ancient (Phase 2B timeframe)  
**Replacement**: Phase 2B is complete, no replacement needed

### Removed Makefile Targets (2025-01-15)

- **tier2-config**: Used stub script (integration_test_all_protocols_config.sh)
- **tier2-config-virtual**: Used stub script
- **tier2-config-physical**: Used stub script

**Replacement**: Use `make tier2` (uses correct script: integration_test_all_protocols.sh)

### Migration Summary

**Total Removed**: 4 scripts (~905 lines) + 3 Makefile targets  
**Impact**: Cleaner project structure, no confusion from obsolete/stub scripts  
**Documentation**: See doc/SCRIPT-CONSOLIDATION-COMPLETE.md for full details

---

## Summary Statistics

**Total Scripts**: 31  
**Total Lines**: ~7,595 (after consolidation)

### By Category

| Category | Count | Percentage |
|----------|-------|------------|
| Integration Tests | 9 | 29.0% |
| Core Libraries | 4 | 12.9% |
| Library Tests | 4 | 12.9% |
| Standalone Tests | 4 | 12.9% |
| Development Tools | 3 | 9.7% |
| Root Utilities | 3 | 9.7% |
| Utilities | 4 | 12.9% |

### Migration Status

**can-utils → Rust Tools**: 100% Complete

| Tool | Scripts | Status |
|------|---------|--------|
| cansend-rs | 5 | ✅ All prefer Rust version |
| candump-rs | 2 | ✅ All prefer Rust version |
| cando-cfg | 15+ | ✅ Universal adoption |

### Test Coverage

- **Library Tests**: 49/50 tests passing (98%)
  - config_helpers.sh: 19/20 (95%)
  - simulator_helpers.sh: 15/15 (100%)
  - websocket_helpers.sh: Validated
  - device_lookup_helpers.sh: Validated

- **Integration Tests**: 
  - tier1: 48/48 tests (100%)
  - tier2: 56/56 tests (100%)

---

## Quick Reference Card

### Daily Development

```bash
# Most common commands
make tier1                          # Quick validation (10 min)
./scripts/follow-log.sh             # Tail latest log
./scripts/dev-tools/clear-diagnostics.sh  # Clear caches

# Full testing
make tier2                          # Full integration (30 min)

# CAN setup (once)
sudo ./scripts/set_can_privileges.sh all
```

### Script Locations

```
scripts/
├── follow-log.sh                   # Tail latest log
├── generate-man.sh                 # Generate man pages
├── install-man.sh                  # Install man pages
├── j1939_progress.sh               # Progress dashboard
├── set_can_privileges.sh           # CAN setup
├── test_*.sh                       # Standalone tests (4)
├── dev-tools/                      # Development (3)
├── integration/                    # Integration (9 + 4 libs + 4 tests)
├── packaging/                      # Distribution (1)
└── testing/                        # Test utilities (1)
```

---

## Additional Resources

### Documentation

- **Investigation**: doc/SCRIPT-AUDIT-INVESTIGATION.md (477 lines)
- **Consolidation Plan**: doc/SCRIPT-CONSOLIDATION-PLAN.md (623 lines)
- **Completion Summary**: doc/SCRIPT-CONSOLIDATION-COMPLETE.md (471 lines)
- **cando-cfg Integration**: doc/CANDO-CFG-INTEGRATION-COMPLETE.md
- **Migration Guide**: doc/CANDO-CFG-MIGRATION-GUIDE.md

### Related Documentation

- **Project Standards**: doc/AI-WORKFLOW-GUIDE.md
- **Testing Framework**: Makefile (comprehensive targets)
- **Configuration**: cando.toml (single source of truth)

---

**Last Updated**: 2025-01-15  
**Maintained By**: Cando-RS Team  
**Status**: Complete and current