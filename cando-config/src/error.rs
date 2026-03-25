//! Error types for configuration system

use std::path::PathBuf;
use thiserror::Error;

/// Configuration error types
#[derive(Error, Debug)]
pub enum ConfigError {
    /// IO error reading configuration file
    #[error("Failed to read configuration file {0}: {1}")]
    IoError(PathBuf, #[source] std::io::Error),

    /// YAML parsing error
    #[error("Failed to parse configuration file {0}: {1}")]
    ParseError(PathBuf, #[source] serde_yml::Error),

    /// Configuration validation error
    #[error("Configuration validation failed: {0}")]
    ValidationError(String),

    /// Device not found in configuration
    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    /// Duplicate device name
    #[error("Duplicate device name: {0}")]
    DuplicateDeviceName(String),

    /// Duplicate WebSocket port
    #[error("Duplicate WebSocket port {0} for devices: {1} and {2}")]
    DuplicateWebSocketPort(u16, String, String),

    /// Duplicate device ID on same interface
    #[error("Duplicate device ID '{0}' on interface '{1}' for devices: {2} and {3}")]
    DuplicateDeviceId(String, String, String, String),

    /// Invalid device ID
    #[error("Invalid device ID '{0}': {1}")]
    InvalidDeviceId(String, String),

    /// Invalid protocol
    #[error("Invalid protocol '{0}'. Expected one of: j1939")]
    InvalidProtocol(String),

    /// Invalid device type
    #[error("Invalid device type '{0}'. Expected one of: j1939")]
    InvalidDeviceType(String),

    /// Port out of range
    #[error("WebSocket port {0} out of valid range (10752-10799)")]
    PortOutOfRange(u16),

    /// Missing required field
    #[error("Missing required field '{0}' in device '{1}'")]
    MissingRequiredField(String, String),

    /// Invalid configuration version
    #[error("Unsupported configuration version: {0} (expected: {1})")]
    UnsupportedVersion(String, String),

    /// Physical hardware cannot have WebSocket port (Rule 1.1)
    #[error("Physical hardware device '{0}' cannot have websocket_port (hardware_present=true)")]
    PhysicalHardwareWithWebSocket(String),

    /// Simulator must have WebSocket port (Rule 1.2)
    #[error("Simulator device '{0}' must have websocket_port (hardware_present=false)")]
    SimulatorMissingWebSocket(String),

    /// Physical hardware on virtual interface (Rule 2.1)
    #[error(
        "Physical hardware device '{0}' cannot use virtual interface '{1}' (hardware_present=true)"
    )]
    PhysicalHardwareOnVirtualInterface(String, String),

    /// Environment not found
    #[error("Environment not found: {0}")]
    EnvironmentNotFound(String),

    /// Duplicate environment name
    #[error("Duplicate environment name: {0}")]
    DuplicateEnvironmentName(String),

    /// Invalid device reference format
    #[error("Invalid device reference: {0}")]
    InvalidDeviceReference(String),
}

/// Result type for configuration operations
pub type Result<T> = std::result::Result<T, ConfigError>;
