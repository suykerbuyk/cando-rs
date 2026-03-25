# Rust-Can-Util Design Document

## Overview

`rust-can-util` is the primary command-line interface (CLI) utility within the Cando-RS workspace, designed for CAN bus message processing and analysis. It serves as a versatile tool for encoding CAN messages based on DBC (Database Container) files, generating hexadecimal dumps and ASCII descriptions, and optionally transmitting messages over a specified CAN interface. This utility is particularly suited for industrial applications involving motor control systems, such as those using J1939-compatible protocols for devices like fans and pumps.

Key features include:
- Parsing DBC files to extract message definitions.
- Encoding messages with embedded device IDs (supporting both decimal and hexadecimal input).
- Flexible output formats: plain text (with hex dumps and ASCII descriptions), CSV, or JSON.
- Optional transmission of encoded messages via SocketCAN interfaces (e.g., `vcan0` or physical `can0`).
- Complete protocol support for EMP, HVPC (with full 6-opcode multiplexed payload support), UDC, and J1939.
- Integration with the shared `cando-core` library for core parsing and encoding logic.

This tool is one of 12 binary utilities in the Cando-RS ecosystem, emphasizing reusability, error handling, and extensibility. It supports multiple CAN protocols including EMP (Electric Motor Power), HVPC (High Voltage Power Control) with complete multiplexed command support, UDC (Unidirectional DC Converter), and J1939, making it ideal for development, testing, and debugging in embedded systems.

## Architecture

`rust-can-util` follows a modular, workspace-integrated architecture, leveraging the Cando-RS shared infrastructure for consistency across utilities. It is structured as a binary crate with a single entry point (`main.rs`), relying on `cando-core` for shared functionality.

### High-Level Components
- **CLI Parsing**: Uses `clap` for argument handling, with shared `CommonArgs` from `cando-core/cli.rs`.
- **DBC Parsing and Encoding**: Delegates to `cando-core` functions like `parse_dbc` and `encode_message`.
- **Output Handling**: Supports multiple formats via a formatter trait in `cando-core/output.rs`.
- **CAN Transmission**: Utilizes `socketcan` for optional message sending.
- **Error Management**: Centralized error handling with `anyhow` and a shared `handle_error` function.

### File Structure
```
rust-can-util/
├── Cargo.toml                  # Crate metadata and dependencies
└── src/
    └── main.rs                 # Main entry point, argument parsing, core logic, and unit tests
```

- No additional modules are defined in this crate; all shared logic is imported from `cando-core`.
- Tests are embedded in `main.rs` under a `#[cfg(test)]` module.

### Workflow
1. Parse CLI arguments using `clap`.
2. Validate inputs (e.g., DBC file existence, device ID range 0-255).
3. Load and parse the DBC file via `cando-core`.
4. Encode the message with provided fields and device ID.
5. Generate output (hex dump, ASCII description, or formatted data).
6. If specified, transmit the frame over the CAN interface.

This design promotes separation of concerns: CLI-specific logic stays in `rust-can-util`, while CAN protocol handling is abstracted in `cando-core`.

## CLI Arguments

The CLI is defined using `clap` with the following arguments (from `main.rs`):

- **Common Arguments** (from `cando-core::CommonArgs`):
  - `--output <FORMAT>`: Output format (text, csv, json).
  - `--csv`: Shortcut for CSV output.
  - `--json`: Shortcut for JSON output.
  - `--dbc-path <PATH>`: Path to DBC file (default: `../dbc/EMP.dbc`).

- **Utility-Specific Arguments**:
  - `--device-id <ID>`: Device ID to embed (hex like `0x8A` or decimal 0-255). Embedded as source address in the CAN ID.
  - `--message <NAME>`: Name of the CAN message from the DBC file (e.g., `MCM_MotorCommandMessage`).
  - `--fields <KEY=VALUE>`: Optional signal values as comma-separated pairs (e.g., `"speed=50,status=1"`). Defaults to empty.
  - `--send-interface <INTERFACE>`: Optional CAN interface for transmission (e.g., `vcan0`).
  - `--generate-manpage`: Hidden flag for man page generation (requires `manpages` feature).

Example Usage:
```bash
# EMP Protocol - Motor Control
cargo run --release -p rust-can-util -- --dbc-path dbc/EMP.dbc --device-id 0x8A --message MCM_MotorCommandMessage --fields "MCM_OnOffDirectionCommand=1,MCM_MotorSpeedCommand=1500" --send-interface vcan0

# HVPC Protocol - High Voltage Power Control (Opcode 2 - NED Reset)
cargo run --release -p rust-can-util -- --device-id 0x8A --message HVPC_Command --fields "HVPC_Command_Opcode=2" --send-interface vcan0

# HVPC Protocol - Valve Command with Float Value (Opcode 4)
cargo run --release -p rust-can-util -- --device-id 0x8A --message HVPC_Command --fields "HVPC_Command_Opcode=4,hvpc_valvecmd_valvecmd=0.5,hvpc_valvecmd_valvemode=1" --send-interface vcan0
```

For full details, refer to the generated man page: `man rust-can-util`.

## Key Functions and Implementation

### Main Entry Point (`main`)
- Collects raw arguments and handles special flags (e.g., man page generation).
- Parses arguments into `Args` struct.
- Calls `run` and handles errors via `handle_error`.

### Core Logic (`run`)
- Validates DBC file existence.
- Parses device ID (supports hex/decimal with range check 0-255).
- Parses DBC and encodes message using `cando-core::encode_message`.
- Handles output formatting (text with hex dump/ASCII, or CSV/JSON).
- If `--send-interface` is provided, creates and transmits a `CanFrame` via `socketcan`.

### Helper Functions (Imported from `cando-core`)
- `parse_dbc`: Loads and parses message from DBC file.
- `encode_message`: Encodes message with device ID and field values.
- `get_output_format` / `get_formatter`: Determines and applies output format.
- `print_ascii_desc` / `print_hex_dump`: Generates human-readable outputs.

### Error Handling
- Uses `anyhow::Result` for propagation.
- Custom errors for invalid device IDs, missing files, or transmission failures.

### Man Page Generation
- Enabled via `--features manpages`.
- Uses `clap_mangen` to generate man pages from the `Args` command definition.

## Protocol Support

### HVPC (High Voltage Power Control) Protocol

The utility provides complete support for HVPC protocol commands with full multiplexed payload support for all 6 opcodes:

#### HVPC Command Opcodes
- **Opcode 0 - Channel Group Command**: Controls output channel groups with open/close masks and override signals
  ```bash
  rust-can-util --device-id 0x8A --message HVPC_Command --fields "HVPC_Command_Opcode=0,hvpc_chgrpcmd_closemask7_0=255,hvpc_chgrpcmd_openmask7_0=128"
  ```

- **Opcode 1 - Group HVIL Command**: High Voltage Interlock (HVIL) control with active/inactive masks
  ```bash
  rust-can-util --device-id 0x8A --message HVPC_Command --fields "HVPC_Command_Opcode=1,hvpc_hvilcmd_actmask7_0=255,hvpc_hvilcmd_inactmask7_0=0"
  ```

- **Opcode 2 - NED Reset Command**: Network Event Data reset functionality
  ```bash
  rust-can-util --device-id 0x8A --message HVPC_Command --fields "HVPC_Command_Opcode=2"
  ```

- **Opcode 3 - Shutdown Command**: Emergency shutdown and safe state control
  ```bash
  rust-can-util --device-id 0x8A --message HVPC_Command --fields "HVPC_Command_Opcode=3"
  ```

- **Opcode 4 - Valve Command**: Valve control with mode, command value (float), and temperature enable
  ```bash
  rust-can-util --device-id 0x8A --message HVPC_Command --fields "HVPC_Command_Opcode=4,hvpc_valvecmd_valvecmd=0.75,hvpc_valvecmd_valvemode=2"
  ```

- **Opcode 5 - Reprogram Initialization Command**: Firmware reprogramming initialization with key
  ```bash
  rust-can-util --device-id 0x8A --message HVPC_Command --fields "HVPC_Command_Opcode=5,hvpc_reprogramcmd_keyreprogram=12345"
  ```

#### HVPC Signal Field Names
Each opcode supports specific signal fields. The utility automatically handles the multiplexed payload based on the opcode value:

- **Opcode 0 fields**: `hvpc_reserved_1c`, `hvpc_reserved_1b`, `hvpc_reserved_1a`, `hvpc_chgrpcmd_closemask11_8`, `hvpc_chgrpcmd_closemask7_0`, `hvpc_chgrpcmd_openmask11_8`, `hvpc_chgrpcmd_openmask7_0`, `hvpc_chgrpcmd_gifoverride`
- **Opcode 1 fields**: `hvpc_reserved_3c`, `hvpc_reserved_3b`, `hvpc_reserved_3a`, `hvpc_hvilcmd_actmask11_8`, `hvpc_hvilcmd_actmask7_0`, `hvpc_hvilcmd_inactmask11_8`, `hvpc_hvilcmd_inactmask7_0`
- **Opcode 2 fields**: `hvpc_reserved_3d`
- **Opcode 3 fields**: `hvpc_reserved_3e`
- **Opcode 4 fields**: `hvpc_reserved_5a`, `hvpc_valvecmd_valvemode`, `hvpc_valvecmd_valvecmd` (float), `hvpc_valvecmd_enableelevtemp`
- **Opcode 5 fields**: `hvpc_reserved_5b`, `hvpc_reprogramcmd_keyreprogram`

### EMP (Electric Motor Power) Protocol

Standard support for motor control messages like `MCM_MotorCommandMessage`.

### UDC and J1939 Protocols

Additional protocol support for comprehensive CAN bus testing.

## Testing

Unit tests are embedded in `main.rs` under `#[cfg(test)] mod tests`. They focus on critical paths like argument parsing and error handling. Currently, there are 4 tests:

- `test_device_id_parsing_decimal`: Verifies decimal input (e.g., "138" → 138).
- `test_device_id_parsing_hex`: Verifies hex input (e.g., "0x8A" → 138).
- `test_device_id_out_of_range`: Checks parsing succeeds but simulates range validation (>255).
- `test_run_with_invalid_dbc`: Ensures error on nonexistent DBC path.

Run tests with `cargo test -p rust-can-util`. These are unit tests; integration tests could be added in a `tests/` directory for end-to-end CLI scenarios.

Test coverage emphasizes recent features (e.g., hex parsing) and failure modes. Expansion opportunities include mocking `socketcan` for transmission tests.

## Dependencies

- **Production**:
  - `anyhow`: Error handling.
  - `chrono`: Timestamps in outputs.
  - `clap`: CLI parsing.
  - `clap_mangen`: Man page generation (feature-gated).
  - `cando-core`: Shared CAN logic (workspace dependency).
  - `socketcan`: CAN interface interactions.

- **Dev Dependencies**: None explicitly; tests use built-in `#[test]` framework.

All versions are managed via the workspace `Cargo.toml` for consistency.

## Integration with Cando-Core

`rust-can-util` is tightly integrated with `cando-core` and `cando-messages`, the shared libraries of the Cando-RS workspace:
- Imports core types (e.g., `CommonArgs`, `OutputFormat`) and functions (e.g., `parse_dbc`, `encode_message`).
- Uses protocol-specific encoding functions like `try_encode_hvpc()` for complete HVPC command support.
- Leverages the `encode_real()` methods from `cando-messages` for accurate CAN frame generation.
- Reuses error handling (`handle_error`) and utilities (e.g., formatters).
- Benefits from workspace-level testing and dependencies, ensuring consistency with other utilities like `emp-simulator` and `hvpc-simulator`.

This modular design allows `rust-can-util` to focus on CLI specifics while leveraging the ecosystem's robust CAN infrastructure. The Phase 2.5 enhancements added complete HVPC protocol support with multiplexed payload handling, making the utility capable of generating all HVPC command variations with proper signal encoding. Changes in `cando-core` and `cando-messages` (e.g., improved parsing, new protocol support) automatically propagate to this utility.