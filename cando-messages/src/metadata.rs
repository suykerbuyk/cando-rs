//! Compile-time metadata for CAN messages and signals
//!
//! This module provides metadata structures that describe the properties of CAN
//! messages and signals as defined in DBC files. Unlike runtime DBC parsing, this
//! metadata is generated at compile-time and embedded as static data.
//!
//! ## Purpose
//!
//! The metadata system enables CLI tools and introspection utilities to:
//! - Display message and signal information without runtime DBC parsing
//! - Enumerate all messages in a protocol
//! - Access signal properties (bit positions, scaling, units, etc.)
//! - Look up value descriptions (enumerations)
//!
//! ## Usage Example
//!
//! ```rust
//! use cando_messages::metadata::{HasMetadata, ProtocolMetadata};
//! use cando_messages::j1939::WAND;
//!
//! // Access message metadata via trait
//! let msg_meta = WAND::metadata();
//! println!("Message: {}", msg_meta.name);
//! println!("CAN ID: 0x{:X}", msg_meta.can_id);
//! println!("Signals: {}", msg_meta.signals.len());
//!
//! // Iterate over signals
//! for signal in msg_meta.signals {
//!     println!("  {}: {} bits, factor={}, unit={}",
//!         signal.name, signal.signal_size, signal.factor, signal.unit);
//! }
//! ```
//!
//! ## Protocol-Level Metadata
//!
//! ```rust
//! use cando_messages::j1939;
//!
//! // Access protocol-level metadata constant
//! let protocol = &j1939::J1939_METADATA;
//! println!("Protocol: {}", protocol.name);
//! println!("Messages: {}", protocol.messages.len());
//!
//! // Enumerate all messages in protocol
//! for msg_meta in protocol.messages {
//!     println!("  {}: {} signals", msg_meta.name, msg_meta.signals.len());
//! }
//! ```

/// Byte order (endianness) for signal encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    /// Little-endian byte order (Intel format)
    LittleEndian,
    /// Big-endian byte order (Motorola format)
    BigEndian,
}

/// Signal value type (signed or unsigned)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    /// Unsigned integer value
    Unsigned,
    /// Signed integer value (two's complement)
    Signed,
}

/// Metadata describing a CAN signal
///
/// Contains all the information needed to understand and process a signal
/// as defined in the DBC file, including bit layout, scaling, and value
/// descriptions.
#[derive(Debug, Clone, Copy)]
pub struct SignalMetadata {
    /// Signal name as defined in DBC file
    pub name: &'static str,

    /// Starting bit position in the CAN message data
    ///
    /// For little-endian signals, this is the bit position of the LSB.
    /// For big-endian signals, this is the bit position of the MSB.
    pub start_bit: u64,

    /// Size of the signal in bits
    pub signal_size: u64,

    /// Byte order (endianness) of the signal
    pub byte_order: ByteOrder,

    /// Value type (signed or unsigned)
    pub value_type: ValueType,

    /// Scaling factor applied to raw value
    ///
    /// Physical value = (raw_value * factor) + offset
    pub factor: f64,

    /// Offset applied after scaling
    ///
    /// Physical value = (raw_value * factor) + offset
    pub offset: f64,

    /// Minimum valid value (in physical units)
    pub min: f64,

    /// Maximum valid value (in physical units)
    pub max: f64,

    /// Engineering unit for the signal (e.g., "rpm", "°C", "%")
    pub unit: &'static str,

    /// Value descriptions (enumeration mappings)
    ///
    /// Maps raw integer values to descriptive strings.
    /// Example: [(0, "Off"), (1, "On"), (2, "Error")]
    pub value_descriptions: &'static [(u64, &'static str)],

    /// Optional comment or description for the signal
    pub comment: &'static str,
}

/// Metadata describing a CAN message
///
/// Contains information about a complete CAN message including its ID,
/// size, and all signals it contains.
#[derive(Debug, Clone, Copy)]
pub struct MessageMetadata {
    /// Message name as defined in DBC file
    pub name: &'static str,

    /// CAN identifier (11-bit or 29-bit extended)
    ///
    /// For messages with device ID embedding (J1939), this is the base ID
    /// before the device ID is inserted.
    pub can_id: u32,

    /// Data Length Code - number of data bytes
    pub dlc: u8,

    /// Array of signals contained in this message
    pub signals: &'static [SignalMetadata],

    /// Whether this message uses multiplexing
    ///
    /// Multiplexed messages have different signal layouts depending on
    /// a multiplexer signal value (similar to a switch statement).
    pub is_multiplexed: bool,

    /// Optional comment or description for the message
    pub comment: &'static str,

    /// Transmitter node name (if specified in DBC)
    pub transmitter: &'static str,
}

/// Protocol-level metadata container
///
/// Aggregates all messages for a complete protocol (e.g., J1939).
/// This is typically exposed as a module-level constant like `J1939_METADATA`.
#[derive(Debug, Clone, Copy)]
pub struct ProtocolMetadata {
    /// Protocol name (e.g., "J1939")
    pub name: &'static str,

    /// Protocol version string (if available)
    pub version: &'static str,

    /// Array of all message metadata in this protocol
    pub messages: &'static [&'static MessageMetadata],

    /// Optional protocol-level comment or description
    pub comment: &'static str,
}

/// Trait for message types that provide metadata
///
/// All generated message types implement this trait, allowing generic
/// code to access their metadata.
///
/// # Example
///
/// ```rust
/// use cando_messages::metadata::HasMetadata;
/// use cando_messages::j1939::WAND;
///
/// fn print_message_info<T: HasMetadata>() {
///     let meta = T::metadata();
///     println!("{}: {} bytes, {} signals",
///         meta.name, meta.dlc, meta.signals.len());
/// }
///
/// print_message_info::<WAND>();
/// ```
pub trait HasMetadata {
    /// Returns a reference to this message type's metadata
    fn metadata() -> &'static MessageMetadata;
}

impl SignalMetadata {
    /// Returns true if this signal has value descriptions (enumerations)
    pub fn has_value_descriptions(&self) -> bool {
        !self.value_descriptions.is_empty()
    }

    /// Look up a value description for a given raw value
    ///
    /// Returns Some(description) if found, None otherwise.
    pub fn get_value_description(&self, raw_value: u64) -> Option<&'static str> {
        self.value_descriptions
            .iter()
            .find(|(val, _)| *val == raw_value)
            .map(|(_, desc)| *desc)
    }

    /// Returns true if this signal uses scaling (factor != 1.0 or offset != 0.0)
    pub fn is_scaled(&self) -> bool {
        (self.factor - 1.0).abs() > f64::EPSILON || self.offset.abs() > f64::EPSILON
    }

    /// Returns true if this signal is signed
    pub fn is_signed(&self) -> bool {
        matches!(self.value_type, ValueType::Signed)
    }

    /// Returns true if this signal uses big-endian byte order
    pub fn is_big_endian(&self) -> bool {
        matches!(self.byte_order, ByteOrder::BigEndian)
    }
}

impl MessageMetadata {
    /// Find a signal by name
    ///
    /// Returns Some(signal_metadata) if found, None otherwise.
    pub fn find_signal(&self, name: &str) -> Option<&SignalMetadata> {
        self.signals.iter().find(|s| s.name == name)
    }

    /// Returns the number of signals in this message
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    /// Returns true if this message has a comment
    pub fn has_comment(&self) -> bool {
        !self.comment.is_empty()
    }

    /// Returns true if this message has a transmitter specified
    pub fn has_transmitter(&self) -> bool {
        !self.transmitter.is_empty()
    }

    /// Returns an iterator over all signals
    pub fn signals_iter(&self) -> impl Iterator<Item = &SignalMetadata> {
        self.signals.iter()
    }
}

impl ProtocolMetadata {
    /// Find a message by name
    ///
    /// Returns Some(message_metadata) if found, None otherwise.
    pub fn find_message(&self, name: &str) -> Option<&'static MessageMetadata> {
        self.messages.iter().copied().find(|m| m.name == name)
    }

    /// Find a message by CAN ID
    ///
    /// Returns Some(message_metadata) if found, None otherwise.
    /// Note: For messages with device ID embedding, this matches the base CAN ID.
    pub fn find_message_by_id(&self, can_id: u32) -> Option<&'static MessageMetadata> {
        self.messages.iter().copied().find(|m| m.can_id == can_id)
    }

    /// Returns the total number of messages in this protocol
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Returns the total number of signals across all messages
    pub fn total_signal_count(&self) -> usize {
        self.messages.iter().map(|m| m.signals.len()).sum()
    }

    /// Returns an iterator over all messages
    pub fn messages_iter(&self) -> impl Iterator<Item = &'static MessageMetadata> {
        self.messages.iter().copied()
    }

    /// Returns true if this protocol has a version string
    pub fn has_version(&self) -> bool {
        !self.version.is_empty()
    }

    /// Returns true if this protocol has a comment
    pub fn has_comment(&self) -> bool {
        !self.comment.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_metadata_helpers() {
        let signal = SignalMetadata {
            name: "test_signal",
            start_bit: 0,
            signal_size: 16,
            byte_order: ByteOrder::LittleEndian,
            value_type: ValueType::Unsigned,
            factor: 0.5,
            offset: 10.0,
            min: 0.0,
            max: 100.0,
            unit: "rpm",
            value_descriptions: &[(0, "Off"), (1, "On")],
            comment: "Test signal",
        };

        assert!(signal.is_scaled());
        assert!(!signal.is_signed());
        assert!(!signal.is_big_endian());
        assert!(signal.has_value_descriptions());
        assert_eq!(signal.get_value_description(0), Some("Off"));
        assert_eq!(signal.get_value_description(1), Some("On"));
        assert_eq!(signal.get_value_description(99), None);
    }

    #[test]
    fn test_message_metadata_helpers() {
        static SIGNAL1: SignalMetadata = SignalMetadata {
            name: "signal1",
            start_bit: 0,
            signal_size: 8,
            byte_order: ByteOrder::LittleEndian,
            value_type: ValueType::Unsigned,
            factor: 1.0,
            offset: 0.0,
            min: 0.0,
            max: 255.0,
            unit: "",
            value_descriptions: &[],
            comment: "",
        };

        static SIGNAL2: SignalMetadata = SignalMetadata {
            name: "signal2",
            start_bit: 8,
            signal_size: 8,
            byte_order: ByteOrder::LittleEndian,
            value_type: ValueType::Unsigned,
            factor: 1.0,
            offset: 0.0,
            min: 0.0,
            max: 255.0,
            unit: "",
            value_descriptions: &[],
            comment: "",
        };

        static MESSAGE: MessageMetadata = MessageMetadata {
            name: "TestMessage",
            can_id: 0x123,
            dlc: 8,
            signals: &[SIGNAL1, SIGNAL2],
            is_multiplexed: false,
            comment: "Test message",
            transmitter: "TestNode",
        };

        assert_eq!(MESSAGE.signal_count(), 2);
        assert!(MESSAGE.find_signal("signal1").is_some());
        assert!(MESSAGE.find_signal("signal2").is_some());
        assert!(MESSAGE.find_signal("nonexistent").is_none());
        assert!(MESSAGE.has_comment());
        assert!(MESSAGE.has_transmitter());
    }

    #[test]
    fn test_protocol_metadata_helpers() {
        static SIGNAL: SignalMetadata = SignalMetadata {
            name: "signal",
            start_bit: 0,
            signal_size: 8,
            byte_order: ByteOrder::LittleEndian,
            value_type: ValueType::Unsigned,
            factor: 1.0,
            offset: 0.0,
            min: 0.0,
            max: 255.0,
            unit: "",
            value_descriptions: &[],
            comment: "",
        };

        static MSG1: MessageMetadata = MessageMetadata {
            name: "Message1",
            can_id: 0x100,
            dlc: 8,
            signals: &[SIGNAL],
            is_multiplexed: false,
            comment: "",
            transmitter: "",
        };

        static MSG2: MessageMetadata = MessageMetadata {
            name: "Message2",
            can_id: 0x200,
            dlc: 8,
            signals: &[SIGNAL],
            is_multiplexed: false,
            comment: "",
            transmitter: "",
        };

        static PROTOCOL: ProtocolMetadata = ProtocolMetadata {
            name: "TestProtocol",
            version: "1.0",
            messages: &[&MSG1, &MSG2],
            comment: "Test protocol",
        };

        assert_eq!(PROTOCOL.message_count(), 2);
        assert_eq!(PROTOCOL.total_signal_count(), 2);
        assert!(PROTOCOL.find_message("Message1").is_some());
        assert!(PROTOCOL.find_message("Message2").is_some());
        assert!(PROTOCOL.find_message("Nonexistent").is_none());
        assert!(PROTOCOL.find_message_by_id(0x100).is_some());
        assert!(PROTOCOL.find_message_by_id(0x200).is_some());
        assert!(PROTOCOL.find_message_by_id(0x999).is_none());
        assert!(PROTOCOL.has_version());
        assert!(PROTOCOL.has_comment());
    }
}
