//! CAN socket sender for cando-send
//!
//! Handles opening CAN sockets and sending frames.

use crate::frame::BuiltFrame;
use anyhow::{Context, Result};
use socketcan::{CanFdSocket, CanSocket, Socket};

/// Send a CAN frame to the specified interface
///
/// # Arguments
///
/// * `interface` - The CAN interface name (e.g., "vcan0", "can0")
/// * `frame` - The frame to send (either CAN or CAN FD)
///
/// # Examples
///
/// ```no_run
/// use cando_send::parser::parse_frame;
/// use cando_send::frame::build_frame;
/// use cando_send::sender::send_frame;
///
/// let parsed = parse_frame("123#DEADBEEF").unwrap();
/// let frame = build_frame(&parsed).unwrap();
/// send_frame("vcan0", &frame).unwrap();
/// ```
pub fn send_frame(interface: &str, frame: &BuiltFrame) -> Result<()> {
    // Handle based on frame type
    match frame {
        BuiltFrame::Can(can_frame) => {
            // Use regular CAN socket
            let socket = open_socket(interface)?;
            socket
                .write_frame(can_frame)
                .with_context(|| format!("Failed to send frame on interface '{}'", interface))?;
        }
        BuiltFrame::Fd(fd_frame) => {
            // Use CAN FD socket
            let socket = open_fd_socket(interface)?;
            socket.write_frame(fd_frame).with_context(|| {
                format!("Failed to send CAN FD frame on interface '{}'", interface)
            })?;
        }
    }

    Ok(())
}

/// Send a frame to an already-open socket
///
/// This is useful for replay mode where we want to reuse a socket
/// for multiple frame transmissions.
///
/// # Arguments
///
/// * `socket` - An open CAN socket
/// * `frame` - The frame to send (either CAN or CAN FD)
pub fn send_frame_to_socket(socket: &CanSocket, frame: &BuiltFrame) -> Result<()> {
    match frame {
        BuiltFrame::Can(can_frame) => {
            socket
                .write_frame(can_frame)
                .context("Failed to send frame")?;
        }
        BuiltFrame::Fd(_) => {
            anyhow::bail!(
                "Cannot send CAN FD frame to regular CAN socket. Use CAN FD socket instead."
            );
        }
    }
    Ok(())
}

/// Open a CAN socket for the specified interface
///
/// # Arguments
///
/// * `interface` - The CAN interface name (e.g., "vcan0", "can0")
///
/// # Returns
///
/// An open CanSocket ready for transmission
pub fn open_socket(interface: &str) -> Result<CanSocket> {
    // Validate interface name
    if interface.is_empty() {
        return Err(anyhow::anyhow!("Interface name cannot be empty"));
    }

    // Open the socket
    let socket = CanSocket::open(interface)
        .with_context(|| format!("Failed to open CAN interface '{}'", interface))?;

    Ok(socket)
}

/// Open a CAN FD socket for the specified interface
///
/// # Arguments
///
/// * `interface` - The CAN interface name (e.g., "vcan0", "can0")
///
/// # Returns
///
/// An open CanFdSocket ready for CAN FD transmission
pub fn open_fd_socket(interface: &str) -> Result<CanFdSocket> {
    // Validate interface name
    if interface.is_empty() {
        return Err(anyhow::anyhow!("Interface name cannot be empty"));
    }

    // Open the CAN FD socket
    let socket = CanFdSocket::open(interface)
        .with_context(|| format!("Failed to open CAN FD interface '{}'", interface))?;

    Ok(socket)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_socket_empty_interface() {
        let result = open_socket("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_open_socket_invalid_interface() {
        let result = open_socket("invalid_interface_name_12345");
        assert!(result.is_err());
    }
}
