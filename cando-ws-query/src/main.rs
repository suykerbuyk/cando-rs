//! WebSocket Query Tool for Simulator State Validation
//!
//! This tool queries simulator state via WebSocket for integration testing.
//!
//! # Usage
//!
//! Query full state:
//! ```bash
//! cando-ws-query --port 10999 query
//! ```
//!
//! Extract a specific field:
//! ```bash
//! cando-ws-query --port 10999 extract eec12_engnexhst1gssnsr1pwrsppl
//! ```
//!
//! Validate a field value:
//! ```bash
//! cando-ws-query --port 10999 validate eec12_engnexhst1gssnsr1pwrsppl 2
//! cando-ws-query --port 10999 validate temperature 25.5 --tolerance 0.5
//! ```
//!
//! Pause simulator broadcasting (for test isolation):
//! ```bash
//! cando-ws-query --port 10999 pause
//! ```
//!
//! Resume simulator broadcasting:
//! ```bash
//! cando-ws-query --port 10999 resume
//! ```
//!
//! Reset simulator state to defaults:
//! ```bash
//! cando-ws-query --port 10999 reset
//! ```

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::Value;
use tokio::time::{Duration, timeout};
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// WebSocket query tool for simulator state validation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// WebSocket port number
    #[arg(short, long)]
    port: u16,

    /// Connection timeout in seconds
    #[arg(short, long, default_value = "5")]
    timeout: u64,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Query full simulator state and output as JSON
    Query,

    /// Extract a specific field value from simulator state
    Extract {
        /// Field name to extract (e.g., "eec12_engnexhst1gssnsr1pwrsppl")
        field_name: String,
    },

    /// Validate that a field matches expected value
    Validate {
        /// Field name to validate
        field_name: String,

        /// Expected value (numeric or string)
        expected_value: String,

        /// Tolerance for floating point comparisons
        #[arg(long, default_value = "0.0")]
        tolerance: f64,
    },

    /// Pause simulator CAN message broadcasting (for test isolation)
    Pause,

    /// Resume simulator CAN message broadcasting
    Resume,

    /// Reset simulator state to defaults
    Reset,

    /// Wait for a specific CAN message to be received (deterministic test verification)
    WaitForMessage {
        /// CAN ID to wait for (hex format like 0x18FCCC0F)
        #[arg(value_parser = parse_can_id)]
        can_id: u32,

        /// Timeout in milliseconds
        #[arg(long, default_value = "1000")]
        timeout_ms: u64,
    },
}

/// Parse CAN ID from hex or decimal string
fn parse_can_id(s: &str) -> Result<u32, String> {
    if let Some(hex_str) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex_str, 16).map_err(|e| format!("Invalid hex CAN ID: {}", e))
    } else {
        s.parse::<u32>()
            .map_err(|e| format!("Invalid decimal CAN ID: {}", e))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Execute command
    match args.command {
        Command::Query => {
            // Query simulator state and output as pretty JSON
            let state = query_simulator_state(args.port, args.timeout).await?;
            println!("{}", serde_json::to_string_pretty(&state)?);
        }
        Command::Extract { field_name } => {
            // Query simulator state and extract field value
            let state = query_simulator_state(args.port, args.timeout).await?;
            let value = extract_field(&state, &field_name)?;
            println!("{}", value);
        }
        Command::Validate {
            field_name,
            expected_value,
            tolerance,
        } => {
            // Query simulator state and validate field value
            let state = query_simulator_state(args.port, args.timeout).await?;
            validate_field(&state, &field_name, &expected_value, tolerance)?;
            println!(
                "Field '{}' matches expected value: {} (actual: {})",
                field_name,
                expected_value,
                extract_field(&state, &field_name)?
            );
        }
        Command::Pause => {
            // Pause simulator broadcasting
            pause_broadcast(args.port, args.timeout).await?;
            println!("Simulator broadcasting paused");
        }
        Command::Resume => {
            // Resume simulator broadcasting
            resume_broadcast(args.port, args.timeout).await?;
            println!("Simulator broadcasting resumed");
        }
        Command::Reset => {
            // Reset simulator state to defaults
            reset_simulator(args.port, args.timeout).await?;
            println!("Simulator state reset to defaults");
        }
        Command::WaitForMessage { can_id, timeout_ms } => {
            // Wait for a specific CAN message to be received
            let result = wait_for_message(args.port, args.timeout, can_id, timeout_ms).await?;
            if result.found {
                println!(
                    "Message received: CAN ID 0x{:08X} at {}ms (waited {}ms)",
                    result.can_id, result.timestamp_ms, result.elapsed_ms
                );
            } else {
                println!(
                    "Message NOT received: CAN ID 0x{:08X} (timeout after {}ms)",
                    result.can_id, result.elapsed_ms
                );
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

/// Query simulator state via WebSocket
///
/// Sends a GetState message and returns the parsed JSON state.
/// Handles both wrapped and flat JSON response formats.
async fn query_simulator_state(port: u16, timeout_secs: u64) -> Result<Value> {
    let url = format!("ws://127.0.0.1:{}", port);

    // Connect with timeout
    let (ws_stream, _) = timeout(Duration::from_secs(timeout_secs), connect_async(&url))
        .await
        .context("Connection timeout")?
        .context("Failed to connect to WebSocket")?;

    let (mut write, mut read) = ws_stream.split();

    // Send GetState message
    let get_state_msg = r#"{"type":"GetState"}"#;
    use futures_util::{SinkExt, StreamExt};

    write
        .send(Message::Text(get_state_msg.to_string().into()))
        .await
        .context("Failed to send GetState message")?;

    // Receive response with timeout
    let response = timeout(Duration::from_secs(timeout_secs), read.next())
        .await
        .context("Response timeout")?
        .context("WebSocket connection closed")?
        .context("Failed to receive response")?;

    // Parse response
    let text = response
        .to_text()
        .context("Response is not valid text")?
        .to_string();
    let data: Value = serde_json::from_str(&text).context("Failed to parse JSON response")?;

    // Handle multiple response formats:
    // 1. StateResponse format (new): {"type": "StateResponse", "state_json": "{...}"}
    // 2. StateUpdate format (legacy): {"type": "StateUpdate", "state": {...}}
    // 3. Flat format: {...} (direct state dictionary)
    if let Some(obj) = data.as_object() {
        // Check for StateResponse format (state query framework)
        if obj.get("type") == Some(&Value::String("StateResponse".to_string()))
            && let Some(Value::String(state_json)) = obj.get("state_json") {
                // Parse the state_json string into a JSON object
                let state: Value =
                    serde_json::from_str(state_json).context("Failed to parse state_json field")?;
                return Ok(state);
            }

        // Check for StateUpdate format (legacy)
        if obj.get("type") == Some(&Value::String("StateUpdate".to_string()))
            && let Some(state) = obj.get("state") {
                // Wrapped format (legacy/future)
                return Ok(state.clone());
            }
    }

    // Flat state format (current simulator implementation)
    if data.is_object() {
        return Ok(data);
    }

    anyhow::bail!("Unexpected response format: {}", data);
}

/// Extract a specific field value from state
fn extract_field(state: &Value, field_name: &str) -> Result<String> {
    let value = state
        .get(field_name)
        .with_context(|| format!("Field '{}' not found in state", field_name))?;

    // Convert value to string representation
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Number(n) => Ok(n.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Null => Ok("null".to_string()),
        _ => Ok(serde_json::to_string(value)?),
    }
}

/// Validate that a field matches the expected value
fn validate_field(
    state: &Value,
    field_name: &str,
    expected_value: &str,
    tolerance: f64,
) -> Result<()> {
    let actual_value_str = extract_field(state, field_name)?;

    // Try numeric comparison first (with tolerance)
    if let (Ok(actual), Ok(expected)) = (
        actual_value_str.parse::<f64>(),
        expected_value.parse::<f64>(),
    ) {
        let diff = (actual - expected).abs();
        if diff <= tolerance {
            return Ok(());
        }
        anyhow::bail!(
            "Field '{}' value mismatch: expected {} +/- {}, got {} (diff: {})",
            field_name,
            expected,
            tolerance,
            actual,
            diff
        );
    }

    // Fall back to string comparison
    if actual_value_str == expected_value {
        return Ok(());
    }

    anyhow::bail!(
        "Field '{}' value mismatch: expected '{}', got '{}'",
        field_name,
        expected_value,
        actual_value_str
    );
}

/// Pause simulator CAN message broadcasting
///
/// Sends a PauseBroadcast message to stop the simulator from sending CAN frames.
/// This is useful for test isolation to prevent simulator's own broadcasts from
/// interfering with test message validation.
async fn pause_broadcast(port: u16, timeout_secs: u64) -> Result<()> {
    send_command(port, timeout_secs, r#"{"type":"PauseBroadcast"}"#).await
}

/// Resume simulator CAN message broadcasting
///
/// Sends a ResumeBroadcast message to allow the simulator to resume sending CAN frames.
async fn resume_broadcast(port: u16, timeout_secs: u64) -> Result<()> {
    send_command(port, timeout_secs, r#"{"type":"ResumeBroadcast"}"#).await
}

/// Reset simulator state to defaults
///
/// Sends a Reset message to restore the simulator to its default state.
async fn reset_simulator(port: u16, timeout_secs: u64) -> Result<()> {
    send_command(port, timeout_secs, r#"{"type":"Reset"}"#).await
}

/// Send a command to the simulator via WebSocket
///
/// Generic helper function for sending simple commands that don't require a response value.
async fn send_command(port: u16, timeout_secs: u64, command: &str) -> Result<()> {
    let url = format!("ws://127.0.0.1:{}", port);

    // Connect with timeout
    let (ws_stream, _) = timeout(Duration::from_secs(timeout_secs), connect_async(&url))
        .await
        .context("Connection timeout")?
        .context("Failed to connect to WebSocket")?;

    let (mut write, mut read) = ws_stream.split();

    // Send command message
    use futures_util::{SinkExt, StreamExt};

    write
        .send(Message::Text(command.to_string().into()))
        .await
        .context("Failed to send command message")?;

    // Receive response with timeout (simulator sends StateUpdate response)
    let _response = timeout(Duration::from_secs(timeout_secs), read.next())
        .await
        .context("Response timeout")?
        .context("WebSocket connection closed")?
        .context("Failed to receive response")?;

    // Command executed successfully (we don't need to parse the response)
    Ok(())
}

/// Result of waiting for a message
#[derive(Debug)]
struct WaitForMessageResult {
    can_id: u32,
    timestamp_ms: u64,
    elapsed_ms: u64,
    found: bool,
}

/// Wait for a specific CAN message to be received by the simulator
///
/// Sends a WaitForMessage command and returns the result indicating
/// whether the message was received within the timeout period.
async fn wait_for_message(
    port: u16,
    connection_timeout_secs: u64,
    can_id: u32,
    timeout_ms: u64,
) -> Result<WaitForMessageResult> {
    use futures_util::{SinkExt, StreamExt};
    use std::time::Instant;

    let url = format!("ws://127.0.0.1:{}", port);
    let start = Instant::now();
    let timeout_duration = Duration::from_millis(timeout_ms);
    let poll_interval = Duration::from_millis(10); // Poll every 10ms

    // Connect ONCE - maintain persistent connection during polling
    let (ws_stream, _) = timeout(
        Duration::from_secs(connection_timeout_secs),
        connect_async(&url),
    )
    .await
    .context("Connection timeout")?
    .context("Failed to connect to WebSocket")?;

    let (mut write, mut read) = ws_stream.split();

    loop {
        let elapsed = start.elapsed();

        if elapsed >= timeout_duration {
            // Timeout reached - return not found

            return Ok(WaitForMessageResult {
                can_id,
                timestamp_ms: 0,
                elapsed_ms: elapsed.as_millis() as u64,
                found: false,
            });
        }

        // Send WaitForMessage command on persistent connection
        let wait_msg = serde_json::json!({
            "type": "WaitForMessage",
            "can_id": can_id,
            "timeout_ms": 0, // Immediate check, no server-side waiting
        });

        write
            .send(Message::Text(wait_msg.to_string().into()))
            .await
            .context("Failed to send WaitForMessage command")?;

        // Read response(s), filtering out state update broadcasts
        let response_timeout = Duration::from_secs(2);
        loop {
            let response = timeout(response_timeout, read.next())
                .await
                .context("Response timeout")?
                .context("WebSocket connection closed")?
                .context("Failed to receive response")?;

            // Parse response
            let text = response
                .to_text()
                .context("Response is not valid text")?
                .to_string();

            let data: Value =
                serde_json::from_str(&text).context("Failed to parse JSON response")?;

            // Parse response type
            if let Some(obj) = data.as_object() {
                let msg_type = obj.get("type").and_then(|v| v.as_str());

                match msg_type {
                    Some("MessageReceived") => {
                        // This is our command response!
                        let found = obj
                            .get("found")
                            .and_then(|v| v.as_bool())
                            .context("Missing or invalid found")?;

                        if found {
                            // Message was found!

                            let result = WaitForMessageResult {
                                can_id: obj
                                    .get("can_id")
                                    .and_then(|v| v.as_u64())
                                    .context("Missing or invalid can_id")?
                                    as u32,
                                timestamp_ms: obj
                                    .get("timestamp_ms")
                                    .and_then(|v| v.as_u64())
                                    .context("Missing or invalid timestamp_ms")?,
                                elapsed_ms: start.elapsed().as_millis() as u64,
                                found: true,
                            };
                            return Ok(result);
                        } else {
                            // Message not found yet, break inner loop to continue polling

                            break;
                        }
                    }
                    Some("StateUpdate") => {
                        // State update broadcast - filter it out and keep reading

                        continue;
                    }
                    _ => {
                        // Unknown or unexpected message type
                        if obj.contains_key("device_id") {
                            // Likely a state object, filter it out

                            continue;
                        } else {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }

        // Sleep before next poll

        tokio::time::sleep(poll_interval).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_field_string() {
        let state = json!({"test_field": "hello"});
        assert_eq!(extract_field(&state, "test_field").unwrap(), "hello");
    }

    #[test]
    fn test_extract_field_number() {
        let state = json!({"test_field": 42});
        assert_eq!(extract_field(&state, "test_field").unwrap(), "42");
    }

    #[test]
    fn test_extract_field_bool() {
        let state = json!({"test_field": true});
        assert_eq!(extract_field(&state, "test_field").unwrap(), "true");
    }

    #[test]
    fn test_extract_field_not_found() {
        let state = json!({"other_field": "value"});
        assert!(extract_field(&state, "test_field").is_err());
    }

    #[test]
    fn test_validate_field_numeric_exact() {
        let state = json!({"test_field": 42});
        assert!(validate_field(&state, "test_field", "42", 0.0).is_ok());
    }

    #[test]
    fn test_validate_field_numeric_tolerance() {
        let state = json!({"test_field": 42.3});
        assert!(validate_field(&state, "test_field", "42.5", 0.5).is_ok());
        assert!(validate_field(&state, "test_field", "42.5", 0.1).is_err());
    }

    #[test]
    fn test_validate_field_string() {
        let state = json!({"test_field": "hello"});
        assert!(validate_field(&state, "test_field", "hello", 0.0).is_ok());
        assert!(validate_field(&state, "test_field", "world", 0.0).is_err());
    }
}
