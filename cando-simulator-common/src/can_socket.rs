//! CAN socket utilities for simulators
//!
//! This module provides simplified CAN interface operations with automatic
//! ID detection and consistent error handling.

use crate::{Result, SimulatorError};
use socketcan::{CanFrame, CanSocket, Socket};

/// CAN interface wrapper with ergonomic API
///
/// Provides simplified operations for CAN communication with automatic
/// standard/extended ID detection and consistent error handling.
///
/// # Example
///
/// ```rust,ignore
/// use cando_simulator_common::CanInterface;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Open CAN interface
/// let can = CanInterface::open("vcan0")?;
///
/// // Read frames in a loop
/// loop {
///     match can.read_frame() {
///         Ok(frame) => {
///             println!("Received CAN frame");
///         }
///         Err(_) => {
///             // No frame available (non-blocking mode)
///             std::thread::sleep(std::time::Duration::from_millis(10));
///         }
///     }
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct CanInterface {
    socket: CanSocket,
    interface_name: String,
}

impl CanInterface {
    /// Open a CAN interface by name
    ///
    /// # Arguments
    ///
    /// * `interface` - Name of the CAN interface (e.g., "can0", "vcan0")
    ///
    /// # Errors
    ///
    /// Returns `SimulatorError::CanInterface` if the interface cannot be opened.
    pub fn open(interface: &str) -> Result<Self> {
        let socket = CanSocket::open(interface).map_err(|e| {
            SimulatorError::can_interface(format!(
                "Failed to open CAN interface '{}': {}",
                interface, e
            ))
        })?;

        // Set non-blocking mode
        socket.set_nonblocking(true).map_err(|e| {
            SimulatorError::can_interface(format!(
                "Failed to set non-blocking mode on '{}': {}",
                interface, e
            ))
        })?;

        Ok(Self {
            socket,
            interface_name: interface.to_string(),
        })
    }

    /// Read a CAN frame from the interface
    ///
    /// Returns `Ok(frame)` if a frame is available, or an error if the read fails.
    /// Returns `Err` with `WouldBlock` if no frame is available (non-blocking mode).
    ///
    /// # Errors
    ///
    /// Returns `SimulatorError::CanSocketIo` for I/O errors.
    pub fn read_frame(&self) -> Result<CanFrame> {
        self.socket.read_frame().map_err(|e| {
            if e.kind() == std::io::ErrorKind::WouldBlock {
                e.into()
            } else {
                SimulatorError::CanSocketIo(std::io::Error::new(
                    e.kind(),
                    format!(
                        "Failed to read from CAN interface '{}': {}",
                        self.interface_name, e
                    ),
                ))
            }
        })
    }

    /// Write a CAN frame to the interface
    ///
    /// # Arguments
    ///
    /// * `frame` - The CAN frame to send
    ///
    /// # Errors
    ///
    /// Returns `SimulatorError::CanSocketIo` if the write fails.
    pub fn write_frame(&self, frame: &CanFrame) -> Result<()> {
        self.socket.write_frame(frame).map_err(|e| {
            SimulatorError::can_interface(format!(
                "Failed to write to CAN interface '{}': {}",
                self.interface_name, e
            ))
        })?;
        Ok(())
    }

    /// Get the name of the CAN interface
    pub fn interface_name(&self) -> &str {
        &self.interface_name
    }

    /// Set read timeout for the socket
    ///
    /// # Arguments
    ///
    /// * `timeout` - Optional timeout duration. None for blocking mode.
    pub fn set_read_timeout(&self, timeout: Option<std::time::Duration>) -> Result<()> {
        self.socket.set_read_timeout(timeout).map_err(|e| {
            SimulatorError::can_interface(format!(
                "Failed to set read timeout on '{}': {}",
                self.interface_name, e
            ))
        })?;
        Ok(())
    }

    /// Set write timeout for the socket
    ///
    /// # Arguments
    ///
    /// * `timeout` - Optional timeout duration. None for blocking mode.
    pub fn set_write_timeout(&self, timeout: Option<std::time::Duration>) -> Result<()> {
        self.socket.set_write_timeout(timeout).map_err(|e| {
            SimulatorError::can_interface(format!(
                "Failed to set write timeout on '{}': {}",
                self.interface_name, e
            ))
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_name() {
        // Can't actually open interface in tests without hardware/vcan setup,
        // but we can test error messages
        let result = CanInterface::open("nonexistent_interface_12345");
        assert!(result.is_err());

        if let Err(SimulatorError::CanInterface(msg)) = result {
            assert!(msg.contains("nonexistent_interface_12345"));
            assert!(msg.contains("Failed to open CAN interface"));
        } else {
            panic!("Expected CanInterface error");
        }
    }

    #[test]
    fn test_error_messages_contain_context() {
        let result = CanInterface::open("invalid_can_interface");
        assert!(result.is_err());

        let err = result.unwrap_err();
        let err_msg = err.to_string();
        assert!(err_msg.contains("CAN interface"));
    }
}
