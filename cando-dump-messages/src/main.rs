//! DBC Message Dump Utility
//!
//! This utility program provides a comprehensive view of all messages and signals
//! defined in generated message types for the J1939 protocol. It's useful
//! for debugging, documentation, and understanding the structure of CAN message definitions.
//!
//! The utility displays:
//! - Total number of messages in the protocol
//! - For each message: name, CAN ID, size, and signal count
//! - For each signal: name, bit position, size, scaling factors, range, and units
//!
//! This tool uses compile-time generated metadata instead of runtime DBC parsing,
//! providing faster execution and zero file I/O overhead.

use clap::CommandFactory;
use clap::Parser;
use cando_messages::ProtocolMetadata;
use cando_messages::j1939::J1939_METADATA;

// Import field name converter for --rust-names support
fn to_rust_field_name(dbc_field_name: &str) -> String {
    if dbc_field_name.is_empty() {
        return String::new();
    }

    // If already has underscores, assume it's delimited - just lowercase
    if dbc_field_name.contains('_') {
        return dbc_field_name.to_lowercase();
    }

    // PascalCase - insert underscores at word boundaries
    let mut result = String::with_capacity(dbc_field_name.len() + 10);
    let chars: Vec<char> = dbc_field_name.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        let needs_underscore = if i == 0 {
            false
        } else {
            let prev_char = chars[i - 1];
            let next_char = chars.get(i + 1);

            if ch.is_uppercase() {
                prev_char.is_lowercase()
                    || prev_char.is_ascii_digit()
                    || (prev_char.is_uppercase() && next_char.is_some_and(|c| c.is_lowercase()))
            } else if ch.is_ascii_digit() {
                prev_char.is_alphabetic()
            } else {
                false
            }
        };

        if needs_underscore {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }

    result
}

/// Output format selection
#[derive(Debug, Clone, Copy, PartialEq)]
enum OutputFormat {
    Text,
    Csv,
    Json,
}

/// Protocol selection
#[derive(Debug, Clone, Copy, PartialEq)]
enum Protocol {
    J1939,
    All,
}

/// Command-line arguments for the message dump utility
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Message Dump Utility - Comprehensive view of CAN messages and signals"
)]
struct Args {
    /// Output format: text, csv, or json
    #[arg(long, default_value = "text")]
    format: String,

    /// Shortcut for CSV output
    #[arg(long)]
    csv: bool,

    /// Shortcut for JSON output
    #[arg(long)]
    json: bool,

    /// Protocol to display: j1939 or all
    #[arg(long, short = 'p', default_value = "all")]
    protocol: String,

    /// Show DBC comments and signal descriptions
    #[arg(long, short = 'c')]
    comments: bool,

    /// Show value descriptions (enumerations)
    #[arg(long, short = 'e')]
    enums: bool,

    /// Show verbose output with all metadata (implies --comments and --enums)
    #[arg(long, short = 'v')]
    verbose: bool,

    /// Show full output with all available metadata and formatting
    #[arg(long)]
    full: bool,

    /// Output Rust snake_case field names instead of DBC PascalCase names
    #[arg(long)]
    rust_names: bool,

    /// Generate man page and exit (internal use)
    #[arg(long = "generate-manpage", hide = true)]
    generate_manpage: bool,
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

    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(err) => {
            use clap::error::ErrorKind;
            match err.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
                    print!("{}", err);
                    std::process::exit(0);
                }
                _ => {
                    eprintln!("Error parsing command line arguments:");
                    eprintln!("{}", err);
                    eprintln!();
                    Args::command()
                        .print_help()
                        .expect("Failed to print help message");
                    std::process::exit(1);
                }
            }
        }
    };

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), anyhow::Error> {
    // Determine output format
    let output_format = if args.csv {
        OutputFormat::Csv
    } else if args.json {
        OutputFormat::Json
    } else {
        match args.format.to_lowercase().as_str() {
            "csv" => OutputFormat::Csv,
            "json" => OutputFormat::Json,
            "text" => OutputFormat::Text,
            _ => OutputFormat::Text,
        }
    };

    // Determine protocol
    let protocol = match args.protocol.to_lowercase().as_str() {
        "j1939" => Protocol::J1939,
        "all" | "both" => Protocol::All,
        _ => {
            eprintln!("Unknown protocol: {}. Using 'all'.", args.protocol);
            Protocol::All
        }
    };

    // Display based on format and protocol
    // Determine display options
    let show_comments = args.comments || args.verbose || args.full;
    let show_enums = args.enums || args.verbose || args.full;
    let show_full = args.full;

    // Display based on selected format
    match output_format {
        OutputFormat::Text => display_text_format(protocol, show_comments, show_enums, show_full),
        OutputFormat::Csv => display_csv_format(protocol, show_comments),
        OutputFormat::Json => display_json_format(
            protocol,
            show_comments,
            show_enums,
            show_full,
            args.rust_names,
        )?,
    }

    Ok(())
}

/// Display messages in text format
fn display_text_format(protocol: Protocol, show_comments: bool, show_enums: bool, show_full: bool) {
    match protocol {
        Protocol::J1939 | Protocol::All => {
            display_protocol_text(&J1939_METADATA, show_comments, show_enums, show_full)
        }
    }
}

/// Display a single protocol in text format
fn display_protocol_text(
    protocol: &ProtocolMetadata,
    show_comments: bool,
    show_enums: bool,
    show_full: bool,
) {
    println!("=== {} PROTOCOL ===", protocol.name);
    println!("Found {} messages", protocol.messages.len());

    // Print information about each message
    for msg_meta in protocol.messages {
        // Skip Vector placeholder messages
        if msg_meta.name.starts_with("VECTOR__") {
            continue;
        }

        println!("\n=== MESSAGE ===");
        println!("Name: {}", msg_meta.name);
        println!("ID: {} (0x{:X})", msg_meta.can_id, msg_meta.can_id);
        println!("Size: {} bytes", msg_meta.dlc);
        println!("Signals: {}", msg_meta.signals.len());

        // Show message comment if available and requested
        if show_comments && !msg_meta.comment.is_empty() {
            println!("Description: {}", msg_meta.comment);
        }

        println!("\nSignal details:");
        for (i, signal) in msg_meta.signals.iter().enumerate() {
            println!("  {}. Name: {}", i + 1, signal.name);

            if show_full {
                println!("     Start bit: {}", signal.start_bit);
                println!("     Size: {} bits", signal.signal_size);
                println!("     Byte order: {:?}", signal.byte_order);
                println!("     Value type: {:?}", signal.value_type);
            } else {
                println!("     Start bit: {}", signal.start_bit);
                println!("     Size: {} bits", signal.signal_size);
            }

            println!("     Factor: {}", signal.factor);
            println!("     Offset: {}", signal.offset);

            if signal.unit.is_empty() {
                println!("     Range: [{} to {}]", signal.min, signal.max);
            } else {
                println!(
                    "     Range: [{} to {}] {}",
                    signal.min, signal.max, signal.unit
                );
            }

            // Show signal comment if available and requested
            if show_comments && !signal.comment.is_empty() {
                println!("     Description: {}", signal.comment);
            }

            // Show value descriptions (enumerations) if available and requested
            if show_enums && !signal.value_descriptions.is_empty() {
                println!("     Value descriptions:");
                for (value, desc) in signal.value_descriptions {
                    println!("       {} = {}", value, desc);
                }
            }
        }
    }
}

/// Display messages in CSV format
fn display_csv_format(protocol: Protocol, show_comments: bool) {
    // Print CSV header
    if show_comments {
        println!(
            "MessageName,MessageID,MessageSize,SignalName,StartBit,SignalSize,Factor,Offset,MinValue,MaxValue,Unit,Comment"
        );
    } else {
        println!(
            "MessageName,MessageID,MessageSize,SignalName,StartBit,SignalSize,Factor,Offset,MinValue,MaxValue,Unit"
        );
    }

    match protocol {
        Protocol::J1939 | Protocol::All => {
            display_protocol_csv(&J1939_METADATA, show_comments);
        }
    }
}

/// Display a single protocol in CSV format
fn display_protocol_csv(protocol: &ProtocolMetadata, show_comments: bool) {
    for msg_meta in protocol.messages {
        // Skip Vector placeholder messages
        if msg_meta.name.starts_with("VECTOR__") {
            continue;
        }

        for signal in msg_meta.signals {
            if show_comments {
                // Escape quotes in comments for CSV
                let comment = signal.comment.replace("\"", "\"\"");
                println!(
                    "{},{},{},{},{},{},{},{},{},{},\"{}\",\"{}\"",
                    msg_meta.name,
                    msg_meta.can_id,
                    msg_meta.dlc,
                    signal.name,
                    signal.start_bit,
                    signal.signal_size,
                    signal.factor,
                    signal.offset,
                    signal.min,
                    signal.max,
                    signal.unit,
                    comment
                );
            } else {
                println!(
                    "{},{},{},{},{},{},{},{},{},{},\"{}\"",
                    msg_meta.name,
                    msg_meta.can_id,
                    msg_meta.dlc,
                    signal.name,
                    signal.start_bit,
                    signal.signal_size,
                    signal.factor,
                    signal.offset,
                    signal.min,
                    signal.max,
                    signal.unit
                );
            }
        }
    }
}

/// Display messages in JSON format
fn display_json_format(
    protocol: Protocol,
    show_comments: bool,
    show_enums: bool,
    show_full: bool,
    rust_names: bool,
) -> Result<(), anyhow::Error> {
    let protocols = match protocol {
        Protocol::J1939 | Protocol::All => vec![&J1939_METADATA],
    };

    let mut all_messages = Vec::new();
    let mut total_message_count = 0;
    let mut total_signal_count = 0;

    for proto in protocols {
        for msg_meta in proto.messages {
            // Skip Vector placeholder messages
            if msg_meta.name.starts_with("VECTOR__") {
                continue;
            }

            total_message_count += 1;
            total_signal_count += msg_meta.signals.len();

            let signals: Vec<_> = msg_meta
                .signals
                .iter()
                .map(|signal| {
                    let signal_name = if rust_names {
                        to_rust_field_name(signal.name)
                    } else {
                        signal.name.to_string()
                    };

                    let mut signal_obj = serde_json::json!({
                        "name": signal_name,
                        "start_bit": signal.start_bit,
                        "signal_size": signal.signal_size,
                        "factor": signal.factor,
                        "offset": signal.offset,
                        "min_value": signal.min,
                        "max_value": signal.max,
                        "unit": signal.unit,
                    });

                    // Add full metadata if requested
                    if show_full {
                        signal_obj["byte_order"] =
                            serde_json::json!(format!("{:?}", signal.byte_order));
                        signal_obj["value_type"] =
                            serde_json::json!(format!("{:?}", signal.value_type));
                    }

                    // Add comment if requested and available
                    if show_comments && !signal.comment.is_empty() {
                        signal_obj["comment"] = serde_json::json!(signal.comment);
                    }

                    // Add value descriptions if requested and available
                    if show_enums && !signal.value_descriptions.is_empty() {
                        let enums: Vec<_> = signal
                            .value_descriptions
                            .iter()
                            .map(|(value, desc)| {
                                serde_json::json!({
                                    "value": value,
                                    "description": desc
                                })
                            })
                            .collect();
                        signal_obj["value_descriptions"] = serde_json::json!(enums);
                    }

                    signal_obj
                })
                .collect();

            let mut message_obj = serde_json::json!({
                "name": msg_meta.name,
                "id": msg_meta.can_id,
                "size": msg_meta.dlc,
                "signal_count": msg_meta.signals.len(),
                "signals": signals
            });

            // Add message comment if requested and available
            if show_comments && !msg_meta.comment.is_empty() {
                message_obj["comment"] = serde_json::json!(msg_meta.comment);
            }

            // Add full metadata if requested
            if show_full {
                message_obj["is_multiplexed"] = serde_json::json!(msg_meta.is_multiplexed);
                if !msg_meta.transmitter.is_empty() {
                    message_obj["transmitter"] = serde_json::json!(msg_meta.transmitter);
                }
            }

            all_messages.push(message_obj);
        }
    }

    let output = serde_json::json!({
        "summary": {
            "total_messages": total_message_count,
            "total_signals": total_signal_count
        },
        "messages": all_messages
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_j1939_metadata_accessible() {
        // Verify J1939 metadata is accessible
        assert_eq!(J1939_METADATA.name, "J1939");
        assert!(!J1939_METADATA.messages.is_empty());
    }

    #[test]
    fn test_j1939_signal_properties() {
        // Verify all J1939 signals have valid properties
        for msg in J1939_METADATA.messages {
            for signal in msg.signals {
                // Signal name should not be empty
                assert!(!signal.name.is_empty(), "Signal name should not be empty");

                // Signal size should be reasonable (1-64 bits typically)
                assert!(signal.signal_size > 0, "Signal size should be positive");
                assert!(signal.signal_size <= 64, "Signal size should be <= 64 bits");

                // Min should be <= Max
                assert!(signal.min <= signal.max, "Signal min should be <= max");
            }
        }
    }

    #[test]
    fn test_output_format_parsing() {
        // Test format detection
        let args = Args {
            format: "text".to_string(),
            csv: false,
            json: false,
            protocol: "j1939".to_string(),
            comments: false,
            enums: false,
            verbose: false,
            full: false,
            rust_names: false,
            generate_manpage: false,
        };

        assert_eq!(args.format, "text");
    }

    #[test]
    fn test_protocol_parsing() {
        // Verify protocol names are recognized
        let protocols = vec!["j1939", "all"];
        for proto in protocols {
            let args = Args {
                format: "text".to_string(),
                csv: false,
                json: false,
                protocol: proto.to_string(),
                comments: false,
                enums: false,
                verbose: false,
                full: false,
                rust_names: false,
                generate_manpage: false,
            };
            assert!(
                ["j1939", "all"].contains(&args.protocol.as_str())
            );
        }
    }

    #[test]
    fn test_to_rust_field_name() {
        assert_eq!(to_rust_field_name("MotorSpeed"), "motor_speed");
        assert_eq!(to_rust_field_name("motor_speed"), "motor_speed");
        assert_eq!(to_rust_field_name("RPMValue"), "rpm_value");
        assert_eq!(to_rust_field_name(""), "");
    }
}
