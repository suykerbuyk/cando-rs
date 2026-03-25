//! J1939 Protocol Implementation Module
//!
//! This module provides real implementations for J1939 (SAE J1939 vehicle bus standard)
//! message types, organized by functional categories for better maintainability.
//!
//! # Organization
//!
//! The J1939 implementation is organized into functional modules:
//!
//! - **Engine Control**: Engine management, turbocharger control, exhaust systems
//! - **Braking/Safety**: ABS, emergency braking, safety systems
//! - **Power Management**: Electrical systems, energy management, power supply controls
//! - **Diagnostics**: J1939-73 diagnostic trouble codes, fault monitoring, lamp control
//! - **Sensors**: Basic sensor data (wheel speed, displays)
//! - **Transmission**: Gear control, clutch management (future)
//! - **Vehicle Dynamics**: Speed, motion, stability control (future)
//!
//! # Implementation Pattern
//!
//! Each message follows the established 4-step implementation pattern:
//!
//! 1. **Low-level encode/decode functions** in `{category}/encoders.rs`
//! 2. **Integration methods** in `{category}/implementations.rs`
//! 3. **Round-trip tests** in `tests/j1939_roundtrip.rs`
//! 4. **CLI integration** in `cando-util`
//!
//! # Examples
//!
//! ## Engine Control Messages
//!
//! ```rust
//! use cando_messages::{DeviceId, j1939::EEC21};
//!
//! let message = EEC21 {
//!     device_id: DeviceId::from(0x42),
//!     engn_exhst_mnfld_aslt_prssr_1: 250.5, // 250.5 kPa
//!     engn_exhst_mnfld_aslt_prssr_2: 180.2, // 180.2 kPa
//! };
//!
//! let (can_id, data) = message.encode().unwrap();
//! let decoded = EEC21::decode(can_id, &data).unwrap();
//! ```
//!
//! ## Safety System Messages
//!
//! ```rust
//! use cando_messages::{DeviceId, j1939::AEBS2};
//!
//! let message = AEBS2 {
//!     device_id: DeviceId::from(0x42),
//!     dv_atvt_dd_f_advd_eb_sst: 1,
//!     aebs_2_message_counter: 10,
//!     aebs_2_message_checksum: 5,
//! };
//!
//! let (can_id, data) = message.encode().unwrap();
//! ```

// Import generated J1939 message types
use crate::common::*;
include!("../generated/j1939.rs");

// Import generated J1939-73 diagnostic message types
include!("../generated/j1939_73.rs");

// Functional category modules
pub mod braking_safety;
pub mod diagnostics;
pub mod engine_control;
pub mod power_management;
pub mod sensors;

// ============================================================================
// DTC Helper Framework Exports
// ============================================================================

// Note: DTC-style diagnostic messages (DM01, DM02, DM06, DM12, DM23) are
// already available in this module via the include! statement above.

// Re-export DTC helper framework from diagnostics module
pub use diagnostics::{DiagnosticTroubleCode, DtcMessage, LampState, LampStatus};

// ============================================================================
// Command Message Helper Exports
// ============================================================================

// Re-export command message helpers from diagnostics module
pub use diagnostics::{DM11Helpers, DM13Helpers, DM22Helpers, NetworkAction};

// All encode/decode methods are now generated directly on the message types
// No separate encoder modules needed

// ============================================================================
// Common J1939 Helper Functions
// ============================================================================

/// Validates J1939 CAN ID format.
///
/// J1939 uses 29-bit extended CAN IDs with specific structure.
/// This function verifies the ID is within valid range.
///
/// # Arguments
/// * `can_id` - The CAN identifier to validate
///
/// # Returns
/// `true` if valid J1939 format, `false` otherwise
pub fn is_valid_j1939_can_id(can_id: u32) -> bool {
    // J1939 uses 29-bit extended identifiers
    can_id <= 0x1FFFFFFF
}

/// Extracts device ID from J1939 CAN identifier.
///
/// Simple extraction of the source address (lower 8 bits). For PDU-aware
/// extraction that handles both PDU1 destination and PDU2 source addresses,
/// use [`crate::encoder::extract_device_id`] instead.
///
/// # Arguments
/// * `can_id` - The 29-bit J1939 CAN identifier
///
/// # Returns
/// Extracted device ID value (0-255)
#[deprecated(
    since = "0.1.0",
    note = "Use crate::encoder::extract_device_id() for PDU-aware extraction, or crate::common::CAN_DEVICE_ID_MASK directly"
)]
pub fn extract_j1939_device_id(can_id: u32) -> u8 {
    (can_id & crate::common::CAN_DEVICE_ID_MASK) as u8
}

/// Embeds device ID into J1939 CAN identifier.
///
/// Simple replacement of the lower 8 bits. For PDU-aware embedding that
/// correctly handles PDU1 destination vs PDU2 source addressing, use
/// [`crate::encoder::embed_device_id`] instead.
///
/// # Arguments
/// * `base_can_id` - The base CAN ID (without device ID)
/// * `device_id` - The device ID to embed (0-255)
///
/// # Returns
/// Complete J1939 CAN ID with device ID embedded
#[deprecated(
    since = "0.1.0",
    note = "Use crate::encoder::embed_device_id() for PDU-aware embedding, or crate::common::CAN_BASE_ID_MASK directly"
)]
pub fn embed_j1939_device_id(base_can_id: u32, device_id: u8) -> u32 {
    (base_can_id & crate::common::CAN_BASE_ID_MASK) | (device_id as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j1939_can_id_validation() {
        assert!(is_valid_j1939_can_id(0x00000000));
        assert!(is_valid_j1939_can_id(0x1FFFFFFF));
        assert!(!is_valid_j1939_can_id(0x20000000));
        assert!(!is_valid_j1939_can_id(0xFFFFFFFF));
    }

    #[test]
    fn test_device_id_extraction() {
        assert_eq!(extract_j1939_device_id(0x18FEF18A), 0x8A);
        assert_eq!(extract_j1939_device_id(0x0CFE4800), 0x00);
        assert_eq!(extract_j1939_device_id(0x18EEFF01), 0x01);
    }

    #[test]
    fn test_device_id_embedding() {
        let base_id = 0x18FEF100;
        assert_eq!(embed_j1939_device_id(base_id, 0x8A), 0x18FEF18A);
        assert_eq!(embed_j1939_device_id(base_id, 0x00), 0x18FEF100);
        assert_eq!(embed_j1939_device_id(base_id, 0xFF), 0x18FEF1FF);
    }

    #[test]
    fn test_device_id_round_trip() {
        let original_id = 0x18FEF18A;
        let device = extract_j1939_device_id(original_id);
        let base = original_id & 0xFFFFFF00;
        let reconstructed = embed_j1939_device_id(base, device);
        assert_eq!(original_id, reconstructed);
    }
}
