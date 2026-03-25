# Cando CAN Simulators Documentation

**Purpose**: Comprehensive guide to CAN device simulators in the Cando-RS project  
**Status**: Current as of 2025-01-20  
**Version**: 2.0

---

## 📋 Table of Contents

- [Overview](#overview)
- [Available Simulators](#available-simulators)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Protocol Support](#protocol-support)
- [Common Features](#common-features)
- [Individual Simulator Details](#individual-simulator-details)
- [WebSocket API](#websocket-api)
- [Testing Integration](#testing-integration)
- [Architecture](#architecture)

---

## Overview

Cando-RS provides four CAN device simulators for testing and development:

1. **EMP Simulator** - Electric Motor Power devices (fans and pumps)
2. **HVPC Simulator** - High Voltage Power Control units
3. **UDC Simulator** - Universal DC converter devices
4. **J1939 Simulator** - SAE J1939 Engine Control Units

All simulators share common infrastructure via `cando-simulator-common` crate and are **configuration-driven** using `cando.yaml`.

### Key Features

✅ **Configuration-Driven**: Uses `cando.yaml` for all device settings  
✅ **Multi-Protocol**: EMP supports J1939, proprietary, and hybrid modes  
✅ **Realistic Physics**: Speed ramping, thermal simulation, load models  
✅ **WebSocket API**: Real-time state broadcasting and remote control  
✅ **Fault Injection**: Simulate various fault conditions  
✅ **Integration Testing**: Designed for automated tier2/tier2-physical tests  
✅ **Production Ready**: 100% test pass rate, zero warnings

---

## Available Simulators

### EMP Simulator (`emp-simulator`)

Simulates EMP (Electric Motor Power) devices - fans and pumps.

**Protocol Support**:
- **J1939** (default) - SAE J1939 vendor-specific PGNs (32000, 62320, 64513)
- **Proprietary** - Legacy EMP-specific protocol (MCM, MSM1/2/3, MET)
- **Hybrid** - Both protocols simultaneously (testing only)

**Variants**: Fan, Pump  
**Hardware Validated**: J1939 mode confirmed against real EMP devices (2025-11-07)

### HVPC Simulator (`hvpc-simulator`)

Simulates High Voltage Power Control units.

**Protocol**: Proprietary  
**Features**: Voltage/current control, HVIL monitoring, fault simulation

### UDC Simulator (`udc-simulator`)

Simulates Universal DC Converter devices.

**Protocol**: Proprietary  
**Features**: DC-DC conversion, multiple voltage outputs, efficiency modeling

### J1939 Simulator (`j1939-simulator`)

Simulates SAE J1939 Engine Control Units.

**Protocol**: Standard J1939  
**Features**: Engine parameters, transmission control, diagnostic messages (DM1)

---

## Quick Start

### Prerequisites

```bash
# Build all simulators
cargo build --release --workspace

# Setup virtual CAN interface (for testing)
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

### Configuration-Driven Usage (Recommended)

All simulators now use `cando.yaml` configuration:

```bash
# Start EMP fan using device definition from config
./target/release/emp-simulator \
  --config cando.yaml \
  --device-name "EMP Test Device"

# Start with interface override
./target/release/emp-simulator \
  --config cando.yaml \
  --device-name "EMP Test Device" \
  --interface vcan0

# Start HVPC simulator
./target/release/hvpc-simulator \
  --config cando.toml \
  --device-name "HVPC Test Device"

# Start J1939 ECU
./target/release/j1939-simulator \
  --config cando.toml \
  --device-name "J1939 Test ECU"

# Start UDC simulator
./target/release/udc-simulator \
  --config cando.toml \
  --device-name "UDC Test Device"
```

### Legacy Command-Line Usage

Simulators still support direct CLI arguments:

```bash
# EMP with CLI args (bypasses config file)
./target/release/emp-simulator \
  --interface vcan0 \
  --device-id 0x82 \
  --protocol j1939 \
  --variant fan \
  --websocket-port 10754

# HVPC with CLI args
./target/release/hvpc-simulator \
  --interface vcan0 \
  --device-id 0x8A \
  --websocket-port 10755
```

---

## Configuration

### Configuration File Structure

Simulators use `cando.toml` for device definitions:

```toml
# Example EMP device configuration
[[devices]]
name = "EMP Test Device"
friendly_name = "EMP Test Simulator (Virtual)"
type = "emp"
device_id = "0x82"
variant = "fan"
interface = "vcan0"
protocol = "j1939"                # j1939, proprietary, or hybrid
websocket_port = 10754
enabled = true
description = "EMP simulator for tier2 virtual integration testing"
hardware_present = false
tags = ["test", "virtual", "emp", "tier2", "j1939"]
owner = "Automated Testing"
notes = "J1939 protocol matches real hardware behavior validated 2025-11-07"
```

### Configuration Search Path

Simulators search for configuration in this order:
1. `--config <path>` (command-line argument)
2. `./cando.toml` (current directory)
3. `~/.config/cando/cando.toml` (user config)
4. `/etc/cando/cando.toml` (system config, Unix only)

### Device Selection

Use `--device-name` to reference any device by name or friendly_name:

```bash
# By name field
--device-name "EMP Test Device"

# By friendly_name field
--device-name "EMP Test Simulator (Virtual)"
```

---

## Protocol Support

### EMP Protocol Modes

The EMP simulator supports three protocol modes:

#### 1. J1939 Mode (Default)

**Status**: Hardware validated (2025-11-07)  
**Configuration**: `protocol = "j1939"`

Uses SAE J1939 vendor-specific PGNs:
- **PGN 32000** (0x7D00): EMP Command message (~4 Hz)
- **PGN 62320** (0xF370): EMP Status 2 message (10 Hz, primary)
- **PGN 64513** (0xFC01): EMP Status 1 message (1 Hz)
- **PGN 65226** (0xFECA): DM1 Active Diagnostic Trouble Codes (1 Hz)
- **PGN 60928** (0xEE00): Address Claimed (once at startup)

**Real Hardware Validation**: Hardware capture analysis showed 100% J1939 operation with zero proprietary messages in 3,198 captured frames.

#### 2. Proprietary Mode (Legacy)

**Status**: Legacy testing support only  
**Configuration**: `protocol = "proprietary"`

Uses EMP-specific proprietary messages:
- **MCM**: Motor Command Message
- **MSM1**: Motor Status Message 1
- **MSM2**: Motor Status Message 2 (10 Hz)
- **MSM3**: Motor Status Message 3
- **MET**: Measured External Temperature (10 Hz)

**Note**: Real EMP hardware does NOT use proprietary protocol. This mode is maintained for backward compatibility and testing only.

#### 3. Hybrid Mode (Testing)

**Status**: Testing and development only  
**Configuration**: `protocol = "hybrid"`

Sends BOTH J1939 and proprietary messages simultaneously. Useful for:
- Protocol compatibility testing
- Legacy system migration
- Comprehensive message decoding validation

**Note**: Real hardware operates in J1939 mode only. Hybrid mode is for testing flexibility.

### Other Protocol Support

- **HVPC**: Proprietary protocol only
- **UDC**: Proprietary protocol only
- **J1939-ECU**: Standard SAE J1939 protocol

---

## Common Features

All simulators share these features via `cando-simulator-common`:

### WebSocket API

Real-time state broadcasting and remote control via WebSocket.

**Default Port**: Configurable per device (10754-10768 range)  
**Endpoint**: `ws://localhost:<port>/ws`

**Message Types**:
- **State Updates**: Broadcast every 100ms with current device state
- **Commands**: Remote control (power, RPM, faults)
- **Query**: On-demand state queries

Example WebSocket state query:
```bash
# Query simulator state via WebSocket
./scripts/integration/query_websocket_state.sh "EMP Test Device" 10754
```

### Physics Simulation

All simulators include realistic physics:

**Speed/RPM Control**:
- Configurable acceleration/deceleration rates
- Exponential ramping for realistic behavior
- Load-dependent response

**Thermal Simulation**:
- Ambient → Operating temperature transitions
- Load-dependent thermal behavior
- Configurable time constants (typically 60 seconds)

**Power Modeling**:
- Realistic power consumption curves
- Load-based efficiency
- Voltage/current relationships

### Fault Injection

Simulate various fault conditions for testing:
- Overcurrent conditions
- Overvoltage/undervoltage
- Overtemperature
- Communication failures
- HVIL (High Voltage Interlock) faults

### Logging and Debug

**Debug Mode**: `--debug` flag enables detailed logging
- CAN message transmission/reception
- State transitions
- Command processing
- WebSocket connections

**Log Format**: Structured logging with timestamps

---

## Individual Simulator Details

### EMP Simulator

**Detailed Documentation**: See `EMP_SIMULATOR.md` for comprehensive EMP simulator guide

**Executable**: `emp-simulator`  
**Configuration Type**: `type = "emp"`

**Variants**:
- **Fan**: Max 4350 RPM (HV spec), 3A current, thermal 30°C → 50°C under load
- **Pump**: Max 4350 RPM (HV spec), 3A current, thermal 50°C → 30°C under load

**Power Model**: Cubic relationship P = 3000W × (RPM/4350)³

**Specifications**: Based on EMP HV datasheets (15-inch high voltage fan/pump)
- Voltage: 450-850 VDC (nominal 600V)
- Current: 3A continuous @ 450V
- Max Power: 3kW
- RPM Range: 500-4350 RPM

**CLI Options**:
```
--config <FILE>              Path to cando.toml
--device-name <NAME>         Device name from config
--interface <INTERFACE>      CAN interface (e.g., vcan0)
--device-id <ID>             Device ID (0x80-0xFF)
--protocol <PROTOCOL>        j1939, proprietary, or hybrid
--variant <VARIANT>          fan or pump
--websocket-port <PORT>      WebSocket server port
--debug                      Enable debug logging
```

### HVPC Simulator

**Detailed Documentation**: See `HVPC_SIMULATOR.md` for comprehensive HVPC simulator guide

**Executable**: `hvpc-simulator`  
**Configuration Type**: `type = "hvpc"`

**Features**:
- High voltage power control simulation
- HVIL monitoring
- Voltage/current regulation
- Multiple fault modes

**CLI Options**: Similar to EMP, minus protocol selection

### UDC Simulator

**Detailed Documentation**: See `UDC_SIMULATOR.md` for comprehensive UDC simulator guide

**Executable**: `udc-simulator`  
**Configuration Type**: `type = "udc"`

**Features**:
- DC-DC conversion simulation
- Multiple voltage outputs
- Efficiency modeling
- Load-dependent behavior

### J1939 Simulator

**Detailed Documentation**: See `J1939_SIMULATOR.md` for comprehensive J1939 simulator guide

**Executable**: `j1939-simulator`  
**Configuration Type**: `type = "j1939"`

**Features**:
- SAE J1939 engine control unit
- Standard J1939 PGNs (engine speed, torque, temperature)
- DM1 diagnostic messages
- Address claiming

---

## WebSocket API

All simulators provide a WebSocket API for real-time monitoring and control.

### Connection

```javascript
const ws = new WebSocket('ws://localhost:10754/ws');

ws.onopen = () => {
    console.log('Connected to simulator');
};

ws.onmessage = (event) => {
    const state = JSON.parse(event.data);
    console.log('Device state:', state);
};
```

### State Message Format (EMP Example)

```json
{
  "device_id": 130,
  "ignition_enable": true,
  "rpm": 1500.0,
  "target_rpm": 1500.0,
  "power": 458.5,
  "temperature": 45.2,
  "voltage": 600.0,
  "current": 0.764,
  "direction": 1,
  "messages": 1234
}
```

### Command Messages

Send JSON commands to control the simulator:

```json
{
  "command": "set_rpm",
  "value": 2000
}
```

---

## Testing Integration

Simulators are designed for integration with tier2 testing:

### Tier2 Virtual Tests

```bash
# Runs all 4 simulators on vcan0
make tier2

# With visual monitoring
WITH_WEBUI=1 make tier2
```

### Tier2 Physical Tests

```bash
# Runs simulators on physical CAN interfaces (can0/can1)
make tier2-physical

# With visual monitoring
WITH_WEBUI=1 make tier2-physical
```

### Interactive Testing

```bash
# EMP J1939 motor control demonstration
make test-webui-interactive

# Custom cycle count
make test-webui-interactive CYCLES=5
```

### Integration Test Scripts

Simulators are started by integration test scripts:
- `scripts/integration/tier2_*.sh` - Tier2 test orchestration
- `scripts/integration/lib/simulator_helpers.sh` - Simulator start/stop functions
- `scripts/integration/control_emp_j1939_motors.sh` - Motor control script

**Configuration-Driven Pattern**: All test scripts query `cando.toml` for device configurations, ports, and interfaces. No hardcoded values.

---

## Architecture

### Component Structure

```
┌─────────────────────────────────────────────────┐
│         Individual Simulators                   │
│  (emp, hvpc, udc, j1939-simulator)             │
│                                                 │
│  ┌──────────────────────────────────────────┐  │
│  │  Device-Specific Logic                   │  │
│  │  - Protocol implementation               │  │
│  │  - Physics models                        │  │
│  │  - Device states                         │  │
│  └──────────────┬───────────────────────────┘  │
│                 │                               │
│  ┌──────────────▼───────────────────────────┐  │
│  │  cando-simulator-common                │  │
│  │  - WebSocket server                      │  │
│  │  - CAN frame utilities                   │  │
│  │  - CLI argument parsing                  │  │
│  │  - State management traits               │  │
│  │  - Physics utilities (ramping, thermal)  │  │
│  └──────────────┬───────────────────────────┘  │
│                 │                               │
└─────────────────┼───────────────────────────────┘
                  │
   ┌──────────────▼───────────────────────────┐
   │  cando-messages                        │
   │  - Generated message types               │
   │  - encode_real() / decode_real()         │
   │  - Protocol definitions                  │
   └──────────────┬───────────────────────────┘
                  │
   ┌──────────────▼───────────────────────────┐
   │  cando-config                          │
   │  - Configuration loading                 │
   │  - Device queries                        │
   │  - Environment management                │
   └──────────────────────────────────────────┘
```

### cando-simulator-common Library

Shared infrastructure eliminating code duplication:

**Modules**:
- `websocket.rs` - Generic WebSocket server with state broadcasting
- `can_frame.rs` - CAN frame creation utilities
- `cli.rs` - Common CLI argument definitions
- `physics.rs` - Physics simulation (ramping, thermal)

**Benefits**:
- ~600 lines of code elimination across simulators
- Consistent behavior and APIs
- Single source of truth for common patterns
- Easier maintenance and testing

**Usage**: All simulators depend on `cando-simulator-common.workspace = true`

---

## Migration Notes

### From Old CLI to Configuration-Driven

**Old Approach** (still works):
```bash
emp-simulator vcan0 --device-type fan --device-id 0x8A
```

**New Approach** (recommended):
```bash
emp-simulator --config cando.toml --device-name "EMP Test Device"
```

### Protocol Configuration Migration

EMP devices should use J1939 protocol to match hardware:

**Old**: `protocol = "proprietary"` (legacy)  
**New**: `protocol = "j1939"` (hardware-validated default)

See `doc/REALITY_BASED_EMP_PROTOCOL_CONFIG.md` for complete migration details.

---

## Troubleshooting

### Simulator Won't Start

**Check CAN interface exists**:
```bash
ip link show vcan0
```

**Create if missing**:
```bash
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```

### Configuration Not Found

**Check search paths**:
```bash
# Current directory
ls -la cando.toml

# User config
ls -la ~/.config/cando/cando.toml

# Or provide explicit path
--config /path/to/cando.toml
```

### Port Already in Use

**Check WebSocket port availability**:
```bash
lsof -i :10754
```

**Use different port**:
```bash
--websocket-port 10800
```

### Protocol Mismatch

If testing with real EMP hardware:
- Use `protocol = "j1939"` (validated against hardware)
- Avoid `proprietary` mode (not used by real devices)
- See hardware capture analysis in `doc/REALITY_BASED_EMP_PROTOCOL_CONFIG.md`

---

## References

### Documentation

- **Configuration Guide**: `doc/AI-WORKFLOW-GUIDE.md` - Configuration-driven pattern
- **EMP Protocol Config**: `doc/REALITY_BASED_EMP_PROTOCOL_CONFIG.md` - J1939 migration
- **WebUI Integration**: `doc/REALTIME_MONITORING_WEBUI.md` - Visual monitoring
- **Integration Testing**: `doc/AI-WORKFLOW-GUIDE.md` - Testing requirements

### Individual Simulator Documentation

- **EMP Simulator**: `EMP_SIMULATOR.md` - Complete EMP simulator guide (J1939/proprietary/hybrid protocols)
- **HVPC Simulator**: `HVPC_SIMULATOR.md` - Complete HVPC simulator guide (high voltage power control)
- **UDC Simulator**: `UDC_SIMULATOR.md` - Complete UDC simulator guide (DC-DC conversion)
- **J1939 Simulator**: `J1939_SIMULATOR.md` - Complete J1939 simulator guide (SAE J1939 ECU)

### Example Configurations

- `cando.toml` - Unified configuration with multiple environments
- Use `--environment` flag to select test or production configurations

### Source Code

- `emp-simulator/src/main.rs` - EMP simulator implementation
- `hvpc-simulator/src/main.rs` - HVPC simulator implementation
- `udc-simulator/src/main.rs` - UDC simulator implementation
- `j1939-simulator/src/main.rs` - J1939 ECU simulator implementation
- `cando-simulator-common/` - Shared infrastructure library

### Hardware Specifications

- **EMP HV Fan**: https://www.emp-corp.com/product/15-inch-high-voltage-fan/
- **EMP HV Pump**: https://www.emp-corp.com/product/wp150-high-voltage/
- **SAE J1939**: Standard for heavy-duty vehicle communications

---

## Contributing

### Adding New Simulators

1. Create new crate: `cargo new --bin new-device-simulator`
2. Add dependency: `cando-simulator-common.workspace = true`
3. Implement device-specific logic
4. Use common WebSocket, CAN, and CLI infrastructure
5. Add to `cando.toml` configuration
6. Create integration tests

### Testing Changes

```bash
# Unit tests
cargo test --workspace

# Integration tests
make tier1
make tier2

# Specific simulator
cargo test -p emp-simulator
```

---

**Document Version**: 2.0  
**Last Updated**: 2025-01-20  
**Status**: Current - Reflects production state after WebUI migration and EMP protocol validation