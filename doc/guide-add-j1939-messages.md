# How to Add J1939 Messages to Cando-RS

**Version**: 1.0  
**Last Updated**: 2025-01-15  
**Audience**: Developers adding new J1939 messages to cando-rs

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Understanding the Architecture](#understanding-the-architecture)
3. [The Code Generator Workflow](#the-code-generator-workflow)
4. [Pattern 1: Simple Messages](#pattern-1-simple-messages-no-business-logic)
5. [Pattern 2: Messages with Helper Methods](#pattern-2-messages-with-helper-methods)
6. [Pattern 3: Complex Business Logic](#pattern-3-complex-business-logic)
7. [Testing Strategies](#testing-strategies)
8. [Integration Checklist](#integration-checklist)
9. [Common Pitfalls](#common-pitfalls)
10. [Examples](#examples)

---

## Overview

Cando-rs uses a **code generator** to automatically create Rust structs and encode/decode implementations from DBC (Database CAN) files. This eliminates hand-written encoder boilerplate and ensures consistency across protocols.

### Key Benefits

- **Type Safety**: Generated structs are strongly typed
- **Correctness**: Bit manipulation is generated correctly
- **Maintainability**: Single source of truth (DBC file)
- **Consistency**: All protocols use the same patterns

### Three Implementation Patterns

| Pattern | Use Case | Effort | Example |
|---------|----------|--------|---------|
| **Pattern 1** | Simple data messages | 30 min | DM05 (Diagnostic Readiness) |
| **Pattern 2** | Messages needing helpers | 2-4 hours | DM06 (Pending DTCs) |
| **Pattern 3** | Complex/regulatory messages | 1-2 days | DM20 (Monitor Performance) |

---

## Understanding the Architecture

### Code Organization

```
cando-rs/
├── dbc/                          # DBC source files
│   ├── j1939.dbc                # Main J1939 protocol (100 messages)
│   ├── j1939-73-DTCs-split.dbc  # Diagnostic messages (DM01-DM31)
│   ├── EMP.dbc, HVPC.dbc, etc.  # Protocol-specific DBCs
│   └── .checksums.json          # Generation tracking
├── cando-codegen/
│   └── src/generator.rs         # Code generator implementation
├── cando-messages/
│   ├── src/
│   │   ├── generated/           # Generated code (DO NOT EDIT BY HAND)
│   │   │   ├── j1939.rs        # Generated from j1939.dbc
│   │   │   ├── j1939_73.rs     # Generated from j1939-73-DTCs-split.dbc
│   │   │   └── ...
│   │   ├── j1939/
│   │   │   ├── diagnostics/    # Business logic for diagnostics
│   │   │   │   ├── mod.rs      # DiagnosticMessageType enum
│   │   │   │   ├── dm03_helpers.rs  # DM03 business logic
│   │   │   │   └── ...
│   │   │   └── mod.rs
│   │   └── lib.rs              # Public API exports
│   └── tests/                   # Test suites
└── Makefile                     # Includes codegen targets
```

### Data Flow

```
DBC File → Code Generator → Generated Rust Code → Business Logic → Public API
   ↓           ↓                    ↓                  ↓              ↓
j1939.dbc   generator.rs      j1939.rs          helpers.rs      lib.rs
```

---

## The Code Generator Workflow

### Step 1: Understand the DBC File

DBC files use a text format to define CAN messages. Here's an example:

```dbc
BO_ 2566835710 DM04: 13 Vector__XXX
 SG_ FreezeFrameLength : 0|8@1+ (1,0) [0|255] "Byte"
 SG_ DM04_01SPN : 8|16@1+ (1,0) [0|0] ""
 SG_ DM04_01FMI : 24|5@1+ (1,0) [0|0] ""
 SG_ EngSpeed : 56|16@1+ (0.125,0) [0|8031.875] "rpm"
```

**Key elements**:
- `BO_` - Message definition
- `2566835710` - CAN ID (decimal)
- `DM04` - Message name
- `13` - Data length (bytes)
- `SG_` - Signal definition
- `0|8@1+` - Start bit | Length | Byte order | Signed/Unsigned
- `(0.125,0)` - Factor and offset for scaling

### Step 2: Run the Code Generator

```bash
# Generate code for all protocols
make codegen

# Or generate for specific protocol
cd cando-codegen
cargo run -- \
  --dbc ../dbc/j1939-73-DTCs-split.dbc \
  --output ../cando-messages/src/generated/j1939_73.rs
```

### Step 3: Verify Generated Code

The generator creates:

```rust
pub struct DM04 {
    pub device_id: DeviceId,
    pub freezeframelength: u64,
    pub dm04_01spn: u64,
    pub dm04_01fmi: u64,
    pub dm04_01spn_high: f64,
    pub dm04_01oc: u64,
    pub dm04_01cm: u64,
    pub engtorquemode: u64,
    pub engintakemanifold1press: f64,
    pub engspeed: f64,
    pub engpercentloadatcurrentspeed: u64,
    pub enginecoolanttemperature: f64,
    pub wheelbasedvehiclespeed: f64,
}

impl DM04 {
    pub const BASE_CAN_ID: u32 = 419351808;
    pub const DLC: u8 = 13;
    
    pub fn decode(can_id: u32, data: &[u8]) -> Result<Self, DecodeError> {
        // Generated decode implementation
    }
    
    pub fn encode(&self) -> Result<(u32, [u8; 13]), DecodeError> {
        // Generated encode implementation
    }
}

pub static DM04_METADATA: MessageMetadata = MessageMetadata {
    name: "DM04",
    can_id: 0x18FECD00,
    dlc: 13,
    // ... more metadata
};
```

**What you get automatically**:
- ✅ Struct definition with typed fields
- ✅ Complete `decode()` implementation
- ✅ Complete `encode()` implementation
- ✅ Metadata for introspection
- ✅ Doc comments for all fields
- ✅ Type-safe `DeviceId` handling

---

## Pattern 1: Simple Messages (No Business Logic)

### When to Use

Use this pattern for messages that:
- Are pure data (no complex business rules)
- Don't need validation logic
- Don't need convenience methods
- Are self-explanatory from field names

**Examples**: DM04 (Freeze Frame), DM05 (Diagnostic Readiness), DM08 (Test Results)

### Step-by-Step Guide

#### 1. Verify Generated Code Exists

```bash
# Search for the message struct
grep "pub struct DM05" cando-messages/src/generated/j1939_73.rs
```

If found, the message is already generated! Skip to step 3.

#### 2. (If needed) Add to DBC File

If the message doesn't exist, add it to the appropriate DBC file:

```dbc
BO_ 2566835966 DM05: 8 Vector__XXX
 SG_ ActiveTroubleCodeCount : 0|8@1+ (1,0) [0|240] ""
 SG_ PreviouslyActiveTroubleCodeCount : 8|8@1+ (1,0) [0|240] ""
 SG_ OBDCompliance : 16|8@1+ (1,0) [0|240] ""
 ...
```

Then regenerate:
```bash
make codegen
```

#### 3. Export from Public API

Edit `cando-messages/src/lib.rs`:

```rust
// Add to J1939-73 exports section
pub use generated::j1939_73::{
    DM01, DM02, DM03,
    DM05,  // <-- Add this line
    // ... other messages
};
```

#### 4. Write Roundtrip Tests

Create or update `cando-messages/tests/j1939_roundtrip.rs`:

```rust
#[test]
fn test_dm05_roundtrip_basic() {
    use cando_messages::{DM05, DeviceId, DecodeError};
    
    // Create a test message
    let original = DM05 {
        device_id: DeviceId::DIAGNOSTIC_TOOL_1,
        activetroublecodecount: 5,
        previouslyactivetroublecodecount: 10,
        obdcompliance: 0x15, // WWH-OBD
        // ... set other fields
    };
    
    // Encode
    let (can_id, data) = original.encode()
        .expect("Encode should succeed");
    
    // Verify CAN ID
    assert_eq!(can_id & 0xFFFFFF00, DM05::BASE_CAN_ID);
    assert_eq!(data.len(), DM05::DLC as usize);
    
    // Decode
    let decoded = DM05::decode(can_id, &data)
        .expect("Decode should succeed");
    
    // Verify all fields match
    assert_eq!(decoded.device_id, original.device_id);
    assert_eq!(decoded.activetroublecodecount, original.activetroublecodecount);
    assert_eq!(decoded.previouslyactivetroublecodecount, original.previouslyactivetroublecodecount);
    // ... verify other fields
}
```

#### 5. Run Tests

```bash
cargo test -p cando-messages dm05
```

#### 6. (Optional) Add to can-log-analyzer

Edit `can-log-analyzer/src/main.rs`:

```rust
fn decode_diagnostic_message(&self, ...) -> Result<Vec<Signal>> {
    match diagnostic_type {
        DiagnosticMessageType::DM01 => { /* ... */ }
        DiagnosticMessageType::DM02 => { /* ... */ }
        DiagnosticMessageType::DM03 => { /* ... */ }
        DiagnosticMessageType::DM05 => {
            let dm05 = DM05::decode(log_entry.can_id, &log_entry.data)
                .map_err(|e| anyhow::anyhow!("DM05 decode error: {:?}", e))?;
            
            vec![
                ("ActiveTroubleCodeCount", dm05.activetroublecodecount.to_string()),
                ("PreviouslyActiveTroubleCodeCount", dm05.previouslyactivetroublecodecount.to_string()),
                ("OBDCompliance", format!("0x{:02X}", dm05.obdcompliance)),
                // ... other fields
            ]
        }
        // ... other messages
    }
}
```

### Effort: 30 minutes per message

---

## Pattern 2: Messages with Helper Methods

### When to Use

Use this pattern for messages that:
- Have repetitive field patterns (e.g., 19 DTC slots)
- Need convenience accessors
- Benefit from validation logic
- Have common usage patterns

**Examples**: DM06 (Pending DTCs), DM12 (Emissions DTCs), DM23 (Previously Active DTCs)

### Step-by-Step Guide

#### 1. Follow Pattern 1 Steps 1-5

Ensure basic functionality works first.

#### 2. Create Helper Module

Create `cando-messages/src/j1939/diagnostics/dm06_helpers.rs`:

```rust
//! DM06 Helper Methods - Pending Diagnostic Trouble Codes
//!
//! Business logic for working with DM06 messages.

use crate::generated::j1939_73::DM06;

/// Diagnostic Trouble Code representation
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticTroubleCode {
    pub spn: u32,       // Suspect Parameter Number
    pub fmi: u8,        // Failure Mode Identifier
    pub oc: u8,         // Occurrence Count
    pub cm: bool,       // Conversion Method
}

/// Helper methods for DM06 (Pending DTCs)
pub trait DM06Helpers {
    /// Extract all active DTCs from the message
    fn get_pending_dtcs(&self) -> Vec<DiagnosticTroubleCode>;
    
    /// Check if any DTCs are pending
    fn has_pending_dtcs(&self) -> bool;
    
    /// Get lamp status summary
    fn lamp_status_summary(&self) -> String;
}

impl DM06Helpers for DM06 {
    fn get_pending_dtcs(&self) -> Vec<DiagnosticTroubleCode> {
        let mut dtcs = Vec::new();
        
        // Check each DTC slot (19 slots total)
        if self.dm06_01spn != 0xFFFF || self.dm06_01fmi != 0xFF {
            dtcs.push(DiagnosticTroubleCode {
                spn: self.dm06_01spn as u32 + (self.dm06_01spn_high as u32),
                fmi: self.dm06_01fmi as u8,
                oc: self.dm06_01oc as u8,
                cm: self.dm06_01cm != 0,
            });
        }
        
        // Repeat for dm06_02, dm06_03, ... dm06_19
        // (Pattern repeats for all 19 slots)
        
        dtcs
    }
    
    fn has_pending_dtcs(&self) -> bool {
        self.dm06_01spn != 0xFFFF || self.dm06_01fmi != 0xFF
        // OR any other slot has data
    }
    
    fn lamp_status_summary(&self) -> String {
        format!(
            "MIL:{} RSL:{} AWL:{} PL:{}",
            lamp_status_str(self.malfunctionindicatorlampstatus),
            lamp_status_str(self.redstoplampstatus),
            lamp_status_str(self.amberwarninglampstatus),
            lamp_status_str(self.protectlampstatus),
        )
    }
}

fn lamp_status_str(status: u64) -> &'static str {
    match status {
        0 => "Off",
        1 => "On",
        2 => "Reserved",
        3 => "N/A",
        _ => "Invalid",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DeviceId;
    
    #[test]
    fn test_get_pending_dtcs_with_one_dtc() {
        let dm06 = DM06 {
            device_id: DeviceId::HVPC_UNIT,
            dm06_01spn: 1234,
            dm06_01fmi: 5,
            dm06_01oc: 10,
            dm06_01cm: 0,
            dm06_01spn_high: 0.0,
            // All other slots empty (0xFFFF/0xFF)
            dm06_02spn: 0xFFFF,
            dm06_02fmi: 0xFF,
            // ... rest of fields
            malfunctionindicatorlampstatus: 1,
            redstoplampstatus: 0,
            amberwarninglampstatus: 0,
            protectlampstatus: 0,
            flashmalfuncindicatorlamp: 0,
            flashredstoplamp: 0,
            flashamberwarninglamp: 0,
            flashprotectlamp: 0,
        };
        
        let dtcs = dm06.get_pending_dtcs();
        assert_eq!(dtcs.len(), 1);
        assert_eq!(dtcs[0].spn, 1234);
        assert_eq!(dtcs[0].fmi, 5);
        assert_eq!(dtcs[0].oc, 10);
    }
    
    #[test]
    fn test_lamp_status_summary() {
        let dm06 = DM06 {
            device_id: DeviceId::HVPC_UNIT,
            malfunctionindicatorlampstatus: 1, // On
            redstoplampstatus: 0,               // Off
            amberwarninglampstatus: 1,          // On
            protectlampstatus: 0,               // Off
            // ... other fields
        };
        
        let summary = dm06.lamp_status_summary();
        assert_eq!(summary, "MIL:On RSL:Off AWL:On PL:Off");
    }
}
```

#### 3. Export Helper Trait

Edit `cando-messages/src/j1939/diagnostics/mod.rs`:

```rust
// Add module declaration
mod dm06_helpers;

// Re-export helpers
pub use dm06_helpers::{DM06Helpers, DiagnosticTroubleCode};
```

#### 4. Update lib.rs Exports

```rust
// Add to diagnostics exports
pub use j1939::diagnostics::{
    DM03Helpers,
    DM06Helpers,  // <-- Add this
    DiagnosticMessageType,
    get_diagnostic_message_type,
};
```

#### 5. Write Comprehensive Tests

Create `cando-messages/tests/dm06_comprehensive.rs`:

```rust
use cando_messages::{DM06, DM06Helpers, DeviceId};

#[test]
fn test_dm06_multiple_dtcs() {
    let dm06 = DM06 {
        device_id: DeviceId::HVPC_UNIT,
        // First DTC
        dm06_01spn: 1234,
        dm06_01fmi: 5,
        dm06_01oc: 10,
        dm06_01cm: 0,
        dm06_01spn_high: 0.0,
        // Second DTC
        dm06_02spn: 5678,
        dm06_02fmi: 2,
        dm06_02oc: 3,
        dm06_02cm: 1,
        dm06_02spn_high: 0.0,
        // Rest empty
        // ...
    };
    
    let dtcs = dm06.get_pending_dtcs();
    assert_eq!(dtcs.len(), 2);
    
    assert_eq!(dtcs[0].spn, 1234);
    assert_eq!(dtcs[1].spn, 5678);
}

#[test]
fn test_dm06_no_pending_dtcs() {
    let dm06 = DM06 {
        device_id: DeviceId::HVPC_UNIT,
        // All slots empty
        dm06_01spn: 0xFFFF,
        dm06_01fmi: 0xFF,
        // ...
    };
    
    assert!(!dm06.has_pending_dtcs());
    let dtcs = dm06.get_pending_dtcs();
    assert_eq!(dtcs.len(), 0);
}

#[test]
fn test_dm06_lamp_status_combinations() {
    // Test various lamp combinations
    let test_cases = vec![
        (0, 0, 0, 0, "MIL:Off RSL:Off AWL:Off PL:Off"),
        (1, 1, 0, 0, "MIL:On RSL:On AWL:Off PL:Off"),
        (1, 0, 1, 0, "MIL:On RSL:Off AWL:On PL:Off"),
    ];
    
    for (mil, rsl, awl, pl, expected) in test_cases {
        let dm06 = DM06 {
            device_id: DeviceId::HVPC_UNIT,
            malfunctionindicatorlampstatus: mil,
            redstoplampstatus: rsl,
            amberwarninglampstatus: awl,
            protectlampstatus: pl,
            // ... other fields
        };
        
        assert_eq!(dm06.lamp_status_summary(), expected);
    }
}
```

#### 6. Integrate with can-log-analyzer

Use the helper methods for cleaner output:

```rust
DiagnosticMessageType::DM06 => {
    let dm06 = DM06::decode(log_entry.can_id, &log_entry.data)?;
    
    let mut signals = vec![
        ("LampStatus", dm06.lamp_status_summary()),
    ];
    
    // Add DTC information
    for (i, dtc) in dm06.get_pending_dtcs().iter().enumerate() {
        signals.push((
            format!("DTC_{}", i + 1),
            format!("SPN:{} FMI:{} OC:{}", dtc.spn, dtc.fmi, dtc.oc),
        ));
    }
    
    signals
}
```

### Effort: 2-4 hours per message

---

## Pattern 3: Complex Business Logic

### When to Use

Use this pattern for messages that:
- Have regulatory compliance requirements
- Need complex validation logic
- Implement state machines
- Require deep J1939-73 specification knowledge

**Examples**: DM20 (Monitor Performance Ratio), DM28 (Permanent DTCs), DM29 (Regulated DTC Counts)

### Step-by-Step Guide

#### 1. Research the Specification

Before implementing, thoroughly understand the J1939-73 specification:

- Read the relevant sections of SAE J1939-73
- Understand regulatory requirements (EPA, CARB, EU, etc.)
- Identify validation rules
- Document expected behaviors

**Example**: DM20 Monitor Performance Ratio

From J1939-73:
- Reports numerator/denominator for OBD monitor ratios
- Used for regulatory compliance (EPA 40 CFR Part 86)
- Must track monitor completion vs. driving conditions
- Ratios must be within regulatory limits

#### 2. Create Specification Document

Create `cando-messages/src/j1939/diagnostics/dm20_spec.md`:

```markdown
# DM20 - Monitor Performance Ratio Specification

## Purpose
Reports the performance ratio of on-board diagnostic (OBD) monitors
for regulatory compliance with EPA 40 CFR Part 86.

## Regulatory Context
- Required for OBD II systems
- EPA monitors compliance with emission standards
- Ratios must meet minimum performance criteria

## Message Structure
- Numerator: Times monitor has run to completion
- Denominator: Times conditions were met for monitor
- Ignition Cycle Counter: Total ignition cycles
- OBD Monitoring Conditions: Status flags

## Validation Rules
1. Denominator must be >= Numerator
2. Ratios must be >= 0.0 and <= 1.0
3. Ignition counter must increment monotonically

## Business Logic Requirements
- Calculate performance ratio (Numerator / Denominator)
- Validate ratio is within acceptable range
- Check for regulatory compliance (ratio >= minimum threshold)
```

#### 3. Implement Business Logic

Create `cando-messages/src/j1939/diagnostics/dm20_helpers.rs`:

```rust
//! DM20 Helper Methods - Monitor Performance Ratio
//!
//! Implements business logic for OBD monitor performance ratios
//! per SAE J1939-73 and EPA 40 CFR Part 86.

use crate::generated::j1939_73::DM20;

/// Monitor performance ratio data
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorRatio {
    pub numerator: u16,
    pub denominator: u16,
    pub ratio: f64,
}

/// Regulatory compliance status
#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant { reason: String },
    Insufficient { reason: String },
}

pub trait DM20Helpers {
    /// Calculate the performance ratio
    fn calculate_ratio(&self) -> Result<MonitorRatio, String>;
    
    /// Check if ratio meets regulatory requirements
    fn check_compliance(&self, min_ratio: f64) -> ComplianceStatus;
    
    /// Validate message contents
    fn validate(&self) -> Result<(), String>;
}

impl DM20Helpers for DM20 {
    fn calculate_ratio(&self) -> Result<MonitorRatio, String> {
        // Extract numerator and denominator from message
        let numerator = self.obd_monitoring_conditions_encountered_count;
        let denominator = self.ignition_cycle_counter;
        
        // Validate
        if denominator == 0 {
            return Err("Denominator cannot be zero".to_string());
        }
        
        if numerator > denominator {
            return Err(format!(
                "Numerator ({}) cannot exceed denominator ({})",
                numerator, denominator
            ));
        }
        
        let ratio = numerator as f64 / denominator as f64;
        
        Ok(MonitorRatio {
            numerator: numerator as u16,
            denominator: denominator as u16,
            ratio,
        })
    }
    
    fn check_compliance(&self, min_ratio: f64) -> ComplianceStatus {
        match self.calculate_ratio() {
            Err(e) => ComplianceStatus::Insufficient { 
                reason: format!("Invalid data: {}", e) 
            },
            Ok(ratio_data) => {
                if ratio_data.denominator < 100 {
                    ComplianceStatus::Insufficient {
                        reason: "Insufficient driving conditions (< 100 cycles)".to_string(),
                    }
                } else if ratio_data.ratio >= min_ratio {
                    ComplianceStatus::Compliant
                } else {
                    ComplianceStatus::NonCompliant {
                        reason: format!(
                            "Ratio {:.3} below minimum {:.3}",
                            ratio_data.ratio, min_ratio
                        ),
                    }
                }
            }
        }
    }
    
    fn validate(&self) -> Result<(), String> {
        // Validate numerator <= denominator
        self.calculate_ratio()?;
        
        // Add more validation as needed
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DeviceId;
    
    #[test]
    fn test_calculate_ratio_valid() {
        let dm20 = DM20 {
            device_id: DeviceId::HVPC_UNIT,
            obd_monitoring_conditions_encountered_count: 80,
            ignition_cycle_counter: 100,
            // ... other fields
        };
        
        let ratio = dm20.calculate_ratio().unwrap();
        assert_eq!(ratio.numerator, 80);
        assert_eq!(ratio.denominator, 100);
        assert!((ratio.ratio - 0.8).abs() < 0.001);
    }
    
    #[test]
    fn test_compliance_pass() {
        let dm20 = DM20 {
            device_id: DeviceId::HVPC_UNIT,
            obd_monitoring_conditions_encountered_count: 85,
            ignition_cycle_counter: 100,
            // ...
        };
        
        // EPA minimum is typically 0.336
        let status = dm20.check_compliance(0.336);
        assert!(matches!(status, ComplianceStatus::Compliant));
    }
    
    #[test]
    fn test_compliance_fail() {
        let dm20 = DM20 {
            device_id: DeviceId::HVPC_UNIT,
            obd_monitoring_conditions_encountered_count: 20,
            ignition_cycle_counter: 100,
            // ...
        };
        
        let status = dm20.check_compliance(0.336);
        assert!(matches!(status, ComplianceStatus::NonCompliant { .. }));
    }
    
    #[test]
    fn test_insufficient_data() {
        let dm20 = DM20 {
            device_id: DeviceId::HVPC_UNIT,
            obd_monitoring_conditions_encountered_count: 50,
            ignition_cycle_counter: 80, // < 100 cycles
            // ...
        };
        
        let status = dm20.check_compliance(0.336);
        assert!(matches!(status, ComplianceStatus::Insufficient { .. }));
    }
}
```

#### 4. Document Business Context

Add comprehensive module documentation:

```rust
//! # DM20 - Monitor Performance Ratio
//!
//! ## Business Purpose
//!
//! The Monitor Performance Ratio (MPR) is a regulatory requirement for
//! on-board diagnostic (OBD) systems in vehicles. It measures how often
//! emission-related diagnostic monitors run compared to driving conditions.
//!
//! ## Regulatory Background
//!
//! - **EPA 40 CFR Part 86**: Requires minimum performance ratios
//! - **Typical threshold**: 0.336 (monitors must run 33.6% of the time)
//! - **Enforcement**: Used in vehicle inspections and certification
//!
//! ## Real-World Usage
//!
//! ### Fleet Management
//!
//! Track whether vehicles are meeting emission monitoring requirements:
//! ```rust
//! use cando_messages::{DM20, DM20Helpers};
//!
//! # let dm20 = get_dm20_from_vehicle();
//! match dm20.check_compliance(0.336) {
//!     ComplianceStatus::Compliant => {
//!         println!("Vehicle is compliant");
//!     }
//!     ComplianceStatus::NonCompliant { reason } => {
//!         println!("Vehicle failed: {}", reason);
//!         // Schedule maintenance or inspection
//!     }
//!     ComplianceStatus::Insufficient { reason } => {
//!         println!("Insufficient data: {}", reason);
//!         // Vehicle needs more driving cycles
//!     }
//! }
//! ```
//!
//! ### Diagnostic Tools
//!
//! External diagnostic scanners can query and display MPR:
//! ```rust
//! # use cando_messages::{DM20, DM20Helpers};
//! # let dm20 = get_dm20_from_vehicle();
//! let ratio = dm20.calculate_ratio().unwrap();
//! println!("Monitor ran {} out of {} times ({:.1}%)",
//!     ratio.numerator,
//!     ratio.denominator,
//!     ratio.ratio * 100.