//! Configuration loading with automatic search path resolution

use crate::{
    config::{DeviceConfig, DeviceType, CandoConfig},
    error::{ConfigError, Result},
};
use std::fs;
use std::path::{Path, PathBuf};

impl CandoConfig {
    /// Load configuration with automatic search path
    ///
    /// Search order:
    /// 1. Explicit path (if provided)
    /// 2. Environment variable: CANDO_CONFIG
    /// 3. Current directory: ./cando.yaml
    /// 4. User config: ~/.config/cando/cando.yaml
    /// 5. System config: /etc/cando/cando.yaml (Unix only)
    /// 6. Built-in defaults
    ///
    /// # Arguments
    ///
    /// * `explicit_path` - Optional explicit configuration file path
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    /// use std::path::PathBuf;
    ///
    /// // Load with automatic search
    /// let config = CandoConfig::load(None).unwrap();
    ///
    /// // Load from specific file
    /// let config = CandoConfig::load(Some(PathBuf::from("my-config.yaml")));
    /// ```
    pub fn load(explicit_path: Option<PathBuf>) -> Result<Self> {
        // 1. Try explicit path provided via CLI
        if let Some(path) = explicit_path {
            return Self::load_from_file(&path);
        }

        // 2. Try environment variable
        if let Ok(env_path) = std::env::var("CANDO_CONFIG") {
            let path = PathBuf::from(env_path);
            if path.exists() {
                return Self::load_from_file(&path);
            }
        }

        // 3. Try current directory
        let cwd_path = PathBuf::from("./cando.yaml");
        if cwd_path.exists() {
            return Self::load_from_file(&cwd_path);
        }

        // 4. Try user config directory
        if let Some(mut user_config) = dirs::config_dir() {
            user_config.push("cando");
            user_config.push("cando.yaml");
            if user_config.exists() {
                return Self::load_from_file(&user_config);
            }
        }

        // 5. Try system config (Unix/Linux only)
        #[cfg(unix)]
        {
            let system_path = PathBuf::from("/etc/cando/cando.yaml");
            if system_path.exists() {
                return Self::load_from_file(&system_path);
            }
        }

        // 6. Fall back to built-in defaults
        Ok(Self::default())
    }

    /// Load configuration from a specific file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The file contains invalid YAML
    /// - The configuration fails validation
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cando_config::CandoConfig;
    /// use std::path::Path;
    ///
    /// let config = CandoConfig::load_from_file(Path::new("cando.yaml"))?;
    /// # Ok::<(), cando_config::ConfigError>(())
    /// ```
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let contents =
            fs::read_to_string(path).map_err(|e| ConfigError::IoError(path.to_path_buf(), e))?;

        let config: CandoConfig = serde_yml::from_str(&contents)
            .map_err(|e| ConfigError::ParseError(path.to_path_buf(), e))?;

        // Validate configuration
        config.validate().map_err(|e| {
            eprintln!("Validation failed for {}: {}", path.display(), e);
            e
        })?;

        Ok(config)
    }

    /// Load configuration from a specific file, validating only for a specific environment
    ///
    /// This method loads the configuration file but only validates what's necessary
    /// for the specified environment. This avoids validation warnings/errors for
    /// unused environments (e.g., hardware environments when running virtual tests).
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    /// * `environment_name` - Name of the environment to validate
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The file contains invalid YAML
    /// - The environment doesn't exist
    /// - The environment configuration fails validation
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cando_config::CandoConfig;
    /// use std::path::Path;
    ///
    /// // Load config and validate only the webui-simple environment
    /// let config = CandoConfig::load_from_file_for_environment(
    ///     Path::new("cando.yaml"),
    ///     "webui-simple"
    /// )?;
    /// # Ok::<(), cando_config::ConfigError>(())
    /// ```
    pub fn load_from_file_for_environment(path: &Path, environment_name: &str) -> Result<Self> {
        let contents =
            fs::read_to_string(path).map_err(|e| ConfigError::IoError(path.to_path_buf(), e))?;

        let config: CandoConfig = serde_yml::from_str(&contents)
            .map_err(|e| ConfigError::ParseError(path.to_path_buf(), e))?;

        // Validate only for the specified environment
        config
            .validate_for_environment(environment_name)
            .map_err(|e| {
                eprintln!(
                    "Validation failed for {} (environment '{}'): {}",
                    path.display(),
                    environment_name,
                    e
                );
                e
            })?;

        Ok(config)
    }

    /// Get device by environment and device key
    ///
    /// # Arguments
    ///
    /// * `environment_name` - Name of the environment
    /// * `device_key` - Device key within the environment
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let device = config.get_device("webui-simple", "test_device");
    /// ```
    pub fn get_device(&self, environment_name: &str, device_key: &str) -> Option<&DeviceConfig> {
        self.environments
            .get(environment_name)?
            .devices
            .get(device_key)
    }

    /// Get all enabled devices across all environments
    ///
    /// Returns tuples of (environment_name, device_key, device_config)
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let enabled = config.enabled_devices();
    /// ```
    pub fn enabled_devices(&self) -> Vec<(&str, &str, &DeviceConfig)> {
        self.environments
            .iter()
            .filter(|(_, env)| env.enabled)
            .flat_map(|(env_name, env)| {
                env.devices.iter().filter(|(_, device)| device.enabled).map(
                    move |(device_key, device)| (env_name.as_str(), device_key.as_str(), device),
                )
            })
            .collect()
    }

    /// Get all enabled devices for a specific environment
    ///
    /// Returns tuples of (environment_name, device_key, device_config) for only
    /// the specified environment.
    ///
    /// # Arguments
    ///
    /// * `environment_name` - Name of the environment to filter by
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let devices = config.enabled_devices_for_environment("webui-simple");
    /// ```
    pub fn enabled_devices_for_environment(&self, environment_name: &str) -> Vec<(&str, &str, &DeviceConfig)> {
        self.environments
            .iter()
            .filter(|(env_key, env)| env_key.as_str() == environment_name && env.enabled)
            .flat_map(|(env_key, env)| {
                env.devices
                    .iter()
                    .filter(|(_, device)| device.enabled)
                    .map(move |(device_key, device)| {
                        (env_key.as_str(), device_key.as_str(), device)
                    })
            })
            .collect()
    }

    /// Get all devices of a specific type across all environments
    ///
    /// Returns tuples of (environment_name, device_key, device_config)
    ///
    /// # Arguments
    ///
    /// * `device_type` - The type of devices to filter for
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    /// use cando_config::DeviceType;
    ///
    /// let config = CandoConfig::default();
    /// let j1939_devices = config.devices_by_type(DeviceType::J1939);
    /// ```
    pub fn devices_by_type(&self, device_type: DeviceType) -> Vec<(&str, &str, &DeviceConfig)> {
        self.environments
            .iter()
            .flat_map(|(env_name, env)| {
                env.devices
                    .iter()
                    .filter(|(_, device)| device.device_type == device_type)
                    .map(move |(device_key, device)| {
                        (env_name.as_str(), device_key.as_str(), device)
                    })
            })
            .collect()
    }

    /// Get all devices across all environments (enabled and disabled)
    ///
    /// Returns tuples of (environment_name, device_key, device_config)
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let all = config.all_devices();
    /// ```
    pub fn all_devices(&self) -> Vec<(&str, &str, &DeviceConfig)> {
        self.environments
            .iter()
            .flat_map(|(env_name, env)| {
                env.devices.iter().map(move |(device_key, device)| {
                    (env_name.as_str(), device_key.as_str(), device)
                })
            })
            .collect()
    }

    /// Check if a device exists in a specific environment
    ///
    /// # Arguments
    ///
    /// * `environment_name` - Name of the environment
    /// * `device_key` - Device key to check
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let exists = config.has_device("webui-simple", "test_device");
    /// ```
    pub fn has_device(&self, environment_name: &str, device_key: &str) -> bool {
        self.get_device(environment_name, device_key).is_some()
    }

    /// Get the number of enabled devices across all environments
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let count = config.enabled_device_count();
    /// ```
    pub fn enabled_device_count(&self) -> usize {
        self.enabled_devices().len()
    }

    /// Get the total number of devices across all environments (including disabled)
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let count = config.device_count();
    /// ```
    pub fn device_count(&self) -> usize {
        self.all_devices().len()
    }

    /// Get environment by name
    ///
    /// # Arguments
    ///
    /// * `name` - Environment name (map key)
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let env = config.get_environment("physical-lab");
    /// ```
    pub fn get_environment(&self, name: &str) -> Option<&crate::config::Environment> {
        self.environments.get(name)
    }

    /// Get all enabled environments
    ///
    /// Returns tuples of (environment_name, environment_config)
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let enabled = config.enabled_environments();
    /// ```
    pub fn enabled_environments(&self) -> Vec<(&str, &crate::config::Environment)> {
        self.environments
            .iter()
            .filter(|(_, env)| env.enabled)
            .map(|(name, env)| (name.as_str(), env))
            .collect()
    }

    /// Get all devices for a specific environment
    ///
    /// Returns tuples of (device_key, device_config)
    ///
    /// # Arguments
    ///
    /// * `env_name` - Environment name
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let devices = config.environment_devices("webui-simple");
    /// ```
    pub fn environment_devices(&self, env_name: &str) -> Vec<(&str, &DeviceConfig)> {
        if let Some(env) = self.get_environment(env_name) {
            env.devices
                .iter()
                .map(|(key, device)| (key.as_str(), device))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get devices by tag across all environments
    ///
    /// Returns tuples of (environment_name, device_key, device_config)
    ///
    /// # Arguments
    ///
    /// * `tag` - Tag to filter by
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let hardware = config.devices_by_tag("hardware");
    /// ```
    pub fn devices_by_tag(&self, tag: &str) -> Vec<(&str, &str, &DeviceConfig)> {
        self.environments
            .iter()
            .flat_map(|(env_name, env)| {
                env.devices
                    .iter()
                    .filter(|(_, device)| device.tags.iter().any(|t| t == tag))
                    .map(move |(device_key, device)| {
                        (env_name.as_str(), device_key.as_str(), device)
                    })
            })
            .collect()
    }

    /// Get physical devices only (hardware_present = true) across all environments
    ///
    /// Returns tuples of (environment_name, device_key, device_config)
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let physical = config.physical_devices();
    /// ```
    pub fn physical_devices(&self) -> Vec<(&str, &str, &DeviceConfig)> {
        self.environments
            .iter()
            .flat_map(|(env_name, env)| {
                env.devices
                    .iter()
                    .filter(|(_, device)| device.hardware_present)
                    .map(move |(device_key, device)| {
                        (env_name.as_str(), device_key.as_str(), device)
                    })
            })
            .collect()
    }

    /// Get simulated devices only (hardware_present = false) across all environments
    ///
    /// Returns tuples of (environment_name, device_key, device_config)
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// let simulated = config.simulated_devices();
    /// ```
    pub fn simulated_devices(&self) -> Vec<(&str, &str, &DeviceConfig)> {
        self.environments
            .iter()
            .flat_map(|(env_name, env)| {
                env.devices
                    .iter()
                    .filter(|(_, device)| !device.hardware_present)
                    .map(move |(device_key, device)| {
                        (env_name.as_str(), device_key.as_str(), device)
                    })
            })
            .collect()
    }

    /// Create a new CandoConfig containing only the specified environment
    ///
    /// This is useful for running tools against a single environment.
    ///
    /// # Arguments
    ///
    /// * `environment_name` - Name of the environment to extract
    ///
    /// # Errors
    ///
    /// Returns an error if the environment doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::default();
    /// // let single_env = config.for_environment("webui-simple")?;
    /// # Ok::<(), cando_config::ConfigError>(())
    /// ```
    pub fn for_environment(&self, environment_name: &str) -> Result<Self> {
        let env = self
            .get_environment(environment_name)
            .ok_or_else(|| ConfigError::EnvironmentNotFound(environment_name.to_string()))?;

        let mut environments = std::collections::HashMap::new();
        environments.insert(environment_name.to_string(), env.clone());

        Ok(Self {
            version: self.version.clone(),
            defaults: self.defaults.clone(),
            webui: self.webui.clone(),
            environments,
            test: self.test.clone(),
            network: self.network.clone(),
            logging: self.logging.clone(),
        })
    }

    /// Resolve device configuration with inheritance applied
    ///
    /// Applies three-level inheritance: defaults -> environment -> device
    ///
    /// # Arguments
    ///
    /// * `environment_name` - Name of the environment
    /// * `device_key` - Device key within the environment
    ///
    /// # Returns
    ///
    /// A new DeviceConfig with all inherited values resolved
    pub fn resolve_device(&self, environment_name: &str, device_key: &str) -> Option<DeviceConfig> {
        let env = self.environments.get(environment_name)?;
        let device = env.devices.get(device_key)?;

        let mut resolved = device.clone();

        // Apply interface inheritance: device -> environment -> defaults
        if resolved.interface.is_none() {
            resolved.interface = env
                .can_interface
                .clone()
                .or_else(|| Some(self.defaults.can_interface.clone()));
        }

        // Apply protocol inheritance: device -> environment -> defaults
        if resolved.protocol.is_none() {
            resolved.protocol = Some(self.defaults.protocol.clone());
        }

        Some(resolved)
    }

    /// Parse device reference in "environment:device" format
    ///
    /// # Arguments
    ///
    /// * `reference` - Device reference string (e.g., "webui-simple:test_device")
    ///
    /// # Returns
    ///
    /// Tuple of (environment_name, device_key)
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Format is invalid (missing colon)
    /// - Either component is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use cando_config::CandoConfig;
    ///
    /// let (env, device) = CandoConfig::parse_device_reference("webui-simple:test_device").unwrap();
    /// assert_eq!(env, "webui-simple");
    /// assert_eq!(device, "test_device");
    /// ```
    pub fn parse_device_reference(reference: &str) -> Result<(String, String)> {
        let parts: Vec<&str> = reference.splitn(2, ':').collect();

        if parts.len() != 2 {
            return Err(ConfigError::InvalidDeviceReference(
                "Device reference must be in format 'environment:device'".to_string(),
            ));
        }

        let env = parts[0].trim();
        let device = parts[1].trim();

        if env.is_empty() || device.is_empty() {
            return Err(ConfigError::InvalidDeviceReference(
                "Environment and device name cannot be empty".to_string(),
            ));
        }

        Ok((env.to_string(), device.to_string()))
    }

    /// Get device by reference string (supports "environment:device" format)
    ///
    /// This method provides flexible device lookup supporting both explicit environment
    /// parameters and colon-separated format. It enforces environment isolation by
    /// requiring environment context.
    ///
    /// # Arguments
    ///
    /// * `reference` - Device reference ("env:device" or just "device" with explicit environment)
    /// * `explicit_environment` - Optional explicit environment name
    ///
    /// # Returns
    ///
    /// Reference to the device configuration
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - No environment context provided (neither explicit nor in reference)
    /// - Environment not found
    /// - Device not found in environment
    /// - Invalid reference format
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cando_config::CandoConfig;
    ///
    /// let config = CandoConfig::load(None).unwrap();
    ///
    /// // Using colon-separated format
    /// let device = config.get_device_by_reference("webui-simple:test_device", None).unwrap();
    ///
    /// // Using explicit environment
    /// let device = config.get_device_by_reference("test_device", Some("webui-simple")).unwrap();
    /// ```
    pub fn get_device_by_reference(
        &self,
        reference: &str,
        explicit_environment: Option<&str>,
    ) -> Result<&DeviceConfig> {
        let (env, device_key) = if let Some(env) = explicit_environment {
            // Explicit environment provided - reference is just device key
            (env.to_string(), reference.to_string())
        } else if reference.contains(':') {
            // Parse "environment:device" format
            Self::parse_device_reference(reference)?
        } else {
            // No environment context - ERROR
            return Err(ConfigError::InvalidDeviceReference(
                format!(
                    "Device reference '{}' requires environment context. Use --environment flag or 'environment:device' format",
                    reference
                )
            ));
        };

        self.get_device(&env, &device_key).ok_or_else(|| {
            ConfigError::DeviceNotFound(format!("{}:{}", env, device_key))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CandoConfig::default();
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.defaults.can_interface, "vcan0");
        assert_eq!(config.defaults.protocol, "j1939");
    }

    #[test]
    fn test_empty_environments() {
        let config = CandoConfig::default();
        assert_eq!(config.environments.len(), 0);
        assert_eq!(config.all_devices().len(), 0);
    }

    // Tests for device reference parsing
    #[test]
    fn test_parse_device_reference_valid() {
        let (env, device) = CandoConfig::parse_device_reference("webui-simple:test_device").unwrap();
        assert_eq!(env, "webui-simple");
        assert_eq!(device, "test_device");
    }

    #[test]
    fn test_parse_device_reference_with_multiple_colons() {
        // Should split on first colon only
        let (env, device) = CandoConfig::parse_device_reference("env:device:with:colons").unwrap();
        assert_eq!(env, "env");
        assert_eq!(device, "device:with:colons");
    }

    #[test]
    fn test_parse_device_reference_with_whitespace() {
        let (env, device) = CandoConfig::parse_device_reference(" webui-simple : test_device ").unwrap();
        assert_eq!(env, "webui-simple");
        assert_eq!(device, "test_device");
    }

    #[test]
    fn test_parse_device_reference_missing_colon() {
        let result = CandoConfig::parse_device_reference("just_device");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("environment:device"));
    }

    #[test]
    fn test_parse_device_reference_empty_environment() {
        let result = CandoConfig::parse_device_reference(":test_device");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_parse_device_reference_empty_device() {
        let result = CandoConfig::parse_device_reference("webui-simple:");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_parse_device_reference_only_whitespace() {
        let result = CandoConfig::parse_device_reference("  :  ");
        assert!(result.is_err());
    }

    // Helper function to create a test config with multiple environments
    fn create_test_config_for_reference_tests() -> CandoConfig {
        use crate::config::{Defaults, DeviceType, Environment, WebUiConfig};
        use std::collections::HashMap;

        let mut config = CandoConfig {
            version: "1.0.0".to_string(),
            defaults: Defaults::default(),
            webui: WebUiConfig::default(),
            environments: HashMap::new(),
            test: crate::config::TestConfig::default(),
            network: crate::config::NetworkConfig::default(),
            logging: crate::config::LoggingConfig::default(),
        };

        // Create webui-simple environment
        let mut webui_env = Environment {
            friendly_name: Some("WebUI Simple".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("vcan0".to_string()),
            enabled: true,
            tags: vec![],
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };

        let mut test_device = DeviceConfig {
            friendly_name: Some("Test Device".to_string()),
            device_type: DeviceType::J1939,
            device_id: "0x82".to_string(),
            interface: Some("vcan0".to_string()),
            protocol: Some("j1939".to_string()),
            websocket_port: Some(10756),
            enabled: true,
            description: None,
            variant: Some("ecu".to_string()),
            voltage_specification: "600VDC".to_string(),
            debug: None,
            no_console: None,
            hardware_present: false,
            serial_number: None,
            firmware_version: None,
            manufacture_date: None,
            location: None,
            last_validated: None,
            validation_status: None,
            validation_notes: None,
            tags: vec![],
            owner: None,
            notes: None,
        };

        webui_env.devices.insert("test_device".to_string(), test_device.clone());
        config.environments.insert("webui-simple".to_string(), webui_env);

        // Create physical-lab environment with same device key but different ID
        let mut physical_env = Environment {
            friendly_name: Some("Physical Lab".to_string()),
            devices: HashMap::new(),
            description: None,
            location: None,
            can_interface: Some("can2".to_string()),
            enabled: true,
            tags: vec![],
            created_date: None,
            last_modified: None,
            owner: None,
            test_plan: None,
            notes: None,
        };

        test_device.device_id = "0x83".to_string();
        test_device.interface = Some("can2".to_string());
        test_device.hardware_present = true;
        test_device.websocket_port = None;

        physical_env.devices.insert("test_device".to_string(), test_device);
        config.environments.insert("physical-lab".to_string(), physical_env);

        config
    }

    // Tests for get_device_by_reference
    #[test]
    fn test_get_device_by_reference_with_colon_format() {
        let config = create_test_config_for_reference_tests();
        let device = config.get_device_by_reference("webui-simple:test_device", None).unwrap();
        assert_eq!(device.device_id, "0x82");
        assert_eq!(device.interface.as_deref(), Some("vcan0"));
    }

    #[test]
    fn test_get_device_by_reference_with_explicit_environment() {
        let config = create_test_config_for_reference_tests();
        let device = config.get_device_by_reference("test_device", Some("webui-simple")).unwrap();
        assert_eq!(device.device_id, "0x82");
    }

    #[test]
    fn test_get_device_by_reference_disambiguates_same_key_different_env() {
        let config = create_test_config_for_reference_tests();

        // webui-simple:test_device should get device 0x82
        let device1 = config.get_device_by_reference("webui-simple:test_device", None).unwrap();
        assert_eq!(device1.device_id, "0x82");

        // physical-lab:test_device should get device 0x83
        let device2 = config.get_device_by_reference("physical-lab:test_device", None).unwrap();
        assert_eq!(device2.device_id, "0x83");
    }

    #[test]
    fn test_get_device_by_reference_no_context_errors() {
        let config = create_test_config_for_reference_tests();
        let result = config.get_device_by_reference("test_device", None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("environment context"));
        assert!(err.contains("--environment"));
    }

    #[test]
    fn test_get_device_by_reference_environment_not_found() {
        let config = create_test_config_for_reference_tests();
        let result = config.get_device_by_reference("nonexistent:test_device", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_get_device_by_reference_device_not_found() {
        let config = create_test_config_for_reference_tests();
        let result = config.get_device_by_reference("webui-simple:nonexistent", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_get_device_by_reference_explicit_env_overrides_colon() {
        let config = create_test_config_for_reference_tests();
        // When explicit environment is provided, reference is treated as device key
        // So this should look for device "webui-simple:test_device" (literal) in physical-lab
        let result = config.get_device_by_reference("webui-simple:test_device", Some("physical-lab"));
        // This should fail because there's no device with key "webui-simple:test_device" in physical-lab
        assert!(result.is_err());
    }
}
