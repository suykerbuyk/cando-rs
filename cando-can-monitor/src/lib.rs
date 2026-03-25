#![warn(missing_docs)]
//! # Cando CAN Monitor
//!
//! A shared library for monitoring CAN interfaces and replaying candump logs with robust
//! error handling and structured logging.
//!
//! ## Features
//!
//! - **Multi-Interface Reading**: Monitor multiple CAN interfaces (can0, can1, vcan0) simultaneously
//! - **Candump Log Replay**: Replay captured CAN traffic with configurable rate control and looping
//! - **Device ID Routing**: Route messages by device ID, not by interface
//! - **Protocol Detection**: Automatic detection of J1939 protocols
//! - **Robust Error Handling**: Graceful degradation when interfaces fail
//! - **Structured Logging**: Built-in tracing support for observability
//!
//! ## Architecture
//!
//! The library provides a clean callback-based API for CAN message handling:
//!
//! 1. **Reader**: Aggregates frames from multiple interfaces (live or replay)
//! 2. **Decoder**: Extracts device IDs and identifies protocol/message types
//! 3. **Dispatcher**: Routes decoded messages to registered callbacks by device ID
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use cando_can_monitor::{CanInterfaceConfig, MessageDispatcher, DecodedMessage};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Initialize tracing
//!     tracing_subscriber::fmt::init();
//!
//!     // Configure interfaces to monitor
//!     let configs = vec![
//!         CanInterfaceConfig::live("can0"),
//!         CanInterfaceConfig::replay(
//!             "logs/traffic.log".into(),
//!             100, // 100 messages per second
//!             true, // loop at end
//!         ),
//!     ];
//!
//!     // Create dispatcher and register callbacks
//!     let mut dispatcher = MessageDispatcher::new();
//!     dispatcher.register(0x8A, |msg: DecodedMessage| {
//!         println!("Device 0x8A: {} @ {:?}", msg.message_name, msg.timestamp);
//!     });
//!
//!     // Start monitoring (this blocks)
//!     // monitor.start(configs, dispatcher).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! The library uses custom error types and implements graceful degradation:
//!
//! - **Interface Failures**: Continues monitoring remaining interfaces
//! - **Malformed Data**: Logs errors and skips invalid frames
//! - **Transient Errors**: Implements retry logic for recoverable failures
//!
//! ## Performance
//!
//! - Supports replay rates up to 1000 messages/second
//! - Minimal overhead with async I/O
//! - Efficient frame aggregation from multiple sources

// Re-export key types from dependencies
pub use chrono::{DateTime, Utc};
pub use socketcan::CanFrame;

// Module declarations
mod decoder;
mod dispatcher;
mod error;
mod reader;
mod replay;
mod types;

// Public exports
pub use decoder::{decode_message, extract_device_id, identify_protocol};
pub use dispatcher::MessageDispatcher;
pub use error::{DecodeError, InterfaceError, ReplayError, Result};
pub use reader::MultiInterfaceReader;
pub use replay::LogReplayer;
pub use types::{
    CanInterfaceConfig, DecodedMessage, MessageCallback, Protocol, ReplayRate, SourcedFrame,
};

/// Initialize tracing subscriber with reasonable defaults.
///
/// This is a convenience function for quick setup. For production use,
/// consider configuring the subscriber with custom filters and formatting.
///
/// # Example
///
/// ```rust
/// cando_can_monitor::init_tracing();
/// tracing::info!("Tracing initialized");
/// ```
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_crate_compiles() {
        // Basic smoke test to ensure the crate structure is valid
        assert_eq!(2 + 2, 4);
    }
}
