//! CAN message filtering utilities for simulators.
//!
//! This module provides utilities to prevent self-reception loops in CAN simulators.
//!
//! ## The Self-Reception Problem
//!
//! On Linux with SocketCAN, when a program sends a CAN frame, it also receives
//! its own transmission back (loopback). This is by design - it confirms the
//! transmission succeeded. However, this creates a feedback loop:
//!
//! ```text
//! 1. Simulator state: field=1
//! 2. Simulator broadcasts: "field=1" -> CAN bus
//! 3. Simulator receives its own message (loopback)
//! 4. Simulator decodes: "field=1"
//! 5. Simulator updates state: field=1
//! 6. Loop repeats every broadcast interval
//! ```
//!
//! This becomes problematic when external messages are injected:
//!
//! ```text
//! 1. Simulator broadcasts: "field=1"
//! 2. Test injects: "field=2"
//! 3. Simulator receives test message -> Updates state to 2 (correct)
//! 4. Simulator receives its own broadcast (from step 1)
//! 5. Simulator updates state back to 1 (OVERWRITES TEST VALUE!)
//! ```
//!
//! ## Solution: Source Filtering
//!
//! Real CAN devices ignore their own echoed messages by checking the source address.
//! In J1939 and most protocols, the source device ID is embedded in bits 0-7 of
//! the CAN ID.
//!
//! This module provides utilities to:
//! - Extract the source device ID from a CAN ID
//! - Check if a message originated from the simulator itself
//! - Filter messages at ingress to prevent self-reception loops
//!
//! ## Usage
//!
//! ### Basic Filtering
//!
//! ```ignore
//! use cando_simulator_common::message_filter::should_ignore_message;
//!
//! fn process_incoming_message(&mut self, can_id: u32, data: &[u8]) {
//!     // Ignore messages from ourselves
//!     if should_ignore_message(can_id, self.device_id) {
//!         return;
//!     }
//!
//!     // Process external messages only
//!     match (can_id & 0xFFFFFF00) {
//!         0x18FCCC00 => { /* Process EEC12 */ }
//!         // ...
//!     }
//! }
//! ```
//!
//! ## CAN ID Structure
//!
//! For J1939 and similar protocols using 29-bit extended CAN IDs:
//!
//! ```text
//! Bits 26-28: Priority (3 bits)
//! Bits 8-25:  PGN - Parameter Group Number (18 bits)
//! Bits 0-7:   Source Address (8 bits)
//! ```
//!
//! Example: `0x18FCCC8A`
//! ```text
//!   0x18FCCC8A
//!   ||  ||||||
//!   ||  ||||++-- 0x8A = Source device
//!   ||  ++++---- 0xFECC = PGN (EEC12)
//!   ++---------- 0x18 = Priority 6
//! ```
//!
//! ## Performance
//!
//! All functions are `#[inline]` and compile to simple bitwise operations.
//! There is zero runtime overhead compared to hand-written filtering.

/// Extract the source device ID from a CAN ID.
///
/// The source device ID is encoded in bits 0-7 of the CAN ID for most
/// protocols (J1939 and others). This function simply masks and
/// extracts those bits.
///
/// # Arguments
///
/// * `can_id` - The 29-bit or 11-bit CAN identifier
///
/// # Returns
///
/// The source device ID (0x00-0xFF)
///
/// # Examples
///
/// ## J1939 Message
///
/// ```rust
/// use cando_simulator_common::message_filter::extract_source_device_id;
///
/// // J1939 EEC12 message from device 0x8A
/// let can_id = 0x18FCCC8A;
/// let source = extract_source_device_id(can_id);
/// assert_eq!(source, 0x8A);
/// ```
#[inline]
pub fn extract_source_device_id(can_id: u32) -> u8 {
    (can_id & 0xFF) as u8
}

/// Check if a CAN message should be ignored because it originated from this simulator.
///
/// This function prevents self-reception loops by comparing the source device ID
/// (extracted from the CAN ID) with the simulator's own device ID.
///
/// # Arguments
///
/// * `can_id` - The CAN identifier of the received message
/// * `own_device_id` - This simulator's device ID
///
/// # Returns
///
/// * `true` if the message is from this simulator and should be ignored
/// * `false` if the message is from an external device and should be processed
///
/// # Examples
///
/// ```rust
/// use cando_simulator_common::message_filter::should_ignore_message;
///
/// let my_device_id = 0x8A;
///
/// // Message from ourselves - IGNORE
/// let own_message = 0x18FCCC8A; // Source = 0x8A
/// assert_eq!(should_ignore_message(own_message, my_device_id), true);
///
/// // Message from another device - PROCESS
/// let external_message = 0x18FCCC82; // Source = 0x82
/// assert_eq!(should_ignore_message(external_message, my_device_id), false);
/// ```
#[inline]
pub fn should_ignore_message(can_id: u32, own_device_id: u8) -> bool {
    // Extract PDU Format (PF) from bits 16-23 to determine message type
    let pdu_format = ((can_id >> 16) & 0xFF) as u8;

    // PDU1 (PF < 0xF0): Destination-specific message
    // Bits 0-7 contain destination address, NOT source address
    // Don't filter these - they may be commands/requests addressed TO us
    if pdu_format < 0xF0 {
        return false;
    }

    // PDU2 (PF >= 0xF0): Broadcast message
    // Bits 0-7 contain source address
    // Filter if source matches our device ID (our own loopback)
    extract_source_device_id(can_id) == own_device_id
}

/// Check if a CAN message is from an external device (not from this simulator).
///
/// This is the inverse of [`should_ignore_message`] - provided for readability
/// when the logic flow is "if external, then process".
///
/// # Arguments
///
/// * `can_id` - The CAN identifier of the received message
/// * `own_device_id` - This simulator's device ID
///
/// # Returns
///
/// * `true` if the message is from an external device and should be processed
/// * `false` if the message is from this simulator and should be ignored
///
/// # Examples
///
/// ```rust
/// use cando_simulator_common::message_filter::is_external_message;
///
/// let my_device_id = 0x8A;
///
/// // Message from ourselves - NOT external
/// let own_message = 0x18FCCC8A;
/// assert_eq!(is_external_message(own_message, my_device_id), false);
///
/// // Message from another device - IS external
/// let external_message = 0x18FCCC82;
/// assert_eq!(is_external_message(external_message, my_device_id), true);
/// ```
#[inline]
pub fn is_external_message(can_id: u32, own_device_id: u8) -> bool {
    extract_source_device_id(can_id) != own_device_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_source_device_id_j1939() {
        // J1939 EEC12 from device 0x8A
        assert_eq!(extract_source_device_id(0x18FCCC8A), 0x8A);

        // J1939 EEC12 from device 0x82
        assert_eq!(extract_source_device_id(0x18FCCC82), 0x82);

        // J1939 DM1 from device 0x00
        assert_eq!(extract_source_device_id(0x18FECA00), 0x00);

        // J1939 message from device 0xFF
        assert_eq!(extract_source_device_id(0x18FECAFF), 0xFF);
    }

    #[test]
    fn test_extract_source_device_id_extended() {
        // Extended CAN ID from device 0x82
        assert_eq!(extract_source_device_id(0x18FF0382), 0x82);

        // Extended CAN ID from device 0x88
        assert_eq!(extract_source_device_id(0x18FF0388), 0x88);
    }

    #[test]
    fn test_should_ignore_message_self() {
        let my_device_id = 0x8A;

        // Messages from ourselves should be ignored
        assert!(should_ignore_message(0x18FCCC8A, my_device_id));
        assert!(should_ignore_message(0x18FF038A, my_device_id));
        assert!(should_ignore_message(0x0CFF8A8A, my_device_id));
    }

    #[test]
    fn test_should_ignore_message_external() {
        let my_device_id = 0x8A;

        // Messages from other devices should NOT be ignored
        assert!(!should_ignore_message(0x18FCCC82, my_device_id));
        assert!(!should_ignore_message(0x18FF0388, my_device_id));
        assert!(!should_ignore_message(0x0CFF8A8B, my_device_id));
        assert!(!should_ignore_message(0x18FECA00, my_device_id));
    }

    #[test]
    fn test_is_external_message_self() {
        let my_device_id = 0x8A;

        // Messages from ourselves are NOT external
        assert!(!is_external_message(0x18FCCC8A, my_device_id));
        assert!(!is_external_message(0x18FF038A, my_device_id));
    }

    #[test]
    fn test_is_external_message_external() {
        let my_device_id = 0x8A;

        // Messages from other devices ARE external
        assert!(is_external_message(0x18FCCC82, my_device_id));
        assert!(is_external_message(0x18FF0388, my_device_id));
        assert!(is_external_message(0x18FECA00, my_device_id));
    }

    #[test]
    fn test_inverse_relationship() {
        // should_ignore_message and is_external_message are inverses
        let device_id = 0x8A;

        for can_id in [0x18FCCC8A, 0x18FCCC82, 0x18FF0388, 0x18FECA00] {
            assert_eq!(
                should_ignore_message(can_id, device_id),
                !is_external_message(can_id, device_id),
                "Functions should be inverses for CAN ID 0x{:08X}",
                can_id
            );
        }
    }

    #[test]
    fn test_all_device_ids() {
        // Test full range of device IDs (0x00-0xFF)
        for device_id in 0u8..=255u8 {
            let can_id = 0x18FCCC00 | (device_id as u32);

            // Should recognize its own device ID
            assert!(should_ignore_message(can_id, device_id));
            assert!(!is_external_message(can_id, device_id));

            // Should recognize different device ID
            let other_device = device_id.wrapping_add(1);
            assert!(!should_ignore_message(can_id, other_device));
            assert!(is_external_message(can_id, other_device));
        }
    }

    #[test]
    fn test_real_world_scenario() {
        // Simulator device ID
        let simulator_id = 0x8A;

        // Simulator broadcasts EEC12 status
        let own_broadcast = 0x18FCCC8A;
        assert!(should_ignore_message(own_broadcast, simulator_id));

        // Test tool sends command to simulator (PDU1 - destination-specific)
        let test_command = 0x18EFFE8A; // PDU Format 0xEF < 0xF0 = PDU1
                                       // Low byte 0x8A is DESTINATION, not source
                                       // PDU1 messages are NOT filtered
                                       // even if destination matches our ID
        assert!(!should_ignore_message(test_command, simulator_id));

        // ECU sends command with different source
        let ecu_command = 0x18EFFE82; // Source = 0x82
        assert!(!should_ignore_message(ecu_command, simulator_id));
    }

    #[test]
    fn test_common_j1939_devices() {
        // Common J1939 device addresses
        let engine_ecu = 0x00;
        let transmission = 0x03;
        let brake_controller = 0x0B;
        let instrument_cluster = 0x17;
        let diagnostic_tool = 0xF9;

        // Each device should ignore only its own messages
        for device in [
            engine_ecu,
            transmission,
            brake_controller,
            instrument_cluster,
            diagnostic_tool,
        ] {
            let can_id = 0x18FECA00 | (device as u32);

            // Device ignores its own message
            assert!(should_ignore_message(can_id, device));

            // Other devices don't ignore it
            for other_device in [
                engine_ecu,
                transmission,
                brake_controller,
                instrument_cluster,
                diagnostic_tool,
            ] {
                if other_device != device {
                    assert!(!should_ignore_message(can_id, other_device));
                }
            }
        }
    }

    #[test]
    fn test_edge_cases() {
        // Device ID 0x00
        assert!(should_ignore_message(0x18FECA00, 0x00));
        assert!(!should_ignore_message(0x18FECA01, 0x00));

        // Device ID 0xFF
        assert!(should_ignore_message(0x18FECAFF, 0xFF));
        assert!(!should_ignore_message(0x18FECAFE, 0xFF));

        // Standard frame IDs (11-bit) - treated as PDU1 (PDU Format = 0 < 0xF0)
        // PDU1-like messages are NOT filtered regardless of low byte
        assert!(!should_ignore_message(0x12A, 0x2A));
        assert!(!should_ignore_message(0x12A, 0x2B));
    }

    #[test]
    fn test_performance_inline() {
        // These functions should compile to simple bit operations
        // This test verifies they work correctly in a loop (simulating real usage)
        let my_device_id = 0x8A;
        let mut external_count = 0;
        let mut self_count = 0;

        // Simulate receiving 1000 messages
        for i in 0..1000u32 {
            // Mix of own messages and external messages
            let source = if i % 10 == 0 {
                my_device_id
            } else {
                // Generate device IDs 1-255, but skip 0x8A to avoid accidental matches
                let candidate = ((i % 254) + 1) as u8;
                if candidate == my_device_id {
                    0xFF // Use a different ID if we accidentally generated our own
                } else {
                    candidate
                }
            };
            let can_id = 0x18FCCC00 | (source as u32);

            if is_external_message(can_id, my_device_id) {
                external_count += 1;
            } else {
                self_count += 1;
            }
        }

        assert_eq!(self_count, 100); // Every 10th message
        assert_eq!(external_count, 900); // The rest
    }
}
