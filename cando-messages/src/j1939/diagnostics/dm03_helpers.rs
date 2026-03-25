//! DM03 Helper Functions and Extensions
//!
//! Provides convenience methods and factory functions for working with DM03
//! (Diagnostic Data Clear/Reset) messages in fleet management and testing scenarios.

use super::DiagnosticMessageType;
use crate::common::{DecodeError, DeviceId};
use crate::j1939::DM03;

/// DM03 Helper Methods
///
/// Extension trait providing convenience methods for DM03 message handling
/// in fleet management, diagnostic workflows, and testing scenarios.
///
/// # Example
/// ```
/// use cando_messages::j1939::diagnostics::DM03Helpers;
/// use cando_messages::j1939::DM03;
/// use cando_messages::common::DeviceId;
///
/// // Create a clear command
/// let clear_cmd = DM03::create_clear_command(DeviceId::from(0x82));
/// assert!(clear_cmd.is_valid_fleet_command());
/// assert!(clear_cmd.targets_device(DeviceId::from(0x82)));
///
/// // Get workflow context
/// let workflow = clear_cmd.maintenance_workflow_context();
/// assert!(workflow.contains("DTC_CLEAR_REQUEST"));
/// ```
pub trait DM03Helpers {
    /// Create a diagnostic clear command for a specific device
    ///
    /// Factory method that creates a DM03 message configured to clear DTCs
    /// on the specified target device.
    ///
    /// # Arguments
    /// * `device_id` - Target device for the clear command
    ///
    /// # Returns
    /// DM03 message configured to clear DTCs on the specified device
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM03Helpers;
    /// use cando_messages::j1939::DM03;
    /// use cando_messages::common::DeviceId;
    ///
    /// let clear_cmd = DM03::create_clear_command(DeviceId::from(0x42));
    /// assert_eq!(clear_cmd.device_id, DeviceId::from(0x42));
    /// ```
    fn create_clear_command(device_id: DeviceId) -> Self;

    /// Check if this is a valid fleet management command
    ///
    /// Validates that the DM03 message is properly formed for fleet operations.
    /// All DM03 messages are valid by definition in the current implementation,
    /// but this method provides a hook for future validation logic.
    ///
    /// # Returns
    /// `true` for all valid DM03 messages
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM03Helpers;
    /// use cando_messages::j1939::DM03;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM03::create_clear_command(DeviceId::from(0x59));
    /// assert!(cmd.is_valid_fleet_command());
    /// ```
    fn is_valid_fleet_command(&self) -> bool;

    /// Check if this command targets a specific device
    ///
    /// Useful for filtering and routing diagnostic commands in multi-device
    /// fleet management scenarios.
    ///
    /// # Arguments
    /// * `device_id` - Device to check against
    ///
    /// # Returns
    /// `true` if this command targets the specified device
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM03Helpers;
    /// use cando_messages::j1939::DM03;
    /// use cando_messages::common::DeviceId;
    ///
    /// let device_id = DeviceId::from(0x82);
    /// let cmd = DM03::create_clear_command(device_id);
    ///
    /// assert!(cmd.targets_device(device_id));
    /// assert!(!cmd.targets_device(DeviceId::from(0x59)));
    /// ```
    fn targets_device(&self, device_id: DeviceId) -> bool;

    /// Get expected response message types after clear command
    ///
    /// After a DM03 clear command is sent, the target device should respond with:
    /// - DM01 (Active DTCs) - showing cleared state
    /// - DM02 (Previously Active DTCs) - updated with cleared DTCs
    ///
    /// This method returns the diagnostic message types that should be monitored
    /// for responses after sending a clear command.
    ///
    /// # Returns
    /// Vector of diagnostic message types that should respond to this command
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::{DM03Helpers, DiagnosticMessageType};
    /// use cando_messages::j1939::DM03;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM03::create_clear_command(DeviceId::from(0x42));
    /// let responses = cmd.expected_response_types();
    ///
    /// assert_eq!(responses.len(), 2);
    /// assert!(responses.contains(&DiagnosticMessageType::DM01));
    /// assert!(responses.contains(&DiagnosticMessageType::DM02));
    /// ```
    fn expected_response_types(&self) -> Vec<DiagnosticMessageType>;

    /// Get human-readable command description
    ///
    /// Returns a descriptive string suitable for maintenance logs and user interfaces.
    ///
    /// # Returns
    /// Command description string
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM03Helpers;
    /// use cando_messages::j1939::DM03;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM03::create_clear_command(DeviceId::from(0x82));
    /// let description = cmd.get_command_description();
    ///
    /// assert!(description.contains("Clear/Reset DTCs"));
    /// assert!(description.contains("0x82"));
    /// ```
    fn get_command_description(&self) -> String;

    /// Get the target device for this command
    ///
    /// Returns the device ID that this clear command targets.
    ///
    /// # Returns
    /// Target device ID
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM03Helpers;
    /// use cando_messages::j1939::DM03;
    /// use cando_messages::common::DeviceId;
    ///
    /// let device_id = DeviceId::from(0x82);
    /// let cmd = DM03::create_clear_command(device_id);
    ///
    /// assert_eq!(cmd.target_device(), device_id);
    /// ```
    fn target_device(&self) -> DeviceId;

    /// Generate maintenance workflow context string
    ///
    /// Creates a structured workflow identifier for logging, tracking, and
    /// integration with fleet management systems.
    ///
    /// Format: `DTC_CLEAR_REQUEST|DEVICE:XX|ACTION:RESET_DIAGNOSTIC_DATA|TIMESTAMP:YYYY-MM-DD HH:MM:SS`
    ///
    /// # Returns
    /// Workflow context string suitable for logging and tracking
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM03Helpers;
    /// use cando_messages::j1939::DM03;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM03::create_clear_command(DeviceId::from(0x82));
    /// let context = cmd.maintenance_workflow_context();
    ///
    /// assert!(context.contains("DTC_CLEAR_REQUEST"));
    /// assert!(context.contains("DEVICE:82"));
    /// assert!(context.contains("ACTION:RESET_DIAGNOSTIC_DATA"));
    /// assert!(context.contains("TIMESTAMP:"));
    /// ```
    fn maintenance_workflow_context(&self) -> String;

    /// Decode with enhanced error context (alias for decode)
    ///
    /// Provided for backward compatibility with test code that expects
    /// this method name. Internally just calls the standard decode method.
    ///
    /// # Arguments
    /// * `can_id` - J1939 CAN ID
    /// * `data` - CAN data bytes
    ///
    /// # Returns
    /// Decoded DM03 message or error
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM03Helpers;
    /// use cando_messages::j1939::DM03;
    /// use cando_messages::common::DeviceId;
    ///
    /// let original = DM03::create_clear_command(DeviceId::from(0x42));
    /// let (can_id, data) = original.encode().unwrap();
    ///
    /// let decoded = DM03::decode_real(can_id, &data).unwrap();
    /// assert_eq!(decoded.device_id, original.device_id);
    /// ```
    fn decode_real(can_id: u32, data: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

impl DM03Helpers for DM03 {
    fn create_clear_command(device_id: DeviceId) -> Self {
        DM03 { device_id }
    }

    fn is_valid_fleet_command(&self) -> bool {
        // All DM03 messages are valid fleet commands
        // Could add additional validation here if needed:
        // - Check device ID is not broadcast (0xFF)
        // - Check device ID is in valid range
        // - Check device ID is not null address (0xFE)
        //
        // For now, accept all as valid.
        true
    }

    fn targets_device(&self, device_id: DeviceId) -> bool {
        self.device_id == device_id
    }

    fn expected_response_types(&self) -> Vec<DiagnosticMessageType> {
        // After DM03 clear command, expect updated diagnostics from target device:
        // - DM01 (Active DTCs) should show cleared state
        // - DM02 (Previously Active DTCs) should be updated with cleared faults
        vec![DiagnosticMessageType::DM01, DiagnosticMessageType::DM02]
    }

    fn get_command_description(&self) -> String {
        format!(
            "DM03 Clear/Reset DTCs for device 0x{:02X}",
            self.device_id.as_u8()
        )
    }

    fn target_device(&self) -> DeviceId {
        self.device_id
    }

    fn maintenance_workflow_context(&self) -> String {
        use std::time::SystemTime;

        // Get current timestamp as seconds since UNIX epoch
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let seconds = now.as_secs();

        // Simple timestamp format: seconds since epoch
        // For production, consider using a proper datetime library
        format!(
            "DTC_CLEAR_REQUEST|DEVICE:{:02X}|ACTION:RESET_DIAGNOSTIC_DATA|TIMESTAMP:{}",
            self.device_id.as_u8(),
            seconds
        )
    }

    fn decode_real(can_id: u32, data: &[u8]) -> Result<Self, DecodeError> {
        // Just call the standard decode method
        DM03::decode(can_id, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_clear_command() {
        let device_id = DeviceId::from(0x82);
        let cmd = DM03::create_clear_command(device_id);
        assert_eq!(cmd.device_id, device_id);
    }

    #[test]
    fn test_create_clear_command_various_devices() {
        let devices = vec![
            DeviceId::from(0x42),
            DeviceId::from(0x59),
            DeviceId::from(0x82),
            DeviceId::from(0x80),
        ];

        for device_id in devices {
            let cmd = DM03::create_clear_command(device_id);
            assert_eq!(cmd.device_id, device_id);
        }
    }

    #[test]
    fn test_is_valid_fleet_command() {
        let cmd = DM03::create_clear_command(DeviceId::from(0x42));
        assert!(cmd.is_valid_fleet_command());

        // Test various device IDs
        let cmd2 = DM03::create_clear_command(DeviceId::from(0x00));
        assert!(cmd2.is_valid_fleet_command());

        let cmd3 = DM03::create_clear_command(DeviceId::from(0xFF));
        assert!(cmd3.is_valid_fleet_command());
    }

    #[test]
    fn test_targets_device() {
        let device_id = DeviceId::from(0x59);
        let cmd = DM03::create_clear_command(device_id);

        assert!(cmd.targets_device(device_id));
        assert!(!cmd.targets_device(DeviceId::from(0x82)));
        assert!(!cmd.targets_device(DeviceId::from(0x42)));
    }

    #[test]
    fn test_expected_response_types() {
        let cmd = DM03::create_clear_command(DeviceId::from(0x42));
        let responses = cmd.expected_response_types();

        assert_eq!(responses.len(), 2);
        assert!(responses.contains(&DiagnosticMessageType::DM01));
        assert!(responses.contains(&DiagnosticMessageType::DM02));
    }

    #[test]
    fn test_maintenance_workflow_context() {
        let cmd = DM03::create_clear_command(DeviceId::from(0x82));
        let context = cmd.maintenance_workflow_context();

        assert!(context.contains("DTC_CLEAR_REQUEST"));
        assert!(context.contains("DEVICE:82"));
        assert!(context.contains("ACTION:RESET_DIAGNOSTIC_DATA"));
    }

    #[test]
    fn test_maintenance_workflow_context_format() {
        let devices = vec![
            (DeviceId::from(0x00), "DEVICE:00"),
            (DeviceId::from(0x82), "DEVICE:82"),
            (DeviceId::from(0xFF), "DEVICE:FF"),
        ];

        for (device_id, expected_device_str) in devices {
            let cmd = DM03::create_clear_command(device_id);
            let context = cmd.maintenance_workflow_context();
            assert!(context.contains(expected_device_str));
        }
    }

    #[test]
    fn test_get_command_description() {
        let cmd = DM03::create_clear_command(DeviceId::from(0x82));
        let description = cmd.get_command_description();

        assert!(description.contains("Clear/Reset DTCs"));
        assert!(description.contains("0x82"));
        assert!(description.contains("DM03"));
    }

    #[test]
    fn test_target_device() {
        let device_id = DeviceId::from(0x8A);
        let cmd = DM03::create_clear_command(device_id);

        assert_eq!(cmd.target_device(), device_id);
    }

    #[test]
    fn test_target_device_various() {
        let devices = vec![
            DeviceId::from(0x00),
            DeviceId::from(0x82),
            DeviceId::from(0x42),
            DeviceId::from(0xFF),
        ];

        for device_id in devices {
            let cmd = DM03::create_clear_command(device_id);
            assert_eq!(cmd.target_device(), device_id);
        }
    }

    #[test]
    fn test_maintenance_workflow_context_includes_timestamp() {
        let cmd = DM03::create_clear_command(DeviceId::from(0x82));
        let context = cmd.maintenance_workflow_context();

        assert!(context.contains("TIMESTAMP:"));
        // Verify timestamp is present (seconds since epoch format)
        // Extract the timestamp value after "TIMESTAMP:"
        if let Some(ts_start) = context.find("TIMESTAMP:") {
            let ts_part = &context[ts_start + 10..]; // Skip "TIMESTAMP:"
            let ts_value: String = ts_part.chars().take_while(|c| c.is_numeric()).collect();
            assert!(
                !ts_value.is_empty(),
                "Timestamp should contain numeric value"
            );
            assert!(
                ts_value.parse::<u64>().is_ok(),
                "Timestamp should be a valid u64"
            );
        } else {
            panic!("Context should contain TIMESTAMP:");
        }
    }

    #[test]
    fn test_decode_real() {
        let original = DM03::create_clear_command(DeviceId::from(0x42));
        let (can_id, data) = original.encode().unwrap();

        let decoded = DM03::decode_real(can_id, &data).unwrap();
        assert_eq!(decoded.device_id, original.device_id);
    }

    #[test]
    fn test_decode_real_matches_decode() {
        let device_id = DeviceId::from(0x82);
        let original = DM03::create_clear_command(device_id);
        let (can_id, data) = original.encode().unwrap();

        let decoded_real = DM03::decode_real(can_id, &data).unwrap();
        let decoded_std = DM03::decode(can_id, &data).unwrap();

        assert_eq!(decoded_real.device_id, decoded_std.device_id);
    }

    #[test]
    fn test_roundtrip_with_helpers() {
        let device_id = DeviceId::from(0x59);
        let original = DM03::create_clear_command(device_id);

        // Encode
        let (can_id, data) = original.encode().unwrap();

        // Verify CAN ID format (DM03 BASE_CAN_ID + device)
        assert_eq!(can_id & 0xFFFFFF00, DM03::BASE_CAN_ID);

        // Decode
        let decoded = DM03::decode_real(can_id, &data).unwrap();

        // Verify
        assert_eq!(decoded.device_id, original.device_id);
        assert!(decoded.targets_device(device_id));
        assert!(decoded.is_valid_fleet_command());
    }
}
