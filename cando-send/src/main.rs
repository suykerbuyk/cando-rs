//! cando-send - Rust implementation of cansend utility
//!
//! A production-ready replacement for the can-utils cansend tool with enhanced
//! file replay capabilities.
//!
//! ## Modes
//!
//! ### Single Frame Mode (original cansend compatibility)
//! ```bash
//! cando-send vcan0 123#DEADBEEF
//! cando-send vcan0 5A1#11.22.33.44
//! cando-send vcan0 00000123#R5
//! cando-send vcan0 456##311223344
//! ```
//!
//! ### File Replay Mode (enhanced functionality)
//! ```bash
//! cando-send vcan0 --file candump.log --rate 100
//! cando-send vcan0 --file traffic.json --interval 10
//! cando-send vcan0 --file test.log --verbose
//! ```

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod frame;
mod parser;
mod replay;
mod sender;

/// Send CAN frames to a CAN interface
///
/// This tool operates in two modes:
///
/// 1. Single Frame Mode: Send one CAN frame and exit
///    cando-send <interface> <frame>
///
/// 2. File Replay Mode: Replay frames from a log file
///    cando-send <interface> --file <path> [--rate <rate>]
///
/// Examples:
///   # Single frame mode
///   cando-send vcan0 123#DEADBEEF
///   cando-send vcan0 5A1#11.22.33.44.55.66.77.88
///   cando-send vcan0 00000123#R5
///   cando-send vcan0 456##311223344
///
///   # File replay mode
///   cando-send vcan0 --file candump.log --rate 100
///   cando-send vcan0 --file traffic.json --interval 10
///   cando-send vcan0 --file test.log --verbose
#[derive(Parser, Debug)]
#[command(name = "cando-send")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// CAN interface name (e.g., vcan0, can0)
    #[arg(value_name = "INTERFACE", required_unless_present = "generate_manpage")]
    interface: Option<String>,

    /// CAN frame specification (single frame mode): <can_id>#{data}
    ///
    /// Formats:
    ///   - Standard ID (3 hex): 123#DEADBEEF
    ///   - Extended ID (8 hex): 00000123#11223344
    ///   - RTR frame: 123#R or 123#R5
    ///   - CAN FD: 123##311223344
    ///   - Dot-separated data: 5A1#11.22.33.44
    ///   - Empty data: 5AA#
    ///
    /// Not used when --file is specified.
    #[arg(value_name = "FRAME", required_unless_present_any = ["file", "generate_manpage"])]
    frame_spec: Option<String>,

    /// Log file to replay (file replay mode)
    ///
    /// Supported formats:
    ///   - candump ASCII: (timestamp) interface CANID#DATA
    ///   - candump JSON: structured JSON objects
    ///
    /// Format is auto-detected from file content.
    #[arg(short = 'f', long = "file", value_name = "PATH")]
    file: Option<PathBuf>,

    /// Replay rate in messages per second (file replay mode)
    ///
    /// Examples:
    ///   --rate 100    (100 messages/second, 10ms between frames)
    ///   --rate 1000   (1000 messages/second, 1ms between frames)
    ///   --rate 10     (10 messages/second, 100ms between frames)
    ///
    /// If neither --rate nor --interval is specified, frames are sent
    /// as fast as possible (no delay).
    #[arg(short = 'r', long = "rate", value_name = "RATE")]
    rate: Option<f64>,

    /// Fixed interval between frames in milliseconds (file replay mode)
    ///
    /// This overrides --rate if both are specified.
    ///
    /// Examples:
    ///   --interval 10   (10ms between frames = 100 msgs/sec)
    ///   --interval 100  (100ms between frames = 10 msgs/sec)
    ///   --interval 1    (1ms between frames = 1000 msgs/sec)
    #[arg(short = 'i', long = "interval", value_name = "MS")]
    interval_ms: Option<u64>,

    /// Verbose output (progress reporting and diagnostics)
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Generate man page to stdout (hidden flag for man page generation)
    #[arg(long = "generate-manpage", hide = true)]
    generate_manpage: bool,
}

impl Args {
    /// Validate command-line arguments
    fn validate(&self) -> Result<()> {
        // Validate interface name is provided
        if self.interface.is_none() {
            anyhow::bail!("Interface name is required");
        }

        // Validate interface name is not empty
        if let Some(ref interface) = self.interface
            && interface.is_empty() {
                anyhow::bail!("Interface name cannot be empty");
            }

        // Mode detection: either frame_spec XOR file must be present
        match (&self.frame_spec, &self.file) {
            (Some(_), Some(_)) => {
                anyhow::bail!(
                    "Cannot specify both frame specification and --file option.\n\
                     Use either single frame mode or file replay mode, not both."
                );
            }
            (None, None) => {
                anyhow::bail!(
                    "Must specify either a frame specification or --file option.\n\
                     Examples:\n\
                     Single frame: cando-send vcan0 123#DEADBEEF\n\
                     File replay:  cando-send vcan0 --file candump.log"
                );
            }
            _ => {}
        }

        // Validate rate control options
        if let Some(rate) = self.rate
            && rate <= 0.0 {
                anyhow::bail!("Replay rate must be greater than 0");
            }

        Ok(())
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // Parse and validate command-line arguments
    let args = Args::parse();

    // Handle man page generation (must be first, before validation)
    if args.generate_manpage {
        return generate_manpage();
    }

    args.validate()?;

    // Execute based on mode
    if let Some(file_path) = &args.file {
        // File replay mode
        run_replay_mode(&args, file_path)
    } else if let Some(frame_spec) = &args.frame_spec {
        // Single frame mode
        run_single_frame_mode(&args, frame_spec)
    } else {
        unreachable!("Validation should catch this case");
    }
}

/// Execute single frame mode
fn run_single_frame_mode(args: &Args, frame_spec: &str) -> Result<()> {
    // Parse the frame specification
    let parsed_frame =
        parser::parse_frame(frame_spec).context("Failed to parse frame specification")?;

    // Build the CAN frame
    let can_frame = frame::build_frame(&parsed_frame).context("Failed to build CAN frame")?;

    // Send the frame (interface is guaranteed to be Some by validation)
    let interface = args.interface.as_ref().unwrap();
    sender::send_frame(interface, &can_frame).context("Failed to send CAN frame")?;

    if args.verbose {
        eprintln!("Successfully sent frame to {}", interface);
    }

    Ok(())
}

/// Execute file replay mode
fn run_replay_mode(args: &Args, file_path: &PathBuf) -> Result<()> {
    // Build replay configuration
    let interval = args
        .interval_ms
        .map(std::time::Duration::from_millis);

    // Interface is guaranteed to be Some by validation
    let interface = args.interface.as_ref().unwrap().clone();

    let config = replay::ReplayConfig {
        interface: interface.clone(),
        rate: args.rate,
        interval,
        verbose: args.verbose,
    };

    if args.verbose {
        eprintln!("Starting file replay mode");
        eprintln!("Interface: {}", interface);
        eprintln!("File: {}", file_path.display());
        if let Some(rate) = args.rate {
            eprintln!("Rate: {} messages/second", rate);
        }
        if let Some(interval) = args.interval_ms {
            eprintln!("Interval: {}ms between frames", interval);
        }
    }

    // Open the CAN socket once (reused for all frames)
    let socket = sender::open_socket(&interface).context("Failed to open CAN socket")?;

    // Replay the file
    let stats = replay::replay_file(file_path, &config, |frame| {
        sender::send_frame_to_socket(&socket, frame)
    })
    .context("Failed to replay file")?;

    // Print statistics
    if args.verbose {
        eprintln!("\nReplay complete:");
    } else {
        eprintln!("Replay statistics:");
    }
    eprintln!("  Frames read:          {}", stats.frames_read);
    eprintln!("  Frames sent:          {}", stats.frames_sent);
    if stats.frames_skipped > 0 {
        eprintln!("  Frames skipped:       {}", stats.frames_skipped);
    }
    if stats.interface_mismatch > 0 {
        eprintln!("  Interface mismatch:   {}", stats.interface_mismatch);
    }

    // Return error if no frames were sent
    if stats.frames_sent == 0 {
        anyhow::bail!("No frames were successfully sent");
    }

    Ok(())
}

/// Generate man page to stdout (requires 'manpages' feature)
#[cfg(feature = "manpages")]
fn generate_manpage() -> Result<()> {
    use clap::CommandFactory;

    let cmd = Args::command();
    let man = clap_mangen::Man::new(cmd);
    man.render(&mut std::io::stdout())?;
    Ok(())
}

#[cfg(not(feature = "manpages"))]
fn generate_manpage() -> Result<()> {
    anyhow::bail!(
        "Man page generation requires the 'manpages' feature. Build with: cargo build --features manpages"
    )
}
