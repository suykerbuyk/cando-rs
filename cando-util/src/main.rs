//! CAN Bus Utility for Message Encoding and Transmission
//!
//! This is the main CLI application for the cando-rs CAN utility. It provides
//! functionality to encode CAN messages with device IDs, display formatted output,
//! and optionally send messages over CAN interfaces.
//!
//! The utility supports J1939-style extended CAN identifiers and is designed for
//! industrial CAN bus applications using the J1939 (SAE J1939 vehicle bus standard) protocol.

use anyhow::{Result, anyhow};
use chrono::Utc;
use clap::Parser;
use std::path::PathBuf;
use std::process::Command;

use cando_config::CandoConfig;
use cando_core::parse_device_id;

use cando_messages::j1939::DM03;
use cando_messages::j1939;
use socketcan::EmbeddedFrame;
use socketcan::Socket;
use socketcan::{CanFrame, CanSocket, ExtendedId};
use std::collections::HashMap;

mod encoder;
use encoder::{EncodedMessage, encode_message};

mod builder;

/// Available specialized commands for the CAN utility
#[derive(Parser, Debug)]
enum Commands {
    /// J1939-73 Diagnostic commands (DM01, DM02, DM03, etc.)
    Diagnostics {
        /// Device ID to embed (hex like 0x8A or decimal like 138)
        /// Device identifier for diagnostic operations
        ///
        /// This ID specifies which device to target for diagnostic operations.
        /// Common J1939 device IDs include:
        /// - 0x82 (130 decimal): Power unit / Engine controller
        /// - 0x88 (136 decimal): Refrigeration unit
        /// - 0x8A (138 decimal): Diagnostic tool / Tester
        ///
        /// The device ID will be embedded in the CAN message identifier
        /// following J1939-73 diagnostic addressing conventions.
        #[arg(
            long,
            help = "Device ID for diagnostic operations (e.g., 0x8A=138 for diagnostic tool)"
        )]
        device_id: String,

        /// Optional: Send the message over this CAN interface
        /// CAN interface for message transmission
        ///
        /// If specified, the encoded diagnostic message will be transmitted over the given
        /// CAN interface. The interface must be available and properly configured.
        /// Common interface names include 'can0', 'can1', 'vcan0' for virtual interfaces.
        #[arg(
            long,
            help = "CAN interface name for message transmission (e.g., 'can0', 'vcan0')"
        )]
        send_interface: Option<String>,

        /// Output format: text, json, or csv
        /// Format for displaying the encoded message
        ///
        /// - text: Human-readable format with diagnostic details
        /// - json: Structured JSON format for programmatic use
        /// - csv: Comma-separated values for data analysis
        #[arg(long, short = 'f', default_value = "text")]
        format: String,

        /// Diagnostic command to send
        #[command(subcommand)]
        command: DiagnosticCommands,
    },

    /// List all devices from configuration
    ListDevices {
        /// Configuration file path (optional, uses search path if not specified)
        #[arg(long, help = "Path to cando.yaml configuration file")]
        config: Option<PathBuf>,

        /// Filter by device type (j1939)
        #[arg(long, help = "Filter devices by type")]
        device_type: Option<String>,

        /// Filter by tag
        #[arg(long, help = "Filter devices by tag")]
        tag: Option<String>,

        /// Show only enabled devices
        #[arg(long, help = "Show only enabled devices")]
        enabled_only: bool,

        /// Show only physical hardware devices
        #[arg(long, help = "Show only physical hardware devices")]
        physical_only: bool,

        /// Show only simulated devices
        #[arg(long, help = "Show only simulated devices")]
        simulated_only: bool,

        /// Output format: text, json, or csv
        #[arg(long, short = 'f', default_value = "text")]
        format: String,
    },

    /// Interactive message builder (TUI) or quick lookup mode
    ///
    /// # Interactive Mode (TUI)
    /// ```bash
    /// cando-util builder
    /// ```
    ///
    /// # Non-Interactive Quick Lookup Mode
    /// ```bash
    /// # List all devices
    /// cando-util builder --list-devices
    ///
    /// # List messages for a device
    /// cando-util builder --device "J1939 ECU" --list-messages
    ///
    /// # Show fields for a specific message
    /// cando-util builder --device "J1939 ECU" --message EEC1 --show-fields
    /// ```
    Builder {
        /// Configuration file path (optional, uses search path if not specified)
        #[arg(long, help = "Path to cando.yaml configuration file")]
        config: Option<PathBuf>,

        /// Environment name to load from configuration
        #[arg(long, help = "Environment name to load")]
        environment: Option<String>,

        /// List all available devices from configuration
        #[arg(long, help = "List all devices (non-interactive mode)")]
        list_devices: bool,

        /// Select a specific device by name
        #[arg(long, help = "Device name for non-interactive lookup")]
        device: Option<String>,

        /// List all messages for the selected device
        #[arg(long, help = "List all messages for device (requires --device)")]
        list_messages: bool,

        /// Select a specific message by name
        #[arg(
            long,
            help = "Message name for non-interactive lookup (requires --device)"
        )]
        message: Option<String>,

        /// Show fields for the selected message
        #[arg(long, help = "Show message fields (requires --device and --message)")]
        show_fields: bool,

        /// Output format for non-interactive mode: text, json, csv
        #[arg(
            long,
            short = 'f',
            default_value = "text",
            help = "Output format (text, json, csv)"
        )]
        format: String,
    },

    /// Show detailed information about a specific device
    ShowDevice {
        /// Device name to display
        #[arg(help = "Name of the device to show")]
        device_name: String,

        /// Configuration file path (optional, uses search path if not specified)
        #[arg(long, help = "Path to cando.yaml configuration file")]
        config: Option<PathBuf>,

        /// Output format: text, json, or csv
        #[arg(long, short = 'f', default_value = "text")]
        format: String,
    },

    /// List all environments from configuration
    ListEnvironments {
        /// Configuration file path (optional, uses search path if not specified)
        #[arg(long, help = "Path to cando.yaml configuration file")]
        config: Option<PathBuf>,

        /// Show only enabled environments
        #[arg(long, help = "Show only enabled environments")]
        enabled_only: bool,

        /// Show only hardware environments
        #[arg(long, help = "Show only hardware environments")]
        hardware_only: bool,

        /// Filter by tag
        #[arg(long, help = "Filter environments by tag")]
        tag: Option<String>,

        /// Output format: text, json, or csv
        #[arg(long, short = 'f', default_value = "text")]
        format: String,
    },
}

/// J1939-73 Diagnostic command types
#[derive(Parser, Debug)]
enum DiagnosticCommands {
    /// DM03 - Diagnostic Data Clear/Reset command
    ClearDtc {
        /// Target device ID for DTC clear operation
        /// Device to send clear command to
        ///
        /// This specifies which device should clear its Diagnostic Trouble Codes (DTCs).
        /// The target device will clear both active (DM01) and previously active (DM02) DTCs.
        /// Common target devices:
        /// - 0x82 (130): Power unit / Engine controller
        /// - 0x88 (136): Refrigeration unit
        /// - 0xFF (255): Global address (all devices)
        #[arg(
            long,
            help = "Target device ID to clear DTCs (e.g., 0x82 for power unit, 0xFF for all)"
        )]
        target_device: String,
    },
}

/// Command-line arguments for the CAN utility
#[derive(Parser, Debug)]
#[command(
    version,
    about = "CAN Message Encoder - Encode and transmit J1939 CAN messages from field assignments"
)]
struct Args {
    /// Configuration file path (optional, uses search path if not specified)
    #[arg(long, global = true, help = "Path to cando.yaml configuration file")]
    config: Option<PathBuf>,

    /// Environment to validate (validates only this environment instead of entire config)
    ///
    /// When specified, only the devices and configuration relevant to this environment
    /// are validated. This avoids validation warnings for unused environments.
    /// If not specified, the entire configuration file is validated.
    #[arg(
        long,
        global = true,
        help = "Environment name for validation (e.g., 'tier2-virtual')"
    )]
    environment: Option<String>,

    /// Device name from configuration (alternative to --device-id)
    #[arg(
        long,
        help = "Device name from cando.yaml",
        conflicts_with = "device_id"
    )]
    device: Option<String>,

    /// Device ID to embed (hex like 0x8A or decimal)
    /// Device identifier to embed in CAN messages
    ///
    /// This ID will be embedded as the source address in the CAN message identifier,
    /// typically in the lower 8 bits for J1939 compatibility. Valid range is 0-255 (0x00-0xFF).
    #[arg(
        long,
        help = "Device ID to embed as source address (0-255, hex like 0x8A or decimal)"
    )]
    device_id: Option<String>,

    /// Message name (e.g., EEC1)
    /// Name of the CAN message to process
    ///
    /// Specifies which message definition to use. The name must match exactly
    /// with a generated message type (case-sensitive). Examples:
    /// - J1939: DM1_DiagnosticMessage, EEC1_ElectronicEngineController1
    #[arg(long, help = "Name of the message to encode")]
    message: Option<String>,

    /// Optional fields as key=value pairs (e.g., "speed=50,status=1")
    /// Signal field assignments for the message
    ///
    /// Specify signal values using comma-separated key=value pairs.
    /// Field names must match the generated signal names exactly.
    /// Values can be integers or floating-point numbers.
    #[arg(long, default_value = "")]
    fields: String,

    /// Optional: Send the message over this CAN interface (e.g., vcan0)
    /// Optional CAN interface for message transmission
    ///
    /// If specified, the encoded message will be sent over the given CAN interface.
    /// The interface must be available and properly configured (e.g., 'can0', 'vcan0').
    /// Use 'ip link show type can' to list available CAN interfaces.
    #[arg(
        long,
        help = "CAN interface name for message transmission (e.g., 'can0', 'vcan0')"
    )]
    send_interface: Option<String>,

    /// Output format: text, json, or csv
    /// Format for displaying the encoded message and signal values
    ///
    /// - text: Human-readable format with signal names, values, and units
    /// - json: Structured JSON format suitable for API integration
    /// - csv: Comma-separated values for spreadsheet analysis
    #[arg(long, short = 'f', default_value = "text")]
    format: String,

    /// Generate a template bash script with all message fields and comments
    ///
    /// Creates a ready-to-edit bash script showing all fields with their ranges,
    /// units, and default values. This helps compose messages without memorizing
    /// field names or looking up valid ranges.
    #[arg(
        long,
        help = "Generate template script with all fields, ranges, and units (use with --device and --message)"
    )]
    template: bool,

    /// Generate man page and exit (internal use)
    #[arg(long = "generate-manpage", hide = true)]
    generate_manpage: bool,

    /// Specialized commands (optional)
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Main entry point for the CAN utility application
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
    match args.command {
        Some(Commands::Diagnostics {
            device_id,
            send_interface,
            format,
            command,
        }) => handle_diagnostic_command(&device_id, &command, send_interface.as_deref(), &format),
        Some(Commands::ListDevices {
            config,
            device_type,
            tag,
            enabled_only,
            physical_only,
            simulated_only,
            format,
        }) => handle_list_devices(
            config.as_deref(),
            args.environment.as_deref(),
            ListDevicesFilters {
                device_type: device_type.clone(),
                tag: tag.clone(),
                enabled_only,
                physical_only,
                simulated_only,
            },
            &format,
        ),
        Some(Commands::ShowDevice {
            device_name,
            config,
            format,
        }) => handle_show_device(
            &device_name,
            config.as_deref(),
            args.environment.as_deref(),
            &format,
        ),
        Some(Commands::ListEnvironments {
            config,
            enabled_only,
            hardware_only,
            tag,
            format,
        }) => handle_list_environments(
            config.as_deref(),
            args.environment.as_deref(),
            enabled_only,
            hardware_only,
            tag.as_deref(),
            &format,
        ),
        Some(Commands::Builder {
            config,
            environment,
            list_devices,
            device,
            list_messages,
            message,
            show_fields,
            format,
        }) => {
            // Load configuration
            let loaded_config = load_config(config.as_deref(), environment.as_deref())?;

            // Check if non-interactive mode is requested
            if list_devices {
                builder::handle_list_devices(&loaded_config, environment.as_deref(), &format)
            } else if list_messages {
                if let Some(device_name) = device {
                    builder::handle_list_messages(&loaded_config, &device_name, &format)
                } else {
                    Err(anyhow!("--list-messages requires --device <name>"))
                }
            } else if show_fields {
                if let Some(device_name) = device {
                    if let Some(message_name) = message {
                        builder::handle_show_fields(
                            &loaded_config,
                            &device_name,
                            &message_name,
                            &format,
                        )
                    } else {
                        Err(anyhow!("--show-fields requires --message <name>"))
                    }
                } else {
                    Err(anyhow!("--show-fields requires --device <name>"))
                }
            } else {
                // Run interactive builder (TUI mode)
                builder::run_builder(
                    loaded_config,
                    config.as_ref().map(|p| p.to_string_lossy().to_string()),
                    environment.clone(),
                )
            }
        }
        None => {
            // Default behavior: handle J1939 messages
            // Resolve device parameters from config if --device is specified
            let (device_id, resolved_interface, _resolved_protocol) =
                if let Some(device_name) = &args.device {
                    let environment = args.environment.as_deref().ok_or_else(|| {
                    anyhow!(
                        "--environment is required when using --device to specify device context"
                    )
                })?;
                    let config = load_config(args.config.as_deref(), Some(environment))?;
                    let resolved = resolve_device_params(&config, device_name, environment)?;
                    (
                        resolved.device_id,
                        Some(resolved.interface),
                        Some(resolved.protocol),
                    )
                } else {
                    let device_id = args.device_id.ok_or_else(|| {
                        anyhow!("Either --device or --device-id is required for message encoding")
                    })?;
                    (device_id, None, None)
                };

            let message = args
                .message
                .ok_or_else(|| anyhow!("--message is required for message encoding"))?;

            // Use CLI interface if specified, otherwise use resolved from config
            let interface = args.send_interface.or(resolved_interface);

            // If --template is specified, generate template and exit
            if args.template {
                return generate_template(
                    &device_id,
                    &message,
                    interface.as_deref(),
                    args.device.as_deref(),
                );
            }

            handle_default_command(
                &device_id,
                &message,
                &args.fields,
                interface.as_deref(),
                &args.format,
            )
        }
    }
}

fn handle_diagnostic_command(
    device_id_str: &str,
    command: &DiagnosticCommands,
    send_interface: Option<&str>,
    format: &str,
) -> Result<()> {
    // Parse device ID (this will be the source device - the diagnostic tool)
    let _device_id = parse_device_id(device_id_str)?;

    // Handle different diagnostic commands
    match command {
        DiagnosticCommands::ClearDtc { target_device } => {
            // Parse target device ID
            let target_device_id = parse_device_id(target_device)?;

            // Create DM03 message
            let dm03_msg = DM03 {
                device_id: target_device_id,
            };

            // Encode the message
            let (can_id, data) = dm03_msg
                .encode()
                .map_err(|e| anyhow!("Failed to encode DM03 message: {:?}", e))?;

            // Create encoded message structure for display
            let encoded = EncodedMessage {
                message_name: "DM03_Clear_DTCs".to_string(),
                protocol: "J1939-73".to_string(),
                can_id,
                data: data.to_vec(),
            };

            // Display the message
            display_diagnostic_message(&encoded, &dm03_msg, format)?;

            // Send message if interface specified
            if let Some(interface_name) = send_interface {
                send_can_frame(interface_name, can_id, &data)?;
                println!(
                    "DM03 clear command sent to device 0x{:02X} on {}",
                    target_device_id.as_u8(),
                    interface_name
                );
            }

            Ok(())
        }
    }
}

fn display_diagnostic_message(encoded: &EncodedMessage, dm03: &DM03, format: &str) -> Result<()> {
    match format {
        "text" => display_diagnostic_text(encoded, dm03),
        "json" => display_diagnostic_json(encoded, dm03),
        "csv" => display_diagnostic_csv(encoded, dm03),
        _ => Err(anyhow!("Unknown output format: {}", format)),
    }
}

fn display_diagnostic_text(encoded: &EncodedMessage, dm03: &DM03) -> Result<()> {
    println!(
        "Message: {} | ID: 0x{:08X} | DLC: {} | Target Device: 0x{:02X}",
        encoded.message_name,
        encoded.can_id,
        encoded.data.len(),
        dm03.device_id.as_u8()
    );
    println!("  Command: Clear Diagnostic Trouble Codes (DTCs)");
    println!("  Protocol: J1939-73 Diagnostic Messages");
    println!("  PGN: 0xFECC (DM03 - Diagnostic Data Clear/Reset)");
    println!(
        "  Target: Device 0x{:02X} will clear active and previously active DTCs",
        dm03.device_id.as_u8()
    );
    println!("  Expected Response: Updated DM01 (active DTCs) and DM02 (previously active DTCs)");
    println!(
        "  Data: {} bytes (command-only message)",
        encoded.data.len()
    );
    Ok(())
}

fn display_diagnostic_json(encoded: &EncodedMessage, dm03: &DM03) -> Result<()> {
    let json_data = serde_json::json!({
        "message_name": encoded.message_name,
        "protocol": encoded.protocol,
        "can_id": format!("0x{:08X}", encoded.can_id),
        "pgn": "0xFECC",
        "dlc": encoded.data.len(),
        "target_device_id": format!("0x{:02X}", dm03.device_id.as_u8()),
        "command_type": "clear_dtcs",
        "description": "Clear active and previously active Diagnostic Trouble Codes",
        "expected_responses": ["DM01", "DM02"],
        "data_hex": encoded.data.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join("")
    });
    println!("{}", serde_json::to_string_pretty(&json_data)?);
    Ok(())
}

fn display_diagnostic_csv(encoded: &EncodedMessage, dm03: &DM03) -> Result<()> {
    println!("message_name,protocol,can_id,dlc,field_name,field_value");
    println!(
        "{},{},0x{:08X},{},target_device_id,0x{:02X}",
        encoded.message_name,
        encoded.protocol,
        encoded.can_id,
        encoded.data.len(),
        dm03.device_id.as_u8()
    );
    println!(
        "{},{},0x{:08X},{},command_type,clear_dtcs",
        encoded.message_name,
        encoded.protocol,
        encoded.can_id,
        encoded.data.len()
    );
    Ok(())
}

fn handle_default_command(
    device_id_str: &str,
    message: &str,
    fields_str: &str,
    send_interface: Option<&str>,
    format: &str,
) -> Result<()> {
    // Parse device ID
    let device_id = parse_device_id(device_id_str)?;

    // Parse field assignments
    let field_map = parse_fields(fields_str)?;

    // Encode message using generated types
    let encoded = encode_message(message, device_id, &field_map)?;

    // Display the encoded message
    display_message(&encoded, &field_map, format)?;

    // Display hex dump
    let timestamp = Utc::now().to_rfc3339();
    print_hex_dump(encoded.can_id, &encoded.data, &timestamp);

    // Optionally send over CAN
    if let Some(interface) = send_interface {
        send_can_frame(interface, encoded.can_id, &encoded.data)?;
    }

    Ok(())
}

/// Parse field assignments from comma-separated key=value pairs
fn parse_fields(fields_str: &str) -> Result<HashMap<String, f64>> {
    let mut map = HashMap::new();

    for pair in fields_str.split(',').filter(|s| !s.is_empty()) {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid field assignment: '{}'", pair));
        }

        let value = parts[1]
            .parse::<f64>()
            .map_err(|e| anyhow!("Invalid value '{}': {}", parts[1], e))?;

        map.insert(parts[0].to_string(), value);
    }

    Ok(map)
}

/// Display the encoded message with signal values
fn display_message(
    encoded: &EncodedMessage,
    field_map: &HashMap<String, f64>,
    format: &str,
) -> Result<()> {
    match format {
        "text" => display_text(encoded, field_map),
        "json" => display_json(encoded, field_map),
        "csv" => display_csv(encoded, field_map),
        _ => Err(anyhow!("Unknown output format: {}", format)),
    }
}

/// Display message in text format
fn display_text(encoded: &EncodedMessage, field_map: &HashMap<String, f64>) -> Result<()> {
    // Get metadata for the message
    let metadata = get_message_metadata(&encoded.message_name, &encoded.protocol)?;

    println!(
        "Message: {} | ID: 0x{:08X} | DLC: {}",
        metadata.name, encoded.can_id, metadata.dlc
    );

    // Display signal values
    for (field_name, value) in field_map {
        // Find the signal metadata
        let normalized = field_name.to_lowercase();
        if let Some(signal) = metadata
            .signals
            .iter()
            .find(|s| s.name.to_lowercase() == normalized)
        {
            // Check if this signal has value descriptions
            if !signal.value_descriptions.is_empty() {
                if let Some(desc) = signal.get_value_description(*value as u64) {
                    println!("  {}: {}", signal.name, desc);
                } else {
                    println!("  {}: {} {}", signal.name, value, signal.unit);
                }
            } else {
                println!("  {}: {} {}", signal.name, value, signal.unit);
            }
        } else {
            println!("  {}: {}", field_name, value);
        }
    }

    Ok(())
}

/// Display message in JSON format
fn display_json(encoded: &EncodedMessage, field_map: &HashMap<String, f64>) -> Result<()> {
    println!("{{");
    println!("  \"message\": \"{}\",", encoded.message_name);
    println!("  \"protocol\": \"{}\",", encoded.protocol);
    println!("  \"can_id\": \"0x{:08X}\",", encoded.can_id);
    println!("  \"dlc\": {},", encoded.data.len());
    println!("  \"signals\": {{");

    let entries: Vec<_> = field_map.iter().collect();
    for (i, (field_name, value)) in entries.iter().enumerate() {
        let comma = if i < entries.len() - 1 { "," } else { "" };
        println!("    \"{}\": {}{}", field_name, value, comma);
    }

    println!("  }}");
    println!("}}");
    Ok(())
}

/// Display message in CSV format
fn display_csv(encoded: &EncodedMessage, field_map: &HashMap<String, f64>) -> Result<()> {
    // Header
    println!("message,protocol,can_id,dlc,field,value");

    // Data rows
    for (field_name, value) in field_map {
        println!(
            "{},{},0x{:08X},{},{},{}",
            encoded.message_name,
            encoded.protocol,
            encoded.can_id,
            encoded.data.len(),
            field_name,
            value
        );
    }

    Ok(())
}

/// Get message metadata by name and protocol
fn get_message_metadata(
    message_name: &str,
    protocol: &str,
) -> Result<&'static cando_messages::metadata::MessageMetadata> {
    match protocol {
        "J1939" => j1939::J1939_METADATA
            .find_message(message_name)
            .ok_or_else(|| anyhow!("Message '{}' not found in J1939 protocol", message_name)),
        _ => Err(anyhow!(
            "Unsupported protocol: '{}' (supported: J1939)",
            protocol
        )),
    }
}

/// Print a hex dump of the CAN frame data
fn print_hex_dump(can_id: u32, data: &[u8], timestamp: &str) {
    println!(
        "Timestamp: {} | CAN Frame (Extended ID: 0x{:08X}, DLC: {})",
        timestamp,
        can_id,
        data.len()
    );

    // Print hex dump in rows of 8 bytes
    for (i, chunk) in data.chunks(8).enumerate() {
        print!("{:08x}  ", i * 8);

        // Print hex values
        for byte in chunk {
            print!("{:02x}", byte);
        }

        // Pad if less than 8 bytes
        for _ in chunk.len()..8 {
            print!("  ");
        }

        print!("  |");

        // Print ASCII representation
        for byte in chunk {
            let c = if byte.is_ascii_graphic() || *byte == b' ' {
                *byte as char
            } else {
                '.'
            };
            print!("{}", c);
        }

        // Pad ASCII if less than 8 bytes
        for _ in chunk.len()..8 {
            print!(" ");
        }

        println!("|");
    }
}

/// Send a CAN frame over the specified interface
fn send_can_frame(interface: &str, can_id: u32, data: &[u8]) -> Result<()> {
    // Open the CAN socket interface
    let socket = CanSocket::open(interface)
        .map_err(|e| anyhow!("Failed to open CAN interface '{}': {}", interface, e))?;

    // Create the CAN frame with extended ID
    let id = ExtendedId::new(can_id & 0x1FFFFFFF)
        .ok_or(anyhow!("Invalid CAN extended ID: 0x{:X}", can_id))?;
    let frame = CanFrame::new(id, data).ok_or(anyhow!("Failed to create CAN frame"))?;

    // Transmit the frame
    socket
        .write_frame(&frame)
        .map_err(|e| anyhow!("Failed to transmit CAN frame on '{}': {}", interface, e))?;

    println!(
        "Message transmitted successfully on interface: {}",
        interface
    );
    println!("  CAN ID: 0x{:08X}", can_id);
    println!("  Data Length: {} bytes", data.len());

    Ok(())
}

/// Load configuration from file or search path
fn load_config(
    config_path: Option<&std::path::Path>,
    environment: Option<&str>,
) -> Result<CandoConfig> {
    if let Some(path) = config_path {
        // Explicit path provided
        if let Some(env_name) = environment {
            CandoConfig::load_from_file_for_environment(path, env_name).map_err(|e| {
                anyhow!(
                    "Failed to load configuration from {} for environment '{}': {}",
                    path.display(),
                    env_name,
                    e
                )
            })
        } else {
            CandoConfig::load_from_file(path).map_err(|e| {
                anyhow!(
                    "Failed to load configuration from {}: {}",
                    path.display(),
                    e
                )
            })
        }
    } else {
        // Use search path with environment-aware validation
        let default_paths = vec![
            std::path::PathBuf::from("./cando-test.yaml"),
            std::path::PathBuf::from("./cando.yaml"),
        ];

        // Try environment variable first
        if let Ok(env_path) = std::env::var("CANDO_CONFIG") {
            let path = std::path::PathBuf::from(env_path);
            if path.exists() {
                return if let Some(env_name) = environment {
                    CandoConfig::load_from_file_for_environment(&path, env_name).map_err(|e| {
                        anyhow!(
                            "Failed to load configuration from {}: {}",
                            path.display(),
                            e
                        )
                    })
                } else {
                    CandoConfig::load_from_file(&path).map_err(|e| {
                        anyhow!(
                            "Failed to load configuration from {}: {}",
                            path.display(),
                            e
                        )
                    })
                };
            }
        }

        // Try default paths
        for path in &default_paths {
            if path.exists() {
                return if let Some(env_name) = environment {
                    CandoConfig::load_from_file_for_environment(path, env_name).map_err(|e| {
                        anyhow!(
                            "Failed to load configuration from {}: {}",
                            path.display(),
                            e
                        )
                    })
                } else {
                    CandoConfig::load_from_file(path).map_err(|e| {
                        anyhow!(
                            "Failed to load configuration from {}: {}",
                            path.display(),
                            e
                        )
                    })
                };
            }
        }

        // Fall back to defaults if no config file found
        Ok(CandoConfig::default())
    }
}

/// Resolved device parameters from configuration
#[derive(Debug)]
struct ResolvedDeviceParams {
    device_id: String,
    interface: String,
    protocol: String,
    #[allow(dead_code)]
    websocket_port: u16,
}

/// Resolve device parameters from configuration by device name
fn resolve_device_params(
    config: &CandoConfig,
    device_name: &str,
    environment: &str,
) -> Result<ResolvedDeviceParams> {
    // Search only within the specified environment for the device
    let device_tuple = config
        .enabled_devices_for_environment(environment)
        .into_iter()
        .find(|(_, device_key, _)| *device_key == device_name)
        .ok_or_else(|| {
            anyhow!(
                "Device '{}' not found in environment '{}'. Use 'cando-util list-devices --environment {}' to see available devices.",
                device_name,
                environment,
                environment
            )
        })?;

    let (_env_name, _device_key, device) = device_tuple;

    Ok(ResolvedDeviceParams {
        device_id: device.device_id.clone(),
        interface: device
            .interface
            .clone()
            .unwrap_or_else(|| "can0".to_string()),
        protocol: device
            .protocol
            .clone()
            .unwrap_or_else(|| "j1939".to_string()),
        websocket_port: device.websocket_port.unwrap_or(10750),
    })
}

/// Filters for list-devices command
struct ListDevicesFilters {
    device_type: Option<String>,
    tag: Option<String>,
    enabled_only: bool,
    physical_only: bool,
    simulated_only: bool,
}

/// Handle list-devices command
fn handle_list_devices(
    config_path: Option<&std::path::Path>,
    environment: Option<&str>,
    filters: ListDevicesFilters,
    format: &str,
) -> Result<()> {
    let config = load_config(config_path, environment)?;

    // Start with all devices (returns Vec<(&str, &str, &DeviceConfig)>)
    let mut devices = config.all_devices();

    // Apply filters
    if filters.enabled_only {
        devices.retain(|(_, _, d)| d.enabled);
    }

    if filters.physical_only {
        devices.retain(|(_, _, d)| d.hardware_present);
    }

    if filters.simulated_only {
        devices.retain(|(_, _, d)| !d.hardware_present);
    }

    if let Some(ref dt) = filters.device_type {
        let dt_lower = dt.to_lowercase();
        devices.retain(|(_, _, d)| format!("{:?}", d.device_type).to_lowercase() == dt_lower);
    }

    if let Some(ref tag_filter) = filters.tag {
        devices.retain(|(_, _, d)| d.tags.contains(&tag_filter.to_string()));
    }

    match format {
        "json" => {
            let json_devices: Vec<_> = devices
                .iter()
                .map(|(env_name, device_key, d)| {
                    serde_json::json!({
                        "environment": env_name,
                        "name": device_key,
                        "friendly_name": d.friendly_name,
                        "type": format!("{:?}", d.device_type).to_lowercase(),
                        "device_id": d.device_id,
                        "interface": d.interface,
                        "protocol": d.protocol,
                        "websocket_port": d.websocket_port,
                        "enabled": d.enabled,
                        "hardware_present": d.hardware_present,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json_devices)?);
        }
        "csv" => {
            println!(
                "environment,name,friendly_name,type,device_id,interface,protocol,websocket_port,enabled,hardware_present"
            );
            for (env_name, device_key, device) in devices {
                println!(
                    "{},{},{},{},{},{},{},{},{},{}",
                    env_name,
                    device_key,
                    device.friendly_name.as_deref().unwrap_or(""),
                    format!("{:?}", device.device_type).to_lowercase(),
                    device.device_id,
                    device.interface.as_deref().unwrap_or(""),
                    device.protocol.as_deref().unwrap_or(""),
                    device
                        .websocket_port
                        .map(|p| p.to_string())
                        .unwrap_or_default(),
                    device.enabled,
                    device.hardware_present
                );
            }
        }
        _ => {
            // text format
            if devices.is_empty() {
                println!("No devices found matching criteria");
                return Ok(());
            }

            println!("Found {} device(s):\n", devices.len());
            for (env_name, device_key, device) in devices {
                println!("  Environment: {}", env_name);
                println!("  Name: {}", device_key);
                println!(
                    "  Friendly Name: {}",
                    device.friendly_name.as_deref().unwrap_or("N/A")
                );
                println!("  Type: {:?}", device.device_type);
                println!("  Device ID: {}", device.device_id);
                println!(
                    "  Interface: {}",
                    device.interface.as_deref().unwrap_or("N/A")
                );
                println!(
                    "  Protocol: {}",
                    device.protocol.as_deref().unwrap_or("N/A")
                );
                if let Some(port) = device.websocket_port {
                    println!("  WebSocket Port: {}", port);
                }
                println!("  Enabled: {}", device.enabled);
                println!(
                    "  Hardware: {}",
                    if device.hardware_present {
                        "Physical"
                    } else {
                        "Simulated"
                    }
                );
                if let Some(desc) = &device.description {
                    println!("  Description: {}", desc);
                }
                println!();
            }
        }
    }

    Ok(())
}

/// Handle show-device command
fn handle_show_device(
    device_name: &str,
    config_path: Option<&std::path::Path>,
    environment: Option<&str>,
    format: &str,
) -> Result<()> {
    let config = load_config(config_path, environment)?;

    // Search across all environments for a device with this key
    let device_tuple = config
        .all_devices()
        .into_iter()
        .find(|(_, device_key, _)| *device_key == device_name)
        .ok_or_else(|| anyhow!("Device '{}' not found in configuration", device_name))?;

    let (env_name, device_key, device) = device_tuple;

    match format {
        "json" => {
            let json_device = serde_json::json!({
                "environment": env_name,
                "name": device_key,
                "friendly_name": device.friendly_name,
                "type": format!("{:?}", device.device_type).to_lowercase(),
                "device_id": device.device_id,
                "interface": device.interface,
                "protocol": device.protocol,
                "websocket_port": device.websocket_port,
                "enabled": device.enabled,
                "hardware_present": device.hardware_present,
                "description": device.description,
                "variant": device.variant,
                "serial_number": device.serial_number,
                "firmware_version": device.firmware_version,
                "location": device.location,
                "validation_status": device.validation_status,
                "last_validated": device.last_validated,
                "owner": device.owner,
            });
            println!("{}", serde_json::to_string_pretty(&json_device)?);
        }
        "csv" => {
            println!("field,value");
            println!("environment,{}", env_name);
            println!("name,{}", device_key);
            if let Some(ref fname) = device.friendly_name {
                println!("friendly_name,{}", fname);
            }
            println!(
                "type,{}",
                format!("{:?}", device.device_type).to_lowercase()
            );
            println!("device_id,{}", device.device_id);
            if let Some(ref iface) = device.interface {
                println!("interface,{}", iface);
            }
            if let Some(ref proto) = device.protocol {
                println!("protocol,{}", proto);
            }
            if let Some(port) = device.websocket_port {
                println!("websocket_port,{}", port);
            }
            println!("enabled,{}", device.enabled);
            println!("hardware_present,{}", device.hardware_present);
            if let Some(desc) = &device.description {
                println!("description,{}", desc);
            }
            if let Some(variant) = &device.variant {
                println!("variant,{}", variant);
            }
            if let Some(serial) = &device.serial_number {
                println!("serial_number,{}", serial);
            }
            if let Some(firmware) = &device.firmware_version {
                println!("firmware_version,{}", firmware);
            }
            if let Some(location) = &device.location {
                println!("location,{}", location);
            }
            if let Some(status) = &device.validation_status {
                println!("validation_status,{:?}", status);
            }
            if let Some(validated) = &device.last_validated {
                println!("last_validated,{}", validated);
            }
            if let Some(owner) = &device.owner {
                println!("owner,{}", owner);
            }
        }
        _ => {
            // text format
            println!("Environment: {}", env_name);
            println!("Device: {}", device_key);
            println!(
                "  Friendly Name: {}",
                device.friendly_name.as_deref().unwrap_or("N/A")
            );
            println!("  Type: {:?}", device.device_type);
            println!("  Device ID: {}", device.device_id);
            println!(
                "  Interface: {}",
                device.interface.as_deref().unwrap_or("N/A")
            );
            println!(
                "  Protocol: {}",
                device.protocol.as_deref().unwrap_or("N/A")
            );
            if let Some(port) = device.websocket_port {
                println!("  WebSocket Port: {}", port);
            }
            println!("  Enabled: {}", device.enabled);
            println!(
                "  Hardware: {}",
                if device.hardware_present {
                    "Physical"
                } else {
                    "Simulated"
                }
            );

            if let Some(desc) = &device.description {
                println!("  Description: {}", desc);
            }
            if let Some(variant) = &device.variant {
                println!("  Variant: {}", variant);
            }
            if let Some(serial) = &device.serial_number {
                println!("  Serial Number: {}", serial);
            }
            if let Some(firmware) = &device.firmware_version {
                println!("  Firmware: {}", firmware);
            }
            if let Some(location) = &device.location {
                println!("  Location: {}", location);
            }
            if let Some(status) = &device.validation_status {
                println!("  Validation Status: {:?}", status);
            }
            if let Some(validated) = &device.last_validated {
                println!("  Last Validated: {}", validated);
            }
            if let Some(owner) = &device.owner {
                println!("  Owner: {}", owner);
            }
        }
    }

    Ok(())
}

/// Handle list-environments command
fn handle_list_environments(
    config_path: Option<&std::path::Path>,
    environment: Option<&str>,
    enabled_only: bool,
    hardware_only: bool,
    tag: Option<&str>,
    format: &str,
) -> Result<()> {
    let config = load_config(config_path, environment)?;

    // Start with all environments (HashMap returns tuples of (name, environment))
    let mut environments: Vec<_> = config.environments.iter().collect();

    // Apply filters
    if enabled_only {
        environments.retain(|(_, e)| e.enabled);
    }

    if hardware_only {
        // Filter to environments where all devices are physical hardware
        environments.retain(|(env_name, _)| {
            let env_devices = config.environment_devices(env_name);
            !env_devices.is_empty() && env_devices.iter().all(|(_, d)| d.hardware_present)
        });
    }

    if let Some(tag_filter) = tag {
        environments.retain(|(_, e)| e.tags.contains(&tag_filter.to_string()));
    }

    match format {
        "json" => {
            let json_envs: Vec<_> = environments
                .iter()
                .map(|(name, e)| {
                    serde_json::json!({
                        "name": name,
                        "friendly_name": e.friendly_name,
                        "devices": e.devices.keys().collect::<Vec<_>>(),
                        "can_interface": e.can_interface,
                        "enabled": e.enabled,
                        "description": e.description,
                        "location": e.location,
                        "tags": e.tags,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json_envs)?);
        }
        "csv" => {
            println!("name,friendly_name,devices,can_interface,enabled,location");
            for (name, env) in environments {
                let device_keys: Vec<_> = env.devices.keys().map(|k| k.as_str()).collect();
                println!(
                    "{},{},{},{},{},{}",
                    name,
                    env.friendly_name.as_deref().unwrap_or(""),
                    device_keys.join(";"),
                    env.can_interface.as_deref().unwrap_or(""),
                    env.enabled,
                    env.location.as_deref().unwrap_or("")
                );
            }
        }
        _ => {
            // text format
            if environments.is_empty() {
                println!("No environments found matching criteria");
                return Ok(());
            }

            println!("Found {} environment(s):\n", environments.len());
            for (name, env) in environments {
                println!("  Name: {}", name);
                println!(
                    "  Friendly Name: {}",
                    env.friendly_name.as_deref().unwrap_or("N/A")
                );
                let device_keys: Vec<_> = env.devices.keys().map(|k| k.as_str()).collect();
                println!("  Devices: {}", device_keys.join(", "));
                if let Some(interface) = &env.can_interface {
                    println!("  CAN Interface: {}", interface);
                }

                println!("  Enabled: {}", env.enabled);
                if let Some(desc) = &env.description {
                    println!("  Description: {}", desc);
                }
                if let Some(location) = &env.location {
                    println!("  Location: {}", location);
                }
                if !env.tags.is_empty() {
                    println!("  Tags: {}", env.tags.join(", "));
                }
                println!();
            }
        }
    }

    Ok(())
}

/// Generate a template bash script for a message with all fields and metadata
fn generate_template(
    device_id: &str,
    message: &str,
    interface: Option<&str>,
    device_name: Option<&str>,
) -> Result<()> {
    // Get message metadata from dump-messages
    let metadata = get_template_metadata(message)?;

    // Generate template header
    println!("#!/bin/bash");
    println!("# Template for {} message", message);
    println!("# Generated by cando-util --template");
    println!("#");
    println!("# Message: {}", metadata.name);
    println!("# CAN ID: 0x{:08X}", metadata.id);
    println!("# Size: {} bytes", metadata.size);
    println!("#");
    println!("# INSTRUCTIONS:");
    println!(
        "# 1. Copy this template to a file (e.g., test_{}.sh)",
        message.to_lowercase()
    );
    println!(
        "# 2. Make it executable: chmod +x test_{}.sh",
        message.to_lowercase()
    );
    println!("# 3. Edit the field values below");
    println!("# 4. Run the script to send the message");
    println!();

    // Build the command
    println!("cando-util \\");

    // Add device parameter
    if let Some(dev_name) = device_name {
        println!("  --device \"{}\" \\", dev_name);
    } else {
        println!("  --device-id {} \\", device_id);
    }

    println!("  --message {} \\", message);
    println!("  --fields \"\\");

    // Add all fields with metadata comments
    for (idx, signal) in metadata.signals.iter().enumerate() {
        let is_last = idx == metadata.signals.len() - 1;
        let continuation = if is_last { "" } else { "," };

        // Convert signal name to lowercase for field name (matches encoder's normalize_field_name)
        let field_name = signal.name.to_lowercase();

        // Generate default value
        // For reserved fields that "must be set to 1", calculate all-ones value
        let default_value = if signal.name.to_lowercase().contains("reserved") {
            // Calculate all-ones value based on signal range
            let max_as_int = signal.max_value as u64;
            format!("{}", max_as_int)
        } else if signal.min_value >= 0.0 {
            format!("{}", signal.min_value)
        } else {
            "0".to_string()
        };

        // Print field line with inline comment
        print!("{}={}{}  ", field_name, default_value, continuation);

        // Print metadata comment
        print!("# Range: {}-{}", signal.min_value, signal.max_value);
        if !signal.unit.is_empty() {
            print!(" {}", signal.unit);
        }
        if signal.factor != 1.0 {
            print!(", Factor: {}", signal.factor);
        }
        // Mark reserved fields
        if signal.name.to_lowercase().contains("reserved") {
            print!(" [AUTO: Protocol reserved bits]");
        }
        println!(" ({})", signal.name);
    }

    println!("\" \\");

    // Add interface if specified
    if let Some(iface) = interface {
        println!("  --send-interface {}", iface);
    } else {
        println!("  # --send-interface vcan0  # Uncomment to send message");
    }

    println!();
    println!("# Example usage:");
    println!("# ./test_{}.sh", message.to_lowercase());
    println!("#");
    println!("# To see the encoded message without sending:");
    println!("# Remove or comment out the --send-interface line");

    Ok(())
}

/// Signal metadata for template generation
#[derive(Debug, serde::Deserialize)]
struct SignalMetadata {
    name: String,
    min_value: f64,
    max_value: f64,
    unit: String,
    factor: f64,
}

/// Message metadata for template generation
#[derive(Debug, serde::Deserialize)]
struct MessageMetadata {
    id: u32,
    name: String,
    size: u8,
    signals: Vec<SignalMetadata>,
}

/// Messages wrapper for JSON response
#[derive(Debug, serde::Deserialize)]
struct DumpMessagesResponse {
    messages: Vec<MessageMetadata>,
}

/// Get message metadata by calling dump-messages
fn get_template_metadata(message_name: &str) -> Result<MessageMetadata> {
    // Try to find dump-messages in the same directory or PATH
    let dump_messages = find_dump_messages_binary()?;

    // Run dump-messages --json --protocol all
    let output = Command::new(dump_messages)
        .args(["--json", "--protocol", "all"])
        .output()
        .map_err(|e| anyhow!("Failed to execute dump-messages: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("dump-messages failed: {}", stderr));
    }

    // Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let response: DumpMessagesResponse = serde_json::from_str(&stdout)
        .map_err(|e| anyhow!("Failed to parse dump-messages output: {}", e))?;

    // Find the requested message
    response
        .messages
        .into_iter()
        .find(|m| m.name == message_name)
        .ok_or_else(|| {
            anyhow!(
                "Message '{}' not found in dump-messages output",
                message_name
            )
        })
}

/// Find dump-messages binary in same directory as cando-util or in PATH
fn find_dump_messages_binary() -> Result<String> {
    // Try same directory as current executable first
    if let Ok(exe_path) = std::env::current_exe()
        && let Some(exe_dir) = exe_path.parent()
    {
        let dump_messages_path = exe_dir.join("cando-dump-messages");
        if dump_messages_path.exists() {
            return Ok(dump_messages_path.to_string_lossy().to_string());
        }
        // Also try the old name
        let dump_messages_path = exe_dir.join("dump-messages");
        if dump_messages_path.exists() {
            return Ok(dump_messages_path.to_string_lossy().to_string());
        }
    }

    // Fall back to PATH
    Ok("cando-dump-messages".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id_parsing_decimal() {
        let result = parse_device_id("138");
        assert!(result.is_ok());
        assert_eq!(u8::from(result.unwrap()), 138);
    }

    #[test]
    fn test_device_id_parsing_hex() {
        let result = parse_device_id("0x8A");
        assert!(result.is_ok());
        assert_eq!(u8::from(result.unwrap()), 0x8A);
    }

    #[test]
    fn test_device_id_out_of_range() {
        let result = parse_device_id("300");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("between 0 and 255")
        );
    }

    #[test]
    fn test_parse_fields_valid() {
        let result = parse_fields("field1=10.5,field2=20.0");
        assert!(result.is_ok());

        let map = result.unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("field1"), Some(&10.5));
        assert_eq!(map.get("field2"), Some(&20.0));
    }

    #[test]
    fn test_parse_fields_empty() {
        let result = parse_fields("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_fields_invalid() {
        let result = parse_fields("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_fields_invalid_value() {
        let result = parse_fields("field=notanumber");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_id_masking() {
        let can_id: u32 = 0x98EFFE8A;
        let masked = can_id & 0x1FFFFFFF;
        assert_eq!(masked, 0x18EFFE8A);
        assert!(ExtendedId::new(masked).is_some());
    }

    #[test]
    fn test_invalid_id_error() {
        let invalid_id: u32 = 0x98EFFE8A;
        assert!(ExtendedId::new(invalid_id).is_none());
        assert!(ExtendedId::new(invalid_id & 0x1FFFFFFF).is_some());
    }

    // ===== Configuration Integration Tests =====

    /// Helper function to create a test configuration
    #[cfg(test)]
    fn create_test_config() -> CandoConfig {
        use cando_config::{Defaults, DeviceConfig, DeviceType, Environment, WebUiConfig};
        use std::collections::HashMap;

        let mut environments = HashMap::new();

        // Test environment with simulated devices
        let mut test_env_devices = HashMap::new();
        test_env_devices.insert(
            "ecu-a".to_string(),
            DeviceConfig {
                friendly_name: Some("ECU A".to_string()),
                device_type: DeviceType::J1939,
                device_id: "0x82".to_string(),
                interface: Some("vcan0".to_string()),
                protocol: Some("j1939".to_string()),
                websocket_port: Some(10754),
                enabled: true,
                description: Some("Test ECU".to_string()),
                variant: Some("ecu".to_string()),
                debug: None,
                no_console: None,
                hardware_present: false,
                serial_number: None,
                firmware_version: None,
                manufacture_date: None,
                location: None,
                validation_status: None,
                last_validated: None,
                validation_notes: None,
                tags: Vec::new(),
                owner: None,
                notes: None,
                voltage_specification: "12VDC".to_string(),
            },
        );
        test_env_devices.insert(
            "ecu-b".to_string(),
            DeviceConfig {
                friendly_name: Some("ECU B".to_string()),
                device_type: DeviceType::J1939,
                device_id: "0x88".to_string(),
                interface: Some("vcan0".to_string()),
                protocol: Some("j1939".to_string()),
                websocket_port: Some(10755),
                enabled: true,
                description: Some("Test ECU B".to_string()),
                variant: Some("ecu".to_string()),
                debug: None,
                no_console: None,
                hardware_present: false,
                serial_number: Some("ECU-001".to_string()),
                firmware_version: Some("1.2.3".to_string()),
                manufacture_date: None,
                location: Some("Lab".to_string()),
                validation_status: None,
                last_validated: None,
                validation_notes: None,
                tags: Vec::new(),
                owner: Some("Test Team".to_string()),
                notes: None,
                voltage_specification: "12VDC".to_string(),
            },
        );
        test_env_devices.insert(
            "disabled-device".to_string(),
            DeviceConfig {
                friendly_name: Some("Disabled Device".to_string()),
                device_type: DeviceType::J1939,
                device_id: "0x90".to_string(),
                interface: Some("vcan0".to_string()),
                protocol: Some("j1939".to_string()),
                websocket_port: Some(10758),
                enabled: false,
                description: Some("Disabled for testing".to_string()),
                variant: None,
                debug: None,
                no_console: None,
                hardware_present: false,
                serial_number: None,
                firmware_version: None,
                manufacture_date: None,
                location: None,
                validation_status: None,
                last_validated: None,
                validation_notes: None,
                tags: Vec::new(),
                owner: None,
                notes: None,
                voltage_specification: "12VDC".to_string(),
            },
        );

        environments.insert(
            "test-env".to_string(),
            Environment {
                friendly_name: Some("Test Environment".to_string()),
                devices: test_env_devices,
                description: Some("Test environment".to_string()),
                location: Some("Test Lab".to_string()),
                can_interface: Some("vcan0".to_string()),
                enabled: true,
                tags: vec!["test".to_string()],
                created_date: None,
                last_modified: None,
                owner: None,
                test_plan: None,
                notes: None,
            },
        );

        // Hardware environment with physical device (no websocket_port)
        let mut hardware_env_devices = HashMap::new();
        hardware_env_devices.insert(
            "ecu-hw".to_string(),
            DeviceConfig {
                friendly_name: Some("Hardware ECU".to_string()),
                device_type: DeviceType::J1939,
                device_id: "0x83".to_string(),
                interface: Some("can0".to_string()),
                protocol: Some("j1939".to_string()),
                websocket_port: None, // Physical hardware doesn't have websocket
                enabled: true,
                description: Some("Hardware ECU".to_string()),
                variant: Some("ecu".to_string()),
                debug: None,
                no_console: None,
                hardware_present: true,
                serial_number: Some("ECU-HW-001".to_string()),
                firmware_version: Some("2.0.0".to_string()),
                manufacture_date: None,
                location: Some("Bench 3".to_string()),
                validation_status: None,
                last_validated: None,
                validation_notes: None,
                tags: Vec::new(),
                owner: None,
                notes: None,
                voltage_specification: "12VDC".to_string(),
            },
        );

        environments.insert(
            "hardware-env".to_string(),
            Environment {
                friendly_name: Some("Hardware Environment".to_string()),
                devices: hardware_env_devices,
                description: Some("Hardware environment".to_string()),
                location: Some("Lab Bench 1".to_string()),
                can_interface: Some("can0".to_string()),
                enabled: true,
                tags: vec!["hardware".to_string(), "j1939".to_string()],
                created_date: None,
                last_modified: None,
                owner: None,
                test_plan: None,
                notes: None,
            },
        );

        CandoConfig {
            version: "1.0.0".to_string(),
            defaults: Defaults::default(),
            webui: WebUiConfig::default(),
            environments,
            test: cando_config::TestConfig::default(),
            network: cando_config::NetworkConfig::default(),
            logging: cando_config::LoggingConfig::default(),
        }
    }

    #[test]
    fn test_resolve_device_params_valid() {
        let config = create_test_config();
        let result = resolve_device_params(&config, "ecu-a", "test-env");
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.device_id, "0x82");
        assert_eq!(params.interface, "vcan0");
        assert_eq!(params.protocol, "j1939");
        assert_eq!(params.websocket_port, 10754);
    }

    #[test]
    fn test_resolve_device_params_hardware() {
        let config = create_test_config();
        let result = resolve_device_params(&config, "ecu-hw", "hardware-env");
        assert!(result.is_ok());
        let params = result.unwrap();
        assert_eq!(params.device_id, "0x83");
        assert_eq!(params.interface, "can0");
        assert_eq!(params.protocol, "j1939");
    }

    #[test]
    fn test_resolve_device_params_not_found() {
        let config = create_test_config();
        let result = resolve_device_params(&config, "nonexistent", "test-env");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_resolve_device_params_disabled() {
        let config = create_test_config();
        let result = resolve_device_params(&config, "disabled-device", "test-env");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_config_get_device_valid() {
        let config = create_test_config();
        let device = config.get_device("test-env", "ecu-a");
        assert!(device.is_some());
        let device = device.unwrap();
        assert_eq!(device.friendly_name, Some("ECU A".to_string()));
    }

    #[test]
    fn test_config_enabled_devices() {
        let config = create_test_config();
        let enabled = config.enabled_devices();
        assert_eq!(enabled.len(), 3); // All except disabled-device
    }

    #[test]
    fn test_total_device_count() {
        let config = create_test_config();
        assert_eq!(config.all_devices().len(), 4);
    }
}
