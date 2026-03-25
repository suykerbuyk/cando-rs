//! Message decoding and protocol identification.
//!
//! This module handles extracting device IDs from CAN IDs and identifying
//! the protocol and message type for incoming frames.

use crate::error::DecodeError;
use crate::types::{DecodedMessage, Protocol, SourcedFrame};
use socketcan::{EmbeddedFrame, Id};
use tracing::{debug, trace, warn};

/// Extract device ID from a CAN ID based on protocol conventions.
///
/// Intentional simplification: extracts the lower 8 bits (source address) without
/// PDU1/PDU2 distinction. This is correct for the monitor's use case of identifying
/// message sources for display grouping. For PDU-aware device ID extraction that
/// handles J1939 PDU1 destination addressing, see
/// [`cando_messages::encoder::extract_device_id`].
///
/// # Example
///
/// ```
/// use cando_can_monitor::extract_device_id;
///
/// // J1939 CAN ID with device 0x88
/// let device_id = extract_device_id(0x18FECA88);
/// assert_eq!(device_id, 0x88);
/// ```
pub fn extract_device_id(can_id: u32) -> u8 {
    (can_id & 0xFF) as u8
}

/// Identify the protocol from a CAN ID.
///
/// Uses CAN ID patterns to determine which protocol family a message belongs to.
///
/// # CAN ID Patterns
///
/// - **J1939**: Standard J1939 PGN-based IDs (0x18xxxxxx range)
///
/// # Example
///
/// ```
/// use cando_can_monitor::{identify_protocol, Protocol};
///
/// let protocol = identify_protocol(0x18FECA00);
/// assert_eq!(protocol, Protocol::J1939);
/// ```
pub fn identify_protocol(can_id: u32) -> Protocol {
    // J1939 messages (broad range, 0x18xxxxxx and similar extended ranges)
    // J1939 uses 29-bit extended CAN IDs with priority in upper bits
    if (can_id & 0xFF000000) == 0x18000000
        || (can_id & 0xFF000000) == 0x19000000
        || (can_id & 0xFF000000) == 0x1C000000
        || (can_id & 0xFF000000) == 0x1D000000
    {
        return Protocol::J1939;
    }

    Protocol::Unknown
}

/// Identify the specific message type name from a CAN ID.
///
/// Returns a human-readable message type name (e.g., "DM01", "EEC1").
///
/// # Arguments
///
/// * `can_id` - The CAN ID to identify
/// * `protocol` - The protocol family (from `identify_protocol`)
///
/// # Returns
///
/// A message type name string, or "Unknown" if not recognized.
pub fn identify_message_type(can_id: u32, protocol: Protocol) -> String {
    match protocol {
        Protocol::J1939 => {
            // Extract PGN from CAN ID (J1939 standard)
            let pgn = if (can_id & 0x00FF0000) < 0x00F00000 {
                // PDU1 format (destination specific)
                (can_id >> 8) & 0x03FF00
            } else {
                // PDU2 format (broadcast)
                (can_id >> 8) & 0x03FFFF
            };

            match pgn {
                0xFECA => "DM01".to_string(), // Active DTCs
                0xFECB => "DM02".to_string(), // Previously active DTCs
                0xFECC => "DM03".to_string(), // Diagnostic data clear/reset
                0xFEF5 => "AMB".to_string(),  // Ambient Conditions
                0xF004 => "EEC1".to_string(), // Electronic Engine Controller 1
                0xF003 => "EEC2".to_string(), // Electronic Engine Controller 2
                0xFEF1 => "CCVS".to_string(), // Cruise Control/Vehicle Speed
                _ => format!("J1939_PGN_0x{:04X}", pgn),
            }
        }

        Protocol::Unknown => format!("UNKNOWN_0x{:08X}", can_id),
    }
}

/// Decode a sourced CAN frame into a structured message.
///
/// Extracts device ID, identifies protocol and message type, and packages
/// everything into a `DecodedMessage` for further processing.
///
/// # Arguments
///
/// * `sourced_frame` - Frame with source information
///
/// # Returns
///
/// * `Ok(DecodedMessage)` - Successfully decoded message
/// * `Err(DecodeError)` - Decoding failed (unknown protocol, invalid data, etc.)
///
/// # Example
///
/// ```no_run
/// use cando_can_monitor::{decode_message, SourcedFrame};
/// use socketcan::{CanFrame, EmbeddedFrame, ExtendedId, Id};
///
/// let can_id = Id::Extended(ExtendedId::new(0x18FECA88).unwrap());
/// let frame = CanFrame::new(can_id, &[0xA7, 0x2E, 0x00, 0x7D]).unwrap();
/// let sourced = SourcedFrame::new(frame, "can0".to_string());
///
/// match decode_message(&sourced) {
///     Ok(msg) => println!("Device 0x{:02X}: {}", msg.device_id, msg.message_name),
///     Err(e) => eprintln!("Decode error: {}", e),
/// }
/// ```
pub fn decode_message(sourced_frame: &SourcedFrame) -> Result<DecodedMessage, DecodeError> {
    let frame = &sourced_frame.frame;

    // Extract CAN ID
    let can_id = match frame.id() {
        Id::Standard(id) => id.as_raw() as u32,
        Id::Extended(id) => id.as_raw(),
    };

    trace!(
        "Decoding CAN ID 0x{:08X} from {}",
        can_id,
        sourced_frame.source
    );

    // Identify protocol
    let protocol = identify_protocol(can_id);

    // Log unknown protocols at debug level (not error - many valid messages may be unknown)
    if protocol == Protocol::Unknown {
        debug!(
            "Unknown protocol for CAN ID 0x{:08X} from {}",
            can_id, sourced_frame.source
        );
    }

    // Extract device ID
    let device_id = extract_device_id(can_id);

    // Identify message type
    let message_name = identify_message_type(can_id, protocol);

    // Extract frame data
    let data = frame.data().to_vec();

    // Validate data length (basic sanity check)
    if data.is_empty() {
        warn!(
            "Empty frame data for {} message 0x{:08X}",
            protocol.name(),
            can_id
        );
    }

    // Create decoded message
    let decoded = DecodedMessage::new(
        device_id,
        protocol,
        message_name,
        can_id,
        data,
        sourced_frame.timestamp,
        sourced_frame.source.clone(),
        *frame,
    );

    trace!(
        "Decoded: Device 0x{:02X}, Protocol {}, Message {}",
        decoded.device_id,
        decoded.protocol.name(),
        decoded.message_name
    );

    Ok(decoded)
}

/// Batch decode multiple sourced frames.
///
/// Decodes frames and returns both successful results and errors.
/// This allows processing to continue even if some frames fail to decode.
///
/// # Returns
///
/// A tuple of `(successful_messages, errors)`.
#[allow(dead_code)]
pub fn decode_batch(
    frames: &[SourcedFrame],
) -> (Vec<DecodedMessage>, Vec<(SourcedFrame, DecodeError)>) {
    let mut messages = Vec::new();
    let mut errors = Vec::new();

    for frame in frames {
        match decode_message(frame) {
            Ok(msg) => messages.push(msg),
            Err(e) => errors.push((frame.clone(), e)),
        }
    }

    (messages, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use socketcan::{CanFrame, EmbeddedFrame, ExtendedId, Id, StandardId};

    #[test]
    fn test_extract_device_id() {
        assert_eq!(extract_device_id(0x18FECA88), 0x88);
        assert_eq!(extract_device_id(0x18FECA8A), 0x8A);
        assert_eq!(extract_device_id(0x18F37082), 0x82);
        assert_eq!(extract_device_id(0x18F311FF), 0xFF);
        assert_eq!(extract_device_id(0x18F31300), 0x00);
    }

    #[test]
    fn test_identify_protocol_j1939() {
        // DM01 (0xFECA)
        assert_eq!(identify_protocol(0x18FECA00), Protocol::J1939);

        // EEC1 (0xF004)
        assert_eq!(identify_protocol(0x18F00400), Protocol::J1939);

        // Generic J1939 messages
        assert_eq!(identify_protocol(0x18AA5500), Protocol::J1939);

        // J1939 with different priority bits
        assert_eq!(identify_protocol(0x19FF48FF), Protocol::J1939);
        assert_eq!(identify_protocol(0x1CFF4FFF), Protocol::J1939);
    }

    #[test]
    fn test_identify_protocol_unknown() {
        assert_eq!(identify_protocol(0x12345678), Protocol::Unknown);
        assert_eq!(identify_protocol(0x00000000), Protocol::Unknown);
    }

    #[test]
    fn test_identify_message_type_j1939() {
        assert_eq!(identify_message_type(0x18FECA00, Protocol::J1939), "DM01");
        assert_eq!(identify_message_type(0x18FECB00, Protocol::J1939), "DM02");
        assert_eq!(identify_message_type(0x18FECC00, Protocol::J1939), "DM03");
        assert_eq!(identify_message_type(0x18F00400, Protocol::J1939), "EEC1");
    }

    #[test]
    fn test_decode_message_j1939() {
        let can_id = Id::Extended(ExtendedId::new(0x18FECA00).unwrap());
        let frame = CanFrame::new(can_id, &[0xFF, 0xFF, 0xFF, 0xFF]).unwrap();
        let sourced = SourcedFrame::new(frame, "vcan0".to_string());

        let decoded = decode_message(&sourced).expect("Should decode successfully");

        assert_eq!(decoded.device_id, 0x00);
        assert_eq!(decoded.protocol, Protocol::J1939);
        assert_eq!(decoded.message_name, "DM01");
        assert_eq!(decoded.can_id, 0x18FECA00);
    }

    #[test]
    fn test_decode_message_unknown_protocol() {
        let can_id = Id::Extended(ExtendedId::new(0x12345678).unwrap());
        let frame = CanFrame::new(can_id, &[1, 2, 3]).unwrap();
        let sourced = SourcedFrame::new(frame, "can0".to_string());

        let decoded = decode_message(&sourced).expect("Should decode (as unknown)");

        assert_eq!(decoded.protocol, Protocol::Unknown);
        assert!(decoded.message_name.contains("UNKNOWN"));
    }

    #[test]
    fn test_decode_batch() {
        let frames = vec![
            SourcedFrame::new(
                CanFrame::new(
                    Id::Extended(ExtendedId::new(0x18FECA88).unwrap()),
                    &[0x8C, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xF8],
                )
                .unwrap(),
                "can0".to_string(),
            ),
            SourcedFrame::new(
                CanFrame::new(
                    Id::Extended(ExtendedId::new(0x18F00400).unwrap()),
                    &[0xA7, 0x2E, 0x00, 0x7D],
                )
                .unwrap(),
                "can0".to_string(),
            ),
            SourcedFrame::new(
                CanFrame::new(
                    Id::Extended(ExtendedId::new(0x18FECA00).unwrap()),
                    &[0xFF, 0xFF],
                )
                .unwrap(),
                "vcan0".to_string(),
            ),
        ];

        let (messages, errors) = decode_batch(&frames);

        assert_eq!(messages.len(), 3);
        assert_eq!(errors.len(), 0);
        assert_eq!(messages[0].message_name, "DM01");
        assert_eq!(messages[1].message_name, "EEC1");
        assert_eq!(messages[2].message_name, "DM01");
    }

    #[test]
    fn test_decode_standard_can_id() {
        // Standard (11-bit) CAN IDs should also work
        let can_id = Id::Standard(StandardId::new(0x123).unwrap());
        let frame = CanFrame::new(can_id, &[1, 2, 3]).unwrap();
        let sourced = SourcedFrame::new(frame, "can0".to_string());

        let decoded = decode_message(&sourced).expect("Should decode");
        assert_eq!(decoded.can_id, 0x123);
        assert!(decoded.is_standard_id());
    }

    #[test]
    fn test_empty_frame_data() {
        let can_id = Id::Extended(ExtendedId::new(0x18FECA88).unwrap());
        let frame = CanFrame::new(can_id, &[]).unwrap();
        let sourced = SourcedFrame::new(frame, "can0".to_string());

        // Should still decode, just with warning logged
        let decoded = decode_message(&sourced).expect("Should decode empty frame");
        assert_eq!(decoded.data.len(), 0);
    }
}
