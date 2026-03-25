//! Data loading for message metadata
//!
//! This module handles loading protocol metadata from cando-messages
//! and building lists of available messages for different device types.

use anyhow::Result;
use cando_config::CandoConfig;

use super::{DeviceInfo, MessageInfo};

/// Load devices from configuration (Non-interactive mode)
pub fn load_devices(config: &CandoConfig) -> Vec<DeviceInfo> {
    config
        .enabled_devices()
        .iter()
        .map(|(_, device_key, device)| DeviceInfo::from_config(device_key, device))
        .collect()
}

/// Load messages for a given device based on its type and protocol
pub fn load_messages_for_device(_device: &DeviceInfo) -> Result<Vec<MessageInfo>> {
    // All devices use J1939 protocol in cando-rs
    load_j1939_messages()
}

/// Load J1939 messages (commonly used subset)
fn load_j1939_messages() -> Result<Vec<MessageInfo>> {
    use cando_messages::j1939;

    // Common J1939 messages that are most useful for testing
    let messages = vec![
        // Engine/ECU messages
        MessageInfo::from_metadata(&j1939::EEC1_METADATA, "j1939"),
        MessageInfo::from_metadata(&j1939::EEC2_METADATA, "j1939"),
        // Ambient Conditions
        MessageInfo::from_metadata(&j1939::AMB3_METADATA, "j1939"),
        // Engine Temperature
        MessageInfo::from_metadata(&j1939::ET1_METADATA, "j1939"),
        // Engine Fluid Level/Pressure
        MessageInfo::from_metadata(&j1939::EFLP1_METADATA, "j1939"),
    ];

    // Note: J1939 has 100+ messages. We're showing the most commonly used ones.
    // Future enhancement: Add search/filter to browse all J1939 messages.

    Ok(messages)
}

/// Get display name for a device type + protocol combination
pub fn get_protocol_display_name(_device: &DeviceInfo) -> String {
    "J1939 ECU".to_string()
}

/// Filter messages by search text
pub fn filter_messages(messages: &[MessageInfo], filter: &str) -> Vec<MessageInfo> {
    if filter.is_empty() {
        return messages.to_vec();
    }

    let filter_lower = filter.to_lowercase();
    messages
        .iter()
        .filter(|msg| {
            msg.name.to_lowercase().contains(&filter_lower)
                || msg.comment.to_lowercase().contains(&filter_lower)
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cando_config::DeviceType;

    fn create_test_device(device_type: DeviceType, protocol: &str) -> DeviceInfo {
        DeviceInfo {
            name: "test".to_string(),
            friendly_name: "Test Device".to_string(),
            device_type,
            protocol: protocol.to_string(),
            device_id: "0x82".to_string(),
            interface: "vcan0".to_string(),
        }
    }

    #[test]
    fn test_load_j1939() {
        let device = create_test_device(DeviceType::J1939, "j1939");
        let messages = load_messages_for_device(&device).unwrap();
        assert!(!messages.is_empty());
        assert!(messages.iter().any(|m| m.name.contains("EEC1")));
    }

    #[test]
    fn test_filter_messages() {
        let messages = vec![
            MessageInfo {
                name: "EEC1".to_string(),
                can_id: 0x18F00400,
                dlc: 8,
                signals: vec![],
                comment: "Electronic Engine Controller 1".to_string(),
                protocol: "j1939".to_string(),
            },
            MessageInfo {
                name: "ET1".to_string(),
                can_id: 0x18FEEE00,
                dlc: 8,
                signals: vec![],
                comment: "Engine Temperature 1".to_string(),
                protocol: "j1939".to_string(),
            },
        ];

        let filtered = filter_messages(&messages, "EEC");
        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].name.contains("EEC"));
    }
}
