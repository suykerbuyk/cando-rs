# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-13

Initial release of Cando-RS, an open-source CAN bus toolkit with compile-time
code generation. Derived from the proteus-rs project with proprietary protocol
support removed.

### Added

- **J1939 protocol support** — full SAE J1939 message encoding/decoding with
  DBC-based code generation
- **Code generation** (`cando-codegen`) — compile-time Rust code generation from
  DBC files with type-safe message structs, signal accessors, and builder patterns
- **CLI tools**:
  - `cando-util` — multi-protocol CAN message builder and sender
  - `cando-dump` — CAN frame dumper with protocol-aware decoding
  - `cando-send` — simple CAN frame sender
  - `cando-monitor` — real-time CAN bus monitor with TUI
  - `cando-dump-messages` — message metadata inspector with comments, enums, and
    field documentation
  - `cando-cfg` — configuration management tool
  - `cando-log-analyzer` — CAN log file analyzer
  - `cando-ws-query` — WebSocket query tool for simulator state
- **J1939 simulator** (`cando-j1939-sim`) — virtual J1939 device for testing and
  development
- **Cross-compilation support** — x86_64 and aarch64, both glibc (dynamic) and
  musl (static via Zig toolchain)
- **Debian packaging** — cargo-deb based `.deb` generation for amd64 and arm64
- **Man page generation** — automatic man page generation for all CLI tools
- **Shell completions** — bash, zsh, and fish completions for all CLI tools
- **Tiered CI/CD** — Tier 1 (unit + integration) and Tier 2 (full-stack vcan)
  validation
