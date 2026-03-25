//! Configuration validation logic

use crate::{
    config::{DeviceConfig, CandoConfig},
    error::{ConfigError, Result},
};
use std::collections::HashMap;

/// Supported configuration version
const SUPPORTED_VERSION: &str = "1.0.0";

/// Valid port range for WebSocket services
const MIN_WEBSOCKET_PORT: u16 = 10752;
const MAX_WEBSOCKET_PORT: u16 = 10799;

/// Valid protocols
const VALID_PROTOCOLS: &[&str] = &["j1939"];

impl CandoConfig {
    /// Validate the entire configuration
    ///
    /// Checks:
    /// - Configuration version compatibility
    /// - WebUI configuration
    /// - All devices in all environments (with inheritance resolved)
    /// - Hardware vs simulator rules (Rules 1.1, 1.2, 2.1, 2.2)
    /// - Unique WebSocket ports across all enabled devices
    /// - Unique device IDs per interface across all enabled devices
    /// - Valid device IDs and protocols
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::ValidationError` if any validation fails
    pub fn validate(&self) -> Result<()> {
        // Validate version
        self.validate_version()?;

        // Validate WebUI configuration
        self.validate_webui_config()?;

        // Validate all devices in all environments
        for (env_name, environment) in &self.environments {
            for device_key in environment.devices.keys() {
                // Resolve inheritance for validation
                let resolved_device =
                    self.resolve_device(env_name, device_key).ok_or_else(|| {
                        ConfigError::ValidationError(format!(
                            "Failed to resolve device '{}' in environment '{}'",
                            device_key, env_name
                        ))
                    })?;

                self.validate_device(env_name, device_key, &resolved_device)?;
            }
        }

        // Validate uniqueness constraints across all devices
        self.validate_unique_websocket_ports()?;
        self.validate_unique_device_ids()?;

        Ok(())
    }

    /// Validate configuration for a specific environment
    ///
    /// This validates only what's needed to run the specified environment:
    /// - Configuration version compatibility
    /// - WebUI configuration
    /// - Devices used in the environment
    /// - Uniqueness constraints for relevant devices
    ///
    /// Use this for normal operations to avoid noise from unused environments.
    /// Use `validate()` for full configuration validation (e.g., CI/CD).
    ///
    /// # Arguments
    ///
    /// * `environment_name` - The name of the environment to validate
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::ValidationError` if validation fails
    /// Returns `ConfigError::EnvironmentNotFound` if environment doesn't exist
    pub fn validate_for_environment(&self, environment_name: &str) -> Result<()> {
        // Validate version
        self.validate_version()?;

        // Validate WebUI configuration
        self.validate_webui_config()?;

        // Find the environment
        let environment = self
            .environments
            .get(environment_name)
            .ok_or_else(|| ConfigError::EnvironmentNotFound(environment_name.to_string()))?;

        // Validate only devices in this environment
        for device_key in environment.devices.keys() {
            // Resolve inheritance for validation
            let resolved_device = self
                .resolve_device(environment_name, device_key)
                .ok_or_else(|| {
                    ConfigError::ValidationError(format!(
                        "Failed to resolve device '{}' in environment '{}'",
                        device_key, environment_name
                    ))
                })?;

            self.validate_device(environment_name, device_key, &resolved_device)?;
        }

        // Validate uniqueness constraints
        // (We still need to check all devices to avoid port/ID conflicts)
        self.validate_unique_websocket_ports()?;
        self.validate_unique_device_ids()?;

        Ok(())
    }

    /// Validate configuration version
    fn validate_version(&self) -> Result<()> {
        if self.version != SUPPORTED_VERSION {
            return Err(ConfigError::UnsupportedVersion(
                self.version.clone(),
                SUPPORTED_VERSION.to_string(),
            ));
        }
        Ok(())
    }

    /// Validate WebUI configuration
    fn validate_webui_config(&self) -> Result<()> {
        // Validate WebUI HTTP port is in valid range (used for both HTTP and WebSocket)
        if !is_port_in_range(self.webui.http_port) {
            return Err(ConfigError::PortOutOfRange(self.webui.http_port));
        }

        // Validate display config
        let valid_display_configs = &["light", "dark", "auto"];
        if !valid_display_configs.contains(&self.webui.display_config.as_str()) {
            return Err(ConfigError::ValidationError(format!(
                "Invalid display_config '{}'. Expected one of: {}",
                self.webui.display_config,
                valid_display_configs.join(", ")
            )));
        }

        Ok(())
    }

    /// Validate a single device configuration (with inheritance already resolved)
    ///
    /// # Arguments
    ///
    /// * `env_name` - Environment name (for error messages)
    /// * `device_key` - Device key (for error messages)
    /// * `device` - Resolved device configuration (with inheritance applied)
    fn validate_device(
        &self,
        env_name: &str,
        device_key: &str,
        device: &DeviceConfig,
    ) -> Result<()> {
        let device_id = format!("{}:{}", env_name, device_key);

        // Validate friendly name if present
        if let Some(ref friendly_name) = device.friendly_name
            && friendly_name.trim().is_empty() {
                return Err(ConfigError::ValidationError(format!(
                    "Device '{}': friendly_name cannot be empty",
                    device_id
                )));
            }

        // Validate device ID format and range
        validate_device_id(&device.device_id, &device_id)?;

        // Validate interface (should be resolved by now)
        let interface = device.interface.as_ref().ok_or_else(|| {
            ConfigError::MissingRequiredField("interface".to_string(), device_id.clone())
        })?;

        if interface.trim().is_empty() {
            return Err(ConfigError::MissingRequiredField(
                "interface".to_string(),
                device_id.clone(),
            ));
        }

        // Validate protocol (should be resolved by now)
        let protocol = device.protocol.as_ref().ok_or_else(|| {
            ConfigError::MissingRequiredField("protocol".to_string(), device_id.clone())
        })?;

        validate_protocol(protocol, &device_id)?;

        // Rule 1.1 & 1.2: Hardware vs Simulator WebSocket Port validation
        if device.hardware_present {
            // Rule 1.1: Physical hardware cannot have websocket_port
            if device.websocket_port.is_some() {
                return Err(ConfigError::PhysicalHardwareWithWebSocket(
                    device_id.clone(),
                ));
            }
        } else {
            // Rule 1.2: Simulators must have websocket_port
            if device.websocket_port.is_none() {
                return Err(ConfigError::SimulatorMissingWebSocket(device_id.clone()));
            }

            // Validate WebSocket port range
            if let Some(port) = device.websocket_port
                && !is_port_in_range(port) {
                    return Err(ConfigError::PortOutOfRange(port));
                }
        }

        // Rule 2.1: Physical hardware must use physical interfaces (not vcan*)
        if device.hardware_present && is_virtual_interface(interface) {
            return Err(ConfigError::PhysicalHardwareOnVirtualInterface(
                device_id.clone(),
                interface.clone(),
            ));
        }

        // Rule 2.2: Simulators can use any interface (no validation needed - permissive)

        Ok(())
    }

    /// Validate WebSocket ports are unique within each environment
    ///
    /// Since only one environment is active at runtime, ports only need to be unique
    /// within each environment, not globally. This allows the same port to be reused
    /// across different environments for consistency (e.g., "test_device" always uses 10758).
    fn validate_unique_websocket_ports(&self) -> Result<()> {
        // Validate each environment independently
        for (env_name, environment) in &self.environments {
            let mut port_to_device: HashMap<u16, String> = HashMap::new();

            // Check WebUI ports don't conflict with devices in this environment
            port_to_device.insert(self.webui.http_port, "WebUI-HTTP".to_string());

            for (device_key, device) in &environment.devices {
                // Skip disabled devices - they won't be running so no port conflicts possible
                if !device.enabled {
                    continue;
                }

                // Skip devices without websocket_port (physical hardware)
                if let Some(port) = device.websocket_port {
                    let device_id = format!("{}:{}", env_name, device_key);

                    if let Some(existing_device) = port_to_device.get(&port) {
                        return Err(ConfigError::DuplicateWebSocketPort(
                            port,
                            existing_device.clone(),
                            device_id,
                        ));
                    }
                    port_to_device.insert(port, device_id);
                }
            }
        }

        Ok(())
    }

    /// Validate device IDs are unique per CAN interface within each environment
    ///
    /// Since only one environment is active at runtime, device IDs only need to be unique
    /// per interface within each environment, not globally. This allows the same device ID
    /// to be reused across different environments (e.g., same device in different test contexts).
    ///
    /// Checks that no two enabled devices on the same CAN interface have the same device ID
    /// within the same environment. Disabled devices are skipped since they won't be running.
    ///
    /// This prevents CAN ID collisions that would cause communication errors.
    fn validate_unique_device_ids(&self) -> Result<()> {
        // Validate each environment independently
        for (env_name, environment) in &self.environments {
            // Map of (interface, device_id) -> device_identifier within this environment
            let mut id_map: HashMap<(String, String), String> = HashMap::new();

            for (device_key, device) in &environment.devices {
                // Skip disabled devices - they won't be running so no ID conflicts possible
                if !device.enabled {
                    continue;
                }

                // Resolve interface (should have inheritance applied)
                let resolved_device = match self.resolve_device(env_name, device_key) {
                    Some(d) => d,
                    None => continue, // Skip if can't resolve
                };

                let interface = match resolved_device.interface {
                    Some(ref iface) => iface.clone(),
                    None => continue, // Skip if no interface (will be caught by validate_device)
                };

                let key = (interface.clone(), device.device_id.clone());
                let device_id = format!("{}:{}", env_name, device_key);

                if let Some(existing_device) = id_map.get(&key) {
                    return Err(ConfigError::DuplicateDeviceId(
                        device.device_id.clone(),
                        interface,
                        existing_device.clone(),
                        device_id,
                    ));
                }

                id_map.insert(key, device_id);
            }
        }

        Ok(())
    }
}

/// Validate device ID format and range
///
/// Accepts:
/// - Hex format with 0x prefix: "0x82", "0xAA"
/// - Decimal format: "130", "255"
///
/// Range: 0x00-0xFF (0-255)
fn validate_device_id(device_id: &str, device_identifier: &str) -> Result<()> {
    let id_str = device_id.trim();

    if id_str.is_empty() {
        return Err(ConfigError::MissingRequiredField(
            "device_id".to_string(),
            device_identifier.to_string(),
        ));
    }

    // Try parsing as hex (with 0x prefix)
    if id_str.starts_with("0x") || id_str.starts_with("0X") {
        match u8::from_str_radix(&id_str[2..], 16) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(ConfigError::InvalidDeviceId(
                    device_id.to_string(),
                    format!("Invalid hex format: {}", e),
                ));
            }
        }
    }

    // Try parsing as decimal
    match id_str.parse::<u8>() {
        Ok(_) => Ok(()),
        Err(e) => Err(ConfigError::InvalidDeviceId(
            device_id.to_string(),
            format!("Invalid format (expected hex '0xNN' or decimal): {}", e),
        )),
    }
}

/// Validate protocol value
fn validate_protocol(protocol: &str, device_identifier: &str) -> Result<()> {
    if protocol.trim().is_empty() {
        return Err(ConfigError::MissingRequiredField(
            "protocol".to_string(),
            device_identifier.to_string(),
        ));
    }

    if !VALID_PROTOCOLS.contains(&protocol) {
        return Err(ConfigError::InvalidProtocol(protocol.to_string()));
    }

    Ok(())
}

/// Check if port is in valid range
fn is_port_in_range(port: u16) -> bool {
    (MIN_WEBSOCKET_PORT..=MAX_WEBSOCKET_PORT).contains(&port)
}

/// Check if interface is a virtual CAN interface
fn is_virtual_interface(interface: &str) -> bool {
    interface.starts_with("vcan")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Defaults, DeviceType, Environment, WebUiConfig};

    /// Create a minimal valid test config
    fn create_test_config() -> CandoConfig {
        CandoConfig {
            version: "1.0.0".to_string(),
            defaults: Defaults::default(),
            webui: WebUiConfig::default(),
            environments: HashMap::new(),
            test: crate::config::TestConfig::default(),
            network: crate::config::NetworkConfig::default(),
            logging: crate::config::LoggingConfig::default(),
        }
    }

    /// Create a minimal valid simulator device
    fn create_simulator_device() -> DeviceConfig {
        DeviceConfig {
            friendly_name: Some("Test Device".to_string()),
            device_type: DeviceType::J1939,
            device_id: "0x82".to_string(),
            interface: Some("vcan0".to_string()),
            protocol: Some("j1939".to_string()),
            websocket_port: Some(10756),
            enabled: true,
            hardware_present: false,
            description: None,
            variant: None,
            voltage_specification: "600VDC".to_string(),
            debug: None,
            no_console: None,
            serial_number: None,
            firmware_version: None,
            manufacture_date: None,
            location: None,
            last_validated: None,
            validation_status: None,
            validation_notes: None,
            tags: Vec::new(),
            owner: None,
            notes: None,
        }
    }

    /// Create a minimal valid physical hardware device
    fn create_hardware_device() -> DeviceConfig {
        DeviceConfig {
            friendly_name: Some("Physical Device".to_string()),
            device_type: DeviceType::J1939,
            device_id: "0x82".to_string(),
            interface: Some("can0".to_string()),
            protocol: Some("j1939".to_string()),
            websocket_port: None, // Physical hardware has no websocket
            enabled: true,
            hardware_present: true,
            description: None,
            variant: None,
            voltage_specification: "600VDC".to_string(),
            debug: None,
            no_console: None,
            serial_number: Some("SN12345".to_string()),
            firmware_version: None,
            manufacture_date: None,
            location: None,
            last_validated: None,
            validation_status: None,
            validation_notes: None,
            tags: Vec::new(),
            owner: None,
            notes: None,
        }
    }

    #[test]
    fn test_valid_device_ids() {
        assert!(validate_device_id("0x82", "test").is_ok());
        assert!(validate_device_id("0xAA", "test").is_ok());
        assert!(validate_device_id("130", "test").is_ok());
        assert!(validate_device_id("255", "test").is_ok());
    }

    #[test]
    fn test_invalid_device_ids() {
        assert!(validate_device_id("", "test").is_err());
        assert!(validate_device_id("0xGG", "test").is_err());
        assert!(validate_device_id("256", "test").is_err());
        assert!(validate_device_id("invalid", "test").is_err());
    }

    #[test]
    fn test_valid_protocols() {
        assert!(validate_protocol("j1939", "test").is_ok());
    }

    #[test]
    fn test_invalid_protocols() {
        assert!(validate_protocol("", "test").is_err());
        assert!(validate_protocol("unknown", "test").is_err());
        assert!(validate_protocol("INVALID", "test").is_err());
    }

    #[test]
    fn test_port_range() {
        assert!(is_port_in_range(10752));
        assert!(is_port_in_range(10799));
        assert!(!is_port_in_range(10751));
        assert!(!is_port_in_range(10800));
    }

    #[test]
    fn test_virtual_interface_detection() {
        assert!(is_virtual_interface("vcan0"));
        assert!(is_virtual_interface("vcan1"));
        assert!(!is_virtual_interface("can0"));
        assert!(!is_virtual_interface("can1"));
    }

    #[test]
    fn test_rule_1_1_physical_hardware_no_websocket() {
        let mut config = create_test_config();
        let mut device = create_hardware_device();
        device.websocket_port = Some(10756); // Invalid: physical with websocket

        let mut env = Environment {
            friendly_name: Some("Test".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("can0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };

        env.devices.insert("test_device".to_string(), device);
        config.environments.insert("test_env".to_string(), env);

        let result = config.validate();
        assert!(result.is_err());
        if let Err(ConfigError::PhysicalHardwareWithWebSocket(_)) = result {
            // Expected error
        } else {
            panic!("Expected PhysicalHardwareWithWebSocket error");
        }
    }

    #[test]
    fn test_rule_1_2_simulator_requires_websocket() {
        let mut config = create_test_config();
        let mut device = create_simulator_device();
        device.websocket_port = None; // Invalid: simulator without websocket

        let mut env = Environment {
            friendly_name: Some("Test".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };

        env.devices.insert("test_device".to_string(), device);
        config.environments.insert("test_env".to_string(), env);

        let result = config.validate();
        assert!(result.is_err());
        if let Err(ConfigError::SimulatorMissingWebSocket(_)) = result {
            // Expected error
        } else {
            panic!("Expected SimulatorMissingWebSocket error");
        }
    }

    #[test]
    fn test_rule_2_1_physical_hardware_on_physical_interface() {
        let mut config = create_test_config();
        let device = create_hardware_device(); // Uses can0

        let mut env = Environment {
            friendly_name: Some("Test".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("can0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };

        env.devices.insert("test_device".to_string(), device);
        config.environments.insert("test_env".to_string(), env);

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_rule_2_1_physical_hardware_on_virtual_interface_error() {
        let mut config = create_test_config();
        let mut device = create_hardware_device();
        device.interface = Some("vcan0".to_string()); // Invalid: physical on vcan

        let mut env = Environment {
            friendly_name: Some("Test".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };

        env.devices.insert("test_device".to_string(), device);
        config.environments.insert("test_env".to_string(), env);

        let result = config.validate();
        assert!(result.is_err());
        if let Err(ConfigError::PhysicalHardwareOnVirtualInterface(_, _)) = result {
            // Expected error
        } else {
            panic!("Expected PhysicalHardwareOnVirtualInterface error");
        }
    }

    #[test]
    fn test_rule_2_2_simulator_on_any_interface() {
        let mut config = create_test_config();

        // Test simulator on vcan0
        let device1 = create_simulator_device(); // Uses vcan0
        let mut env1 = Environment {
            friendly_name: Some("Test1".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        env1.devices.insert("sim1".to_string(), device1);
        config.environments.insert("test_env1".to_string(), env1);

        // Test simulator on physical can0
        let mut device2 = create_simulator_device();
        device2.interface = Some("can0".to_string());
        device2.websocket_port = Some(10757);
        let mut env2 = Environment {
            friendly_name: Some("Test2".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("can0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        env2.devices.insert("sim2".to_string(), device2);
        config.environments.insert("test_env2".to_string(), env2);

        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_websocket_ports() {
        let mut config = create_test_config();

        // Test that duplicate ports WITHIN the same environment are rejected
        let device1 = create_simulator_device(); // Port 10756
        let device2 = create_simulator_device(); // Same port 10756

        let mut env1 = Environment {
            friendly_name: Some("Test1".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        // Both devices in the SAME environment with duplicate port
        env1.devices.insert("device1".to_string(), device1);
        env1.devices.insert("device2".to_string(), device2);
        config.environments.insert("env1".to_string(), env1);

        let result = config.validate();
        assert!(result.is_err());
        if let Err(ConfigError::DuplicateWebSocketPort(_, _, _)) = result {
            // Expected error
        } else {
            panic!("Expected DuplicateWebSocketPort error");
        }
    }

    #[test]
    fn test_duplicate_websocket_ports_across_environments_allowed() {
        let mut config = create_test_config();

        // Test that duplicate ports ACROSS different environments are allowed
        let device1 = create_simulator_device(); // Port 10756
        let mut env1 = Environment {
            friendly_name: Some("Test1".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        env1.devices.insert("device1".to_string(), device1);
        config.environments.insert("env1".to_string(), env1);

        let device2 = create_simulator_device(); // Same port 10756, different environment
        let mut env2 = Environment {
            friendly_name: Some("Test2".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        env2.devices.insert("device2".to_string(), device2);
        config.environments.insert("env2".to_string(), env2);

        // This should now succeed - same port in different environments is OK
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_device_ids_same_interface() {
        let mut config = create_test_config();

        // Test that duplicate device IDs WITHIN the same environment are rejected
        let device1 = create_simulator_device(); // ID 0x82, vcan0
        let mut device2 = create_simulator_device(); // Same ID 0x82, same vcan0
        device2.websocket_port = Some(10757); // Different port

        let mut env1 = Environment {
            friendly_name: Some("Test1".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        // Both devices in the SAME environment with duplicate device ID
        env1.devices.insert("device1".to_string(), device1);
        env1.devices.insert("device2".to_string(), device2);
        config.environments.insert("env1".to_string(), env1);

        let result = config.validate();
        assert!(result.is_err());
        if let Err(ConfigError::DuplicateDeviceId(_, _, _, _)) = result {
            // Expected error
        } else {
            panic!("Expected DuplicateDeviceId error");
        }
    }

    #[test]
    fn test_duplicate_device_ids_across_environments_allowed() {
        let mut config = create_test_config();

        // Test that duplicate device IDs ACROSS different environments are allowed
        let device1 = create_simulator_device(); // ID 0x82, vcan0
        let mut env1 = Environment {
            friendly_name: Some("Test1".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        env1.devices.insert("device1".to_string(), device1);
        config.environments.insert("env1".to_string(), env1);

        let mut device2 = create_simulator_device(); // Same ID 0x82, different environment
        device2.websocket_port = Some(10757); // Different port
        let mut env2 = Environment {
            friendly_name: Some("Test2".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        env2.devices.insert("device2".to_string(), device2);
        config.environments.insert("env2".to_string(), env2);

        // This should now succeed - same device ID in different environments is OK
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_same_device_id_different_interfaces() {
        let mut config = create_test_config();

        let device1 = create_simulator_device(); // ID 0x82, vcan0
        let mut env1 = Environment {
            friendly_name: Some("Test1".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        env1.devices.insert("device1".to_string(), device1);
        config.environments.insert("env1".to_string(), env1);

        let mut device2 = create_simulator_device(); // Same ID 0x82, but vcan1
        device2.interface = Some("vcan1".to_string());
        device2.websocket_port = Some(10757);
        let mut env2 = Environment {
            friendly_name: Some("Test2".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan1".to_string()),
            enabled: true,
            tags: Vec::new(),
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };
        env2.devices.insert("device2".to_string(), device2);
        config.environments.insert("env2".to_string(), env2);

        let result = config.validate();
        assert!(result.is_ok()); // Should be OK - different interfaces
    }
}
