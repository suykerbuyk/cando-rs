//! J1939-73 Diagnostic Message Framework
//!
//! This module implements the SAE J1939-73 diagnostic standard, providing
//! comprehensive support for vehicle diagnostic trouble codes (DTCs), lamp
//! control, and fault monitoring capabilities.
//!
//! # J1939-73 Diagnostic Standard Overview
//!
//! J1939-73 defines the diagnostic layer for J1939 networks, enabling:
//!
//! - **Active DTC Reporting** (DM01) - Current active faults
//! - **Previously Active DTCs** (DM02) - Historical fault information
//! - **Diagnostic Data Clear/Reset** (DM03) - Fault acknowledgment
//! - **Freeze Frame Parameters** (DM04) - Fault context data
//! - **Diagnostic Readiness** (DM05) - System test completion status
//!
//! # Diagnostic Trouble Code (DTC) Structure
//!
//! Each DTC consists of four key components:
//!
//! ```text
//! ┌─────────────────┬──────────────────────────────────────────┐
//! │ SPN (19 bits)   │ Suspect Parameter Number - identifies    │
//! │                 │ the specific parameter/component         │
//! ├─────────────────┼──────────────────────────────────────────┤
//! │ FMI (5 bits)    │ Failure Mode Identifier - type of fault │
//! │                 │ (0=Above Normal, 1=Below Normal, etc.)   │
//! ├─────────────────┼──────────────────────────────────────────┤
//! │ OC (7 bits)     │ Occurrence Count - fault frequency      │
//! │                 │ (1-125 count, 126=error, 127=N/A)       │
//! ├─────────────────┼──────────────────────────────────────────┤
//! │ CM (1 bit)      │ Conversion Method - data interpretation  │
//! │                 │ flag for diagnostic tools               │
//! └─────────────────┴──────────────────────────────────────────┘
//! ```
//!
//! # Business Applications
//!
//! ## Fleet Management
//!
//! Monitor vehicle health by checking diagnostic lamp status and active faults.
//! The DM01 message provides real-time diagnostic information for fleet operations.
//!
//! ## Predictive Maintenance
//!
//! Track fault occurrence counts to schedule maintenance before critical failures.
//! Higher occurrence counts indicate components requiring immediate attention.
//!
//! ## Regulatory Compliance
//!
//! Generate compliance reports using diagnostic lamp status, particularly the
//! Malfunction Indicator Lamp (MIL) for emissions-related diagnostics.
//!
//! # Implementation Pattern
//!
//! All diagnostic messages follow the proven 6-step implementation pattern:
//!
//! 1. **Low-level encode/decode** in `encoders.rs`
//! 2. **High-level business logic** in `implementations.rs`
//! 3. **Comprehensive testing** (4 tests per message)
//! 4. **CLI integration** with diagnostic aliases
//! 5. **Simulator support** with fault injection capabilities
//! 6. **Documentation** with business context and examples
//!
//! # Example Usage
//!
//! ## Basic Diagnostic Message Access
//!
//! ```rust
//! use cando_messages::{DeviceId, j1939::DM01};
//!
//! // Access diagnostic information from a decoded message
//! # let dm01 = DM01 {
//! #     device_id: DeviceId::from(0x42),
//! #     red_stop_lamp_status: 1,
//! #     malfunction_indicator_lamp_status: 1,
//! #     dm01_01spn: 7945,
//! #     dm01_01fmi: 9,
//! #     protect_lamp_status: 0, amber_warning_lamp_status: 0,
//! #     flash_red_stop_lamp: 0, flash_protect_lamp: 0, flash_malfunc_indicator_lamp: 0, flash_amber_warning_lamp: 0,
//! #     dm01_01oc: 5, dm01_01cm: 0, dm01_01spn_high: 0.0,
//! #     dm01_02spn: 0xFFFF, dm01_02fmi: 0xFF, dm01_02oc: 0xFF, dm01_02cm: 0xFF, dm01_02spn_high: 0.0,
//! #     dm01_03spn: 0xFFFF, dm01_03fmi: 0xFF, dm01_03oc: 0xFF, dm01_03cm: 0xFF, dm01_03spn_high: 0.0,
//! #     dm01_04spn: 0xFFFF, dm01_04fmi: 0xFF, dm01_04oc: 0xFF, dm01_04cm: 0xFF, dm01_04spn_high: 0.0,
//! #     dm01_05spn: 0xFFFF, dm01_05fmi: 0xFF, dm01_05oc: 0xFF, dm01_05cm: 0xFF, dm01_05spn_high: 0.0,
//! #     dm01_06spn: 0xFFFF, dm01_06fmi: 0xFF, dm01_06oc: 0xFF, dm01_06cm: 0xFF, dm01_06spn_high: 0.0,
//! #     dm01_07spn: 0xFFFF, dm01_07fmi: 0xFF, dm01_07oc: 0xFF, dm01_07cm: 0xFF, dm01_07spn_high: 0.0,
//! #     dm01_08spn: 0xFFFF, dm01_08fmi: 0xFF, dm01_08oc: 0xFF, dm01_08cm: 0xFF, dm01_08spn_high: 0.0,
//! #     dm01_09spn: 0xFFFF, dm01_09fmi: 0xFF, dm01_09oc: 0xFF, dm01_09cm: 0xFF, dm01_09spn_high: 0.0,
//! #     dm01_10spn: 0xFFFF, dm01_10fmi: 0xFF, dm01_10oc: 0xFF, dm01_10cm: 0xFF, dm01_10spn_high: 0.0,
//! #     dm01_11spn: 0xFFFF, dm01_11fmi: 0xFF, dm01_11oc: 0xFF, dm01_11cm: 0xFF, dm01_11spn_high: 0.0,
//! #     dm01_12spn: 0xFFFF, dm01_12fmi: 0xFF, dm01_12oc: 0xFF, dm01_12cm: 0xFF, dm01_12spn_high: 0.0,
//! #     dm01_13spn: 0xFFFF, dm01_13fmi: 0xFF, dm01_13oc: 0xFF, dm01_13cm: 0xFF, dm01_13spn_high: 0.0,
//! #     dm01_14spn: 0xFFFF, dm01_14fmi: 0xFF, dm01_14oc: 0xFF, dm01_14cm: 0xFF, dm01_14spn_high: 0.0,
//! #     dm01_15spn: 0xFFFF, dm01_15fmi: 0xFF, dm01_15oc: 0xFF, dm01_15cm: 0xFF, dm01_15spn_high: 0.0,
//! #     dm01_16spn: 0xFFFF, dm01_16fmi: 0xFF, dm01_16oc: 0xFF, dm01_16cm: 0xFF, dm01_16spn_high: 0.0,
//! #     dm01_17spn: 0xFFFF, dm01_17fmi: 0xFF, dm01_17oc: 0xFF, dm01_17cm: 0xFF, dm01_17spn_high: 0.0,
//! #     dm01_18spn: 0xFFFF, dm01_18fmi: 0xFF, dm01_18oc: 0xFF, dm01_18cm: 0xFF, dm01_18spn_high: 0.0,
//! #     dm01_19spn: 0xFFFF, dm01_19fmi: 0xFF, dm01_19oc: 0xFF, dm01_19cm: 0xFF, dm01_19spn_high: 0.0,
//! # };
//!
//! println!("Device {}: Diagnostic status", dm01.device_id.to_u8());
//!
//! // Check for critical conditions
//! if dm01.red_stop_lamp_status == 1 {
//!     println!("Critical condition - red stop lamp active");
//! }
//!
//! // Check for active faults
//! if dm01.dm01_01spn != 0xFFFF && dm01.dm01_01fmi != 0xFF {
//!     println!("Active fault: SPN {}, FMI {}", dm01.dm01_01spn, dm01.dm01_01fmi);
//! }
//! ```
//!
//! # Technical Architecture
//!
//! The diagnostic framework is built on several key principles:
//!
//! ## Separation of Concerns
//!
//! - **Low-level encoding/decoding** handles bit manipulation and CAN frame formatting
//! - **Business logic layer** provides diagnostic-specific validation and convenience methods
//! - **Application layer** integrates with fleet management and maintenance systems
//!
//! ## Real-world Validation
//!
//! All implementations are validated against actual vehicle data:
//!
//! - **Mystery Solved**: Unknown CAN messages 0x18FECA88/0x18FECA82 identified as DM01
//! - **Real Data**: SPN 7945, FMI 9, OC 5 from actual refrigeration system faults
//! - **Production Ready**: Handles edge cases and regulatory compliance requirements
//!
//! ## Scalability Architecture
//!
//! The framework supports expansion to the full J1939-73 diagnostic suite:
//!
//! - **DM01-DM05**: Core diagnostic messages (DM01 complete, others ready for implementation)
//! - **DM06-DM31**: Extended diagnostic capabilities (future expansion)
//! - **Multiple DTCs**: Each message can report up to 19 simultaneous faults
//!
//! # Message Implementation Status
//!
//! - ✅ **DM01** - Active Diagnostic Trouble Codes (Complete)
//! - 🔄 **DM02** - Previously Active DTCs (Ready for implementation)
//! - 🔄 **DM03** - Clear/Reset DTCs (Ready for implementation)
//! - 🔄 **DM04** - Freeze Frame Data (Ready for implementation)
//! - 🔄 **DM05** - Diagnostic Readiness (Ready for implementation)
//!
//! # Integration Points
//!
//! The diagnostic framework integrates with:
//!
//! - **J1939 Simulator**: 28 message types including DM01 broadcasting
//! - **CLI Tools**: `rust-can-util` diagnostic message support
//! - **WebSocket API**: Real-time diagnostic monitoring
//! - **Fleet Management**: Fault injection, lamp control, maintenance scheduling
//!
//! This provides a complete end-to-end diagnostic solution from CAN bus
//! message reception through business logic processing to fleet management
//! system integration.

use crate::common::DeviceId;

/// J1939-73 Diagnostic Message Types
///
/// Covers DM01-DM31 diagnostic messages as defined in SAE J1939-73.
/// Not all message types are currently implemented in the codebase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticMessageType {
    /// DM01 - Active Diagnostic Trouble Codes
    DM01 = 0x01,
    /// DM02 - Previously Active Diagnostic Trouble Codes
    DM02 = 0x02,
    /// DM03 - Diagnostic Data Clear/Reset
    DM03 = 0x03,
    /// DM04 - Freeze Frame
    DM04 = 0x04,
    /// DM05 - Diagnostic Readiness 1
    DM05 = 0x05,
    /// DM06 - Pending Diagnostic Trouble Codes
    DM06 = 0x06,
    /// DM07 - Command Non-Continuously Monitored Test
    DM07 = 0x07,
    /// DM08 - Test Results
    DM08 = 0x08,
    /// DM11 - Diagnostic Data Clear/Reset for Active DTCs
    DM11 = 0x0B,
    /// DM12 - Emissions-Related Active DTCs
    DM12 = 0x0C,
    /// DM13 - Stop Start Broadcast
    DM13 = 0x0D,
    /// DM20 - Monitor Performance Ratio
    DM20 = 0x14,
    /// DM21 - Diagnostic Readiness 2
    DM21 = 0x15,
    /// DM22 - Individual Clear/Reset of Active and Previously Active DTC
    DM22 = 0x16,
    /// DM23 - Emissions-Related Previously Active DTC
    DM23 = 0x17,
    /// DM24 - SPN Support
    DM24 = 0x18,
    /// DM25 - Expanded Freeze Frame
    DM25 = 0x19,
    /// DM26 - Diagnostic Readiness 3
    DM26 = 0x1A,
    /// DM27 - All Pending DTCs
    DM27 = 0x1B,
    /// DM28 - Permanent DTCs
    DM28 = 0x1C,
    /// DM29 - Regulated DTC Counts
    DM29 = 0x1D,
    /// DM30 - Scaled Test Results
    DM30 = 0x1E,
    /// DM31 - DTC to Lamp Association
    DM31 = 0x1F,
}

impl DiagnosticMessageType {
    /// Get the PGN for this diagnostic message type
    pub const fn pgn(self) -> u32 {
        match self {
            DiagnosticMessageType::DM01 => 0xFECA,
            DiagnosticMessageType::DM02 => 0xFECB,
            DiagnosticMessageType::DM03 => 0xFECC,
            DiagnosticMessageType::DM04 => 0xFECD,
            DiagnosticMessageType::DM05 => 0xFECE,
            DiagnosticMessageType::DM06 => 0xFECF,
            DiagnosticMessageType::DM07 => 0xFED3,
            DiagnosticMessageType::DM08 => 0xFED4,
            DiagnosticMessageType::DM11 => 0xFED5,
            DiagnosticMessageType::DM12 => 0xFED6,
            DiagnosticMessageType::DM13 => 0xFED7,
            DiagnosticMessageType::DM20 => 0xFEB4,
            DiagnosticMessageType::DM21 => 0xFEB5,
            DiagnosticMessageType::DM22 => 0xFEB6,
            DiagnosticMessageType::DM23 => 0xFEB7,
            DiagnosticMessageType::DM24 => 0xFEB8,
            DiagnosticMessageType::DM25 => 0xFDB7,
            DiagnosticMessageType::DM26 => 0xFDB8,
            DiagnosticMessageType::DM27 => 0xFDB9,
            DiagnosticMessageType::DM28 => 0xFDBA,
            DiagnosticMessageType::DM29 => 0xFDBB,
            DiagnosticMessageType::DM30 => 0xFEBC,
            DiagnosticMessageType::DM31 => 0xFEBD,
        }
    }

    /// Get the BASE_CAN_ID for this diagnostic message type (PGN + priority)
    pub const fn base_can_id(self) -> u32 {
        // J1939 format: Priority (3 bits) + Reserved (1 bit) + DP (1 bit) + PF (8 bits) + PS (8 bits) + SA (8 bits)
        // For diagnostic messages: Priority 6 (0x18), format: 0x18 << 24 | PGN << 8
        0x18000000 | (self.pgn() << 8)
    }

    /// Get human-readable name
    pub const fn name(self) -> &'static str {
        match self {
            DiagnosticMessageType::DM01 => "DM01_Active_DTCs",
            DiagnosticMessageType::DM02 => "DM02_Previously_Active_DTCs",
            DiagnosticMessageType::DM03 => "DM03_Clear_Reset_DTCs",
            DiagnosticMessageType::DM04 => "DM04_Freeze_Frame",
            DiagnosticMessageType::DM05 => "DM05_Diagnostic_Readiness_1",
            DiagnosticMessageType::DM06 => "DM06_Pending_DTCs",
            DiagnosticMessageType::DM07 => "DM07_Command_Non_Continuously_Monitored_Test",
            DiagnosticMessageType::DM08 => "DM08_Test_Results",
            DiagnosticMessageType::DM11 => "DM11_Clear_Active_DTCs",
            DiagnosticMessageType::DM12 => "DM12_Emissions_Active_DTCs",
            DiagnosticMessageType::DM13 => "DM13_Stop_Start_Broadcast",
            DiagnosticMessageType::DM20 => "DM20_Monitor_Performance_Ratio",
            DiagnosticMessageType::DM21 => "DM21_Diagnostic_Readiness_2",
            DiagnosticMessageType::DM22 => "DM22_Individual_Clear_Reset",
            DiagnosticMessageType::DM23 => "DM23_Emissions_Previously_Active",
            DiagnosticMessageType::DM24 => "DM24_SPN_Support",
            DiagnosticMessageType::DM25 => "DM25_Expanded_Freeze_Frame",
            DiagnosticMessageType::DM26 => "DM26_Diagnostic_Readiness_3",
            DiagnosticMessageType::DM27 => "DM27_All_Pending_DTCs",
            DiagnosticMessageType::DM28 => "DM28_Permanent_DTCs",
            DiagnosticMessageType::DM29 => "DM29_Regulated_DTC_Counts",
            DiagnosticMessageType::DM30 => "DM30_Scaled_Test_Results",
            DiagnosticMessageType::DM31 => "DM31_DTC_Lamp_Association",
        }
    }
}

/// Detect diagnostic message type from CAN ID
///
/// Extracts the PGN from the CAN ID and matches against known diagnostic message types.
///
/// # Arguments
/// * `can_id` - J1939 extended CAN ID (29-bit)
///
/// # Returns
/// * `Some(DiagnosticMessageType)` if the CAN ID matches a known diagnostic message
/// * `None` if not a diagnostic message
///
/// # Example
/// ```
/// use cando_messages::j1939::diagnostics::{get_diagnostic_message_type, DiagnosticMessageType};
///
/// let dm01_can_id = 0x18FECA00; // DM01 from device 0x00
/// let dm_type = get_diagnostic_message_type(dm01_can_id);
/// assert_eq!(dm_type, Some(DiagnosticMessageType::DM01));
///
/// let non_diag_can_id = 0x18FEF100; // EEC1 (not diagnostic)
/// assert_eq!(get_diagnostic_message_type(non_diag_can_id), None);
/// ```
pub fn get_diagnostic_message_type(can_id: u32) -> Option<DiagnosticMessageType> {
    // Extract PGN from CAN ID (bits 8-23)
    let pgn = (can_id >> 8) & 0xFFFF;

    match pgn {
        0xFECA => Some(DiagnosticMessageType::DM01),
        0xFECB => Some(DiagnosticMessageType::DM02),
        0xFECC => Some(DiagnosticMessageType::DM03),
        0xFECD => Some(DiagnosticMessageType::DM04),
        0xFECE => Some(DiagnosticMessageType::DM05),
        0xFECF => Some(DiagnosticMessageType::DM06),
        0xFED3 => Some(DiagnosticMessageType::DM07),
        0xFED4 => Some(DiagnosticMessageType::DM08),
        0xFED5 => Some(DiagnosticMessageType::DM11),
        0xFED6 => Some(DiagnosticMessageType::DM12),
        0xFED7 => Some(DiagnosticMessageType::DM13),
        0xFEB4 => Some(DiagnosticMessageType::DM20),
        0xFEB5 => Some(DiagnosticMessageType::DM21),
        0xFEB6 => Some(DiagnosticMessageType::DM22),
        0xFEB7 => Some(DiagnosticMessageType::DM23),
        0xFEB8 => Some(DiagnosticMessageType::DM24),
        0xFDB7 => Some(DiagnosticMessageType::DM25),
        0xFDB8 => Some(DiagnosticMessageType::DM26),
        0xFDB9 => Some(DiagnosticMessageType::DM27),
        0xFDBA => Some(DiagnosticMessageType::DM28),
        0xFDBB => Some(DiagnosticMessageType::DM29),
        0xFEBC => Some(DiagnosticMessageType::DM30),
        0xFEBD => Some(DiagnosticMessageType::DM31),
        _ => None,
    }
}

// DM03 Helper Methods Module
mod dm03_helpers;
pub use dm03_helpers::DM03Helpers;

// DTC Helper Framework - Common helpers for DM01, DM02, DM06, DM12, DM23
mod dtc_helpers;
mod dtc_impls;
pub use dtc_helpers::{DiagnosticTroubleCode, DtcMessage, LampState, LampStatus};

// Command Message Helpers - DM11, DM13, DM22
mod command_helpers;
pub use command_helpers::{DM11Helpers, DM13Helpers, DM22Helpers, NetworkAction};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_type_detection() {
        // Test DM01
        assert_eq!(
            get_diagnostic_message_type(0x18FECA00),
            Some(DiagnosticMessageType::DM01)
        );
        assert_eq!(
            get_diagnostic_message_type(0x18FECA88),
            Some(DiagnosticMessageType::DM01)
        );

        // Test DM02
        assert_eq!(
            get_diagnostic_message_type(0x18FECB00),
            Some(DiagnosticMessageType::DM02)
        );

        // Test DM03
        assert_eq!(
            get_diagnostic_message_type(0x18FECC00),
            Some(DiagnosticMessageType::DM03)
        );

        // Test non-diagnostic message
        assert_eq!(get_diagnostic_message_type(0x18FEF100), None);
    }

    #[test]
    fn test_diagnostic_type_pgn() {
        assert_eq!(DiagnosticMessageType::DM01.pgn(), 0xFECA);
        assert_eq!(DiagnosticMessageType::DM02.pgn(), 0xFECB);
        assert_eq!(DiagnosticMessageType::DM03.pgn(), 0xFECC);
    }

    #[test]
    fn test_diagnostic_type_base_can_id() {
        assert_eq!(DiagnosticMessageType::DM01.base_can_id(), 0x18FECA00);
        assert_eq!(DiagnosticMessageType::DM02.base_can_id(), 0x18FECB00);
        assert_eq!(DiagnosticMessageType::DM03.base_can_id(), 0x18FECC00);
    }

    #[test]
    fn test_diagnostic_type_name() {
        assert_eq!(DiagnosticMessageType::DM01.name(), "DM01_Active_DTCs");
        assert_eq!(
            DiagnosticMessageType::DM02.name(),
            "DM02_Previously_Active_DTCs"
        );
        assert_eq!(DiagnosticMessageType::DM03.name(), "DM03_Clear_Reset_DTCs");
    }
}
