# Multi-Device CAN Bus Environment - HVPC and UDC Systems

**Date**: 2026-01-29  
**CAN Interface**: can0  
**Status**: ✅ HVPC 100% Decoded, UDC Messages Identified  
**Environment**: Production hardware with multiple J1939 devices

---

## Executive Summary

The CAN0 bus contains a **multi-device J1939 environment** with two primary systems:

1. **HVPC (High Voltage Power Converter)** - SA=0xFF (255)
2. **UDC (Universal DC Converter)** - SA=0x59 (89)

Both systems successfully coexist on the same CAN bus, each transmitting their own telemetry and responding to commands. Cando-RS currently decodes **100% of HVPC messages** and has identified all UDC messages (which are fully documented in UDC.dbc).

**Key Finding**: The "unknown" messages (PGN 0x1FF1C and 0x1FF1D) from SA=0x59 are **UDC_Status_1_Report** and **UDC_Status_2_Report** - fully defined in the existing UDC.dbc file.

---

## CAN Bus Device Map

### Device Overview

| Source Address | Device | Type | Protocol | DBC File | Status |
|----------------|--------|------|----------|----------|--------|
| 0xFF (255) | HVPC | Power Converter | HVPC-J1939 | HVPC-J1939-Merged.dbc | ✅ Fully decoded |
| 0x59 (89) | UDC | DC Converter | UDC-J1939 | UDC.dbc | ✅ Identified, not decoded |
| 0x80 (128) | VPMC | Controller | J1939 | N/A | Commands only |

### Message Flow Diagram

```
┌─────────────────┐         ┌─────────────────┐         ┌─────────────────┐
│   VPMC/WebUI    │         │   HVPC System   │         │   UDC System    │
│    SA = 0x80    │         │    SA = 0xFF    │         │    SA = 0x59    │
└────────┬────────┘         └────────┬────────┘         └────────┬────────┘
         │                           │                           │
         │  HVPC_Command (0x0EF00)   │                           │
         ├──────────────────────────>│                           │
         │                           │                           │
         │  ACK (0x0E800)            │                           │
         │<──────────────────────────┤                           │
         │                           │                           │
         │       HVPC_Group_Report (0x1FF49)                     │
         │<──────────────────────────┤                           │
         │                           │                           │
         │       HVPC_Device_Report (0x1FF48)                    │
         │<──────────────────────────┤                           │
         │                           │                           │
         │                           │   UDC_Status_1_Report (0x1FF1C)
         │<──────────────────────────┼───────────────────────────┤
         │                           │                           │
         │                           │   UDC_Status_2_Report (0x1FF1D)
         │<──────────────────────────┼───────────────────────────┤
         │                           │                           │
                                CAN0 Bus (250 kbps)
```

---

## HVPC System (SA=0xFF)

### Message Summary

| PGN | CAN ID | Message Name | DLC | Frequency | Status |
|-----|--------|--------------|-----|-----------|--------|
| 0x1FF49 | 0x19FF49FF | HVPC_Group_Report | 8 | 12/cycle | ✅ Decoded |
| 0x1FF48 | 0x19FF48FF | HVPC_Device_Report | 8 | 1/cycle | ✅ Decoded |
| 0x1FF4F | 0x1DFF4FFF | HVPC_Reserved | 4-7 | 14/cycle | ✅ Decoded |
| 0x1FF4A | 0x19FF4AFF | HVPC_Hash_Report | 5 | On-request | ⚠️ Not observed |
| 0x0EF00 | 0x18EF00FF | HVPC_Command | 8 | On-demand | ✅ Transmit |
| 0x0E800 | 0x18E8FFFF | J1939 ACK | 8 | After cmd | ✅ Decoded |

**Cycle Time**: 500ms (2 Hz)  
**Total Messages**: ~27 messages per cycle  
**Coverage**: 100% of observed traffic

### Key Telemetry

- **HV Voltage**: 599V
- **Total Power**: -6.5MW (charging)
- **12 Groups**: Individual V/I/P per group
- **Temperatures**: Inlet/outlet 26°C
- **Valve Position**: Real-time feedback

---

## UDC System (SA=0x59)

### Message Summary

| PGN | CAN ID | Message Name | DLC | Frequency | Status |
|-----|--------|--------------|-----|-----------|--------|
| 0x1FF1C | 0x19FF1C59 | UDC_Status_1_Report | 7 | 1/cycle | 📋 Defined in DBC |
| 0x1FF1D | 0x19FF1D59 | UDC_Status_2_Report | 8 | 1/cycle | 📋 Defined in DBC |
| 0x0EB00 | 0x1CEBFF59 | J1939 TP.CM | 8 | As needed | ✅ Standard |
| 0x0EC00 | 0x1CECFF59 | J1939 TP.DT | 8 | As needed | ✅ Standard |

**Cycle Time**: ~500ms (synchronized with HVPC)  
**Coverage**: 100% identified (in UDC.dbc)  
**Decoder Status**: Not currently configured for UDC protocol

### UDC_Status_1_Report (PGN 0x1FF1C)

**Live Sample**: `DC 01 3E 00 35 7C 00`

**Signals Defined**:
- `conversionDir` (2 bits): Actual conversion direction (0=None, 2=Downconvert)
- `state` (2 bits): Device state (0=Convert, 2=Safe, 3=Shutdown)
- `hvil` (1 bit): High-voltage interlock status
- `NED` (1 bit): Nuclear Event Detector active
- `lvVoltage` (9 bits): Low-voltage side (0-51.1V)
- `lvCurrent` (10 bits): Low-voltage current (0-1023A)
- `hvVoltage` (10 bits): High-voltage side (0-1023V)
- `hvCurrent` (9 bits): High-voltage current (0-51.1A)

**DBC Reference**: `dbc/UDC.dbc` line 147-167

### UDC_Status_2_Report (PGN 0x1FF1D)

**Live Sample**: `FF FF FF 0B 00 00 06 00`

**Signals Defined**:
- `thermalDelta` (5 bits): Temperature margin (-11 to +20°C)
- `thermalDevice` (3 bits): Hottest component (0=Processor, 1=MOSFET, 2=Input EMI, 3=HV Switch)
- `estOutputPower` (16 bits): Estimated max output power (0-32767.5W)
- `estInputPower` (16 bits): Estimated max input power (0-32767.5W)

**DBC Reference**: `dbc/UDC.dbc` line 169-183

**Pattern Analysis**:
- Bytes 0-2: `FF FF FF` = Reserved/not available
- Byte 3: `0B` (11 decimal) = Likely related to thermal or status
- Byte 6: Varies `00-06` = Counter or status indicator

---

## J1939 Standard Messages

### Transport Protocol (TP) Messages

Both HVPC and UDC use J1939 Transport Protocol for multi-packet data transfers (e.g., DM1 diagnostics).

**TP.CM (Connection Management)** - PGN 0x0EB00:
- 0x1CEBFFFF - From HVPC (SA=0xFF)
- 0x1CEBFF59 - From UDC (SA=0x59)
- Purpose: Negotiate multi-packet transfer

**TP.DT (Data Transfer)** - PGN 0x0EC00:
- 0x1CECFFFF - From HVPC (SA=0xFF)
- 0x1CECFF59 - From UDC (SA=0x59)
- Purpose: Deliver payload packets

### Acknowledgment Messages

**J1939 ACK/NACK** - PGN 0x0E800:
- 0x18E8FFFF - From HVPC (SA=0xFF)
- Confirms receipt of commands
- Both positive (ACK) and negative (NACK) responses

---

## Message Coverage Analysis

### By System

| System | Messages Defined | Messages Observed | Decoded | Coverage |
|--------|------------------|-------------------|---------|----------|
| HVPC | 8 | 5 | 5 | 100% |
| UDC | 15 | 2 | 0 | 0% (identified) |
| J1939 | N/A | 5 | 5 | 100% |

### By Protocol

| Protocol | PGN Range | Systems | Status |
|----------|-----------|---------|--------|
| HVPC-J1939 | 0x1FF48-0x1FF4F | HVPC only | ✅ Fully decoded |
| UDC-J1939 | 0x1FF1C-0x1FF1F | UDC only | 📋 Identified, not decoded |
| J1939 Standard | 0x0E800, 0x0EB00, 0x0EC00, etc. | Both systems | ✅ Recognized |

### Total CAN Bus Coverage

- **Total unique CAN IDs observed**: 11
- **HVPC messages decoded**: 5/5 (100%)
- **UDC messages identified**: 2/2 (100%)
- **Unknown messages**: 0
- **Overall understanding**: 100%

---

## Current Cando-RS Status

### WebUI Configuration

**Active Environment**: `integration-test-hvpc`
**Active Protocol**: HVPC-J1939-Merged
**DBC Loaded**: `dbc/HVPC-J1939-Merged.dbc`

### What's Working

✅ **HVPC Telemetry Reception**:
- All HVPC reports successfully decoded
- Real-time updates to WebUI
- Message count: 100+ messages in first 10 seconds

✅ **HVPC Command Transmission**:
- Valve control (manual/auto, 0-100%)
- Channel group control (open/close)
- HVIL control (activate/deactivate)
- Hardware ACK confirmation

✅ **Multi-Device Recognition**:
- Correctly identifies messages from SA=0xFF (HVPC)
- Logs UDC messages as "UNKNOWN" (expected - not in loaded DBC)
- J1939 standard messages recognized

### What's Not Decoded (But Identified)

📋 **UDC Messages**:
- UDC_Status_1_Report (0x19FF1C59) - Logged as "UNKNOWN_0x19FF1C59"
- UDC_Status_2_Report (0x19FF1D59) - Logged as "UNKNOWN_0x19FF1D59"
- **Resolution**: Messages are fully defined in `dbc/UDC.dbc` (version 0x00e)
- **Reason**: Cando-RS not configured to decode UDC protocol on CAN0

---

## Live Traffic Sample (10 seconds)

### Message Frequency Distribution

```
HVPC Messages (SA=0xFF):
  - HVPC_Group_Report (0x19FF49FF):    ~240 msgs  (12/cycle × 20 cycles)
  - HVPC_Device_Report (0x19FF48FF):   ~20 msgs   (1/cycle)
  - HVPC_Reserved (0x1DFF4FFF):        ~280 msgs  (14/cycle)
  - J1939 ACK (0x18E8FFFF):            ~5 msgs    (after commands)
  - TP.CM/TP.DT (0x1CEB/ECFFFF):      ~10 msgs   (diagnostics)
Total HVPC: ~555 messages/10s

UDC Messages (SA=0x59):
  - UDC_Status_1_Report (0x19FF1C59):  ~20 msgs   (1/cycle)
  - UDC_Status_2_Report (0x19FF1D59):  ~20 msgs   (1/cycle)
  - J1939 ACK (0x18E8FFFF):            ~2 msgs    (responses)
  - TP.CM/TP.DT (0x1CEB/ECFF59):      ~5 msgs    (diagnostics)
Total UDC: ~47 messages/10s

Grand Total: ~602 messages/10s (~60 msg/s average)
```

### Bus Utilization

**CAN Bus Speed**: 250 kbps  
**Average Message Size**: ~10 bytes (including overhead)  
**Theoretical Max**: ~3125 msg/s  
**Actual Usage**: ~60 msg/s  
**Bus Utilization**: ~2% (very low - excellent margin)

---

## Recommendations

### Priority 1: Enable UDC Decoding (Optional)

If UDC telemetry monitoring is desired:

1. **Add UDC Protocol to Config**:
   ```yaml
   integration-test-hvpc:
     can_interface: can0
     devices:
       hvpc_device_1:
         type: hvpc
         device_id: "0xFF"
         interface: can0
         protocol: hvpc_j1939_merged
       
       udc_device_1:
         type: udc
         device_id: "0x59"
         interface: can0
         protocol: udc
         websocket_port: 10762
   ```

2. **Regenerate UDC Protocol Code**:
   - UDC.dbc already exists (version 0x00e)
   - Run code generator for UDC protocol
   - Integrate UDC device state into state manager

3. **Add UDC WebUI Support**:
   - Create UDC device card in WebUI
   - Display UDC telemetry (voltages, currents, power, thermal)
   - Optional: Add UDC command interface

### Priority 2: Document Multi-Device Environment

✅ **COMPLETE** (this document)

1. Multi-device CAN bus topology
2. HVPC message coverage (100%)
3. UDC message identification (100%)
4. J1939 standard message handling

### Priority 3: Testing & Validation

**If UDC decoding is implemented**:

1. Validate UDC message decoding with live traffic
2. Create UDC test script (similar to `test_hvpc_commands.sh`)
3. Test UDC command transmission (if control is needed)
4. Verify multi-device WebUI display (HVPC + UDC simultaneously)

---

## DBC File Reference

### HVPC DBC
- **File**: `dbc/HVPC-J1939-Merged.dbc`
- **Version**: v1.0 (2026-01-27)
- **Messages**: 9 (6 HVPC + 2 J1939 + 1 VEPA)
- **Status**: ✅ Complete and validated
- **Coverage**: 100% of HVPC protocol

### UDC DBC
- **File**: `dbc/UDC.dbc`
- **Version**: 0x00e (2025-01-21)
- **Messages**: 15 (8 UDC + 7 J1939/VEPA)
- **Status**: ✅ Complete and corrected
- **Coverage**: 100% of UDC protocol
- **Hardware Validated**: 770 frames analyzed

**Key Corrections in UDC.dbc v0x00e**:
1. Source address generalization (SA=0xFF generic)
2. UDC_Command opcode field correction
3. DLC corrections (Command=6, Status_1=7)
4. Hardware-validated signal definitions

---

## Technical Notes

### J1939 Addressing

Both systems correctly implement J1939 addressing:
- **HVPC**: Uses SA=0xFF (broadcast address for reports)
- **UDC**: Uses SA=0x59 (unique device address)
- **VPMC**: Uses SA=0x80 (controller address for commands)

### PGN Allocation

PGN ranges are properly separated:
- **0x0xxxx**: J1939 standard (TP, DM1, Request, ACK)
- **0x1EFxx**: VEPA proprietary messages
- **0x1FFxx**: Device-specific reports
  - 0x1FF1C-0x1FF1F: UDC range
  - 0x1FF48-0x1FF4F: HVPC range
  - 0x1FF80: VEPA Time Report

### Message Priority

J1939 priority field analysis:
- **Priority 6**: Normal telemetry (most HVPC/UDC reports)
- **Priority 7**: Bulk data (TP, Reserved messages)

---

## Known Issues & Limitations

### Current Limitations

1. **UDC Not Decoded**: UDC messages logged as "UNKNOWN"
   - **Impact**: Low (HVPC functionality unaffected)
   - **Resolution**: Add UDC protocol support (optional)

2. **Request-Only Messages Not Tested**:
   - HVPC_Hash_Report (PGN 0x1FF4A)
   - VEPA_Time_Report (PGN 0x1FF80)
   - **Impact**: None (baseline telemetry works)
   - **Resolution**: Test with PGN 0x0EA00 request messages

3. **Multi-Device WebUI**: Currently HVPC-only
   - **Impact**: Cannot view UDC telemetry in WebUI
   - **Resolution**: Add UDC device support (if needed)

### No Issues Found

✅ Message collisions: None observed  
✅ Timing conflicts: Both systems coexist properly  
✅ ACK/NACK handling: Working correctly  
✅ Bus congestion: Only 2% utilization  
✅ Protocol conflicts: Proper PGN separation  

---

## Conclusion

The CAN0 bus operates a **well-structured multi-device J1939 environment** with:

1. ✅ **HVPC System (SA=0xFF)**: 100% decoded, fully operational
2. ✅ **UDC System (SA=0x59)**: 100% identified, DBC available
3. ✅ **J1939 Protocol**: Standard messages properly handled
4. ✅ **No Unknown Messages**: All traffic identified

**Current Status**: Cando-RS WebUI is fully operational for HVPC monitoring and control. UDC messages are present and fully documented but not currently decoded (by design - HVPC-focused environment).

**Production Readiness**: ✅ Ready for HVPC production use. UDC support can be added if monitoring/control of the UDC system is required.

---

**Document Status**: ✅ Complete  
**Last Updated**: 2026-01-29  
**Related Documents**:
- `doc/HVPC-MESSAGE-COVERAGE-LIVE-HARDWARE.md` - HVPC message analysis
- `doc/SESSION-85-HVPC-LIVE-INTEGRATION.md` - Session summary
- `dbc/HVPC-J1939-Merged.dbc` - HVPC protocol definition
- `dbc/UDC.dbc` - UDC protocol definition