#![warn(missing_docs)]
//! # CAN Device Simulator Common
//!
//! Shared functionality for CAN device simulators (J1939 and custom protocols).
//!
//! This crate eliminates code duplication across simulators by providing:
//! - Generic WebSocket server for state broadcasting
//! - Simplified CAN socket operations
//! - Common CLI argument definitions with configuration file support
//! - Consistent error handling
//!
//! ## Features
//!
//! ### WebSocket Server
//!
//! Generic WebSocket server that can broadcast any simulator state that implements
//! the `SimulatorState` trait. Handles connection management, JSON serialization,
//! and broadcast channel integration automatically.
//!
//! ### CAN Socket Utilities
//!
//! Simplified CAN interface operations with automatic ID detection (standard vs extended),
//! consistent error handling, and ergonomic API.
//!
//! ### Common CLI Arguments
//!
//! Standard command-line arguments used by all simulators (interface, websocket_port,
//! debug flags, etc.) with configuration file support and precedence rules.
//!
//! ### Error Types
//!
//! Consistent error handling across all simulators with context-rich error messages.
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use cando_simulator_common::{
//!     SimulatorWebSocketServer, SimulatorState, CommonSimulatorArgs,
//!     CanInterface, Result,
//! };
//! use serde::{Serialize, Deserialize};
//! use std::sync::Arc;
//! use tokio::sync::{broadcast, Mutex};
//! use clap::Parser;
//!
//! #[derive(Clone, Serialize, Deserialize)]
//! struct MySimulatorState {
//!     value: f64,
//!     active: bool,
//! }
//!
//! impl SimulatorState for MySimulatorState {}
//!
//! #[derive(Parser)]
//! struct Args {
//!     #[command(flatten)]
//!     common: CommonSimulatorArgs,
//!
//!     /// Simulator-specific argument
//!     #[arg(long, default_value = "100.0")]
//!     max_value: f64,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let args = Args::parse();
//!
//!     // Resolve configuration with precedence
//!     let config = args.common.resolve_config()?;
//!     args.common.init_tracing();
//!
//!     // Create state and broadcast channel
//!     let state = Arc::new(Mutex::new(MySimulatorState {
//!         value: 0.0,
//!         active: true,
//!     }));
//!     let (broadcast_tx, _) = broadcast::channel(100);
//!
//!     // Start WebSocket server
//!     let ws_server = SimulatorWebSocketServer::new(state.clone(), broadcast_tx.clone(), None);
//!     ws_server.start(config.websocket_port).await?;
//!
//!     // Open CAN interface
//!     let can = CanInterface::open(&config.interface)?;
//!
//!     // Run simulator...
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Design Philosophy
//!
//! This crate follows these principles:
//! - **Generic**: Works with any simulator state via traits
//! - **Zero-cost**: Generics compile to same assembly as hand-written code
//! - **Ergonomic**: Simple, intuitive APIs that reduce boilerplate
//! - **Consistent**: Same patterns across all simulators
//! - **Testable**: Well-isolated components with clear interfaces

pub mod can_frame;
pub mod can_socket;
pub mod cli;
pub mod device_id;
pub mod error;
pub mod message_filter;
pub mod message_tracking;
pub mod physics;
pub mod websocket;

// Re-exports for convenience
pub use can_frame::{create_can_frame, create_j1939_frame, create_standard_frame, FrameType};
pub use can_socket::CanInterface;
pub use cli::{CommonSimulatorArgs, ResolvedConfig};
pub use device_id::{parse_device_id, DeviceIdValidator, J1939DeviceIdValidator};
pub use error::{Result, SimulatorError};
pub use message_filter::{extract_source_device_id, is_external_message, should_ignore_message};
pub use message_tracking::{MessageTracking, ReceivedMessage, DEFAULT_MESSAGE_BUFFER_SIZE};
pub use physics::{clamp_with_hysteresis, exponential_ramp, linear_ramp, thermal_model};
pub use websocket::{
    handle_websocket_command, start_simulator_websocket, CommandHandler, SimulatorState,
    SimulatorWebSocketServer, StateQueryable, WebSocketMessage,
};
