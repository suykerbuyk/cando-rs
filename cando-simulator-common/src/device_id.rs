//! Device ID parsing and validation for simulators.
//!
//! This module provides unified device ID parsing with protocol-specific validation,
//! eliminating duplicate parsing code across all simulators.
//!
//! # Examples
//!
//! ```rust
//! use cando_simulator_common::device_id::{parse_device_id, J1939DeviceIdValidator};
//!
//! // Parse hex device ID
//! let device_id = parse_device_id("0x8B", &J1939DeviceIdValidator)?;
//! assert_eq!(device_id, 0x8B);
//!
//! // Parse decimal device ID
//! let device_id = parse_device_id("139", &J1939DeviceIdValidator)?;
//! assert_eq!(device_id, 0x8B);
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::{anyhow, Result};

/// Protocol-specific device ID validator trait.
///
/// Each protocol implements this trait to define valid device IDs and
/// provide user-friendly error messages.
///
/// # Examples
///
/// ```rust
/// use cando_simulator_common::device_id::DeviceIdValidator;
/// use anyhow::{anyhow, Result};
///
/// struct MyProtocolValidator;
///
/// impl DeviceIdValidator for MyProtocolValidator {
///     fn validate(&self, id: u32) -> Result<()> {
///         if id >= 0x10 && id <= 0x1F {
///             Ok(())
///         } else {
///             Err(anyhow!("Device ID must be between 0x10 and 0x1F"))
///         }
///     }
///
///     fn valid_range_description(&self) -> &'static str {
///         "0x10 through 0x1F"
///     }
/// }
/// ```
pub trait DeviceIdValidator {
    /// Validate a device ID for this protocol.
    ///
    /// Returns `Ok(())` if the device ID is valid for this protocol,
    /// or an error with a descriptive message if invalid.
    fn validate(&self, device_id: u32) -> Result<()>;

    /// Get a human-readable description of valid device IDs.
    ///
    /// This is used in error messages to help users understand what
    /// device IDs are acceptable.
    fn valid_range_description(&self) -> &'static str;
}

/// Parse a device ID string (hex or decimal) with protocol-specific validation.
///
/// This function handles both hexadecimal (with "0x" prefix) and decimal
/// device ID strings, then validates the result using the provided validator.
///
/// # Arguments
///
/// * `id_str` - Device ID as string (e.g., "0x8A", "138", "0X82")
/// * `validator` - Protocol-specific validator implementing `DeviceIdValidator`
///
/// # Returns
///
/// * `Ok(u32)` - The parsed and validated device ID
/// * `Err` - If parsing fails or validation fails
///
/// # Errors
///
/// Returns an error if:
/// - The string cannot be parsed as a number
/// - The parsed value fails protocol-specific validation
///
/// # Examples
///
/// ```rust
/// use cando_simulator_common::device_id::{parse_device_id, J1939DeviceIdValidator};
///
/// // Parse hex format
/// let id = parse_device_id("0x8B", &J1939DeviceIdValidator)?;
/// assert_eq!(id, 0x8B);
///
/// // Parse decimal format
/// let id = parse_device_id("139", &J1939DeviceIdValidator)?;
/// assert_eq!(id, 0x8B);
///
/// // Invalid device ID (out of range)
/// let result = parse_device_id("0x100", &J1939DeviceIdValidator);
/// assert!(result.is_err());
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn parse_device_id<V: DeviceIdValidator>(id_str: &str, validator: &V) -> Result<u32> {
    // Parse hex or decimal
    let id = if id_str.starts_with("0x") || id_str.starts_with("0X") {
        u32::from_str_radix(&id_str[2..], 16).map_err(|e| {
            anyhow!(
                "Invalid hex device ID '{}': {}. Expected format: 0xXX (e.g., 0x8A)",
                id_str,
                e
            )
        })?
    } else {
        id_str.parse::<u32>().map_err(|e| {
            anyhow!(
                "Invalid device ID '{}': {}. Expected decimal number or hex (0xXX)",
                id_str,
                e
            )
        })?
    };

    // Validate using protocol-specific rules
    validator.validate(id).map_err(|e| {
        anyhow!(
            "Invalid device ID 0x{:02X} (decimal {}) for this protocol.\n  Reason: {}\n  Valid range: {}",
            id,
            id,
            e,
            validator.valid_range_description()
        )
    })?;

    Ok(id)
}

// ============================================================================
// Pre-defined validators
// ============================================================================

/// J1939 protocol device ID validator.
///
/// Valid device IDs: `0x00` through `0xFF` (0-255)
///
/// J1939 uses 8-bit source addresses, so any value from 0-255 is valid.
///
/// # Examples
///
/// ```rust
/// use cando_simulator_common::device_id::{parse_device_id, J1939DeviceIdValidator};
///
/// let id = parse_device_id("0x8B", &J1939DeviceIdValidator)?;
/// assert_eq!(id, 0x8B);
///
/// let id = parse_device_id("255", &J1939DeviceIdValidator)?;
/// assert_eq!(id, 0xFF);
///
/// // Out of range
/// let result = parse_device_id("0x100", &J1939DeviceIdValidator);
/// assert!(result.is_err());
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct J1939DeviceIdValidator;

impl DeviceIdValidator for J1939DeviceIdValidator {
    fn validate(&self, id: u32) -> Result<()> {
        if id > 0xFF {
            Err(anyhow!(
                "J1939 source address must be between 0x00 and 0xFF"
            ))
        } else {
            Ok(())
        }
    }

    fn valid_range_description(&self) -> &'static str {
        "0x00 through 0xFF (J1939 source address)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Parsing tests
    // ========================================================================

    #[test]
    fn test_parse_hex_lowercase() {
        let id = parse_device_id("0x8b", &J1939DeviceIdValidator).unwrap();
        assert_eq!(id, 0x8B);
    }

    #[test]
    fn test_parse_hex_uppercase() {
        let id = parse_device_id("0X8B", &J1939DeviceIdValidator).unwrap();
        assert_eq!(id, 0x8B);
    }

    #[test]
    fn test_parse_decimal() {
        let id = parse_device_id("139", &J1939DeviceIdValidator).unwrap();
        assert_eq!(id, 139); // 0x8B
    }

    #[test]
    fn test_parse_invalid_hex() {
        let result = parse_device_id("0xZZ", &J1939DeviceIdValidator);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid hex"));
    }

    #[test]
    fn test_parse_invalid_decimal() {
        let result = parse_device_id("not_a_number", &J1939DeviceIdValidator);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid device ID"));
    }

    // ========================================================================
    // J1939 validator tests
    // ========================================================================

    #[test]
    fn test_j1939_valid_full_range() {
        // Test boundaries and some middle values
        let valid_ids = vec![0x00, 0x01, 0x7F, 0x80, 0xFE, 0xFF];
        for id in valid_ids {
            let id_str = format!("0x{:02X}", id);
            let result = parse_device_id(&id_str, &J1939DeviceIdValidator);
            assert!(result.is_ok(), "ID {} should be valid", id_str);
        }
    }

    #[test]
    fn test_j1939_invalid_above_range() {
        let result = parse_device_id("0x100", &J1939DeviceIdValidator);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("0xFF"));
    }

    #[test]
    fn test_j1939_max_valid() {
        let id = parse_device_id("255", &J1939DeviceIdValidator).unwrap();
        assert_eq!(id, 0xFF);
    }

    // ========================================================================
    // Error message quality tests
    // ========================================================================

    #[test]
    fn test_error_message_contains_hex_and_decimal() {
        let result = parse_device_id("0x100", &J1939DeviceIdValidator);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("0x100") || err.contains("256")); // Hex or decimal representation
    }

    #[test]
    fn test_error_message_contains_valid_range() {
        let result = parse_device_id("0x100", &J1939DeviceIdValidator);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("0xFF"));
    }

    #[test]
    fn test_error_message_user_friendly() {
        let result = parse_device_id("999", &J1939DeviceIdValidator);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        // Should contain helpful information
        assert!(err.contains("Reason:"));
        assert!(err.contains("Valid range:"));
    }
}
