# J1939 Simulator Documentation

**Current as of**: 2025-01-20  
**Version**: 2.0

---

## Overview

The J1939 Simulator provides realistic simulation of SAE J1939-compliant devices for CAN bus testing. It emulates standard J1939 vehicle bus devices with comprehensive message support and stateful behavior.

**Quick Facts**:
- **Configuration-Driven**: Uses `cando.toml` for device definitions
- **SAE J1939 Standard**: Implements SAE J1939 vehicle bus protocol
- **Message Support**: CN (Crash Notification), WAND (Wand Angle), LDISP (Linear Displacement)
- **Diagnostic Messages**: DM1 (Active DTCs), DM3 (Clear/Reset Previously Active DTCs)
- **Stateful Simulation**: Realistic state transitions and behavior modeling
- **WebSocket API**: Real-time state broadcasting and remote control
- **Integration Ready**: Designed for tier2 automated testing

**See also**: `doc/simulators/README.md` for comprehensive simulator documentation

---

## Quick Start

### Basic Usage

```bash
# Configuration-driven (recommended)
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 Test ECU"

# Legacy CLI (still supported)
./target/release/j1939-simulator \
  --interface vcan0 \
  --device-id 0x8B \
  --websocket-port 10756
```

### Common Use Cases

```bash
# Start J1939 ECU for tier2 testing
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 Test ECU"

# Start with interface override
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 Test ECU" \
  --interface vcan0

# Start with custom device ID
./target/release/j1939-simulator \
  --interface vcan0 \
  --device-id 0x8A \
  --websocket-port 10756

# Enable debug logging
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 Test ECU" \
  --debug

# Start multiple J1939 devices
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 ECU 1"

./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 ECU 2"
```

---

## Configuration

### Device Definition in cando.toml

```toml
[[devices]]
name = "J1939 Test ECU"
friendly_name = "J1939 Test ECU (Virtual)"
type = "j1939"
device_id = "0x8B"              # J1939 device ID (0x00-0xFF)
interface = "vcan0"             # CAN interface
protocol = "j1939"              # SAE J1939 protocol
websocket_port = 10756          # WebSocket API port
enabled = true
description = "J1939 ECU simulator for tier2 virtual integration testing"
hardware_present = false
tags = ["test", "virtual", "j1939", "tier2", "ecu"]
owner = "Automated Testing"
notes = "SAE J1939-compliant ECU with crash notification and sensor support"
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
--device-name "J1939 Test ECU"

# By friendly_name field
--device-name "J1939 Test ECU (Virtual)"
```

### Multi-Device Configuration

For multi-device testing, define multiple J1939 devices:

```toml
[[devices]]
name = "J1939 ECU 1"
friendly_name = "J1939 Engine Controller"
type = "j1939"
device_id = "0x00"
interface = "vcan0"
websocket_port = 10756
enabled = true

[[devices]]
name = "J1939 ECU 2"
friendly_name = "J1939 Transmission Controller"
type = "j1939"
device_id = "0x03"
interface = "vcan0"
websocket_port = 10759
enabled = true

[[devices]]
name = "J1939 ECU 3"
friendly_name = "J1939 Body Controller"
type = "j1939"
device_id = "0x21"
interface = "vcan0"
websocket_port = 10760
enabled = true
```

---

## Command-Line Options

### Required Arguments

```
ARGUMENTS:
  <INTERFACE>  CAN interface name (when not using --config)
```

### Optional Arguments

```
OPTIONS:
  -c, --config <FILE>              Path to cando.toml configuration file
  -n, --device-name <NAME>         Device name from config (required with --config)
  -I, --device-id <ID>             Device ID (0x00-0xFF) [default: 0x8A]
      --interface <INTERFACE>      Override CAN interface from config
  -p, --websocket-port <PORT>      WebSocket port [default: 8080]
  -d, --debug                      Enable debug output
  -h, --help                       Print help
  -V, --version                    Print version
```

### Device ID Range

**J1939 supports full address range**:
- **0x00-0xF7**: Standard device addresses (247 devices)
- **0xF8-0xFD**: Reserved addresses
- **0xFE**: NULL address (not assigned)
- **0xFF**: Global/broadcast address

**Common Device IDs**:
- `0x00` - Engine #1
- `0x01` - Engine #2
- `0x03` - Transmission #1
- `0x21` - Body Controller
- `0x31` - Cab Controller #1
- `0x8A-0x8F` - Custom/test devices

### Legacy CLI Examples

For backward compatibility, direct CLI arguments are still supported:

```bash
# Engine controller
./target/release/j1939-simulator \
  --interface vcan0 \
  --device-id 0x00 \
  --websocket-port 10756

# Transmission controller
./target/release/j1939-simulator \
  --interface vcan0 \
  --device-id 0x03 \
  --websocket-port 10759

# With debug logging
./target/release/j1939-simulator \
  --interface can0 \
  --device-id 0x8B \
  --debug
```

---

## CAN Protocol

The J1939 simulator implements the SAE J1939 vehicle bus standard.

### Protocol Type

**SAE J1939**: Industry-standard vehicle bus protocol based on CAN 2.0B (29-bit extended identifiers)

### J1939 PGN Structure

**29-bit CAN ID Format**:
```
Priority (3 bits) | Reserved (1 bit) | Data Page (1 bit) | PDU Format (8 bits) | PDU Specific (8 bits) | Source Address (8 bits)
```

**PGN (Parameter Group Number)**: 18-bit value identifying message type
- Reserved + Data Page + PDU Format + PDU Specific

### Incoming Messages (Listens For)

The simulator responds to J1939 commands and queries:

| Message | PGN | Description |
|---------|-----|-------------|
| **DM3** | 65228 (0xFECC) | Diagnostic Message 3: Clear/Reset Previously Active DTCs |

**DM3 Processing**:
- Clears previously active diagnostic trouble codes
- Resets crash notification state
- Acknowledges with appropriate response

### Outgoing Messages (Sends)

| Message | PGN | Rate | Description |
|---------|-----|------|-------------|
| **CN** | 64965 (0xFDC5) | 10 Hz | Crash Notification message |
| **WAND** | 64966 (0xFDC6) | 10 Hz | Wand Angle sensor data |
| **LDISP** | 64967 (0xFDC7) | 10 Hz | Linear Displacement sensor data |
| **DM1** | 65226 (0xFECA) | 1 Hz | Active Diagnostic Trouble Codes |

**Message Details**:

**CN (Crash Notification)**:
- Crash detection status
- Crash type indicator
- Crash event counter
- Checksum for data integrity

**WAND (Wand Angle)**:
- Wand angle measurement (degrees)
- Sensor status flags
- Data quality indicators

**LDISP (Linear Displacement)**:
- Linear position measurement (mm)
- Displacement rate
- Sensor calibration status

**DM1 (Active DTCs)**:
- Standard J1939 diagnostic message
- Reports currently active fault codes
- Lamp status (MIL, Red Stop Lamp, Amber Warning Lamp, Protect Lamp)

---

## Simulation Details

### Crash Notification System

**Crash Detection States**:
- **No Crash**: Normal operation, no crash detected
- **Crash Detected**: Impact event detected, crash notification active
- **Crash Acknowledged**: Event acknowledged, transitioning to recovery

**Crash Types**:
- Type 0: No crash
- Type 1: Front impact
- Type 2: Rear impact
- Type 3: Side impact
- Type 4: Rollover

**Crash Counter**:
- Increments with each crash event
- Persistent across resets (configurable)
- Used for event tracking and logging

**Checksum Calculation**:
- CRC-based checksum for data integrity
- Validates crash notification messages
- Detects communication errors

### Wand Angle Sensor

**Angle Range**: 0-360 degrees

**Simulation Features**:
- Smooth angle transitions
- Configurable angle rate
- Sensor status simulation (valid/invalid)
- Calibration state tracking

**Use Cases**:
- Steering angle simulation
- Boom angle monitoring
- Articulation angle tracking

### Linear Displacement Sensor

**Displacement Range**: -1000 to +1000 mm

**Simulation Features**:
- Position tracking
- Velocity calculation
- Direction indication
- Sensor health monitoring

**Use Cases**:
- Suspension travel
- Hydraulic cylinder position
- Actuator position feedback

### Diagnostic Message Support

**DM1 (Active DTCs)**:
- Reports currently active fault codes
- Lamp status (MIL, warning, stop lamps)
- SPN (Suspect Parameter Number) and FMI (Failure Mode Indicator)

**DM3 (Clear DTCs)**:
- Clears previously active DTCs
- Resets crash notification
- Clears event counters (optional)

---

## Console Commands

Interactive console interface for manual control and testing:

```
J1939> help                     # Show all available commands
J1939> status                   # Display current device state
J1939> crash trigger 1          # Trigger crash (type 1)
J1939> crash clear              # Clear crash notification
J1939> wand 45.5                # Set wand angle (degrees)
J1939> ldisp 250                # Set linear displacement (mm)
J1939> dtc add 1234 5           # Add DTC (SPN 1234, FMI 5)
J1939> dtc clear                # Clear all DTCs
J1939> pause                    # Pause CAN message broadcasting
J1939> resume                   # Resume CAN message broadcasting
J1939> reset                    # Reset to default state
J1939> log                      # Show recent state history
J1939> quit                     # Exit simulator
```

### Crash Commands

```
crash trigger <type>    Trigger crash notification
  Types: 0 (none), 1 (front), 2 (rear), 3 (side), 4 (rollover)

crash clear             Clear crash notification and reset counter

crash status            Display current crash state
```

### Sensor Commands

```
wand <angle>           Set wand angle (0-360 degrees)
  Example: wand 45.5

ldisp <position>       Set linear displacement (-1000 to +1000 mm)
  Example: ldisp 250
```

### Diagnostic Commands

```
dtc add <spn> <fmi>    Add diagnostic trouble code
  SPN: Suspect Parameter Number (0-524287)
  FMI: Failure Mode Indicator (0-31)
  Example: dtc add 1234 5

dtc clear              Clear all active DTCs

dtc list               List all active DTCs
```

### Control Commands

```
pause                  Pause CAN message broadcasting (for test isolation)
resume                 Resume CAN message broadcasting

reset                  Reset device to default state (clears crash, DTCs, sensors)
```

### System Commands

```
log                    Display recent state history (last 10 entries)
status                 Comprehensive status display
quit                   Exit simulator
```

---

## WebSocket API

Real-time control and telemetry via WebSocket on configurable port.

### Connection

**Endpoint**: `ws://localhost:<port>/ws`  
**Default Port**: Configured in cando.toml or via `--websocket-port`

**Example**:
```bash
# Device configured with websocket_port = 10756
wscat -c ws://localhost:10756/ws
```

### Message Format

All messages are JSON with a `type` field:

```json
{
  "type": "MessageType",
  "parameter": "value"
}
```

### Client → Server Messages

**Get Current State**:
```json
{"type": "GetState"}
```

**Trigger Crash**:
```json
{
  "type": "TriggerCrash",
  "crash_type": 1
}
```
Valid crash types: `0` (none), `1` (front), `2` (rear), `3` (side), `4` (rollover)

**Clear Crash**:
```json
{"type": "ClearCrash"}
```

**Set Wand Angle**:
```json
{
  "type": "SetWandAngle",
  "angle": 45.5
}
```

**Set Linear Displacement**:
```json
{
  "type": "SetLinearDisplacement",
  "position": 250
}
```

**Add DTC**:
```json
{
  "type": "AddDtc",
  "spn": 1234,
  "fmi": 5
}
```

**Clear DTCs**:
```json
{"type": "ClearDtcs"}
```

**Pause Broadcasting**:
```json
{"type": "PauseBroadcast"}
```

**Resume Broadcasting**:
```json
{"type": "ResumeBroadcast"}
```

**Reset Device**:
```json
{"type": "Reset"}
```

### Server → Client Messages

**State Update** (automatic broadcast at 10 Hz):
```json
{
  "type": "StateUpdate",
  "state": {
    "device_id": 139,
    "crash_detected": false,
    "crash_type": 0,
    "crash_counter": 0,
    "crash_checksum": 0,
    "wand_angle": 45.5,
    "linear_displacement": 250,
    "active_dtcs": [
      {"spn": 1234, "fmi": 5}
    ],
    "broadcast_paused": false,
    "uptime_seconds": 3600
  }
}
```

**Error Response**:
```json
{
  "type": "Error",
  "message": "Invalid crash type"
}
```

---

## State History Logging

The simulator maintains message tracking and state history.

### Message Tracking

**In-Memory History**: Last 100 received messages tracked for test verification

**Tracked Information**:
- Message timestamp
- PGN (Parameter Group Number)
- Source address
- Data payload
- Processing status (recognized/unrecognized)

### State Snapshots

State changes logged with:
- Crash events and acknowledgments
- DTC additions and clearances
- Sensor value changes
- Broadcast pause/resume events

---

## Testing Integration

### Tier2 Virtual Tests

J1939 simulator integrates with tier2 testing:

```bash
# Start as part of tier2 tests
make tier2

# With visual monitoring
WITH_WEBUI=1 make tier2
```

Configuration loaded from `cando-test.toml` automatically.

### Tier2 Physical Tests

For physical CAN hardware:

```bash
# Physical hardware tests
make tier2-physical

# With visual monitoring
WITH_WEBUI=1 make tier2-physical
```

Uses physical device configurations from `cando-test.toml`.

### Interactive Testing

Test J1939 independently:

```bash
# Start simulator
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 Test ECU"

# In another terminal, monitor CAN
candump vcan0

# In another terminal, decode J1939
j1939-decode vcan0
```

### Integration Test Scripts

J1939 included in automated integration tests:

```bash
# Run specific J1939 test
./scripts/integration/validate_j1939.sh

# Configuration-driven test
CONFIG_FILE=cando-test.toml \
ENVIRONMENT=tier2-virtual \
./scripts/integration/integration_test_all_protocols_config.sh
```

---

## Signal Coverage

The J1939 simulator provides comprehensive J1939 signal coverage:

### Crash Notification (4 signals)
- `crash_detected` - Boolean crash detection status
- `crash_type` - Crash type identifier (0-4)
- `crash_counter` - Event counter (increments per crash)
- `crash_checksum` - Data integrity checksum

### Sensor Data (2 signals)
- `wand_angle` - Wand angle measurement (0-360°)
- `linear_displacement` - Linear position (-1000 to +1000 mm)

### Diagnostic Messages (2+ signals)
- `active_dtcs` - List of active DTCs (SPN + FMI)
- `dtc_count` - Number of active DTCs
- `lamp_status` - MIL, warning, and stop lamp states

### Control Signals (2 signals)
- `broadcast_paused` - Message broadcasting state
- `device_id` - J1939 source address

### Device Information (2 signals)
- `message_count` - Total messages processed
- `uptime_seconds` - System uptime

**Total**: ~15 signals with J1939-compliant behavior

---

## Testing Scenarios

### Basic Crash Notification Test

```bash
# 1. Start simulator
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 Test ECU"

# 2. Console commands
J1939> status                # Initial state (no crash)
J1939> crash trigger 1       # Trigger front impact
J1939> status                # Verify crash detected
J1939> crash clear           # Clear crash
J1939> status                # Verify cleared
```

### Sensor Data Test

```bash
# Console commands
J1939> wand 0                # Start at 0°
J1939> wand 90               # 90° angle
J1939> wand 180              # 180° angle
J1939> wand 270              # 270° angle
J1939> ldisp -500            # -500mm displacement
J1939> ldisp 0               # Center position
J1939> ldisp 500             # +500mm displacement
J1939> status                # Verify sensor values
```

### Diagnostic Code Test

```bash
# Console commands
J1939> dtc add 110 5         # Add coolant temp high (SPN 110, FMI 5)
J1939> dtc add 190 3         # Add engine speed high (SPN 190, FMI 3)
J1939> dtc list              # List active DTCs
J1939> status                # Verify DM1 broadcasts
J1939> dtc clear             # Clear all DTCs
J1939> status                # Verify cleared
```

### DM3 Clear Test

```bash
# Terminal 1: Start simulator
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 Test ECU"

# Terminal 1: Console commands
J1939> crash trigger 1       # Trigger crash
J1939> dtc add 110 5         # Add DTC

# Terminal 2: Send DM3 clear
cansend vcan0 18ECFF8B#00FF00FF00FFFFFF  # DM3 message

# Terminal 1: Verify cleared
J1939> status                # Check crash and DTCs cleared
```

### Broadcast Pause Test (Test Isolation)

```bash
# Console commands
J1939> status                # Normal broadcasting
J1939> pause                 # Pause CAN messages
# Verify no messages on CAN bus
J1939> resume                # Resume broadcasting
J1939> status                # Verify messages resume
```

### Multi-Device Test

```bash
# Terminal 1: Engine controller (0x00)
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 ECU 1"

# Terminal 2: Transmission controller (0x03)
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 ECU 2"

# Terminal 3: Body controller (0x21)
./target/release/j1939-simulator \
  --config cando-test.toml \
  --device-name "J1939 ECU 3"

# Terminal 4: Monitor all devices
candump vcan0 | grep -E "(00|03|21)"
```

---

## Integration Examples

### Python WebSocket Control

```python
import asyncio
import websockets
import json

async def control_j1939():
    uri = "ws://localhost:10756/ws"
    
    async with websockets.connect(uri) as websocket:
        # Get current state
        await websocket.send(json.dumps({"type": "GetState"}))
        response = await websocket.recv()
        state = json.loads(response)
        print(f"Device ID: 0x{state['state']['device_id']:02X}")
        
        # Trigger crash event
        await websocket.send(json.dumps({
            "type": "TriggerCrash",
            "crash_type": 1  # Front impact
        }))
        
        # Set sensor values
        await websocket.send(json.dumps({
            "type": "SetWandAngle",
            "angle": 45.0
        }))
        
        await websocket.send(json.dumps({
            "type": "SetLinearDisplacement",
            "position": 250
        }))
        
        # Add diagnostic code
        await websocket.send(json.dumps({
            "type": "AddDtc",
            "spn": 110,  # Coolant temperature
            "fmi": 5     # Current above normal
        }))
        
        # Monitor state updates
        for _ in range(10):
            update = await websocket.recv()
            data = json.loads(update)
            if data["type"] == "StateUpdate":
                print(f"Crash: {data['state']['crash_detected']}, "
                      f"Wand: {data['state']['wand_angle']:.1f}°, "
                      f"LDISP: {data['state']['linear_displacement']} mm, "
                      f"DTCs: {len(data['state']['active_dtcs'])}")
        
        # Clear crash and DTCs
        await websocket.send(json.dumps({"type": "ClearCrash"}))
        await websocket.send(json.dumps({"type": "ClearDtcs"}))

asyncio.run(control_j1939())
```

### CAN Message Decoding (Python)

```python
import can
import struct

def decode_cn_message(data):
    """Decode CN (Crash Notification) message."""
    crash_detected = bool(data[0] & 0x01)
    crash_type = (data[0] >> 1) & 0x07
    crash_counter = data[1]
    crash_checksum = data[2]
    
    return {
        "crash_detected": crash_detected,
        "crash_type": crash_type,
        "crash_counter": crash_counter,
        "crash_checksum": crash_checksum
    }

def decode_wand_message(data):
    """Decode WAND (Wand Angle) message."""
    angle = struct.unpack('<f', data[0:4])[0]  # Little-endian float
    
    return {
        "wand_angle": angle
    }

def decode_ldisp_message(data):
    """Decode LDISP (Linear Displacement) message."""
    displacement = struct.unpack('<f', data[0:4])[0]  # Little-endian float
    
    return {
        "linear_displacement": displacement
    }

# Monitor CAN bus
bus = can.interface.Bus(interface='socketcan', channel='vcan0')

for msg in bus:
    pgn = (msg.arbitration_id >> 8) & 0x3FFFF
    source_address = msg.arbitration_id & 0xFF
    
    if pgn == 64965:  # CN
        cn = decode_cn_message(msg.data)
        print(f"CN from 0x{source_address:02X}: {cn}")
    elif pgn == 64966:  # WAND
        wand = decode_wand_message(msg.data)
        print(f"WAND from 0x{source_address:02X}: {wand}")
    elif pgn == 64967:  # LDISP
        ldisp = decode_ldisp_message(msg.data)
        print(f"LDISP from 0x{source_address:02X}: {ldisp}")
```

### Message Statistics Collection

```python
import can
from collections import defaultdict
import time

def collect_j1939_statistics(duration_seconds=60):
    """Collect J1939 message statistics."""
    bus = can.interface.Bus(interface='socketcan', channel='vcan0')
    
    stats = defaultdict(lambda: {"count": 0, "last_timestamp": 0})
    start_time = time.time()
    
    print(f"Collecting statistics for {duration_seconds} seconds...")
    
    try:
        while time.time() - start_time < duration_seconds:
            msg = bus.recv(timeout=1.0)
            if msg:
                pgn = (msg.arbitration_id >> 8) & 0x3FFFF
                source = msg.arbitration_id & 0xFF
                key = (pgn, source)
                
                stats[key]["count"] += 1
                stats[key]["last_timestamp"] = msg.timestamp
    
    except KeyboardInterrupt:
        pass
    
    # Print results
    print("\n=== J1939 Message Statistics ===")
    for (pgn, source), data in sorted(stats.items()):
        rate = data["count"] / duration_seconds
        pgn_name = {
            64965: "CN",
            64966: "WAND",
            64967: "LDISP",
            65226: "DM1"
        }.get(pgn, f"PGN_{pgn}")
        
        print(f"PGN {pgn} ({pgn_name}) from 0x{source:02X}: "
              f"{data['count']} messages ({rate:.1f} Hz)")

# Usage
collect_j1939_statistics(60)
```

---

## Troubleshooting

### Common Issues

**"Failed to open CAN interface"**
- Ensure CAN interface exists: `ip link show`
- For virtual CAN: `sudo ip link add dev vcan0 type vcan && sudo ip link set up vcan0`
- Check interface is up: `sudo ip link set up can0`
- Verify permissions: user must have CAN socket access

**"Device not found in configuration"**
- Check device name spelling: `--device-name "J1939 Test ECU"`
- Verify device exists in config: `grep -A5 "J1939 Test ECU" cando.toml`
- Use `cando-cfg list-devices` to see all available devices

**"Invalid device ID"**
- J1939 supports 0x00-0xFF (full range)
- Common addresses: 0x00 (Engine #1), 0x03 (Transmission #1)
- Update configuration or use valid ID: `--device-id 0x8B`

**"WebSocket connection refused"**
- Check port availability: `netstat -ln | grep 10756`
- Try different port: `--websocket-port 8081`
- Verify firewall settings allow localhost connections

**"Messages not appearing on CAN bus"**
- Check if broadcast is paused: `J1939> status`
- Resume if paused: `J1939> resume`
- Verify device ID doesn't conflict with other devices
- Enable debug mode: `--debug`

**"DM3 clear not working"**
- Verify DM3 message format (PGN 65228)
- Check source address matches
- Enable debug to see received messages
- Ensure simulator is listening for DM3

### Debug Mode

Run with `--debug` flag for comprehensive logging:

```bash
./target/release/j1939-simulator \
  --config cando.toml \
  --device-name "J1939 Test ECU" \
  --debug
```

**Debug output includes**:
- Received CAN messages with PGN and source address
- Parsed J1939 messages with decoded fields
- Sent messages (CN, WAND, LDISP, DM1)
- WebSocket connection events
- State transitions and changes
- Message processing status (recognized/unrecognized)
- DTC management events

### Performance Monitoring

Monitor simulator performance:

| Metric | Expected Value | Notes |
|--------|---------------|-------|
| **CAN Message Rate** | 31 Hz | CN (10 Hz) + WAND (10 Hz) + LDISP (10 Hz) + DM1 (1 Hz) |
| **WebSocket Updates** | 10 Hz | State broadcasts |
| **Message Processing** | <1ms | Per message |
| **Memory Usage** | <20 MB | With 100-message history |
| **CPU Usage** | <3% | Efficient async design |

**Check performance**:
```bash
# CPU and memory usage
top -p $(pgrep j1939-simulator)

# CAN message statistics
candump -n 310 vcan0 | grep "8B" | wc -l  # Count messages in 10 seconds (31 Hz)

# WebSocket traffic
netstat -an | grep 10756

# Message rate per PGN
candump vcan0 | grep -E "FDC5|FDC6|FDC7|FECA"  # CN, WAND, LDISP, DM1
```

---

## References

### Documentation
- **Simulator Overview**: `doc/simulators/README.md` - Comprehensive simulator guide
- **Configuration Guide**: `cando.toml` - Device configuration examples
- **Integration Testing**: `doc/testing-webui-visual-flag.md` - Visual monitoring during tests
- **J1939 Standard**: SAE J1939 vehicle bus specification

### Example Configurations
- `cando.toml` - Unified configuration with multiple environments
- Use `--environment` flag to select test or production configurations

### Source Code
- **Simulator**: `j1939-simulator/src/main.rs`
- **Common Library**: `cando-simulator-common/`
- **Messages**: `cando-messages/src/j1939/`

### J1939 Specifications
- **Protocol**: SAE J1939 vehicle bus standard
- **PGNs Implemented**: CN (64965), WAND (64966), LDISP (64967), DM1 (65226), DM3 (65228)
- **Address Range**: 0x00-0xFF (full J1939 address space)

---

## Development

### Building from Source

```bash
# Clone repository
git clone <repository-url>
cd cando-rs

# Build simulator
cargo build --release --bin j1939-simulator

# Verify build
./target/release/j1939-simulator --version
```

### Running Tests

```bash
# Unit tests
cargo test -p j1939-simulator

# Integration tests
make tier2

# With visual monitoring
WITH_WEBUI=1 make tier2
```

### Architecture

The J1939 simulator is modular with clear separation of concerns:

**Core Components**:
- **CAN Handler**: Receives and processes J1939 messages
- **Message Decoder**: Parses J1939 PGN structure and payloads
- **State Manager**: Maintains device state and sensor values
- **Message Broadcaster**: Sends CN, WAND, LDISP, and DM1 messages
- **Diagnostic Manager**: Handles DTC addition, clearing, and DM1 reporting
- **WebSocket Server**: Real-time API for remote control and monitoring
- **Console Interface**: Interactive command-line control
- **Message Tracking**: Records recent messages for test verification

**Key Modules**:
- `handle_can_frame()` - CAN message reception and routing
- `process_incoming_message()` - J1939 message parsing
- `process_dm3()` - DM3 clear/reset command processing
- `broadcast_messages()` - Periodic J1939 message transmission
- `build_cn_message()` - Crash notification message construction
- `build_wand_message()` - Wand angle message construction
- `build_ldisp_message()` - Linear displacement message construction
- `build_dm1_message()` - DM1 diagnostic message construction
- `start_websocket_server()` - WebSocket API initialization
- `start_console_ui()` - Interactive console interface

### J1939 PGN Construction

**29-bit CAN ID Format**:
```rust
fn build_j1939_can_id(priority: u8, pgn: u32, source_address: u8) -> u32 {
    let mut can_id: u32 = 0;
    
    // Priority (3 bits, bits 26-28)
    can_id |= ((priority & 0x07) as u32) << 26;
    
    // PGN (18 bits, bits 8-25)
    can_id |= (pgn & 0x3FFFF) << 8;
    
    // Source Address (8 bits, bits 0-7)
    can_id |= source_address as u32;
    
    can_id | 0x80000000  // Extended frame flag
}
```

### Extending Functionality

To add new features:

1. **New J1939 Messages**: Add PGN support
   ```rust
   const NEW_PGN: u32 = 65280;  // Example PGN
   
   fn build_new_message(&self) -> Vec<u8> {
       // Construct message payload
       vec![0; 8]
   }
   ```

2. **New WebSocket Commands**: Add to message enum
   ```rust
   pub enum J1939WebSocketMessage {
       // Existing variants...
       NewCommand { parameter: String },
   }
   ```

3. **New Console Commands**: Extend console UI command parser
   ```rust
   "newcmd" => {
       // Handle new command
   }
   ```

4. **Additional Sensors**: Extend state struct
   ```rust
   pub struct SimulatorState {
       // Existing fields...
       pub new_sensor_value: f32,
   }
   ```

5. **Enhanced DTC Support**: Add more diagnostic messages
   ```rust
   // DM2 (Previously Active DTCs)
   // DM5 (Diagnostic Readiness 1)
   // DM11 (Diagnostic Data Clear/Reset)
   ```

---

## Contributing

When modifying J1939 simulator:

1. **Maintain J1939 Compliance**: Follow SAE J1939 standard specifications
2. **Test Thoroughly**: Run tier2 tests before committing
3. **Update Documentation**: Keep this file synchronized with code
4. **Configuration-Driven**: Use cando.toml for all device parameters
5. **Zero Warnings**: Maintain clean builds with no compiler warnings
6. **PGN Accuracy**: Verify PGN construction matches J1939 standard

---

**Document Version**: 2.0  
**Last Updated**: 2025-01-20  
**Status**: Current, configuration-driven architecture