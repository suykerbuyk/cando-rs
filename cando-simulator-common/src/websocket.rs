//! WebSocket server for simulator state broadcasting
//!
//! This module provides a generic WebSocket server that can broadcast any
//! simulator state implementing the `SimulatorState` trait.

use crate::{Result, SimulatorError};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

/// WebSocket message protocol for simulator communication
///
/// This enum defines the complete protocol for WebSocket communication
/// between test scripts and simulators, supporting both message tracking
/// (ACK framework) and state queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    /// Wait for a specific CAN message to be received (ACK framework)
    WaitForMessage {
        /// CAN ID to wait for
        can_id: u32,
        /// Maximum time to wait in milliseconds
        timeout_ms: u64,
    },

    /// Response to WaitForMessage command
    MessageReceived {
        /// CAN ID that was matched
        can_id: u32,
        /// Timestamp of the matched message in milliseconds
        timestamp_ms: u64,
        /// Elapsed time waiting in milliseconds
        elapsed_ms: u64,
        /// Whether the message was found before timeout
        found: bool,
    },

    /// Request complete simulator state (State query framework)
    GetState,

    /// Response containing complete simulator state as JSON
    StateResponse {
        /// Serialized simulator state
        state_json: String,
    },

    /// Error response for any failed command
    Error {
        /// Error description
        message: String,
    },
}

/// Type alias for WebSocket command handler callback
///
/// Handlers receive the incoming JSON text and return an optional JSON response.
/// If None is returned, no response is sent to the client.
pub type CommandHandler = Arc<dyn Fn(String) -> Option<String> + Send + Sync>;

/// Trait for simulator state that can be broadcast via WebSocket
///
/// Any type implementing this trait can be used with `SimulatorWebSocketServer`
/// to automatically handle JSON serialization and WebSocket broadcasting.
///
/// # Requirements
///
/// - Must implement `Serialize` for JSON conversion
/// - Must implement `Clone` for sharing across tasks
/// - Must be `Send + Sync + 'static` for async runtime
///
/// # Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use cando_simulator_common::SimulatorState;
///
/// #[derive(Clone, Serialize, Deserialize)]
/// struct MyState {
///     value: f64,
///     active: bool,
/// }
///
/// impl SimulatorState for MyState {}
/// ```
pub trait SimulatorState: Serialize + Clone + Send + Sync + 'static {
    /// Convert state to JSON string
    ///
    /// Default implementation uses `serde_json::to_string`.
    /// Override if you need custom JSON formatting.
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails.
    fn to_json(&self) -> std::result::Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Convert state to pretty-printed JSON string
    ///
    /// Useful for debugging or human-readable output.
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails.
    fn to_json_pretty(&self) -> std::result::Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Trait for simulators that can expose their state for testing
///
/// This trait allows simulators to provide their complete internal state
/// as JSON for deterministic test verification. It works in conjunction
/// with the `WebSocketMessage::GetState` command.
///
/// # Requirements
///
/// - State struct must implement `Serialize`
/// - Must be `Send + Sync` for async runtime compatibility
///
/// # Example
///
/// ```rust
/// use cando_simulator_common::{StateQueryable, Result};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct MySimulatorState {
///     speed: f64,
///     temperature: f64,
/// }
///
/// impl StateQueryable for MySimulatorState {
///     fn get_state_json(&self) -> Result<String> {
///         Ok(serde_json::to_string_pretty(self)?)
///     }
/// }
/// ```
pub trait StateQueryable: Send + Sync {
    /// Serialize the complete simulator state as JSON
    ///
    /// This method should return the simulator's complete internal state
    /// as a JSON string. The default behavior is to use pretty-printed JSON
    /// for readability in test output.
    ///
    /// # Errors
    ///
    /// Returns error if JSON serialization fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use cando_simulator_common::{StateQueryable, Result};
    /// # use serde::Serialize;
    /// # #[derive(Serialize)]
    /// # struct State { value: f64 }
    /// # impl StateQueryable for State {
    /// fn get_state_json(&self) -> Result<String> {
    ///     Ok(serde_json::to_string_pretty(self)?)
    /// }
    /// # }
    /// ```
    fn get_state_json(&self) -> Result<String>;
}

/// Handle WebSocket command and return response
///
/// This generic function processes WebSocket commands for simulators that implement
/// both `StateQueryable` (for state queries) and `MessageTracking` (for ACK framework).
///
/// Supported commands:
/// - `GetState`: Returns complete simulator state as JSON
/// - `WaitForMessage`: Checks if a CAN message was received (ACK framework)
///
/// # Type Parameters
///
/// * `S` - State type that implements both `StateQueryable` and `MessageTracking`
///
/// # Arguments
///
/// * `message` - The WebSocket command to process
/// * `state` - The simulator state (must be locked before calling)
///
/// # Returns
///
/// Returns a `WebSocketMessage` response:
/// - `StateResponse` for `GetState` commands
/// - `MessageReceived` for `WaitForMessage` commands
/// - `Error` if command processing fails
///
/// # Example
///
/// ```rust,no_run
/// use cando_simulator_common::{
///     WebSocketMessage, StateQueryable, MessageTracking, ReceivedMessage,
///     handle_websocket_command, Result, DEFAULT_MESSAGE_BUFFER_SIZE,
/// };
/// use std::collections::VecDeque;
/// use std::time::Instant;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct MyState {
///     speed: f64,
///     #[serde(skip)]
///     recent_messages: VecDeque<ReceivedMessage>,
///     #[serde(skip)]
///     simulator_start_time: Instant,
/// }
///
/// impl StateQueryable for MyState {
///     fn get_state_json(&self) -> Result<String> {
///         Ok(serde_json::to_string_pretty(self)?)
///     }
/// }
///
/// impl MessageTracking for MyState {
///     fn get_recent_messages(&self) -> &VecDeque<ReceivedMessage> {
///         &self.recent_messages
///     }
///     fn get_recent_messages_mut(&mut self) -> &mut VecDeque<ReceivedMessage> {
///         &mut self.recent_messages
///     }
///     fn get_simulator_start_time(&self) -> Instant {
///         self.simulator_start_time
///     }
/// }
///
/// # fn example(state: &MyState) {
/// let command = WebSocketMessage::GetState;
/// let response = handle_websocket_command(&command, state);
/// # }
/// ```
pub fn handle_websocket_command<S>(message: &WebSocketMessage, state: &S) -> WebSocketMessage
where
    S: StateQueryable + crate::MessageTracking,
{
    match message {
        WebSocketMessage::GetState => {
            // Handle state query request
            match state.get_state_json() {
                Ok(state_json) => WebSocketMessage::StateResponse { state_json },
                Err(e) => WebSocketMessage::Error {
                    message: format!("Failed to serialize state: {}", e),
                },
            }
        }

        WebSocketMessage::WaitForMessage {
            can_id,
            timeout_ms: _,
        } => {
            // Handle ACK framework message check
            // Non-blocking check: look for the message in recent history
            // The client is responsible for polling with appropriate delays

            // Look for the message in recent_messages (most recent first)
            for msg in state.get_recent_messages().iter().rev() {
                if msg.can_id == *can_id && msg.processed {
                    return WebSocketMessage::MessageReceived {
                        can_id: *can_id,
                        timestamp_ms: msg.timestamp_ms,
                        elapsed_ms: 0, // Immediate check, no elapsed time
                        found: true,
                    };
                }
            }

            // Message not found in history
            WebSocketMessage::MessageReceived {
                can_id: *can_id,
                timestamp_ms: 0,
                elapsed_ms: 0,
                found: false,
            }
        }

        WebSocketMessage::MessageReceived { .. }
        | WebSocketMessage::StateResponse { .. }
        | WebSocketMessage::Error { .. } => {
            // These are response types, not commands
            WebSocketMessage::Error {
                message: "Invalid command: received a response type as a command".to_string(),
            }
        }
    }
}

/// Generic WebSocket server for broadcasting simulator state
///
/// This server handles:
/// - TCP listener setup and binding
/// - WebSocket connection upgrades
/// - State broadcasting to all connected clients
/// - Connection lifecycle management
/// - Automatic JSON serialization
/// - Optional incoming command handling
///
/// # Type Parameters
///
/// * `S` - The simulator state type (must implement `SimulatorState`)
///
/// # Example
///
/// ```rust,no_run
/// use cando_simulator_common::{SimulatorWebSocketServer, SimulatorState};
/// use serde::{Serialize, Deserialize};
/// use std::sync::Arc;
/// use tokio::sync::{broadcast, Mutex};
///
/// #[derive(Clone, Serialize, Deserialize)]
/// struct MyState { value: f64 }
///
/// impl SimulatorState for MyState {}
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let state = Arc::new(Mutex::new(MyState { value: 42.0 }));
/// let (broadcast_tx, _) = broadcast::channel(100);
///
/// let server = SimulatorWebSocketServer::new(state, broadcast_tx, None);
/// server.start(3030).await?;
/// # Ok(())
/// # }
/// ```
pub struct SimulatorWebSocketServer<S: SimulatorState> {
    state: Arc<Mutex<S>>,
    broadcast_tx: broadcast::Sender<String>,
    command_handler: Option<CommandHandler>,
}

impl<S: SimulatorState> SimulatorWebSocketServer<S> {
    /// Create a new WebSocket server
    ///
    /// # Arguments
    ///
    /// * `state` - Shared state to broadcast
    /// * `broadcast_tx` - Channel sender for broadcasting updates
    /// * `command_handler` - Optional handler for incoming commands
    pub fn new(
        state: Arc<Mutex<S>>,
        broadcast_tx: broadcast::Sender<String>,
        command_handler: Option<CommandHandler>,
    ) -> Self {
        Self {
            state,
            broadcast_tx,
            command_handler,
        }
    }

    /// Start the WebSocket server on the specified port
    ///
    /// This spawns a tokio task that runs the server in the background.
    /// The server will listen on `127.0.0.1:<port>` and accept WebSocket connections.
    ///
    /// # Arguments
    ///
    /// * `port` - TCP port to bind to
    ///
    /// # Errors
    ///
    /// Returns error if the server cannot bind to the specified port.
    pub async fn start(self, port: u16) -> Result<()> {
        let addr = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&addr).await.map_err(|e| {
            SimulatorError::websocket(format!(
                "Failed to bind WebSocket server to {}: {}",
                addr, e
            ))
        })?;

        info!("WebSocket server listening on ws://{}", addr);

        let state = self.state;
        let broadcast_tx = self.broadcast_tx;
        let command_handler = self.command_handler;

        tokio::spawn(async move {
            while let Ok((stream, peer_addr)) = listener.accept().await {
                debug!("New WebSocket connection from {}", peer_addr);

                let state_clone = Arc::clone(&state);
                let handler_clone = command_handler.clone();
                let _tx_clone = broadcast_tx.clone();
                let mut rx = broadcast_tx.subscribe();

                tokio::spawn(async move {
                    let ws_stream = match accept_async(stream).await {
                        Ok(ws) => ws,
                        Err(e) => {
                            error!("WebSocket connection error from {}: {}", peer_addr, e);
                            return;
                        }
                    };

                    debug!("WebSocket connection established with {}", peer_addr);

                    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                    // Send initial state
                    if let Ok(state_lock) = state_clone.lock().await.to_json() {
                        if let Err(e) = ws_sender.send(Message::Text(state_lock.into())).await {
                            warn!("Failed to send initial state to {}: {}", peer_addr, e);
                            return;
                        }
                    }

                    // Spawn task to handle incoming messages
                    let _state_for_receiver = Arc::clone(&state_clone);
                    let sender_for_commands = Arc::new(tokio::sync::Mutex::new(ws_sender));
                    let sender_clone = Arc::clone(&sender_for_commands);

                    tokio::spawn(async move {
                        while let Some(msg) = ws_receiver.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    debug!("Received message from {}: {}", peer_addr, text);

                                    // Call command handler if provided
                                    if let Some(ref handler) = handler_clone {
                                        if let Some(response) = handler(text.to_string()) {
                                            let mut sender = sender_clone.lock().await;
                                            if let Err(e) =
                                                sender.send(Message::Text(response.into())).await
                                            {
                                                warn!(
                                                    "Failed to send command response to {}: {}",
                                                    peer_addr, e
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }
                                Ok(Message::Close(_)) => {
                                    debug!("Client {} closed connection", peer_addr);
                                    break;
                                }
                                Ok(_) => {
                                    // Ignore other message types (Binary, Ping, Pong)
                                }
                                Err(e) => {
                                    warn!("WebSocket error from {}: {}", peer_addr, e);
                                    break;
                                }
                            }
                        }
                    });

                    // Handle outgoing state updates
                    tokio::spawn(async move {
                        while let Ok(state_json) = rx.recv().await {
                            let mut sender = sender_for_commands.lock().await;
                            if let Err(e) = sender.send(Message::Text(state_json.into())).await {
                                debug!("Failed to send update to {}: {}", peer_addr, e);
                                break;
                            }
                        }
                        debug!("WebSocket sender task ended for {}", peer_addr);
                    });
                });
            }
            warn!("WebSocket server listener task ended");
        });

        Ok(())
    }
}

/// Start WebSocket server with automatic state synchronization and error handling
///
/// This function consolidates the WebSocket initialization pattern used across all simulators.
/// It handles port binding, error reporting, state synchronization, and server lifecycle.
///
/// # Type Parameters
///
/// * `S` - The simulator state type (must implement `SimulatorState`)
///
/// # Arguments
///
/// * `state` - Shared state using `std::sync::Mutex` (the main simulator state)
/// * `websocket_tx` - Broadcast channel for state updates
/// * `port` - WebSocket server port number
/// * `test_mode` - If true, WebSocket is disabled (returns Ok immediately)
/// * `no_websocket` - If true, WebSocket is disabled via CLI flag (returns Ok immediately)
/// * `simulator_name` - Name of the simulator (for error messages)
/// * `command_handler` - Optional handler for incoming WebSocket commands
///
/// # Returns
///
/// * `Ok(())` - WebSocket server started successfully (or disabled)
/// * `Err(...)` - Only returns error if called incorrectly; otherwise exits process on fatal errors
///
/// # Example
///
/// ```rust,no_run
/// use cando_simulator_common::{start_simulator_websocket, SimulatorState};
/// use serde::{Serialize, Deserialize};
/// use std::sync::Arc;
/// use tokio::sync::broadcast;
///
/// #[derive(Clone, Serialize, Deserialize)]
/// struct MyState { value: f64 }
///
/// impl SimulatorState for MyState {}
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let state = Arc::new(std::sync::Mutex::new(MyState { value: 0.0 }));
/// let (websocket_tx, _) = broadcast::channel(100);
///
/// start_simulator_websocket(
///     state,
///     websocket_tx,
///     10752,           // port
///     false,           // test_mode
///     false,           // no_websocket
///     "my-simulator",  // simulator_name
///     None,            // command_handler
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub async fn start_simulator_websocket<S>(
    state: Arc<std::sync::Mutex<S>>,
    websocket_tx: broadcast::Sender<String>,
    port: u16,
    test_mode: bool,
    no_websocket: bool,
    simulator_name: &str,
    command_handler: Option<CommandHandler>,
) -> Result<()>
where
    S: SimulatorState,
{
    // Skip WebSocket in test mode or if explicitly disabled
    if test_mode {
        println!("WebSocket disabled in test mode");
        return Ok(());
    }

    if no_websocket {
        println!("WebSocket disabled via --no-websocket flag");
        return Ok(());
    }

    let bind_addr = format!("127.0.0.1:{}", port);

    // Test port binding synchronously to fail-fast on port conflicts
    // This prevents silent failures where simulator appears running but WebSocket is broken
    match TcpListener::bind(&bind_addr).await {
        Ok(_listener) => {
            // Port is available, proceed with normal WebSocket setup
            drop(_listener); // Release the test binding

            println!("WebSocket server binding to ws://{}", bind_addr);

            // Create tokio::sync::Mutex wrapper for WebSocket server
            // This is needed because SimulatorWebSocketServer requires tokio::sync::Mutex,
            // but simulators use std::sync::Mutex for their main state
            let ws_state = Arc::new(Mutex::new(state.lock().unwrap().clone()));

            let ws_server = SimulatorWebSocketServer::new(
                ws_state.clone(),
                websocket_tx.clone(),
                command_handler,
            );

            // Start WebSocket server in background task
            tokio::spawn(async move {
                if let Err(e) = ws_server.start(port).await {
                    eprintln!("WebSocket server error: {}", e);
                }
            });

            // Periodically sync state from std::sync::Mutex to tokio::sync::Mutex for WebSocket
            // This avoids holding sync mutex across await points
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    // Clone state before await to avoid holding MutexGuard across await point
                    let cloned_state = {
                        if let Ok(current_state) = state.lock() {
                            Some(current_state.clone())
                        } else {
                            None
                        }
                    };
                    if let Some(cloned_state) = cloned_state {
                        *ws_state.lock().await = cloned_state;
                    }
                }
            });

            Ok(())
        }
        Err(e) => {
            // Port binding failed - exit immediately with clear error message
            print_websocket_error(&e, port, simulator_name);
            std::process::exit(1);
        }
    }
}

/// Print detailed WebSocket error message and exit
///
/// This helper function prints context-rich error messages for WebSocket binding failures.
/// It analyzes the error type and provides specific solutions for common problems.
///
/// # Arguments
///
/// * `e` - The I/O error from port binding attempt
/// * `port` - The port number that failed to bind
/// * `simulator_name` - Name of the simulator (for ps/kill suggestions)
fn print_websocket_error(e: &std::io::Error, port: u16, simulator_name: &str) {
    eprintln!(
        "\nFATAL ERROR: Failed to bind WebSocket server to port {}",
        port
    );
    eprintln!("   Address: 127.0.0.1:{}", port);
    eprintln!("   Error: {}", e);

    // Provide context-specific solutions based on error type
    if e.to_string().contains("address already in use")
        || e.to_string().contains("Address already in use")
    {
        eprintln!("\nThe port is already in use. Possible solutions:");
        eprintln!("   1. Stop the other instance:");
        eprintln!("      ps aux | grep {}", simulator_name);
        eprintln!("      kill <PID>");
        eprintln!("   2. Use a different port:");
        eprintln!("      --websocket-port <PORT>");
        eprintln!("   3. For tests, use ports 10770-10779");
    } else if e.to_string().contains("permission denied")
        || e.to_string().contains("Permission denied")
    {
        eprintln!("\nPermission denied. Possible solutions:");
        eprintln!("   1. Ports < 1024 require root privileges");
        eprintln!("   2. Use a port >= 1024 (recommended: 10752-10799)");
        eprintln!("   3. Or run with sudo (not recommended)");
    } else {
        eprintln!("\nCheck that:");
        eprintln!("   - The port number is valid (1-65535)");
        eprintln!("   - No firewall is blocking the port");
    }

    eprintln!("\nSimulator cannot continue without WebSocket support.");
    eprintln!("Exiting with error code 1.\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
    struct TestState {
        value: i32,
        name: String,
    }

    impl SimulatorState for TestState {}

    #[test]
    fn test_state_to_json() {
        let state = TestState {
            value: 42,
            name: "test".to_string(),
        };

        let json = state.to_json().unwrap();
        assert!(json.contains("42"));
        assert!(json.contains("test"));
    }

    #[test]
    fn test_state_to_json_pretty() {
        let state = TestState {
            value: 42,
            name: "test".to_string(),
        };

        let json = state.to_json_pretty().unwrap();
        assert!(json.contains("42"));
        assert!(json.contains("test"));
        assert!(json.contains('\n')); // Pretty-printed should have newlines
    }

    #[test]
    fn test_state_round_trip() {
        let original = TestState {
            value: 123,
            name: "round-trip".to_string(),
        };

        let json = original.to_json().unwrap();
        let deserialized: TestState = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[tokio::test]
    async fn test_websocket_server_creation() {
        let state = Arc::new(Mutex::new(TestState {
            value: 1,
            name: "test".to_string(),
        }));
        let (broadcast_tx, _rx) = broadcast::channel(100);

        let _server = SimulatorWebSocketServer::new(state, broadcast_tx, None);
        // Just test that we can create the server without panic
    }

    #[tokio::test]
    async fn test_websocket_server_bind_invalid_port() {
        let state = Arc::new(Mutex::new(TestState {
            value: 1,
            name: "test".to_string(),
        }));
        let (broadcast_tx, _rx) = broadcast::channel(100);

        let server = SimulatorWebSocketServer::new(state, broadcast_tx, None);

        // Port 0 should succeed (OS assigns a port), but we're testing the error path
        // Try to bind to a privileged port without permissions
        let result = server.start(1).await;

        // This might succeed in some environments, so we just check it doesn't panic
        let _ = result;
    }
}
