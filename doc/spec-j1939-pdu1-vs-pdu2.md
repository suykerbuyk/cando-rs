# J1939 PDU1 vs PDU2 Reference Guide

**Purpose**: Definitive reference for understanding PDU1 and PDU2 message types in J1939  
**Audience**: Developers implementing J1939 protocol support  
**Status**: Living document - update as understanding evolves

---

## Table of Contents

- [Quick Reference](#quick-reference)
- [What is a PDU?](#what-is-a-pdu)
- [J1939 CAN ID Structure](#j1939-can-id-structure)
- [PDU1: Destination-Specific Messages](#pdu1-destination-specific-messages)
- [PDU2: Broadcast Messages](#pdu2-broadcast-messages)
- [Critical Differences](#critical-differences)
- [PGN Calculation Rules](#pgn-calculation-rules)
- [Real-World Examples](#real-world-examples)
- [Common Pitfalls](#common-pitfalls)
- [Implementation Guidelines](#implementation-guidelines)
- [References](#references)

---

## Quick Reference

| Criteria | PDU1 | PDU2 |
|----------|------|------|
| **PF Value** | 0-239 (0x00-0xEF) | 240-255 (0xF0-0xFF) |
| **Detection** | `PF < 240` | `PF >= 240` |
| **Bits 15-8 Meaning** | Destination Address (DA) | Group Extension (GE) |
| **Addressing Type** | Peer-to-peer | Broadcast |
| **PS in PGN?** | NO (PS = DA) | YES (PS = GE) |
| **Typical Use** | Commands, requests | Status, data |
| **Example PGN** | 32000 (0x7D00) | 62320 (0xF370) |

**Rule of Thumb**:
- **Commands TO devices**: Usually PDU1
- **Status FROM devices**: Usually PDU2

---

## What is a PDU?

**PDU = Protocol Data Unit**

In J1939, every CAN message is classified as either **PDU1** or **PDU2** based on the value of the **PF (PDU Format)** byte in the 29-bit extended CAN identifier.

The PDU type determines:
1. How to interpret bits 15-8 of the CAN ID (DA vs GE)
2. How to calculate the Parameter Group Number (PGN)
3. Whether the message is destination-specific or broadcast

---

## J1939 CAN ID Structure

### 29-bit Extended CAN Identifier Breakdown

```
┌────────────────────────────────────────────────────────────┐
│ 28  27  26 │ 25  24  23  22  21  20  19  18 │ 17  16 │ ... │
├────────────┼────────────────────────────────┼────────┼─────┤
│  Priority  │      PDU Format (PF)          │ DP/EDP │     │
│   (P)      │         8 bits                │ 2 bits │     │
│  3 bits    │                               │        │     │
└────────────┴────────────────────────────────┴────────┴─────┘
         ▲                    ▲                   ▲
         │                    │                   │
     Priority            THIS BYTE           Data Page
      (0-7)           DETERMINES PDU        (0-3)
                        TYPE!

┌─────────────────────────────────────────────────────────────┐
│ ... │ 15  14  13  12  11  10   9   8 │  7   6   5   4 ... 0 │
├─────┼────────────────────────────────┼──────────────────────┤
│ ... │     PDU Specific (PS)          │   Source Address     │
│     │         8 bits                 │      (SA)            │
│     │  ┌─────────────────────────┐   │      8 bits          │
│     │  │ PDU1: Destination (DA)  │   │                      │
│     │  │ PDU2: Group Ext (GE)    │   │                      │
│     │  └─────────────────────────┘   │                      │
└─────┴────────────────────────────────┴──────────────────────┘
              ▲                                ▲
              │                                │
         MEANING CHANGES                  Always source
         BASED ON PF!                     address
```

### Field Descriptions

| Field | Bits | Description |
|-------|------|-------------|
| **P** (Priority) | 28-26 (3 bits) | Message priority (0=highest, 7=lowest) |
| **EDP** (Extended Data Page) | 25 (1 bit) | Extended data page selector |
| **DP** (Data Page) | 24 (1 bit) | Data page selector |
| **PF** (PDU Format) | 23-16 (8 bits) | **Determines PDU1 vs PDU2** |
| **PS** (PDU Specific) | 15-8 (8 bits) | DA (PDU1) or GE (PDU2) |
| **SA** (Source Address) | 7-0 (8 bits) | Sending device address |

---

## PDU1: Destination-Specific Messages

### Definition

**PDU1**: Messages where `PF < 240` (0xF0)

These are **peer-to-peer** communications directed to a specific device.

### PS Field Interpretation: Destination Address (DA)

For PDU1 messages, bits 15-8 contain the **Destination Address**:
- WHO you are sending the message TO
- The target device that should act on this message

### Addressing Fields

```
┌─────┬─────┬─────┬─────┐
│  P  │ PF  │ DA  │ SA  │
│ +DP │     │     │     │
└─────┴─────┴─────┴─────┘
       ^     ^     ^
       │     │     └─── Source Address: WHO is sending
       │     └───────── Destination Address: WHO should receive
       └─────────────── PF < 240: This is PDU1
```

### PGN Calculation for PDU1

**Important**: The DA (PS byte) is **NOT** part of the PGN!

```
PGN = (DP << 16) | (PF << 8) | 0x00
      └─────────┘   └──────┘   └──┘
       Data Page    PF byte    Always 0x00!
```

**Why?**: The PS byte contains addressing (DA), not message identification.

### Example: EMP J1939 Motor Command (PGN 32000)

**Scenario**: Controller (0x02) commands fan (0x82) to turn on

```
CAN ID: 0x187D8202

Breakdown:
  0x18    = Priority 6 (110b), DP=0, EDP=0
  0x7D    = PF = 125 (< 240, therefore PDU1)
  0x82    = PS = Destination Address (fan)
  0x02    = SA = Source Address (controller)

PGN Calculation:
  DP = 0, PF = 0x7D, PS = ignored (0x00 for PGN)
  PGN = (0 << 16) | (0x7D << 8) | 0x00
  PGN = 0x007D00 = 32000 decimal

Message Meaning:
  "Motor command TO device 0x82 FROM device 0x02"
```

### Common PDU1 Use Cases

- **Commands**: Motor control, valve control, actuator commands
- **Requests**: Request for specific parameter from specific device
- **Peer-to-peer**: Device-to-device communications
- **Diagnostic**: Diagnostic requests to specific ECU

---

## PDU2: Broadcast Messages

### Definition

**PDU2**: Messages where `PF >= 240` (0xF0)

These are **broadcast** communications sent to all devices on the network.

### PS Field Interpretation: Group Extension (GE)

For PDU2 messages, bits 15-8 contain the **Group Extension**:
- Part of the message identifier (PGN)
- Extends the PGN space for more message types
- NOT an address

### Addressing Fields

```
┌─────┬─────┬─────┬─────┐
│  P  │ PF  │ GE  │ SA  │
│ +DP │     │     │     │
└─────┴─────┴─────┴─────┘
       ^     ^     ^
       │     │     └─── Source Address: WHO is broadcasting
       │     └───────── Group Extension: Part of PGN
       └─────────────── PF >= 240: This is PDU2
```

### PGN Calculation for PDU2

**Important**: The GE (PS byte) **IS** part of the PGN!

```
PGN = (DP << 16) | (PF << 8) | GE
      └─────────┘   └──────┘   └─┘
       Data Page    PF byte    Group Extension
```

**Why?**: The PS byte extends the message identifier, allowing more PGNs.

### Example: EMP J1939 Motor Status (PGN 62320)

**Scenario**: Fan (0x82) broadcasts its status to all devices

```
CAN ID: 0x18F37082

Breakdown:
  0x18    = Priority 6 (110b), DP=0, EDP=0
  0xF3    = PF = 243 (>= 240, therefore PDU2)
  0x70    = PS = Group Extension (part of PGN)
  0x82    = SA = Source Address (fan)

PGN Calculation:
  DP = 0, PF = 0xF3, GE = 0x70
  PGN = (0 << 16) | (0xF3 << 8) | 0x70
  PGN = 0x00F370 = 62320 decimal

Message Meaning:
  "Motor status broadcast FROM device 0x82"
```

### Common PDU2 Use Cases

- **Status Messages**: Device status, sensor readings
- **Periodic Data**: Regular data broadcasts (speed, temperature, etc.)
- **Announcements**: Device presence, configuration data
- **Diagnostics**: Active fault codes, system health

---

## Critical Differences

### Addressing Semantics

| Aspect | PDU1 | PDU2 |
|--------|------|------|
| **PS Byte Name** | DA (Destination Address) | GE (Group Extension) |
| **PS Byte Meaning** | WHO receives message | Part of message type |
| **PS in PGN?** | NO | YES |
| **Addressing** | 1-to-1 (unicast) | 1-to-many (broadcast) |
| **Recipient** | Specific device | All devices |

### PGN Calculation Difference

```
PDU1: PGN = (DP << 16) | (PF << 8) | 0x00    ← PS ignored
PDU2: PGN = (DP << 16) | (PF << 8) | GE      ← PS included
```

### Base ID Masking

When stripping addressing to get "base message ID":

```rust
// PDU1: Strip BOTH DA (bits 15-8) AND SA (bits 7-0)
base_id = can_id & 0xFFFF0000;  // Keep only P+EDP+DP+PF

// PDU2: Strip ONLY SA (bits 7-0), keep GE
base_id = can_id & 0xFFFFFF00;  // Keep P+EDP+DP+PF+GE
```

### Encoding Requirements

```rust
// PDU1: Must set BOTH DA and SA
can_id = (base_id & 0xFFFF0000) | (da << 8) | sa;

// PDU2: Only set SA (GE already in base_id)
can_id = (base_id & 0xFFFFFF00) | sa;
```

---

## PGN Calculation Rules

### Rule 1: Always Check PF First

```rust
fn calculate_pgn(can_id: u32) -> u32 {
    let pf = (can_id >> 16) & 0xFF;
    let dp = (can_id >> 24) & 0x01;
    let edp = (can_id >> 25) & 0x01;
    
    if pf < 240 {
        // PDU1: PS byte is DA, not part of PGN
        (edp << 17) | (dp << 16) | (pf << 8) | 0x00
    } else {
        // PDU2: PS byte is GE, part of PGN
        let ge = (can_id >> 8) & 0xFF;
        (edp << 17) | (dp << 16) | (pf << 8) | ge
    }
}
```

### Rule 2: Same PGN, Different Recipients (PDU1)

PDU1 messages can have the same PGN but different CAN IDs due to DA:

```
PGN 32000 to fan:   0x187D8202  (PGN=0x7D00, DA=0x82, SA=0x02)
PGN 32000 to pump:  0x187D8802  (PGN=0x7D00, DA=0x88, SA=0x02)
                         ^^
                    Different DA, SAME PGN!
```

### Rule 3: Different PGN, Different GE (PDU2)

PDU2 messages with different GE values are DIFFERENT PGNs:

```
PGN 62320: 0x18F37082  (PF=0xF3, GE=0x70)
PGN 62321: 0x18F37182  (PF=0xF3, GE=0x71)
                ^^
           Different GE, DIFFERENT PGNs!
```

---

## Real-World Examples

### Example Set 1: EMP J1939 Messages

#### Command (PDU1): PGN 32000 - Electrified Accessory Motor Command

```
Message: Controller commands fan to turn on
PGN: 32000 (0x7D00)
PF: 0x7D (125) < 240 → PDU1

CAN ID: 0x187D8202
  Priority: 6
  PF: 0x7D (PDU1)
  DA: 0x82 (destination: fan)
  SA: 0x02 (source: controller)

Interpretation: "Command TO fan (0x82) FROM controller (0x02)"
```

#### Status (PDU2): PGN 62320 - Electrified Accessory Motor Status 2

```
Message: Fan broadcasts its status
PGN: 62320 (0xF370)
PF: 0xF3 (243) >= 240 → PDU2

CAN ID: 0x18F37082
  Priority: 6
  PF: 0xF3 (PDU2)
  GE: 0x70 (group extension, part of PGN)
  SA: 0x82 (source: fan)

Interpretation: "Status broadcast FROM fan (0x82)"
```

### Example Set 2: Standard J1939 Messages

#### Command (PDU1): PGN 0 - Request Message

```
Message: Request data from specific device
PGN: 0 (0x0000)
PF: 0x00 (0) < 240 → PDU1

CAN ID: 0x18EA8B02
  Priority: 6
  PF: 0xEA (234) < 240 → PDU1
  DA: 0x8B (destination: specific ECU)
  SA: 0x02 (source: requester)

Note: Request PGN uses PF=0xEA (234), still PDU1!
```

#### Status (PDU2): PGN 61444 - Electronic Engine Controller 1

```
Message: Engine broadcasts RPM, torque, etc.
PGN: 61444 (0xF004)
PF: 0xF0 (240) >= 240 → PDU2

CAN ID: 0x0CF00400
  Priority: 3
  PF: 0xF0 (240) >= 240 → PDU2
  GE: 0x04 (group extension)
  SA: 0x00 (source: engine)
```

---

## Common Pitfalls

### Pitfall 1: Assuming All Commands are PDU1

**Wrong Assumption**: "If it's a command, it must be PDU1"

**Reality**: Some broadcast commands use PDU2 (e.g., TSC1 - Torque/Speed Control)

**Rule**: Always check PF value, not message purpose.

### Pitfall 2: Including DA in PDU1 PGN

**Wrong**:
```rust
// DON'T DO THIS for PDU1!
pgn = (pf << 8) | da;  // ❌ DA is NOT part of PGN
```

**Correct**:
```rust
if pf < 240 {
    pgn = (pf << 8);  // ✓ PS byte is not part of PGN
}
```

### Pitfall 3: Using device_id as SA for PDU1 Commands

**Wrong**:
```rust
// Sending command TO device 0x82
let can_id = base_id | 0x82;  // ❌ Puts device in SA field!
// Result: 0x187D0082 (DA=0x00, SA=0x82) WRONG!
```

**Correct**:
```rust
// Sending command TO device 0x82 FROM controller 0x02
let can_id = (base_id & 0xFFFF0000) | (0x82 << 8) | 0x02;
// Result: 0x187D8202 (DA=0x82, SA=0x02) CORRECT!
```

### Pitfall 4: Stripping Too Many Bits

**Wrong**:
```rust
// Using same mask for both types
base_id = can_id & 0xFFFF0000;  // ❌ Strips GE from PDU2!
```

**Correct**:
```rust
if is_pdu1(can_id) {
    base_id = can_id & 0xFFFF0000;  // Strip DA and SA
} else {
    base_id = can_id & 0xFFFFFF00;  // Strip SA, keep GE
}
```

---

## Implementation Guidelines

### 1. Always Detect PDU Type First

```rust
fn is_pdu1(can_id: u32) -> bool {
    let pf = (can_id >> 16) & 0xFF;
    pf < 240
}
```

### 2. Encode Based on PDU Type

```rust
fn encode_j1939_id(
    base_id: u32,
    device_id: u8,
    source_addr: Option<u8>
) -> u32 {
    if is_pdu1(base_id) {
        // PDU1: device_id=DA, source_addr=SA
        let da = (device_id as u32) << 8;
        let sa = source_addr.unwrap_or(0x0F) as u32;
        (base_id & 0xFFFF0000) | da | sa
    } else {
        // PDU2: device_id=SA
        (base_id & 0xFFFFFF00) | (device_id as u32)
    }
}
```

### 3. Extract Addresses Correctly

```rust
fn extract_addresses(can_id: u32) -> (Option<u8>, u8) {
    let sa = (can_id & 0xFF) as u8;
    
    if is_pdu1(can_id) {
        let da = ((can_id >> 8) & 0xFF) as u8;
        (Some(da), sa)  // (destination, source)
    } else {
        (None, sa)      // (no destination, source)
    }
}
```

### 4. Calculate PGN Correctly

```rust
fn calculate_pgn(can_id: u32) -> u32 {
    let pf = (can_id >> 16) & 0xFF;
    let dp_edp = (can_id >> 24) & 0x03;
    
    if pf < 240 {
        // PDU1: PS=DA, not part of PGN
        (dp_edp << 16) | (pf << 8)
    } else {
        // PDU2: PS=GE, part of PGN
        let ge = (can_id >> 8) & 0xFF;
        (dp_edp << 16) | (pf << 8) | ge
    }
}
```

---

## Real-World Case Study: Message Decoding Bug

### Background (2025-01-20)

During the first physical lab test with real EMP hardware, a critical bug was discovered that affected **all PDU1 message decoding** in monitor-can and can-log-analyzer tools. This case study documents the discovery, root cause, and solution.

### Lab Setup

**Hardware**:
- EMP Fan (device 0x82) in J1939 mode
- EMP Pump (device 0x88) in J1939 mode
- External controller (source 0x02) sending commands
- CAN interface: can2 at 500kbps (non-standard bitrate)

**Messages in Use**:
- Command: `EMP_J1939_CMD_32000_ElectrifiedAccessoryMotor` (PGN 32000, PDU1)
- Status: `EMP_J1939_ST2_62320` (PGN 62320, PDU2)

### The Problem

**What Failed**:
```
CAN ID 0x187D8202 (PGN 32000 to fan)  → ❌ "Unknown Message"
CAN ID 0x187D8802 (PGN 32000 to pump) → ❌ "Unknown Message"
CAN ID 0x18F37082 (PGN 62320 from fan)  → ✅ Decoded correctly
CAN ID 0x18F37088 (PGN 62320 from pump) → ✅ Decoded correctly
```

**Pattern Observed**: PDU1 messages (commands) failed to decode, while PDU2 messages (status) worked perfectly.

### Root Cause Analysis

**The Bug**:
```rust
// OLD CODE (BROKEN for PDU1):
let base_id = lookup_id & CAN_BASE_ID_MASK;  // 0xFFFFFF00
// For PDU1 0x187D8202: base = 0x187D8200 (destination 0x82 preserved!)
// Database contains:     0x187D0000 (no addresses)
// Result: NO MATCH → "Unknown Message"
```

**Why PDU2 Worked**:
```rust
// For PDU2 0x18F37082: base = 0x18F37000 (source stripped, GE preserved)
// Database contains:     0x18F37000 (GE 0x70 is part of PGN)
// Result: MATCH → Decodes correctly
```

**The Problem**: A single mask (0xFFFFFF00) was used for all messages. This works for PDU2 because:
- Bits 15-8: Group Extension (part of PGN, must be preserved)
- Bits 7-0: Source Address (variable, must be stripped)

But for PDU1:
- Bits 15-8: Destination Address (variable, must be stripped)
- Bits 7-0: Source Address (variable, must be stripped)

### The Solution

**Implementation**:

1. **PDU Detection Function** (`cando-messages/src/common.rs`):
```rust
pub fn is_j1939_pdu1(can_id: u32) -> bool {
    let pf = (can_id >> 16) & 0xFF;
    pf < 240  // PDU1 if PF < 240
}

pub const CAN_PDU1_BASE_MASK: u32 = 0xFFFF0000;
```

2. **PDU-Aware Base ID Function**:
```rust
pub fn get_j1939_base_id(can_id: u32) -> u32 {
    let masked_id = can_id & CAN_EFF_MASK;
    if is_j1939_pdu1(masked_id) {
        masked_id & CAN_PDU1_BASE_MASK  // 0xFFFF0000 - strip both addresses
    } else {
        masked_id & CAN_BASE_ID_MASK    // 0xFFFFFF00 - strip source only
    }
}
```

3. **Fixed Message Lookup** (monitor-can, can-log-analyzer):
```rust
// NEW CODE (CORRECT):
let base_id = get_j1939_base_id(lookup_id);
// For PDU1 0x187D8202: base = 0x187D0000 ✓ MATCHES database!
// For PDU2 0x18F37082: base = 0x18F37000 ✓ MATCHES database!
```

### Verification

**Test Suite Created**: `cando-messages/tests/pdu1_pdu2_decoding.rs`
- 10 comprehensive tests covering PDU1/PDU2 detection and masking
- All tests derive values from message metadata (zero hardcoded values)
- Integrated into standard `cargo test` workflow

**Files Modified**:
- `cando-messages/src/common.rs` - Core PDU detection helpers
- `monitor-can/src/main.rs` - Fixed message lookup
- `can-log-analyzer/src/main.rs` - Fixed message decoding
- Test suite with 100% pass rate

### Key Lessons

1. **PDU Type Matters for Masking**: The same mask cannot work for both PDU1 and PDU2 messages due to different byte semantics.

2. **Physical Testing is Essential**: This bug was not caught in virtual testing because most messages were PDU2. Only real hardware with PDU1 commands revealed the issue.

3. **Configuration-Driven Testing**: Tests must use message metadata, never hardcoded CAN IDs, to remain valid as protocols evolve.

4. **Standards vs. Reality**: Lab setup used 500kbps (not standard 250kbps) and custom EMP J1939 messages (not standard TSC1). Always verify physical reality.

### Impact

**Before Fix**:
- ❌ All PDU1 messages failed to decode
- ❌ Motor commands appeared as "Unknown Message"
- ❌ Lab testing and development blocked

**After Fix**:
- ✅ Both PDU1 and PDU2 messages decode correctly
- ✅ Full lab testing capabilities restored
- ✅ Comprehensive test coverage prevents regression
- ✅ Zero build warnings, 100% test pass rate

This case study demonstrates the critical importance of understanding PDU1/PDU2 differences in real-world implementations.

---

## Real-World Case Study 2: PDU1 Encoding Bug (2025-01-21)

### Background

After the PDU1 decoding bug was fixed, further testing revealed a critical **encoding bug** where PDU1 commands were being sent with incorrect source/destination addresses.

### The Bug

The `embed_device_id()` function only handled PDU2 messages correctly:
- PDU2 (broadcast): device_id → SA (correct ✅)
- PDU1 (destination-specific): device_id → SA (WRONG ❌)

For PDU1 messages, **both** DA and SA must be set, but the code only set SA.

**Result**: Commands sent as `0x187D0082` (to 0x00 from 0x82) instead of `0x187D8202` (to 0x82 from 0x02).

### The Fix (Commit 029a8d3)

**Solution**: Added optional `source_addr` parameter to `embed_device_id()` with protocol-aware logic for PDU1 vs PDU2.

**Key Design Decision**: Source addresses come from configuration (`cando.toml` → `[test.sources]`), respecting the configuration-driven architecture.

### Verification

- 56/56 cando-messages tests passing
- 102/102 doctests passing  
- 48/48 tier1 integration tests passing
- Physical testing validated with real EMP hardware

### Key Lessons

1. **PDU1 requires both DA and SA** - Missing either makes messages invalid
2. **Configuration-driven is critical** - Source addresses must be configurable
3. **Protocol-aware encoding** - Cannot treat all J1939 messages identically
4. **Comprehensive testing** - Both encoding AND decoding must be tested

---


## Testing Checklist

### PDU1 Encoding Tests

- [ ] Command to device 0x82 from source 0x02
- [ ] Command to device 0x88 from source 0x02
- [ ] Command with default source (0x0F)
- [ ] Verify DA field (bits 15-8) is set correctly
- [ ] Verify SA field (bits 7-0) is set correctly
- [ ] PGN calculation ignores DA byte

### PDU2 Encoding Tests

- [ ] Status from device 0x82
- [ ] Status from device 0x88
- [ ] Verify GE field (bits 15-8) preserved from base
- [ ] Verify SA field (bits 7-0) is device ID
- [ ] PGN calculation includes GE byte
- [ ] Source parameter ignored for PDU2

### Edge Cases

- [ ] PF = 239 (0xEF) - Last PDU1
- [ ] PF = 240 (0xF0) - First PDU2
- [ ] All zeros CAN ID
- [ ] All ones CAN ID
- [ ] Multiple source addresses

---

## Quick Debugging Guide

### Problem: Device doesn't respond to command

**Check**:
1. Is it PDU1? (`PF < 240`)
2. Is DA field set to target device? (bits 15-8)
3. Is SA field set to sender? (bits 7-0)
4. Compare with working Python/reference implementation

### Problem: Wrong PGN calculated

**Check**:
1. Are you including PS byte for PDU2?
2. Are you excluding PS byte for PDU1?
3. Are DP/EDP bits included?

### Problem: Message not decoded

**Check**:
1. Are you stripping correct bits for base ID?
2. PDU1: Strip 16 bits (0xFFFF0000)
3. PDU2: Strip 8 bits (0xFFFFFF00)

---

## References

### J1939 Standards

- **SAE J1939-21**: Data Link Layer (CAN ID structure, PDU types)
- **SAE J1939-71**: Vehicle Application Layer (PGN definitions)
- **SAE J1939-73**: Application Layer - Diagnostics
- **SAE J1939-81**: Network Management

### Online Resources

- J1939 Digital Annex: Official PGN database
- CSS Electronics CAN Database: J1939 message examples
- Kvaser J1939 Tutorial: Excellent explanations

### Internal Documents

- `cando-messages/src/common.rs` - PDU detection and masking functions (is_j1939_pdu1, get_j1939_base_id)
- `cando-messages/src/encoder.rs` - Encoding functions (embed_device_id, extract_device_id)
- `cando-messages/tests/pdu1_pdu2_decoding.rs` - Comprehensive PDU test suite (10 tests)
- `monitor-can/src/main.rs` - Uses get_j1939_base_id() for message lookup
- `can-log-analyzer/src/main.rs` - Uses get_j1939_base_id() for message decoding

---

## Document History

| Date | Version | Changes |
|------|---------|---------|
| 2025-01-21 | 1.0 | Initial creation - comprehensive PDU1/PDU2 reference |
| 2025-01-20 | 1.1 | Added real-world case study documenting PDU1 decoding bug |
| 2025-01-21 | 1.2 | Added PDU1 encoding bug case study (commit 029a8d3) |
| 2025-11-22 | 1.3 | Consolidated with investigation and implementation plan docs |

---

**Remember**: 
- **PF < 240** = PDU1 (destination-specific, PS=DA)
- **PF >= 240** = PDU2 (broadcast, PS=GE)
- **Always check PF first!**