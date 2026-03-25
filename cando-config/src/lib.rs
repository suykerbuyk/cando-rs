#![warn(missing_docs)]
//! # Cando Configuration System
//!
//! This crate provides YAML-based configuration management for cando-rs tools and simulators.
//!
//! ## Features
//!
//! - **Single Source of Truth**: All device configurations in one YAML file
//! - **Named Devices**: Use friendly names like "Primary ECU" or "Transmission Controller"
//! - **Configuration Precedence**: CLI args -> Environment -> File -> Defaults
//! - **Automatic Discovery**: Search path for configuration files
//! - **Validation**: Comprehensive validation of all configuration values
//! - **Multi-Instance Support**: Run multiple devices from one configuration
//!
//! ## Quick Start
//!
//! ```
//! use cando_config::CandoConfig;
//!
//! // Load configuration with automatic search
//! let config = CandoConfig::load(None).unwrap();
//!
//! // Get a device by environment and key
//! if let Some(device) = config.get_device("test_env", "test_device") {
//!     let friendly = device.friendly_name.as_deref().unwrap_or("(unnamed)");
//!     let interface = device.interface.as_deref().unwrap_or("(inherited)");
//!     println!("Device: {} on {}", friendly, interface);
//! }
//!
//! // Get all enabled devices (returns tuples)
//! let enabled = config.enabled_devices();
//! println!("Found {} enabled devices", enabled.len());
//! for (env_name, device_key, device) in enabled {
//!     println!("  - {}:{}", env_name, device_key);
//! }
//! ```
//!
//! ## Configuration File Format
//!
//! ```yaml
//! version: "1.0.0"
//!
//! defaults:
//!   can_interface: vcan0
//!   protocol: j1939
//!   websocket_enabled: true
//!
//! webui:
//!   http_port: 10752
//!   auto_detect_devices: true
//!
//! environments:
//!   webui-simple:
//!     friendly_name: "WebUI Simple Test"
//!     devices:
//!       test_device:
//!         friendly_name: "Primary ECU"
//!         type: j1939
//!         device_id: "0x82"
//!         websocket_port: 10754
//!         variant: ecu
//! ```
//!
//! ## Search Path
//!
//! Configuration files are searched in this order:
//! 1. Explicit path via `--config` flag
//! 2. Environment variable: `CANDO_CONFIG`
//! 3. Current directory: `./cando.yaml`
//! 4. User config: `~/.config/cando/cando.yaml`
//! 5. System config: `/etc/cando/cando.yaml` (Unix only)
//! 6. Built-in defaults
//!
//! ## Configuration Precedence
//!
//! For each parameter, values are resolved with this precedence:
//! 1. CLI argument (highest priority)
//! 2. Environment variable
//! 3. Device-specific config
//! 4. Global defaults in config file
//! 5. Built-in defaults (lowest priority)

// Module declarations
mod config;
mod error;
mod loader;
mod validator;

// Public API exports
pub use config::{
    Defaults, DeviceConfig, DeviceType, Environment, LoggingConfig, NetworkConfig, CandoConfig,
    TestConfig, TestSourcesConfig, ValidationStatus, WebUiConfig,
};
pub use error::{ConfigError, Result};

// Re-export for convenience
pub use config::CandoConfig as Config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_exports() {
        // Verify all main types are accessible
        let _config: CandoConfig = CandoConfig::default();
        let _device_type: DeviceType = DeviceType::J1939;
        let _result: Result<()> = Ok(());
    }
}
