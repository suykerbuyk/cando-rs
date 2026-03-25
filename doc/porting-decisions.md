# Porting Decisions: proteus-rs → cando-rs

This document captures the key decisions made when forking cando-rs from
proteus-rs, and the rationale behind each. It serves as a reference for
future development to understand why things are the way they are.

## Fork Strategy

cando-rs was created as an open-source subset of proteus-rs with all
proprietary protocol support removed. The goal was a clean, standalone
J1939 CAN bus toolkit — not a crippled version of proteus-rs with features
disabled.

**Approach**: Extract and adapt, not ifdef. Proprietary code was removed
entirely rather than hidden behind feature flags or compile-time switches.
This keeps the codebase simple and avoids the maintenance burden of
conditional compilation paths that can't be tested in the open-source CI.

## What Was Removed

### Proprietary Protocols

| Protocol | What it was | Why removed |
|----------|-------------|-------------|
| EMP | Electric motor/pump control protocol | Proprietary to specific hardware vendor |
| HVPC | High-voltage power controller protocol | Proprietary, 12-channel group telemetry |
| UDC | Universal device controller protocol | Proprietary opcode-based protocol |

These three protocols represented ~70% of the proteus-rs codebase by volume.
Their DBC files, codegen templates, simulators, CLI tools, WebUI pages, and
integration tests were all removed.

### Removed Binaries

| proteus-rs binary | Purpose | Why removed |
|-------------------|---------|-------------|
| `emp-simulator` | EMP protocol simulator | EMP-specific |
| `hvpc-simulator` | HVPC protocol simulator with 12-channel telemetry | HVPC-specific |
| `udc-simulator` | UDC protocol simulator | UDC-specific |
| `proteus-webui` | Browser-based telemetry dashboard | Primarily EMP/HVPC visualization |
| `count-hvpc-signals` | HVPC DBC signal counter | HVPC-specific |
| `can-gateway` | CAN message routing between interfaces | Tightly coupled to EMP/HVPC |
| `playwright-test` | WebUI browser automation tests | Depends on proteus-webui |

### Removed Makefile Targets

| Target | Purpose | Why removed |
|--------|---------|-------------|
| `test-hvpc-simulator` | HVPC simulator integration test | HVPC-specific |
| `test-emp-simulator` | EMP simulator integration test | EMP-specific |
| `tier2-physical` | Physical CAN hardware testing | Requires proprietary EMP hardware |
| `test-webui` | Playwright WebUI tests | Depends on proteus-webui |
| `test-ping-messages` | Physical CAN ping format test | EMP-specific message format |
| `test-protocol` | Multi-protocol test runner | Runs EMP/HVPC protocol tests |
| `test-simulators` | All-simulators test runner | Runs EMP/HVPC/UDC simulators |
| `all-physical` | Full physical test suite | Includes proprietary hardware |

## What Was Kept

### J1939 Protocol Stack

The complete J1939 implementation was retained:
- DBC-based code generation for J1939 message types
- J1939 simulator (`cando-j1939-sim`)
- Message encoding/decoding with full PGN support
- J1939-73 diagnostics support

### Core Infrastructure

| Component | Notes |
|-----------|-------|
| Codegen pipeline | DBC → Rust code generation, protocol-agnostic |
| CLI framework | clap-based argument parsing, man page generation, shell completions |
| CAN abstraction | socketcan bindings, vcan support, CAP_NET_RAW handling |
| Cross-compilation | x86_64 and aarch64 targets, glibc and musl (via Zig) |
| Debian packaging | cargo-deb integration, meta-package pattern |
| CI/CD | GitHub Actions release workflow with multi-arch builds |
| Tiered testing | Tier 1 (unit + integration), Tier 2 (vcan full-stack) |

### Retained Binaries (10)

| Binary | Purpose |
|--------|---------|
| `cando-util` | General-purpose CAN utility (send, receive, decode) |
| `cando-dump` | Raw CAN frame dumper (candump replacement) |
| `cando-send` | CAN frame sender (cansend replacement) |
| `cando-monitor` | Live CAN bus monitor with protocol decoding |
| `cando-dump-messages` | Decoded message dumper with metadata flags |
| `cando-cfg` | Configuration file management |
| `cando-codegen` | DBC-to-Rust code generator |
| `cando-j1939-sim` | J1939 protocol simulator |
| `cando-log-analyzer` | CAN log file analysis |
| `cando-ws-query` | WebSocket query tool for CAN data |

## Key Adaptation Decisions

### integration_test_all_protocols.sh — Complete Rewrite

The proteus-rs version was ~70KB and ~95% EMP/HVPC/UDC-specific code
(multi-protocol simulator management, EMP motor control tests, HVPC
channel group tests, UDC opcode tests, cross-protocol interference
testing). Rather than stripping it down, it was rewritten from scratch
as a ~280-line J1939-only tier 2 test. The original structure was too
deeply coupled to the multi-protocol architecture to be worth salvaging.

### validate_all_protocols.sh — Aggressive Stripping

This script had a cleaner structure than the integration test, with
per-protocol test sections that could be individually removed. The
4-protocol loop (`emp hvpc j1939 udc`) was reduced to J1939-only, and
all EMP/HVPC/UDC encoding validation tests were removed entirely.

### validate_dump_messages.sh — Protocol Array Reduction

The protocols array changed from `("emp" "hvpc" "udc" "j1939")` to
`("j1939")`. The test structure was preserved since it's protocol-agnostic
— it validates metadata flags (`--comments`, `--enums`, `--full`,
`--verbose`) which work the same regardless of protocol.

### Makefile — Selective Target Removal

The Makefile was adapted by removing proprietary targets while preserving
the overall structure (help system, variable definitions, target groupings).
The `validate-integration` target was changed from running HVPC+EMP
simulator tests to running only the J1939 simulator test. The
`validate-cli` tool list was updated to the 5 cando-rs CLI tools (was 8
proteus-rs tools including proprietary simulators).

### Playwright Test — Kept as Placeholder

`test_playwright_webui.sh` was kept as a placeholder that gracefully
exits if no `playwright-test` binary is found. The rationale: a WebUI
may be added to cando-rs in the future, and having the test infrastructure
skeleton in place reduces friction when that happens.

### CHANGELOG.md — Fresh Start

Rather than copying proteus-rs history, a fresh CHANGELOG was created with
a single v0.1.0 entry. A heritage note mentions proteus-rs by name — this
is the only place in the codebase where "proteus" appears in functional
content (as opposed to git history).

## Naming Conventions

All proteus-rs naming was systematically replaced:

| Context | proteus-rs | cando-rs |
|---------|-----------|----------|
| Package name | `proteus-rs` | `cando-rs` |
| Rust identifiers | `proteus_rs` | `cando_rs` |
| Display name | `Proteus-RS` | `Cando-RS` |
| Config file | `proteus.yaml` | `cando.yaml` |
| Meta package | `proteus-meta` | `cando-meta` |
| Udev rules | `99-proteus-can.rules` | `99-cando-can.rules` |
| Sudoers file | `proteus-testing` | `cando-testing` |

See `doc/porting-binary-mapping.md` for the complete binary name mapping.
