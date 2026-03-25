//! Core type definitions for CAN monitoring.
//!
//! This module contains the fundamental data structures used throughout the
//! CAN monitoring library.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use socketcan::CanFrame;
use std::path::PathBuf;
use std::sync::Arc;

/// Configuration for a single CAN interface.
///
/// Supports both live SocketCAN interfaces and candump log file replay.
#[derive(Debug, Clone, PartialEq)]
pub enum CanInterfaceConfig {
    /// Monitor a live SocketCAN interface.
    Live {
        /// Interface name (e.g., "can0", "vcan0")
        interface: String,
    },

    /// Replay from a candump log file.
    Replay {
        /// Path to candump log file
        path: PathBuf,
        /// Replay rate configuration
        rate: ReplayRate,
        /// Whether to loop back to the beginning when reaching end of file
        loop_at_end: bool,
    },
}

impl CanInterfaceConfig {
    /// Create a new live interface configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use cando_can_monitor::CanInterfaceConfig;
    ///
    /// let config = CanInterfaceConfig::live("can0");
    /// ```
    pub fn live(interface: impl Into<String>) -> Self {
        CanInterfaceConfig::Live {
            interface: interface.into(),
        }
    }

    /// Create a new replay configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use cando_can_monitor::CanInterfaceConfig;
    /// use std::path::PathBuf;
    ///
    /// let config = CanInterfaceConfig::replay(
    ///     PathBuf::from("traffic.log"),
    ///     100,  // 100 messages per second
    ///     true, // loop at end
    /// );
    /// ```
    pub fn replay(path: PathBuf, messages_per_sec: u32, loop_at_end: bool) -> Self {
        CanInterfaceConfig::Replay {
            path,
            rate: ReplayRate::Fixed(messages_per_sec),
            loop_at_end,
        }
    }

    /// Create a replay configuration with time-preserving playback.
    pub fn replay_timed(path: PathBuf, loop_at_end: bool) -> Self {
        CanInterfaceConfig::Replay {
            path,
            rate: ReplayRate::Timed,
            loop_at_end,
        }
    }

    /// Get a human-readable name for this interface.
    pub fn name(&self) -> String {
        match self {
            CanInterfaceConfig::Live { interface } => interface.clone(),
            CanInterfaceConfig::Replay { path, .. } => path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
        }
    }

    /// Returns true if this is a live interface configuration.
    pub fn is_live(&self) -> bool {
        matches!(self, CanInterfaceConfig::Live { .. })
    }

    /// Returns true if this is a replay configuration.
    pub fn is_replay(&self) -> bool {
        matches!(self, CanInterfaceConfig::Replay { .. })
    }
}

/// Replay rate configuration for candump log playback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayRate {
    /// Replay at a fixed rate (messages per second).
    ///
    /// Valid range: 1 to 10000 messages per second.
    Fixed(u32),

    /// Preserve original timing from log file timestamps.
    ///
    /// Plays back messages with the same inter-message delays as in the
    /// original capture.
    Timed,
}

impl ReplayRate {
    /// Validate that the replay rate is within acceptable bounds.
    pub fn validate(&self) -> Result<(), String> {
        match self {
            ReplayRate::Fixed(rate) if *rate < 1 || *rate > 10000 => Err(format!(
                "Invalid replay rate {} (must be between 1 and 10000)",
                rate
            )),
            _ => Ok(()),
        }
    }

    /// Get the delay between messages in milliseconds (for Fixed rate only).
    pub fn delay_ms(&self) -> Option<u64> {
        match self {
            ReplayRate::Fixed(rate) if *rate > 0 => Some(1000 / (*rate as u64)),
            _ => None,
        }
    }
}

/// Protocol identification for decoded messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    /// SAE J1939 vehicle bus protocol
    J1939,
    /// Unknown or unrecognized protocol
    Unknown,
}

impl Protocol {
    /// Get a human-readable name for this protocol.
    pub fn name(&self) -> &'static str {
        match self {
            Protocol::J1939 => "J1939",
            Protocol::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// A decoded CAN message with metadata.
///
/// Contains the extracted device ID, protocol identification, and raw frame data.
#[derive(Debug, Clone)]
pub struct DecodedMessage {
    /// Device ID extracted from CAN ID
    pub device_id: u8,

    /// Identified protocol
    pub protocol: Protocol,

    /// Human-readable message type name (e.g., "DM01", "EEC1")
    pub message_name: String,

    /// Raw CAN ID
    pub can_id: u32,

    /// Raw frame data
    pub data: Vec<u8>,

    /// Timestamp when the message was received/replayed
    pub timestamp: DateTime<Utc>,

    /// Source interface name
    pub interface: String,

    /// Original CAN frame
    pub frame: CanFrame,
}

impl DecodedMessage {
    /// Create a new decoded message.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device_id: u8,
        protocol: Protocol,
        message_name: String,
        can_id: u32,
        data: Vec<u8>,
        timestamp: DateTime<Utc>,
        interface: String,
        frame: CanFrame,
    ) -> Self {
        Self {
            device_id,
            protocol,
            message_name,
            can_id,
            data,
            timestamp,
            interface,
            frame,
        }
    }

    /// Get the data length code (DLC) of the frame.
    pub fn dlc(&self) -> usize {
        self.data.len()
    }

    /// Check if this is a standard (11-bit) CAN ID.
    pub fn is_standard_id(&self) -> bool {
        self.can_id <= 0x7FF
    }

    /// Check if this is an extended (29-bit) CAN ID.
    pub fn is_extended_id(&self) -> bool {
        !self.is_standard_id()
    }

    /// Format the message data as a hex string.
    pub fn data_hex(&self) -> String {
        self.data
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Type alias for message callback functions.
///
/// Callbacks are invoked when a message for a registered device ID is received.
pub type MessageCallback = Arc<dyn Fn(DecodedMessage) + Send + Sync>;

/// Frame source information for tracking where frames originate.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FrameSource {
    /// Interface name or log file name
    pub name: String,
    /// Whether this is a live interface
    pub is_live: bool,
    /// Timestamp when the source was created
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
impl FrameSource {
    /// Create a new frame source.
    pub fn new(name: String, is_live: bool) -> Self {
        Self {
            name,
            is_live,
            created_at: Utc::now(),
        }
    }
}

/// Aggregated frame with source information.
#[derive(Debug, Clone)]
pub struct SourcedFrame {
    /// The CAN frame
    pub frame: CanFrame,
    /// Source interface information
    pub source: String,
    /// Timestamp when received
    pub timestamp: DateTime<Utc>,
}

impl SourcedFrame {
    /// Create a new sourced frame.
    pub fn new(frame: CanFrame, source: String) -> Self {
        Self {
            frame,
            source,
            timestamp: Utc::now(),
        }
    }

    /// Create a sourced frame with a specific timestamp.
    pub fn with_timestamp(frame: CanFrame, source: String, timestamp: DateTime<Utc>) -> Self {
        Self {
            frame,
            source,
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socketcan::{EmbeddedFrame, ExtendedId, Id, StandardId};

    #[test]
    fn test_can_interface_config_live() {
        let config = CanInterfaceConfig::live("can0");
        assert!(config.is_live());
        assert!(!config.is_replay());
        assert_eq!(config.name(), "can0");
    }

    #[test]
    fn test_can_interface_config_replay() {
        let config = CanInterfaceConfig::replay(PathBuf::from("test.log"), 100, true);
        assert!(!config.is_live());
        assert!(config.is_replay());
        assert_eq!(config.name(), "test.log");
    }

    #[test]
    fn test_replay_rate_validation() {
        assert!(ReplayRate::Fixed(100).validate().is_ok());
        assert!(ReplayRate::Fixed(1).validate().is_ok());
        assert!(ReplayRate::Fixed(10000).validate().is_ok());
        assert!(ReplayRate::Timed.validate().is_ok());

        assert!(ReplayRate::Fixed(0).validate().is_err());
        assert!(ReplayRate::Fixed(10001).validate().is_err());
    }

    #[test]
    fn test_replay_rate_delay_calculation() {
        assert_eq!(ReplayRate::Fixed(100).delay_ms(), Some(10));
        assert_eq!(ReplayRate::Fixed(1000).delay_ms(), Some(1));
        assert_eq!(ReplayRate::Fixed(10).delay_ms(), Some(100));
        assert_eq!(ReplayRate::Timed.delay_ms(), None);
    }

    #[test]
    fn test_protocol_display() {
        assert_eq!(Protocol::J1939.to_string(), "J1939");
        assert_eq!(Protocol::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_protocol_name() {
        assert_eq!(Protocol::J1939.name(), "J1939");
        assert_eq!(Protocol::Unknown.name(), "Unknown");
    }

    #[test]
    fn test_decoded_message_helpers() {
        let can_id = Id::Extended(ExtendedId::new(0x18FECA88).unwrap());
        let frame = CanFrame::new(can_id, &[0xA7, 0x2E, 0x00, 0x7D]).unwrap();
        let msg = DecodedMessage::new(
            0x88,
            Protocol::J1939,
            "DM01".to_string(),
            0x18FECA88,
            vec![0xA7, 0x2E, 0x00, 0x7D],
            Utc::now(),
            "can0".to_string(),
            frame,
        );

        assert_eq!(msg.dlc(), 4);
        assert!(msg.is_extended_id());
        assert!(!msg.is_standard_id());
        assert_eq!(msg.data_hex(), "A7 2E 00 7D");
    }

    #[test]
    fn test_sourced_frame_creation() {
        let can_id = Id::Standard(StandardId::new(0x123).unwrap());
        let frame = CanFrame::new(can_id, &[1, 2, 3]).unwrap();
        let sourced = SourcedFrame::new(frame, "can0".to_string());
        assert_eq!(sourced.source, "can0");
        assert_eq!(sourced.frame.data(), &[1, 2, 3]);
    }

    #[test]
    fn test_frame_source() {
        let source = FrameSource::new("vcan0".to_string(), true);
        assert_eq!(source.name, "vcan0");
        assert!(source.is_live);
    }
}
