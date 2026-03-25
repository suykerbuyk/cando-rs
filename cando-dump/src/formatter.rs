//! CAN frame formatting for candump-compatible output
//!
//! This module provides formatters for converting CAN frames to various
//! output formats: candump (standard), JSON, and decoded (protocol-aware).

use colored::Colorize;
use cando_can_monitor::SourcedFrame;
use serde_json::json;
use socketcan::{CanFrame, EmbeddedFrame, Id};
use std::collections::HashMap;

use cando_messages::common::{get_j1939_base_id, CAN_EFF_MASK};
use cando_messages::metadata::MessageMetadata;
use cando_messages::j1939::{
    self,
    diagnostics::{get_diagnostic_message_type, DiagnosticMessageType, DtcMessage, LampState},
    DM01, DM02, DM06, DM12, DM13, DM22, DM23,
};

/// Output format selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Standard candump format: (timestamp) interface CANID#DATA
    Candump,
    /// JSON format with structured data
    Json,
    /// Decoded format with protocol-aware message decoding
    Decoded,
}

impl OutputFormat {
    /// Parse output format from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "candump" => Some(Self::Candump),
            "json" => Some(Self::Json),
            "decoded" => Some(Self::Decoded),
            _ => None,
        }
    }
}

/// Protocol type for decoded messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Protocol {
    J1939,
}

impl Protocol {
    fn as_str(&self) -> &'static str {
        match self {
            Protocol::J1939 => "J1939",
        }
    }
}

/// Known message metadata wrapper
#[derive(Debug, Clone, Copy)]
struct KnownMessage {
    name: &'static str,
    metadata: &'static MessageMetadata,
    protocol: Protocol,
}

/// Trait for formatting CAN frames to output strings
pub trait Formatter: Send + Sync {
    /// Format a CAN frame with timestamp to a string
    ///
    /// # Arguments
    /// * `frame` - The sourced CAN frame (includes interface and timestamp)
    /// * `timestamp` - The calculated timestamp value (based on timestamp mode)
    ///
    /// # Returns
    /// A formatted string ready for output
    fn format_frame(&self, frame: &SourcedFrame, timestamp: f64) -> String;
}

/// Standard candump format formatter
///
/// Produces output compatible with the candump utility:
/// ```text
/// (timestamp) interface CANID#DATA
/// ```
///
/// Examples:
/// ```text
/// (1234567890.123456) vcan0 123#DEADBEEF
/// (1234567890.234567) can0 18FEF100#0102030405060708
/// ```
#[derive(Debug, Clone)]
pub struct CandumpFormatter {
    /// Timestamp mode for formatting
    timestamp_mode: char,
    /// Color output level (0 = off, 1+ = increasing colors)
    color_level: u8,
    /// Enable ASCII output mode
    ascii_mode: bool,
}

impl CandumpFormatter {
    /// Create a new candump formatter
    pub fn new(timestamp_mode: char, color_level: u8, ascii_mode: bool) -> Self {
        Self {
            timestamp_mode,
            color_level,
            ascii_mode,
        }
    }

    /// Format CAN ID according to candump convention
    fn format_can_id(&self, frame: &CanFrame) -> String {
        use socketcan::Id;

        match frame.id() {
            Id::Standard(id) => {
                format!("{:03X}", id.as_raw())
            }
            Id::Extended(id) => {
                format!("{:08X}", id.as_raw())
            }
        }
    }

    /// Format CAN data bytes as hex string
    fn format_data(&self, frame: &CanFrame) -> String {
        frame
            .data()
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<_>>()
            .join("")
    }

    /// Format timestamp according to mode
    fn format_timestamp(&self, timestamp: f64) -> String {
        match self.timestamp_mode {
            'A' => {
                let secs = timestamp as i64;
                let nsecs = ((timestamp - secs as f64) * 1_000_000_000.0) as u32;

                if let Some(dt) = chrono::DateTime::from_timestamp(secs, nsecs) {
                    let local = dt.with_timezone(&chrono::Local);
                    format!("{}", local.format("%Y-%m-%d %H:%M:%S.%6f"))
                } else {
                    format!("{:.6}", timestamp)
                }
            }
            _ => {
                format!("{:.6}", timestamp)
            }
        }
    }

    /// Format data with ASCII output if enabled
    fn format_data_with_ascii(&self, frame: &CanFrame) -> String {
        let hex = self.format_data(frame);

        if !self.ascii_mode || frame.data().is_empty() {
            return hex;
        }

        let ascii: String = frame
            .data()
            .iter()
            .map(|&byte| {
                if (0x20..=0x7E).contains(&byte) {
                    byte as char
                } else {
                    '.'
                }
            })
            .collect();

        format!("{}  '{}'", hex, ascii)
    }

    /// Apply color to output based on color level
    fn colorize_output(&self, output: String, frame: &SourcedFrame) -> String {
        if self.color_level == 0 {
            return output;
        }

        let raw_id = match frame.frame.id() {
            socketcan::Id::Standard(id) => id.as_raw() as u32,
            socketcan::Id::Extended(id) => id.as_raw(),
        };

        match self.color_level {
            1 => {
                if raw_id % 2 == 0 {
                    output.cyan().to_string()
                } else {
                    output.yellow().to_string()
                }
            }
            2 => {
                match raw_id % 4 {
                    0 => output.cyan().to_string(),
                    1 => output.yellow().to_string(),
                    2 => output.green().to_string(),
                    _ => output.magenta().to_string(),
                }
            }
            _ => {
                match raw_id % 6 {
                    0 => output.red().to_string(),
                    1 => output.green().to_string(),
                    2 => output.yellow().to_string(),
                    3 => output.blue().to_string(),
                    4 => output.magenta().to_string(),
                    _ => output.cyan().to_string(),
                }
            }
        }
    }
}

impl Formatter for CandumpFormatter {
    fn format_frame(&self, frame: &SourcedFrame, timestamp: f64) -> String {
        let timestamp_str = self.format_timestamp(timestamp);
        let interface = &frame.source;
        let can_id = self.format_can_id(&frame.frame);
        let data = if self.ascii_mode {
            self.format_data_with_ascii(&frame.frame)
        } else {
            self.format_data(&frame.frame)
        };

        let output = format!("({}) {} {}#{}", timestamp_str, interface, can_id, data);

        self.colorize_output(output, frame)
    }
}

/// JSON format formatter
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct JsonFormatter {
    /// Timestamp mode for formatting
    timestamp_mode: char,
}

impl JsonFormatter {
    /// Create a new JSON formatter
    pub fn new(timestamp_mode: char) -> Self {
        Self { timestamp_mode }
    }

    /// Extract raw CAN ID value
    fn get_raw_id(frame: &CanFrame) -> u32 {
        match frame.id() {
            Id::Standard(id) => id.as_raw() as u32,
            Id::Extended(id) => id.as_raw(),
        }
    }

    /// Check if frame has extended ID
    fn is_extended(frame: &CanFrame) -> bool {
        matches!(frame.id(), Id::Extended(_))
    }
}

impl Formatter for JsonFormatter {
    fn format_frame(&self, frame: &SourcedFrame, timestamp: f64) -> String {
        let raw_id = Self::get_raw_id(&frame.frame);
        let extended = Self::is_extended(&frame.frame);
        let data_bytes: Vec<u8> = frame.frame.data().to_vec();
        let data_hex = data_bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>();

        let json_obj = json!({
            "timestamp": timestamp,
            "interface": &frame.source,
            "can_id": format!("0x{:X}", raw_id),
            "can_id_decimal": raw_id,
            "extended": extended,
            "dlc": frame.frame.dlc(),
            "data": data_bytes,
            "data_hex": data_hex,
        });

        json_obj.to_string()
    }
}

/// Decoded format formatter
///
/// Produces human-readable output with protocol-aware message decoding.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DecodedFormatter {
    /// Timestamp mode for formatting
    timestamp_mode: char,
    /// Base candump formatter for first line
    candump_formatter: CandumpFormatter,
    /// Known message metadata by base CAN ID
    known_messages: HashMap<u32, KnownMessage>,
}

impl DecodedFormatter {
    /// Create a new decoded formatter
    pub fn new(timestamp_mode: char) -> Self {
        let mut formatter = Self {
            timestamp_mode,
            candump_formatter: CandumpFormatter::new(timestamp_mode, 0, false),
            known_messages: HashMap::new(),
        };

        // Load all protocol metadata
        formatter.load_metadata();

        formatter
    }

    /// Load metadata for all protocols
    fn load_metadata(&mut self) {
        // Load J1939 metadata
        let j1939_metadata = &j1939::J1939_METADATA;
        for &msg_meta in j1939_metadata.messages {
            let known_msg = KnownMessage {
                name: msg_meta.name,
                metadata: msg_meta,
                protocol: Protocol::J1939,
            };

            let can_id = msg_meta.can_id & CAN_EFF_MASK;
            let base_id = get_j1939_base_id(can_id);

            self.known_messages.insert(base_id, known_msg);
            if base_id != can_id {
                self.known_messages.insert(can_id, known_msg);
            }
        }
    }

    /// Attempt to decode frame using cando-messages
    fn try_decode(&self, frame: &CanFrame) -> Option<String> {
        let can_id = match frame.id() {
            Id::Standard(id) => id.as_raw() as u32,
            Id::Extended(id) => id.as_raw(),
        };

        let masked_id = can_id & CAN_EFF_MASK;
        let base_id = get_j1939_base_id(masked_id);
        let device_id = can_id & 0xFF;

        // Try metadata-based decoding first
        if let Some(known_msg) = self
            .known_messages
            .get(&base_id)
            .or_else(|| self.known_messages.get(&masked_id))
        {
            return self.decode_with_metadata(frame, known_msg, device_id);
        }

        // Try J1939-73 diagnostic message decoding
        if let Some(diag_type) = get_diagnostic_message_type(can_id) {
            return self.decode_diagnostic_message(frame, diag_type, device_id);
        }

        None
    }

    /// Decode using message metadata
    fn decode_with_metadata(
        &self,
        frame: &CanFrame,
        known_msg: &KnownMessage,
        device_id: u32,
    ) -> Option<String> {
        let data = frame.data();

        // Convert data to u64 for bit manipulation
        let mut data_u64 = 0u64;
        for (i, &byte) in data.iter().enumerate().take(8) {
            data_u64 |= (byte as u64) << (i * 8);
        }

        let mut output = format!("  Protocol: {}\n", known_msg.protocol.as_str());
        output.push_str(&format!("  Message: {}\n", known_msg.name));
        output.push_str(&format!("  Device ID: 0x{:02X}\n", device_id));
        output.push_str("  Signals:\n");

        // Decode each signal
        for signal in known_msg.metadata.signals {
            if signal.signal_size == 0 {
                continue;
            }

            let mask = if signal.signal_size >= 64 {
                u64::MAX
            } else {
                (1u64 << signal.signal_size) - 1
            };
            let raw_value = (data_u64 >> signal.start_bit) & mask;

            let scaled_value = (raw_value as f64) * signal.factor + signal.offset;

            if signal.unit.is_empty() {
                output.push_str(&format!("    {}: {:.3}\n", signal.name, scaled_value));
            } else {
                output.push_str(&format!(
                    "    {}: {:.3} {}\n",
                    signal.name, scaled_value, signal.unit
                ));
            }
        }

        Some(output)
    }

    /// Decode J1939-73 diagnostic message
    fn decode_diagnostic_message(
        &self,
        frame: &CanFrame,
        diag_type: DiagnosticMessageType,
        device_id: u32,
    ) -> Option<String> {
        let data = frame.data();
        let can_id = match frame.id() {
            Id::Standard(id) => id.as_raw() as u32,
            Id::Extended(id) => id.as_raw(),
        };

        let mut output = String::from("  Protocol: J1939-73 Diagnostics\n");
        output.push_str(&format!("  Device ID: 0x{:02X}\n", device_id));

        match diag_type {
            DiagnosticMessageType::DM01 => {
                output.push_str("  Message: DM01 (Active DTCs)\n");
                if let Ok(msg) = DM01::decode(can_id, data) {
                    self.format_dtc_message(&mut output, &msg);
                }
            }
            DiagnosticMessageType::DM02 => {
                output.push_str("  Message: DM02 (Previously Active DTCs)\n");
                if let Ok(msg) = DM02::decode(can_id, data) {
                    self.format_dtc_message(&mut output, &msg);
                }
            }
            DiagnosticMessageType::DM06 => {
                output.push_str("  Message: DM06 (Pending DTCs)\n");
                if let Ok(msg) = DM06::decode(can_id, data) {
                    self.format_dtc_message(&mut output, &msg);
                }
            }
            DiagnosticMessageType::DM12 => {
                output.push_str("  Message: DM12 (Emissions-Related Active DTCs)\n");
                if let Ok(msg) = DM12::decode(can_id, data) {
                    self.format_dtc_message(&mut output, &msg);
                }
            }
            DiagnosticMessageType::DM23 => {
                output.push_str("  Message: DM23 (Previously Active Emissions DTCs)\n");
                if let Ok(msg) = DM23::decode(can_id, data) {
                    self.format_dtc_message(&mut output, &msg);
                }
            }
            DiagnosticMessageType::DM13 => {
                output.push_str("  Message: DM13 (Stop Start Broadcast)\n");
                if let Ok(_msg) = DM13::decode(can_id, data) {
                    output.push_str("  (Detailed decoding not implemented)\n");
                }
            }
            DiagnosticMessageType::DM22 => {
                output.push_str("  Message: DM22 (Individual Clear/Reset)\n");
                if let Ok(_msg) = DM22::decode(can_id, data) {
                    output.push_str("  (Detailed decoding not implemented)\n");
                }
            }
            _ => {
                output.push_str(&format!("  Message: {:?}\n", diag_type));
                output.push_str("  (Detailed decoding not implemented)\n");
            }
        }

        Some(output)
    }

    /// Format DTC message with lamp status and DTCs
    fn format_dtc_message<T: DtcMessage>(&self, output: &mut String, msg: &T) {
        let lamp_status = msg.lamp_status();
        output.push_str("  Lamps:\n");
        output.push_str(&format!(
            "    MIL: {}\n",
            Self::format_lamp_state(lamp_status.mil)
        ));
        output.push_str(&format!(
            "    Red Stop: {}\n",
            Self::format_lamp_state(lamp_status.red_stop)
        ));
        output.push_str(&format!(
            "    Amber Warning: {}\n",
            Self::format_lamp_state(lamp_status.amber_warning)
        ));
        output.push_str(&format!(
            "    Protect: {}\n",
            Self::format_lamp_state(lamp_status.protect)
        ));

        let dtcs = msg.get_active_dtcs();
        if dtcs.is_empty() {
            output.push_str("  DTCs: None\n");
        } else {
            output.push_str(&format!("  DTCs ({}):\n", dtcs.len()));
            for dtc in dtcs {
                output.push_str(&format!(
                    "    SPN: {}, FMI: {}, OC: {}\n",
                    dtc.spn, dtc.fmi, dtc.occurrence_count
                ));
            }
        }
    }

    /// Format lamp state as human-readable string
    fn format_lamp_state(state: LampState) -> &'static str {
        match state {
            LampState::Off => "Off",
            LampState::On => "On",
            LampState::Reserved => "Reserved",
            LampState::NotAvailable => "N/A",
        }
    }
}

impl Formatter for DecodedFormatter {
    fn format_frame(&self, frame: &SourcedFrame, timestamp: f64) -> String {
        // Start with candump format line
        let mut output = self.candump_formatter.format_frame(frame, timestamp);

        // Try to decode and add decoded information
        if let Some(decoded) = self.try_decode(&frame.frame) {
            output.push('\n');
            output.push_str(&decoded);
        }

        output
    }
}

/// Create a formatter based on output format
pub fn create_formatter(
    format: OutputFormat,
    timestamp_mode: char,
    color_level: u8,
    ascii_mode: bool,
) -> Box<dyn Formatter> {
    match format {
        OutputFormat::Candump => Box::new(CandumpFormatter::new(
            timestamp_mode,
            color_level,
            ascii_mode,
        )),
        OutputFormat::Json => Box::new(JsonFormatter::new(timestamp_mode)),
        OutputFormat::Decoded => Box::new(DecodedFormatter::new(timestamp_mode)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use socketcan::{CanFrame, EmbeddedFrame, ExtendedId, StandardId};

    fn create_test_frame(id: u32, extended: bool, data: &[u8]) -> SourcedFrame {
        use socketcan::Id;

        let frame = if extended {
            let ext_id = ExtendedId::new(id).unwrap();
            CanFrame::new(Id::Extended(ext_id), data).unwrap()
        } else {
            let std_id = StandardId::new(id as u16).unwrap();
            CanFrame::new(Id::Standard(std_id), data).unwrap()
        };

        SourcedFrame {
            source: "vcan0".to_string(),
            frame,
            timestamp: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_format_standard_id() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x123, false, &[0xDE, 0xAD, 0xBE, 0xEF]);
        let output = formatter.format_frame(&frame, 1234567890.123456);

        assert!(output.contains("vcan0"));
        assert!(output.contains("123#DEADBEEF"));
        assert!(output.contains("1234567890.123456"));
    }

    #[test]
    fn test_format_extended_id() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x18FEF100, true, &[0x01, 0x02, 0x03, 0x04]);
        let output = formatter.format_frame(&frame, 1234567890.123456);

        assert!(output.contains("vcan0"));
        assert!(output.contains("18FEF100#01020304"));
        assert!(output.contains("1234567890.123456"));
    }

    #[test]
    fn test_format_empty_data() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x100, false, &[]);
        let output = formatter.format_frame(&frame, 0.0);

        assert!(output.contains("100#"));
        assert!(output.ends_with("100#"));
    }

    #[test]
    fn test_format_full_data() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(
            0x7FF,
            false,
            &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77],
        );
        let output = formatter.format_frame(&frame, 1.0);

        assert!(output.contains("7FF#0011223344556677"));
    }

    #[test]
    fn test_timestamp_formatting_absolute() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let timestamp = formatter.format_timestamp(1234567890.123456);
        assert_eq!(timestamp, "1234567890.123456");
    }

    #[test]
    fn test_timestamp_formatting_delta() {
        let formatter = CandumpFormatter::new('d', 0, false);
        let timestamp = formatter.format_timestamp(0.001234);
        assert_eq!(timestamp, "0.001234");
    }

    #[test]
    fn test_timestamp_formatting_zero() {
        let formatter = CandumpFormatter::new('z', 0, false);
        let timestamp = formatter.format_timestamp(123.456789);
        assert_eq!(timestamp, "123.456789");
    }

    #[test]
    fn test_can_id_formatting_standard() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x001, false, &[]);
        let can_id = formatter.format_can_id(&frame.frame);
        assert_eq!(can_id, "001");
    }

    #[test]
    fn test_can_id_formatting_standard_max() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x7FF, false, &[]);
        let can_id = formatter.format_can_id(&frame.frame);
        assert_eq!(can_id, "7FF");
    }

    #[test]
    fn test_can_id_formatting_extended() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x00000001, true, &[]);
        let can_id = formatter.format_can_id(&frame.frame);
        assert_eq!(can_id, "00000001");
    }

    #[test]
    fn test_can_id_formatting_extended_max() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x1FFFFFFF, true, &[]);
        let can_id = formatter.format_can_id(&frame.frame);
        assert_eq!(can_id, "1FFFFFFF");
    }

    #[test]
    fn test_data_formatting() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x123, false, &[0xAB, 0xCD, 0xEF, 0x01]);
        let data = formatter.format_data(&frame.frame);
        assert_eq!(data, "ABCDEF01");
    }

    #[test]
    fn test_full_output_format() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x123, false, &[0xDE, 0xAD, 0xBE, 0xEF]);
        let output = formatter.format_frame(&frame, 1234567890.123456);

        assert!(output.starts_with("(1234567890.123456)"));
        assert!(output.contains(" vcan0 "));
        assert!(output.ends_with("123#DEADBEEF"));
    }

    #[test]
    fn test_ascii_mode() {
        let formatter = CandumpFormatter::new('a', 0, true);
        let frame = create_test_frame(0x123, false, &[0x48, 0x65, 0x6C, 0x6C, 0x6F]); // "Hello"
        let output = formatter.format_frame(&frame, 0.0);

        assert!(output.contains("48656C6C6F"));
        assert!(output.contains("'Hello'"));
    }

    #[test]
    fn test_ascii_mode_non_printable() {
        let formatter = CandumpFormatter::new('a', 0, true);
        let frame = create_test_frame(0x123, false, &[0x00, 0x41, 0xFF, 0x42]);
        let output = formatter.format_frame(&frame, 0.0);

        assert!(output.contains("0041FF42"));
        assert!(output.contains("'.A.B'"));
    }

    #[test]
    fn test_color_disabled() {
        let formatter = CandumpFormatter::new('a', 0, false);
        let frame = create_test_frame(0x123, false, &[0xDE, 0xAD]);
        let output = formatter.format_frame(&frame, 0.0);

        assert!(!output.contains("\x1b["));
    }

    #[test]
    fn test_color_enabled() {
        let formatter = CandumpFormatter::new('a', 1, false);
        let frame = create_test_frame(0x123, false, &[0xDE, 0xAD]);
        let output = formatter.format_frame(&frame, 0.0);

        assert!(!output.is_empty());
    }

    #[test]
    fn test_decoded_formatter_j1939_dm1() {
        let formatter = DecodedFormatter::new('a');
        // Real DM1 message: PGN 0x0FECA, device 0x59
        let frame = create_test_frame(
            0x1CFECA59,
            true,
            &[0x04, 0xFD, 0x00, 0xFF, 0xE1, 0x13, 0xFF, 0xFF],
        );
        let output = formatter.format_frame(&frame, 1234567890.123456);

        // Should contain candump format line
        assert!(output.contains("1CFECA59#04FD00FF"));

        // Should decode as J1939 (either via metadata or diagnostic path)
        assert!(output.contains("Protocol:") && output.contains("J1939"));
        assert!(output.contains("Device ID: 0x59") || output.contains("0x59"));

        // Should contain decoded signal information (metadata or diagnostic path)
        // The message gets decoded either with signal names from metadata
        // or with lamp/DTC info from the diagnostic decoder
        let has_decoded_content = output.contains("Signals:")
            || output.contains("Lamps:")
            || output.contains("MIL:")
            || output.contains("lampStatus")
            || output.contains("DM01")
            || output.contains("Active");
        assert!(has_decoded_content, "Expected decoded content in output: {}", output);
    }

    #[test]
    fn test_decoded_formatter_unknown_message() {
        let formatter = DecodedFormatter::new('a');
        let frame = create_test_frame(0x12345678, true, &[0x01, 0x02, 0x03, 0x04]);
        let output = formatter.format_frame(&frame, 1234567890.123456);

        assert!(output.contains("12345678#01020304"));

        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(
            lines.len(),
            1,
            "Unknown message should only have candump line"
        );
    }

    #[test]
    fn test_decoded_formatter_metadata_loaded() {
        let formatter = DecodedFormatter::new('a');

        assert!(!formatter.known_messages.is_empty());

        let message_count = formatter.known_messages.len();
        assert!(
            message_count > 5,
            "Should have loaded J1939 protocol messages"
        );
    }

    #[test]
    fn test_output_format_from_str() {
        assert_eq!(
            OutputFormat::from_str("candump"),
            Some(OutputFormat::Candump)
        );
        assert_eq!(OutputFormat::from_str("json"), Some(OutputFormat::Json));
        assert_eq!(
            OutputFormat::from_str("decoded"),
            Some(OutputFormat::Decoded)
        );
        assert_eq!(
            OutputFormat::from_str("CANDUMP"),
            Some(OutputFormat::Candump)
        );
        assert_eq!(OutputFormat::from_str("invalid"), None);
    }
}
