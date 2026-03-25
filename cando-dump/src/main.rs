//! cando-dump - Rust implementation of candump utility
//!
//! A 100% compatible replacement for the candump utility from can-utils,
//! implemented in Rust and integrated with the cando-rs toolkit.

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod dumper;
mod filter;
mod formatter;
mod timestamp;

use dumper::{CanDumper, DumperConfig};
use formatter::{create_formatter, OutputFormat};

#[derive(Parser, Debug)]
#[command(
    name = "cando-dump",
    version,
    about = "Dump CAN bus frames to stdout or log file",
    long_about = "A Rust implementation of the candump utility for monitoring CAN bus traffic.\n\
                  Provides 100% command-line compatibility with can-utils candump while\n\
                  integrating with the cando-rs CAN toolkit."
)]
struct Args {
    /// CAN interfaces to monitor with optional filters
    ///
    /// Format: interface[,filter1][,filter2][,...][,j|J]
    ///
    /// Filter syntax:
    ///   id:mask   - Include: (received_id & mask) == (id & mask)
    ///   id~mask   - Exclude: (received_id & mask) != (id & mask)
    ///   #error    - Error frames only
    ///   j or J    - AND logic for multiple filters (default: OR)
    ///
    /// Examples:
    ///   can0,123:7FF         - Only ID 0x123 on can0
    ///   can0,100~700:7FF     - Exclude 0x100-0x1FF on can0
    ///   vcan0,123:7FF,j      - Multiple filters with AND logic
    #[arg(required = true, value_name = "INTERFACE")]
    interfaces: Vec<String>,

    /// Timestamp mode: (a)bsolute, (d)elta, (z)ero, (A)bsolute with date
    #[arg(short = 't', default_value = "a", value_name = "TYPE")]
    timestamp_mode: char,

    /// Log CAN frames to file (default: cando-dump-YYYY-MM-DD_HHMMSS.log)
    #[arg(short = 'l')]
    log_mode: bool,

    /// Specify log filename (implies -l)
    #[arg(short = 'f', value_name = "FILE")]
    log_file: Option<PathBuf>,

    /// Terminate after receiving N frames
    #[arg(short = 'n', value_name = "COUNT")]
    frame_count: Option<usize>,

    /// Enable color output (can be specified multiple times for more colors)
    #[arg(short = 'c', action = clap::ArgAction::Count)]
    color_level: u8,

    /// Enable ASCII output (show printable characters)
    #[arg(short = 'a')]
    ascii_mode: bool,

    /// Silent mode: 0=off, 1=animation, 2=silent
    #[arg(short = 's', value_name = "LEVEL", default_value = "0")]
    silent_mode: u8,

    /// Terminate after timeout (milliseconds with no frames)
    #[arg(short = 'T', value_name = "MSECS")]
    timeout_ms: Option<u64>,

    /// Output format: candump (default), json, decoded
    #[arg(long = "format", value_name = "FORMAT", default_value = "candump")]
    format: String,

    /// Display statistics on exit
    #[arg(long = "stats")]
    stats_mode: bool,

    /// Generate man page and exit (internal use only)
    #[arg(long = "generate-manpage", hide = true)]
    generate_manpage: bool,
}

impl Args {
    /// Validate arguments and check for incompatibilities
    fn validate(&self) -> Result<()> {
        // Validate timestamp mode
        match self.timestamp_mode {
            'a' | 'd' | 'z' | 'A' => {}
            _ => anyhow::bail!(
                "Invalid timestamp mode: '{}'. Valid modes: a (absolute), d (delta), z (zero), A (absolute with date)",
                self.timestamp_mode
            ),
        }

        // Validate interfaces are not empty
        if self.interfaces.is_empty() {
            anyhow::bail!("At least one CAN interface must be specified");
        }

        // Validate silent mode
        if self.silent_mode > 2 {
            anyhow::bail!(
                "Invalid silent mode: {}. Valid values: 0 (off), 1 (animation), 2 (silent)",
                self.silent_mode
            );
        }

        // Validate output format
        if OutputFormat::from_str(&self.format).is_none() {
            anyhow::bail!(
                "Invalid output format: '{}'. Valid formats: candump, json, decoded",
                self.format
            );
        }

        Ok(())
    }

    /// Convert Args into DumperConfig
    fn into_config(self) -> Result<DumperConfig> {
        let log_file = if let Some(path) = self.log_file {
            Some(path)
        } else if self.log_mode {
            // Generate default filename: cando-dump-YYYY-MM-DD_HHMMSS.log
            let now = chrono::Local::now();
            let filename = format!("cando-dump-{}.log", now.format("%Y-%m-%d_%H%M%S"));
            Some(PathBuf::from(filename))
        } else {
            None
        };

        // Parse interface filter specifications
        let mut filter_set = filter::FilterSet::new();
        let mut interface_names = Vec::new();

        for spec in &self.interfaces {
            let (iface, iface_filter) = filter::parse_interface_filter(spec)
                .with_context(|| format!("Failed to parse interface specification: '{}'", spec))?;

            interface_names.push(iface.clone());

            // Only add to filter set if filters are specified
            if !iface_filter.filters.is_empty() {
                filter_set.add_interface_filter(iface_filter);
            }
        }

        Ok(DumperConfig {
            interfaces: interface_names,
            timestamp_mode: self.timestamp_mode,
            log_file,
            frame_count: self.frame_count,
            filter_set,
            color_level: self.color_level,
            ascii_mode: self.ascii_mode,
            silent_mode: self.silent_mode,
            timeout_ms: self.timeout_ms,
            stats_mode: self.stats_mode,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Check for manpage generation BEFORE parsing args
    // This allows --generate-manpage to work even with invalid args
    let raw_args: Vec<String> = std::env::args().collect();

    if raw_args.contains(&"--generate-manpage".to_string()) {
        #[cfg(feature = "manpages")]
        {
            use clap::CommandFactory;
            use clap_mangen::Man;
            let man = Man::new(Args::command());
            man.render(&mut std::io::stdout())
                .context("Failed to render man page to stdout")?;
            std::process::exit(0);
        }
        #[cfg(not(feature = "manpages"))]
        {
            eprintln!("Error: Manpage generation not enabled. Rebuild with --features=manpages");
            std::process::exit(1);
        }
    }

    // Parse command-line arguments
    let args = Args::parse();

    // Validate arguments
    args.validate().context("Invalid command-line arguments")?;

    // Parse output format
    let output_format = OutputFormat::from_str(&args.format).expect("Format already validated");

    // Save formatter parameters before consuming args
    let timestamp_mode = args.timestamp_mode;
    let color_level = args.color_level;
    let ascii_mode = args.ascii_mode;

    // Convert to config
    let config = args.into_config()?;

    // Set up Ctrl-C handler for graceful shutdown
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let mut shutdown_tx = Some(shutdown_tx);

    ctrlc::set_handler(move || {
        if let Some(tx) = shutdown_tx.take() {
            let _ = tx.send(());
        }
    })
    .context("Failed to set Ctrl-C handler")?;

    // Create formatter based on output format
    let formatter = create_formatter(output_format, timestamp_mode, color_level, ascii_mode);

    // Create and run dumper
    let mut dumper = CanDumper::new(config, formatter)
        .await
        .context("Failed to create CAN dumper")?;

    // Run until completion or Ctrl-C
    tokio::select! {
        result = dumper.run() => {
            result.context("Error during CAN dump operation")?;
        }
        _ = shutdown_rx => {
            eprintln!("\nShutdown signal received, stopping...");
        }
    }

    // Display statistics if enabled (on normal exit or SIGINT)
    dumper.display_stats();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_validation_valid() {
        let args = Args {
            interfaces: vec!["vcan0".to_string()],
            timestamp_mode: 'a',
            log_mode: false,
            log_file: None,
            frame_count: None,
            color_level: 0,
            ascii_mode: false,
            silent_mode: 0,
            timeout_ms: None,
            format: "candump".to_string(),
            stats_mode: false,
            generate_manpage: false,
        };

        assert!(args.validate().is_ok());
    }

    #[test]
    fn test_args_validation_invalid_timestamp() {
        let args = Args {
            interfaces: vec!["vcan0".to_string()],
            timestamp_mode: 'x',
            log_mode: false,
            log_file: None,
            frame_count: None,
            color_level: 0,
            ascii_mode: false,
            silent_mode: 0,
            timeout_ms: None,
            format: "candump".to_string(),
            stats_mode: false,
            generate_manpage: false,
        };

        assert!(args.validate().is_err());
    }

    #[test]
    fn test_config_generation_with_log_file() {
        let args = Args {
            interfaces: vec!["vcan0".to_string()],
            timestamp_mode: 'a',
            log_mode: false,
            log_file: Some(PathBuf::from("test.log")),
            frame_count: Some(100),
            color_level: 0,
            ascii_mode: false,
            silent_mode: 0,
            timeout_ms: None,
            format: "candump".to_string(),
            stats_mode: false,
            generate_manpage: false,
        };

        let config = args.into_config().unwrap();
        assert_eq!(config.interfaces, vec!["vcan0"]);
        assert_eq!(config.timestamp_mode, 'a');
        assert_eq!(config.log_file, Some(PathBuf::from("test.log")));
        assert_eq!(config.frame_count, Some(100));
    }

    #[test]
    fn test_config_generation_with_log_mode() {
        let args = Args {
            interfaces: vec!["vcan0".to_string()],
            timestamp_mode: 'a',
            log_mode: true,
            log_file: None,
            frame_count: None,
            color_level: 0,
            ascii_mode: false,
            silent_mode: 0,
            timeout_ms: None,
            format: "candump".to_string(),
            stats_mode: false,
            generate_manpage: false,
        };

        let config = args.into_config().unwrap();
        assert_eq!(config.interfaces, vec!["vcan0"]);
        assert!(config.log_file.is_some());
        // Verify filename format
        let filename = config.log_file.unwrap();
        let filename_str = filename.to_str().unwrap();
        assert!(filename_str.starts_with("cando-dump-"));
        assert!(filename_str.ends_with(".log"));
    }
}
