# Cando-RS — Open-Source CAN Bus Development Platform for Rust

Turn any DBC file into native Rust code your IDE actually understands — structs it autocompletes, types your compiler checks, and signals your debugger can inspect.

Cando-RS is a complete CAN bus development platform: a code generator, a set of CLI tools that replace the C-based can-utils, a physics-based J1939 simulator, and a library architecture designed as infrastructure you build your own CAN bus applications on top of.

**18 workspace crates · 10 binaries · 339 J1939 message definitions · 505,000 lines of generated type-safe Rust · 1,293 tests**

Dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

## Why Cando-RS

CAN bus tools parse untrusted data from physical hardware. A frame arrives as 8 bytes, and your code must extract bit fields, apply scaling, and present results. In C — the language of the standard can-utils — every extraction is manual pointer arithmetic. Every parse is an opportunity for an off-by-one error, a buffer overread, or a silent truncation.

Cando-RS replaces all of that with compile-time generated, type-safe Rust. The code generator reads industry-standard DBC specifications and produces native Rust modules — complete with structs, encoders, decoders, builders, and signal metadata. The generated code is checked into the repository. You do not need the DBC files to build the project. Clone, build, run.

DBC files are copyrighted by their vendors, but the generated Rust code is a derivative work — and derivative works are not encumbered by the original copyright. Once Cando-RS generates native Rust from a DBC specification, your project is completely decoupled from the original copyrighted files.

## Quick Start

```bash
# Build everything
cargo build --workspace

# Run any tool
cargo run --bin cando-dump -- --help
cargo run --bin cando-send -- --help
cargo run --bin cando-codegen -- status

# Run the full test suite
cargo test --workspace
```

For cross-compiled static binaries:

```bash
# x86_64 static binary
make build-musl-release

# aarch64 (Raspberry Pi, embedded Linux)
make build-aarch64-musl-release

# Debian packages with man pages and shell completions
make build-deb-amd64
make build-deb-arm64
```

## Code Generator

The code generator treats DBC files the way a compiler treats source code. It reads vendor DBC specifications and produces:

- **Rust structs** with typed fields for every message and signal
- **Encode/decode functions** that pack and unpack signals at the bit level
- **Builder patterns** for constructing messages with compile-time validation
- **Signal metadata** — units, ranges, scaling factors, value descriptions — available at runtime
- **Multiplexer-aware types** with opcode-based dispatching and validation

Every generated message carries compile-time signal metadata:

```rust
pub struct SignalMetadata {
    pub name: &'static str,
    pub start_bit: u64,
    pub signal_size: u64,
    pub byte_order: ByteOrder,     // LittleEndian or BigEndian
    pub value_type: ValueType,     // Signed or Unsigned
    pub factor: f64,               // physical = (raw * factor) + offset
    pub offset: f64,
    pub min: f64,
    pub max: f64,
    pub unit: &'static str,        // "rpm", "°C", "%", etc.
    pub value_descriptions: &'static [(u64, &'static str)],
    pub comment: &'static str,
}
```

The amplification ratio is roughly 170:1 between generator source and generated output. When a vendor ships an updated DBC file, drop it in, run the generator, and the Rust compiler tells you exactly what changed and where it propagates. The type system becomes your regression test for protocol changes.

### Integrity Tracking

The generator tracks integrity with a three-layer SHA-256 checksum system:

| Layer | Detects |
|---|---|
| DBC file hash | Vendor specification updates |
| Generator source hash | Algorithm changes that could cause silent drift |
| Output hash | Manual edits to generated code |

Up to 20 historical entries are maintained for regression detection. You always know exactly why a regeneration is needed.

```bash
# Check sync status for all protocols
cando-codegen status

# Regenerate a specific protocol
cando-codegen generate --protocol j1939

# Force regeneration (bypass checksum check)
cando-codegen generate --protocol j1939 --force

# Regenerate everything
cando-codegen generate-all

# Validate DBC/output alignment
cando-codegen validate
```

### PDU1/PDU2: Correct by Construction

J1939 messages come in two flavors based on the PDU Format byte in the 29-bit CAN ID:

- **PDU1** (PF < 240): destination-specific — strip both destination and source addresses when matching
- **PDU2** (PF >= 240): broadcast — strip only the source address, preserve the group extension

Use the wrong mask and PDU1 commands silently fail to decode while PDU2 status messages work fine. Cando-RS solves this at the type level — the encoder provides PDU-aware extract and embed functions that automatically detect the PDU type and apply the correct masking, verified by property-based tests across randomized inputs.

## CLI Tools

### cando-dump — CAN Traffic Monitor

Drop-in replacement for `candump` with protocol-aware decoding.

```bash
# Monitor multiple interfaces
cando-dump can0,vcan0

# Filter by CAN ID with masks
cando-dump can0,123:7FF                     # Only ID 0x123
cando-dump can0,100~700:7FF                 # Exclude range

# Decoded output with signal names, values, and units
cando-dump vcan0 --format decoded

# JSON output for test automation
cando-dump vcan0 --format json

# Timestamps: absolute (a), delta (d), zero-based (z), date (A)
cando-dump vcan0 -t d -c -a                 # Delta + color + ASCII

# Log to file with frame limit
cando-dump vcan0 -l -n 1000 --stats
```

**Features:** 3 output formats (candump ASCII, JSON, decoded) · 4 timestamp modes · per-interface CAN ID filtering with include/exclude masks · file logging with automatic timestamped filenames · color-coded output · frame counting · statistics reporting · timeout-based termination

### cando-send — CAN Frame Transmission

Drop-in replacement for `cansend` with file replay.

```bash
# Single frame modes
cando-send vcan0 123#DEADBEEF               # Standard frame
cando-send vcan0 5A1#11.22.33.44.55.66.77.88  # Dot notation
cando-send vcan0 00000123#R5                 # RTR frame
cando-send vcan0 456##311223344              # CAN FD frame

# File replay with rate control
cando-send vcan0 --file candump.log --rate 100
cando-send vcan0 --file traffic.json --interval 10 --verbose
```

**Features:** standard/extended/RTR/CAN FD frames · dot-separated data notation · file replay with format auto-detection (candump ASCII and JSON) · configurable replay rates and intervals · progress reporting

### cando-monitor — Real-Time Protocol Decoder

Real-time TUI that decodes CAN traffic against J1939 protocol definitions.

```bash
# Basic monitoring with decoding
cando-monitor vcan0

# Filter by message name patterns
cando-monitor vcan0 --filter EEC1,EEC2
cando-monitor vcan0 -f "DM" --decoded-only

# Raw hex + decoded side by side
cando-monitor vcan0 --show-raw --stats-interval 60
```

**Features:** real-time message decoding with engineering units · message name pattern filtering · decoded-only mode · raw hex dump display · per-message timing statistics

### cando-util — Interactive Message Builder

TUI that walks you through constructing CAN frames without manual hex calculation.

```bash
# Interactive builder
cando-util builder

# Quick lookup
cando-util builder --list-devices
cando-util builder --device "J1939 ECU" --list-messages
cando-util builder --device "J1939 ECU" --message EEC1 --show-fields

# Diagnostics
cando-util diagnostics --device-id 0x8A dm01
cando-util diagnostics --device-id 0x82 dm02 --send-interface vcan0
```

**Features:** device/message/field selection with search · real-time range validation · sentinel value detection · engineering unit display · diagnostic command support (DM01-DM03) · non-interactive mode with JSON/CSV output

### cando-log-analyzer — Log File Analysis

Parse and analyze candump log files with full J1939 decoding.

```bash
# Decode a log file
cando-log-analyzer candump.log --decoded-only

# Filter by device and message
cando-log-analyzer candump.log --device-filter 0x82,0x8A --message-filter "EEC,DM"

# Statistical and timing analysis
cando-log-analyzer candump.log --statistics --timing-analysis

# Export decoded data
cando-log-analyzer candump.log --export-json decoded.json
cando-log-analyzer candump.log --export-csv signals.csv
```

**Features:** candump log parsing · device ID and message name filtering · decoded-only mode · statistical and timing analysis · JSON and CSV export

### cando-dump-messages — Message Database Inspector

Introspect the compiled message database — 2,146 J1939 message definitions with full signal metadata.

```bash
# Browse all messages
cando-dump-messages --protocol j1939

# Export as JSON or CSV
cando-dump-messages --json --full
cando-dump-messages --csv --rust-names

# Include comments and enum descriptions
cando-dump-messages --verbose
```

### cando-cfg — Configuration Query Tool

Query the shared `cando.yaml` configuration from shell scripts and automation.

```bash
# Device properties
cando-cfg get-device-id "J1939 Test ECU"     # → 0x42
cando-cfg get-port "J1939 Test ECU"          # → 10999
cando-cfg get-interface "J1939 Test ECU"     # → vcan0

# Listing and inspection
cando-cfg list-devices --enabled-only
cando-cfg show-device "Test Device" --format json
cando-cfg list-environments
```

### cando-ws-query — WebSocket State Validation

Query and validate simulator state for test automation.

```bash
# Query full state
cando-ws-query --port 10999 query

# Extract specific field
cando-ws-query --port 10999 extract engine_rpm

# Validate with tolerance
cando-ws-query --port 10999 validate temperature 25.5 --tolerance 0.5

# Wait for a specific CAN message
cando-ws-query --port 10999 wait-for-message 0x18FCCC0F --timeout-ms 5000

# Simulator control
cando-ws-query --port 10999 pause
cando-ws-query --port 10999 resume
```

## J1939 Simulator

The simulator models first-order system dynamics — not instant state changes. Each physical quantity follows exponential ramp functions with its own time constant. A motor command does not instantly set the RPM; the simulated motor accelerates along a realistic curve. Thermal models lag behind electrical events the way real hardware does.

```bash
# Start simulator on virtual CAN interface
cando-j1939-sim --interface vcan0

# Custom device ID and WebSocket port
cando-j1939-sim --interface vcan0 --device-id 0x82 --websocket-port 10999
```

### Simulated Subsystems

| Subsystem | Signals | Examples |
|---|---|---|
| Engine | RPM, load, coolant/exhaust temp, throttle | 600–3000 RPM range, thermal lag |
| High-Voltage Energy Storage | SoC, cell voltages, temperatures, fan control | 8-parameter fan modeling |
| DC-DC Converter | High/low voltage, power limiting | Operational command processing |
| Transmission | Gear selection, shaft speeds | 6-gear, input/output tracking |
| Braking (ABS/AEBS/EBC) | Pedal position, pressure, wheel speed | 0–1250 kPa, ABS activation |
| Sensors (WAND/LDISP) | Wand angle, linear displacement | -250° to 252°, 0–6425 mm |
| Diagnostics (J1939-73) | DM01/DM02 faults, DM03 clearing | Full DTC lifecycle |

### WebSocket Control API

Control the simulator remotely via JSON messages:

```json
{"type": "SetEngineRPM", "rpm": 2500.0}
{"type": "SetWandAngle", "angle": 180.5}
{"type": "SetBrakePedal", "position": 75.0}
{"type": "SetCrash", "detected": true, "crash_type": 5}
```

Integration tests query simulator state through WebSocket to verify end-to-end behavior on virtual CAN interfaces — no physical hardware required for CI.

## Workspace Architecture

Cando-RS is designed as infrastructure you build on. The crates separate concerns so you can depend on exactly what you need:

| Crate | Type | Purpose |
|---|---|---|
| `cando-messages` | Library | Generated CAN message types with encode/decode/metadata (505K LOC) |
| `cando-config` | Library | YAML-based device and environment configuration |
| `cando-core` | Library | Shared types, CLI helpers, and re-exports |
| `cando-can-monitor` | Library | Protocol-aware CAN decoding layer |
| `cando-simulator-common` | Library | Trait interfaces for building custom simulators |
| `cando-codegen` | Binary | DBC-to-Rust code generator |
| `cando-send` | Binary | CAN frame transmission (cansend replacement) |
| `cando-dump` | Binary | CAN traffic monitoring (candump replacement) |
| `cando-monitor` | Binary | Real-time TUI protocol decoder |
| `cando-util` | Binary | Interactive message builder and diagnostics |
| `cando-j1939-sim` | Binary | J1939 device simulator with physics models |
| `cando-cfg` | Binary | Configuration query tool |
| `cando-log-analyzer` | Binary | Candump log file analysis |
| `cando-dump-messages` | Binary | Message database introspection |
| `cando-ws-query` | Binary | WebSocket state query and validation |
| `cando-meta` | Meta | Workspace packaging and metadata |

### Configuration System

A single `cando.yaml` file serves as the source of truth for device identity across all tools:

```yaml
devices:
  - name: "J1939 Test ECU"
    device_id: "0x42"
    can_interface: "vcan0"
    websocket_port: 10999
    enabled: true
    device_type: "j1939"
    tags: ["ecu", "test"]
```

Query from shell scripts, reference from Rust code, and know that every tool in the toolkit is pointing at the same device with the same parameters.

## Build System

### Development

```bash
make build-workspace        # Debug build + man pages
make test-all               # All unit tests
make validate               # Full CI validation (Tier 1, <15 min)
make codegen-all            # Regenerate changed protocols
```

### Cross-Compilation

Static musl binaries via Zig. Zero runtime dependencies. Copy to an embedded Linux system, run it, done.

```bash
make build-musl-release              # Static x86_64
make build-aarch64-musl-release      # Static ARM64 (Raspberry Pi)
make build-all-targets               # All architectures, debug + release
```

### Packaging

```bash
make build-deb-amd64        # Debian package for x86_64
make build-deb-arm64        # Debian package for aarch64
```

Debian packages include man pages, shell completions (bash, zsh, fish), and a postinst script that sets `CAP_NET_RAW` capabilities so CAN tools work without root.

### CI/CD

GitHub Actions builds on tag push, tests installation across Ubuntu 20.04/22.04/24.04 and Debian 11/12, and publishes releases with SHA256 checksums.

## Testing

The test suite is tiered:

| Tier | Scope | Hardware Required |
|---|---|---|
| Tier 1 | Unit tests, integration tests, CI validation | None |
| Tier 2 | Virtual CAN interfaces + physics simulator | Linux with vcan |
| Tier 2-Physical | Hardware-in-the-loop validation | Real CAN interfaces |

```bash
make test-all               # Tier 1: all unit tests
make validate               # Tier 1: full CI validation pipeline
```

Tests include unit tests, integration tests, property-based tests ([proptest](https://crates.io/crates/proptest)), and benchmarks ([criterion](https://crates.io/crates/criterion)).

## Documentation

The `doc/` directory contains 42 reference documents:

- **Architecture** — TUI design, message builder, multiplexer encoding, network gateway, WebUI routing
- **Build** — cross-compilation, Debian packaging, CAP_NET_RAW, system overview
- **Guides** — adding J1939 messages, codegen quick reference, configuration, SocketCAN setup, man pages
- **Simulators** — J1939 simulator design, physics models
- **Specifications** — multi-device support, J1939 expansion, PDU1 vs PDU2, field name migration
- **Testing** — integration framework, message verification, state query, synchronization, code coverage

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
