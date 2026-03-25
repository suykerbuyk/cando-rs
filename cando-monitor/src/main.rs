//! Real-Time CAN Bus Monitoring Utility
//!
//! This utility provides comprehensive real-time monitoring of CAN bus traffic
//! for J1939 (SAE J1939 vehicle bus standard) systems.
//! It connects to Linux CAN interfaces and decodes messages using compile-time
//! generated metadata, displaying all signals in human-readable format.
//!
//! Features:
//! - Real-time CAN message monitoring and decoding
//! - Support for J1939 message formats
//! - Human-readable signal decoding with engineering units
//! - Raw hex dumps for unknown message types
//! - Comprehensive error handling and graceful degradation
//! - Configurable filtering and display options

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use clap::Parser;
use socketcan::{CanFrame, CanSocket, EmbeddedFrame, Socket};

// CAN ID bit manipulation constants imported from common module

use std::collections::HashMap;
use std::time::Duration;

use cando_messages::common::{CAN_EFF_MASK, CAN_STD_ID_MAX, get_j1939_base_id};
use cando_messages::metadata::{MessageMetadata, SignalMetadata};
use cando_messages::j1939;

/// Command-line arguments for the CAN monitoring utility
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Real-time CAN bus monitoring utility for J1939 systems"
)]
struct Args {
    /// CAN interface to monitor (e.g., can0, vcan0)
    #[arg(help = "CAN interface name (e.g., can0, vcan0)")]
    interface: String,

    /// Show only decoded messages (hide unknown messages)
    #[arg(long, short = 'd')]
    decoded_only: bool,

    /// Show only specific message types (comma-separated list)
    #[arg(long, short = 'f')]
    filter: Option<String>,

    /// Show raw hex dumps along with decoded messages
    #[arg(long, short = 'r')]
    show_raw: bool,

    /// Quiet mode - reduce output verbosity
    #[arg(long, short = 'q')]
    quiet: bool,

    /// Show statistics every N seconds
    #[arg(long, default_value = "30")]
    stats_interval: u64,

    /// Generate man page and exit (internal use)
    #[arg(long = "generate-manpage", hide = true)]
    generate_manpage: bool,
}

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

/// Statistics tracking for monitoring session
#[derive(Debug, Default)]
struct MonitoringStats {
    total_messages: u64,
    decoded_messages: u64,
    unknown_messages: u64,
    j1939_messages: u64,
    error_frames: u64,
    start_time: Option<DateTime<Utc>>,
}

/// Main monitoring application
struct CanMonitor {
    socket: CanSocket,
    known_messages: HashMap<u32, KnownMessage>,
    stats: MonitoringStats,
    args: Args,
    message_filters: Option<Vec<String>>,
}

impl CanMonitor {
    /// Create a new CAN monitor instance
    fn new(args: Args) -> Result<Self> {
        // Open CAN socket
        let socket = CanSocket::open(&args.interface)
            .map_err(|e| anyhow!("Failed to open CAN interface '{}': {}", args.interface, e))?;

        // Set socket timeout for non-blocking operation
        socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .map_err(|e| anyhow!("Failed to set socket timeout: {}", e))?;

        // Parse message filters
        let message_filters = args
            .filter
            .as_ref()
            .map(|f| f.split(',').map(|s| s.trim().to_string()).collect());

        let mut monitor = Self {
            socket,
            known_messages: HashMap::new(),
            stats: MonitoringStats::default(),
            args,
            message_filters,
        };

        // Load message metadata
        monitor.load_message_metadata();

        Ok(monitor)
    }

    /// Load message metadata from generated types
    fn load_message_metadata(&mut self) {
        if !self.args.quiet {
            println!("Loading message metadata...");
        }

        // Load J1939 messages
        let j1939_count = self.load_protocol_messages(&j1939::J1939_METADATA, Protocol::J1939);
        if !self.args.quiet {
            println!("   Loaded {} J1939 messages", j1939_count);
        }

        if !self.args.quiet {
            println!("   Total known messages: {}", self.known_messages.len());
            println!();
        }
    }

    /// Load messages from a specific protocol metadata
    fn load_protocol_messages(
        &mut self,
        protocol_meta: &'static cando_messages::metadata::ProtocolMetadata,
        protocol: Protocol,
    ) -> usize {
        let mut count = 0;
        for &msg_meta in protocol_meta.messages {
            let known_msg = KnownMessage {
                name: msg_meta.name,
                metadata: msg_meta,
                protocol,
            };

            // Extract base ID using J1939-aware logic
            // This correctly handles both PDU1 (strips dest+src) and PDU2 (strips src only)
            let exact_id = msg_meta.can_id & CAN_EFF_MASK;
            let base_id = get_j1939_base_id(exact_id);

            self.known_messages.insert(base_id, known_msg);

            // Also store by exact 29-bit ID for precise matching
            if base_id != exact_id {
                self.known_messages.insert(exact_id, known_msg);
            }

            count += 1;
        }
        count
    }

    /// Start the monitoring loop
    fn monitor(&mut self) -> Result<()> {
        self.stats.start_time = Some(Utc::now());

        if !self.args.quiet {
            println!(
                "Starting CAN monitoring on interface '{}'",
                self.args.interface
            );
            println!("   Press Ctrl+C to stop monitoring\n");
            println!("{}", "=".repeat(80));
        }

        let mut last_stats_time = Utc::now();

        loop {
            // Read CAN frame with timeout
            let read_result = self.socket.read_frame();
            match read_result {
                Ok(frame) => {
                    self.process_frame(frame)?;
                }
                Err(e) => {
                    // Handle timeout gracefully (allows for periodic stats display)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut
                    {
                        // Check if we should display stats
                        let now = Utc::now();
                        let duration = now.signed_duration_since(last_stats_time).num_seconds();
                        if duration >= self.args.stats_interval as i64 {
                            self.display_stats();
                            last_stats_time = now;
                        }
                        continue;
                    } else {
                        return Err(anyhow!("Error reading CAN frame: {}", e));
                    }
                }
            }
        }
    }

    /// Process a received CAN frame
    fn process_frame(&mut self, frame: CanFrame) -> Result<()> {
        self.stats.total_messages += 1;
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string();

        // Handle error frames
        if let CanFrame::Error(_) = &frame {
            self.stats.error_frames += 1;
            if !self.args.decoded_only {
                println!("ERROR FRAME at {}", timestamp);
            }
            return Ok(());
        }

        let can_id = frame.id();
        let data = frame.data();
        let can_id_u32 = match can_id {
            socketcan::Id::Standard(id) => id.as_raw() as u32,
            socketcan::Id::Extended(id) => id.as_raw(), // Already 29-bit, no need for EFF flag
        };

        // Try to find matching message definition
        let known_msg_opt = self.find_matching_message(can_id_u32);
        if let Some(known_msg) = known_msg_opt {
            self.process_known_message(&known_msg, can_id_u32, data, &timestamp)?;
        } else {
            self.process_unknown_message(can_id_u32, data, &timestamp)?;
        }

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

    /// Process a known/decoded message
    fn process_known_message(
        &mut self,
        known_msg: &KnownMessage,
        can_id: u32,
        data: &[u8],
        timestamp: &str,
    ) -> Result<()> {
        self.stats.decoded_messages += 1;

        match known_msg.protocol {
            Protocol::J1939 => self.stats.j1939_messages += 1,
        }

        // Apply message filters if specified
        if let Some(filters) = &self.message_filters
            && !filters.iter().any(|f| known_msg.name.contains(f))
        {
            return Ok(());
        }

        // Display message header
        println!(
            "[{}] {} at {}",
            known_msg.protocol.as_str(),
            known_msg.name,
            timestamp
        );

        // Show raw hex dump if requested
        if self.args.show_raw {
            print_hex_dump(can_id, data, timestamp);
        }

        // Decode and display signals
        let field_values = decode_message_signals(known_msg.metadata, data)?;
        print_signal_values(known_msg.metadata, &field_values);

        // Add separator for readability
        if !self.args.quiet {
            println!("{}", "-".repeat(40));
        }

        Ok(())
    }

    /// Process an unknown/undecoded message
    fn process_unknown_message(&mut self, can_id: u32, data: &[u8], timestamp: &str) -> Result<()> {
        self.stats.unknown_messages += 1;

        // Skip if only showing decoded messages
        if self.args.decoded_only {
            return Ok(());
        }

        println!("Unknown Message at {}", timestamp);

        // Mask to 29-bit CAN ID for display
        let can_id_29bit = can_id & CAN_EFF_MASK;

        // Show standard CAN message format
        println!(
            "   CAN ID: 0x{:08X} ({}) - {}",
            can_id_29bit,
            can_id_29bit,
            classify_can_id(can_id_29bit)
        );
        println!("   DLC: {} bytes", data.len());

        // Show raw data in hex (print_hex_dump handles masking)
        print_hex_dump(can_id, data, timestamp);

        if !self.args.quiet {
            println!("{}", "-".repeat(40));
        }

        Ok(())
    }

    /// Display monitoring statistics
    fn display_stats(&self) {
        if self.args.quiet {
            return;
        }

        let elapsed = if let Some(start_time) = self.stats.start_time {
            Utc::now().signed_duration_since(start_time).num_seconds()
        } else {
            0
        };

        println!("\nMONITORING STATISTICS ({}s elapsed)", elapsed);
        println!("   Total Messages:   {}", self.stats.total_messages);
        println!(
            "   Decoded Messages: {} ({:.1}%)",
            self.stats.decoded_messages,
            if self.stats.total_messages > 0 {
                (self.stats.decoded_messages as f64 / self.stats.total_messages as f64) * 100.0
            } else {
                0.0
            }
        );
        println!("   Unknown Messages: {}", self.stats.unknown_messages);
        println!("   J1939 Messages:   {}", self.stats.j1939_messages);
        if self.stats.error_frames > 0 {
            println!("   Error Frames:     {}", self.stats.error_frames);
        }

        if elapsed > 0 {
            println!(
                "   Message Rate:     {:.1} msg/sec",
                self.stats.total_messages as f64 / elapsed as f64
            );
        }
        println!("{}", "=".repeat(50));
    }
}

/// Decode message signals from raw CAN data using metadata
fn decode_message_signals(metadata: &MessageMetadata, data: &[u8]) -> Result<HashMap<String, f64>> {
    let mut field_values = HashMap::new();

    // Convert data to u64 for bit manipulation
    let mut data_u64 = 0u64;
    for (i, &byte) in data.iter().enumerate().take(8) {
        data_u64 |= (byte as u64) << (i * 8);
    }

    // Decode each signal
    for signal in metadata.signals {
        // Skip zero-length signals to prevent shift overflow
        if signal.signal_size == 0 {
            continue;
        }

        let raw_value = extract_signal_value(data_u64, signal);

        // Apply scaling: engineering_value = (raw_value * factor) + offset
        let engineering_value = (raw_value as f64 * signal.factor) + signal.offset;

        field_values.insert(signal.name.to_string(), engineering_value);
    }

    Ok(field_values)
}

/// Extract a signal value from raw CAN data
fn extract_signal_value(data: u64, signal: &SignalMetadata) -> u64 {
    let start_bit = signal.start_bit as usize;
    let length = signal.signal_size as usize;

    // Extract raw value
    let mask = (1u64 << length) - 1;
    (data >> start_bit) & mask
}

/// Print signal values in human-readable format
fn print_signal_values(metadata: &MessageMetadata, field_values: &HashMap<String, f64>) {
    println!(
        "Message: {} | ID: 0x{:X} | DLC: {}",
        metadata.name, metadata.can_id, metadata.dlc
    );

    for signal in metadata.signals {
        let value = *field_values.get(signal.name).unwrap_or(&0.0);

        // Check if signal has value descriptions (enumerations)
        let display_value = if !signal.value_descriptions.is_empty() {
            // Convert back to raw value for enum lookup
            // Protect against divide-by-zero
            if signal.factor == 0.0 {
                eprintln!(
                    "WARNING: Signal '{}' has factor of 0.0, cannot decode enum value",
                    signal.name
                );
                format!("{:.2}", value)
            } else {
                let raw_value = ((value - signal.offset) / signal.factor).round() as u64;

                if let Some(description) = signal.get_value_description(raw_value) {
                    format!("{} ({})", description, raw_value)
                } else {
                    format!("{:.2}", value)
                }
            }
        } else {
            format!("{:.2}", value)
        };

        println!("  {}: {} {}", signal.name, display_value, signal.unit);
    }
}

/// Print a formatted hex dump of CAN message data
fn print_hex_dump(can_id: u32, bytes: &[u8], timestamp: &str) {
    // Mask to 29-bit for display
    let can_id_29bit = can_id & CAN_EFF_MASK;
    println!(
        "Timestamp: {} | CAN Frame (Extended ID: 0x{:08X}, DLC: {})",
        timestamp,
        can_id_29bit,
        bytes.len()
    );

    let hex_str = hex::encode(bytes);
    let mut offset = 0;

    for chunk in hex_str.chars().collect::<Vec<char>>().chunks(32) {
        let chunk_str: String = chunk.iter().collect();
        let byte_len = chunk_str.len() / 2;

        let ascii: String = bytes[offset..offset + byte_len]
            .iter()
            .map(|&b| {
                if (32..=126).contains(&b) {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();

        println!(
            "{:08x}  {}  |{}|",
            offset,
            chunk_str
                .chars()
                .collect::<Vec<_>>()
                .chunks(8)
                .map(|g| g.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join(" "),
            ascii
        );
        offset += byte_len;
    }
}

/// Classify CAN ID type for unknown messages
fn classify_can_id(can_id: u32) -> &'static str {
    // Standard CAN IDs are 11-bit (0x000 - 0x7FF)
    // Extended CAN IDs are 29-bit (0x800 - 0x1FFFFFFF)
    if can_id > CAN_STD_ID_MAX {
        "Extended Frame (29-bit)"
    } else {
        "Standard Frame (11-bit)"
    }
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

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    // Validate interface name
    if args.interface.is_empty() {
        return Err(anyhow!("CAN interface name cannot be empty"));
    }

    // Create and start monitor
    let mut monitor = CanMonitor::new(args)?;

    // Set up Ctrl+C handler for graceful shutdown
    ctrlc::set_handler(move || {
        println!("\nMonitoring stopped by user");
        std::process::exit(0);
    })
    .map_err(|e| anyhow!("Failed to set Ctrl+C handler: {}", e))?;

    // Start monitoring
    monitor.monitor()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_can_id() {
        // Extended ID (29-bit) - IDs > 0x7FF are extended
        assert_eq!(classify_can_id(0x18FF238A), "Extended Frame (29-bit)");

        // Standard ID (11-bit) - IDs <= 0x7FF are standard
        assert_eq!(classify_can_id(0x123), "Standard Frame (11-bit)");
    }

    #[test]
    fn test_stats_initialization() {
        let stats = MonitoringStats::default();
        assert_eq!(stats.total_messages, 0);
        assert_eq!(stats.decoded_messages, 0);
        assert_eq!(stats.unknown_messages, 0);
    }

    #[test]
    fn test_protocol_enum_display() {
        assert_eq!(Protocol::J1939.as_str(), "J1939");
    }

    #[test]
    fn test_args_parsing_defaults() {
        let args = Args::parse_from(["cando-monitor", "vcan0"]);

        assert_eq!(args.interface, "vcan0");
        assert!(!args.decoded_only);
        assert!(!args.show_raw);
        assert!(!args.quiet);
        assert_eq!(args.stats_interval, 30);
        assert!(args.filter.is_none());
    }
}
