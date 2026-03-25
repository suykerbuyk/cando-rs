//! Device ID parsing utilities
//!
//! This module provides utilities for parsing device ID strings into DeviceId values.
//! Device IDs can be specified in either hexadecimal (with 0x prefix) or decimal format.

use anyhow::{Result, anyhow};
use cando_messages::common::DeviceId;

/// Parse a device ID string (hexadecimal or decimal) into a DeviceId
///
/// Accepts device IDs in two formats:
/// - Hexadecimal: "0x82" or "0X82" (case-insensitive prefix)
/// - Decimal: "130"
///
/// Device IDs must be in the range 0-255 (valid u8 range).
///
/// # Arguments
///
/// * `id_str` - Device ID string to parse
///
/// # Returns
///
/// * `Ok(DeviceId)` - Successfully parsed device ID
/// * `Err` - If parsing fails or value is out of range
///
/// # Examples
///
/// ```
/// use cando_core::device_id::parse_device_id;
///
/// # fn main() -> anyhow::Result<()> {
/// // Hexadecimal format
/// let device_id = parse_device_id("0x82")?;
/// assert_eq!(u8::from(device_id), 0x82);
///
/// // Decimal format
/// let device_id = parse_device_id("130")?;
/// assert_eq!(u8::from(device_id), 130);
///
/// // Out of range fails
/// assert!(parse_device_id("256").is_err());
/// # Ok(())
/// # }
/// ```
pub fn parse_device_id(id_str: &str) -> Result<DeviceId> {
    let id = if id_str.starts_with("0x") || id_str.starts_with("0X") {
        u32::from_str_radix(&id_str[2..], 16)
            .map_err(|e| anyhow!("Invalid hexadecimal device ID '{}': {}", id_str, e))?
    } else {
        id_str
            .parse::<u32>()
            .map_err(|e| anyhow!("Invalid decimal device ID '{}': {}", id_str, e))?
    };

    if id > 255 {
        return Err(anyhow!("Device ID must be between 0 and 255, got {}", id));
    }

    Ok(DeviceId::from(id as u8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_lowercase() {
        let result = parse_device_id("0x82");
        assert!(result.is_ok());
        assert_eq!(u8::from(result.unwrap()), 0x82);
    }

    #[test]
    fn test_parse_hex_uppercase_prefix() {
        let result = parse_device_id("0X8A");
        assert!(result.is_ok());
        assert_eq!(u8::from(result.unwrap()), 0x8A);
    }

    #[test]
    fn test_parse_hex_uppercase_digits() {
        let result = parse_device_id("0xAB");
        assert!(result.is_ok());
        assert_eq!(u8::from(result.unwrap()), 0xAB);
    }

    #[test]
    fn test_parse_decimal() {
        let result = parse_device_id("138");
        assert!(result.is_ok());
        assert_eq!(u8::from(result.unwrap()), 138);
    }

    #[test]
    fn test_parse_zero() {
        let result = parse_device_id("0");
        assert!(result.is_ok());
        assert_eq!(u8::from(result.unwrap()), 0);
    }

    #[test]
    fn test_parse_max_value() {
        let result = parse_device_id("255");
        assert!(result.is_ok());
        assert_eq!(u8::from(result.unwrap()), 255);

        let result_hex = parse_device_id("0xFF");
        assert!(result_hex.is_ok());
        assert_eq!(u8::from(result_hex.unwrap()), 0xFF);
    }

    #[test]
    fn test_parse_out_of_range() {
        let result = parse_device_id("256");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("between 0 and 255")
        );
    }

    #[test]
    fn test_parse_out_of_range_hex() {
        let result = parse_device_id("0x100");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("between 0 and 255")
        );
    }

    #[test]
    fn test_parse_invalid_hex() {
        let result = parse_device_id("0xGG");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid hex"));
    }

    #[test]
    fn test_parse_invalid_decimal() {
        let result = parse_device_id("abc");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid decimal"));
    }

    #[test]
    fn test_parse_empty_string() {
        let result = parse_device_id("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_negative() {
        let result = parse_device_id("-1");
        assert!(result.is_err());
    }

    #[test]
    fn test_known_device_ids() {
        // Test various device ID values
        let dev_a = parse_device_id("0x8A").unwrap();
        assert_eq!(u8::from(dev_a), 0x8A);

        let dev_b = parse_device_id("0x82").unwrap();
        assert_eq!(u8::from(dev_b), 0x82);

        let dev_c = parse_device_id("0x83").unwrap();
        assert_eq!(u8::from(dev_c), 0x83);

        let dev_d = parse_device_id("0x59").unwrap();
        assert_eq!(u8::from(dev_d), 0x59);
    }
}
