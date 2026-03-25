//! CAN Log Analyzer - Comprehensive candump log parser with verbose human-readable decoding
//!
//! This utility parses candump log files and provides detailed human-readable analysis
//! of CAN messages using DBC file definitions. It can decode J1939 messages,
//! extract signal values, show value descriptions, and provide statistical analysis.

//!
//! Features:
//! - Parse standard candump log format
//! - Decode messages using j1939.dbc files
//! - Extract and display all signal values with engineering units
//! - Show enumerated value descriptions from VAL_ entries
//! - Provide message frequency and timing analysis
//! - Support for multiple device IDs and message filtering
//! - Export decoded data in various formats (JSON, CSV, human-readable)
//! - Statistical analysis of signal ranges and message patterns

use anyhow::Result;
#[allow(unused_imports)]
use clap::CommandFactory;
use clap::{Parser, ValueEnum};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use cando_messages::common::{CAN_EFF_MASK, get_j1939_base_id};
use cando_messages::metadata::{HasMetadata, MessageMetadata};
use cando_messages::j1939::{
    self, DM01, DM02, DM03, DM04, DM05, DM06, DM07, DM08, DM11, DM12, DM13, DM20, DM21, DM22,
    DM23, DM24, DM29, DM30, DM31,
    diagnostics::{
        DM13Helpers, DM22Helpers, DiagnosticMessageType, DtcMessage, LampState,
        get_diagnostic_message_type,
    },
};

// CAN ID bit manipulation constants imported from common module

/// Output format options
#[derive(ValueEnum, Clone, Debug, Default)]
enum OutputFormat {
    /// Human-readable text format (default)
    #[default]
    Text,
    /// Comma-separated values format
    Csv,
    /// JSON format
    Json,
}

/// Command-line arguments for the CAN log analyzer
#[derive(Parser, Debug)]
#[command(
    version,
    about = "CAN Log Analyzer - Parse candump logs with comprehensive DBC decoding (J1939)"
)]
struct Args {
    /// Path to candump log file
    #[arg(help = "Path to candump log file")]
    log_file: String,

    /// Show only decoded messages (hide unknown)
    #[arg(long, short = 'd')]
    decoded_only: bool,

    /// Filter by specific device IDs (comma-separated hex values)
    #[arg(long, short = 'f')]
    device_filter: Option<String>,

    /// Filter by message name patterns (comma-separated)
    #[arg(long, short = 'm')]
    message_filter: Option<String>,

    /// Show detailed signal analysis
    #[arg(long, short = 'v')]
    verbose: bool,

    /// Show statistics summary
    #[arg(long, short = 's')]
    statistics: bool,

    /// Export decoded data to JSON file
    #[arg(long)]
    export_json: Option<String>,

    /// Export decoded data to CSV file
    #[arg(long)]
    export_csv: Option<String>,

    /// Output format
    #[arg(long, short = 'o', value_enum, default_value = "text")]
    output: OutputFormat,

    /// CSV output format (short flag)
    #[arg(long = "csv", help = "Output in CSV format (same as --output=csv)")]
    csv: bool,

    /// JSON output format (short flag)
    #[arg(long = "json", help = "Output in JSON format (same as --output=json)")]
    json: bool,

    /// Text output format (short flag)
    #[arg(long = "text", help = "Output in text format (same as --output=text)")]
    text: bool,

    /// Show message timing analysis
    #[arg(long, short = 't')]
    timing_analysis: bool,

    /// Limit number of messages to process (for large files)
    #[arg(long, short = 'l')]
    limit: Option<usize>,

    /// Generate man page and exit (internal use)
    #[arg(long = "generate-manpage", hide = true)]
    generate_manpage: bool,
}

/// Parsed CAN log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CanLogEntry {
    interface: String,
    can_id: u32,
    dlc: u8,
    data: Vec<u8>,
    timestamp: Option<f64>, // Optional timestamp if present in log
    line_number: usize,
}

/// Decoded message with signal values
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DecodedMessage {
    log_entry: CanLogEntry,
    message_name: String,
    device_id: u32,
    dbc_type: String,
    signal_values: HashMap<String, SignalValue>,
    timestamp_formatted: String,
}

/// Signal value with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignalValue {
    raw_value: f64,
    engineering_value: f64,
    unit: String,
    description: Option<String>, // From VAL_ entries
    min_value: f64,
    max_value: f64,
    factor: f64,
    offset: f64,
}

/// Message statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageStats {
    message_name: String,
    count: usize,
    first_seen: f64,
    last_seen: f64,
    frequency_hz: f64,
    device_ids: Vec<u32>,
    signal_stats: HashMap<String, SignalStats>,
}

/// Signal statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignalStats {
    min_value: f64,
    max_value: f64,
    avg_value: f64,
    sample_count: usize,
    unit: String,
    value_descriptions: Vec<(i64, String)>,
}

/// J1939 analysis result
struct J1939Analysis {
    pgn: u32,
    #[allow(dead_code)]
    priority: u8,
    #[allow(dead_code)]
    source_address: u8,
    description: String,
}

/// Main CAN log analyzer
/// Represents a known CAN message definition
#[derive(Debug, Clone, Copy)]
struct KnownMessage {
    name: &'static str,
    metadata: &'static MessageMetadata,
    protocol: Protocol,
}

/// Protocol type for message classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
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

pub struct CanLogAnalyzer {
    args: Args,
    known_messages: HashMap<u32, KnownMessage>,
    device_filters: Option<Vec<u32>>,
    message_filters: Option<Vec<String>>,
    decoded_messages: Vec<DecodedMessage>,
    message_stats: HashMap<String, MessageStats>,
    log_entries: Vec<CanLogEntry>,
}

impl CanLogAnalyzer {
    /// Create new analyzer instance
    fn new(mut args: Args) -> Result<Self> {
        // Handle short flag overrides for output format
        if args.csv {
            args.output = OutputFormat::Csv;
        } else if args.json {
            args.output = OutputFormat::Json;
        } else if args.text {
            args.output = OutputFormat::Text;
        }

        let mut analyzer = Self {
            device_filters: Self::parse_device_filter(&args.device_filter)?,
            message_filters: Self::parse_message_filter(&args.message_filter),
            args,
            known_messages: HashMap::new(),
            decoded_messages: Vec::new(),
            message_stats: HashMap::new(),
            log_entries: Vec::new(),
        };

        // Load metadata
        analyzer.load_metadata()?;

        Ok(analyzer)
    }

    /// Parse device filter from command line
    fn parse_device_filter(filter_str: &Option<String>) -> Result<Option<Vec<u32>>> {
        match filter_str {
            Some(s) => {
                let mut devices = Vec::new();
                for part in s.split(',') {
                    let part = part.trim();
                    let device_id = if part.starts_with("0x") || part.starts_with("0X") {
                        u32::from_str_radix(&part[2..], 16)?
                    } else {
                        part.parse::<u32>()?
                    };
                    devices.push(device_id);
                }
                Ok(Some(devices))
            }
            None => Ok(None),
        }
    }

    /// Parse message filter from command line
    fn parse_message_filter(filter_str: &Option<String>) -> Option<Vec<String>> {
        filter_str
            .as_ref()
            .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
    }

    /// Load metadata for J1939 messages
    fn load_metadata(&mut self) -> Result<()> {
        println!("Loading message metadata...");

        // Load J1939 metadata
        let j1939_metadata = &j1939::J1939_METADATA;
        for &msg_meta in j1939_metadata.messages {
            let known_msg = KnownMessage {
                name: msg_meta.name,
                metadata: msg_meta,
                protocol: Protocol::J1939,
            };

            // Store by exact ID and base ID for flexible matching
            // Use J1939-aware base ID calculation for correct PDU1/PDU2 handling
            let can_id = msg_meta.can_id & CAN_EFF_MASK;
            let base_id = get_j1939_base_id(can_id);

            self.known_messages.insert(base_id, known_msg);
            if base_id != can_id {
                self.known_messages.insert(can_id, known_msg);
            }
        }

        println!(
            "   Loaded {} J1939 message definitions",
            j1939_metadata.messages.len(),
        );
        Ok(())
    }

    /// Parse candump log file
    pub fn parse_log_file(&mut self) -> Result<()> {
        println!("Parsing CAN log file: {}", self.args.log_file);

        let file = File::open(&self.args.log_file)?;
        let reader = BufReader::new(file);

        // Regex for parsing candump formats:
        // Format 1 (expanded): interface CAN_ID [DLC] data_bytes
        let line_regex = Regex::new(r"^\s*(\w+)\s+([0-9A-Fa-f]+)\s+\[(\d+)\]\s+(.+)$")?;
        // Format 2 (compact): (timestamp) interface CAN_ID#data
        let compact_regex = Regex::new(r"^\s*\([\d.]+\)\s+(\w+)\s+([0-9A-Fa-f]+)#([0-9A-Fa-f]+)$")?;

        let mut processed_count = 0;
        let mut line_number = 0;

        for line in reader.lines() {
            line_number += 1;
            let line = line?;

            // Skip empty lines and comments
            if line.trim().is_empty() || line.trim().starts_with('#') {
                continue;
            }

            // Try expanded format first
            if let Some(caps) = line_regex.captures(&line) {
                let interface = caps[1].to_string();
                let can_id = u32::from_str_radix(&caps[2], 16)?;
                let dlc = caps[3].parse::<u8>()?;
                let data_str = &caps[4];

                // Parse data bytes
                let mut data = Vec::new();
                for byte_str in data_str.split_whitespace() {
                    if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
                        data.push(byte);
                    }
                }

                // Validate DLC matches data length
                if data.len() != dlc as usize {
                    eprintln!(
                        "Warning: Line {}: DLC {} doesn't match data length {}",
                        line_number,
                        dlc,
                        data.len()
                    );
                    continue;
                }

                let log_entry = CanLogEntry {
                    interface,
                    can_id,
                    dlc,
                    data,
                    timestamp: None,
                    line_number,
                };

                self.log_entries.push(log_entry);
                processed_count += 1;

                // Check limit
                if let Some(limit) = self.args.limit
                    && processed_count >= limit
                {
                    break;
                }
            } else if let Some(caps) = compact_regex.captures(&line) {
                // Try compact format: (timestamp) interface CAN_ID#data
                let interface = caps[1].to_string();
                let can_id = u32::from_str_radix(&caps[2], 16)?;
                let data_str = &caps[3];

                // Parse data bytes from compact hex string (no spaces)
                let mut data = Vec::new();
                for i in (0..data_str.len()).step_by(2) {
                    if i + 1 < data_str.len()
                        && let Ok(byte) = u8::from_str_radix(&data_str[i..i + 2], 16)
                    {
                        data.push(byte);
                    }
                }

                let dlc = data.len() as u8;

                let log_entry = CanLogEntry {
                    interface,
                    can_id,
                    dlc,
                    data,
                    timestamp: None, // Could parse timestamp if needed
                    line_number,
                };

                self.log_entries.push(log_entry);
                processed_count += 1;

                // Check limit
                if let Some(limit) = self.args.limit
                    && processed_count >= limit
                {
                    break;
                }
            } else {
                eprintln!(
                    "Warning: Line {}: Couldn't parse candump format: {}",
                    line_number, line
                );
            }
        }

        println!(
            "   Parsed {} CAN messages from log file",
            processed_count
        );
        Ok(())
    }

    /// Decode messages using metadata and analyze unknown messages
    pub fn decode_messages(&mut self) -> Result<()> {
        println!("Decoding messages using metadata and analyzing unknown messages...");

        let mut decoded_count = 0;
        let mut unknown_count = 0;

        for log_entry in self.log_entries.iter() {
            // Extract device ID from CAN ID (lower 8 bits)
            let device_id = log_entry.can_id & 0xFF;

            // Apply device filter
            if let Some(ref filters) = self.device_filters
                && !filters.contains(&device_id)
            {
                continue;
            }

            // Try to find matching message by CAN ID
            let known_msg_opt = self.find_matching_message(log_entry.can_id);

            let mut decoded = false;
            if let Some(known_msg) = known_msg_opt {
                // Apply message filter
                if let Some(ref filters) = self.message_filters
                    && !filters.iter().any(|f| known_msg.name.contains(f))
                {
                    continue;
                }

                match self.decode_signals(
                    log_entry,
                    known_msg.metadata,
                    known_msg.name,
                    device_id,
                    known_msg.protocol.as_str(),
                ) {
                    Ok(decoded_msg) => {
                        self.decoded_messages.push(decoded_msg);
                        decoded_count += 1;
                        decoded = true;
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to decode message at line {}: {}",
                            log_entry.line_number, e
                        );
                    }
                }
            }

            // Try J1939-73 diagnostic message decoding if metadata didn't match
            if !decoded && let Some(diagnostic_type) = get_diagnostic_message_type(log_entry.can_id)
            {
                match self.decode_diagnostic_message(log_entry, device_id, diagnostic_type) {
                    Ok(decoded_msg) => {
                        self.decoded_messages.push(decoded_msg);
                        decoded_count += 1;
                        decoded = true;
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to decode diagnostic message at line {}: {}",
                            log_entry.line_number, e
                        );
                    }
                }
            }

            // If not decoded by metadata or diagnostics, try flexible matching based on message patterns
            if !decoded
                && let Some(flexible_match) = self.try_flexible_dbc_matching(log_entry, device_id)
            {
                self.decoded_messages.push(flexible_match);
                decoded_count += 1;
                decoded = true;
            }

            // If still not decoded, create a raw analysis entry
            if !decoded {
                let raw_decoded = self.create_raw_analysis(log_entry, device_id);
                self.decoded_messages.push(raw_decoded);
                unknown_count += 1;
            }
        }

        println!(
            "   Decoded {} messages via metadata/diagnostics, {} analyzed as raw/unknown",
            decoded_count, unknown_count
        );
        Ok(())
    }

    /// Find a matching message definition for the given CAN ID
    fn find_matching_message(&self, can_id: u32) -> Option<KnownMessage> {
        // Ensure we're working with 29-bit ID
        let lookup_id = can_id & CAN_EFF_MASK;

        // First try exact match
        if let Some(msg) = self.known_messages.get(&lookup_id) {
            return Some(*msg);
        }

        // Try base ID match using J1939-aware lookup
        // This correctly handles both PDU1 (strips dest+src) and PDU2 (strips src only)
        let base_id = get_j1939_base_id(lookup_id);
        self.known_messages.get(&base_id).copied()
    }

    /// Decode signals from CAN data using metadata
    fn decode_signals(
        &self,
        log_entry: &CanLogEntry,
        metadata: &MessageMetadata,
        message_name: &str,
        device_id: u32,
        dbc_type: &str,
    ) -> Result<DecodedMessage> {
        let mut signal_values = HashMap::new();

        // Convert data to u64 for bit manipulation
        let mut data_u64 = 0u64;
        for (i, &byte) in log_entry.data.iter().enumerate().take(8) {
            data_u64 |= (byte as u64) << (i * 8);
        }

        // Decode each signal using metadata
        for signal in metadata.signals {
            let start_bit = signal.start_bit as usize;
            let length = signal.signal_size as usize;

            // Skip signals with size 0 (invalid/placeholder signals)
            if length == 0 {
                continue;
            }

            // Extract raw value
            let mask = (1u64 << length) - 1;
            let raw_value = (data_u64 >> start_bit) & mask;

            // Apply scaling: engineering_value = (raw_value * factor) + offset
            let engineering_value = (raw_value as f64 * signal.factor) + signal.offset;

            // Look up value description if available
            let description = signal
                .value_descriptions
                .iter()
                .find(|(value, _)| *value == raw_value)
                .map(|(_, desc)| desc.to_string());

            let signal_value = SignalValue {
                raw_value: raw_value as f64,
                engineering_value,
                unit: signal.unit.to_string(),
                description,
                min_value: signal.min,
                max_value: signal.max,
                factor: signal.factor,
                offset: signal.offset,
            };

            signal_values.insert(signal.name.to_string(), signal_value);
        }

        let decoded_msg = DecodedMessage {
            log_entry: log_entry.clone(),
            message_name: message_name.to_string(),
            device_id,
            dbc_type: dbc_type.to_string(),
            signal_values,
            timestamp_formatted: format!("Line {}", log_entry.line_number),
        };

        Ok(decoded_msg)
    }

    /// Decode J1939-73 diagnostic messages using the diagnostic framework
    fn decode_diagnostic_message(
        &self,
        log_entry: &CanLogEntry,
        device_id: u32,
        diagnostic_type: DiagnosticMessageType,
    ) -> Result<DecodedMessage> {
        let mut signal_values = HashMap::new();

        let message_name = match diagnostic_type {
            // DTC-style messages (DM01, DM02, DM06, DM12, DM23) - use DtcMessage trait
            DiagnosticMessageType::DM01 => {
                let msg = DM01::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM01 decode error: {:?}", e))?;
                self.add_dtc_signal_values(&mut signal_values, &msg, "Active");
                "DM01_Active_Diagnostic_Trouble_Codes".to_string()
            }
            DiagnosticMessageType::DM02 => {
                let msg = DM02::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM02 decode error: {:?}", e))?;
                self.add_dtc_signal_values(&mut signal_values, &msg, "Previously_Active");
                "DM02_Previously_Active_DTCs".to_string()
            }
            DiagnosticMessageType::DM06 => {
                let msg = DM06::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM06 decode error: {:?}", e))?;
                self.add_dtc_signal_values(&mut signal_values, &msg, "Pending");
                "DM06_Emission_Pending_DTCs".to_string()
            }
            DiagnosticMessageType::DM12 => {
                let msg = DM12::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM12 decode error: {:?}", e))?;
                self.add_dtc_signal_values(&mut signal_values, &msg, "MIL_On");
                "DM12_Emission_MIL_On_DTCs".to_string()
            }
            DiagnosticMessageType::DM23 => {
                let msg = DM23::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM23 decode error: {:?}", e))?;
                self.add_dtc_signal_values(&mut signal_values, &msg, "Previously_MIL_On");
                "DM23_Emission_Previously_MIL_On_DTCs".to_string()
            }
            DiagnosticMessageType::DM03 => {
                let dm03 = DM03::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM03 decode error: {:?}", e))?;

                // DM03 is a command message with no payload data
                signal_values.insert(
                    "Command_Type".to_string(),
                    SignalValue {
                        raw_value: 1.0,
                        engineering_value: 1.0,
                        unit: "command".to_string(),
                        description: Some("Clear/Reset Diagnostic Data Command".to_string()),
                        min_value: 0.0,
                        max_value: 1.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                signal_values.insert(
                    "Target_Device".to_string(),
                    SignalValue {
                        raw_value: dm03.device_id.as_u8() as f64,
                        engineering_value: dm03.device_id.as_u8() as f64,
                        unit: "device_id".to_string(),
                        description: Some(format!(
                            "Target device for DTC clear command: 0x{:02X}",
                            dm03.device_id.as_u8()
                        )),
                        min_value: 0.0,
                        max_value: 255.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                "DM03_Clear_Reset_DTCs".to_string()
            }
            DiagnosticMessageType::DM11 => {
                let dm11 = DM11::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM11 decode error: {:?}", e))?;

                // DM11 is a zero-length command message to clear active DTCs
                signal_values.insert(
                    "Command_Type".to_string(),
                    SignalValue {
                        raw_value: 1.0,
                        engineering_value: 1.0,
                        unit: "command".to_string(),
                        description: Some("Clear Active DTCs Command".to_string()),
                        min_value: 0.0,
                        max_value: 1.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                signal_values.insert(
                    "Target_Device".to_string(),
                    SignalValue {
                        raw_value: dm11.device_id.as_u8() as f64,
                        engineering_value: dm11.device_id.as_u8() as f64,
                        unit: "device_id".to_string(),
                        description: Some(format!(
                            "Target device for clear active DTCs: 0x{:02X}",
                            dm11.device_id.as_u8()
                        )),
                        min_value: 0.0,
                        max_value: 255.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                "DM11_Clear_Active_DTCs".to_string()
            }
            DiagnosticMessageType::DM13 => {
                let dm13 = DM13::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM13 decode error: {:?}", e))?;

                // DM13 is a stop/start broadcast command for network control
                let is_stop = dm13.is_stop_command();
                let is_resume = dm13.is_resume_command();

                signal_values.insert(
                    "Command_Type".to_string(),
                    SignalValue {
                        raw_value: if is_stop {
                            1.0
                        } else if is_resume {
                            0.0
                        } else {
                            2.0
                        },
                        engineering_value: if is_stop {
                            1.0
                        } else if is_resume {
                            0.0
                        } else {
                            2.0
                        },
                        unit: "command".to_string(),
                        description: Some(if is_stop {
                            "Stop Broadcast".to_string()
                        } else if is_resume {
                            "Resume Operation".to_string()
                        } else {
                            "Network Control".to_string()
                        }),
                        min_value: 0.0,
                        max_value: 2.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                signal_values.insert(
                    "Suspend_Duration".to_string(),
                    SignalValue {
                        raw_value: dm13.get_suspend_duration_seconds() as f64,
                        engineering_value: dm13.get_suspend_duration_seconds() as f64,
                        unit: "seconds".to_string(),
                        description: Some(format!(
                            "Suspend duration: {} seconds",
                            dm13.get_suspend_duration_seconds()
                        )),
                        min_value: 0.0,
                        max_value: 64255.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                signal_values.insert(
                    "Affects_J1939_Network1".to_string(),
                    SignalValue {
                        raw_value: if dm13.affects_j1939_network1() {
                            1.0
                        } else {
                            0.0
                        },
                        engineering_value: if dm13.affects_j1939_network1() {
                            1.0
                        } else {
                            0.0
                        },
                        unit: "bool".to_string(),
                        description: Some(format!(
                            "J1939 Network #1: {}",
                            if dm13.affects_j1939_network1() {
                                "affected"
                            } else {
                                "not affected"
                            }
                        )),
                        min_value: 0.0,
                        max_value: 1.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                "DM13_Stop_Start_Broadcast".to_string()
            }
            DiagnosticMessageType::DM22 => {
                let dm22 = DM22::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM22 decode error: {:?}", e))?;

                // DM22 is an individual clear command for specific DTCs
                signal_values.insert(
                    "Command_Type".to_string(),
                    SignalValue {
                        raw_value: 2.0,
                        engineering_value: 2.0,
                        unit: "command".to_string(),
                        description: Some("Individual Clear/Reset DTC Command".to_string()),
                        min_value: 0.0,
                        max_value: 2.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                signal_values.insert(
                    "Target_Device".to_string(),
                    SignalValue {
                        raw_value: dm22.device_id.as_u8() as f64,
                        engineering_value: dm22.device_id.as_u8() as f64,
                        unit: "device_id".to_string(),
                        description: Some(format!(
                            "Target device: 0x{:02X}",
                            dm22.device_id.as_u8()
                        )),
                        min_value: 0.0,
                        max_value: 255.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                signal_values.insert(
                    "Target_SPN".to_string(),
                    SignalValue {
                        raw_value: dm22.get_target_spn() as f64,
                        engineering_value: dm22.get_target_spn() as f64,
                        unit: "SPN".to_string(),
                        description: Some(format!(
                            "Target Suspect Parameter Number: {}",
                            dm22.get_target_spn()
                        )),
                        min_value: 0.0,
                        max_value: 524287.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                signal_values.insert(
                    "Target_FMI".to_string(),
                    SignalValue {
                        raw_value: dm22.get_target_fmi() as f64,
                        engineering_value: dm22.get_target_fmi() as f64,
                        unit: "FMI".to_string(),
                        description: Some(format!(
                            "Target Failure Mode Identifier: {}",
                            dm22.get_target_fmi()
                        )),
                        min_value: 0.0,
                        max_value: 31.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                "DM22_Individual_Clear_Reset_DTC".to_string()
            }
            // Phase 4: Simple diagnostic messages (Pattern 1)
            DiagnosticMessageType::DM04 => {
                DM04::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM04 decode error: {:?}", e))?;

                signal_values =
                    self.extract_signals_from_metadata(DM04::metadata(), &log_entry.data);

                "DM04_Freeze_Frame_Parameters".to_string()
            }
            DiagnosticMessageType::DM05 => {
                DM05::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM05 decode error: {:?}", e))?;

                signal_values =
                    self.extract_signals_from_metadata(DM05::metadata(), &log_entry.data);

                "DM05_Diagnostic_Readiness_1".to_string()
            }
            DiagnosticMessageType::DM07 => {
                DM07::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM07 decode error: {:?}", e))?;

                signal_values =
                    self.extract_signals_from_metadata(DM07::metadata(), &log_entry.data);

                "DM07_Command_Non_Continuously_Monitored_Test".to_string()
            }

            DiagnosticMessageType::DM08 => {
                let dm08 = DM08::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM08 decode error: {:?}", e))?;

                signal_values =
                    self.extract_signals_from_metadata(DM08::metadata(), &log_entry.data);

                // Add pass/fail status (custom logic)
                let passed = dm08.test_value >= dm08.test_limit_minimum
                    && dm08.test_value <= dm08.test_limit_maximum;
                signal_values.insert(
                    "Test_Result".to_string(),
                    SignalValue {
                        raw_value: if passed { 1.0 } else { 0.0 },
                        engineering_value: if passed { 1.0 } else { 0.0 },
                        unit: "status".to_string(),
                        description: Some(if passed {
                            "PASSED".to_string()
                        } else {
                            "FAILED".to_string()
                        }),
                        min_value: 0.0,
                        max_value: 1.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                "DM08_Test_Results".to_string()
            }
            DiagnosticMessageType::DM24 => {
                DM24::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM24 decode error: {:?}", e))?;

                signal_values =
                    self.extract_signals_from_metadata(DM24::metadata(), &log_entry.data);

                "DM24_SPN_Support".to_string()
            }
            DiagnosticMessageType::DM30 => {
                let dm30 = DM30::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM30 decode error: {:?}", e))?;

                signal_values =
                    self.extract_signals_from_metadata(DM30::metadata(), &log_entry.data);

                // Add pass/fail status (custom logic)
                let passed = dm30.test_value >= dm30.test_limit_minimum
                    && dm30.test_value <= dm30.test_limit_maximum;
                signal_values.insert(
                    "Test_Result".to_string(),
                    SignalValue {
                        raw_value: if passed { 1.0 } else { 0.0 },
                        engineering_value: if passed { 1.0 } else { 0.0 },
                        unit: "status".to_string(),
                        description: Some(if passed {
                            "PASSED".to_string()
                        } else {
                            "FAILED".to_string()
                        }),
                        min_value: 0.0,
                        max_value: 1.0,
                        factor: 1.0,
                        offset: 0.0,
                    },
                );

                "DM30_Scaled_Test_Results".to_string()
            }
            // Phase 5: Advanced diagnostic messages
            DiagnosticMessageType::DM20 => {
                let dm20 = DM20::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM20 decode error: {:?}", e))?;

                signal_values =
                    self.extract_signals_from_metadata(DM20::metadata(), &log_entry.data);

                // Calculate and display monitor ratio (custom logic)
                if dm20.appl_sys_monitor_denominator > 0 {
                    let ratio = (dm20.appl_sys_monitor_numerator as f64)
                        / (dm20.appl_sys_monitor_denominator as f64);
                    signal_values.insert(
                        "Monitor_Ratio".to_string(),
                        SignalValue {
                            raw_value: ratio,
                            engineering_value: ratio,
                            unit: "ratio".to_string(),
                            description: Some(format!("Performance ratio: {:.3}", ratio)),
                            min_value: 0.0,
                            max_value: 1.0,
                            factor: 1.0,
                            offset: 0.0,
                        },
                    );
                }

                "DM20_Monitor_Performance_Ratio".to_string()
            }
            DiagnosticMessageType::DM21 => {
                let dm21 = DM21::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM21 decode error: {:?}", e))?;

                signal_values =
                    self.extract_signals_from_metadata(DM21::metadata(), &log_entry.data);

                // Add MIL status indicator (custom logic)
                if dm21.distance_while_mi_lis_activated > 0 {
                    signal_values.insert(
                        "MIL_Status".to_string(),
                        SignalValue {
                            raw_value: 1.0,
                            engineering_value: 1.0,
                            unit: "status".to_string(),
                            description: Some("MIL has been activated".to_string()),
                            min_value: 0.0,
                            max_value: 1.0,
                            factor: 1.0,
                            offset: 0.0,
                        },
                    );
                }

                "DM21_Diagnostic_Readiness_2".to_string()
            }
            // Phase 6: Additional diagnostic messages (using metadata-based extraction)
            DiagnosticMessageType::DM29 => {
                let dm29 = DM29::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM29 decode error: {:?}", e))?;

                // Use metadata-based signal extraction
                signal_values =
                    self.extract_signals_from_metadata(DM29::metadata(), &log_entry.data);

                // Add summary if there are DTCs
                if dm29.all_pending_dt_cs > 0 {
                    signal_values.insert(
                        "DTC_Summary".to_string(),
                        SignalValue {
                            raw_value: dm29.all_pending_dt_cs as f64,
                            engineering_value: dm29.all_pending_dt_cs as f64,
                            unit: "status".to_string(),
                            description: Some(format!(
                                "{} total pending ({} emission-related, {} MIL-on, {} permanent)",
                                dm29.all_pending_dt_cs,
                                dm29.pending_dt_cs,
                                dm29.mil_on_dt_cs,
                                dm29.permanent_dt_cs
                            )),
                            min_value: 0.0,
                            max_value: 255.0,
                            factor: 1.0,
                            offset: 0.0,
                        },
                    );
                }

                "DM29_Regulated_DTC_Counts".to_string()
            }
            DiagnosticMessageType::DM31 => {
                DM31::decode(log_entry.can_id, &log_entry.data)
                    .map_err(|e| anyhow::anyhow!("DM31 decode error: {:?}", e))?;

                // Use metadata-based signal extraction
                signal_values =
                    self.extract_signals_from_metadata(DM31::metadata(), &log_entry.data);

                "DM31_DTC_To_Lamp_Association".to_string()
            }
            _ => format!("DM{:02}_Diagnostic_Message", diagnostic_type as u8),
        };

        let decoded_msg = DecodedMessage {
            log_entry: log_entry.clone(),
            message_name,
            device_id,
            dbc_type: "J1939-73_Diagnostic".to_string(),
            signal_values,
            timestamp_formatted: format!("Line {}", log_entry.line_number),
        };

        Ok(decoded_msg)
    }

    /// Extract signal values from raw CAN data using message metadata
    ///
    /// This function automatically extracts all signals from a CAN message using
    /// the generated metadata, eliminating the need for manual signal extraction.
    fn extract_signals_from_metadata(
        &self,
        metadata: &'static MessageMetadata,
        data: &[u8],
    ) -> HashMap<String, SignalValue> {
        use cando_messages::encoder::{apply_scaling, extract_signal};

        let mut signal_values = HashMap::new();

        for signal in metadata.signals {
            // Extract raw value from CAN data
            let raw_value = match extract_signal(
                data,
                signal.start_bit as usize,
                signal.signal_size as usize,
            ) {
                Ok(val) => val,
                Err(_) => {
                    // If extraction fails, skip this signal
                    continue;
                }
            };

            // Apply scaling if needed
            let engineering_value = if signal.factor != 1.0 || signal.offset != 0.0 {
                apply_scaling(
                    raw_value,
                    signal.factor,
                    signal.offset,
                    matches!(
                        signal.value_type,
                        cando_messages::metadata::ValueType::Signed
                    ),
                    signal.signal_size as usize,
                )
            } else {
                raw_value as f64
            };

            // Get value description if available
            let description = if !signal.value_descriptions.is_empty() {
                // Try to find matching value description
                signal
                    .value_descriptions
                    .iter()
                    .find(|(val, _)| *val == raw_value)
                    .map(|(_, desc)| desc.to_string())
            } else {
                None
            };

            // Create description with value and unit
            let full_description = if let Some(val_desc) = description {
                Some(format!("{}: {}", signal.name, val_desc))
            } else if !signal.unit.is_empty() {
                Some(format!(
                    "{}: {:.2} {}",
                    signal.name, engineering_value, signal.unit
                ))
            } else {
                Some(format!("{}: {:.2}", signal.name, engineering_value))
            };

            signal_values.insert(
                signal.name.to_string(),
                SignalValue {
                    raw_value: raw_value as f64,
                    engineering_value,
                    unit: signal.unit.to_string(),
                    description: full_description,
                    min_value: signal.min,
                    max_value: signal.max,
                    factor: signal.factor,
                    offset: signal.offset,
                },
            );
        }

        signal_values
    }

    /// Helper function to add DTC signal values from any DtcMessage implementation
    fn add_dtc_signal_values<T: DtcMessage>(
        &self,
        signal_values: &mut HashMap<String, SignalValue>,
        msg: &T,
        dtc_type: &str,
    ) {
        let lamp_status = msg.lamp_status();
        let active_dtcs = msg.get_active_dtcs();

        // Add lamp status signals
        self.add_lamp_signal(signal_values, "MIL", lamp_status.mil);
        self.add_lamp_signal(signal_values, "Red_Stop_Lamp", lamp_status.red_stop);
        self.add_lamp_signal(
            signal_values,
            "Amber_Warning_Lamp",
            lamp_status.amber_warning,
        );
        self.add_lamp_signal(signal_values, "Protect_Lamp", lamp_status.protect);

        // Add flash signals if any are active
        if lamp_status.any_flash_active() {
            self.add_lamp_signal(signal_values, "Flash_MIL", lamp_status.flash_mil);
            self.add_lamp_signal(signal_values, "Flash_Red_Stop", lamp_status.flash_red_stop);
            self.add_lamp_signal(
                signal_values,
                "Flash_Amber_Warning",
                lamp_status.flash_amber_warning,
            );
            self.add_lamp_signal(signal_values, "Flash_Protect", lamp_status.flash_protect);
        }

        // Add fault count
        signal_values.insert(
            format!("{}_Fault_Count", dtc_type),
            SignalValue {
                raw_value: active_dtcs.len() as f64,
                engineering_value: active_dtcs.len() as f64,
                unit: "count".to_string(),
                description: Some(format!(
                    "Number of {} faults",
                    dtc_type.replace('_', " ").to_lowercase()
                )),
                min_value: 0.0,
                max_value: 19.0,
                factor: 1.0,
                offset: 0.0,
            },
        );

        // Add all active DTCs
        for (i, dtc) in active_dtcs.iter().enumerate() {
            let prefix = format!("{}_{:02}", dtc_type, i + 1);

            signal_values.insert(
                format!("{}_SPN", prefix),
                SignalValue {
                    raw_value: dtc.spn as f64,
                    engineering_value: dtc.spn as f64,
                    unit: "SPN".to_string(),
                    description: Some(format!("Suspect Parameter Number: {}", dtc.spn)),
                    min_value: 0.0,
                    max_value: 524287.0,
                    factor: 1.0,
                    offset: 0.0,
                },
            );

            signal_values.insert(
                format!("{}_FMI", prefix),
                SignalValue {
                    raw_value: dtc.fmi as f64,
                    engineering_value: dtc.fmi as f64,
                    unit: "FMI".to_string(),
                    description: Some(format!("FMI {}: {}", dtc.fmi, dtc.fmi_description())),
                    min_value: 0.0,
                    max_value: 31.0,
                    factor: 1.0,
                    offset: 0.0,
                },
            );

            signal_values.insert(
                format!("{}_OC", prefix),
                SignalValue {
                    raw_value: dtc.occurrence_count as f64,
                    engineering_value: dtc.occurrence_count as f64,
                    unit: "count".to_string(),
                    description: Some(format!("Occurrence Count: {}", dtc.occurrence_count)),
                    min_value: 0.0,
                    max_value: 127.0,
                    factor: 1.0,
                    offset: 0.0,
                },
            );

            signal_values.insert(
                format!("{}_CM", prefix),
                SignalValue {
                    raw_value: if dtc.conversion_method { 1.0 } else { 0.0 },
                    engineering_value: if dtc.conversion_method { 1.0 } else { 0.0 },
                    unit: "method".to_string(),
                    description: Some(
                        if dtc.conversion_method {
                            "J1939-73"
                        } else {
                            "J1587"
                        }
                        .to_string(),
                    ),
                    min_value: 0.0,
                    max_value: 1.0,
                    factor: 1.0,
                    offset: 0.0,
                },
            );
        }
    }

    /// Helper to add a lamp status signal
    fn add_lamp_signal(
        &self,
        signal_values: &mut HashMap<String, SignalValue>,
        name: &str,
        state: LampState,
    ) {
        let (value, desc) = match state {
            LampState::Off => (0.0, "OFF"),
            LampState::On => (1.0, "ON"),
            LampState::Reserved => (2.0, "RESERVED"),
            LampState::NotAvailable => (3.0, "N/A"),
        };

        signal_values.insert(
            name.to_string(),
            SignalValue {
                raw_value: value,
                engineering_value: value,
                unit: "status".to_string(),
                description: Some(format!("{} ({})", name.replace('_', " "), desc)),
                min_value: 0.0,
                max_value: 3.0,
                factor: 1.0,
                offset: 0.0,
            },
        );
    }

    /// Create raw analysis for unknown messages
    fn create_raw_analysis(&self, log_entry: &CanLogEntry, device_id: u32) -> DecodedMessage {
        let mut signal_values = HashMap::new();

        // Analyze the message structure and extract interesting patterns
        let message_name = self.identify_message_pattern(log_entry);

        // Create pseudo-signals for raw data analysis
        for (i, &byte) in log_entry.data.iter().enumerate() {
            signal_values.insert(
                format!("Byte_{}", i),
                SignalValue {
                    raw_value: byte as f64,
                    engineering_value: byte as f64,
                    unit: "raw".to_string(),
                    description: Some(format!("Raw byte {} value", i)),
                    min_value: 0.0,
                    max_value: 255.0,
                    factor: 1.0,
                    offset: 0.0,
                },
            );
        }

        // Add J1939 interpretation if applicable
        if self.is_j1939_message(log_entry.can_id) {
            let j1939_analysis = self.analyze_j1939_id(log_entry.can_id);
            signal_values.insert(
                "J1939_PGN".to_string(),
                SignalValue {
                    raw_value: j1939_analysis.pgn as f64,
                    engineering_value: j1939_analysis.pgn as f64,
                    unit: "PGN".to_string(),
                    description: Some(j1939_analysis.description),
                    min_value: 0.0,
                    max_value: 262143.0,
                    factor: 1.0,
                    offset: 0.0,
                },
            );
        }

        DecodedMessage {
            log_entry: log_entry.clone(),
            message_name,
            device_id,
            dbc_type: "RAW".to_string(),
            signal_values,
            timestamp_formatted: format!("Line {}", log_entry.line_number),
        }
    }

    /// Identify message patterns based on CAN ID and data
    fn identify_message_pattern(&self, log_entry: &CanLogEntry) -> String {
        format!("Unknown_0x{:08X}", log_entry.can_id)
    }

    /// Check if message follows J1939 format
    fn is_j1939_message(&self, can_id: u32) -> bool {
        // J1939 extended frames typically have specific patterns
        can_id > 0x00FFFFFF // Extended frame format
    }

    /// Analyze J1939 CAN ID structure
    fn analyze_j1939_id(&self, can_id: u32) -> J1939Analysis {
        let priority = (can_id >> 26) & 0x07;
        let _reserved = (can_id >> 25) & 0x01;
        let data_page = (can_id >> 24) & 0x01;
        let pdu_format = (can_id >> 16) & 0xFF;
        let pdu_specific = (can_id >> 8) & 0xFF;
        let source_address = can_id & 0xFF;

        let pgn = if pdu_format >= 240 {
            // PDU Format 2 (broadcast)
            (data_page << 16) | (pdu_format << 8) | pdu_specific
        } else {
            // PDU Format 1 (peer-to-peer)
            (data_page << 16) | (pdu_format << 8)
        };

        let description = match pgn {
            65267 => "Vehicle Position (0xFEF3)".to_string(),
            65226 => "Active Diagnostic Trouble Codes (0xFECA)".to_string(),
            65235 => "Electronic Engine Controller 1 (0xFED3)".to_string(),
            _ => format!(
                "PGN {} (0x{:04X}) - Priority: {}, SA: 0x{:02X}",
                pgn, pgn, priority, source_address
            ),
        };

        J1939Analysis {
            pgn,
            priority: priority as u8,
            source_address: source_address as u8,
            description,
        }
    }

    /// Try flexible matching for real-world message patterns
    /// Note: With metadata-based matching, this is mostly redundant as find_matching_message
    /// already does flexible matching by base ID. Kept for compatibility.
    fn try_flexible_dbc_matching(
        &self,
        log_entry: &CanLogEntry,
        device_id: u32,
    ) -> Option<DecodedMessage> {
        // Map real-world CAN IDs to likely message base IDs
        let base_id = match get_j1939_base_id(log_entry.can_id & CAN_EFF_MASK) {
            0x18F37000 => Some(0x18F37000), // MSM2
            0x1CEBFF00 => Some(0x1CEBFF00), // MSM1
            0x1CECFF00 => Some(0x1CECFF00), // MET
            0x18FC0100 => Some(0x18FC0100), // MSM3
            _ => None,
        };

        if let Some(bid) = base_id
            && let Some(known_msg) = self.known_messages.get(&bid)
        {
            // Try to decode using metadata
            if let Ok(decoded_msg) = self.decode_signals(
                log_entry,
                known_msg.metadata,
                known_msg.name,
                device_id,
                known_msg.protocol.as_str(),
            ) {
                return Some(decoded_msg);
            }
        }
        None
    }

    /// Generate comprehensive statistics
    pub fn generate_statistics(&mut self) -> Result<()> {
        println!("Generating message and signal statistics...");

        for decoded_msg in &self.decoded_messages {
            let key = format!("{}_{}", decoded_msg.dbc_type, decoded_msg.message_name);

            let stats = self
                .message_stats
                .entry(key.clone())
                .or_insert(MessageStats {
                    message_name: decoded_msg.message_name.clone(),
                    count: 0,
                    first_seen: 0.0,
                    last_seen: 0.0,
                    frequency_hz: 0.0,
                    device_ids: Vec::new(),
                    signal_stats: HashMap::new(),
                });

            stats.count += 1;

            // Track device IDs
            if !stats.device_ids.contains(&decoded_msg.device_id) {
                stats.device_ids.push(decoded_msg.device_id);
            }

            // Update signal statistics
            for (signal_name, signal_value) in &decoded_msg.signal_values {
                let signal_stats =
                    stats
                        .signal_stats
                        .entry(signal_name.clone())
                        .or_insert(SignalStats {
                            min_value: signal_value.engineering_value,
                            max_value: signal_value.engineering_value,
                            avg_value: 0.0,
                            sample_count: 0,
                            unit: signal_value.unit.clone(),
                            value_descriptions: Vec::new(),
                        });

                // Update min/max
                signal_stats.min_value = signal_stats.min_value.min(signal_value.engineering_value);
                signal_stats.max_value = signal_stats.max_value.max(signal_value.engineering_value);

                // Update running average
                signal_stats.avg_value = (signal_stats.avg_value
                    * signal_stats.sample_count as f64
                    + signal_value.engineering_value)
                    / (signal_stats.sample_count + 1) as f64;
                signal_stats.sample_count += 1;
            }
        }

        println!(
            "   Generated statistics for {} unique message types",
            self.message_stats.len()
        );
        Ok(())
    }

    /// Display comprehensive analysis results
    pub fn display_results(&self) -> Result<()> {
        match self.args.output {
            OutputFormat::Text => self.display_results_text(),
            OutputFormat::Csv => self.display_results_csv(),
            OutputFormat::Json => self.display_results_json(),
        }
    }

    /// Display results in text format (original format)
    fn display_results_text(&self) -> Result<()> {
        println!("\n{}", "=".repeat(80));
        println!("COMPREHENSIVE CAN LOG ANALYSIS RESULTS");
        println!("{}", "=".repeat(80));

        self.display_summary();

        if self.args.statistics {
            self.display_statistics();
        }

        self.display_decoded_messages();

        if self.args.timing_analysis {
            self.display_timing_analysis();
        }

        Ok(())
    }

    /// Display results in CSV format
    fn display_results_csv(&self) -> Result<()> {
        // Print CSV header
        println!(
            "Line,MessageName,DeviceID,CANID,DLC,DBCType,SignalName,RawValue,EngineeringValue,Unit,Description"
        );

        // Print data rows
        for msg in &self.decoded_messages {
            for (signal_name, signal_value) in &msg.signal_values {
                // Skip internal signals for cleaner CSV
                if signal_name.starts_with("Byte_") || signal_name == "J1939_PGN" {
                    continue;
                }

                println!(
                    "{},{},0x{:02X},0x{:08X},{},{},{},{},{},\"{}\",\"{}\"",
                    msg.log_entry.line_number,
                    msg.message_name,
                    msg.device_id,
                    msg.log_entry.can_id,
                    msg.log_entry.dlc,
                    msg.dbc_type,
                    signal_name,
                    signal_value.raw_value,
                    signal_value.engineering_value,
                    signal_value.unit,
                    signal_value.description.as_deref().unwrap_or("")
                );
            }
        }

        Ok(())
    }

    /// Display results in JSON format
    fn display_results_json(&self) -> Result<()> {
        let json_data = serde_json::json!({
            "summary": {
                "total_entries": self.log_entries.len(),
                "decoded_messages": self.decoded_messages.len(),
                "unique_message_types": self.message_stats.len(),
                "unique_can_ids": self.log_entries.iter().map(|e| e.can_id).collect::<std::collections::HashSet<_>>().len(),
                "unique_device_ids": self.decoded_messages.iter().map(|m| m.device_id).collect::<std::collections::HashSet<_>>().len()
            },
            "decoded_messages": self.decoded_messages,
            "statistics": if self.args.statistics { Some(&self.message_stats) } else { None }
        });

        println!("{}", serde_json::to_string_pretty(&json_data)?);
        Ok(())
    }

    /// Display summary information
    fn display_summary(&self) {
        println!("\nSUMMARY");
        println!("{}", "-".repeat(40));
        println!("Total log entries: {}", self.log_entries.len());
        println!("Decoded messages: {}", self.decoded_messages.len());
        println!(
            "Unknown messages: {}",
            self.log_entries.len() - self.decoded_messages.len()
        );
        println!("Unique message types: {}", self.message_stats.len());

        // Count unique CAN IDs
        let unique_can_ids: std::collections::HashSet<u32> =
            self.log_entries.iter().map(|e| e.can_id).collect();
        println!("Unique CAN IDs: {}", unique_can_ids.len());

        // Count unique device IDs
        let unique_devices: std::collections::HashSet<u32> =
            self.decoded_messages.iter().map(|m| m.device_id).collect();
        println!("Unique device IDs: {}", unique_devices.len());

        if !unique_devices.is_empty() {
            print!("Device IDs found: ");
            let mut devices: Vec<u32> = unique_devices.into_iter().collect();
            devices.sort();
            for (i, device_id) in devices.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("0x{:02X}", device_id);
            }
            println!();
        }
    }

    /// Display message statistics
    fn display_statistics(&self) {
        println!("\nMESSAGE STATISTICS");
        println!("{}", "-".repeat(40));

        let mut stats: Vec<_> = self.message_stats.values().collect();
        stats.sort_by(|a, b| b.count.cmp(&a.count));

        for stat in stats {
            println!("\n  {} ({} occurrences)", stat.message_name, stat.count);
            print!("   Devices: ");
            for (i, device_id) in stat.device_ids.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("0x{:02X}", device_id);
            }
            println!();

            if self.args.verbose && !stat.signal_stats.is_empty() {
                println!("   Signal ranges:");
                let mut signals: Vec<_> = stat.signal_stats.iter().collect();
                signals.sort_by_key(|(name, _)| *name);

                for (signal_name, signal_stat) in signals {
                    println!(
                        "     {}: {:.3} to {:.3} (avg: {:.3}) {} [n={}]",
                        signal_name,
                        signal_stat.min_value,
                        signal_stat.max_value,
                        signal_stat.avg_value,
                        signal_stat.unit,
                        signal_stat.sample_count
                    );
                }
            }
        }
    }

    /// Format a DTC message with enhanced human-readable output
    fn format_dtc_message_summary(&self, msg: &DecodedMessage) -> Option<String> {
        // Check if this is a DTC-style message
        if !msg.message_name.starts_with("DM0")
            && !msg.message_name.starts_with("DM1")
            && !msg.message_name.starts_with("DM2")
        {
            return None;
        }

        // Skip DM03 (command message)
        if msg.message_name.starts_with("DM03") {
            return None;
        }

        let mut output = String::new();

        // Extract fault count
        let fault_count = msg
            .signal_values
            .iter()
            .find(|(k, _)| k.ends_with("_Fault_Count"))
            .map(|(_, v)| v.engineering_value as usize)
            .unwrap_or(0);

        // Extract lamp status
        let mil = msg
            .signal_values
            .get("MIL")
            .map(|v| {
                if v.engineering_value == 1.0 {
                    "ON"
                } else {
                    "OFF"
                }
            })
            .unwrap_or("OFF");
        let red_stop = msg
            .signal_values
            .get("Red_Stop_Lamp")
            .map(|v| {
                if v.engineering_value == 1.0 {
                    "ON"
                } else {
                    "OFF"
                }
            })
            .unwrap_or("OFF");
        let amber = msg
            .signal_values
            .get("Amber_Warning_Lamp")
            .map(|v| {
                if v.engineering_value == 1.0 {
                    "ON"
                } else {
                    "OFF"
                }
            })
            .unwrap_or("OFF");

        output.push_str("\n   Diagnostic Summary:\n");
        output.push_str(&format!("      Active Faults: {}\n", fault_count));
        output.push_str(&format!(
            "      MIL: {}  |  Red Stop: {}  |  Amber Warning: {}\n",
            mil, red_stop, amber
        ));

        // List all faults with details
        if fault_count > 0 {
            output.push_str("   \n   Active Faults:\n");

            for i in 1..=fault_count {
                // Find SPN, FMI, OC for this fault
                let prefix_active = format!("Active_{:02}", i);
                let prefix_prev = format!("Previously_Active_{:02}", i);
                let prefix_pending = format!("Pending_{:02}", i);
                let prefix_mil = format!("MIL_On_{:02}", i);
                let prefix_prevmil = format!("Previously_MIL_On_{:02}", i);

                let prefix = if msg
                    .signal_values
                    .contains_key(&format!("{}_SPN", prefix_active))
                {
                    prefix_active
                } else if msg
                    .signal_values
                    .contains_key(&format!("{}_SPN", prefix_prev))
                {
                    prefix_prev
                } else if msg
                    .signal_values
                    .contains_key(&format!("{}_SPN", prefix_pending))
                {
                    prefix_pending
                } else if msg
                    .signal_values
                    .contains_key(&format!("{}_SPN", prefix_mil))
                {
                    prefix_mil
                } else {
                    prefix_prevmil
                };

                if let Some(spn) = msg.signal_values.get(&format!("{}_SPN", prefix)) {
                    let fmi = msg.signal_values.get(&format!("{}_FMI", prefix));
                    let oc = msg.signal_values.get(&format!("{}_OC", prefix));
                    let _cm = msg.signal_values.get(&format!("{}_CM", prefix));

                    let fmi_desc = fmi
                        .and_then(|f| f.description.as_ref())
                        .map(|d| d.as_str())
                        .unwrap_or("");

                    output.push_str(&format!(
                        "      {}. SPN {} - {}\n",
                        i, spn.engineering_value as u32, fmi_desc
                    ));

                    if let Some(oc_val) = oc {
                        output.push_str(&format!(
                            "         Occurrence Count: {}\n",
                            oc_val.engineering_value as u32
                        ));
                    }
                }
            }
        }

        Some(output)
    }

    /// Display detailed decoded messages
    fn display_decoded_messages(&self) {
        println!("\nDECODED MESSAGES");
        println!("{}", "-".repeat(40));

        let display_count = if self.args.verbose {
            self.decoded_messages.len()
        } else {
            self.decoded_messages.len().min(50)
        };

        for (i, msg) in self.decoded_messages.iter().take(display_count).enumerate() {
            if !self.args.verbose && i >= 10 {
                println!("... showing first 10 messages (use --verbose for all)");
                break;
            }

            println!(
                "\n  {} [{}] - Device 0x{:02X} ({})",
                msg.message_name, msg.dbc_type, msg.device_id, msg.timestamp_formatted
            );

            // Show raw CAN data with enhanced formatting
            print!(
                "   Raw: CAN ID 0x{:08X} [{}] ",
                msg.log_entry.can_id, msg.log_entry.dlc
            );
            for byte in &msg.log_entry.data {
                print!("{:02X} ", byte);
            }
            println!();

            // Enhanced DTC message formatting
            if let Some(dtc_summary) = self.format_dtc_message_summary(msg) {
                print!("{}", dtc_summary);
            }

            // For RAW messages, show additional analysis
            if msg.dbc_type == "RAW" {
                println!("   Analysis: Unknown CAN device message (not in DBC)");
                if let Some(j1939_signal) = msg.signal_values.get("J1939_PGN") {
                    println!(
                        "   J1939: {}",
                        j1939_signal.description.as_deref().unwrap_or("")
                    );
                }

                // Show data pattern analysis
                self.show_data_pattern_analysis(&msg.log_entry);
            }

            // Show decoded signals
            if !msg.signal_values.is_empty() {
                let signal_type = if msg.dbc_type == "RAW" {
                    "Raw Data Bytes"
                } else {
                    "Decoded Signals"
                };
                println!("   {}:", signal_type);
                let mut signals: Vec<_> = msg.signal_values.iter().collect();
                signals.sort_by_key(|(name, _)| *name);

                for (signal_name, signal_value) in signals {
                    if signal_name.starts_with("Byte_") && msg.dbc_type == "RAW" {
                        // Show bytes in a more compact format for raw messages
                        continue;
                    }
                    if signal_name == "J1939_PGN" && msg.dbc_type == "RAW" {
                        // Skip J1939_PGN in detailed signal list, it's shown above
                        continue;
                    }

                    let desc_str = match &signal_value.description {
                        Some(desc) => format!(" ({})", desc),
                        None => String::new(),
                    };

                    println!(
                        "     {}: {:.3} {} [raw: {:.0}]{}",
                        signal_name,
                        signal_value.engineering_value,
                        signal_value.unit,
                        signal_value.raw_value,
                        desc_str
                    );
                }

                // For raw messages, show byte summary
                if msg.dbc_type == "RAW" {
                    print!("   Bytes: ");
                    for (i, byte) in msg.log_entry.data.iter().enumerate() {
                        if i > 0 {
                            print!(", ");
                        }
                        print!("{}:0x{:02X}", i, byte);
                    }
                    println!();
                }
            }
        }

        if display_count < self.decoded_messages.len() {
            println!(
                "\n... {} more decoded messages (use --verbose to see all)",
                self.decoded_messages.len() - display_count
            );
        }
    }

    /// Display timing analysis
    fn display_timing_analysis(&self) {
        println!("\nTIMING ANALYSIS");
        println!("{}", "-".repeat(40));
        println!("(Note: candump format doesn't include timestamps)");
        println!("Message sequence shows relative ordering only");

        // Show message frequency by counting occurrences
        println!("\nEstimated message frequencies (based on occurrence count):");
        let mut stats: Vec<_> = self.message_stats.values().collect();
        stats.sort_by(|a, b| b.count.cmp(&a.count));

        for stat in stats {
            println!("  {}: {} occurrences", stat.message_name, stat.count);
        }
    }

    /// Export results to JSON
    pub fn export_json(&self) -> Result<()> {
        if let Some(ref json_file) = self.args.export_json {
            println!("Exporting decoded data to JSON: {}", json_file);

            let json_data = serde_json::json!({
                "summary": {
                    "total_entries": self.log_entries.len(),
                    "decoded_messages": self.decoded_messages.len(),
                    "unique_message_types": self.message_stats.len()
                },
                "decoded_messages": self.decoded_messages,
                "statistics": self.message_stats
            });

            let mut file = File::create(json_file)?;
            file.write_all(serde_json::to_string_pretty(&json_data)?.as_bytes())?;
            println!("   JSON export completed");
        }
        Ok(())
    }

    /// Show data pattern analysis for raw messages
    fn show_data_pattern_analysis(&self, log_entry: &CanLogEntry) {
        if log_entry.data.len() >= 2 {
            println!("   Pattern: Unknown - analyzing as generic data");
        }
    }

    /// Export results to CSV
    pub fn export_csv(&self) -> Result<()> {
        if let Some(ref csv_file) = self.args.export_csv {
            println!("Exporting decoded data to CSV: {}", csv_file);

            let mut file = File::create(csv_file)?;

            // Write CSV header
            writeln!(
                file,
                "Line,MessageName,DeviceID,CANID,DLC,SignalName,RawValue,EngineeringValue,Unit,Description"
            )?;

            // Write data rows
            for msg in &self.decoded_messages {
                for (signal_name, signal_value) in &msg.signal_values {
                    writeln!(
                        file,
                        "{},{},0x{:02X},0x{:08X},{},{},{},{},\"{}\",\"{}\"",
                        msg.log_entry.line_number,
                        msg.message_name,
                        msg.device_id,
                        msg.log_entry.can_id,
                        msg.log_entry.dlc,
                        signal_name,
                        signal_value.raw_value,
                        signal_value.engineering_value,
                        signal_value.unit,
                        signal_value.description.as_deref().unwrap_or("")
                    )?;
                }
            }

            println!("   CSV export completed");
        }
        Ok(())
    }

    /// Run complete analysis
    pub fn run(&mut self) -> Result<()> {
        self.parse_log_file()?;
        self.decode_messages()?;
        self.generate_statistics()?;
        self.display_results()?;

        // Only export to files if explicitly requested (separate from output format)
        self.export_json()?;
        self.export_csv()?;

        // Only show completion message for text format to avoid cluttering structured output
        if matches!(self.args.output, OutputFormat::Text) {
            println!("\nCAN log analysis completed successfully!");
        }

        Ok(())
    }
}

/// Handle errors by printing them and exiting with non-zero status
fn handle_error(e: anyhow::Error) -> ! {
    eprintln!("Error: {}", e);
    std::process::exit(1);
}

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.contains(&"--generate-manpage".to_string()) {
        #[cfg(feature = "manpages")]
        {
            use clap::CommandFactory;
            use clap_mangen::Man;
            let man = Man::new(Args::command());
            man.render(&mut std::io::stdout())
                .expect("Failed to render man page to stdout");
            return;
        }
        #[cfg(not(feature = "manpages"))]
        {
            eprintln!(
                "clap_mangen feature not enabled. Build with --features manpages to generate man pages"
            );
            std::process::exit(1);
        }
    }

    let args = Args::parse();

    let mut analyzer = match CanLogAnalyzer::new(args) {
        Ok(analyzer) => analyzer,
        Err(e) => handle_error(e),
    };

    if let Err(e) = analyzer.run() {
        handle_error(e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_filter_parsing() {
        assert_eq!(
            CanLogAnalyzer::parse_device_filter(&Some("0x82,0x83".to_string()))
                .expect("Device filter parsing should not fail in test"),
            Some(vec![0x82, 0x83])
        );
        assert_eq!(
            CanLogAnalyzer::parse_device_filter(&Some("130,131".to_string()))
                .expect("Device filter parsing should not fail in test"),
            Some(vec![130, 131])
        );
        assert_eq!(
            CanLogAnalyzer::parse_device_filter(&None)
                .expect("Device filter parsing should not fail in test"),
            None
        );
    }

    #[test]
    fn test_message_filter_parsing() {
        assert_eq!(
            CanLogAnalyzer::parse_message_filter(&Some("Motor,Status".to_string())),
            Some(vec!["Motor".to_string(), "Status".to_string()])
        );
        assert_eq!(CanLogAnalyzer::parse_message_filter(&None), None);
    }

    #[test]
    fn test_can_log_entry_creation() {
        let entry = CanLogEntry {
            interface: "can0".to_string(),
            can_id: 0x18F37082,
            dlc: 8,
            data: vec![0x83, 0x17, 0x00, 0x7D, 0xFF, 0xFF, 0xFF, 0xFF],
            timestamp: None,
            line_number: 1,
        };

        assert_eq!(entry.interface, "can0");
        assert_eq!(entry.can_id, 0x18F37082);
        assert_eq!(entry.data.len(), 8);
    }
}
