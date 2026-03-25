//! CAN frame construction for cando-send
//!
//! Converts parsed frame specifications into socketcan frames ready for transmission.

use crate::parser::ParsedFrame;
use anyhow::Result;
use socketcan::frame::FdFlags;
use socketcan::{CanFdFrame, CanFrame, EmbeddedFrame, ExtendedId, Frame, Id, StandardId};

/// A frame that can be either Classical CAN or CAN FD
pub enum BuiltFrame {
    /// Classical CAN frame (including RTR)
    Can(CanFrame),
    /// CAN FD frame
    Fd(CanFdFrame),
}

#[allow(dead_code)]
impl BuiltFrame {
    /// Check if this is a CAN FD frame
    pub fn is_canfd(&self) -> bool {
        matches!(self, BuiltFrame::Fd(_))
    }

    /// Check if this is a remote frame
    pub fn is_remote_frame(&self) -> bool {
        match self {
            BuiltFrame::Can(frame) => frame.is_remote_frame(),
            BuiltFrame::Fd(_) => false,
        }
    }

    /// Get the ID of the frame
    pub fn id(&self) -> Id {
        match self {
            BuiltFrame::Can(frame) => frame.id(),
            BuiltFrame::Fd(frame) => frame.id(),
        }
    }

    /// Get the data bytes
    pub fn data(&self) -> &[u8] {
        match self {
            BuiltFrame::Can(frame) => frame.data(),
            BuiltFrame::Fd(frame) => frame.data(),
        }
    }

    /// Get the DLC
    pub fn dlc(&self) -> usize {
        match self {
            BuiltFrame::Can(frame) => frame.dlc(),
            BuiltFrame::Fd(frame) => frame.len(),
        }
    }
}

/// Build a socketcan frame from a ParsedFrame
///
/// # Arguments
///
/// * `parsed` - The parsed frame specification
///
/// # Returns
///
/// A BuiltFrame (either CanFrame or CanFdFrame) ready for transmission
///
/// # Examples
///
/// ```
/// use cando_send::parser::parse_frame;
/// use cando_send::frame::build_frame;
///
/// let parsed = parse_frame("123#DEADBEEF").unwrap();
/// let frame = build_frame(&parsed).unwrap();
/// ```
pub fn build_frame(parsed: &ParsedFrame) -> Result<BuiltFrame> {
    // CAN FD frames are handled separately
    if parsed.is_canfd {
        return build_canfd_frame(parsed).map(BuiltFrame::Fd);
    }

    // Validate data length for Classical CAN (0-8 bytes)
    if parsed.data.len() > 8 {
        return Err(anyhow::anyhow!(
            "Data length {} exceeds maximum of 8 bytes for Classical CAN",
            parsed.data.len()
        ));
    }

    // Create the ID
    let id = if parsed.extended {
        let ext_id = ExtendedId::new(parsed.id)
            .ok_or_else(|| anyhow::anyhow!("Invalid extended CAN ID: 0x{:08X}", parsed.id))?;
        Id::Extended(ext_id)
    } else {
        let std_id = StandardId::new(parsed.id as u16)
            .ok_or_else(|| anyhow::anyhow!("Invalid standard CAN ID: 0x{:03X}", parsed.id))?;
        Id::Standard(std_id)
    };

    // Build the frame based on whether it's RTR or data
    let frame = if parsed.is_rtr {
        // RTR frame - use the data length as DLC
        let dlc = parsed.data.len();
        CanFrame::new_remote(id, dlc)
            .ok_or_else(|| anyhow::anyhow!("Failed to create RTR frame with DLC {}", dlc))?
    } else {
        // Data frame
        CanFrame::new(id, &parsed.data)
            .ok_or_else(|| anyhow::anyhow!("Failed to create CAN data frame"))?
    };

    // Note: DLC override is parsed but not applied due to socketcan API limitations
    // The frame will use the natural DLC based on data length
    if parsed.dlc.is_some() {
        eprintln!("Warning: DLC override requested but not supported by socketcan 3.5 API");
    }

    Ok(BuiltFrame::Can(frame))
}

/// Build a CAN FD frame from a ParsedFrame
fn build_canfd_frame(parsed: &ParsedFrame) -> Result<CanFdFrame> {
    // Validate data length for CAN FD (0-64 bytes)
    if parsed.data.len() > 64 {
        return Err(anyhow::anyhow!(
            "Data length {} exceeds maximum of 64 bytes for CAN FD",
            parsed.data.len()
        ));
    }

    // Create the ID
    let id = if parsed.extended {
        let ext_id = ExtendedId::new(parsed.id)
            .ok_or_else(|| anyhow::anyhow!("Invalid extended CAN ID: 0x{:08X}", parsed.id))?;
        Id::Extended(ext_id)
    } else {
        let std_id = StandardId::new(parsed.id as u16)
            .ok_or_else(|| anyhow::anyhow!("Invalid standard CAN ID: 0x{:03X}", parsed.id))?;
        Id::Standard(std_id)
    };

    // Create FdFlags from the flags byte
    let fd_flags = FdFlags::from_bits_truncate(parsed.fd_flags);

    // Build the CAN FD frame
    let fd_frame = CanFdFrame::with_flags(id, &parsed.data, fd_flags)
        .ok_or_else(|| anyhow::anyhow!("Failed to create CAN FD frame"))?;

    Ok(fd_frame)
}

/// Get a human-readable description of a frame
///
/// Useful for logging and debugging
#[allow(dead_code)]
pub fn describe_frame(frame: &BuiltFrame) -> String {
    match frame {
        BuiltFrame::Can(can_frame) => describe_can_frame(can_frame),
        BuiltFrame::Fd(fd_frame) => describe_fd_frame(fd_frame),
    }
}

/// Describe a classical CAN frame
fn describe_can_frame(frame: &CanFrame) -> String {
    let is_extended = matches!(frame.id(), Id::Extended(_));
    let raw_id = match frame.id() {
        Id::Standard(id) => id.as_raw() as u32,
        Id::Extended(id) => id.as_raw(),
    };

    let id_str = if is_extended {
        format!("{:08X}", raw_id)
    } else {
        format!("{:03X}", raw_id)
    };

    let is_rtr = frame.is_remote_frame();
    let dlc = frame.dlc();
    let data = frame.data();

    let data_str = if is_rtr {
        format!("RTR [{}]", dlc)
    } else if data.is_empty() {
        String::new()
    } else {
        data.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    };

    let frame_type = if is_extended { "EXT" } else { "STD" };

    format!("{} {} [{}] {}", frame_type, id_str, dlc, data_str)
}

/// Describe a CAN FD frame
fn describe_fd_frame(frame: &CanFdFrame) -> String {
    let is_extended = matches!(frame.id(), Id::Extended(_));
    let raw_id = match frame.id() {
        Id::Standard(id) => id.as_raw() as u32,
        Id::Extended(id) => id.as_raw(),
    };

    let id_str = if is_extended {
        format!("{:08X}", raw_id)
    } else {
        format!("{:03X}", raw_id)
    };

    let dlc = frame.len();
    let data = frame.data();
    let data_str = if data.is_empty() {
        String::new()
    } else {
        data.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    };

    format!("FD {} [{}] {}", id_str, dlc, data_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_frame;

    // ===== Frame Construction Tests =====

    #[test]
    fn test_build_standard_frame() {
        let parsed = parse_frame("123#DEADBEEF").unwrap();
        let frame = build_frame(&parsed).unwrap();

        assert!(!frame.is_canfd());
        assert!(matches!(frame.id(), Id::Standard(_)));
        let raw_id = match frame.id() {
            Id::Standard(id) => id.as_raw() as u32,
            Id::Extended(id) => id.as_raw(),
        };
        assert_eq!(raw_id, 0x123);
        assert_eq!(frame.data(), &[0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_build_extended_frame() {
        let parsed = parse_frame("00000123#11223344").unwrap();
        let frame = build_frame(&parsed).unwrap();

        assert!(matches!(frame.id(), Id::Extended(_)));
        let raw_id = match frame.id() {
            Id::Standard(id) => id.as_raw() as u32,
            Id::Extended(id) => id.as_raw(),
        };
        assert_eq!(raw_id, 0x123);
        assert_eq!(frame.data(), &[0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn test_build_frame_no_data() {
        let parsed = parse_frame("5AA#").unwrap();
        let frame = build_frame(&parsed).unwrap();

        assert!(!frame.is_canfd());
        assert!(matches!(frame.id(), Id::Standard(_)));
        let raw_id = match frame.id() {
            Id::Standard(id) => id.as_raw() as u32,
            Id::Extended(id) => id.as_raw(),
        };
        assert_eq!(raw_id, 0x5AA);
        assert!(frame.data().is_empty());
    }

    #[test]
    fn test_build_rtr_frame_no_length() {
        let parsed = parse_frame("123#R").unwrap();
        let frame = build_frame(&parsed).unwrap();

        assert!(!frame.is_canfd());
        assert!(matches!(frame.id(), Id::Standard(_)));
        assert!(frame.is_remote_frame());
        assert_eq!(frame.dlc(), 0);
    }

    #[test]
    fn test_build_rtr_frame_with_length() {
        let parsed = parse_frame("123#R5").unwrap();
        let frame = build_frame(&parsed).unwrap();

        assert!(frame.is_remote_frame());
        assert_eq!(frame.dlc(), 5);
    }

    #[test]
    fn test_build_canfd_frame_with_data() {
        let parsed = parse_frame("213##311223344").unwrap();
        let frame = build_frame(&parsed).unwrap();

        assert!(frame.is_canfd());
        assert_eq!(frame.data(), &[0x11, 0x22, 0x33, 0x44]);
    }
}
