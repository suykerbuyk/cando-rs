//! CAN frame creation utilities for simulators.
//!
//! This module provides simplified CAN frame creation with automatic
//! standard/extended ID detection, eliminating duplicate code across simulators.
//!
//! # Examples
//!
//! ```rust
//! use cando_simulator_common::can_frame::{create_can_frame, FrameType};
//!
//! // Auto-detect frame type based on CAN ID
//! let frame = create_can_frame(0x123, &[1, 2, 3], FrameType::Auto)?;
//!
//! // Force extended frame (J1939)
//! let frame = create_can_frame(0x18EEFF8A, &[1, 2, 3, 4, 5, 6, 7, 8], FrameType::Extended)?;
//!
//! // Force standard frame
//! let frame = create_can_frame(0x456, &[0xAA, 0xBB], FrameType::Standard)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{anyhow, Result};
use socketcan::{CanFrame, EmbeddedFrame, ExtendedId, StandardId};

/// CAN frame type for ID selection.
///
/// Determines whether to use standard (11-bit) or extended (29-bit) CAN IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// Standard 11-bit CAN frames (CAN 2.0A).
    ///
    /// Valid ID range: 0x000 to 0x7FF (0 to 2047).
    Standard,

    /// Extended 29-bit CAN frames (CAN 2.0B).
    ///
    /// Valid ID range: 0x00000000 to 0x1FFFFFFF (0 to 536,870,911).
    /// Required for J1939 and other extended protocols.
    Extended,

    /// Automatic detection based on CAN ID value.
    ///
    /// - IDs <= 0x7FF use standard frames
    /// - IDs > 0x7FF use extended frames
    ///
    /// This is the recommended default for most use cases.
    Auto,
}

/// Create a CAN frame with configurable ID type.
///
/// This function handles the complexity of CAN ID validation and frame creation,
/// supporting both standard (11-bit) and extended (29-bit) identifiers.
///
/// # Arguments
///
/// * `can_id` - The CAN identifier
/// * `data` - The data payload (0-8 bytes)
/// * `frame_type` - Whether to use standard, extended, or auto-detect
///
/// # Returns
///
/// * `Ok(CanFrame)` - Successfully created CAN frame
/// * `Err` - Invalid CAN ID or data length
///
/// # Errors
///
/// Returns an error if:
/// - CAN ID is invalid for the selected frame type
/// - Data length exceeds 8 bytes
/// - Frame creation fails for any reason
///
/// # Examples
///
/// ## Standard Frame (11-bit)
///
/// ```rust
/// use cando_simulator_common::can_frame::{create_can_frame, FrameType};
///
/// let frame = create_can_frame(0x123, &[0x11, 0x22], FrameType::Standard)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// ## Extended Frame (29-bit, J1939)
///
/// ```rust
/// use cando_simulator_common::can_frame::{create_can_frame, FrameType};
///
/// // J1939 PGN 65226 (0xFECA), priority 6, source 0x8A
/// let j1939_id = 0x18FECA8A;
/// let frame = create_can_frame(j1939_id, &[0xFF; 8], FrameType::Extended)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// ## Auto-Detection
///
/// ```rust
/// use cando_simulator_common::can_frame::{create_can_frame, FrameType};
///
/// // Automatically selects standard frame (ID <= 0x7FF)
/// let std_frame = create_can_frame(0x456, &[1, 2, 3], FrameType::Auto)?;
///
/// // Automatically selects extended frame (ID > 0x7FF)
/// let ext_frame = create_can_frame(0x18EEFF8A, &[1, 2, 3, 4], FrameType::Auto)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn create_can_frame(can_id: u32, data: &[u8], frame_type: FrameType) -> Result<CanFrame> {
    use FrameType::*;

    // Validate data length
    if data.len() > 8 {
        return Err(anyhow!(
            "CAN data length must be <= 8 bytes, got {}",
            data.len()
        ));
    }

    // Create appropriate ID type based on frame_type
    let id = match frame_type {
        Standard => {
            // Mask to 11-bit range
            let masked_id = can_id & 0x7FF;
            if can_id != masked_id {
                return Err(anyhow!(
                    "CAN ID 0x{:X} exceeds 11-bit standard frame range (max 0x7FF)",
                    can_id
                ));
            }
            let std_id = StandardId::new(masked_id as u16)
                .ok_or_else(|| anyhow!("Invalid standard CAN ID: 0x{:X}", masked_id))?;
            socketcan::Id::Standard(std_id)
        }

        Extended => {
            // Mask to 29-bit range
            let masked_id = can_id & 0x1FFFFFFF;
            let ext_id = ExtendedId::new(masked_id)
                .ok_or_else(|| anyhow!("Invalid extended CAN ID: 0x{:X}", masked_id))?;
            socketcan::Id::Extended(ext_id)
        }

        Auto => {
            // Auto-detect based on ID value
            if can_id <= 0x7FF {
                // Standard frame
                let std_id = StandardId::new(can_id as u16)
                    .ok_or_else(|| anyhow!("Invalid standard CAN ID: 0x{:X}", can_id))?;
                socketcan::Id::Standard(std_id)
            } else {
                // Extended frame
                let masked_id = can_id & 0x1FFFFFFF;
                let ext_id = ExtendedId::new(masked_id)
                    .ok_or_else(|| anyhow!("Invalid extended CAN ID: 0x{:X}", masked_id))?;
                socketcan::Id::Extended(ext_id)
            }
        }
    };

    // Create the frame
    CanFrame::new(id, data).ok_or_else(|| anyhow!("Failed to create CAN frame"))
}

/// Create a J1939 CAN frame (always 29-bit extended).
///
/// This is a convenience wrapper that ensures J1939 frames are always
/// created with extended IDs, as required by the J1939 standard.
///
/// # Arguments
///
/// * `can_id` - The 29-bit J1939 CAN identifier
/// * `data` - The 8-byte data payload (J1939 always uses 8 bytes)
///
/// # Examples
///
/// ```rust
/// use cando_simulator_common::can_frame::create_j1939_frame;
///
/// // J1939 address claimed message
/// let frame = create_j1939_frame(0x18EEFF8A, &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn create_j1939_frame(can_id: u32, data: &[u8; 8]) -> Result<CanFrame> {
    create_can_frame(can_id, data, FrameType::Extended)
}

/// Create a standard CAN frame (11-bit).
///
/// This is a convenience wrapper for protocols that exclusively use
/// standard 11-bit CAN IDs.
///
/// # Arguments
///
/// * `can_id` - The 11-bit CAN identifier (must be <= 0x7FF)
/// * `data` - The data payload (0-8 bytes)
///
/// # Examples
///
/// ```rust
/// use cando_simulator_common::can_frame::create_standard_frame;
///
/// // Standard CAN message (11-bit ID)
/// let frame = create_standard_frame(0x123, &[1, 2, 3])?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn create_standard_frame(can_id: u32, data: &[u8]) -> Result<CanFrame> {
    create_can_frame(can_id, data, FrameType::Standard)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_frame_creation() {
        // Valid standard ID
        let frame = create_can_frame(0x123, &[1, 2, 3], FrameType::Standard).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Standard(_)));

        // Maximum standard ID
        let frame = create_can_frame(0x7FF, &[1, 2, 3], FrameType::Standard).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Standard(_)));
    }

    #[test]
    fn test_standard_frame_out_of_range() {
        // ID too large for standard frame
        let result = create_can_frame(0x800, &[1, 2, 3], FrameType::Standard);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("11-bit"));
    }

    #[test]
    fn test_extended_frame_creation() {
        // Valid extended ID
        let frame =
            create_can_frame(0x18EEFF8A, &[1, 2, 3, 4, 5, 6, 7, 8], FrameType::Extended).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Extended(_)));

        // Maximum extended ID (29-bit)
        let frame = create_can_frame(0x1FFFFFFF, &[0xFF; 8], FrameType::Extended).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Extended(_)));
    }

    #[test]
    fn test_extended_frame_masking() {
        // ID with bits beyond 29-bit should be masked
        let frame = create_can_frame(0xFFFFFFFF, &[1, 2, 3], FrameType::Extended).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Extended(_)));

        // Verify masking to 29 bits
        if let socketcan::Id::Extended(ext_id) = frame.id() {
            assert_eq!(ext_id.as_raw(), 0x1FFFFFFF);
        }
    }

    #[test]
    fn test_auto_detection_standard() {
        // IDs <= 0x7FF should use standard frame
        let frame = create_can_frame(0x123, &[1, 2, 3], FrameType::Auto).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Standard(_)));

        let frame = create_can_frame(0x7FF, &[1, 2, 3], FrameType::Auto).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Standard(_)));
    }

    #[test]
    fn test_auto_detection_extended() {
        // IDs > 0x7FF should use extended frame
        let frame = create_can_frame(0x800, &[1, 2, 3], FrameType::Auto).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Extended(_)));

        let frame =
            create_can_frame(0x18EEFF8A, &[1, 2, 3, 4, 5, 6, 7, 8], FrameType::Auto).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Extended(_)));
    }

    #[test]
    fn test_data_length_validation() {
        // Valid lengths (0-8)
        for len in 0..=8 {
            let data = vec![0u8; len];
            let result = create_can_frame(0x123, &data, FrameType::Auto);
            assert!(result.is_ok(), "Length {} should be valid", len);
        }

        // Invalid length (9+)
        let data = vec![0u8; 9];
        let result = create_can_frame(0x123, &data, FrameType::Auto);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("8 bytes"));
    }

    #[test]
    fn test_j1939_frame_helper() {
        // J1939 frame should always be extended
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let frame = create_j1939_frame(0x18EEFF8A, &data).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Extended(_)));
    }

    #[test]
    fn test_standard_frame_helper() {
        // Standard helper should work for valid IDs
        let frame = create_standard_frame(0x456, &[1, 2, 3]).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Standard(_)));

        // Should reject IDs > 0x7FF
        let result = create_standard_frame(0x800, &[1, 2, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_data() {
        // Empty data should be valid
        let frame = create_can_frame(0x123, &[], FrameType::Auto).unwrap();
        assert_eq!(frame.data().len(), 0);
    }

    #[test]
    fn test_real_world_ids() {
        // J1939-style extended CAN IDs
        let frame = create_can_frame(0x18EFFE8A, &[1, 2, 3], FrameType::Auto).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Extended(_)));

        let frame = create_can_frame(0x18EFEF8A, &[1, 2, 3, 4, 5], FrameType::Auto).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Extended(_)));

        // J1939 DM1 (extended)
        let frame = create_j1939_frame(0x18FECA8A, &[0xFF; 8]).unwrap();
        assert!(matches!(frame.id(), socketcan::Id::Extended(_)));
    }
}
