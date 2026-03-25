// Error variant fields are self-documenting via the #[error] attribute format strings.
#![allow(missing_docs)]
//! Error types for CAN monitoring operations.
//!
//! This module defines custom error types for different failure scenarios:
//! - Interface errors (CAN socket failures)
//! - Replay errors (log file parsing/reading)
//! - Decode errors (message parsing failures)

use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for operations that can fail with monitoring errors.
pub type Result<T> = std::result::Result<T, MonitorError>;

/// Top-level error type for monitoring operations.
#[derive(Error, Debug)]
pub enum MonitorError {
    /// CAN interface operation failed
    #[error("Interface error: {0}")]
    Interface(#[from] InterfaceError),

    /// Log replay operation failed
    #[error("Replay error: {0}")]
    Replay(#[from] ReplayError),

    /// Message decoding failed
    #[error("Decode error: {0}")]
    Decode(#[from] DecodeError),

    /// Generic I/O error
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Other errors
    #[error("{0}")]
    Other(String),
}

/// Errors related to CAN interface operations.
#[derive(Error, Debug)]
pub enum InterfaceError {
    /// Failed to open CAN interface
    #[error("Failed to open interface '{interface}': {source}")]
    OpenFailed {
        interface: String,
        source: io::Error,
    },

    /// Failed to read from CAN interface
    #[error("Failed to read from interface '{interface}': {source}")]
    ReadFailed {
        interface: String,
        source: io::Error,
    },

    /// Interface disconnected unexpectedly
    #[error("Interface '{interface}' disconnected unexpectedly")]
    Disconnected { interface: String },

    /// Interface not found
    #[error("Interface '{interface}' not found (may need to be created or brought up)")]
    NotFound { interface: String },

    /// Permission denied accessing interface
    #[error(
        "Permission denied for interface '{interface}' (may need sudo or user group membership)"
    )]
    PermissionDenied { interface: String },

    /// Timeout waiting for frames
    #[error("Timeout waiting for frames on interface '{interface}' after {duration_ms}ms")]
    Timeout { interface: String, duration_ms: u64 },

    /// All configured interfaces failed
    #[error("All {count} configured interfaces failed to initialize")]
    AllInterfacesFailed { count: usize },

    /// Invalid interface name
    #[error("Invalid interface name: '{name}'")]
    InvalidName { name: String },
}

impl InterfaceError {
    /// Returns true if this error is potentially transient and worth retrying.
    pub fn is_transient(&self) -> bool {
        matches!(
            self,
            InterfaceError::ReadFailed { .. }
                | InterfaceError::Timeout { .. }
                | InterfaceError::Disconnected { .. }
        )
    }

    /// Returns true if this error is permanent and should not be retried.
    pub fn is_permanent(&self) -> bool {
        matches!(
            self,
            InterfaceError::NotFound { .. }
                | InterfaceError::PermissionDenied { .. }
                | InterfaceError::InvalidName { .. }
        )
    }
}

/// Errors related to log replay operations.
#[derive(Error, Debug)]
pub enum ReplayError {
    /// Log file not found
    #[error("Log file not found: {path:?}")]
    FileNotFound { path: PathBuf },

    /// Failed to read log file
    #[error("Failed to read log file {path:?}: {source}")]
    ReadFailed { path: PathBuf, source: io::Error },

    /// Invalid log file format
    #[error("Invalid candump format at line {line_number}: {reason}")]
    InvalidFormat { line_number: usize, reason: String },

    /// Empty log file
    #[error("Log file is empty: {path:?}")]
    EmptyLog { path: PathBuf },

    /// Invalid replay rate
    #[error("Invalid replay rate: {rate} (must be between 1 and 10000 messages/sec)")]
    InvalidRate { rate: u32 },

    /// Malformed CAN frame in log
    #[error("Malformed CAN frame at line {line_number}: {reason}")]
    MalformedFrame { line_number: usize, reason: String },

    /// Invalid timestamp in log
    #[error("Invalid timestamp at line {line_number}: {timestamp}")]
    InvalidTimestamp {
        line_number: usize,
        timestamp: String,
    },

    /// Replay stopped
    #[error("Replay stopped by user")]
    Stopped,
}

impl ReplayError {
    /// Create a new InvalidFormat error.
    pub fn invalid_format(line_number: usize, reason: impl Into<String>) -> Self {
        ReplayError::InvalidFormat {
            line_number,
            reason: reason.into(),
        }
    }

    /// Create a new MalformedFrame error.
    pub fn malformed_frame(line_number: usize, reason: impl Into<String>) -> Self {
        ReplayError::MalformedFrame {
            line_number,
            reason: reason.into(),
        }
    }
}

/// Errors related to message decoding operations.
#[derive(Error, Debug)]
pub enum DecodeError {
    /// Unknown protocol detected
    #[error("Unknown protocol for CAN ID 0x{can_id:08X}")]
    UnknownProtocol { can_id: u32 },

    /// Unknown message type within a known protocol
    #[error("Unknown {protocol:?} message type for CAN ID 0x{can_id:08X}")]
    UnknownMessageType { protocol: String, can_id: u32 },

    /// Invalid frame data length
    #[error(
        "Invalid frame length {actual} for message type '{message_type}' (expected {expected})"
    )]
    InvalidFrameLength {
        message_type: String,
        expected: usize,
        actual: usize,
    },

    /// Failed to decode message fields
    #[error("Failed to decode {message_type}: {reason}")]
    DecodeFailed {
        message_type: String,
        reason: String,
    },

    /// Invalid device ID
    #[error("Invalid device ID 0x{device_id:02X} for protocol {protocol:?}")]
    InvalidDeviceId { device_id: u8, protocol: String },

    /// Frame data validation failed
    #[error("Frame validation failed for {message_type}: {reason}")]
    ValidationFailed {
        message_type: String,
        reason: String,
    },
}

impl DecodeError {
    /// Returns true if this error indicates an unrecognized but potentially valid message.
    pub fn is_unknown_message(&self) -> bool {
        matches!(
            self,
            DecodeError::UnknownProtocol { .. } | DecodeError::UnknownMessageType { .. }
        )
    }

    /// Returns true if this error indicates corrupted or invalid data.
    pub fn is_corrupted_data(&self) -> bool {
        matches!(
            self,
            DecodeError::InvalidFrameLength { .. }
                | DecodeError::DecodeFailed { .. }
                | DecodeError::ValidationFailed { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_error_transient_detection() {
        let transient = InterfaceError::ReadFailed {
            interface: "can0".to_string(),
            source: io::Error::new(io::ErrorKind::TimedOut, "timeout"),
        };
        assert!(transient.is_transient());
        assert!(!transient.is_permanent());

        let permanent = InterfaceError::NotFound {
            interface: "can0".to_string(),
        };
        assert!(!permanent.is_transient());
        assert!(permanent.is_permanent());
    }

    #[test]
    fn test_interface_error_display() {
        let err = InterfaceError::OpenFailed {
            interface: "can0".to_string(),
            source: io::Error::new(io::ErrorKind::NotFound, "not found"),
        };
        let display = format!("{}", err);
        assert!(display.contains("can0"));
        assert!(display.contains("Failed to open interface"));
    }

    #[test]
    fn test_replay_error_helpers() {
        let err = ReplayError::invalid_format(42, "bad data");
        match err {
            ReplayError::InvalidFormat { line_number, .. } => assert_eq!(line_number, 42),
            _ => panic!("Wrong error variant"),
        }

        let err = ReplayError::malformed_frame(100, "invalid hex");
        match err {
            ReplayError::MalformedFrame { line_number, .. } => assert_eq!(line_number, 100),
            _ => panic!("Wrong error variant"),
        }
    }

    #[test]
    fn test_decode_error_classification() {
        let unknown = DecodeError::UnknownProtocol { can_id: 0x12345678 };
        assert!(unknown.is_unknown_message());
        assert!(!unknown.is_corrupted_data());

        let corrupted = DecodeError::InvalidFrameLength {
            message_type: "DM01".to_string(),
            expected: 8,
            actual: 4,
        };
        assert!(!corrupted.is_unknown_message());
        assert!(corrupted.is_corrupted_data());
    }

    #[test]
    fn test_monitor_error_from_conversions() {
        let interface_err = InterfaceError::NotFound {
            interface: "can0".to_string(),
        };
        let monitor_err: MonitorError = interface_err.into();
        assert!(matches!(monitor_err, MonitorError::Interface(_)));

        let replay_err = ReplayError::FileNotFound {
            path: PathBuf::from("test.log"),
        };
        let monitor_err: MonitorError = replay_err.into();
        assert!(matches!(monitor_err, MonitorError::Replay(_)));

        let decode_err = DecodeError::UnknownProtocol { can_id: 0x123 };
        let monitor_err: MonitorError = decode_err.into();
        assert!(matches!(monitor_err, MonitorError::Decode(_)));
    }

    #[test]
    fn test_error_messages_are_actionable() {
        // Verify error messages contain useful information for debugging
        let err = InterfaceError::PermissionDenied {
            interface: "can0".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("can0"));
        assert!(msg.contains("Permission denied"));
        assert!(msg.contains("sudo") || msg.contains("group"));

        let err = ReplayError::InvalidRate { rate: 50000 };
        let msg = format!("{}", err);
        assert!(msg.contains("50000"));
        assert!(msg.contains("10000")); // Shows the valid range
    }
}
