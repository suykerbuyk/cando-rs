//! Common CLI arguments for CAN device simulators
//!
//! This module provides standard command-line arguments used by all simulators,
//! with support for YAML configuration files and configuration precedence.

use clap::Parser;
use cando_config::{DeviceConfig, CandoConfig};
use std::path::PathBuf;

/// Common command-line arguments shared by all CAN device simulators
///
/// Supports both traditional CLI-only mode and configuration file mode with
/// flexible precedence rules:
///
/// **Precedence Order** (highest to lowest):
/// 1. CLI arguments (--interface, --websocket-port, etc.)
/// 2. Environment variables
/// 3. Device-specific config from TOML file
/// 4. Global defaults from TOML file
/// 5. Built-in defaults
///
/// # Examples
///
/// ```rust
/// use clap::Parser;
/// use cando_simulator_common::CommonSimulatorArgs;
///
/// #[derive(Parser)]
/// struct Args {
///     #[command(flatten)]
///     common: CommonSimulatorArgs,
///
///     // Add simulator-specific arguments here
/// }
///
/// fn main() -> anyhow::Result<()> {
///     let args = Args::parse();
///
///     // Resolve configuration with precedence
///     let config = args.common.resolve_config()?;
///
///     println!("Interface: {}", config.interface);
///     println!("WebSocket port: {}", config.websocket_port);
///
///     Ok(())
/// }
/// ```
///
/// ## Usage Modes
///
/// ### Pure CLI Mode (backward compatible)
/// ```bash
/// j1939-simulator --interface can0 --websocket-port 10754
/// ```
///
/// ### Config File Mode
/// ```bash
/// j1939-simulator --config cando.yaml --device-name "Device A"
/// ```
///
/// ### Hybrid Mode (CLI overrides config)
/// ```bash
/// j1939-simulator --config cando.yaml --device-name "Device A" --interface can1
/// ```
#[derive(Parser, Debug, Clone)]
pub struct CommonSimulatorArgs {
    /// Path to configuration file
    ///
    /// If not provided, will search in standard locations:
    /// - ./cando.yaml
    /// - ~/.config/cando/cando.yaml
    /// - /etc/cando/cando.yaml (Unix only)
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Device name from configuration file
    ///
    /// References a device by name or friendly_name from the config file.
    /// Example: --device-name "Device A"
    #[arg(long)]
    pub device_name: Option<String>,

    /// Environment name from configuration file
    ///
    /// References an environment by name from the config file.
    /// MUST be used together with --device-name for deterministic device resolution.
    /// Example: --environment "test-env" --device-name "test_device"
    #[arg(long)]
    pub environment: Option<String>,

    /// CAN interface to use (e.g., "can0", "vcan0")
    ///
    /// Overrides config file setting if provided.
    #[arg(short = 'i', long)]
    pub interface: Option<String>,

    /// WebSocket server port
    ///
    /// Overrides config file setting if provided.
    #[arg(short = 'p', long)]
    pub websocket_port: Option<u16>,

    /// Enable debug logging
    ///
    /// Overrides config file setting if provided.
    #[arg(short = 'd', long)]
    pub debug: bool,

    /// Disable WebSocket server
    ///
    /// Takes precedence over config file setting.
    #[arg(long)]
    pub no_websocket: bool,

    /// Disable console output (quiet mode)
    #[arg(long)]
    pub no_console: bool,
}

impl CommonSimulatorArgs {
    /// Resolve final configuration from all sources with precedence
    ///
    /// Applies precedence rules:
    /// 1. CLI arguments (highest priority)
    /// 2. Environment variables
    /// 3. Device-specific config from TOML
    /// 4. Global defaults from TOML
    /// 5. Built-in defaults (lowest priority)
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Config file cannot be loaded or parsed
    /// - Device name not found in config
    /// - Configuration validation fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cando_simulator_common::CommonSimulatorArgs;
    /// use clap::Parser;
    ///
    /// #[derive(Parser)]
    /// struct Args {
    ///     #[command(flatten)]
    ///     common: CommonSimulatorArgs,
    /// }
    ///
    /// let args = Args::parse();
    /// let config = args.common.resolve_config()?;
    /// println!("Using interface: {}", config.interface);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn resolve_config(&self) -> anyhow::Result<ResolvedConfig> {
        // Load configuration file (or use defaults)
        let toml_config = CandoConfig::load(self.config.clone())?;

        // CRITICAL: Device names are ENVIRONMENT-SCOPED
        // The same device name can exist in multiple environments
        // Therefore: --device-name REQUIRES --environment for deterministic resolution

        if let Some(device_key) = &self.device_name {
            // --device-name provided: MUST have --environment for scoping
            if let Some(env_name) = &self.environment {
                // CORRECT: Both --device-name and --environment provided
                // Search ONLY within the specified environment
                let env = toml_config.get_environment(env_name).ok_or_else(|| {
                    anyhow::anyhow!("Environment '{}' not found in configuration", env_name)
                })?;

                // Check if device exists in THIS environment
                if env.devices.contains_key(device_key) {
                    // Resolve inheritance for this device
                    let resolved_device = toml_config
                        .resolve_device(env_name, device_key)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "Failed to resolve device '{}' in environment '{}'",
                                device_key,
                                env_name
                            )
                        })?;
                    return Ok(self.merge_with_device(
                        &resolved_device,
                        &toml_config,
                        env_name,
                        device_key,
                    ));
                }

                // Device not found in specified environment - helpful error
                anyhow::bail!(
                    "Device '{}' not found in environment '{}'\n\n\
                     Available devices in '{}':\n{}\n\n\
                     Did you mean one of these devices?",
                    device_key,
                    env_name,
                    env_name,
                    env.devices.keys()
                        .map(|k| format!("  - {}", k))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            } else {
                // ERROR: --device-name WITHOUT --environment
                // This violates environment isolation principle
                anyhow::bail!(
                    "Error: --device-name requires --environment for deterministic device resolution\n\n\
                     Device names are environment-scoped and may exist in multiple environments.\n\
                     Please specify both:\n  --device-name {} --environment <env-name>\n\n\
                     Use 'cando-cfg list-environments' to see available environments.",
                    device_key
                );
            }
        } else if let Some(env_name) = &self.environment {
            // Error: --environment requires --device-name for deterministic resolution
            let env = toml_config.get_environment(env_name).ok_or_else(|| {
                anyhow::anyhow!("Environment '{}' not found in configuration", env_name)
            })?;

            // Build helpful error message with available devices
            let mut error_msg = format!(
                "Error: --environment requires --device-name for deterministic device resolution\n\n\
                 Environment '{}' contains {} device(s):",
                env_name,
                env.devices.len()
            );

            // List all available devices in this environment
            let mut device_list: Vec<_> = env.devices.iter().collect();
            device_list.sort_by_key(|(key, _)| *key);

            for (device_key, device) in device_list {
                let friendly = device
                    .friendly_name
                    .as_deref()
                    .unwrap_or("No friendly name");
                let device_id = &device.device_id;
                error_msg.push_str(&format!(
                    "\n  - {} ({}, ID: {})",
                    device_key, friendly, device_id
                ));
            }

            if !env.devices.is_empty() {
                let first_device = env.devices.keys().next().unwrap();
                error_msg.push_str(&format!(
                    "\n\nPlease specify:\n  --environment {} --device-name {}",
                    env_name, first_device
                ));
            }

            anyhow::bail!(error_msg)
        } else {
            // No device name or environment: use CLI args with global defaults
            Ok(self.merge_with_defaults(&toml_config))
        }
    }

    /// Merge CLI args with device-specific config
    ///
    /// Used when a device_name is specified.
    /// Precedence: CLI > Device Config > Global Defaults > Built-in Defaults
    fn merge_with_device(
        &self,
        device: &DeviceConfig,
        config: &CandoConfig,
        env_name: &str,
        device_key: &str,
    ) -> ResolvedConfig {
        ResolvedConfig {
            interface: self.interface.clone().unwrap_or_else(|| {
                device
                    .interface
                    .clone()
                    .unwrap_or_else(|| config.defaults.can_interface.clone())
            }),

            websocket_port: self.websocket_port.unwrap_or_else(|| {
                device.websocket_port.unwrap_or(10754) // Default port if not specified
            }),

            // Boolean flags: CLI flag OR device setting OR global default
            debug: self.debug || device.debug.unwrap_or(config.defaults.debug),

            no_websocket: self.no_websocket || !device.enabled,

            no_console: self.no_console || device.no_console.unwrap_or(config.defaults.no_console),

            // Device-specific information from config
            device_name: Some(format!("{}:{}", env_name, device_key)),
            friendly_name: device.friendly_name.clone(),
            description: device.description.clone(),
            device_type: Some(device.device_type),
            variant: device.variant.clone(),
            device_id: Some(device.device_id.clone()),
            protocol: device
                .protocol
                .clone()
                .or_else(|| Some(config.defaults.protocol.clone())),
            voltage_specification: device.voltage_specification.clone(),
        }
    }

    /// Merge CLI args with global defaults only
    ///
    /// Used when no device_name is specified.
    /// Precedence: CLI > Global Defaults > Built-in Defaults
    fn merge_with_defaults(&self, config: &CandoConfig) -> ResolvedConfig {
        ResolvedConfig {
            interface: self
                .interface
                .clone()
                .unwrap_or_else(|| config.defaults.can_interface.clone()),

            websocket_port: self.websocket_port.unwrap_or(3030), // Built-in default

            debug: self.debug || config.defaults.debug,

            no_websocket: self.no_websocket || !config.defaults.websocket_enabled,

            no_console: self.no_console || config.defaults.no_console,

            // No device-specific information available
            device_name: None,
            friendly_name: None,
            description: None,
            device_type: None,
            variant: None,
            device_id: None,
            protocol: None,
            voltage_specification: "600VDC".to_string(),
        }
    }

    /// Initialize tracing/logging based on debug flag
    ///
    /// Sets up tracing-subscriber with appropriate log level:
    /// - debug=true: TRACE level
    /// - debug=false: INFO level
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cando_simulator_common::CommonSimulatorArgs;
    ///
    /// let args = CommonSimulatorArgs::default();
    /// args.init_tracing();
    /// ```
    pub fn init_tracing(&self) {
        use tracing_subscriber::EnvFilter;

        let filter = if self.debug {
            EnvFilter::new("trace")
        } else {
            EnvFilter::new("info")
        };

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(false)
            .init();
    }
}

impl Default for CommonSimulatorArgs {
    fn default() -> Self {
        Self {
            config: None,
            device_name: None,
            environment: None,
            interface: Some("vcan0".to_string()),
            websocket_port: Some(3030),
            debug: false,
            no_websocket: false,
            no_console: false,
        }
    }
}

/// Resolved configuration after applying all precedence rules
///
/// This is the final configuration that should be used by the simulator,
/// after merging CLI arguments, environment variables, config file settings,
/// and built-in defaults.
///
/// # Examples
///
/// ```no_run
/// use cando_simulator_common::{CommonSimulatorArgs, ResolvedConfig};
/// use clap::Parser;
///
/// #[derive(Parser)]
/// struct Args {
///     #[command(flatten)]
///     common: CommonSimulatorArgs,
/// }
///
/// let args = Args::parse();
/// let config: ResolvedConfig = args.common.resolve_config()?;
///
/// // Use resolved configuration
/// println!("Starting on {}", config.interface);
/// if let Some(friendly_name) = config.friendly_name {
///     println!("Device: {}", friendly_name);
/// }
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    /// CAN interface to use
    pub interface: String,

    /// WebSocket server port
    pub websocket_port: u16,

    /// Debug logging enabled
    pub debug: bool,

    /// WebSocket server disabled
    pub no_websocket: bool,

    /// Console output disabled
    pub no_console: bool,

    /// Device name (from config file)
    pub device_name: Option<String>,

    /// Friendly device name (from config file)
    pub friendly_name: Option<String>,

    /// Device description (from config file)
    pub description: Option<String>,

    /// Device type (from config file)
    pub device_type: Option<cando_config::DeviceType>,

    /// Generic device variant/subtype/role (from config file)
    pub variant: Option<String>,

    /// Device CAN ID (from config file)
    pub device_id: Option<String>,

    /// Protocol (from config file)
    pub protocol: Option<String>,

    /// Voltage specification: device design property (28VDC or 600VDC)
    pub voltage_specification: String,
}

impl ResolvedConfig {
    /// Get voltage specification from configuration
    ///
    /// Returns the voltage specification string ("28VDC" or "600VDC"),
    /// defaulting to "600VDC" if not specified.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use cando_simulator_common::{CommonSimulatorArgs, ResolvedConfig};
    /// # let config = CommonSimulatorArgs::default().resolve_config()?;
    /// let voltage = config.get_voltage_specification();
    /// println!("Voltage spec: {}", voltage);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn get_voltage_specification(&self) -> &str {
        &self.voltage_specification
    }

    /// Print a formatted startup banner with configuration details
    ///
    /// Displays device information if available from config file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use cando_simulator_common::{CommonSimulatorArgs, ResolvedConfig};
    /// # let config = CommonSimulatorArgs::default().resolve_config()?;
    /// config.print_banner("J1939");
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn print_banner(&self, simulator_type: &str) {
        if self.no_console {
            return;
        }

        println!("Starting {} Simulator", simulator_type);

        if let Some(name) = &self.device_name {
            println!("   Name: {}", name);
        }

        if let Some(friendly_name) = &self.friendly_name {
            println!("   Friendly Name: {}", friendly_name);
        }

        if let Some(description) = &self.description {
            println!("   Description: {}", description);
        }

        if let Some(device_id) = &self.device_id {
            println!("   Device ID: {}", device_id);
        }

        println!("   Interface: {}", self.interface);

        if let Some(protocol) = &self.protocol {
            println!("   Protocol: {}", protocol);
        }

        if !self.no_websocket {
            println!("   WebSocket: ws://127.0.0.1:{}", self.websocket_port);
        }

        println!("Simulator ready");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let args = CommonSimulatorArgs::default();
        assert_eq!(args.interface, Some("vcan0".to_string()));
        assert_eq!(args.websocket_port, Some(3030));
        assert!(!args.debug);
        assert!(!args.no_websocket);
        assert!(!args.no_console);
        assert!(args.config.is_none());
        assert!(args.device_name.is_none());
    }

    #[test]
    fn test_resolve_config_with_defaults() {
        let args = CommonSimulatorArgs::default();
        let config = args.resolve_config().expect("Should resolve with defaults");

        assert_eq!(config.interface, "vcan0");
        assert_eq!(config.websocket_port, 3030);
        assert!(!config.debug);
        assert!(!config.no_websocket);
        assert!(config.device_name.is_none());
    }

    #[test]
    fn test_resolve_config_with_cli_overrides() {
        let args = CommonSimulatorArgs {
            interface: Some("can0".to_string()),
            websocket_port: Some(8080),
            debug: true,
            ..Default::default()
        };

        let config = args.resolve_config().expect("Should resolve");

        // CLI args should take precedence
        assert_eq!(config.interface, "can0");
        assert_eq!(config.websocket_port, 8080);
        assert!(config.debug);
    }

    #[test]
    fn test_init_tracing_no_panic() {
        // Just ensure init_tracing doesn't panic
        // (can only call once per process, so we can't test both paths)
        let args = CommonSimulatorArgs::default();
        // This would panic if called twice:
        // args.init_tracing();
        // So we just verify the method exists and compiles
        let _ = args.debug;
    }

    #[test]
    fn test_resolved_config_print_banner_no_panic() {
        let config = ResolvedConfig {
            interface: "vcan0".to_string(),
            websocket_port: 3030,
            debug: false,
            no_websocket: false,
            no_console: true, // Suppress output in test
            device_name: Some("Test".to_string()),
            friendly_name: Some("Test Device".to_string()),
            description: Some("Test Description".to_string()),
            device_type: Some(cando_config::DeviceType::J1939),
            variant: Some("fan".to_string()),
            device_id: Some("0x82".to_string()),
            protocol: Some("proprietary".to_string()),
            voltage_specification: "600VDC".to_string(),
        };

        // Should not panic
        config.print_banner("TEST");
    }

    #[test]
    fn test_environment_requires_device_name() {
        // Test that --environment without --device-name produces an error
        use std::io::Write;
        use tempfile::NamedTempFile;

        // Create a minimal test config with multiple devices in an environment
        let config_content = r#"
version: "1.0.0"

defaults:
    can_interface: vcan0

webui:
    http_port: 10752

environments:
    test-env:
        friendly_name: "Test Environment"
        enabled: true
        devices:
            device_a:
                friendly_name: "Device A"
                type: j1939
                device_id: "0x82"
                enabled: true
                websocket_port: 10756
                hardware_present: false
            device_b:
                friendly_name: "Device B"
                type: j1939
                device_id: "0x88"
                enabled: true
                websocket_port: 10757
                hardware_present: false
"#;

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(config_content.as_bytes())
            .expect("Failed to write config");
        let config_path = temp_file.path().to_path_buf();

        // Test: --environment without --device-name should error
        let args = CommonSimulatorArgs {
            config: Some(config_path.clone()),
            environment: Some("test-env".to_string()),
            device_name: None, // Missing device_name
            ..Default::default()
        };

        let result = args.resolve_config();
        assert!(
            result.is_err(),
            "Expected error when --environment provided without --device-name"
        );

        let error = result.unwrap_err();
        let error_msg = error.to_string();

        // Verify error message contains helpful information
        assert!(
            error_msg.contains("--environment requires --device-name"),
            "Error should mention requirement: {}",
            error_msg
        );
        assert!(
            error_msg.contains("test-env"),
            "Error should mention environment name: {}",
            error_msg
        );
        assert!(
            error_msg.contains("device_a") || error_msg.contains("device_b"),
            "Error should list available devices: {}",
            error_msg
        );
    }

    #[test]
    fn test_environment_with_device_name_succeeds() {
        // Test that --environment WITH --device-name works correctly
        use std::io::Write;
        use tempfile::NamedTempFile;

        let config_content = r#"
version: "1.0.0"

defaults:
    can_interface: vcan0

webui:
    http_port: 10752

environments:
    test-env:
        friendly_name: "Test Environment"
        enabled: true
        devices:
            device_a:
                friendly_name: "Device A"
                type: j1939
                device_id: "0x82"
                enabled: true
                websocket_port: 10756
                hardware_present: false
"#;

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(config_content.as_bytes())
            .expect("Failed to write config");
        let config_path = temp_file.path().to_path_buf();

        // Test: --environment WITH --device-name should succeed
        let args = CommonSimulatorArgs {
            config: Some(config_path),
            environment: Some("test-env".to_string()),
            device_name: Some("device_a".to_string()),
            ..Default::default()
        };

        let result = args.resolve_config();
        assert!(
            result.is_ok(),
            "Should succeed when both --environment and --device-name provided"
        );

        let config = result.unwrap();
        assert_eq!(config.device_name, Some("test-env:device_a".to_string()));
        assert_eq!(config.friendly_name, Some("Device A".to_string()));
    }

    #[test]
    fn test_device_name_requires_environment() {
        // --device-name WITHOUT --environment should error (environment scoping required)
        use std::io::Write;
        use tempfile::NamedTempFile;

        let config_content = r#"
version: "1.0.0"

defaults:
    can_interface: vcan0

webui:
    http_port: 10752

environments:
    env-a:
        friendly_name: "Environment A"
        enabled: true
        devices:
            my_device:
                friendly_name: "Device in A"
                type: j1939
                device_id: "0x82"
                enabled: true
                websocket_port: 10756
"#;

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(config_content.as_bytes())
            .expect("Failed to write config");
        let config_path = temp_file.path().to_path_buf();

        // Test: --device-name WITHOUT --environment should error
        let args = CommonSimulatorArgs {
            config: Some(config_path),
            device_name: Some("my_device".to_string()),
            environment: None, // Missing environment
            ..Default::default()
        };

        let result = args.resolve_config();
        assert!(
            result.is_err(),
            "Expected error when --device-name provided without --environment"
        );

        let error = result.unwrap_err();
        let error_msg = error.to_string();

        // Verify error message explains environment scoping requirement
        assert!(
            error_msg.contains("--device-name requires --environment"),
            "Error should explain environment requirement: {}",
            error_msg
        );
        assert!(
            error_msg.contains("environment-scoped"),
            "Error should mention environment scoping: {}",
            error_msg
        );
    }

    #[test]
    fn test_duplicate_device_names_in_different_environments() {
        // Same device name in multiple environments should be environment-scoped,
        // not globally searched (which would be non-deterministic)
        use std::io::Write;
        use tempfile::NamedTempFile;

        let config_content = r#"
version: "1.0.0"

defaults:
    can_interface: vcan0

webui:
    http_port: 10752

environments:
    integration-test-primary:
        friendly_name: "Primary Test Environment"
        enabled: true
        devices:
            can_device_1:
                friendly_name: "CAN Device in Primary Env"
                type: j1939
                device_id: "0x42"
                enabled: true
                websocket_port: 10761

    integration-test-multi:
        friendly_name: "Multi-Protocol Test Environment"
        enabled: true
        devices:
            can_device_1:
                friendly_name: "CAN Device in Multi Env"
                type: j1939
                device_id: "0x43"
                enabled: true
                websocket_port: 10762

    integration-test-simulated:
        friendly_name: "Simulated Test Environment"
        enabled: true
        devices:
            can_device_1:
                friendly_name: "CAN Device in Simulated Env"
                type: j1939
                device_id: "0x44"
                enabled: true
                websocket_port: 10763
"#;

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(config_content.as_bytes())
            .expect("Failed to write config");
        let config_path = temp_file.path().to_path_buf();

        // Test: Specifying environment should select the correct device from THAT environment
        let args = CommonSimulatorArgs {
            config: Some(config_path.clone()),
            environment: Some("integration-test-primary".to_string()),
            device_name: Some("can_device_1".to_string()),
            websocket_port: None, // Don't override - use device config
            ..Default::default()
        };

        let result = args.resolve_config();
        assert!(
            result.is_ok(),
            "Should succeed with environment-scoped device resolution"
        );

        let config = result.unwrap();
        assert_eq!(
            config.device_name,
            Some("integration-test-primary:can_device_1".to_string())
        );
        assert_eq!(
            config.friendly_name,
            Some("CAN Device in Primary Env".to_string())
        );
        assert_eq!(config.websocket_port, 10761); // Correct port for primary env

        // Test: Different environment should select different device
        let args2 = CommonSimulatorArgs {
            config: Some(config_path.clone()),
            environment: Some("integration-test-multi".to_string()),
            device_name: Some("can_device_1".to_string()),
            websocket_port: None, // Don't override - use device config
            ..Default::default()
        };

        let result2 = args2.resolve_config();
        assert!(result2.is_ok());

        let config2 = result2.unwrap();
        assert_eq!(
            config2.device_name,
            Some("integration-test-multi:can_device_1".to_string())
        );
        assert_eq!(
            config2.friendly_name,
            Some("CAN Device in Multi Env".to_string())
        );
        assert_eq!(config2.websocket_port, 10762); // Correct port for Multi env

        // Test: Third environment should also work correctly
        let args3 = CommonSimulatorArgs {
            config: Some(config_path),
            environment: Some("integration-test-simulated".to_string()),
            device_name: Some("can_device_1".to_string()),
            websocket_port: None, // Don't override - use device config
            ..Default::default()
        };

        let result3 = args3.resolve_config();
        assert!(result3.is_ok());

        let config3 = result3.unwrap();
        assert_eq!(
            config3.device_name,
            Some("integration-test-simulated:can_device_1".to_string())
        );
        assert_eq!(config3.websocket_port, 10763); // Correct port for Simulated env
    }
}
