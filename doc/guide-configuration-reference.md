# Cando Configuration Reference

**Purpose**: Comprehensive reference for Cando-RS configuration system  
**Status**: Authoritative - Single source of truth for configuration  
**Last Updated**: 2025-01-06 (Session 45)  
**Format**: YAML (cando.yaml)

---

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Configuration Structure](#configuration-structure)
- [Inheritance System](#inheritance-system)
- [Validation Rules](#validation-rules)
- [Environment-Based Port Reuse](#environment-based-port-reuse)
- [Common Workflows](#common-workflows)
- [Using cando-cfg CLI](#using-cando-cfg-cli)
- [Troubleshooting](#troubleshooting)
- [Best Practices](#best-practices)
- [Migration Notes](#migration-notes)

---

## Overview

### What is the Configuration System?

The Cando configuration system provides a **single YAML file** (`cando.yaml`) that defines:

- **Environments**: Logical groupings of devices for different scenarios (lab, testing, development)
- **Devices**: CAN bus devices (physical hardware or simulators) with their properties
- **Inheritance**: Three-level hierarchy (defaults → environment → device)
- **Validation**: Automatic enforcement of hardware/simulator rules

### Key Features

✅ **Single File**: One `cando.yaml` for all environments and devices  
✅ **HashMap-Based**: Devices nested within environments as key-value maps  
✅ **Inheritance**: Smart defaults with environment and device overrides  
✅ **Validation**: Automatic rule enforcement at load time  
✅ **Flexible**: Supports physical hardware, simulators, and mixed environments  
✅ **Environment-Scoped**: Only one environment active at runtime

### Design Philosophy

1. **One Environment at a Time**: Select environment with `--environment <name>` at runtime
2. **Device Containment**: Devices belong to environments (no global device pool)
3. **Port Reuse Across Environments**: Same logical device can use same port in different environments
4. **Hardware Safety**: Validation prevents physical device misconfiguration

---

## Quick Start

### Starting the WebUI

```bash
# Start WebUI with specific environment
./target/debug/cando-webui \
  --cando-config cando.yaml \
  --environment webui-simple

# Open browser
open http://localhost:10752
```

### Running Tests

```bash
# Run with test environment
cargo test -p cando-webui -- --nocapture

# Integration tests
./scripts/integration/integration_test_all_protocols.sh
```

### Querying Configuration

```bash
# List all environments
./target/debug/cando-cfg --config cando.yaml list-environments

# List devices in environment
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple list-devices

# Get device details
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple device-info test_fan
```

---

## Architecture

### HashMap-Based Structure

**Key Design Decision**: Devices are stored as HashMaps within environments, not as a global array.

```yaml
environments:
  webui-simple:                    # Environment key
    friendly_name: "WebUI Simple"
    devices:
      test_fan:                    # Device key (unique within environment)
        friendly_name: "EMP Test Fan"
        device_id: "0x82"
        websocket_port: 10756
      test_pump:                   # Another device key
        friendly_name: "EMP Test Pump"
        device_id: "0x88"
        websocket_port: 10757
```

**Benefits**:
- Clear containment: devices belong to environments
- Fast lookup: O(1) access by key
- No ambiguity: device keys unique per environment
- Consistent with Rust HashMap idioms

### Physical vs Simulated Devices

**Device Classification** (via `hardware_present` flag):

| Type | `hardware_present` | `websocket_port` | `interface` | Use Case |
|------|-------------------|------------------|-------------|----------|
| **Physical Hardware** | `true` | MUST be `None` | Physical (can0, can1) | Real lab equipment |
| **Simulator** | `false` or omitted | MUST be defined | Any (vcan0, can0) | Software simulation |

**Critical Rule**: Physical hardware CANNOT have WebSocket ports (they don't expose control interfaces).

### Environment Isolation

**Runtime Behavior**:
- User selects ONE environment: `--environment <name>`
- Only devices from that environment are loaded
- No cross-environment conflicts possible
- Ports and device IDs can be reused across environments

---

## Configuration Structure

### Complete Schema

```yaml
# Version (required)
version: "1.0.0"

# Global Defaults (optional)
defaults:
  can_interface: vcan0          # Default CAN interface
  protocol: j1939               # Default protocol
  websocket_enabled: true       # Default WebSocket state
  debug: false                  # Default debug mode
  no_console: false             # Default console mode

# WebUI Configuration (required)
webui:
  http_port: 10752              # HTTP and WebSocket port (same port)
  auto_detect_devices: true     # Device auto-detection
  display_config: dark          # UI theme (dark/light)

# Environments (required, must have at least one)
environments:
  environment-key:              # Unique environment key (kebab-case)
    friendly_name: "Display Name"
    description: "Purpose"      # Optional
    location: "Physical Location"  # Optional
    can_interface: vcan0        # Override default
    enabled: true               # Environment enabled?
    tags: [tag1, tag2]          # Optional tags
    owner: "Team Name"          # Optional
    notes: "Additional info"    # Optional
    
    # Devices (HashMap)
    devices:
      device-key:               # Unique device key (snake_case)
        friendly_name: "Device Name"
        type: emp               # Device type (emp, hvpc, udc, j1939)
        device_id: "0x82"       # CAN device ID (hex string)
        variant: fan            # Device variant (fan, pump, etc)
        interface: vcan0        # CAN interface (optional, inherits)
        protocol: j1939         # Protocol (optional, inherits)
        websocket_port: 10756   # WebSocket port (required for simulators)
        enabled: true           # Device enabled?
        hardware_present: false # Physical (true) or simulator (false)
        voltage_specification: "600VDC"  # Voltage rating
        description: "Purpose"  # Optional
        tags: [tag1, tag2]      # Optional
        owner: "Team"           # Optional
        notes: "Notes"          # Optional
        serial_number: "SN001"  # Optional (hardware only)
        firmware_version: "1.0" # Optional

# Test Configuration (optional)
test:
  can_interface: vcan0
  base_websocket_port: 10800

# Network Configuration (optional)
network:
  bind_address: "0.0.0.0"
  max_connections: 100
  timeout_ms: 5000

# Logging Configuration (optional)
logging:
  level: info                   # Logging level
  format: text                  # Output format
```

### Required Fields

**Top Level**:
- `version`: Configuration version
- `webui`: WebUI settings
- `environments`: At least one environment

**Environment Level**:
- Environment key (map key)
- `friendly_name`: Display name
- `devices`: HashMap of devices (can be empty)
- `enabled`: Boolean flag

**Device Level**:
- Device key (map key within environment)
- `type`: Device type (emp, hvpc, udc, j1939)
- `device_id`: CAN device ID
- `variant`: Device variant
- `hardware_present`: Boolean (physical or simulator)
- `voltage_specification`: Voltage rating
- `websocket_port`: Required if hardware_present=false

### Optional with Inheritance

These fields inherit from defaults → environment → device:
- `interface`: CAN interface
- `protocol`: Communication protocol
- `enabled`: Enabled state
- `debug`: Debug mode
- `no_console`: Console mode

---

## Inheritance System

### Three-Level Hierarchy

**Precedence** (highest to lowest):
1. **Device Level** (highest priority)
2. **Environment Level** (medium priority)
3. **Defaults Level** (lowest priority)

### Inheritance Example

```yaml
defaults:
  can_interface: vcan0          # Level 1: Global default
  protocol: j1939

environments:
  physical-lab:
    can_interface: can2         # Level 2: Environment override
    
    devices:
      lab_fan:
        device_id: "0x82"       # Inherits: interface=can2, protocol=j1939
      
      special_fan:
        device_id: "0x84"
        interface: can1         # Level 3: Device override (highest)
        protocol: emp           # Level 3: Device override
```

**Resolution**:
- `lab_fan.interface` = `can2` (from environment)
- `lab_fan.protocol` = `j1939` (from defaults)
- `special_fan.interface` = `can1` (device override)
- `special_fan.protocol` = `emp` (device override)

### API Resolution

Use `resolve_device()` to get fully-resolved device config:

```rust
let resolved = config.resolve_device("physical-lab", "lab_fan")?;
assert_eq!(resolved.interface, Some("can2".to_string()));
```

---

## Validation Rules

### Rule 1: Hardware vs Simulator Rules

#### Rule 1.1: Physical Hardware Cannot Have WebSocket Ports

**Rule**: If `hardware_present: true`, then `websocket_port` MUST be `None`.

**Rationale**: Physical hardware doesn't expose WebSocket control interfaces.

```yaml
# ✅ VALID
lab_fan:
  hardware_present: true
  device_id: "0x82"
  # No websocket_port

# ❌ INVALID
lab_fan:
  hardware_present: true
  websocket_port: 10754  # ERROR!
```

#### Rule 1.2: Simulators MUST Have WebSocket Ports

**Rule**: If `hardware_present: false` or omitted, then `websocket_port` MUST be defined.

```yaml
# ✅ VALID
test_fan:
  hardware_present: false
  websocket_port: 10756

# ❌ INVALID
test_fan:
  hardware_present: false
  # Missing websocket_port - ERROR!
```

### Rule 2: CAN Interface Rules

#### Rule 2.1: Physical Hardware MUST Use Physical Interfaces

**Rule**: If `hardware_present: true`, interface must be physical (can0, can1, can2), NOT virtual (vcan0, vcan1).

```yaml
# ✅ VALID
lab_fan:
  hardware_present: true
  interface: can2

# ❌ INVALID
lab_fan:
  hardware_present: true
  interface: vcan0  # ERROR!
```

#### Rule 2.2: Simulators Can Use Any Interface

**Rule**: Simulators can use either physical or virtual interfaces.

```yaml
# ✅ VALID - Virtual interface
test_fan:
  hardware_present: false
  interface: vcan0

# ✅ VALID - Physical interface (HIL testing)
test_fan:
  hardware_present: false
  interface: can0
```

### Rule 3: Uniqueness Rules (Per-Environment)

#### Rule 3.1: WebSocket Ports Must Be Unique Within Environment

**Rule**: Each `websocket_port` MUST be unique within a single environment.

**Note**: Same port CAN be reused across different environments (see next section).

```yaml
environments:
  webui-simple:
    devices:
      test_fan:
        websocket_port: 10756
      test_pump:
        websocket_port: 10757  # ✅ Different port

      test_hvpc:
        websocket_port: 10756  # ❌ ERROR: Duplicate in same environment
```

#### Rule 3.2: Device IDs Must Be Unique Per Interface Within Environment

**Rule**: Within an environment, `device_id` must be unique per CAN interface.

```yaml
environments:
  webui-simple:
    devices:
      test_fan:
        device_id: "0x82"
        interface: vcan0
      
      test_pump:
        device_id: "0x88"      # ✅ Different ID
        interface: vcan0
      
      test_hvpc:
        device_id: "0x82"      # ✅ OK - Different interface
        interface: vcan1
      
      test_udc:
        device_id: "0x82"      # ❌ ERROR: Duplicate ID on vcan0
        interface: vcan0
```

### Rule 4: Naming Conventions

#### Rule 4.1: Device Keys Use snake_case

**Convention**: Device keys (within environment.devices) use `snake_case`.

**Valid**: `test_fan`, `lab_pump`, `integration_fan_2`  
**Invalid**: `testFan`, `test-fan`, `TestFan`

#### Rule 4.2: Environment Keys Use kebab-case

**Convention**: Environment keys use `kebab-case` for CLI friendliness.

**Valid**: `physical-lab`, `webui-simple`, `integration-test-emp`  
**Acceptable**: `physical_lab` (but kebab-case preferred)

---

## Environment-Based Port Reuse

### Critical Design Principle

**Since only ONE environment is active at runtime**, ports and device IDs MAY be reused across different environments.

### Port Reuse Policy

✅ **REQUIRED**: Ports MUST be unique WITHIN each environment  
✅ **ALLOWED**: Ports MAY be reused ACROSS different environments

### Benefits

1. **Predictability**: Same logical device always uses same port
2. **Developer Experience**: Easy to remember "test_fan is always 10758"
3. **Configuration Simplicity**: No port number sprawl
4. **Debugging Consistency**: Same connection commands across environments

### Example: Valid Port Reuse

```yaml
environments:
  integration-test-emp:
    devices:
      integration_fan:
        websocket_port: 10758  # ✅ OK

  integration-test-multi:
    devices:
      integration_fan:
        websocket_port: 10758  # ✅ OK - Different environment

  integration-test-simulated:
    devices:
      integration_fan:
        websocket_port: 10758  # ✅ OK - Different environment
```

All three `integration_fan` devices use port 10758. This is **correct and intentional** because only ONE environment runs at a time.

### Runtime Behavior

```bash
# Terminal 1: Run webui-simple environment
./target/debug/cando-webui --environment webui-simple
# Binds ports: 10756 (test_fan), 10757 (test_pump)

# Terminal 2: Cannot run integration-test-emp simultaneously
./target/debug/cando-webui --environment integration-test-emp
# Would bind: 10758, 10759, 10760
# No conflict with webui-simple because it's a different run
```

### Validation Behavior

- **Per-Environment Validation**: Checks uniqueness within each environment separately
- **NOT Global Validation**: Does not check across environments
- **Load-Time**: Validates only the selected environment

---

## Common Workflows

### Workflow 1: Start WebUI with Environment

```bash
# 1. List available environments
./target/debug/cando-cfg --config cando.yaml list-environments

# 2. Start WebUI with specific environment
./target/debug/cando-webui \
  --cando-config cando.yaml \
  --environment webui-simple \
  --http-port 10752

# 3. Open browser
open http://localhost:10752
```

### Workflow 2: Add New Environment

```yaml
environments:
  my-new-env:
    friendly_name: "My New Environment"
    description: "Testing new feature X"
    can_interface: vcan0
    enabled: true
    tags: [development, feature-x]
    
    devices:
      my_device:
        friendly_name: "My Test Device"
        type: emp
        device_id: "0x90"
        variant: fan
        websocket_port: 10780
        enabled: true
        hardware_present: false
        voltage_specification: "600VDC"
```

### Workflow 3: Add Device to Existing Environment

```yaml
environments:
  webui-simple:
    devices:
      # Existing devices...
      
      # New device
      new_pump:
        friendly_name: "New Test Pump"
        type: emp
        device_id: "0x89"
        variant: pump
        websocket_port: 10758  # Must be unique in this environment
        enabled: true
        hardware_present: false
        voltage_specification: "600VDC"
```

### Workflow 4: Create Mixed Environment (Hardware + Simulators)

```yaml
environments:
  mixed-test:
    friendly_name: "Mixed Hardware and Simulators"
    can_interface: can0  # Physical interface
    enabled: true
    
    devices:
      # Physical hardware
      real_fan:
        device_id: "0x82"
        type: emp
        variant: fan
        hardware_present: true  # No websocket_port
        serial_number: "FAN-001"
      
      # Simulator
      test_pump:
        device_id: "0x88"
        type: emp
        variant: pump
        hardware_present: false
        websocket_port: 10760  # Required for simulator
```

### Workflow 5: Switch Between Environments

```bash
# Development with simulators
./target/debug/cando-webui --environment webui-simple

# Testing with hardware
./target/debug/cando-webui --environment physical-lab

# Integration testing
./target/debug/cando-webui --environment integration-test-multi
```

---

## Using cando-cfg CLI

### Purpose

`cando-cfg` is a command-line tool for querying and validating configuration without starting services.

### Common Commands

#### List Environments

```bash
# All environments
./target/debug/cando-cfg --config cando.yaml list-environments

# Enabled only
./target/debug/cando-cfg --config cando.yaml list-environments --enabled-only

# With details
./target/debug/cando-cfg --config cando.yaml list-environments --verbose
```

#### List Devices

```bash
# All devices in environment
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple list-devices

# Enabled devices only
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple list-devices --enabled-only

# By device type
./target/debug/cando-cfg --config cando.yaml \
  --environment integration-test-multi list-devices --type emp
```

#### Get Device Details

**IMPORTANT**: All device queries require environment context to ensure deterministic results.

```bash
# Query device using explicit --environment flag
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple get-device-id test_fan
# Output: 0x82

# Query device using colon-separated format
./target/debug/cando-cfg --config cando.yaml \
  get-device-id webui-simple:test_fan
# Output: 0x82

# Get WebSocket port
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple get-port test_fan
# Output: 10756

# Get CAN interface
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple get-interface test_fan
# Output: vcan0

# Get device type
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple get-type test_fan
# Output: emp

# Get device variant
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple get-variant test_fan
# Output: fan

# Full device information
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple show-device test_fan

# JSON output
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple --format json show-device test_fan
```

**Why Environment Context is Required:**

Device keys are scoped to environments, not global. The same device key (e.g., `test_fan`) can exist in multiple environments with different configurations. Without environment context, queries would be non-deterministic.

```bash
# ERROR: No environment context
$ cando-cfg --config cando.yaml get-device-id test_fan
Error: Device reference 'test_fan' requires environment context.
       Use --environment flag or 'environment:device' format

# CORRECT: Two ways to provide environment context
$ cando-cfg --config cando.yaml --environment webui-simple get-device-id test_fan
0x82

$ cando-cfg --config cando.yaml get-device-id webui-simple:test_fan
0x82
```

#### Validate Configuration

```bash
# Validate entire file
./target/debug/cando-cfg --config cando.yaml validate

# Validate specific environment
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple validate
```

### Script Integration

```bash
#!/bin/bash
# Example: Get device configuration for connection

# Set environment context
CANDO_CONFIG="cando.yaml"
ENVIRONMENT="webui-simple"

# Get device WebSocket port (Method 1: explicit --environment)
DEVICE_PORT=$(./target/debug/cando-cfg \
  --config "$CANDO_CONFIG" \
  --environment "$ENVIRONMENT" \
  get-port test_fan)

# Get device ID (Method 2: colon-separated format)
DEVICE_ID=$(./target/debug/cando-cfg \
  --config "$CANDO_CONFIG" \
  get-device-id "${ENVIRONMENT}:test_fan")

echo "Device ID: $DEVICE_ID"
echo "WebSocket Port: $DEVICE_PORT"
echo "Connecting to device..."
wscat -c "ws://localhost:$DEVICE_PORT"
```

**Pipeline-Friendly Workflow:**

```bash
#!/bin/bash
# List devices in environment and query each one

CANDO_CONFIG="cando.yaml"
ENVIRONMENT="webui-simple"

# list-devices outputs plain device names when --environment is specified
./target/debug/cando-cfg \
  --config "$CANDO_CONFIG" \
  --environment "$ENVIRONMENT" \
  list-devices | while read device; do
    DEVICE_ID=$(./target/debug/cando-cfg \
      --config "$CANDO_CONFIG" \
      --environment "$ENVIRONMENT" \
      get-device-id "$device")
    PORT=$(./target/debug/cando-cfg \
      --config "$CANDO_CONFIG" \
      --environment "$ENVIRONMENT" \
      get-port "$device" 2>/dev/null || echo "(none)")
    echo "$device: ID=$DEVICE_ID, Port=$PORT"
done
```

---

## Troubleshooting

### Issue: Configuration Not Found

**Error**: `Failed to load cando.yaml: No such file or directory`

**Solution**:
```bash
# Check current directory
ls -la cando.yaml

# Use absolute path
./target/debug/cando-webui --cando-config /full/path/to/cando.yaml

# Or run from workspace root
cd /path/to/cando-rs
./target/debug/cando-webui --cando-config cando.yaml
```

### Issue: Environment Not Found

**Error**: `Environment 'test-env' not found`

**Solution**:
```bash
# List available environments
./target/debug/cando-cfg --config cando.yaml list-environments

# Check spelling and use exact key
./target/debug/cando-webui --environment webui-simple  # Note: kebab-case
```

### Issue: Device Not Found in Environment

**Error**: `Device 'test_fan' not found in environment 'webui-simple'`

**Solution**:
```bash
# List devices in environment
./target/debug/cando-cfg --config cando.yaml \
  --environment webui-simple list-devices

# Check device key (snake_case)
# Device keys are case-sensitive!
```

### Issue: Duplicate WebSocket Port

**Error**: `Duplicate WebSocket port 10758 within environment`

**Solution**:
```yaml
# Find duplicates in environment
environments:
  my-env:
    devices:
      device1:
        websocket_port: 10758
      device2:
        websocket_port: 10758  # ❌ Conflict!

# Fix: Assign unique port
      device2:
        websocket_port: 10759  # ✅ Unique
```

### Issue: Physical Hardware with WebSocket Port

**Error**: `Physical hardware device cannot have websocket_port`

**Solution**:
```yaml
# ❌ Invalid
lab_fan:
  hardware_present: true
  websocket_port: 10754  # Remove this!

# ✅ Valid
lab_fan:
  hardware_present: true
  # No websocket_port
```

### Issue: Simulator Missing WebSocket Port

**Error**: `Simulator device missing websocket_port`

**Solution**:
```yaml
# ❌ Invalid
test_fan:
  hardware_present: false
  # Missing websocket_port

# ✅ Valid
test_fan:
  hardware_present: false
  websocket_port: 10756  # Add this!
```

---

## Best Practices

### Configuration Management

1. **Version Control**: Always commit cando.yaml to git
2. **Validation**: Run `cando-cfg validate` before committing
3. **Documentation**: Use `description` and `notes` fields liberally
4. **Tags**: Use tags for filtering and organization

### Environment Design

1. **Purpose-Specific**: Create environments for specific use cases
2. **Minimal**: Keep environments focused (don't mix unrelated devices)
3. **Naming**: Use descriptive kebab-case names
4. **Enable/Disable**: Use `enabled` flag instead of deleting environments

### Device Configuration

1. **Descriptive Names**: Use clear, meaningful device keys and friendly names
2. **Consistent Ports**: Reuse same ports across environments for same logical devices
3. **Hardware Safety**: Always set `hardware_present: true` for real equipment
4. **Serial Numbers**: Record serial numbers for physical hardware

### Testing Practices

1. **Isolated Environments**: Create separate environments for different test scenarios
2. **Consistent Device IDs**: Use same device IDs across test environments
3. **Port Ranges**: Use separate port ranges for different environment types
4. **Validate First**: Always validate configuration before running tests

---

## Migration Notes

### From cando.toml to cando.yaml

**Migration completed in Sessions 42-43 (January 2025)**

#### Key Changes

1. **Format**: TOML → YAML
2. **Structure**: Array-based → HashMap-based
3. **Device Location**: Global `[[devices]]` array → Nested in environments
4. **Field Names**: Some fields renamed for clarity

#### Field Mapping

| Old (TOML) | New (YAML) |
|------------|------------|
| `[[environments]]` | `environments:` (map) |
| `name` (environment) | Map key |
| `[[devices]]` | `devices:` (nested in environment) |
| `name` (device) | Map key |
| `type` | `type` (same) |
| `device_id` | `device_id` (same) |

#### Migration Example

```toml
# OLD: cando.toml
[[environments]]
name = "webui-simple"
devices = ["Test Fan"]

[[devices]]
name = "Test Fan"
type = "emp"
device_id = "0x82"
```

```yaml
# NEW: cando.yaml
environments:
  webui-simple:
    devices:
      test_fan:
        type: emp
        device_id: "0x82"
```

### Validation Changes (Session 45)

**Environment-based validation implemented**

- **Old**: Global uniqueness checks across all environments
- **New**: Per-environment uniqueness checks
- **Impact**: Port and device ID reuse now allowed across environments

---

## Related Documentation

- `doc/network/PORT-REGISTRY.md` - Network port allocations
- `doc/AI-WORKFLOW-GUIDE.md` - Development workflow
- `RESUME.md` - Project status and recent changes
- `README.md` - Project overview

---

## Summary

### Key Concepts

1. **Single YAML File**: cando.yaml contains all configuration
2. **HashMap-Based**: Devices nested in environments as key-value maps
3. **One Environment at Runtime**: Select with `--environment` flag
4. **Three-Level Inheritance**: defaults → environment → device
5. **Automatic Validation**: Rules enforced at load time
6. **Port Reuse Allowed**: Across different environments

### Quick Reference

```bash
# Validate configuration
cando-cfg --config cando.yaml validate

# List environments
cando-cfg --config cando.yaml list-environments

# List devices
cando-cfg --config cando.yaml --environment <name> list-devices

# Start WebUI
cando-webui --cando-config cando.yaml --environment <name>

# Start simulator
emp-simulator --config cando.yaml --device-name <key>
```

### Success Criteria

✅ Configuration validates without errors  
✅ All environments have at least one device  
✅ WebSocket ports unique within each environment  
✅ Physical devices don't have websocket_port  
✅ Simulators have websocket_port defined  
✅ Device IDs unique per interface per environment

---

**End of Configuration Reference**