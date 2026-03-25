//! Bit-level encoder/decoder implementations for CAN messages.
//!
//! This module provides the actual encode/decode logic that replaces the
//! stub implementations in the generated message types. It handles all the
//! low-level bit manipulation, signal scaling, and device ID processing
//! required for CAN message processing.
//!
//! # Overview
//!
//! The encoder module provides functions for:
//! - Extracting and packing signals from/to CAN frame data
//! - Applying DBC scaling factors and offsets
//! - Device ID embedding and extraction for J1939-style messages
//! - Message-specific encode/decode implementations
//!
//! # Examples
//!
//! ## Basic Signal Processing
//!
//! ```rust
//! use cando_messages::encoder::{extract_signal, pack_signal};
//!
//! // Extract a 4-bit signal from position 2
//! let data = [0b11110000, 0b10101010];
//! let value = extract_signal(&data, 2, 4).unwrap();
//! assert_eq!(value, 0b1100);  // Extracted bits 2-5 from first byte
//!
//! // Pack a value back into CAN data
//! let mut data = [0u8; 2];
//! pack_signal(&mut data, 0, 8, 0xAB).unwrap();
//! assert_eq!(data[0], 0xAB);
//! ```
//!
//! ## Signal Scaling
//!
//! ```rust
//! use cando_messages::encoder::{apply_scaling, apply_inverse_scaling};
//!
//! // Convert raw CAN value to engineering units
//! let raw_rpm = 3000u64;
//! let actual_rpm = apply_scaling(raw_rpm, 0.5, 0.0, false, 16);
//! assert_eq!(actual_rpm, 1500.0);
//!
//! // Convert back to raw value
//! let raw_back = apply_inverse_scaling(1500.0, 0.5, 0.0, 16);
//! assert_eq!(raw_back, 3000);
//! ```
//!
//! ## Device ID Processing
//!
//! ```rust
//! use cando_messages::{DeviceId, encoder::{embed_device_id, extract_device_id}};
//!
//! let base_id = 0x18F00000;  // PDU2 message
//! let device = DeviceId::from(0x42);
//!
//! // Embed device ID into CAN ID
//! let full_id = embed_device_id(base_id, device, None);
//! assert_eq!(full_id, 0x18F00042);
//!
//! // Extract device ID back out (no unwrap needed - always succeeds)
//! let extracted = extract_device_id(full_id);
//! assert_eq!(extracted, device);
//! ```

use crate::common::{
    DecodeError, DeviceId, ValidationError, CAN_BASE_ID_29BIT_MASK, CAN_BASE_ID_MASK,
    CAN_DEVICE_ID_MASK, CAN_EFF_MASK,
};

// Type aliases to reduce complexity warnings
/// Result type for messages with six u64 signals plus device ID
type SixSignalDecodeResult = Result<(DeviceId, u64, u64, u64, u64, u64, u64), DecodeError>;

// CAN ID bit manipulation constants for J1939-style device ID embedding

// CAN ID constants now imported from common module

/// Extract bits from a byte array at a specific position (little-endian).
///
/// This function extracts a signal of the specified bit length from the given
/// position in a CAN data frame, using little-endian bit ordering as specified
/// in most DBC files.
///
/// # Arguments
///
/// * `data` - The byte array containing the CAN frame data
/// * `start_bit` - The starting bit position (0-based)
/// * `length` - The number of bits to extract (1-64)
///
/// # Returns
///
/// The extracted value as a `u64`, or a `DecodeError` if there's insufficient data.
///
/// # Examples
///
/// ```rust
/// use cando_messages::encoder::extract_signal;
///
/// // Extract a 2-bit value from bit position 2
/// let data = [0b11110100]; // Binary: 11110100
/// let value = extract_signal(&data, 2, 2).unwrap();
/// assert_eq!(value, 0b01); // Bits 2-3: 01
///
/// // Extract across byte boundaries
/// let data = [0b11110000, 0b00001111];
/// let value = extract_signal(&data, 4, 8).unwrap();
/// assert_eq!(value, 0b11111111); // 4 bits from each byte = 0xFF
/// ```
///
/// # Errors
///
/// Returns `DecodeError::InsufficientData` if the data array is too small
/// for the requested bit extraction.
pub fn extract_signal(data: &[u8], start_bit: usize, length: usize) -> Result<u64, DecodeError> {
    if data.len() < (start_bit + length).div_ceil(8) {
        return Err(DecodeError::InsufficientData {
            required: (start_bit + length).div_ceil(8),
            available: data.len(),
        });
    }

    let mut value = 0u64;

    for i in 0..length {
        let bit_pos = start_bit + i;
        let byte_idx = bit_pos / 8;
        let bit_in_byte = bit_pos % 8;

        if byte_idx >= data.len() {
            return Err(DecodeError::InsufficientData {
                required: byte_idx + 1,
                available: data.len(),
            });
        }

        let bit = (data[byte_idx] >> bit_in_byte) & 1;
        value |= (bit as u64) << i;
    }

    Ok(value)
}

/// Pack bits into a byte array at a specific position (little-endian).
///
/// This function packs a value into the specified bit position within a CAN
/// data frame, using little-endian bit ordering. This is the inverse operation
/// of `extract_signal`.
///
/// # Arguments
///
/// * `data` - The mutable byte array to pack data into
/// * `start_bit` - The starting bit position (0-based)
/// * `length` - The number of bits to pack (1-64)
/// * `value` - The value to pack into the data
///
/// # Examples
///
/// ```rust
/// use cando_messages::encoder::pack_signal;
///
/// // Pack a 4-bit value at position 0
/// let mut data = [0u8; 2];
/// pack_signal(&mut data, 0, 4, 0b1010).unwrap();
/// assert_eq!(data[0] & 0x0F, 0b1010);
///
/// // Pack across byte boundaries
/// let mut data = [0u8; 2];
/// pack_signal(&mut data, 4, 8, 0xAB).unwrap();
/// assert_eq!(data[0] & 0xF0, 0xB0); // Lower 4 bits of 0xAB
/// assert_eq!(data[1] & 0x0F, 0x0A); // Upper 4 bits of 0xAB
/// ```
///
/// # Errors
///
/// Returns `DecodeError::InsufficientData` if the data array is too small
/// for the requested bit packing operation.
pub fn pack_signal(
    data: &mut [u8],
    start_bit: usize,
    length: usize,
    value: u64,
) -> Result<(), DecodeError> {
    if data.len() < (start_bit + length).div_ceil(8) {
        return Err(DecodeError::InsufficientData {
            required: (start_bit + length).div_ceil(8),
            available: data.len(),
        });
    }

    let mask = if length >= 64 {
        u64::MAX
    } else {
        (1u64 << length) - 1
    };
    let masked_value = value & mask;

    for i in 0..length {
        let bit_pos = start_bit + i;
        let byte_idx = bit_pos / 8;
        let bit_in_byte = bit_pos % 8;

        if byte_idx >= data.len() {
            return Err(DecodeError::InsufficientData {
                required: byte_idx + 1,
                available: data.len(),
            });
        }

        let bit = (masked_value >> i) & 1;
        if bit != 0 {
            data[byte_idx] |= 1 << bit_in_byte;
        } else {
            data[byte_idx] &= !(1 << bit_in_byte);
        }
    }

    Ok(())
}

/// Apply DBC scaling to convert raw CAN values to engineering units
///
/// DBC signals often use linear scaling defined by factor and offset:
/// `PhysicalValue = (RawValue * Factor) + Offset`
///
/// This function handles the scaling conversion with proper bounds checking
/// and supports both signed and unsigned raw values.
///
/// # Signal Scaling Examples
///
/// ## Temperature with Offset
/// ```rust
/// use cando_messages::encoder::apply_scaling;
///
/// // DBC: SG_ Temperature : 0|8@1+ (1,-40) [-40|215] "°C"
/// // Raw value 65 represents 25°C
/// let temp_celsius = apply_scaling(65, 1.0, -40.0, false, 8);
/// assert_eq!(temp_celsius, 25.0);
/// ```
///
/// ## Voltage with Resolution
/// ```rust
/// use cando_messages::encoder::apply_scaling;
///
/// // DBC: SG_ BatteryVoltage : 0|16@1+ (0.01,0) [0|655.35] "V"
/// // Raw value 1250 represents 12.50V
/// let voltage = apply_scaling(1250, 0.01, 0.0, false, 16);
/// assert_eq!(voltage, 12.50);
/// ```
///
/// ## Signed Current Measurement
/// ```rust
/// use cando_messages::encoder::apply_scaling;
///
/// // DBC: SG_ Current : 0|16@1- (0.1,-3276.8) [-3276.8|3276.7] "A"
/// // Raw value -100 (as two's complement) represents -10.0A discharge
/// let current = apply_scaling(65436, 0.1, -3276.8, true, 16); // 65436 = -100 as u16
/// assert!((current - (-3286.8)).abs() < 0.01);
/// ```
///
/// # Arguments
///
/// * `raw_value` - Raw integer value from CAN message
/// * `factor` - DBC scaling factor (multiplier)
/// * `offset` - DBC offset value (additive)
/// * `is_signed` - Whether raw_value should be interpreted as signed
/// * `bit_width` - Number of bits in the raw signal (for sign extension)
///
/// # Returns
///
/// Physical value in engineering units (floating point)
pub fn apply_scaling(
    raw_value: u64,
    factor: f64,
    offset: f64,
    is_signed: bool,
    bit_width: usize,
) -> f64 {
    if is_signed {
        // Convert to signed value using two's complement
        let sign_bit = 1u64 << (bit_width - 1);
        let signed_value = if raw_value & sign_bit != 0 {
            // Negative value: extend sign bits
            let mask = !((1u64 << bit_width) - 1);
            (raw_value | mask) as i64
        } else {
            raw_value as i64
        };
        (signed_value as f64) * factor + offset
    } else {
        (raw_value as f64) * factor + offset
    }
}

/// Apply inverse DBC scaling to convert engineering units to raw CAN values
///
/// This is the inverse of `apply_scaling`, converting physical values back
/// to raw integer values suitable for packing into CAN messages:
/// `RawValue = (PhysicalValue - Offset) / Factor`
///
/// The function includes bounds checking to ensure the result fits within
/// the specified bit width and handles both signed and unsigned outputs.
///
/// # Engineering to Raw Examples
///
/// ## Temperature Encoding
/// ```rust
/// use cando_messages::encoder::apply_inverse_scaling;
///
/// // Convert 25°C to raw value for DBC signal with (1,-40) scaling
/// let raw_temp = apply_inverse_scaling(25.0, 1.0, -40.0, 8);
/// assert_eq!(raw_temp, 65); // 25 - (-40) = 65
/// ```
///
/// ## High-Resolution Voltage
/// ```rust
/// use cando_messages::encoder::apply_inverse_scaling;
///
/// // Convert 12.34V to raw value for 0.01V resolution signal
/// let raw_voltage = apply_inverse_scaling(12.34, 0.01, 0.0, 16);
/// assert_eq!(raw_voltage, 1234); // 12.34 / 0.01 = 1234
/// ```
///
/// ## Bounds Checking
/// ```rust
/// use cando_messages::encoder::apply_inverse_scaling;
///
/// // Value too large for 8-bit unsigned field gets clamped
/// let raw_clamped = apply_inverse_scaling(300.0, 1.0, 0.0, 8);
/// assert_eq!(raw_clamped, 255); // Clamped to maximum 8-bit value
/// ```
///
/// # Arguments
///
/// * `physical_value` - Engineering value to convert
/// * `factor` - DBC scaling factor (divisor)
/// * `offset` - DBC offset value (subtractive)
/// * `bit_width` - Number of bits in target signal (for bounds checking)
///
/// # Returns
///
/// Raw integer value suitable for CAN message packing, bounded to fit
/// within the specified bit width
pub fn apply_inverse_scaling(
    physical_value: f64,
    factor: f64,
    offset: f64,
    bit_width: usize,
) -> u64 {
    let raw_float = (physical_value - offset) / factor;
    let raw_rounded = raw_float.round() as i64;

    // Clamp to valid range for the bit width
    let max_value = if bit_width >= 64 {
        u64::MAX as i64
    } else {
        ((1u64 << bit_width) - 1) as i64
    };

    raw_rounded.max(0).min(max_value) as u64
}

/// Extract device ID from CAN ID (assumes lower 8 bits contain device ID).
///
/// This function extracts the device ID from a J1939-style CAN identifier
/// where the device ID is embedded in the lower 8 bits of the CAN ID.
///
/// # Arguments
///
/// * `can_id` - The complete CAN identifier
///
/// # Returns
///
/// The extracted `DeviceId`, or a `ValidationError` if the device ID is invalid.
///
/// # Examples
///
/// ```rust
/// use cando_messages::{DeviceId, encoder::extract_device_id};
///
/// // Extract device ID from CAN ID - always succeeds
/// let can_id = 0x18F00042; // PDU2 message, device 0x42 embedded
/// let device = extract_device_id(can_id);
/// assert_eq!(device, DeviceId::from(0x42));
/// assert_eq!(device.as_u8(), 0x42);
///
/// // All u8 values are valid J1939 addresses
/// let broadcast_can_id = 0x18F000FF; // PDU2 broadcast
/// let broadcast = extract_device_id(broadcast_can_id);
/// assert_eq!(broadcast, DeviceId::BROADCAST);
///
/// // Even addresses not explicitly defined work
/// let any_can_id = 0x18F00042; // PDU2 message
/// let any_device = extract_device_id(any_can_id);
/// assert_eq!(any_device.as_u8(), 0x42);
/// assert!(any_device.is_normal_device());
/// ```
/// Extract device ID from J1939 CAN ID (PDU-aware).
///
/// This function correctly handles both PDU1 and PDU2 message types:
/// - **PDU1** (PF < 240): Extracts destination address (DA) from bits 15-8
/// - **PDU2** (PF >= 240): Extracts source address (SA) from bits 7-0
///
/// The semantics match the message type:
/// - Command messages (PDU1): Returns the destination device being commanded
/// - Status messages (PDU2): Returns the source device sending status
///
/// # Arguments
///
/// * `can_id` - The J1939 CAN identifier
///
/// # Returns
///
/// The device ID extracted based on PDU type
///
/// # Examples
///
/// ```rust
/// use cando_messages::{DeviceId, encoder::extract_device_id};
///
/// // PDU1 Command: Extract destination (TO device 0x82)
/// let pdu1_id = 0x187D8202;  // Command TO 0x82 FROM 0x02
/// let device = extract_device_id(pdu1_id);
/// assert_eq!(device, DeviceId::from(0x82));
///
/// // PDU2 Status: Extract source (FROM device 0x82)
/// let pdu2_id = 0x18F37082;  // Status FROM 0x82
/// let device = extract_device_id(pdu2_id);
/// assert_eq!(device, DeviceId::from(0x82));
/// ```
pub fn extract_device_id(can_id: u32) -> DeviceId {
    use crate::common::is_j1939_pdu1;

    // Extract DA and SA to determine message type
    let da = ((can_id >> 8) & 0xFF) as u8;
    let sa = (can_id & 0xFF) as u8;

    // Heuristic: True J1939 PDU1 messages have both DA and SA in normal device range (<0xF0)
    // and they're different (destination != source). This distinguishes from:
    // - Proprietary protocols using placeholder addresses (0xFE, 0xFF)
    // - Fixed DA protocols (like UDC where DA=SA)
    let is_true_j1939_pdu1 = is_j1939_pdu1(can_id) && da < 0xF0 && sa < 0xF0 && da != sa;

    let device_id_raw = if is_true_j1939_pdu1 {
        // True J1939 PDU1: Extract destination address from bits 15-8
        da
    } else {
        // PDU2 or non-J1939 protocol: Extract source address from bits 7-0
        sa
    };
    DeviceId::from(device_id_raw)
}

/// Embed device ID and optional source address into J1939 CAN ID.
///
/// This function correctly handles both PDU1 and PDU2 message types:
/// - **PDU1** (PF < 240): Destination-specific messages (commands, requests)
///   - `device_id` represents the DESTINATION address (DA, bits 15-8)
///   - `source_addr` is REQUIRED and represents the SOURCE address (SA, bits 7-0)
///   - Both addresses must be set for proper PDU1 encoding
/// - **PDU2** (PF >= 240): Broadcast messages (status, periodic data)
///   - `device_id` represents the SOURCE address (SA, bits 7-0)
///   - `source_addr` is ignored (the device_id IS the source)
///   - Bits 15-8 contain Group Extension (GE), part of the PGN
///
/// # Arguments
///
/// * `base_can_id` - Base CAN ID with addressing fields cleared
/// * `device_id` - Device ID (semantics depend on PDU type)
/// * `source_addr` - Source address for PDU1 messages (required for commands)
///
/// # Returns
///
/// Complete 29-bit J1939 CAN ID with proper addressing
///
/// # Examples
///
/// ```rust
/// use cando_messages::{DeviceId, encoder::embed_device_id};
///
/// // PDU1 Command: Send TO fan (0x82) FROM controller (0x02)
/// let base_id = 0x187D0000;  // PGN 32000, PF=0x7D (125 < 240, PDU1)
/// let can_id = embed_device_id(
///     base_id,
///     DeviceId::from(0x82),  // Destination: fan
///     Some(0x02)              // Source: controller
/// );
/// assert_eq!(can_id, 0x187D8202);  // Command TO 0x82 FROM 0x02
///
/// // PDU2 Status: Broadcast FROM fan (0x82)
/// let base_id = 0x18F37000;  // PGN 62320, PF=0xF3 (243 >= 240, PDU2)
/// let can_id = embed_device_id(
///     base_id,
///     DeviceId::from(0x82),  // Source: fan
///     None                    // Source addr not used for PDU2
/// );
/// assert_eq!(can_id, 0x18F37082);  // Broadcast FROM 0x82
///
/// // PDU1 with default source (for testing)
/// let can_id = embed_device_id(
///     0x187D0000,
///     DeviceId::from(0x82),
///     None  // Defaults to 0x0F (TEST_DEVICE)
/// );
/// assert_eq!(can_id, 0x187D820F);
/// ```
///
/// # PDU Type Detection
///
/// The function automatically detects PDU type by examining the PF (PDU Format)
/// byte at bits 23-16:
/// - PF < 240 (0xF0): PDU1 - destination-specific
/// - PF >= 240 (0xF0): PDU2 - broadcast
///
/// # Address Field Layout
///
/// ```text
/// PDU1 (PF < 240):
/// ┌─────┬─────┬─────┬─────┐
/// │  P  │ PF  │ DA  │ SA  │
/// │ +DP │     │     │     │
/// └─────┴─────┴─────┴─────┘
///        ^     ^     ^
///        │     │     └─── Source Address (from source_addr)
///        │     └───────── Destination Address (from device_id)
///        └─────────────── PF < 240
///
/// PDU2 (PF >= 240):
/// ┌─────┬─────┬─────┬─────┐
/// │  P  │ PF  │ GE  │ SA  │
/// │ +DP │     │     │     │
/// └─────┴─────┴─────┴─────┘
///        ^     ^     ^
///        │     │     └─── Source Address (from device_id)
///        │     └───────── Group Extension (part of PGN, in base_can_id)
///        └─────────────── PF >= 240
/// ```
pub fn embed_device_id(base_can_id: u32, device_id: DeviceId, source_addr: Option<u8>) -> u32 {
    use crate::common::{is_j1939_pdu1, CAN_EFF_MASK};

    let masked_id = base_can_id & CAN_EFF_MASK;

    // Check if this is a true J1939 PDU1 message
    // True J1939 PDU1: PF < 240 AND base CAN ID has DA=0x00, SA=0x00
    // This distinguishes J1939 messages from other protocols (UDC, etc.) that use fixed addresses
    let is_true_j1939_pdu1 = is_j1939_pdu1(masked_id) && ((masked_id & 0xFFFF) == 0x0000);

    if is_true_j1939_pdu1 {
        // True J1939 PDU1: Destination-specific message
        // device_id -> DA (bits 15-8), source_addr -> SA (bits 7-0)
        let destination = (device_id.as_u8() as u32) << 8;
        let source = source_addr.unwrap_or(0x0F) as u32; // Default to TEST_DEVICE
        (base_can_id & 0xFFFF0000) | destination | source
    } else {
        // PDU2 or non-J1939 protocol: Simple encoding
        // device_id -> SA (bits 7-0), source_addr ignored
        (base_can_id & CAN_BASE_ID_MASK) | (device_id.as_u8() as u32)
    }
}

/// Validate CAN ID matches expected base ID for message type.
///
/// This function validates that a received CAN ID matches the expected base ID
/// for a specific message type, ignoring the device ID portion (lower 8 bits)
/// and applying the 29-bit Extended CAN ID mask.
///
/// # Arguments
///
/// * `can_id` - The received CAN ID to validate
/// * `expected_base` - The expected base CAN ID for the message type
///
/// # Returns
///
/// `Ok(())` if the CAN ID is valid, or `DecodeError::InvalidCanId` if not.
///
/// # Examples
///
/// ```rust
/// use cando_messages::encoder::validate_can_id;
///
/// let base_id = 0x12345600;
/// let device_can_id = 0x1234568A; // Same base + device ID
///
/// // Should validate successfully
/// let result = validate_can_id(device_can_id, base_id);
/// assert!(result.is_ok());
///
/// // Different base ID should fail
/// let wrong_base = 0x87654300;
/// let result = validate_can_id(device_can_id, wrong_base);
/// assert!(result.is_err());
/// ```
pub fn validate_can_id(can_id: u32, expected_base: u32) -> Result<(), DecodeError> {
    use crate::common::get_j1939_base_id;

    // Extract J1939-aware base ID (handles PDU1/PDU2 correctly)
    // PDU1: strips both DA (bits 15-8) and SA (bits 7-0)
    // PDU2: strips only SA (bits 7-0), preserves GE (bits 15-8)
    let base_part = get_j1939_base_id(can_id);
    let expected_base_part = get_j1939_base_id(expected_base);

    if base_part != expected_base_part {
        return Err(DecodeError::InvalidCanId {
            expected: expected_base_part,
            actual: base_part,
        });
    }
    Ok(())
}

/// MCM_MotorCommandMessage decoder implementation.
///
/// Decodes an MCM (Motor Control Module) Motor Command Message from raw CAN
/// frame data, extracting all signal values and validating the message format.
///
/// # Arguments
///
/// * `can_id` - The CAN identifier for validation
/// * `data` - The 8-byte CAN data frame
///
/// # Returns
///
/// A tuple containing:
/// - `DeviceId` - The target device ID
/// - `u64` - On/Off Direction Command (0-3)
/// - `u64` - Power Hold Command (0-3)
/// - `f64` - Motor Speed Command in RPM
/// - `f64` - Percent Motor Speed Command (0-125%)
///
/// # Examples
///
/// ```rust
/// use cando_messages::{DeviceId, encoder::{decode_mcm_motor_command_message, embed_device_id}};
///
/// // Create test CAN frame data
/// let mut data = [0u8; 8];
/// data[0] = 0x05; // on_off=1, power_hold=1
/// data[1] = 0xD0; data[2] = 0x07; // speed = 2000 raw (1000 RPM)
/// data[3] = 0x64; // percent = 100 raw (50%)
///
/// let can_id = embed_device_id(2565865214u32, DeviceId::from(0x42), None);
/// let result = decode_mcm_motor_command_message(can_id, &data).unwrap();
///
/// assert_eq!(result.0, DeviceId::from(0x42));
/// assert_eq!(result.1, 1); // on_off_direction
/// assert_eq!(result.2, 1); // power_hold
/// assert_eq!(result.3, 1000.0); // motor_speed
/// assert_eq!(result.4, 50.0); // percent_speed
/// ```
pub fn decode_mcm_motor_command_message(
    can_id: u32,
    data: &[u8],
) -> Result<(DeviceId, u64, u64, f64, f64), DecodeError> {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    // Validate base CAN ID
    validate_can_id(can_id, 2565865214)?;

    let device_id = extract_device_id(can_id);

    // MCM_OnOffDirectionCommand: bit 0, length 2
    let on_off_direction = extract_signal(data, 0, 2)?;

    // MCM_PowerHoldCommand: bit 2, length 2
    let power_hold = extract_signal(data, 2, 2)?;

    // MCM_MotorSpeedCommand: bit 8, length 16, factor 0.5
    let motor_speed_raw = extract_signal(data, 8, 16)?;
    let motor_speed = apply_scaling(motor_speed_raw, 0.5, 0.0, false, 16);

    // MCM_PercentMotorSpeedCommand: bit 24, length 8, factor 0.5
    let percent_speed_raw = extract_signal(data, 24, 8)?;
    let percent_speed = apply_scaling(percent_speed_raw, 0.5, 0.0, false, 8);

    Ok((
        device_id,
        on_off_direction,
        power_hold,
        motor_speed,
        percent_speed,
    ))
}

/// MCM_MotorCommandMessage encoder implementation.
///
/// Encodes an MCM (Motor Command Message) into raw CAN frame data with
/// proper signal packing and scaling.
///
/// # Arguments
///
/// * `device_id` - The source device ID
/// * `on_off_direction` - On/off direction command (0-3)
/// * `power_hold` - Power hold command (0-3)
/// * `motor_speed` - Motor speed command in RPM
/// * `percent_speed` - Percent motor speed command (0-125%)
///
/// # Returns
///
/// A tuple containing the CAN ID with embedded device ID and the 8-byte data frame.
///
/// # Errors
///
/// Returns `DecodeError` if signal packing fails.
pub fn encode_mcm_motor_command_message(
    device_id: DeviceId,
    on_off_direction: u64,
    power_hold: u64,
    motor_speed: f64,
    percent_speed: f64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // MCM_OnOffDirectionCommand: bit 0, length 2
    pack_signal(&mut data, 0, 2, on_off_direction)?;

    // MCM_PowerHoldCommand: bit 2, length 2
    pack_signal(&mut data, 2, 2, power_hold)?;

    // MCM_MotorSpeedCommand: bit 8, length 16, factor 0.5
    let motor_speed_raw = apply_inverse_scaling(motor_speed, 0.5, 0.0, 16);
    pack_signal(&mut data, 8, 16, motor_speed_raw)?;

    // MCM_PercentMotorSpeedCommand: bit 24, length 8, factor 0.5
    let percent_speed_raw = apply_inverse_scaling(percent_speed, 0.5, 0.0, 8);
    pack_signal(&mut data, 24, 8, percent_speed_raw)?;

    let can_id = embed_device_id(2565865214, device_id, None);
    Ok((can_id, data))
}

/// MSM2_MotorStatusMessage2 encoder implementation.
///
/// Encodes an MSM2 (Motor Status Message 2) into raw CAN frame data with
/// proper signal packing and scaling.
///
/// # Arguments
///
/// * `device_id` - The source device ID
/// * `on_off_direction` - Motor on/off direction status (0-3)
/// * `controller_status` - Controller status code (0-15)
/// * `command_status` - Command status (0-3)
/// * `motor_speed` - Measured motor speed in RPM
/// * `external_temp` - External temperature in Celsius
/// * `motor_power` - Motor power in Watts
/// * `service_indicator` - Service indicator (0-3)
/// * `operation_status` - Operation status (0-3)
///
/// # Returns
///
/// A tuple containing the CAN ID and 8-byte data array.
///
/// # Examples
///
/// ```rust
/// use cando_messages::{DeviceId, encoder::{encode_msm2_motor_status_message2, extract_device_id}};
///
/// let result = encode_msm2_motor_status_message2(
///     DeviceId::from(0x42),
///     1,      // Motor on, normal direction
///     0,      // Normal controller status
///     1,      // External command status
///     1500.0, // 1500 RPM
///     25.0,   // 25°C
///     750.0,  // 750W
///     0,      // No service required
///     0,      // Normal operation
/// ).unwrap();
///
/// let (can_id, data) = result;
/// assert_eq!(extract_device_id(can_id), DeviceId::from(0x42));
/// assert_eq!(data[0] & 0x03, 1);   // on_off_direction in bits 0-1
/// ```
#[allow(clippy::too_many_arguments)]
pub fn encode_msm2_motor_status_message2(
    device_id: DeviceId,
    on_off_direction: u64,
    controller_status: u64,
    command_status: u64,
    motor_speed: f64,
    external_temp: f64,
    motor_power: f64,
    service_indicator: u64,
    operation_status: u64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // MSM2_OnOffDirectionStatus: bit 0, length 2
    pack_signal(&mut data, 0, 2, on_off_direction)?;

    // MSM2_ControllerStatus: bit 2, length 4
    pack_signal(&mut data, 2, 4, controller_status)?;

    // MSM2_CommandStatus: bit 6, length 2
    pack_signal(&mut data, 6, 2, command_status)?;

    // MSM2_MeasuredMotorSpeed: bit 8, length 16, factor 0.5
    let motor_speed_raw = apply_inverse_scaling(motor_speed, 0.5, 0.0, 16);
    pack_signal(&mut data, 8, 16, motor_speed_raw)?;

    // MSM2_MeasuredExternalTemp: bit 24, length 16, factor 0.03125, offset -273
    let temp_raw = apply_inverse_scaling(external_temp, 0.03125, -273.0, 16);
    pack_signal(&mut data, 24, 16, temp_raw)?;

    // MSM2_MeasuredMotorPower: bit 40, length 16, factor 0.5
    let power_raw = apply_inverse_scaling(motor_power, 0.5, 0.0, 16);
    pack_signal(&mut data, 40, 16, power_raw)?;

    // MSM2_ServiceIndicator: bit 56, length 2
    pack_signal(&mut data, 56, 2, service_indicator)?;

    // MSM2_OperationStatus: bit 58, length 2
    pack_signal(&mut data, 58, 2, operation_status)?;

    let can_id = embed_device_id(2566857726, device_id, None);
    Ok((can_id, data))
}

/// MSM1_MotorStatusMessage1 encoder implementation.
///
/// Encodes an MSM1 (Motor Status Message 1) message into raw CAN frame
/// data with proper signal scaling and device ID embedding.
///
/// # Arguments
///
/// * `device_id` - The source device ID
/// * `on_off_direction_status` - Motor on/off direction status (0-3)
/// * `controller_status` - Controller status code (0-31)
/// * `measured_motor_speed` - Measured motor speed in RPM
/// * `measured_motor_power` - Measured motor power in Watts
/// * `measured_percent_motor_speed` - Measured motor speed as percentage
///
/// # Returns
///
/// A tuple containing the CAN ID and 8-byte data array.
///
/// # Examples
///
/// ```rust
/// use cando_messages::{DeviceId, encoder::{encode_msm1_motor_status_message1, extract_device_id}};
///
/// let result = encode_msm1_motor_status_message1(
///     DeviceId::from(0x42),
///     1,       // Motor on, normal direction
///     16,      // Normal operation
///     1500.0,  // 1500 RPM
///     750,     // 750W
///     75.0,    // 75%
/// ).unwrap();
///
/// let (can_id, data) = result;
/// assert_eq!(extract_device_id(can_id), DeviceId::from(0x42));
/// assert_eq!(data[0] & 0x03, 1);   // on_off_direction in bits 0-1
/// ```
pub fn encode_msm1_motor_status_message1(
    device_id: DeviceId,
    on_off_direction_status: u64,
    controller_status: u64,
    measured_motor_speed: f64,
    measured_motor_power: u64,
    measured_percent_motor_speed: f64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // MSM1_OnOffDirectionStatus: bit 0, length 2
    pack_signal(&mut data, 0, 2, on_off_direction_status)?;

    // MSM1_ControllerStatus: bit 2, length 5
    pack_signal(&mut data, 2, 5, controller_status)?;

    // MSM1_MeasuredMotorSpeed: bit 8, length 16, factor 0.5
    let motor_speed_raw = apply_inverse_scaling(measured_motor_speed, 0.5, 0.0, 16);
    pack_signal(&mut data, 8, 16, motor_speed_raw)?;

    // MSM1_MeasuredMotorPower: bit 40, length 16, factor 1.0
    pack_signal(&mut data, 40, 16, measured_motor_power)?;

    // MSM1_MeasuredPercentMotorSpeed: bit 56, length 8, factor 0.5
    let percent_speed_raw = apply_inverse_scaling(measured_percent_motor_speed, 0.5, 0.0, 8);
    pack_signal(&mut data, 56, 8, percent_speed_raw)?;

    let can_id = embed_device_id(2566849534, device_id, None);
    Ok((can_id, data))
}

/// MSM1_MotorStatusMessage1 decoder implementation.
///
/// Decodes an MSM1 (Motor Status Message 1) message from raw CAN frame data.
///
/// # Arguments
///
/// * `can_id` - The CAN identifier (with embedded device ID)
/// * `data` - The 8-byte CAN data frame
///
/// # Returns
///
/// A tuple containing: device_id, on_off_direction_status, controller_status,
/// measured_motor_speed, measured_motor_power, measured_percent_motor_speed
pub fn decode_msm1_motor_status_message1(
    can_id: u32,
    data: &[u8],
) -> Result<(DeviceId, u64, u64, f64, u64, f64), DecodeError> {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // MSM1_OnOffDirectionStatus: bit 0, length 2
    let on_off_direction = extract_signal(data, 0, 2)?;

    // MSM1_ControllerStatus: bit 2, length 5
    let controller_status = extract_signal(data, 2, 5)?;

    // MSM1_MeasuredMotorSpeed: bit 8, length 16, factor 0.5
    let motor_speed_raw = extract_signal(data, 8, 16)?;
    let motor_speed = apply_scaling(motor_speed_raw, 0.5, 0.0, false, 16);

    // MSM1_MeasuredMotorPower: bit 40, length 16, factor 1.0
    let motor_power = extract_signal(data, 40, 16)?;

    // MSM1_MeasuredPercentMotorSpeed: bit 56, length 8, factor 0.5
    let percent_speed_raw = extract_signal(data, 56, 8)?;
    let percent_speed = apply_scaling(percent_speed_raw, 0.5, 0.0, false, 8);

    Ok((
        device_id,
        on_off_direction,
        controller_status,
        motor_speed,
        motor_power,
        percent_speed,
    ))
}

/// MSM3_MotorStatusMessage3 encoder implementation.
///
/// Encodes an MSM3 (Motor Status Message 3) message into raw CAN frame
/// data with proper signal scaling and device ID embedding.
///
/// # Arguments
///
/// * `device_id` - The source device ID
/// * `motor_voltage` - Motor voltage in Volts
/// * `motor_current` - Motor current in Amperes
/// * `hvil_status` - High Voltage Interlock status (0-3)
///
/// # Returns
///
/// A tuple containing the CAN ID and 8-byte data array.
///
/// # Examples
///
/// ```rust
/// use cando_messages::{DeviceId, encoder::{encode_msm3_motor_status_message3, extract_device_id}};
///
/// let result = encode_msm3_motor_status_message3(
///     DeviceId::from(0x42),
///     320.5,  // 320.5V
///     50.25,  // 50.25A
///     0,      // HVIL Closed
/// ).unwrap();
///
/// let (can_id, data) = result;
/// assert_eq!(extract_device_id(can_id), DeviceId::from(0x42));
/// ```
pub fn encode_msm3_motor_status_message3(
    device_id: DeviceId,
    motor_voltage: f64,
    motor_current: f64,
    hvil_status: u64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // MSM3_MotorVoltage: bit 0, length 16, factor 0.05, offset 0
    let voltage_raw = apply_inverse_scaling(motor_voltage, 0.05, 0.0, 16);
    pack_signal(&mut data, 0, 16, voltage_raw)?;

    // MSM3_MotorCurrent: bit 16, length 16, factor 0.05, offset -1600
    let current_raw = apply_inverse_scaling(motor_current, 0.05, -1600.0, 16);
    pack_signal(&mut data, 16, 16, current_raw)?;

    // MSM3_HVILstatus: bit 32, length 2
    pack_signal(&mut data, 32, 2, hvil_status)?;

    let can_id = embed_device_id(2566857982, device_id, None);
    Ok((can_id, data))
}

/// MSM3_MotorStatusMessage3 decoder implementation.
///
/// Decodes an MSM3 (Motor Status Message 3) message from raw CAN frame data.
///
/// # Arguments
///
/// * `can_id` - The CAN identifier (with embedded device ID)
/// * `data` - The 8-byte CAN data frame
///
/// # Returns
///
/// A tuple containing: device_id, motor_voltage, motor_current, hvil_status
pub fn decode_msm3_motor_status_message3(
    can_id: u32,
    data: &[u8],
) -> Result<(DeviceId, f64, f64, u64), DecodeError> {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // MSM3_MotorVoltage: bit 0, length 16, factor 0.05, offset 0
    let voltage_raw = extract_signal(data, 0, 16)?;
    let motor_voltage = apply_scaling(voltage_raw, 0.05, 0.0, false, 16);

    // MSM3_MotorCurrent: bit 16, length 16, factor 0.05, offset -1600
    let current_raw = extract_signal(data, 16, 16)?;
    let motor_current = apply_scaling(current_raw, 0.05, -1600.0, false, 16);

    // MSM3_HVILstatus: bit 32, length 2
    let hvil_status = extract_signal(data, 32, 2)?;

    Ok((device_id, motor_voltage, motor_current, hvil_status))
}

/// MET_MeasuredExternalTemperature encoder implementation.
///
/// Encodes a MET (Measured External Temperature) message into raw CAN frame
/// data with proper temperature scaling and device ID embedding.
///
/// # Arguments
///
/// * `device_id` - The source device ID
/// * `temperature` - The temperature value in Celsius
///
/// # Returns
///
/// A tuple containing the CAN ID and 8-byte data array.
///
/// # Examples
///
/// ```rust
/// use cando_messages::{DeviceId, encoder::{encode_met_external_temperature, extract_device_id}};
///
/// let result = encode_met_external_temperature(
///     DeviceId::from(0x42),
///     25.0,  // 25°C
/// ).unwrap();
///
/// let (can_id, data) = result;
/// assert_eq!(extract_device_id(can_id), DeviceId::from(0x42));
///
/// // Temperature is scaled: (25 + 273) / 0.03125 = 9536
/// let temp_raw = ((data[1] as u16) << 8) | (data[0] as u16);
/// assert_eq!(temp_raw, 9536);
/// ```
pub fn encode_met_external_temperature(
    device_id: DeviceId,
    temperature: f64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // MET_ExternalTemperatureMeasured: bit 0, length 16, factor 0.03125, offset -273
    let temp_raw = apply_inverse_scaling(temperature, 0.03125, -273.0, 16);
    pack_signal(&mut data, 0, 16, temp_raw)?;

    let can_id = embed_device_id(2566865918, device_id, None);
    Ok((can_id, data))
}

// ============================================================================
// J1939 Protocol Message Encode/Decode Functions
// ============================================================================

/// Encode CN (Crash Notification) message.
///
/// # Signal Layout
/// - CrashChecksum: bits 60-63 (4 bits, unsigned)
/// - CrashCounter: bits 56-59 (4 bits, unsigned)
/// - CrashType: bits 0-4 (5 bits, unsigned)
pub fn encode_cn(
    device_id: DeviceId,
    crashchecksum: u64,
    crashcounter: u64,
    crashtype: u64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // CrashType: bit 0, length 5
    pack_signal(&mut data, 0, 5, crashtype)?;

    // CrashCounter: bit 56, length 4
    pack_signal(&mut data, 56, 4, crashcounter)?;

    // CrashChecksum: bit 60, length 4
    pack_signal(&mut data, 60, 4, crashchecksum)?;

    let can_id = embed_device_id(2163223550, device_id, None);
    Ok((can_id, data))
}

/// Decode CN (Crash Notification) message.
pub fn decode_cn(can_id: u32, data: &[u8]) -> Result<(DeviceId, u64, u64, u64), DecodeError> {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // CrashType: bit 0, length 5
    let crashtype = extract_signal(data, 0, 5)?;

    // CrashCounter: bit 56, length 4
    let crashcounter = extract_signal(data, 56, 4)?;

    // CrashChecksum: bit 60, length 4
    let crashchecksum = extract_signal(data, 60, 4)?;

    Ok((device_id, crashchecksum, crashcounter, crashtype))
}

/// Encode WAND (Wand Angle) message.
///
/// # Signal Layout
/// - WandAngle: bits 0-15 (16 bits, unsigned, factor=0.002, offset=-64.0)
/// - WandSensorFigureOfMerit: bits 16-17 (2 bits, unsigned)
pub fn encode_wand(
    device_id: DeviceId,
    wandangle: f64,
    wandsensorfigureofmerit: u64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // WandAngle: bit 0, length 16, factor 0.002, offset -64.0
    let angle_raw = apply_inverse_scaling(wandangle, 0.002, -64.0, 16);
    pack_signal(&mut data, 0, 16, angle_raw)?;

    // WandSensorFigureOfMerit: bit 16, length 2
    pack_signal(&mut data, 16, 2, wandsensorfigureofmerit)?;

    let can_id = embed_device_id(2230333950, device_id, None);
    Ok((can_id, data))
}

/// Decode WAND (Wand Angle) message.
pub fn decode_wand(can_id: u32, data: &[u8]) -> Result<(DeviceId, f64, u64), DecodeError> {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // WandAngle: bit 0, length 16, factor 0.002, offset -64.0
    let angle_raw = extract_signal(data, 0, 16)?;
    let wandangle = apply_scaling(angle_raw, 0.002, -64.0, false, 16);

    // WandSensorFigureOfMerit: bit 16, length 2
    let wandsensorfigureofmerit = extract_signal(data, 16, 2)?;

    Ok((device_id, wandangle, wandsensorfigureofmerit))
}

/// Encode LDISP (Linear Displacement) message.
///
/// # Signal Layout
/// - MeasuredLinearDisplacement: bits 0-15 (16 bits, unsigned, factor=0.1, offset=0)
/// - LnrDsplmntSnsrSnsrFgrOfMrt: bits 16-17 (2 bits, unsigned)
pub fn encode_ldisp(
    device_id: DeviceId,
    measuredlineardisplacement: f64,
    lnrdsplmntsnsrsnsrfgrofmrt: u64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // MeasuredLinearDisplacement: bit 0, length 16, factor 0.1, offset 0
    let displacement_raw = apply_inverse_scaling(measuredlineardisplacement, 0.1, 0.0, 16);
    pack_signal(&mut data, 0, 16, displacement_raw)?;

    // LnrDsplmntSnsrSnsrFgrOfMrt: bit 16, length 2
    pack_signal(&mut data, 16, 2, lnrdsplmntsnsrsnsrfgrofmrt)?;

    let can_id = embed_device_id(2230334206, device_id, None);
    Ok((can_id, data))
}

/// Decode LDISP (Linear Displacement) message.
pub fn decode_ldisp(can_id: u32, data: &[u8]) -> Result<(DeviceId, f64, u64), DecodeError> {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // MeasuredLinearDisplacement: bit 0, length 16, factor 0.1, offset 0
    let displacement_raw = extract_signal(data, 0, 16)?;
    let measuredlineardisplacement = apply_scaling(displacement_raw, 0.1, 0.0, false, 16);

    // LnrDsplmntSnsrSnsrFgrOfMrt: bit 16, length 2
    let lnrdsplmntsnsrsnsrfgrofmrt = extract_signal(data, 16, 2)?;

    Ok((
        device_id,
        measuredlineardisplacement,
        lnrdsplmntsnsrsnsrfgrofmrt,
    ))
}

/// Encode EEC12 (Engine Exhaust Sensor Power Supply) message.
///
/// # Signal Layout
/// - EngnExhst1GsSnsr1PwrSppl: bits 0-1 (2 bits, unsigned)
/// - Aftrtrtmnt1Otlt1GsSnsrPwrSppl: bits 2-3 (2 bits, unsigned)
/// - EngnExhst2GsSnsr1PwrSppl: bits 4-5 (2 bits, unsigned)
/// - Aftrtrtmnt2Otlt1GsSnsrPwrSppl: bits 6-7 (2 bits, unsigned)
/// - EngnExhst1GsSnsr2PwrSppl: bits 8-9 (2 bits, unsigned)
/// - Aftrtrtmnt1Otlt2GsSnsrPwrSppl: bits 10-11 (2 bits, unsigned)
pub fn encode_eec12(
    device_id: DeviceId,
    engnexhst1gssnsr1pwrsppl: u64,
    aftrtrtmnt1otlt1gssnsrpwrsppl: u64,
    engnexhst2gssnsr1pwrsppl: u64,
    aftrtrtmnt2otlt1gssnsrpwrsppl: u64,
    engnexhst1gssnsr2pwrsppl: u64,
    aftrtrtmnt1otlt2gssnsrpwrsppl: u64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // EngnExhst1GsSnsr1PwrSppl: bit 0, length 2
    pack_signal(&mut data, 0, 2, engnexhst1gssnsr1pwrsppl)?;

    // Aftrtrtmnt1Otlt1GsSnsrPwrSppl: bit 2, length 2
    pack_signal(&mut data, 2, 2, aftrtrtmnt1otlt1gssnsrpwrsppl)?;

    // EngnExhst2GsSnsr1PwrSppl: bit 4, length 2
    pack_signal(&mut data, 4, 2, engnexhst2gssnsr1pwrsppl)?;

    // Aftrtrtmnt2Otlt1GsSnsrPwrSppl: bit 6, length 2
    pack_signal(&mut data, 6, 2, aftrtrtmnt2otlt1gssnsrpwrsppl)?;

    // EngnExhst1GsSnsr2PwrSppl: bit 8, length 2
    pack_signal(&mut data, 8, 2, engnexhst1gssnsr2pwrsppl)?;

    // Aftrtrtmnt1Otlt2GsSnsrPwrSppl: bit 10, length 2
    pack_signal(&mut data, 10, 2, aftrtrtmnt1otlt2gssnsrpwrsppl)?;

    let can_id = embed_device_id(2566704382, device_id, None);
    Ok((can_id, data))
}

/// Decode EEC12 (Engine Exhaust Sensor Power Supply) message.
pub fn decode_eec12(can_id: u32, data: &[u8]) -> SixSignalDecodeResult {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // EngnExhst1GsSnsr1PwrSppl: bit 0, length 2
    let engnexhst1gssnsr1pwrsppl = extract_signal(data, 0, 2)?;

    // Aftrtrtmnt1Otlt1GsSnsrPwrSppl: bit 2, length 2
    let aftrtrtmnt1otlt1gssnsrpwrsppl = extract_signal(data, 2, 2)?;

    // EngnExhst2GsSnsr1PwrSppl: bit 4, length 2
    let engnexhst2gssnsr1pwrsppl = extract_signal(data, 4, 2)?;

    // Aftrtrtmnt2Otlt1GsSnsrPwrSppl: bit 6, length 2
    let aftrtrtmnt2otlt1gssnsrpwrsppl = extract_signal(data, 6, 2)?;

    // EngnExhst1GsSnsr2PwrSppl: bit 8, length 2
    let engnexhst1gssnsr2pwrsppl = extract_signal(data, 8, 2)?;

    // Aftrtrtmnt1Otlt2GsSnsrPwrSppl: bit 10, length 2
    let aftrtrtmnt1otlt2gssnsrpwrsppl = extract_signal(data, 10, 2)?;

    Ok((
        device_id,
        engnexhst1gssnsr1pwrsppl,
        aftrtrtmnt1otlt1gssnsrpwrsppl,
        engnexhst2gssnsr1pwrsppl,
        aftrtrtmnt2otlt1gssnsrpwrsppl,
        engnexhst1gssnsr2pwrsppl,
        aftrtrtmnt1otlt2gssnsrpwrsppl,
    ))
}

/// Encode ETC5 (Transmission Control Status) message.
///
/// # Signal Layout
/// - TrnsmssnHghRngSnsSwth: bits 0-1 (2 bits, unsigned)
/// - TransmissionLowRangeSenseSwitch: bits 2-3 (2 bits, unsigned)
/// - TransmissionSplitterPosition: bits 4-5 (2 bits, unsigned)
/// - TrnsmssnRvrsDrtnSwth: bits 8-9 (2 bits, unsigned)
/// - TransmissionNeutralSwitch: bits 10-11 (2 bits, unsigned)
/// - TrnsmssnFrwrdDrtnSwth: bits 12-13 (2 bits, unsigned)
pub fn encode_etc5(
    device_id: DeviceId,
    trnsmssnhghrngsnsswth: u64,
    transmissionlowrangesenseswitch: u64,
    transmissionsplitterposition: u64,
    trnsmssnrvrsdrtnswth: u64,
    transmissionneutralswitch: u64,
    trnsmssnfrwrddrtnswth: u64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // TrnsmssnHghRngSnsSwth: bit 0, length 2
    pack_signal(&mut data, 0, 2, trnsmssnhghrngsnsswth)?;

    // TransmissionLowRangeSenseSwitch: bit 2, length 2
    pack_signal(&mut data, 2, 2, transmissionlowrangesenseswitch)?;

    // TransmissionSplitterPosition: bit 4, length 2
    pack_signal(&mut data, 4, 2, transmissionsplitterposition)?;

    // TrnsmssnRvrsDrtnSwth: bit 8, length 2
    pack_signal(&mut data, 8, 2, trnsmssnrvrsdrtnswth)?;

    // TransmissionNeutralSwitch: bit 10, length 2
    pack_signal(&mut data, 10, 2, transmissionneutralswitch)?;

    // TrnsmssnFrwrdDrtnSwth: bit 12, length 2
    pack_signal(&mut data, 12, 2, trnsmssnfrwrddrtnswth)?;

    let can_id = embed_device_id(2633942014, device_id, None);
    Ok((can_id, data))
}

/// Decode ETC5 (Transmission Control Status) message.
pub fn decode_etc5(can_id: u32, data: &[u8]) -> SixSignalDecodeResult {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // TrnsmssnHghRngSnsSwth: bit 0, length 2
    let trnsmssnhghrngsnsswth = extract_signal(data, 0, 2)?;

    // TransmissionLowRangeSenseSwitch: bit 2, length 2
    let transmissionlowrangesenseswitch = extract_signal(data, 2, 2)?;

    // TransmissionSplitterPosition: bit 4, length 2
    let transmissionsplitterposition = extract_signal(data, 4, 2)?;

    // TrnsmssnRvrsDrtnSwth: bit 8, length 2
    let trnsmssnrvrsdrtnswth = extract_signal(data, 8, 2)?;

    // TransmissionNeutralSwitch: bit 10, length 2
    let transmissionneutralswitch = extract_signal(data, 10, 2)?;

    // TrnsmssnFrwrdDrtnSwth: bit 12, length 2
    let trnsmssnfrwrddrtnswth = extract_signal(data, 12, 2)?;

    Ok((
        device_id,
        trnsmssnhghrngsnsswth,
        transmissionlowrangesenseswitch,
        transmissionsplitterposition,
        trnsmssnrvrsdrtnswth,
        transmissionneutralswitch,
        trnsmssnfrwrddrtnswth,
    ))
}

/// Encode AEBS2 (Advanced Emergency Braking System 2) message.
///
/// # Signal Layout
/// - DvAtvtDdFAdvdEBSst: bits 0-1 (2 bits, unsigned)
/// - Aebs2MessageCounter: bits 56-59 (4 bits, unsigned)
/// - Aebs2MessageChecksum: bits 60-63 (4 bits, unsigned)
pub fn encode_aebs2(
    device_id: DeviceId,
    dvatvtddfadvdebsst: u64,
    aebs2messagecounter: u64,
    aebs2messagechecksum: u64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // DvAtvtDdFAdvdEBSst: bit 0, length 2
    pack_signal(&mut data, 0, 2, dvatvtddfadvdebsst)?;

    // Aebs2MessageCounter: bit 56, length 4
    pack_signal(&mut data, 56, 4, aebs2messagecounter)?;

    // Aebs2MessageChecksum: bit 60, length 4
    pack_signal(&mut data, 60, 4, aebs2messagechecksum)?;

    let can_id = embed_device_id(2349596414, device_id, None);
    Ok((can_id, data))
}

/// Decode AEBS2 (Advanced Emergency Braking System 2) message.
pub fn decode_aebs2(can_id: u32, data: &[u8]) -> Result<(DeviceId, u64, u64, u64), DecodeError> {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // DvAtvtDdFAdvdEBSst: bit 0, length 2
    let dvatvtddfadvdebsst = extract_signal(data, 0, 2)?;

    // Aebs2MessageCounter: bit 56, length 4
    let aebs2messagecounter = extract_signal(data, 56, 4)?;

    // Aebs2MessageChecksum: bit 60, length 4
    let aebs2messagechecksum = extract_signal(data, 60, 4)?;

    Ok((
        device_id,
        dvatvtddfadvdebsst,
        aebs2messagecounter,
        aebs2messagechecksum,
    ))
}

/// Encode EEC22 (Electronic Engine Controller 22) message.
///
/// # Signal Layout
/// - EngnExhstGsRrltn1ClrIntkPrssr: bits 0-15 (16 bits, unsigned, factor=5.0)
/// - TtlNmrOfCrnkAttmptsDrngEngnLf: bits 16-47 (32 bits, unsigned, factor=1.0)
pub fn encode_eec22(
    device_id: DeviceId,
    engnexhstgsrrltn1clrintkprssr: f64,
    ttlnmrofcrnkattmptsdrngengnlf: u64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // EngnExhstGsRrltn1ClrIntkPrssr: bit 0, length 16, factor 5.0, offset 0.0
    let pressure_raw = apply_inverse_scaling(engnexhstgsrrltn1clrintkprssr, 5.0, 0.0, 16);
    pack_signal(&mut data, 0, 16, pressure_raw)?;

    // TtlNmrOfCrnkAttmptsDrngEngnLf: bit 16, length 32, factor 1.0, offset 0.0
    pack_signal(&mut data, 16, 32, ttlnmrofcrnkattmptsdrngengnlf)?;

    let can_id = embed_device_id(2566624510, device_id, None);
    Ok((can_id, data))
}

/// Decode EEC22 (Electronic Engine Controller 22) message.
pub fn decode_eec22(can_id: u32, data: &[u8]) -> Result<(DeviceId, f64, u64), DecodeError> {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // EngnExhstGsRrltn1ClrIntkPrssr: bit 0, length 16, factor 5.0, offset 0.0
    let pressure_raw = extract_signal(data, 0, 16)?;
    let engnexhstgsrrltn1clrintkprssr = apply_scaling(pressure_raw, 5.0, 0.0, false, 16);

    // TtlNmrOfCrnkAttmptsDrngEngnLf: bit 16, length 32, factor 1.0, offset 0.0
    let ttlnmrofcrnkattmptsdrngengnlf = extract_signal(data, 16, 32)?;

    Ok((
        device_id,
        engnexhstgsrrltn1clrintkprssr,
        ttlnmrofcrnkattmptsdrngengnlf,
    ))
}

/// Encode EEC21 (Electronic Engine Controller 21) message.
///
/// # Signal Layout
/// - EngnExhstMnfldAsltPrssr1: bits 0-15 (16 bits, unsigned, factor=0.1)
/// - EngnExhstMnfldAsltPrssr2: bits 16-31 (16 bits, unsigned, factor=0.1)
pub fn encode_eec21(
    device_id: DeviceId,
    engnexhstmnfldasltprssr1: f64,
    engnexhstmnfldasltprssr2: f64,
) -> Result<(u32, [u8; 8]), DecodeError> {
    let mut data = [0u8; 8];

    // EngnExhstMnfldAsltPrssr1: bit 0, length 16, factor 0.1, offset 0.0
    let pressure1_raw = apply_inverse_scaling(engnexhstmnfldasltprssr1, 0.1, 0.0, 16);
    pack_signal(&mut data, 0, 16, pressure1_raw)?;

    // EngnExhstMnfldAsltPrssr2: bit 16, length 16, factor 0.1, offset 0.0
    let pressure2_raw = apply_inverse_scaling(engnexhstmnfldasltprssr2, 0.1, 0.0, 16);
    pack_signal(&mut data, 16, 16, pressure2_raw)?;

    let can_id = embed_device_id(2566641406, device_id, None);
    Ok((can_id, data))
}

/// Decode EEC21 (Electronic Engine Controller 21) message.
pub fn decode_eec21(can_id: u32, data: &[u8]) -> Result<(DeviceId, f64, f64), DecodeError> {
    if data.len() != 8 {
        return Err(DecodeError::InvalidLength {
            expected: 8,
            actual: data.len() as u8,
        });
    }

    let device_id = extract_device_id(can_id);

    // EngnExhstMnfldAsltPrssr1: bit 0, length 16, factor 0.1, offset 0.0
    let pressure1_raw = extract_signal(data, 0, 16)?;
    let engnexhstmnfldasltprssr1 = apply_scaling(pressure1_raw, 0.1, 0.0, false, 16);

    // EngnExhstMnfldAsltPrssr2: bit 16, length 16, factor 0.1, offset 0.0
    let pressure2_raw = extract_signal(data, 16, 16)?;
    let engnexhstmnfldasltprssr2 = apply_scaling(pressure2_raw, 0.1, 0.0, false, 16);

    Ok((
        device_id,
        engnexhstmnfldasltprssr1,
        engnexhstmnfldasltprssr2,
    ))
}

// ============================================================================
// J1939 Message Encoder Re-exports
// ============================================================================

// Re-export J1939 encoder functions from modular structure
// Encoder functions are now generated directly on message types
// No separate encoder module re-exports needed

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_signal() {
        let data = [0b10101100, 0b11010011]; // Example data

        // Extract 2 bits starting at bit 0 (should be 0b00)
        assert_eq!(extract_signal(&data, 0, 2).unwrap(), 0b00);

        // Extract 2 bits starting at bit 2 (should be 0b11)
        assert_eq!(extract_signal(&data, 2, 2).unwrap(), 0b11);

        // Extract 4 bits starting at bit 4 (should be 0b1010)
        assert_eq!(extract_signal(&data, 4, 4).unwrap(), 0b1010);
    }

    #[test]
    fn test_pack_signal() {
        let mut data = [0u8; 2];

        // Pack 2 bits at position 0
        pack_signal(&mut data, 0, 2, 0b11).unwrap();
        assert_eq!(data[0] & 0b11, 0b11);

        // Pack 2 bits at position 2
        pack_signal(&mut data, 2, 2, 0b01).unwrap();
        assert_eq!((data[0] >> 2) & 0b11, 0b01);
    }

    #[test]
    fn test_scaling() {
        // Test factor 0.5, offset 0
        assert_eq!(apply_scaling(100, 0.5, 0.0, false, 16), 50.0);
        assert_eq!(apply_inverse_scaling(50.0, 0.5, 0.0, 16), 100);

        // Test factor 0.03125, offset -273 (temperature)
        let temp_c = apply_scaling(10000, 0.03125, -273.0, false, 16);
        assert!((temp_c - 39.5).abs() < 0.01); // 10000 * 0.03125 - 273 = 39.5
    }

    #[test]
    fn test_device_id_operations() {
        // Test with a PDU2 message (PF >= 240)
        let base_id = 0x18F00000; // PF = 0xF0 = 240, PDU2
        let device_id = DeviceId::from(0x42);

        let combined = embed_device_id(base_id, device_id, None);
        assert_eq!(combined, 0x18F00042);

        let extracted = extract_device_id(combined);
        assert_eq!(extracted, device_id);
    }

    #[test]
    fn test_pdu1_command_to_fan() {
        // EMP J1939 Command (PGN 32000) to fan
        let base_id = 0x187D0000; // PF=0x7D (125 < 240, PDU1)
        let device_id = DeviceId::from(0x82);
        let source = Some(0x02);

        let can_id = embed_device_id(base_id, device_id, source);

        assert_eq!(can_id, 0x187D8202, "Should be: TO 0x82 FROM 0x02");

        // Verify bit fields
        let priority = (can_id >> 26) & 0x07;
        let pf = (can_id >> 16) & 0xFF;
        let da = (can_id >> 8) & 0xFF;
        let sa = can_id & 0xFF;

        assert_eq!(priority, 6, "Priority should be 6");
        assert_eq!(pf, 0x7D, "PF should be 0x7D (125)");
        assert_eq!(da, 0x82, "Destination should be fan (0x82)");
        assert_eq!(sa, 0x02, "Source should be controller (0x02)");
    }

    #[test]
    fn test_pdu1_command_to_pump() {
        // Command to pump device
        let base_id = 0x187D0000;
        let device_id = DeviceId::from(0x88);
        let source = Some(0x02);

        let can_id = embed_device_id(base_id, device_id, source);

        assert_eq!(can_id, 0x187D8802, "Should be: TO 0x88 FROM 0x02");

        // Verify addressing
        let da = (can_id >> 8) & 0xFF;
        let sa = can_id & 0xFF;
        assert_eq!(da, 0x88, "Destination should be pump");
        assert_eq!(sa, 0x02, "Source should be controller");
    }

    #[test]
    fn test_pdu1_default_source() {
        // PDU1 without explicit source should use TEST_DEVICE (0x0F)
        let base_id = 0x187D0000;
        let device_id = DeviceId::from(0x82);

        let can_id = embed_device_id(base_id, device_id, None);

        assert_eq!(can_id, 0x187D820F, "Should default to source 0x0F");

        let sa = can_id & 0xFF;
        assert_eq!(sa, 0x0F, "Source should default to TEST_DEVICE");
    }

    #[test]
    fn test_pdu2_broadcast_from_fan() {
        // EMP J1939 Status (PGN 62320) from fan
        let base_id = 0x18F37000; // PF=0xF3 (243 >= 240, PDU2)
        let device_id = DeviceId::from(0x82);

        let can_id = embed_device_id(base_id, device_id, None);

        assert_eq!(can_id, 0x18F37082, "Should be: FROM 0x82");

        // Verify bit fields
        let pf = (can_id >> 16) & 0xFF;
        let ge = (can_id >> 8) & 0xFF;
        let sa = can_id & 0xFF;

        assert_eq!(pf, 0xF3, "PF should be 0xF3 (243)");
        assert_eq!(ge, 0x70, "Group extension should be 0x70 (part of PGN)");
        assert_eq!(sa, 0x82, "Source should be fan (0x82)");
    }

    #[test]
    fn test_pdu2_ignores_source_param() {
        // PDU2 should ignore source_addr parameter
        let base_id = 0x18F37000;
        let device_id = DeviceId::from(0x82);

        let can_id_none = embed_device_id(base_id, device_id, None);
        let can_id_some = embed_device_id(base_id, device_id, Some(0x99));

        assert_eq!(can_id_none, can_id_some, "PDU2 should ignore source param");
        assert_eq!(can_id_none, 0x18F37082);
    }

    #[test]
    fn test_pdu1_pdu2_boundary() {
        // Test the boundary between PDU1 (PF < 240) and PDU2 (PF >= 240)

        // PF = 239 (0xEF) - should be PDU1
        let pdu1_boundary = 0x18EF0000;
        let can_id = embed_device_id(pdu1_boundary, DeviceId::from(0x82), Some(0x02));
        assert_eq!(can_id, 0x18EF8202, "PF=239 should be PDU1");

        // PF = 240 (0xF0) - should be PDU2
        let pdu2_boundary = 0x18F00000;
        let can_id = embed_device_id(pdu2_boundary, DeviceId::from(0x82), None);
        assert_eq!(can_id, 0x18F00082, "PF=240 should be PDU2");
    }

    #[test]
    fn test_pdu1_preserves_priority_and_dp() {
        // Test that PDU1 encoding preserves priority and data page bits

        // Priority 3, DP=1, EDP=0, PF=0x7D
        let base_id = 0x0D7D0000; // 0000 1101 0111 1101 0000 0000 0000 0000
        let can_id = embed_device_id(base_id, DeviceId::from(0x82), Some(0x02));

        assert_eq!(can_id, 0x0D7D8202);

        let priority = (can_id >> 26) & 0x07;
        let dp = (can_id >> 24) & 0x01;
        let pf = (can_id >> 16) & 0xFF;

        assert_eq!(priority, 3, "Priority should be preserved");
        assert_eq!(dp, 1, "Data page should be preserved");
        assert_eq!(pf, 0x7D, "PF should be preserved");
    }

    #[test]
    fn test_pdu2_preserves_group_extension() {
        // Test that PDU2 encoding preserves the Group Extension byte

        let base_id = 0x18F37000; // GE = 0x70
        let can_id = embed_device_id(base_id, DeviceId::from(0x82), None);

        let ge = (can_id >> 8) & 0xFF;
        assert_eq!(ge, 0x70, "Group extension should be preserved");
    }

    #[test]
    fn test_mcm_decode() {
        // Create test data with known values
        let mut data = [0u8; 8];

        // Set MCM_OnOffDirectionCommand to 1 (bits 0-1)
        data[0] |= 1;

        // Set MCM_PowerHoldCommand to 1 (bits 2-3)
        data[0] |= 1 << 2;

        // Set MCM_MotorSpeedCommand to 2000 raw (1000 RPM at 0.5 factor)
        // Bits 8-23
        let speed_raw = 2000u16;
        data[1] = speed_raw as u8;
        data[2] = (speed_raw >> 8) as u8;

        // MCM base CAN ID with device ID 0x42 embedded
        let can_id = embed_device_id(2565865214, DeviceId::from(0x42), None);

        let result = decode_mcm_motor_command_message(can_id, &data);

        // Should succeed with valid data
        assert!(result.is_ok());
        let (device_id, on_off, power_hold, motor_speed, _percent_speed) = result.unwrap();

        assert_eq!(device_id, DeviceId::from(0x42));
        assert_eq!(on_off, 1);
        assert_eq!(power_hold, 1);
        assert_eq!(motor_speed, 1000.0); // 2000 * 0.5
    }

    #[test]
    fn test_msm2_encode() {
        let result = encode_msm2_motor_status_message2(
            DeviceId::from(0x42),
            1,      // on_off_direction
            0,      // controller_status (normal)
            1,      // command_status
            1500.0, // motor_speed
            25.0,   // external_temp
            750.0,  // motor_power
            0,      // service_indicator
            0,      // operation_status
        );

        assert!(result.is_ok());
        let (can_id, data) = result.unwrap();

        // Check CAN ID includes device ID
        assert_eq!(extract_device_id(can_id), DeviceId::from(0x42));

        // Verify some basic signal packing
        assert_eq!(data[0] & 0b11, 1); // on_off_direction in bits 0-1

        // Motor speed should be encoded as 3000 raw (1500 * 2)
        let speed_raw = ((data[2] as u16) << 8) | (data[1] as u16);
        assert_eq!(speed_raw, 3000);
    }
}
