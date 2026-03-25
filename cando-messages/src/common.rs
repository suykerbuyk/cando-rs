//! Common types and error handling for CAN message processing.
//!
//! This module provides the foundational types used throughout the cando_messages
//! crate, including device identifiers, validation errors, scaling helpers, and
//! the base traits that all CAN messages implement.
//!
//! # Examples
//!
//! ```rust
//! use cando_messages::common::{DeviceId, Percentage, ValidationError};
//!
//! // Working with device IDs
//! let device = DeviceId::from(0x42);
//! assert_eq!(u8::from(device), 0x42);
//!
//! // Using scaled percentage values
//! let percent = Percentage::new(50.0)?;
//! assert_eq!(percent.value(), 50.0);
//! # Ok::<(), ValidationError>(())
//! ```

use thiserror::Error;

//
// CAN ID Bit Manipulation Constants
//
// These constants are used throughout the cando-rs workspace for consistent
// CAN ID handling, device addressing, and J1939 compliance. They are centralized
// here to avoid duplication and ensure consistency across all tools.
//

/// 29-bit extended frame mask - extracts only the valid CAN ID bits
///
/// CAN extended frames use 29-bit identifiers (0x00000000 to 0x1FFFFFFF).
/// This mask ensures CAN IDs are within the valid extended frame range.
pub const CAN_EFF_MASK: u32 = 0x1FFFFFFF;

/// Base CAN ID mask - masks out the device ID in lower 8 bits
///
/// J1939-style addressing embeds the device/source address in the lower 8 bits.
/// This mask extracts the base message ID without the device-specific addressing.
pub const CAN_BASE_ID_MASK: u32 = 0xFFFFFF00;

/// Combined 29-bit + base ID mask - extracts base ID within 29-bit limit
///
/// Combines CAN_EFF_MASK and CAN_BASE_ID_MASK to extract the base ID
/// while ensuring compliance with 29-bit extended frame limits.
/// Avoids the need for double-masking operations.
pub const CAN_BASE_ID_29BIT_MASK: u32 = 0x1FFFFF00;

/// Device ID mask - extracts the device ID from lower 8 bits
///
/// J1939 and similar protocols embed the source/device address in the
/// lower 8 bits of the CAN identifier. This mask extracts that address.
pub const CAN_DEVICE_ID_MASK: u32 = 0xFF;

/// Maximum standard frame ID (11-bit)
///
/// Standard CAN frames use 11-bit identifiers (0x000 to 0x7FF).
/// IDs above this value indicate extended (29-bit) frames.
pub const CAN_STD_ID_MAX: u32 = 0x7FF;

/// PDU1 mask - masks out both destination and source address bytes
///
/// For J1939 PDU1 messages (PF < 240), both bytes 1 (destination) and 0 (source)
/// are addressing information and must be stripped to get the base message ID.
pub const CAN_PDU1_BASE_MASK: u32 = 0xFFFF0000;

/// Determine if a J1939 CAN ID represents a PDU1 (destination-specific) message.
///
/// PDU1 messages have PF (PDU Format) values less than 240. In these messages,
/// the PS byte (bits 15-8) contains the destination address rather than being
/// part of the PGN.
///
/// # Arguments
/// * `can_id` - The CAN ID to check (should be masked with CAN_EFF_MASK first)
///
/// # Returns
/// `true` if the message is PDU1 (destination-specific), `false` for PDU2 (broadcast)
///
/// # Example
/// ```
/// use cando_messages::common::is_j1939_pdu1;
///
/// let pdu1_id = 0x187D8202;  // PGN 32000, PF=0x7D (125 < 240)
/// assert!(is_j1939_pdu1(pdu1_id));
///
/// let pdu2_id = 0x18F37082;  // PGN 62320, PF=0xF3 (243 >= 240)
/// assert!(!is_j1939_pdu1(pdu2_id));
/// ```
pub fn is_j1939_pdu1(can_id: u32) -> bool {
    let pf = (can_id >> 16) & 0xFF;
    pf < 240
}

/// Get the base message ID for J1939 message lookup, accounting for PDU type.
///
/// This function correctly handles both PDU1 and PDU2 messages:
/// - **PDU1** (PF < 240): Strips both destination (byte 1) and source (byte 0)
/// - **PDU2** (PF >= 240): Strips only source (byte 0), preserves PS (byte 1) as part of PGN
///
/// # Arguments
/// * `can_id` - The CAN ID to process (will be masked with CAN_EFF_MASK)
///
/// # Returns
/// Base message ID suitable for lookup in message databases
///
/// # Example
/// ```
/// use cando_messages::common::get_j1939_base_id;
///
/// // PDU1: Command (PGN 32000, dest 0x82, src 0x02)
/// let pdu1_id = 0x187D8202;
/// assert_eq!(get_j1939_base_id(pdu1_id), 0x187D0000);
///
/// // PDU2: Status (PGN 62320, src 0x82)
/// let pdu2_id = 0x18F37082;
/// assert_eq!(get_j1939_base_id(pdu2_id), 0x18F37000);
/// ```
pub fn get_j1939_base_id(can_id: u32) -> u32 {
    let masked_id = can_id & CAN_EFF_MASK;
    if is_j1939_pdu1(masked_id) {
        // PDU1: Strip both destination and source address bytes
        masked_id & CAN_PDU1_BASE_MASK
    } else {
        // PDU2: Strip only source address byte (PS is part of PGN)
        masked_id & CAN_BASE_ID_MASK
    }
}

/// J1939 PDU1/PDU2 threshold value
///
/// In J1939, the PF (PDU Format) byte determines message addressing:
/// - PF < 240 (0xF0): PDU1 (destination-specific, byte 1 is destination address)
/// - PF >= 240 (0xF0): PDU2 (broadcast, byte 1 is group extension/part of PGN)
pub const J1939_PDU1_THRESHOLD: u8 = 240;

/// PDU1 base ID mask - masks out both destination and source addresses
///
/// For J1939 PDU1 messages (destination-specific), both the destination address
/// (byte 1) and source address (byte 0) are part of the CAN ID but not part of
/// the base message definition. This mask strips both address bytes.
pub const J1939_PDU1_BASE_MASK: u32 = 0xFFFF0000;

/// Errors that occur during message validation.
///
/// These errors represent validation failures that can occur when creating
/// or processing CAN message data, such as values outside acceptable ranges
/// or invalid enumeration values.
///
/// # Examples
///
/// ```rust
/// use cando_messages::common::{ValidationError, Percentage};
///
/// // Range validation error
/// let result = Percentage::new(200.0);
/// match result {
///     Err(ValidationError::OutOfRange { value, min, max }) => {
///         assert_eq!(value, 200.0);
///         assert_eq!(min, 0.0);
///         assert_eq!(max, 125.0);
///     }
///     _ => panic!("Expected range error"),
/// }
/// ```
#[derive(Error, Debug)]
pub enum ValidationError {
    /// A value is outside the acceptable range for a signal.
    #[error("Value {value} out of range [{min}, {max}]")]
    OutOfRange { value: f64, min: f64, max: f64 },

    /// An invalid enumeration value was encountered.
    #[error("Invalid enum value {value} for type {type_name}")]
    InvalidEnum { value: u64, type_name: &'static str },

    /// An invalid device ID was provided.
    #[error("Invalid device ID {value:02X}")]
    InvalidDeviceId { value: u8 },
}

/// Errors that occur during CAN message decoding.
///
/// These errors represent failures that can occur when attempting to decode
/// raw CAN frame data into structured message types.
///
/// # Examples
///
/// ```rust
/// use cando_messages::common::DecodeError;
///
/// // Invalid CAN ID error
/// let error = DecodeError::InvalidCanId {
///     expected: 0x12345600,
///     actual: 0x87654321
/// };
/// assert!(error.to_string().contains("Invalid CAN ID"));
/// ```
#[derive(Error, Debug)]
pub enum DecodeError {
    /// The CAN ID doesn't match the expected value for this message type.
    #[error("Invalid CAN ID: expected {expected:08X}, got {actual:08X}")]
    InvalidCanId { expected: u32, actual: u32 },

    /// The data length doesn't match the expected length for this message type.
    #[error("Invalid data length: expected {expected}, got {actual}")]
    InvalidLength { expected: u8, actual: u8 },

    /// A validation error occurred during decoding.
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),

    /// Not enough data available to extract the required signal.
    #[error("Insufficient data: required {required} bytes, available {available}")]
    InsufficientData { required: usize, available: usize },

    /// The PGN doesn't match the expected value for this message type.
    #[error("Invalid PGN: expected {expected:04X}, got {actual:04X}")]
    InvalidPGN { expected: u32, actual: u32 },

    /// An unsupported multiplexer value was encountered.
    #[error("Unsupported multiplexer value {mux}")]
    UnsupportedMux { mux: u64 },
}

/// Encoding error types for message serialization operations.
///
/// These errors occur when converting from high-level message structs to
/// raw CAN data bytes. Common causes include parameter validation failures
/// and invalid field combinations.
///
/// # Examples
///
/// ```rust
/// use cando_messages::common::EncodeError;
///
/// // Parameter validation error
/// let error = EncodeError::InvalidParameter("SPN 524288 exceeds 19-bit maximum".to_string());
/// assert!(error.to_string().contains("Invalid parameter"));
/// ```
#[derive(Error, Debug)]
pub enum EncodeError {
    /// A parameter value is outside the valid range for the message field.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// The device ID is not valid for this message type.
    #[error("Invalid device ID: {device_id:02X}")]
    InvalidDeviceId { device_id: u8 },

    /// A validation error occurred during encoding.
    #[error("Validation failed: {0}")]
    Validation(#[from] ValidationError),

    /// The data length would exceed the maximum CAN frame size.
    #[error("Data length {length} exceeds maximum CAN frame size (8 bytes)")]
    DataTooLong { length: usize },

    /// Required field is missing or has invalid value.
    #[error("Required field '{field}' is missing or invalid")]
    MissingField { field: String },

    /// SPN (Suspect Parameter Number) value is outside valid range (0-524287).
    #[error("Invalid SPN: {spn} (must be 0-524287)")]
    InvalidSPN { spn: u64 },

    /// FMI (Failure Mode Identifier) value is outside valid range (0-31).
    #[error("Invalid FMI: {fmi} (must be 0-31)")]
    InvalidFMI { fmi: u64 },

    /// Occurrence count value is outside valid range (0-127).
    #[error("Invalid occurrence count: {count} (must be 0-127)")]
    InvalidOccurrenceCount { count: u64 },
}

impl From<DecodeError> for EncodeError {
    fn from(err: DecodeError) -> Self {
        match err {
            DecodeError::Validation(v) => EncodeError::Validation(v),
            _ => EncodeError::InvalidParameter(err.to_string()),
        }
    }
}

impl From<EncodeError> for DecodeError {
    fn from(err: EncodeError) -> Self {
        match err {
            EncodeError::Validation(v) => DecodeError::Validation(v),
            _ => DecodeError::Validation(ValidationError::InvalidEnum {
                value: 0,
                type_name: "EncodeError",
            }),
        }
    }
}

/// J1939 device address (8-bit source address field in CAN ID)
///
/// Supports the full J1939 address space as defined in SAE J1939-81:
/// - `0x00-0xF7`: Normal device addresses (248 addresses)
/// - `0xF8`: Proprietary use
/// - `0xF9-0xFD`: Diagnostic tools / external test equipment (5 addresses)
/// - `0xFE`: Null address (address claiming not complete)
/// - `0xFF`: Global/broadcast address
///
/// # J1939 Standard Compliance
///
/// This type supports all 256 possible J1939 device addresses as required by
/// SAE J1939-81 section 4.1 (Network Management). The address space is divided into:
///
/// ## Normal Device Addresses (0x00-0xF7)
/// These 248 addresses are used for standard network devices. Devices claim
/// addresses during network initialization via the address claiming procedure.
///
/// ## Special Addresses (0xF8-0xFF)
/// - **0xF8**: Reserved for proprietary use
/// - **0xF9-0xFD**: Off-board diagnostic tools and external test equipment
/// - **0xFE**: Null address (used during address claiming before a device has
///   claimed its network address)
/// - **0xFF**: Global/broadcast address (used for messages intended for all devices)
///
/// # Examples
///
/// ```
/// use cando_messages::common::DeviceId;
///
/// // Create from raw value
/// let device = DeviceId::from(0x42);
/// assert_eq!(device.as_u8(), 0x42);
///
/// // Use named constants
/// let test_device = DeviceId::TEST_DEVICE;
/// assert_eq!(test_device.as_u8(), 0x0F);
///
/// // Check address category
/// let diag_tool = DeviceId::DIAGNOSTIC_TOOL_1;
/// assert!(diag_tool.is_diagnostic_tool());
/// assert!(diag_tool.is_special());
///
/// let broadcast = DeviceId::BROADCAST;
/// assert!(broadcast.is_broadcast());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct DeviceId(u8);

impl DeviceId {
    // ============================================================================
    // Special Address Constants (J1939-81 Standard)
    // ============================================================================

    /// Test/diagnostic device for integration testing (0x0F)
    ///
    /// This is a non-standard address chosen for cando integration tests.
    /// It's in the normal device range but unlikely to conflict with real devices.
    pub const TEST_DEVICE: Self = Self(0x0F);

    /// J1939 Diagnostic Tool #1 (0xF9) - SAE J1939-81 section 4.4.2
    ///
    /// Off-board diagnostic tools use addresses 0xF9-0xFD. This allows
    /// external test equipment to send diagnostic requests and commands.
    pub const DIAGNOSTIC_TOOL_1: Self = Self(0xF9);

    /// J1939 Diagnostic Tool #2 (0xFA) - SAE J1939-81 section 4.4.2
    pub const DIAGNOSTIC_TOOL_2: Self = Self(0xFA);

    /// J1939 Diagnostic Tool #3 (0xFB) - SAE J1939-81 section 4.4.2
    pub const DIAGNOSTIC_TOOL_3: Self = Self(0xFB);

    /// J1939 Diagnostic Tool #4 (0xFC) - SAE J1939-81 section 4.4.2
    pub const DIAGNOSTIC_TOOL_4: Self = Self(0xFC);

    /// J1939 Diagnostic Tool #5 (0xFD) - SAE J1939-81 section 4.4.2
    pub const DIAGNOSTIC_TOOL_5: Self = Self(0xFD);

    /// Null address (0xFE) - SAE J1939-81 section 4.5.2
    ///
    /// Used by devices during address claiming before they have successfully
    /// claimed a network address. Devices must not send normal messages from
    /// this address.
    pub const NULL_ADDRESS: Self = Self(0xFE);

    /// Global/broadcast address (0xFF) - SAE J1939-21 section 5.1.2
    ///
    /// Used as the destination address for messages intended for all devices
    /// on the network. Some PGNs require this as the destination address.
    pub const BROADCAST: Self = Self(0xFF);

    /// Proprietary address (0xF8) - SAE J1939-81 section 4.4.3
    ///
    /// Reserved for proprietary use by manufacturers.
    pub const PROPRIETARY: Self = Self(0xF8);

    // ============================================================================
    // Constructors and Accessors
    // ============================================================================

    /// Create a DeviceId from a raw u8 value.
    ///
    /// All values 0x00-0xFF are valid J1939 addresses per SAE J1939-81.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// let device = DeviceId::new(0x82);
    /// assert_eq!(device.as_u8(), 0x82);
    ///
    /// // All values are valid
    /// let any_device = DeviceId::new(0x7F);
    /// assert!(any_device.is_normal_device());
    /// ```
    pub const fn new(id: u8) -> Self {
        Self(id)
    }

    /// Get the raw u8 device address.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// let device = DeviceId::DIAGNOSTIC_TOOL_1;
    /// assert_eq!(device.as_u8(), 0xF9);
    /// ```
    pub const fn as_u8(self) -> u8 {
        self.0
    }

    /// Converts DeviceId to its u8 representation.
    ///
    /// **Deprecated**: Use `as_u8()` instead. This method exists for backward
    /// compatibility during the enum-to-newtype migration.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// let device = DeviceId::from(0x42);
    /// assert_eq!(device.to_u8(), 0x42);
    /// ```
    #[deprecated(since = "0.1.0", note = "Use `as_u8()` instead")]
    pub const fn to_u8(self) -> u8 {
        self.0
    }

    // ============================================================================
    // Category Check Methods
    // ============================================================================

    /// Check if this is a normal device address (0x00-0xF7).
    ///
    /// Normal devices are standard network participants that claim addresses
    /// during network initialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// assert!(DeviceId::new(0x42).is_normal_device());
    /// assert!(DeviceId::new(0x7F).is_normal_device());
    /// assert!(!DeviceId::DIAGNOSTIC_TOOL_1.is_normal_device());
    /// ```
    pub const fn is_normal_device(self) -> bool {
        self.0 <= 0xF7
    }

    /// Check if this is a diagnostic tool address (0xF9-0xFD).
    ///
    /// These addresses are reserved for off-board diagnostic tools and
    /// external test equipment per SAE J1939-81 section 4.4.2.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// assert!(DeviceId::DIAGNOSTIC_TOOL_1.is_diagnostic_tool());
    /// assert!(DeviceId::new(0xFB).is_diagnostic_tool());
    /// assert!(!DeviceId::new(0x42).is_diagnostic_tool());
    /// ```
    pub const fn is_diagnostic_tool(self) -> bool {
        matches!(self.0, 0xF9..=0xFD)
    }

    /// Check if this is the null address (0xFE).
    ///
    /// The null address is used during address claiming before a device
    /// has successfully claimed a network address (SAE J1939-81 section 4.5.2).
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// assert!(DeviceId::NULL_ADDRESS.is_null_address());
    /// assert!(!DeviceId::new(0x42).is_null_address());
    /// ```
    pub const fn is_null_address(self) -> bool {
        self.0 == 0xFE
    }

    /// Check if this is the broadcast address (0xFF).
    ///
    /// The broadcast address is used for messages intended for all devices
    /// on the network (SAE J1939-21 section 5.1.2).
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// assert!(DeviceId::BROADCAST.is_broadcast());
    /// assert!(!DeviceId::new(0x42).is_broadcast());
    /// ```
    pub const fn is_broadcast(self) -> bool {
        self.0 == 0xFF
    }

    /// Check if this is a special address (0xF8-0xFF).
    ///
    /// Special addresses include proprietary, diagnostic tools, null address,
    /// and broadcast per SAE J1939-81 section 4.4.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// assert!(DeviceId::DIAGNOSTIC_TOOL_1.is_special());
    /// assert!(DeviceId::BROADCAST.is_special());
    /// assert!(!DeviceId::new(0x42).is_special());
    /// ```
    pub const fn is_special(self) -> bool {
        self.0 >= 0xF8
    }

    /// Check if this is the proprietary address (0xF8).
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// assert!(DeviceId::PROPRIETARY.is_proprietary());
    /// assert!(!DeviceId::new(0x42).is_proprietary());
    /// ```
    pub const fn is_proprietary(self) -> bool {
        self.0 == 0xF8
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl From<u8> for DeviceId {
    /// Convert a raw u8 value to DeviceId.
    ///
    /// All u8 values are valid J1939 addresses.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// let device = DeviceId::from(0x82);
    /// assert_eq!(device.as_u8(), 0x82);
    /// ```
    fn from(id: u8) -> Self {
        Self::new(id)
    }
}

impl From<DeviceId> for u8 {
    /// Convert DeviceId to its raw u8 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// let device = DeviceId::from(0x80);
    /// let value: u8 = device.into();
    /// assert_eq!(value, 0x80);
    /// ```
    fn from(id: DeviceId) -> u8 {
        id.as_u8()
    }
}

impl std::fmt::Display for DeviceId {
    /// Format DeviceId for display with semantic names where applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// assert_eq!(format!("{}", DeviceId::DIAGNOSTIC_TOOL_1), "DiagnosticTool1");
    /// assert_eq!(format!("{}", DeviceId::BROADCAST), "Broadcast");
    /// assert_eq!(format!("{}", DeviceId::new(0x42)), "Device42");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0xF9 => write!(f, "DiagnosticTool1"),
            0xFA => write!(f, "DiagnosticTool2"),
            0xFB => write!(f, "DiagnosticTool3"),
            0xFC => write!(f, "DiagnosticTool4"),
            0xFD => write!(f, "DiagnosticTool5"),
            0xFE => write!(f, "NullAddress"),
            0xFF => write!(f, "Broadcast"),
            0xF8 => write!(f, "Proprietary"),
            id => write!(f, "Device{:02X}", id),
        }
    }
}

impl std::fmt::LowerHex for DeviceId {
    /// Format DeviceId as lowercase hexadecimal.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// let device = DeviceId::from(0x80);
    /// assert_eq!(format!("{:x}", device), "80");
    /// assert_eq!(format!("{:#x}", device), "0x80");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl std::fmt::UpperHex for DeviceId {
    /// Format DeviceId as uppercase hexadecimal.
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_messages::common::DeviceId;
    ///
    /// let device = DeviceId::DIAGNOSTIC_TOOL_1;
    /// assert_eq!(format!("{:X}", device), "F9");
    /// assert_eq!(format!("{:#X}", device), "0xF9");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.0, f)
    }
}

/// A newtype wrapper for percentage values with automatic validation.
///
/// Represents percentage values in the range 0.0 to 125.0%, with a scale
/// factor of 0.5 (meaning raw CAN values are divided by 2 to get percentages).
///
/// # Examples
///
/// ```rust
/// use cando_messages::common::{Percentage, ValidationError};
///
/// // Create valid percentage
/// let percent = Percentage::new(75.0)?;
/// assert_eq!(percent.value(), 75.0);
///
/// // Values outside range are rejected
/// let too_high = Percentage::new(150.0);
/// assert!(too_high.is_err());
///
/// let too_low = Percentage::new(-5.0);
/// assert!(too_low.is_err());
/// # Ok::<(), ValidationError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage(f64);

impl Percentage {
    /// Minimum allowed percentage value
    pub const MIN: f64 = 0.0;
    /// Maximum allowed percentage value
    pub const MAX: f64 = 125.0;
    /// Scale factor used in CAN encoding/decoding
    pub const SCALE: f64 = 0.5;

    /// Creates a new percentage value with validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cando_messages::common::Percentage;
    ///
    /// let valid = Percentage::new(50.0)?;
    /// assert_eq!(valid.value(), 50.0);
    ///
    /// // Out of range values return errors
    /// assert!(Percentage::new(-1.0).is_err());
    /// assert!(Percentage::new(126.0).is_err());
    /// # Ok::<(), cando_messages::common::ValidationError>(())
    /// ```
    /// Creates a new motor speed value with validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cando_messages::common::MotorSpeed;
    ///
    /// let speed = MotorSpeed::new(2500.0)?;
    /// assert_eq!(speed.rpm(), 2500.0);
    ///
    /// // Negative speeds are invalid
    /// assert!(MotorSpeed::new(-100.0).is_err());
    /// # Ok::<(), cando_messages::common::ValidationError>(())
    /// ```
    pub fn new(value: f64) -> Result<Self, ValidationError> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(ValidationError::OutOfRange {
                value,
                min: Self::MIN,
                max: Self::MAX,
            })
        }
    }

    /// Returns the percentage value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cando_messages::common::Percentage;
    ///
    /// let percent = Percentage::new(75.5)?;
    /// assert_eq!(percent.value(), 75.5);
    /// # Ok::<(), cando_messages::common::ValidationError>(())
    /// ```
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// A newtype wrapper for motor speed values with automatic validation.
///
/// Represents motor speed in RPM with a range from 0.0 to 32127.5 RPM,
/// using a scale factor of 0.5 for CAN encoding/decoding.
///
/// # Examples
///
/// ```rust
/// use cando_messages::common::{MotorSpeed, ValidationError};
///
/// // Create valid motor speed
/// let speed = MotorSpeed::new(1500.0)?;
/// assert_eq!(speed.rpm(), 1500.0);
///
/// // Maximum speed
/// let max_speed = MotorSpeed::new(32127.5)?;
/// assert_eq!(max_speed.rpm(), 32127.5);
///
/// // Invalid speeds are rejected
/// let too_fast = MotorSpeed::new(50000.0);
/// assert!(too_fast.is_err());
/// # Ok::<(), ValidationError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorSpeed(f64);

impl MotorSpeed {
    /// Minimum allowed motor speed in RPM
    pub const MIN: f64 = 0.0;
    /// Maximum allowed motor speed in RPM
    pub const MAX: f64 = 32127.5;
    /// Scale factor used in CAN encoding/decoding
    pub const SCALE: f64 = 0.5;

    pub fn new(value: f64) -> Result<Self, ValidationError> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(ValidationError::OutOfRange {
                value,
                min: Self::MIN,
                max: Self::MAX,
            })
        }
    }

    /// Returns the motor speed in RPM.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cando_messages::common::MotorSpeed;
    ///
    /// let speed = MotorSpeed::new(3000.0)?;
    /// assert_eq!(speed.rpm(), 3000.0);
    /// # Ok::<(), cando_messages::common::ValidationError>(())
    /// ```
    pub fn rpm(&self) -> f64 {
        self.0
    }
}

/// Base trait for all CAN messages.
///
/// This trait defines the common interface that all CAN message types
/// must implement, providing methods for encoding, decoding, and
/// accessing message metadata.
///
/// # Examples
///
/// ```rust
/// use cando_messages::common::{CanMessage, DecodeError};
///
/// // Example implementation (simplified)
/// struct ExampleMessage {
///     data: u32,
/// }
///
/// impl CanMessage for ExampleMessage {
///     const BASE_ID: u32 = 0x123;
///     const NAME: &'static str = "ExampleMessage";
///     const DLC: u8 = 8;
///
///     fn can_id(&self) -> u32 {
///         Self::BASE_ID
///     }
///
///     fn decode(_id: u32, _data: &[u8]) -> Result<Self, DecodeError> {
///         Ok(Self { data: 0 })
///     }
///
///     fn encode(&self) -> Vec<u8> {
///         vec![0; Self::DLC as usize]
///     }
/// }
///
/// // Usage
/// let msg = ExampleMessage { data: 42 };
/// assert_eq!(msg.can_id(), 0x123);
/// assert_eq!(ExampleMessage::NAME, "ExampleMessage");
/// ```
pub trait CanMessage {
    /// The base CAN identifier for this message type
    const BASE_ID: u32;
    /// Human-readable name of the message type
    const NAME: &'static str;
    /// Data Length Code (number of data bytes)
    const DLC: u8;

    /// Returns the complete CAN ID for this message instance
    fn can_id(&self) -> u32;

    /// Decodes a message from raw CAN frame data
    fn decode(id: u32, data: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized;

    /// Encodes the message to raw CAN frame data
    fn encode(&self) -> Vec<u8>;
}

/// Trait for J1939-specific CAN messages.
///
/// Extends the basic `CanMessage` trait with J1939-specific functionality
/// such as priority, Parameter Group Number (PGN), and addressing.
///
/// # Examples
///
/// ```rust
/// use cando_messages::common::{CanMessage, J1939Message, DecodeError};
///
/// struct J1939Example {
///     source: u8,
/// }
///
/// impl CanMessage for J1939Example {
///     const BASE_ID: u32 = 0x18FF0000;
///     const NAME: &'static str = "J1939Example";
///     const DLC: u8 = 8;
///
///     fn can_id(&self) -> u32 { Self::BASE_ID | self.source as u32 }
///     fn decode(_id: u32, _data: &[u8]) -> Result<Self, DecodeError> {
///         Ok(Self { source: 0 })
///     }
///     fn encode(&self) -> Vec<u8> { vec![0; 8] }
/// }
///
/// impl J1939Message for J1939Example {
///     fn priority(&self) -> u8 { 6 }
///     fn pgn(&self) -> u32 { 0xFF00 }
///     fn source_address(&self) -> u8 { self.source }
///     fn destination_address(&self) -> Option<u8> { None }
/// }
///
/// let msg = J1939Example { source: 0x42 };
/// assert_eq!(msg.priority(), 6);
/// assert!(msg.is_broadcast());
/// ```
pub trait J1939Message: CanMessage {
    /// Returns the J1939 priority (0-7, lower is higher priority)
    fn priority(&self) -> u8;

    /// Returns the Parameter Group Number (PGN)
    fn pgn(&self) -> u32;

    /// Returns the source address
    fn source_address(&self) -> u8;

    /// Returns the destination address, or None for broadcast messages
    fn destination_address(&self) -> Option<u8>;

    /// Returns true if this is a broadcast message (no specific destination)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cando_messages::common::{CanMessage, J1939Message, DecodeError};
    /// # struct MockMessage;
    /// # impl CanMessage for MockMessage {
    /// #     const BASE_ID: u32 = 0; const NAME: &'static str = "Mock"; const DLC: u8 = 8;
    /// #     fn can_id(&self) -> u32 { 0 } fn decode(_: u32, _: &[u8]) -> Result<Self, DecodeError> { Ok(Self) } fn encode(&self) -> Vec<u8> { vec![] }
    /// # }
    /// # impl J1939Message for MockMessage {
    /// #     fn priority(&self) -> u8 { 6 } fn pgn(&self) -> u32 { 0 } fn source_address(&self) -> u8 { 0 }
    /// #     fn destination_address(&self) -> Option<u8> { None }
    /// # }
    /// let msg = MockMessage;
    /// assert!(msg.is_broadcast());  // destination_address() returns None
    /// ```
    fn is_broadcast(&self) -> bool {
        self.destination_address().is_none()
    }
}
