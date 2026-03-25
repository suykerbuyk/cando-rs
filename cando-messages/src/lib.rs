//! # Cando Messages - Generated CAN Message Types
//!
//! This crate provides strongly-typed Rust representations of CAN bus messages
//! for industrial motor control systems using the SAE J1939 protocol standard.
//!
//! ## Overview
//!
//! The crate is built around code generation from DBC (Database CAN) files,
//! providing type-safe message encoding/decoding with proper signal scaling,
//! validation, and device ID handling.
//!
//! ## Key Features
//!
//! - **Type Safety**: All signals are represented as proper Rust types with validation
//! - **Device ID Support**: J1939-style extended CAN identifiers with embedded device IDs
//! - **Signal Scaling**: Automatic conversion between raw CAN values and engineering units
//! - **Error Handling**: Comprehensive error types for validation and decoding failures
//! - **Generated Code**: Message types generated from industry-standard DBC files
//!
//! ## Quick Start
//!
//! ### Working with Device IDs
//!
//! ```rust
//! use cando_messages::{DeviceId, ValidationError};
//!
//! // Create device IDs from raw values
//! let device = DeviceId::from(0x42);
//! assert_eq!(device.as_u8(), 0x42);
//!
//! // All 256 J1939 device addresses are valid (0x00-0xFF)
//! let broadcast = DeviceId::from(0xFF);
//! assert_eq!(broadcast, DeviceId::BROADCAST);
//! assert!(broadcast.is_broadcast());
//!
//! // Use semantic methods to check address categories
//! let normal_device = DeviceId::from(0x01);
//! assert!(normal_device.is_normal_device());
//! # Ok::<(), ValidationError>(())
//! ```
//!
//! ### Working with Scaled Values
//!
//! ```rust
//! use cando_messages::{Percentage, MotorSpeed, ValidationError};
//!
//! // Create percentage values with validation
//! let speed_percent = Percentage::new(75.0)?;
//! assert_eq!(speed_percent.value(), 75.0);
//!
//! // Values are range-checked
//! let invalid_percent = Percentage::new(150.0);
//! assert!(invalid_percent.is_err());
//!
//! // Motor speed with proper scaling
//! let motor_speed = MotorSpeed::new(1500.0)?;
//! assert_eq!(motor_speed.rpm(), 1500.0);
//! # Ok::<(), ValidationError>(())
//! ```
//!
//! ### Error Handling
//!
//! ```rust
//! use cando_messages::DeviceId;
//!
//! // All 256 J1939 device addresses are valid (0x00-0xFF)
//! let device = DeviceId::from(0x01);
//! println!("Valid device: {:?}", device);
//! assert!(device.is_normal_device());
//!
//! // Use semantic methods to check address categories
//! let broadcast = DeviceId::from(0xFF);
//! assert!(broadcast.is_broadcast());
//! ```
//!
//! ### Signal Processing
//!
//! ```
//! use cando_messages::encoder::{apply_scaling, apply_inverse_scaling};
//!
//! // Convert raw CAN values to engineering units
//! let raw_value = 2000u64;
//! let rpm = apply_scaling(raw_value, 0.5, 0.0, false, 16);
//! assert_eq!(rpm, 1000.0);
//!
//! // Convert engineering units back to raw values
//! let raw_back = apply_inverse_scaling(1000.0, 0.5, 0.0, 16);
//! assert_eq!(raw_back, 2000);
//! ```
//!
//! ### Complete Example
//!
//! ```
//! use cando_messages::{DeviceId, Percentage, MotorSpeed};
//!
//! // Create a valid device ID
//! let device = DeviceId::try_from(0x42).unwrap();
//! assert_eq!(device.as_u8(), 0x42);
//!
//! // Work with scaled percentage values
//! let speed_percent = Percentage::new(75.0).unwrap();
//! assert_eq!(speed_percent.value(), 75.0);
//!
//! // Work with motor speeds
//! let motor_rpm = MotorSpeed::new(1500.0).unwrap();
//! assert_eq!(motor_rpm.rpm(), 1500.0);
//!
//! // Validation catches invalid values
//! // All 256 J1939 device addresses are now valid (0x00-0xFF)
//! assert!(DeviceId::from(0xFF).is_broadcast());
//! assert!(DeviceId::from(0x01).is_normal_device());
//! assert!(Percentage::new(200.0).is_err());
//! assert!(MotorSpeed::new(-100.0).is_err());
//! ```
//!
//! ## Module Organization
//!
//! - [`common`] - Common types, errors, and traits used across all message types
//! - [`encoder`] - Low-level bit manipulation and signal processing functions
//! - [`metadata`] - Compile-time metadata for message and signal introspection
//! - [`j1939`] - Generated J1939 message types from j1939.dbc

#![allow(unused)]

/// Common types, errors, and traits shared across all message types.
///
/// This module provides the foundational types used throughout the cando_messages
/// crate, including device identifiers, validation errors, scaling helpers, and
/// the base traits that all CAN messages implement.
///
/// # Examples
///
/// ```
/// use cando_messages::common::{DeviceId, ValidationError};
///
/// // Working with device IDs - all 256 J1939 addresses are valid
/// let device = DeviceId::from(0x42);
/// assert_eq!(u8::from(device), 0x42);
///
/// // Test special addresses
/// let broadcast = DeviceId::from(0xFF);
/// assert_eq!(broadcast, DeviceId::BROADCAST);
/// assert!(broadcast.is_broadcast());
///
/// // Test normal device addresses
/// let normal_device = DeviceId::from(0x01);
/// assert!(normal_device.is_normal_device());
/// ```
pub mod common;

/// Low-level encoder/decoder implementations for CAN message processing.
///
/// This module provides the bit-level manipulation functions used to encode
/// and decode CAN message data, including signal extraction, scaling, and
/// device ID embedding/extraction.
///
/// Most users will not need to use these functions directly, as they are
/// used internally by the generated message types.
///
/// # Examples
///
/// ```rust
/// use cando_messages::encoder::{extract_signal, pack_signal};
///
/// let data = [0b10101010, 0b11001100];
///
/// // Extract 4 bits starting at bit position 2
/// let value = extract_signal(&data, 2, 4).unwrap();
/// assert_eq!(value, 0b1010);  // Extracted bits: 1010
/// ```
pub mod encoder;

/// Compile-time metadata for CAN messages and signals.
///
/// This module provides metadata structures that describe the properties of CAN
/// messages and signals as defined in DBC files. Unlike runtime DBC parsing, this
/// metadata is generated at compile-time and embedded as static data.
///
/// # Examples
///
/// ```rust
/// use cando_messages::metadata::{HasMetadata, ProtocolMetadata};
/// use cando_messages::j1939::WAND;
///
/// // Access message metadata
/// let meta = WAND::metadata();
/// println!("Message: {}, CAN ID: 0x{:X}", meta.name, meta.can_id);
///
/// // Iterate over signals
/// for signal in meta.signals {
///     println!("  Signal: {}, Unit: {}", signal.name, signal.unit);
/// }
/// ```
pub mod metadata;

/// Generated J1939 message types from j1939.dbc.
///
/// This module contains all the message and signal types generated from the
/// j1939.dbc file, representing the SAE J1939 vehicle bus standard used in
/// heavy-duty vehicles and industrial equipment.
///
/// The J1939 standard defines Parameter Group Numbers (PGNs) for message
/// identification and Suspect Parameter Numbers (SPNs) for individual data
/// parameters.
///
/// # Examples
///
/// ```
/// use cando_messages::j1939::*;
///
/// // Working with J1939 messages
/// // (specific examples will depend on available messages in j1939.dbc)
/// ```
pub mod j1939;

// Re-export some common types for convenience
pub use common::*;

// Re-export metadata types for easy access
pub use metadata::{
    ByteOrder, HasMetadata, MessageMetadata, ProtocolMetadata, SignalMetadata, ValueType,
};

// ============================================================================
// J1939-73 Phase 4: Simple Diagnostic Messages (Pattern 1)
// ============================================================================
//
// These messages require no business logic or helper methods beyond the
// generated encode/decode functions. They work perfectly as-is from the
// code generator.

/// DM04 - Freeze Frame Parameters
/// Captures system state at the moment a diagnostic trouble code occurs.
pub use j1939::DM04;

/// DM05 - Diagnostic Readiness 1
/// Reports diagnostic system readiness and OBD compliance status.
pub use j1939::DM05;

/// DM07 - Command Non-Continuously Monitored Test
/// Commands a device to execute a specific diagnostic test.
pub use j1939::DM07;

/// DM08 - Test Results
/// Reports results from non-continuously monitored diagnostic tests.
pub use j1939::DM08;

/// DM24 - SPN Support
/// Identifies which SPNs are supported for freeze frames and data streams.
pub use j1939::DM24;

/// DM30 - Scaled Test Results
/// Provides scaled test results for requested diagnostic tests.
pub use j1939::DM30;

// ============================================================================
// J1939-73 Phase 5: Advanced Diagnostic Messages
// ============================================================================
//
// These messages provide extended diagnostic capabilities including
// monitor performance ratios and diagnostic readiness status.

/// DM20 - Monitor Performance Ratio
/// Reports the ratio of monitoring conditions encountered to opportunities.
pub use j1939::DM20;

/// DM21 - Diagnostic Readiness 2
/// Reports distance and time metrics related to MIL activation and DTC clearing.
pub use j1939::DM21;

// ============================================================================
// J1939-73 Phase 6: Additional Diagnostic Messages
// ============================================================================
//
// These messages provide regulated DTC counting and DTC-to-lamp association
// capabilities for comprehensive diagnostic reporting.

/// DM29 - Regulated DTC Counts
/// Reports counts of various DTC categories for regulatory compliance.
pub use j1939::DM29;

/// DM31 - DTC to Lamp Association
/// Reports which lamp should be illuminated for a specific DTC.
pub use j1939::DM31;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j1939_protocol_metadata() {
        let protocol = &j1939::J1939_METADATA;

        assert_eq!(protocol.name, "J1939");
        assert!(protocol.message_count() > 0);
        assert!(protocol.total_signal_count() > 0);

        // Verify we can find some sample messages by name
        // These are actual messages from j1939.dbc
        assert!(protocol.find_message("CN").is_some());
        assert!(protocol.find_message("WAND").is_some());
        assert!(protocol.find_message("LDISP").is_some());
    }
}
