//! Core configuration data structures for cando-rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level cando configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandoConfig {
    /// Configuration version (for future compatibility)
    pub version: String,

    /// Global default values
    pub defaults: Defaults,

    /// WebUI configuration
    pub webui: WebUiConfig,

    /// Environment definitions (map of environment name to environment)
    #[serde(default)]
    pub environments: HashMap<String, Environment>,

    /// Test environment settings
    #[serde(default)]
    pub test: TestConfig,

    /// Network settings
    #[serde(default)]
    pub network: NetworkConfig,

    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
}

impl Default for CandoConfig {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            defaults: Defaults::default(),
            webui: WebUiConfig::default(),
            environments: HashMap::new(),
            test: TestConfig::default(),
            network: NetworkConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

/// Environment definition
///
/// Represents a named collection of devices for a specific physical or simulated setup.
/// Devices are contained directly within the environment as a HashMap.
///
/// # Hardware vs Simulated Devices
///
/// Whether a device is physical hardware or a simulator is determined **per-device**
/// via the `hardware_present` field on each `DeviceConfig`. This allows:
///
/// - **Pure hardware environments**: All devices have `hardware_present = true`
/// - **Pure simulation environments**: All devices have `hardware_present = false`
/// - **Mixed environments**: Some devices physical, some simulated (for comparison testing)
///
/// When an environment is activated:
/// - Devices with `hardware_present = false` -> Simulator processes launched (with WebSocket control)
/// - Devices with `hardware_present = true` -> Expected on CAN bus (no simulator launched)
///
/// # Examples
///
/// Pure hardware environment:
/// ```yaml
/// physical-lab:
///   friendly_name: "Physical Lab Hardware"
///   can_interface: can2
///   enabled: true
///   tags: [hardware, lab]
///   devices:
///     lab_device:
///       hardware_present: true
///       # ... other properties
/// ```
///
/// Pure simulation environment:
/// ```yaml
/// webui-test:
///   friendly_name: "WebUI Testing"
///   can_interface: vcan0
///   enabled: true
///   tags: [test, simulation]
///   devices:
///     test_device:
///       hardware_present: false
///       websocket_port: 10756
///       # ... other properties
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Human-readable environment name
    #[serde(default)]
    pub friendly_name: Option<String>,

    /// Map of device keys to device configurations
    pub devices: HashMap<String, DeviceConfig>,

    /// Optional description
    #[serde(default)]
    pub description: Option<String>,

    /// Physical location description
    #[serde(default)]
    pub location: Option<String>,

    /// CAN interface used by this environment
    #[serde(default)]
    pub can_interface: Option<String>,

    /// Whether this environment is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Searchable tags for filtering
    #[serde(default)]
    pub tags: Vec<String>,

    /// Environment creation date (ISO 8601 format recommended)
    #[serde(default)]
    pub created_date: Option<String>,

    /// Last modification date (ISO 8601 format recommended)
    #[serde(default)]
    pub last_modified: Option<String>,

    /// Environment owner/maintainer
    #[serde(default)]
    pub owner: Option<String>,

    /// Reference to test plan document
    #[serde(default)]
    pub test_plan: Option<String>,

    /// Additional notes
    #[serde(default)]
    pub notes: Option<String>,
}

/// Individual device configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// Human-readable name (e.g., "Primary ECU")
    #[serde(default)]
    pub friendly_name: Option<String>,

    /// Device type (j1939)
    #[serde(rename = "type")]
    pub device_type: DeviceType,

    /// CAN device ID (hex string like "0x82")
    pub device_id: String,

    /// CAN interface name (inherits from environment or defaults if not specified)
    #[serde(default)]
    pub interface: Option<String>,

    /// Protocol to use (inherits from environment or defaults if not specified)
    #[serde(default)]
    pub protocol: Option<String>,

    /// WebSocket port for this device (required for simulators, forbidden for physical hardware)
    #[serde(default)]
    pub websocket_port: Option<u16>,

    /// Whether this device is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Optional description
    #[serde(default)]
    pub description: Option<String>,

    /// Generic device variant/subtype/role
    ///
    /// This field allows any device type to specify variants or subtypes.
    ///
    /// Examples:
    /// - J1939: "ecu", "tcm", "bcm"
    #[serde(default)]
    pub variant: Option<String>,

    /// Voltage specification: device design property (28VDC or 600VDC)
    ///
    /// Valid values: "28VDC", "600VDC"
    /// Default: "600VDC"
    #[serde(default = "default_voltage_specification")]
    pub voltage_specification: String,

    /// Override global debug setting
    #[serde(default)]
    pub debug: Option<bool>,

    /// Override global no_console setting
    #[serde(default)]
    pub no_console: Option<bool>,

    // ===== Physical Hardware Metadata =====
    /// Whether physical hardware is present (false = simulator)
    #[serde(default)]
    pub hardware_present: bool,

    /// Hardware serial number
    #[serde(default)]
    pub serial_number: Option<String>,

    /// Firmware version
    #[serde(default)]
    pub firmware_version: Option<String>,

    /// Manufacture date (ISO 8601 format recommended)
    #[serde(default)]
    pub manufacture_date: Option<String>,

    /// Physical location description
    #[serde(default)]
    pub location: Option<String>,

    // ===== Validation Tracking =====
    /// Last hardware validation date (ISO 8601 format recommended)
    #[serde(default)]
    pub last_validated: Option<String>,

    /// Hardware validation status
    #[serde(default)]
    pub validation_status: Option<ValidationStatus>,

    /// Validation notes or failure reasons
    #[serde(default)]
    pub validation_notes: Option<String>,

    // ===== Organization =====
    /// Searchable tags for filtering
    #[serde(default)]
    pub tags: Vec<String>,

    /// Device owner/maintainer
    #[serde(default)]
    pub owner: Option<String>,

    /// Additional notes
    #[serde(default)]
    pub notes: Option<String>,
}

/// Global default values for all devices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaults {
    /// Default CAN interface name
    #[serde(default = "default_can_interface")]
    pub can_interface: String,

    /// Default protocol
    #[serde(default = "default_protocol")]
    pub protocol: String,

    /// Enable WebSocket by default
    #[serde(default = "default_true")]
    pub websocket_enabled: bool,

    /// Enable debug logging by default
    #[serde(default)]
    pub debug: bool,

    /// Disable console output by default
    #[serde(default)]
    pub no_console: bool,
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            can_interface: default_can_interface(),
            protocol: default_protocol(),
            websocket_enabled: true,
            debug: false,
            no_console: false,
        }
    }
}

/// WebUI-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebUiConfig {
    /// HTTP server port
    pub http_port: u16,

    /// Automatically detect devices
    #[serde(default = "default_true")]
    pub auto_detect_devices: bool,

    /// Display configuration mode
    #[serde(default = "default_display_config")]
    pub display_config: String,
}

impl Default for WebUiConfig {
    fn default() -> Self {
        Self {
            http_port: 10752,
            auto_detect_devices: true,
            display_config: "light".to_string(),
        }
    }
}

/// Hardware validation status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ValidationStatus {
    /// Device passed validation tests
    Passed,
    /// Device failed validation tests
    Failed,
    /// Validation pending or not yet run
    Pending,
    /// Validation status unknown
    #[default]
    Unknown,
}

impl std::fmt::Display for ValidationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationStatus::Passed => write!(f, "passed"),
            ValidationStatus::Failed => write!(f, "failed"),
            ValidationStatus::Pending => write!(f, "pending"),
            ValidationStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Device type enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    /// J1939 (SAE J1939 vehicle bus)
    J1939,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::J1939 => write!(f, "j1939"),
        }
    }
}

/// Test environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    /// CAN interface for tests
    #[serde(default = "default_test_interface")]
    pub can_interface: String,

    /// Base WebSocket port for tests
    #[serde(default = "default_test_port")]
    pub base_websocket_port: u16,

    /// Auto-increment device IDs in tests
    #[serde(default)]
    pub auto_increment_device_ids: bool,

    /// Only use enabled devices in tests
    #[serde(default)]
    pub enabled_devices_only: bool,

    /// Standard test source device ID (e.g., "0x0F")
    ///
    /// This is the SOURCE address that test messages are sent FROM.
    /// Must not conflict with any simulator device IDs.
    #[serde(default)]
    pub standard_source_device_id: Option<String>,

    /// Variant test source device IDs for multi-source testing
    ///
    /// Used for tests that validate message reception from different sources.
    /// Must not conflict with any simulator device IDs.
    #[serde(default)]
    pub variant_source_device_ids: Option<Vec<String>>,

    /// Test source metadata and validation settings
    #[serde(default)]
    pub sources: Option<TestSourcesConfig>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            can_interface: default_test_interface(),
            base_websocket_port: default_test_port(),
            auto_increment_device_ids: false,
            enabled_devices_only: false,
            standard_source_device_id: None,
            variant_source_device_ids: None,
            sources: None,
        }
    }
}

/// Test source configuration and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSourcesConfig {
    /// Description of standard test source
    #[serde(default)]
    pub standard_description: Option<String>,

    /// Description of variant test sources
    #[serde(default)]
    pub variant_description: Option<String>,

    /// Validate that test sources don't conflict with device IDs
    #[serde(default)]
    pub validate_no_conflicts: bool,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Bind address for network services
    #[serde(default = "default_bind_address")]
    pub bind_address: String,

    /// Maximum number of connections
    #[serde(default = "default_max_connections")]
    pub max_connections: usize,

    /// Connection timeout in milliseconds
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_address: default_bind_address(),
            max_connections: default_max_connections(),
            timeout_ms: default_timeout_ms(),
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Log format
    #[serde(default = "default_log_format")]
    pub format: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
        }
    }
}

// Default value functions
fn default_true() -> bool {
    true
}

fn default_test_interface() -> String {
    "vcan0".to_string()
}

fn default_test_port() -> u16 {
    10770
}

fn default_bind_address() -> String {
    "127.0.0.1".to_string()
}

fn default_max_connections() -> usize {
    100
}

fn default_timeout_ms() -> u64 {
    5000
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "pretty".to_string()
}

fn default_display_config() -> String {
    "light".to_string()
}

fn default_voltage_specification() -> String {
    "600VDC".to_string()
}

fn default_can_interface() -> String {
    "vcan0".to_string()
}

fn default_protocol() -> String {
    "j1939".to_string()
}
