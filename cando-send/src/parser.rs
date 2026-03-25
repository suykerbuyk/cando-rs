//! Frame format parser for cando-send
//!
//! Parses CAN frame specifications in the format used by can-utils cansend:
//! - Standard CAN: `<can_id>#{data}`
//! - Examples: `123#DEADBEEF`, `5A1#11.22.33.44.55.66.77.88`

use anyhow::{Context, Result, anyhow};

/// A parsed CAN frame specification
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedFrame {
    /// CAN ID (11-bit or 29-bit)
    pub id: u32,
    /// Whether this is an extended frame (29-bit ID)
    pub extended: bool,
    /// Data bytes (0-8 for Classical CAN, 0-64 for CAN FD)
    pub data: Vec<u8>,
    /// Whether this is an RTR (Remote Transmission Request) frame
    pub is_rtr: bool,
    /// Optional DLC (Data Length Code) override
    pub dlc: Option<u8>,
    /// Whether this is a CAN FD frame
    pub is_canfd: bool,
    /// CAN FD flags byte (BRS, ESI bits)
    pub fd_flags: u8,
}

/// Parse a CAN frame specification string
///
/// Supported formats:
/// - Data frame: `<can_id>#{data}` (e.g., "123#DEADBEEF")
/// - RTR frame: `<can_id>#R{len}` (e.g., "123#R", "123#R3")
/// - Data with DLC: `<can_id>#{data}_{dlc}` (e.g., "123#11223344_B")
/// - RTR with DLC: `<can_id>#R{len}_{dlc}` (e.g., "123#R8_E")
/// - CAN FD frame: `<can_id>##<flags>{data}` (e.g., "123##1", "456##311223344")
///
/// ID formats:
/// - Standard ID: 3 hex digits (e.g., "123")
/// - Extended ID: 8 hex digits (e.g., "00000123")
///
/// # Examples
///
/// ```
/// use cando_send::parser::parse_frame;
///
/// // Standard CAN frame
/// let frame = parse_frame("123#DEADBEEF").unwrap();
/// assert_eq!(frame.id, 0x123);
/// assert!(!frame.extended);
/// assert_eq!(frame.data, vec![0xDE, 0xAD, 0xBE, 0xEF]);
///
/// // CAN FD frame
/// let frame = parse_frame("123##311223344").unwrap();
/// assert_eq!(frame.id, 0x123);
/// assert!(frame.is_canfd);
/// assert_eq!(frame.fd_flags, 3);
/// assert_eq!(frame.data, vec![0x11, 0x22, 0x33, 0x44]);
/// ```
pub fn parse_frame(input: &str) -> Result<ParsedFrame> {
    // Check for CAN FD format (## separator)
    let is_canfd = input.contains("##");

    // Split on separator (## for CAN FD, # for Classical CAN)
    let parts: Vec<&str> = if is_canfd {
        input.split("##").collect()
    } else {
        input.split('#').collect()
    };

    if parts.len() != 2 {
        let sep = if is_canfd { "##" } else { "#" };
        return Err(anyhow!(
            "Invalid frame format: expected '<can_id>{}<data>', got '{}'",
            sep,
            input
        ));
    }

    let id_str = parts[0];
    let data_str = parts[1];

    // Parse CAN ID
    let (id, extended) = parse_can_id(id_str)
        .with_context(|| format!("Failed to parse CAN ID from '{}'", id_str))?;

    // Parse based on frame type
    let (is_rtr, data, dlc, fd_flags) = if is_canfd {
        // CAN FD format: ##<flags>{data}
        let (flags, data) = parse_canfd_frame(data_str)?;
        (false, data, None, flags)
    } else {
        // Classical CAN format
        // Check for DLC override (format: {data}_{dlc})
        let (data_part, dlc) = parse_dlc_override(data_str)?;

        // Check for RTR frame (starts with 'R' or 'r')
        let (is_rtr, data) = if data_part.to_uppercase().starts_with('R') {
            parse_rtr_frame(data_part)?
        } else {
            // Parse regular data frame
            let data = parse_data_bytes(data_part)
                .with_context(|| format!("Failed to parse data bytes from '{}'", data_part))?;
            (false, data)
        };

        // Validate data length for Classical CAN
        if data.len() > 8 {
            return Err(anyhow!(
                "Data too long: {} bytes (max 8 for Classical CAN)",
                data.len()
            ));
        }

        (is_rtr, data, dlc, 0)
    };

    // Validate data length for CAN FD
    if is_canfd && data.len() > 64 {
        return Err(anyhow!(
            "Data too long: {} bytes (max 64 for CAN FD)",
            data.len()
        ));
    }

    // Validate DLC if specified
    if let Some(dlc_val) = dlc
        && dlc_val > 15 {
            return Err(anyhow!("DLC value out of range: {} (max 15)", dlc_val));
        }

    Ok(ParsedFrame {
        id,
        extended,
        data,
        is_rtr,
        dlc,
        is_canfd,
        fd_flags,
    })
}

/// Parse a CAN ID from a hex string
///
/// Returns (id, extended) where extended is true for 29-bit IDs
///
/// - 3 hex digits: Standard 11-bit ID (e.g., "123" -> 0x123, extended=false)
/// - 8 hex digits: Extended 29-bit ID (e.g., "00000123" -> 0x123, extended=true)
fn parse_can_id(id_str: &str) -> Result<(u32, bool)> {
    if id_str.is_empty() {
        return Err(anyhow!("CAN ID cannot be empty"));
    }

    // Determine if extended based on length
    let extended = match id_str.len() {
        3 => false, // Standard 11-bit ID
        8 => true,  // Extended 29-bit ID
        _ => {
            return Err(anyhow!(
                "Invalid CAN ID length: {} (expected 3 for standard or 8 for extended)",
                id_str.len()
            ));
        }
    };

    // Parse hex string to u32
    let id = u32::from_str_radix(id_str, 16)
        .with_context(|| format!("Invalid hex in CAN ID: '{}'", id_str))?;

    // Validate ID range
    if extended {
        // Extended: 29-bit (0x1FFFFFFF max)
        if id > 0x1FFF_FFFF {
            return Err(anyhow!(
                "Extended CAN ID out of range: 0x{:08X} (max 0x1FFFFFFF)",
                id
            ));
        }
    } else {
        // Standard: 11-bit (0x7FF max)
        if id > 0x7FF {
            return Err(anyhow!(
                "Standard CAN ID out of range: 0x{:03X} (max 0x7FF)",
                id
            ));
        }
    }

    Ok((id, extended))
}

/// Parse data bytes from a hex string
///
/// Supports formats:
/// - Continuous hex: "DEADBEEF" -> [0xDE, 0xAD, 0xBE, 0xEF]
/// - Dot-separated: "DE.AD.BE.EF" -> [0xDE, 0xAD, 0xBE, 0xEF]
/// - Mixed: "DEAD.BEEF" -> [0xDE, 0xAD, 0xBE, 0xEF]
/// - Empty: "" -> []
fn parse_data_bytes(data_str: &str) -> Result<Vec<u8>> {
    // Empty data is valid
    if data_str.is_empty() {
        return Ok(Vec::new());
    }

    // Remove all dots to get continuous hex string
    let hex_only: String = data_str.chars().filter(|c| *c != '.').collect();

    // Validate hex characters
    if !hex_only.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(anyhow!(
            "Invalid data: '{}' contains non-hex characters",
            data_str
        ));
    }

    // Must have even number of hex digits (2 per byte)
    if !hex_only.len().is_multiple_of(2) {
        return Err(anyhow!(
            "Invalid data: '{}' has odd number of hex digits ({})",
            data_str,
            hex_only.len()
        ));
    }

    // Parse pairs of hex digits into bytes
    let mut bytes = Vec::new();
    for chunk in hex_only.as_bytes().chunks(2) {
        let hex_byte =
            std::str::from_utf8(chunk).context("Internal error: invalid UTF-8 in hex string")?;
        let byte = u8::from_str_radix(hex_byte, 16)
            .with_context(|| format!("Failed to parse hex byte: '{}'", hex_byte))?;
        bytes.push(byte);
    }

    Ok(bytes)
}

/// Parse DLC override from data string
///
/// Format: {data}_{dlc} where dlc is a single hex digit (0-F)
/// Returns: (data_without_dlc, optional_dlc)
fn parse_dlc_override(data_str: &str) -> Result<(&str, Option<u8>)> {
    // Check if there's an underscore indicating DLC override
    if let Some(underscore_pos) = data_str.rfind('_') {
        let data_part = &data_str[..underscore_pos];
        let dlc_part = &data_str[underscore_pos + 1..];

        // DLC must be a single hex digit
        if dlc_part.len() != 1 {
            return Err(anyhow!(
                "Invalid DLC format: '{}' (expected single hex digit)",
                dlc_part
            ));
        }

        let dlc_char = dlc_part.chars().next().unwrap();
        if !dlc_char.is_ascii_hexdigit() {
            return Err(anyhow!(
                "Invalid DLC value: '{}' (not a hex digit)",
                dlc_part
            ));
        }

        let dlc = u8::from_str_radix(dlc_part, 16)
            .with_context(|| format!("Failed to parse DLC: '{}'", dlc_part))?;

        Ok((data_part, Some(dlc)))
    } else {
        // No DLC override
        Ok((data_str, None))
    }
}

/// Parse RTR frame specification
///
/// Formats:
/// - "R" or "r" - RTR with no length specified
/// - "R3" or "r3" - RTR with length=3
///
/// Returns: (is_rtr, data_bytes)
fn parse_rtr_frame(rtr_str: &str) -> Result<(bool, Vec<u8>)> {
    let upper = rtr_str.to_uppercase();

    if upper == "R" {
        // RTR with no length
        Ok((true, Vec::new()))
    } else if upper.starts_with('R') && upper.len() > 1 {
        // RTR with length specified
        let len_str = &upper[1..];

        // Parse the length as a decimal number
        let len = len_str.parse::<usize>().with_context(|| {
            format!(
                "Invalid RTR length: '{}' (expected decimal number)",
                len_str
            )
        })?;

        if len > 8 {
            return Err(anyhow!(
                "RTR length out of range: {} (max 8 for Classical CAN)",
                len
            ));
        }

        // Create a vector with the specified length (all zeros for RTR)
        let data = vec![0u8; len];
        Ok((true, data))
    } else {
        Err(anyhow!("Invalid RTR format: '{}'", rtr_str))
    }
}

/// Parse CAN FD frame specification
///
/// Format: <flags>{data}
/// - flags: single hex digit (0-F) for BRS/ESI flags
/// - data: hex bytes (up to 64 bytes for CAN FD)
///
/// Returns: (flags, data_bytes)
fn parse_canfd_frame(canfd_str: &str) -> Result<(u8, Vec<u8>)> {
    if canfd_str.is_empty() {
        return Err(anyhow!("CAN FD frame data cannot be empty"));
    }

    // First character should be the flags byte (single hex digit)
    let flags_char = canfd_str.chars().next().unwrap();
    if !flags_char.is_ascii_hexdigit() {
        return Err(anyhow!(
            "Invalid CAN FD flags: '{}' (expected hex digit)",
            flags_char
        ));
    }

    let flags = u8::from_str_radix(&flags_char.to_string(), 16)
        .with_context(|| format!("Failed to parse CAN FD flags: '{}'", flags_char))?;

    // Rest is data bytes
    let data_str = &canfd_str[1..];
    let data = if data_str.is_empty() {
        Vec::new()
    } else {
        parse_data_bytes(data_str)
            .with_context(|| format!("Failed to parse CAN FD data bytes from '{}'", data_str))?
    };

    Ok((flags, data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frame_standard() {
        let frame = parse_frame("123#DEADBEEF").unwrap();
        assert_eq!(frame.id, 0x123);
        assert!(!frame.extended);
        assert!(!frame.is_rtr);
        assert!(!frame.is_canfd);
        assert_eq!(frame.dlc, None);
        assert_eq!(frame.data, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_parse_frame_extended() {
        let frame = parse_frame("00000123#11.22.33.44").unwrap();
        assert_eq!(frame.id, 0x123);
        assert!(frame.extended);
        assert_eq!(frame.data, vec![0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn test_parse_rtr_no_length() {
        let frame = parse_frame("123#R").unwrap();
        assert!(frame.is_rtr);
        assert_eq!(frame.data.len(), 0);
    }

    #[test]
    fn test_parse_canfd_with_data() {
        let frame = parse_frame("213##311223344").unwrap();
        assert!(frame.is_canfd);
        assert_eq!(frame.fd_flags, 3);
        assert_eq!(frame.data, vec![0x11, 0x22, 0x33, 0x44]);
    }
}
