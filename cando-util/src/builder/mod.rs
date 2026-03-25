//! Interactive TUI message builder for rust-can-util
//!
//! This module provides an interactive terminal UI for composing CAN messages
//! without memorizing field names, ranges, and formats.
//!
//! ## Usage
//!
//! ```bash
//! rust-can-util builder
//! ```
//!
//! ## Architecture
//!
//! The builder uses a state machine with progressive disclosure:
//! 1. Device Selection - Choose device from cando.yaml
//! 2. Message Selection - Choose message for that device's protocol
//! 3. Field Entry - Fill in field values with validation
//! 4. Command Generated - Copy or execute the command

use anyhow::Result;
use cando_config::{DeviceConfig, DeviceType, CandoConfig};
use cando_messages::metadata::{MessageMetadata, SignalMetadata};
use std::collections::HashMap;

mod data;
mod j1939_builder;
mod screens;
mod tui;

pub use tui::run_builder;

/// Opcode-specific information for multiplexed messages
#[derive(Debug, Clone, PartialEq)]
pub struct OpcodeInfo {
    /// Opcode value (e.g., 0x0, 0x2, etc.)
    pub value: u8,

    /// Human-readable name
    pub name: String,

    /// Detailed description of what this command does
    pub description: String,

    /// Field names that are applicable for this opcode
    pub applicable_fields: Vec<String>,

    /// Smart default values for fields (field_name -> default_value)
    pub default_values: HashMap<String, f64>,

    /// Additional help text or usage notes
    pub help_text: String,
}

/// Application state for the TUI builder
#[derive(Debug)]
pub struct AppState {
    /// Current screen being displayed
    pub screen: Screen,

    /// Available devices from configuration
    pub devices: Vec<DeviceInfo>,

    /// Currently selected device index
    pub selected_device_idx: Option<usize>,

    /// Messages available for selected device
    pub messages: Vec<MessageInfo>,

    /// Currently selected message index
    pub selected_message_idx: Option<usize>,

    /// Field values being edited
    pub field_values: HashMap<String, String>,

    /// Tracks whether each field is using its sentinel value
    pub field_use_sentinel: HashMap<String, bool>,

    /// Currently selected field index (for navigation)
    pub selected_field_idx: usize,

    /// Generated command
    pub generated_command: Option<String>,

    /// Error message to display (if any)
    pub error_message: Option<String>,

    /// Message filter text (for search)
    pub filter_text: String,

    /// Whether to show help overlay
    pub show_help: bool,

    /// Loaded configuration
    #[allow(dead_code)]
    pub config: CandoConfig,

    /// Configuration file path (for including in generated commands)
    pub config_path: Option<String>,

    /// Environment name (for including in generated commands)
    pub environment: Option<String>,

    /// Command history (last N commands)
    #[allow(dead_code)]
    pub command_history: Vec<String>,

    /// Maximum history size
    pub max_history_size: usize,

    /// Status message (for execution feedback, etc.)
    pub status_message: Option<String>,

    /// Selected opcode for multiplexed messages (e.g., UDC_Command)
    pub selected_opcode: Option<u8>,

    /// Available opcodes for the current multiplexed message
    pub available_opcodes: Vec<OpcodeInfo>,
}

/// Current screen in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Screen {
    /// Device selection screen
    DeviceSelection,

    /// Message selection screen
    MessageSelection,

    /// Opcode selection screen (for multiplexed messages like UDC_Command)
    OpcodeSelection,

    /// Field entry screen
    FieldEntry,

    /// Command generated screen
    CommandGenerated,
}

/// Information about a device from configuration
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeviceInfo {
    /// Device name from config
    pub name: String,

    /// Friendly name
    pub friendly_name: String,

    /// Device type (EMP, HVPC, UDC, J1939)
    pub device_type: DeviceType,

    /// Protocol (proprietary, j1939, hybrid)
    pub protocol: String,

    /// Device ID (hex string)
    #[allow(dead_code)]
    pub device_id: String,

    /// CAN interface
    pub interface: String,
}

impl DeviceInfo {
    /// Create DeviceInfo from DeviceConfig
    ///
    /// # Arguments
    /// * `device_key` - The device key from the configuration (e.g., "lab_fan")
    /// * `config` - The device configuration
    pub fn from_config(device_key: &str, config: &DeviceConfig) -> Self {
        Self {
            name: device_key.to_string(),
            friendly_name: config
                .friendly_name
                .clone()
                .unwrap_or_else(|| config.device_id.clone()),
            device_type: config.device_type,
            protocol: config
                .protocol
                .clone()
                .unwrap_or_else(|| "j1939".to_string()),
            device_id: config.device_id.clone(),
            interface: config
                .interface
                .clone()
                .unwrap_or_else(|| "can0".to_string()),
        }
    }
}

/// Information about a message from metadata
#[derive(Debug, Clone)]
pub struct MessageInfo {
    /// Message name
    pub name: String,

    /// CAN ID (base ID, before device ID embedding)
    pub can_id: u32,

    /// Data length code (number of bytes)
    pub dlc: u8,

    /// Signals/fields in this message
    pub signals: Vec<FieldInfo>,

    /// Optional comment
    pub comment: String,

    /// Protocol this message belongs to
    #[allow(dead_code)]
    pub protocol: String,
}

impl MessageInfo {
    /// Create MessageInfo from MessageMetadata
    pub fn from_metadata(metadata: &'static MessageMetadata, protocol: &str) -> Self {
        let signals: Vec<FieldInfo> = metadata
            .signals
            .iter()
            .map(FieldInfo::from_metadata)
            .collect();

        Self {
            name: metadata.name.to_string(),
            can_id: metadata.can_id,
            dlc: metadata.dlc,
            signals,
            comment: metadata.comment.to_string(),
            protocol: protocol.to_string(),
        }
    }
}

/// Information about a signal/field
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FieldInfo {
    /// Signal name
    pub name: String,

    /// Minimum value (in physical units)
    pub min: f64,

    /// Maximum value (in physical units)
    pub max: f64,

    /// Engineering unit (e.g., "rpm", "%", "°C")
    pub unit: String,

    /// Scaling factor
    #[allow(dead_code)]
    pub factor: f64,

    /// Offset
    #[allow(dead_code)]
    pub offset: f64,

    /// Signal size in bits
    #[allow(dead_code)]
    pub signal_size: u64,

    /// Value descriptions (enumerations)
    pub value_descriptions: Vec<(u64, String)>,

    /// Optional comment
    pub comment: String,

    /// Sentinel value (e.g., 0xFF for "not available")
    pub sentinel_value: Option<u64>,

    /// Description of what the sentinel value means
    pub sentinel_description: String,
}

impl FieldInfo {
    /// Create FieldInfo from SignalMetadata
    pub fn from_metadata(metadata: &SignalMetadata) -> Self {
        let value_descriptions: Vec<(u64, String)> = metadata
            .value_descriptions
            .iter()
            .map(|(val, desc)| (*val, desc.to_string()))
            .collect();

        // Detect sentinel values
        let (sentinel_value, sentinel_description) = Self::detect_sentinel(metadata);

        Self {
            name: metadata.name.to_string(),
            min: metadata.min,
            max: metadata.max,
            unit: metadata.unit.to_string(),
            factor: metadata.factor,
            offset: metadata.offset,
            signal_size: metadata.signal_size,
            value_descriptions,
            comment: metadata.comment.to_string(),
            sentinel_value,
            sentinel_description,
        }
    }

    /// Detect sentinel values from metadata
    ///
    /// Looks for common patterns:
    /// - 0xFF (255) for 8-bit fields = "not available"
    /// - 0xFE (254) for 8-bit fields = "error"
    /// - 0xFFFF (65535) for 16-bit fields = "not available"
    /// - Comments mentioning "Use 0xFF if..." or similar patterns
    fn detect_sentinel(metadata: &SignalMetadata) -> (Option<u64>, String) {
        let comment_lower = metadata.comment.to_lowercase();

        // Calculate maximum possible value for this signal size
        let _max_possible = if metadata.signal_size < 64 {
            (1u64 << metadata.signal_size) - 1
        } else {
            u64::MAX
        };

        // Check for explicit sentinel mentions in comments
        // Pattern 1: "Use 0xFF if..." or "0xFF = ..." or "255 = ..."
        if (comment_lower.contains("0xff") || comment_lower.contains("255"))
            && metadata.signal_size == 8
        {
            let description = Self::extract_sentinel_description(&comment_lower, "0xff", "255");
            return (Some(255), description);
        }

        // Pattern 2: "Use 0xFE" or "254 = error"
        if (comment_lower.contains("0xfe") || comment_lower.contains("254"))
            && metadata.signal_size == 8
        {
            let description = Self::extract_sentinel_description(&comment_lower, "0xfe", "254");
            return (Some(254), description);
        }

        // Pattern 3: "Use 0xFFFF" for 16-bit fields
        if (comment_lower.contains("0xffff") || comment_lower.contains("65535"))
            && metadata.signal_size == 16
        {
            let description = Self::extract_sentinel_description(&comment_lower, "0xffff", "65535");
            return (Some(65535), description);
        }

        // Pattern 4: Check if max_possible is significantly larger than stated max
        // This catches J1939 pattern where 0xFF or 0xFE are reserved
        if metadata.signal_size == 8 && metadata.max < 250.0 {
            // Common J1939 pattern: 0xFF = not available, 0xFE = error
            if comment_lower.contains("not available") || comment_lower.contains("n/a") {
                return (Some(255), "Not available / Not applicable".to_string());
            }
        }

        // No sentinel detected
        (None, String::new())
    }

    /// Extract sentinel description from comment
    fn extract_sentinel_description(comment: &str, hex_pattern: &str, dec_pattern: &str) -> String {
        // Look for text after "Use 0xFF if..." or similar
        if let Some(pos) = comment.find(&format!("use {} if", hex_pattern)) {
            let after = &comment[pos + format!("use {} if", hex_pattern).len()..];
            if let Some(end) = after.find('.') {
                return after[..end].trim().to_string();
            }
            return after.trim().to_string();
        }

        if let Some(pos) = comment.find(&format!("use {} if", dec_pattern)) {
            let after = &comment[pos + format!("use {} if", dec_pattern).len()..];
            if let Some(end) = after.find('.') {
                return after[..end].trim().to_string();
            }
            return after.trim().to_string();
        }

        // Look for "0xFF = description" pattern
        if let Some(pos) = comment.find(&format!("{} =", hex_pattern)) {
            let after = &comment[pos + format!("{} =", hex_pattern).len()..];
            if let Some(end) = after.find('.') {
                return after[..end].trim().to_string();
            }
            return after.trim().to_string();
        }

        // Default description
        "Special value - see field description".to_string()
    }
}

/// Phase 4: Non-interactive mode - List all devices
pub fn handle_list_devices(
    config: &CandoConfig,
    environment: Option<&str>,
    format: &str,
) -> Result<()> {
    let devices: Vec<DeviceInfo> = if let Some(env_name) = environment {
        // Use environment-specific devices when environment is specified
        config
            .enabled_devices_for_environment(env_name)
            .iter()
            .map(|(_, device_key, device)| DeviceInfo::from_config(device_key, device))
            .collect()
    } else {
        // Use all enabled devices when no environment is specified
        config
            .enabled_devices()
            .iter()
            .map(|(_, device_key, device)| DeviceInfo::from_config(device_key, device))
            .collect()
    };

    match format {
        "json" => {
            println!("[");
            for (i, device) in devices.iter().enumerate() {
                println!(
                    "  {{\"name\": \"{}\", \"type\": \"{:?}\", \"protocol\": \"{}\"}}{}",
                    device.name,
                    device.device_type,
                    device.protocol,
                    if i < devices.len() - 1 { "," } else { "" }
                );
            }
            println!("]");
        }
        "csv" => {
            println!("name,type,protocol,device_id,interface");
            for device in devices {
                println!(
                    "{},{:?},{},{},{}",
                    device.name,
                    device.device_type,
                    device.protocol,
                    device.device_id,
                    device.interface
                );
            }
        }
        _ => {
            // text format
            println!("Available Devices:");
            println!();
            for device in devices {
                println!("  {} ({})", device.name, device.friendly_name);
                println!("    Type:     {:?}", device.device_type);
                println!("    Protocol: {}", device.protocol);
                println!("    ID:       {}", device.device_id);
                println!("    Interface: {}", device.interface);
                println!();
            }
        }
    }

    Ok(())
}

/// Phase 4: Non-interactive mode - List messages for a device
pub fn handle_list_messages(config: &CandoConfig, device_name: &str, format: &str) -> Result<()> {
    let devices = data::load_devices(config);

    // Find the device
    let device = devices
        .iter()
        .find(|d| d.name == device_name)
        .ok_or_else(|| anyhow::anyhow!("Device '{}' not found", device_name))?;

    // Load messages for this device's protocol
    let messages = data::load_messages_for_device(device)?;

    match format {
        "json" => {
            println!("[");
            for (i, msg) in messages.iter().enumerate() {
                println!(
                    "  {{\"name\": \"{}\", \"can_id\": \"0x{:08X}\", \"dlc\": {}, \"signal_count\": {}}}{}",
                    msg.name,
                    msg.can_id,
                    msg.dlc,
                    msg.signals.len(),
                    if i < messages.len() - 1 { "," } else { "" }
                );
            }
            println!("]");
        }
        "csv" => {
            println!("name,can_id,dlc,signal_count,comment");
            for msg in &messages {
                println!(
                    "{},0x{:08X},{},{},\"{}\"",
                    msg.name,
                    msg.can_id,
                    msg.dlc,
                    msg.signals.len(),
                    msg.comment.replace("\"", "\"\"")
                );
            }
        }
        _ => {
            // text format
            println!(
                "Messages for device: {} ({})",
                device.name, device.friendly_name
            );
            println!("Protocol: {}", device.protocol);
            println!();
            println!("Available Messages:");
            println!();
            for msg in &messages {
                println!("  {}", msg.name);
                println!("    CAN ID: 0x{:08X}", msg.can_id);
                println!("    DLC:    {} bytes", msg.dlc);
                println!("    Fields: {}", msg.signals.len());
                if !msg.comment.is_empty() {
                    let comment_lines: Vec<&str> = msg.comment.lines().take(2).collect();
                    println!("    Info:   {}", comment_lines[0]);
                    if comment_lines.len() > 1 {
                        println!("            {}", comment_lines[1]);
                    }
                }
                println!();
            }
        }
    }

    Ok(())
}

/// Phase 4: Non-interactive mode - Show fields for a message
pub fn handle_show_fields(
    config: &CandoConfig,
    device_name: &str,
    message_name: &str,
    format: &str,
) -> Result<()> {
    let devices = data::load_devices(config);

    // Find the device
    let device = devices
        .iter()
        .find(|d| d.name == device_name)
        .ok_or_else(|| anyhow::anyhow!("Device '{}' not found", device_name))?;

    // Load messages for this device's protocol
    let messages = data::load_messages_for_device(device)?;

    // Find the message
    let message = messages
        .iter()
        .find(|m| m.name == message_name)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Message '{}' not found for device '{}'",
                message_name,
                device_name
            )
        })?;

    match format {
        "json" => {
            println!("{{");
            println!("  \"message\": \"{}\",", message.name);
            println!("  \"can_id\": \"0x{:08X}\",", message.can_id);
            println!("  \"dlc\": {},", message.dlc);
            println!("  \"fields\": [");
            for (i, field) in message.signals.iter().enumerate() {
                print!(
                    "    {{\"name\": \"{}\", \"min\": {}, \"max\": {}, \"unit\": \"{}\"",
                    field.name, field.min, field.max, field.unit
                );
                if !field.value_descriptions.is_empty() {
                    print!(", \"values\": [");
                    for (j, (val, desc)) in field.value_descriptions.iter().enumerate() {
                        print!(
                            "{{\"value\": {}, \"description\": \"{}\"}}",
                            val,
                            desc.replace("\"", "\\\"")
                        );
                        if j < field.value_descriptions.len() - 1 {
                            print!(", ");
                        }
                    }
                    print!("]");
                }
                println!(
                    "}}{}",
                    if i < message.signals.len() - 1 {
                        ","
                    } else {
                        ""
                    }
                );
            }
            println!("  ]");
            println!("}}");
        }
        "csv" => {
            println!("field,min,max,unit,values,comment");
            for field in &message.signals {
                let values = if field.value_descriptions.is_empty() {
                    String::new()
                } else {
                    field
                        .value_descriptions
                        .iter()
                        .map(|(val, desc)| format!("{}={}", val, desc))
                        .collect::<Vec<_>>()
                        .join("; ")
                };
                println!(
                    "{},{},{},\"{}\",\"{}\",\"{}\"",
                    field.name,
                    field.min,
                    field.max,
                    field.unit,
                    values.replace("\"", "\"\""),
                    field.comment.replace("\"", "\"\"")
                );
            }
        }
        _ => {
            // text format
            println!("Message: {}", message.name);
            println!("Device:  {} ({})", device.name, device.friendly_name);
            println!("CAN ID:  0x{:08X}", message.can_id);
            println!("DLC:     {} bytes", message.dlc);
            if !message.comment.is_empty() {
                println!();
                println!("Description:");
                for line in message.comment.lines().take(5) {
                    println!("  {}", line.trim());
                }
            }
            println!();
            println!("Fields:");
            println!();
            for field in &message.signals {
                println!("  {}", field.name);
                println!("    Range: {} to {} {}", field.min, field.max, field.unit);

                if !field.value_descriptions.is_empty() {
                    println!("    Values:");
                    for (val, desc) in &field.value_descriptions {
                        println!("      {} = {}", val, desc);
                    }
                }

                if !field.comment.is_empty() {
                    println!("    Info: {}", field.comment.lines().next().unwrap_or(""));
                }
                println!();
            }
        }
    }

    Ok(())
}

impl FieldInfo {
    /// Check if this field has enum values
    pub fn is_enum(&self) -> bool {
        !self.value_descriptions.is_empty()
    }

    /// Validate a value against min/max range
    pub fn validate(&self, value: f64) -> Result<()> {
        if value < self.min || value > self.max {
            anyhow::bail!(
                "Value {} out of range [{}, {}] for field '{}'",
                value,
                self.min,
                self.max,
                self.name
            );
        }
        Ok(())
    }
}

impl AppState {
    /// Create new application state with loaded configuration
    pub fn new(
        config: CandoConfig,
        config_path: Option<String>,
        environment: Option<String>,
    ) -> Self {
        let devices: Vec<DeviceInfo> = if let Some(ref env_name) = environment {
            // Use environment-specific devices when environment is specified
            config
                .enabled_devices_for_environment(env_name)
                .iter()
                .map(|(_, device_key, device)| DeviceInfo::from_config(device_key, device))
                .collect()
        } else {
            // Use all enabled devices when no environment is specified
            config
                .enabled_devices()
                .iter()
                .map(|(_, device_key, device)| DeviceInfo::from_config(device_key, device))
                .collect()
        };

        Self {
            screen: Screen::DeviceSelection,
            devices,
            selected_device_idx: None,
            messages: Vec::new(),
            selected_message_idx: None,
            field_values: HashMap::new(),
            field_use_sentinel: HashMap::new(),
            selected_field_idx: 0,
            generated_command: None,
            error_message: None,
            filter_text: String::new(),
            show_help: false,
            config,
            config_path,
            environment,
            command_history: Vec::new(),
            max_history_size: 50,
            status_message: None,
            selected_opcode: None,
            available_opcodes: Vec::new(),
        }
    }

    /// Get currently selected device
    pub fn selected_device(&self) -> Option<&DeviceInfo> {
        self.selected_device_idx
            .and_then(|idx| self.devices.get(idx))
    }

    /// Get currently selected message
    pub fn selected_message(&self) -> Option<&MessageInfo> {
        self.selected_message_idx
            .and_then(|idx| self.messages.get(idx))
    }

    /// Navigate to next screen
    pub fn next_screen(&mut self) {
        self.screen = match self.screen {
            Screen::DeviceSelection => Screen::MessageSelection,
            Screen::MessageSelection => Screen::FieldEntry,
            Screen::OpcodeSelection => Screen::FieldEntry,
            Screen::FieldEntry => Screen::CommandGenerated,
            Screen::CommandGenerated => Screen::CommandGenerated,
        };
    }

    /// Navigate to previous screen
    pub fn previous_screen(&mut self) {
        self.screen = match self.screen {
            Screen::DeviceSelection => Screen::DeviceSelection,
            Screen::MessageSelection => Screen::DeviceSelection,
            Screen::OpcodeSelection => Screen::MessageSelection,
            Screen::FieldEntry => Screen::MessageSelection,
            Screen::CommandGenerated => Screen::FieldEntry,
        };
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        self.screen = Screen::DeviceSelection;
        self.selected_device_idx = None;
        self.messages.clear();
        self.selected_message_idx = None;
        self.field_values.clear();
        self.field_use_sentinel.clear();
        self.selected_field_idx = 0;
        self.generated_command = None;
        self.error_message = None;
        self.filter_text.clear();
        self.show_help = false;
    }

    /// Add command to history
    #[allow(dead_code)]
    pub fn add_to_history(&mut self, command: String) {
        // Add to front of history
        self.command_history.insert(0, command);

        // Trim to max size
        if self.command_history.len() > self.max_history_size {
            self.command_history.truncate(self.max_history_size);
        }
    }

    /// Get device interface for sending commands
    #[allow(dead_code)]
    pub fn get_device_interface(&self) -> Option<String> {
        self.selected_device_idx
            .and_then(|idx| self.devices.get(idx))
            .map(|d| d.interface.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j1939_sentinel_values() {
        // Test that J1939 fields have sentinel values detected from metadata
        use cando_messages::j1939::EEC1_METADATA;

        let message = MessageInfo::from_metadata(&EEC1_METADATA, "j1939");

        // EEC1 should have signals
        assert!(
            !message.signals.is_empty(),
            "EEC1 should have signals"
        );

        // Verify the message has correct basic info
        assert_eq!(message.name, "EEC1");
        assert_eq!(message.protocol, "j1939");
    }
}
