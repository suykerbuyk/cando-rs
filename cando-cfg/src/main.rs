//! cando-cfg - Query Cando configuration from scripts and automation
//!
//! This tool provides a command-line interface for querying the cando.yaml
//! configuration file from scripts and automation, eliminating hardcoded device IDs,
//! ports, and interface names.
//!
//! # Purpose
//!
//! Solve the configuration drift problem by enforcing a single source of truth:
//! - Device IDs defined once in cando.yaml
//! - Bash scripts query configuration dynamically
//! - Changes to config automatically propagate to all scripts
//! - No more manual synchronization between files
//!
//! # Usage Examples
//!
//! ```bash
//! # Get device ID for a device
//! DEVICE_ID=$(cando-cfg get-device-id "Test Device")
//!
//! # Get WebSocket port for J1939 simulator
//! WS_PORT=$(cando-cfg get-port "J1939 Test ECU")
//!
//! # Get CAN interface for a device
//! INTERFACE=$(cando-cfg get-interface "Test Device")
//!
//! # List all enabled devices
//! cando-cfg list-enabled-devices
//!
//! # Show complete device configuration
//! cando-cfg show-device "Test Device"
//! ```

use anyhow::Result;
use clap::{Parser, Subcommand};
use cando_config::CandoConfig;
use std::path::PathBuf;

/// Query Cando configuration from scripts and automation
///
/// This tool provides a single source of truth for test configuration,
/// eliminating hardcoded device IDs, ports, and interfaces in bash scripts.
#[derive(Parser, Debug)]
#[command(name = "cando-cfg")]
#[command(author, version, about, long_about = None)]
#[command(after_help = "EXAMPLES:
    # Get device ID for a device
    cando-cfg get-device-id \"Test Device\"

    # Get WebSocket port for J1939 simulator
    cando-cfg get-port \"J1939 Test ECU\"

    # Get CAN interface
    cando-cfg get-interface \"Test Device\"

    # List all enabled devices
    cando-cfg list-enabled-devices

    # Show complete device info
    cando-cfg show-device \"Test Device\"

    # Get standard test source device ID
    cando-cfg get-test-source

    # List all test sources (standard + variants)
    cando-cfg list-test-sources

    # Get only variant test sources
    cando-cfg get-test-variant-sources

    # Use custom config file
    cando-cfg --config custom.yaml get-device-id \"Fan A\"

CONFIGURATION FILE:
    Default search path (first found):
      1. --config argument
      2. CANDO_CONFIG environment variable
      3. ./cando.yaml
      4. ~/.config/cando/cando.yaml

EXIT CODES:
    0 - Success
    1 - Configuration error (file not found, parse error)
    2 - Device not found
    3 - Invalid query (missing field, etc.)
")]
struct Args {
    /// Path to configuration file
    ///
    /// If not specified, searches default locations:
    /// ./cando.yaml, ~/.config/cando/cando.yaml
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Environment to validate (validates only this environment instead of entire config)
    ///
    /// When specified, only the devices and configuration relevant to this environment
    /// are validated. This avoids validation warnings for unused environments.
    /// If not specified, the entire configuration file is validated.
    #[arg(short, long, value_name = "NAME")]
    environment: Option<String>,

    /// Output format for show-device command
    #[arg(short, long, value_enum, default_value = "shell")]
    format: OutputFormat,

    /// Generate man page and exit (internal use)
    #[arg(long = "generate-manpage", hide = true)]
    generate_manpage: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum OutputFormat {
    /// Shell-friendly key=value format
    Shell,
    /// JSON format
    Json,
    /// Human-readable format
    Pretty,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Get device ID (e.g., 0x82)
    GetDeviceId {
        /// Device name (or 'environment:device' format)
        device_name: String,

        /// Environment name (explicit context)
        #[arg(short, long)]
        environment: Option<String>,
    },

    /// Get WebSocket port number
    GetPort {
        /// Device name (or 'environment:device' format)
        device_name: String,

        /// Environment name (explicit context)
        #[arg(short, long)]
        environment: Option<String>,
    },

    /// Get CAN interface name
    GetInterface {
        /// Device name (or 'environment:device' format)
        device_name: String,

        /// Environment name (explicit context)
        #[arg(short, long)]
        environment: Option<String>,
    },

    /// Get device type (j1939, etc.)
    GetType {
        /// Device name (or 'environment:device' format)
        device_name: String,

        /// Environment name (explicit context)
        #[arg(short, long)]
        environment: Option<String>,
    },

    /// Get device variant
    GetVariant {
        /// Device name (or 'environment:device' format)
        device_name: String,

        /// Environment name (explicit context)
        #[arg(short, long)]
        environment: Option<String>,
    },

    /// List all devices
    ListDevices {
        /// Filter by environment name
        #[arg(short, long)]
        environment: Option<String>,

        /// Only show enabled devices
        #[arg(long)]
        enabled_only: bool,
    },

    /// Show complete device configuration
    ShowDevice {
        /// Device name (or 'environment:device' format)
        device_name: String,

        /// Environment name (explicit context)
        #[arg(short, long)]
        environment: Option<String>,
    },

    /// List all environments
    ListEnvironments,

    /// Show environment details
    ShowEnvironment {
        /// Environment name
        environment_name: String,
    },

    /// Get standard test source device ID
    GetTestSource,

    /// List all test source device IDs (standard + variants)
    ListTestSources,

    /// Get variant test source device IDs for multi-source tests
    GetTestVariantSources,
}

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    // Handle manpage generation before parsing (allows generation even with invalid args)
    if raw_args.contains(&"--generate-manpage".to_string()) {
        if let Err(e) = generate_manpage() {
            eprintln!("Error generating man page: {}", e);
            std::process::exit(1);
        }
        std::process::exit(0);
    }

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(match e.downcast_ref::<ConfigError>() {
            Some(ConfigError::InvalidQuery(_)) => 3,
            _ => 1,
        });
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Load configuration
    let config = load_config(args.config.as_deref(), args.environment.as_deref())?;

    // Execute command
    match args.command {
        Some(Command::GetDeviceId {
            device_name,
            environment,
        }) => {
            let device = config.get_device_by_reference(&device_name, environment.as_deref())?;
            println!("{}", device.device_id);
        }

        Some(Command::GetPort {
            device_name,
            environment,
        }) => {
            let device = config.get_device_by_reference(&device_name, environment.as_deref())?;
            if let Some(port) = device.websocket_port {
                println!("{}", port);
            } else {
                return Err(ConfigError::InvalidQuery(
                    "Device has no websocket_port (physical hardware)".to_string(),
                )
                .into());
            }
        }

        Some(Command::GetInterface {
            device_name,
            environment,
        }) => {
            let device = config.get_device_by_reference(&device_name, environment.as_deref())?;
            if let Some(interface) = &device.interface {
                println!("{}", interface);
            } else {
                return Err(ConfigError::InvalidQuery(
                    "Device has no interface specified (inherited)".to_string(),
                )
                .into());
            }
        }

        Some(Command::GetType {
            device_name,
            environment,
        }) => {
            let device = config.get_device_by_reference(&device_name, environment.as_deref())?;
            println!("{}", device.device_type);
        }

        Some(Command::GetVariant {
            device_name,
            environment,
        }) => {
            let device = config.get_device_by_reference(&device_name, environment.as_deref())?;
            if let Some(variant) = &device.variant {
                println!("{}", variant);
            } else {
                return Err(
                    ConfigError::InvalidQuery("Device has no variant field".to_string()).into(),
                );
            }
        }

        Some(Command::ListDevices {
            environment,
            enabled_only,
        }) => {
            list_devices(&config, environment.as_deref(), enabled_only, &args.format)?;
        }

        Some(Command::ShowDevice {
            device_name,
            environment,
        }) => {
            let device = config.get_device_by_reference(&device_name, environment.as_deref())?;

            // Parse environment and device key for display
            // Store owned strings to ensure proper lifetimes
            let parsed_reference;
            let (env_name, device_key) = if let Some(ref env) = environment {
                (env.as_str(), device_name.as_str())
            } else {
                parsed_reference = CandoConfig::parse_device_reference(&device_name)?;
                (parsed_reference.0.as_str(), parsed_reference.1.as_str())
            };

            show_device(env_name, device_key, device, &args.format)?;
        }

        Some(Command::ListEnvironments) => {
            list_environments(&config, &args.format)?;
        }

        Some(Command::ShowEnvironment { environment_name }) => {
            show_environment(&config, &environment_name, &args.format)?;
        }

        Some(Command::GetTestSource) => {
            if let Some(source) = &config.test.standard_source_device_id {
                println!("{}", source);
                return Ok(());
            }
            eprintln!("Error: No standard test source configured in configuration file");
            std::process::exit(3);
        }

        Some(Command::ListTestSources) => {
            let mut has_sources = false;

            // Print standard source
            if let Some(std) = &config.test.standard_source_device_id {
                println!("{}", std);
                has_sources = true;
            }

            // Print variant sources
            if let Some(variants) = &config.test.variant_source_device_ids {
                for variant in variants {
                    println!("{}", variant);
                    has_sources = true;
                }
            }

            if !has_sources {
                eprintln!("Error: No test sources configured in configuration file");
                std::process::exit(3);
            }
        }

        Some(Command::GetTestVariantSources) => {
            if let Some(variants) = &config.test.variant_source_device_ids {
                for variant in variants {
                    println!("{}", variant);
                }
                return Ok(());
            }
            eprintln!("Error: No variant test sources configured in configuration file");
            std::process::exit(3);
        }

        None => {
            eprintln!("No command specified. Use --help for usage information.");
            std::process::exit(1);
        }
    }

    Ok(())
}

/// Load configuration from file with helpful error messages
fn load_config(path: Option<&std::path::Path>, environment: Option<&str>) -> Result<CandoConfig> {
    let config = if let Some(path) = path {
        if let Some(env_name) = environment {
            // Load with environment-specific validation
            match CandoConfig::load_from_file_for_environment(path, env_name) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!(
                        "Failed to load config from {} for environment '{}'",
                        path.display(),
                        env_name
                    );
                    eprintln!("Error details: {:?}", e);
                    return Err(anyhow::anyhow!(
                        "Failed to load config from {} for environment '{}': {}",
                        path.display(),
                        env_name,
                        e
                    ));
                }
            }
        } else {
            // Load with full validation
            match CandoConfig::load_from_file(path) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Failed to load config from {}", path.display());
                    eprintln!("Error details: {:?}", e);
                    return Err(anyhow::anyhow!(
                        "Failed to load config from {}: {}",
                        path.display(),
                        e
                    ));
                }
            }
        }
    } else {
        // Try default locations
        let default_paths = vec![PathBuf::from("cando.yaml")];

        let mut config = None;
        let mut tried_paths = Vec::new();

        for path in default_paths {
            tried_paths.push(path.display().to_string());
            if path.exists() {
                let result = if let Some(env_name) = environment {
                    CandoConfig::load_from_file_for_environment(&path, env_name)
                } else {
                    CandoConfig::load_from_file(&path)
                };

                match result {
                    Ok(cfg) => {
                        config = Some(cfg);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Failed to parse {}", path.display());
                        eprintln!("Error details: {:?}", e);
                        return Err(anyhow::anyhow!("Failed to parse {}: {}", path.display(), e));
                    }
                }
            }
        }

        if let Some(config) = config {
            config
        } else {
            // Try system locations
            if let Some(home) = dirs::home_dir() {
                let user_config = home.join(".config/cando/cando.yaml");
                if user_config.exists() {
                    let result = if let Some(env_name) = environment {
                        CandoConfig::load_from_file_for_environment(&user_config, env_name)
                    } else {
                        CandoConfig::load_from_file(&user_config)
                    };

                    match result {
                        Ok(cfg) => cfg,
                        Err(e) => {
                            eprintln!("Failed to parse {}", user_config.display());
                            eprintln!("Error details: {:?}", e);
                            return Err(anyhow::anyhow!(
                                "Failed to parse {}: {}",
                                user_config.display(),
                                e
                            ));
                        }
                    }
                } else {
                    anyhow::bail!(
                        "Configuration file not found. Tried:\n  {}",
                        tried_paths.join("\n  ")
                    );
                }
            } else {
                anyhow::bail!(
                    "Configuration file not found. Tried:\n  {}",
                    tried_paths.join("\n  ")
                );
            }
        }
    };

    Ok(config)
}

/// List devices
fn list_devices(
    config: &CandoConfig,
    environment: Option<&str>,
    enabled_only: bool,
    format: &OutputFormat,
) -> Result<()> {
    let devices: Vec<(&str, &str, &cando_config::DeviceConfig)> = if enabled_only {
        config.enabled_devices()
    } else {
        config.all_devices()
    };

    // Filter by environment if specified
    let devices: Vec<_> = if let Some(env_name) = environment {
        // Verify environment exists
        config.get_environment(env_name).ok_or_else(|| {
            ConfigError::InvalidQuery(format!("Environment not found: {}", env_name))
        })?;

        devices
            .into_iter()
            .filter(|(e, _, _)| *e == env_name)
            .collect()
    } else {
        devices
    };

    match format {
        OutputFormat::Shell => {
            for (env_name, device_key, _device) in devices {
                // Output format depends on whether environment was specified
                if environment.is_some() {
                    // When environment is specified, output plain device names
                    println!("{}", device_key);
                } else {
                    // When no environment specified, output "environment:device" format
                    println!("{}:{}", env_name, device_key);
                }
            }
        }
        OutputFormat::Json => {
            // Convert to JSON-friendly format
            let json_devices: Vec<_> = devices
                .iter()
                .map(|(env, key, device)| {
                    serde_json::json!({
                        "environment": env,
                        "key": key,
                        "device": device
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json_devices)?);
        }
        OutputFormat::Pretty => {
            for (env_name, device_key, device) in devices {
                let interface = device.interface.as_deref().unwrap_or("(inherited)");
                println!(
                    "{:<20}:{:<20} | {} | {} | {}",
                    env_name,
                    device_key,
                    device.device_id,
                    interface,
                    if device.enabled {
                        "enabled"
                    } else {
                        "disabled"
                    }
                );
            }
        }
    }

    Ok(())
}

/// Show device information
fn show_device(
    env_name: &str,
    device_key: &str,
    device: &cando_config::DeviceConfig,
    format: &OutputFormat,
) -> Result<()> {
    match format {
        OutputFormat::Shell => {
            println!("environment={}", env_name);
            println!("key={}", device_key);
            println!(
                "friendly_name={}",
                device.friendly_name.as_deref().unwrap_or("")
            );
            println!("type={}", device.device_type);
            println!("device_id={}", device.device_id);
            println!(
                "interface={}",
                device.interface.as_deref().unwrap_or("(inherited)")
            );
            println!(
                "protocol={}",
                device.protocol.as_deref().unwrap_or("(inherited)")
            );
            if let Some(port) = device.websocket_port {
                println!("websocket_port={}", port);
            }
            println!("enabled={}", device.enabled);
            println!("hardware_present={}", device.hardware_present);
            if let Some(variant) = &device.variant {
                println!("variant={}", variant);
            }
            if let Some(desc) = &device.description {
                println!("description={}", desc);
            }
        }
        OutputFormat::Json => {
            let json_output = serde_json::json!({
                "environment": env_name,
                "key": device_key,
                "device": device
            });
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        }
        OutputFormat::Pretty => {
            let friendly = device.friendly_name.as_deref().unwrap_or("(unnamed)");
            println!("Device: {}", friendly);
            println!("  Environment:    {}", env_name);
            println!("  Key:            {}", device_key);
            println!("  Type:           {}", device.device_type);
            println!("  Device ID:      {}", device.device_id);
            println!(
                "  Interface:      {}",
                device.interface.as_deref().unwrap_or("(inherited)")
            );
            println!(
                "  Protocol:       {}",
                device.protocol.as_deref().unwrap_or("(inherited)")
            );
            if let Some(port) = device.websocket_port {
                println!("  WebSocket Port: {}", port);
            } else {
                println!("  WebSocket Port: (none - physical hardware)");
            }
            println!("  Enabled:        {}", device.enabled);
            println!(
                "  Hardware:       {}",
                if device.hardware_present {
                    "Physical"
                } else {
                    "Simulated"
                }
            );
            if let Some(variant) = &device.variant {
                println!("  Variant:        {}", variant);
            }
            if let Some(desc) = &device.description {
                println!("  Description:    {}", desc);
            }
        }
    }

    Ok(())
}

/// List environments
fn list_environments(config: &CandoConfig, format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Shell => {
            for env_name in config.environments.keys() {
                println!("{}", env_name);
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&config.environments)?);
        }
        OutputFormat::Pretty => {
            for (env_name, env) in &config.environments {
                println!(
                    "{:<20} | {} | {} devices",
                    env_name,
                    if env.enabled { "enabled" } else { "disabled" },
                    env.devices.len()
                );
            }
        }
    }

    Ok(())
}

/// Show environment details
fn show_environment(config: &CandoConfig, name: &str, format: &OutputFormat) -> Result<()> {
    let env = config
        .get_environment(name)
        .ok_or_else(|| ConfigError::InvalidQuery(format!("Environment not found: {}", name)))?;

    match format {
        OutputFormat::Shell => {
            println!("name={}", name);
            println!(
                "friendly_name={}",
                env.friendly_name.as_deref().unwrap_or("")
            );
            println!(
                "can_interface={}",
                env.can_interface.as_deref().unwrap_or("")
            );
            println!("enabled={}", env.enabled);

            let device_keys: Vec<&str> = env.devices.keys().map(|k| k.as_str()).collect();
            println!("devices={}", device_keys.join(","));
        }
        OutputFormat::Json => {
            let json_output = serde_json::json!({
                "name": name,
                "environment": env
            });
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        }
        OutputFormat::Pretty => {
            let friendly = env.friendly_name.as_deref().unwrap_or("(unnamed)");
            println!("Environment: {}", friendly);
            println!("  Name:          {}", name);
            println!(
                "  Interface:     {}",
                env.can_interface.as_deref().unwrap_or("N/A")
            );
            println!("  Enabled:       {}", env.enabled);

            println!("  Devices:       {}", env.devices.len());
            for (device_key, device) in &env.devices {
                let friendly = device.friendly_name.as_deref().unwrap_or("(unnamed)");
                println!("    - {} ({})", device_key, friendly);
            }
            if let Some(desc) = &env.description {
                println!("  Description:   {}", desc);
            }
        }
    }

    Ok(())
}

/// Generate man page (requires 'manpages' feature)
#[cfg(feature = "manpages")]
fn generate_manpage() -> Result<()> {
    use clap::CommandFactory;

    let cmd = Args::command();
    let man = clap_mangen::Man::new(cmd);
    man.render(&mut std::io::stdout())?;
    Ok(())
}

/// Generate man page (feature disabled)
#[cfg(not(feature = "manpages"))]
fn generate_manpage() -> Result<()> {
    anyhow::bail!(
        "Man page generation requires the 'manpages' feature. Build with: cargo build --features manpages"
    )
}

/// Custom error types for better exit codes
#[derive(Debug, thiserror::Error)]
enum ConfigError {
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}
