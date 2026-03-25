//! Error types for simulator common functionality
//!
//! This module provides consistent error handling across all CAN device simulators.

use thiserror::Error;

/// Result type alias for simulator operations
pub type Result<T> = std::result::Result<T, SimulatorError>;

/// Common error types for simulator operations
#[derive(Error, Debug)]
pub enum SimulatorError {
    /// CAN interface errors
    #[error("CAN interface error: {0}")]
    CanInterface(String),

    /// CAN socket I/O errors
    #[error("CAN socket I/O error: {0}")]
    CanSocketIo(#[from] std::io::Error),

    /// WebSocket errors
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// WebSocket protocol errors
    #[error("WebSocket protocol error: {0}")]
    WebSocketProtocol(#[from] tokio_tungstenite::tungstenite::Error),

    /// State serialization errors
    #[error("State serialization error: {0}")]
    StateSerialization(#[from] serde_json::Error),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Generic errors
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}

impl SimulatorError {
    /// Create a CAN interface error with context
    pub fn can_interface<S: Into<String>>(msg: S) -> Self {
        SimulatorError::CanInterface(msg.into())
    }

    /// Create a WebSocket error with context
    pub fn websocket<S: Into<String>>(msg: S) -> Self {
        SimulatorError::WebSocket(msg.into())
    }

    /// Create a configuration error with context
    pub fn config<S: Into<String>>(msg: S) -> Self {
        SimulatorError::Config(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SimulatorError::can_interface("test error");
        assert_eq!(err.to_string(), "CAN interface error: test error");

        let err = SimulatorError::websocket("connection failed");
        assert_eq!(err.to_string(), "WebSocket error: connection failed");

        let err = SimulatorError::config("invalid port");
        assert_eq!(err.to_string(), "Configuration error: invalid port");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let sim_err: SimulatorError = io_err.into();
        assert!(sim_err.to_string().contains("CAN socket I/O error"));
    }
}
