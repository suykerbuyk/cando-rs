//! Command Message Helper Functions and Extensions
//!
//! Provides convenience methods and factory functions for working with J1939-73
//! command messages (DM11, DM13, DM22) used for diagnostic control and network
//! management operations.

use super::DiagnosticMessageType;
use crate::common::{DecodeError, DeviceId};
use crate::j1939::{DM11, DM13, DM22};

// ============================================================================
// DM11 Helper Trait - Diagnostic Data Clear/Reset for Active DTCs
// ============================================================================

/// DM11 Helper Methods
///
/// Extension trait providing convenience methods for DM11 (Clear Active DTCs)
/// message handling in diagnostic workflows and testing scenarios.
///
/// DM11 is a broadcast command message that instructs a device to clear all
/// active diagnostic trouble codes. It is a zero-length message (DLC=0) with
/// no data bytes - only the device ID in the CAN ID.
///
/// # Example
/// ```
/// use cando_messages::j1939::diagnostics::DM11Helpers;
/// use cando_messages::j1939::DM11;
/// use cando_messages::common::DeviceId;
///
/// // Create a clear active DTCs command
/// let clear_cmd = DM11::create_clear_active_command(DeviceId::from(0x42));
/// assert!(clear_cmd.is_valid_command());
/// assert!(clear_cmd.targets_device(DeviceId::from(0x42)));
///
/// // Get workflow context
/// let workflow = clear_cmd.diagnostic_workflow_context();
/// assert!(workflow.contains("CLEAR_ACTIVE_DTCS"));
/// ```
pub trait DM11Helpers {
    /// Create a clear active DTCs command for a specific device
    ///
    /// Factory method that creates a DM11 message configured to clear all
    /// active DTCs on the specified target device.
    ///
    /// # Arguments
    /// * `device_id` - Target device for the clear command
    ///
    /// # Returns
    /// DM11 message configured to clear active DTCs on the specified device
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM11Helpers;
    /// use cando_messages::j1939::DM11;
    /// use cando_messages::common::DeviceId;
    ///
    /// let clear_cmd = DM11::create_clear_active_command(DeviceId::from(0x59));
    /// assert_eq!(clear_cmd.device_id, DeviceId::from(0x59));
    /// ```
    fn create_clear_active_command(device_id: DeviceId) -> Self;

    /// Check if this is a valid command
    ///
    /// Validates that the DM11 message is properly formed. All DM11 messages
    /// are valid by definition since they have no data bytes, but this method
    /// provides a hook for future validation logic.
    ///
    /// # Returns
    /// `true` for all valid DM11 messages
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM11Helpers;
    /// use cando_messages::j1939::DM11;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM11::create_clear_active_command(DeviceId::from(0x82));
    /// assert!(cmd.is_valid_command());
    /// ```
    fn is_valid_command(&self) -> bool;

    /// Check if this command targets a specific device
    ///
    /// Useful for filtering and routing diagnostic commands in multi-device
    /// scenarios.
    ///
    /// # Arguments
    /// * `device_id` - Device to check against
    ///
    /// # Returns
    /// `true` if this command targets the specified device
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM11Helpers;
    /// use cando_messages::j1939::DM11;
    /// use cando_messages::common::DeviceId;
    ///
    /// let device_id = DeviceId::from(0x42);
    /// let cmd = DM11::create_clear_active_command(device_id);
    ///
    /// assert!(cmd.targets_device(device_id));
    /// assert!(!cmd.targets_device(DeviceId::from(0x59)));
    /// ```
    fn targets_device(&self, device_id: DeviceId) -> bool;

    /// Get expected response message types after clear command
    ///
    /// After a DM11 clear command is sent, the target device should respond with
    /// DM01 showing no active DTCs (empty message).
    ///
    /// # Returns
    /// Vector of expected diagnostic message types in response
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::{DM11Helpers, DiagnosticMessageType};
    /// use cando_messages::j1939::DM11;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM11::create_clear_active_command(DeviceId::from(0x42));
    /// let responses = cmd.get_expected_response_types();
    /// assert!(responses.contains(&DiagnosticMessageType::DM01));
    /// ```
    fn get_expected_response_types(&self) -> Vec<DiagnosticMessageType>;

    /// Get diagnostic workflow context string
    ///
    /// Returns a human-readable string describing the diagnostic workflow
    /// context for this command, useful for logging and debugging.
    ///
    /// # Returns
    /// String describing the workflow context
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM11Helpers;
    /// use cando_messages::j1939::DM11;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM11::create_clear_active_command(DeviceId::from(0x59));
    /// let context = cmd.diagnostic_workflow_context();
    /// assert!(context.contains("CLEAR_ACTIVE_DTCS"));
    /// ```
    fn diagnostic_workflow_context(&self) -> String;
}

impl DM11Helpers for DM11 {
    fn create_clear_active_command(device_id: DeviceId) -> Self {
        DM11 { device_id }
    }

    fn is_valid_command(&self) -> bool {
        // DM11 is always valid - it's a zero-length message
        true
    }

    fn targets_device(&self, device_id: DeviceId) -> bool {
        self.device_id == device_id
    }

    fn get_expected_response_types(&self) -> Vec<DiagnosticMessageType> {
        vec![
            DiagnosticMessageType::DM01, // Should show no active DTCs
        ]
    }

    fn diagnostic_workflow_context(&self) -> String {
        format!(
            "CLEAR_ACTIVE_DTCS: Device={:?}, Command=DM11",
            self.device_id
        )
    }
}

// ============================================================================
// DM22 Helper Trait - Individual Clear/Reset of Active and Previously Active DTC
// ============================================================================

/// DM22 Helper Methods
///
/// Extension trait providing convenience methods for DM22 (Individual DTC Clear)
/// message handling. DM22 allows targeted clearing of specific DTCs by SPN/FMI,
/// unlike DM11 which clears all DTCs.
///
/// # Example
/// ```
/// use cando_messages::j1939::diagnostics::DM22Helpers;
/// use cando_messages::j1939::DM22;
/// use cando_messages::common::DeviceId;
///
/// // Create command to clear specific DTC (SPN=123, FMI=4)
/// let clear_cmd = DM22::create_individual_clear(DeviceId::from(0x42), 123, 4);
/// assert!(clear_cmd.is_valid_command());
/// assert_eq!(clear_cmd.get_target_spn(), 123);
/// assert_eq!(clear_cmd.get_target_fmi(), 4);
/// ```
pub trait DM22Helpers {
    /// Create an individual DTC clear command
    ///
    /// Factory method that creates a DM22 message configured to clear a
    /// specific DTC identified by SPN and FMI.
    ///
    /// # Arguments
    /// * `device_id` - Target device for the clear command
    /// * `spn` - Suspect Parameter Number (19-bit, 0-524287)
    /// * `fmi` - Failure Mode Identifier (5-bit, 0-31)
    ///
    /// # Returns
    /// DM22 message configured to clear the specified DTC
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM22Helpers;
    /// use cando_messages::j1939::DM22;
    /// use cando_messages::common::DeviceId;
    ///
    /// let clear_cmd = DM22::create_individual_clear(DeviceId::from(0x59), 1234, 5);
    /// assert_eq!(clear_cmd.device_id, DeviceId::from(0x59));
    /// assert_eq!(clear_cmd.get_target_spn(), 1234);
    /// ```
    fn create_individual_clear(device_id: DeviceId, spn: u32, fmi: u8) -> Self;

    /// Get the target SPN (Suspect Parameter Number)
    ///
    /// Extracts the full 19-bit SPN from the split fields (low 16 bits + high 3 bits).
    ///
    /// # Returns
    /// The complete 19-bit SPN value (0-524287)
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM22Helpers;
    /// use cando_messages::j1939::DM22;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM22::create_individual_clear(DeviceId::from(0x42), 99999, 10);
    /// assert_eq!(cmd.get_target_spn(), 99999);
    /// ```
    fn get_target_spn(&self) -> u32;

    /// Get the target FMI (Failure Mode Identifier)
    ///
    /// # Returns
    /// The 5-bit FMI value (0-31)
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM22Helpers;
    /// use cando_messages::j1939::DM22;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM22::create_individual_clear(DeviceId::from(0x82), 100, 15);
    /// assert_eq!(cmd.get_target_fmi(), 15);
    /// ```
    fn get_target_fmi(&self) -> u8;

    /// Check if this is a valid command
    ///
    /// Validates that the DM22 message has valid SPN and FMI values.
    ///
    /// # Returns
    /// `true` if SPN and FMI are within valid ranges
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM22Helpers;
    /// use cando_messages::j1939::DM22;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM22::create_individual_clear(DeviceId::from(0x42), 123, 4);
    /// assert!(cmd.is_valid_command());
    /// ```
    fn is_valid_command(&self) -> bool;

    /// Check if this command targets a specific device
    ///
    /// # Arguments
    /// * `device_id` - Device to check against
    ///
    /// # Returns
    /// `true` if this command targets the specified device
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM22Helpers;
    /// use cando_messages::j1939::DM22;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM22::create_individual_clear(DeviceId::from(0x59), 123, 4);
    /// assert!(cmd.targets_device(DeviceId::from(0x59)));
    /// ```
    fn targets_device(&self, device_id: DeviceId) -> bool;

    /// Check if this command targets a specific DTC
    ///
    /// # Arguments
    /// * `spn` - Suspect Parameter Number to check
    /// * `fmi` - Failure Mode Identifier to check
    ///
    /// # Returns
    /// `true` if this command targets the specified DTC
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM22Helpers;
    /// use cando_messages::j1939::DM22;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM22::create_individual_clear(DeviceId::from(0x42), 123, 4);
    /// assert!(cmd.targets_dtc(123, 4));
    /// assert!(!cmd.targets_dtc(123, 5));
    /// ```
    fn targets_dtc(&self, spn: u32, fmi: u8) -> bool;

    /// Get expected response message types after clear command
    ///
    /// # Returns
    /// Vector of expected diagnostic message types in response
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::{DM22Helpers, DiagnosticMessageType};
    /// use cando_messages::j1939::DM22;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM22::create_individual_clear(DeviceId::from(0x42), 123, 4);
    /// let responses = cmd.get_expected_response_types();
    /// assert!(responses.contains(&DiagnosticMessageType::DM01));
    /// ```
    fn get_expected_response_types(&self) -> Vec<DiagnosticMessageType>;

    /// Get diagnostic workflow context string
    ///
    /// # Returns
    /// String describing the workflow context
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM22Helpers;
    /// use cando_messages::j1939::DM22;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM22::create_individual_clear(DeviceId::from(0x59), 123, 4);
    /// let context = cmd.diagnostic_workflow_context();
    /// assert!(context.contains("INDIVIDUAL_CLEAR"));
    /// ```
    fn diagnostic_workflow_context(&self) -> String;
}

impl DM22Helpers for DM22 {
    fn create_individual_clear(device_id: DeviceId, spn: u32, fmi: u8) -> Self {
        // Split SPN into low 16 bits and high 3 bits
        let spn_low = (spn & 0xFFFF) as u16;
        let spn_high = ((spn >> 16) & 0x7) as f64 * 65536.0; // Scale by 65536

        DM22 {
            device_id,
            individual_dtc_clear_control_byte: 0xFF, // Clear both active and previously active
            ctrl_byte_indic_individual_dtc_clear: 0x00, // Standard clear operation
            dm22_01spn: spn_low,
            dm22_01fmi: fmi,
            dm22_01spn_high: spn_high,
        }
    }

    fn get_target_spn(&self) -> u32 {
        // Reconstruct full 19-bit SPN from split fields
        let spn_low = self.dm22_01spn as u32;
        let spn_high = ((self.dm22_01spn_high / 65536.0) as u32) & 0x7;
        spn_low | (spn_high << 16)
    }

    fn get_target_fmi(&self) -> u8 {
        (self.dm22_01fmi & 0x1F) // 5-bit value
    }

    fn is_valid_command(&self) -> bool {
        // Check SPN is within 19-bit range (0-524287)
        let spn = self.get_target_spn();
        if spn > 524287 {
            return false;
        }

        // Check FMI is within 5-bit range (0-31)
        let fmi = self.get_target_fmi();
        if fmi > 31 {
            return false;
        }

        true
    }

    fn targets_device(&self, device_id: DeviceId) -> bool {
        self.device_id == device_id
    }

    fn targets_dtc(&self, spn: u32, fmi: u8) -> bool {
        self.get_target_spn() == spn && self.get_target_fmi() == fmi
    }

    fn get_expected_response_types(&self) -> Vec<DiagnosticMessageType> {
        vec![
            DiagnosticMessageType::DM01, // Active DTCs (without the cleared one)
            DiagnosticMessageType::DM02, // Previously active (without the cleared one)
        ]
    }

    fn diagnostic_workflow_context(&self) -> String {
        format!(
            "INDIVIDUAL_CLEAR: Device={:?}, SPN={}, FMI={}, Command=DM22",
            self.device_id,
            self.get_target_spn(),
            self.get_target_fmi()
        )
    }
}

// ============================================================================
// DM13 Helper Trait - Stop/Start Broadcast
// ============================================================================

/// DM13 Network Action
///
/// Represents the action to be performed on a communication port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkAction {
    /// Resume normal operation (default state)
    ResumeNormalOperation = 0,
    /// Stop transmission on this port
    StopBroadcast = 1,
    /// Reserved value
    Reserved = 2,
    /// Not available / error state
    NotAvailable = 3,
}

impl From<u64> for NetworkAction {
    fn from(value: u64) -> Self {
        match value & 0x3 {
            0 => NetworkAction::ResumeNormalOperation,
            1 => NetworkAction::StopBroadcast,
            2 => NetworkAction::Reserved,
            _ => NetworkAction::NotAvailable,
        }
    }
}

impl From<NetworkAction> for u64 {
    fn from(action: NetworkAction) -> Self {
        action as u64
    }
}

impl From<u8> for NetworkAction {
    fn from(value: u8) -> Self {
        match value & 0x3 {
            0 => NetworkAction::ResumeNormalOperation,
            1 => NetworkAction::StopBroadcast,
            2 => NetworkAction::Reserved,
            _ => NetworkAction::NotAvailable,
        }
    }
}

impl From<NetworkAction> for u8 {
    fn from(action: NetworkAction) -> Self {
        action as u8
    }
}

/// DM13 Helper Methods
///
/// Extension trait providing convenience methods for DM13 (Stop/Start Broadcast)
/// message handling. DM13 controls network communication on various protocols,
/// allowing diagnostic tools to temporarily suspend vehicle communications.
///
/// # Example
/// ```
/// use cando_messages::j1939::diagnostics::DM13Helpers;
/// use cando_messages::j1939::DM13;
/// use cando_messages::common::DeviceId;
///
/// // Create command to stop J1939 Network #1 for 60 seconds
/// let stop_cmd = DM13::create_stop_broadcast(DeviceId::DIAGNOSTIC_TOOL_1, 60);
/// assert!(stop_cmd.is_stop_command());
/// assert!(stop_cmd.affects_j1939_network1());
/// ```
pub trait DM13Helpers {
    /// Create a stop broadcast command
    ///
    /// Factory method that creates a DM13 message configured to stop broadcast
    /// on J1939 Network #1 for a specified duration.
    ///
    /// # Arguments
    /// * `device_id` - Source device sending the command (typically diagnostic tool)
    /// * `duration_seconds` - How long to suspend (0-64255 seconds)
    ///
    /// # Returns
    /// DM13 message configured to stop broadcast
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM13Helpers;
    /// use cando_messages::j1939::DM13;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM13::create_stop_broadcast(DeviceId::DIAGNOSTIC_TOOL_1, 120);
    /// assert!(cmd.is_stop_command());
    /// ```
    fn create_stop_broadcast(device_id: DeviceId, duration_seconds: u64) -> Self;

    /// Create a resume normal operation command
    ///
    /// Factory method that creates a DM13 message configured to resume normal
    /// operation on all networks.
    ///
    /// # Arguments
    /// * `device_id` - Source device sending the command
    ///
    /// # Returns
    /// DM13 message configured to resume normal operation
    ///
    /// # Example
    /// ```
    /// use cando_messages::j1939::diagnostics::DM13Helpers;
    /// use cando_messages::j1939::DM13;
    /// use cando_messages::common::DeviceId;
    ///
    /// let cmd = DM13::create_resume_operation(DeviceId::DIAGNOSTIC_TOOL_1);
    /// assert!(cmd.is_resume_command());
    /// ```
    fn create_resume_operation(device_id: DeviceId) -> Self;

    /// Check if this is a stop broadcast command
    ///
    /// # Returns
    /// `true` if any network is commanded to stop
    fn is_stop_command(&self) -> bool;

    /// Check if this is a resume operation command
    ///
    /// # Returns
    /// `true` if all networks are commanded to resume
    fn is_resume_command(&self) -> bool;

    /// Check if this command affects J1939 Network #1
    ///
    /// # Returns
    /// `true` if J1939 Network #1 action is stop or resume
    fn affects_j1939_network1(&self) -> bool;

    /// Check if this command affects J1939 Network #2
    ///
    /// # Returns
    /// `true` if J1939 Network #2 action is stop or resume
    fn affects_j1939_network2(&self) -> bool;

    /// Check if this command affects J1939 Network #3
    ///
    /// # Returns
    /// `true` if J1939 Network #3 action is stop or resume
    fn affects_j1939_network3(&self) -> bool;

    /// Get the suspend duration in seconds
    ///
    /// # Returns
    /// Duration in seconds (0-64255)
    fn get_suspend_duration_seconds(&self) -> u64;

    /// Check if hold signal is active
    ///
    /// The hold signal indicates that suspended networks should remain
    /// in the modified state indefinitely.
    ///
    /// # Returns
    /// `true` if hold signal is active
    fn has_hold_signal(&self) -> bool;

    /// Check if this is a valid command
    ///
    /// # Returns
    /// `true` if the command has valid field values
    fn is_valid_command(&self) -> bool;

    /// Get the action for J1939 Network #1
    ///
    /// # Returns
    /// NetworkAction for J1939 Network #1
    fn get_j1939_network1_action(&self) -> NetworkAction;

    /// Get the action for J1939 Network #2
    ///
    /// # Returns
    /// NetworkAction for J1939 Network #2
    fn get_j1939_network2_action(&self) -> NetworkAction;

    /// Get the action for J1939 Network #3
    ///
    /// # Returns
    /// NetworkAction for J1939 Network #3
    fn get_j1939_network3_action(&self) -> NetworkAction;

    /// Get diagnostic workflow context string
    ///
    /// # Returns
    /// String describing the workflow context
    fn diagnostic_workflow_context(&self) -> String;
}

impl DM13Helpers for DM13 {
    fn create_stop_broadcast(device_id: DeviceId, duration_seconds: u64) -> Self {
        DM13 {
            device_id,
            j_1939_network_1: NetworkAction::StopBroadcast.into(),
            sae_j1922: NetworkAction::NotAvailable.into(),
            sae_j1587: NetworkAction::NotAvailable.into(),
            current_data_link: NetworkAction::ResumeNormalOperation.into(),
            manufacturer_specific_port: NetworkAction::NotAvailable.into(),
            sae_j1850: NetworkAction::NotAvailable.into(),
            iso_9141: NetworkAction::NotAvailable.into(),
            j_1939_network_2: NetworkAction::NotAvailable.into(),
            j_1939_network_3: NetworkAction::NotAvailable.into(),
            suspend_signal: 0x0F, // All bits set = suspend active
            hold_signal: 0x00,    // No hold
            suspend_duration: (duration_seconds.min(64255)) as u16,
        }
    }

    fn create_resume_operation(device_id: DeviceId) -> Self {
        DM13 {
            device_id,
            j_1939_network_1: NetworkAction::ResumeNormalOperation.into(),
            sae_j1922: NetworkAction::ResumeNormalOperation.into(),
            sae_j1587: NetworkAction::ResumeNormalOperation.into(),
            current_data_link: NetworkAction::ResumeNormalOperation.into(),
            manufacturer_specific_port: NetworkAction::ResumeNormalOperation.into(),
            sae_j1850: NetworkAction::ResumeNormalOperation.into(),
            iso_9141: NetworkAction::ResumeNormalOperation.into(),
            j_1939_network_2: NetworkAction::ResumeNormalOperation.into(),
            j_1939_network_3: NetworkAction::ResumeNormalOperation.into(),
            suspend_signal: 0x00,
            hold_signal: 0x00,
            suspend_duration: 0,
        }
    }

    fn is_stop_command(&self) -> bool {
        self.get_j1939_network1_action() == NetworkAction::StopBroadcast
            || self.get_j1939_network2_action() == NetworkAction::StopBroadcast
            || self.get_j1939_network3_action() == NetworkAction::StopBroadcast
    }

    fn is_resume_command(&self) -> bool {
        self.get_j1939_network1_action() == NetworkAction::ResumeNormalOperation
            && self.suspend_signal == 0
            && self.hold_signal == 0
    }

    fn affects_j1939_network1(&self) -> bool {
        let action = self.get_j1939_network1_action();
        action == NetworkAction::StopBroadcast || action == NetworkAction::ResumeNormalOperation
    }

    fn affects_j1939_network2(&self) -> bool {
        let action = self.get_j1939_network2_action();
        action == NetworkAction::StopBroadcast || action == NetworkAction::ResumeNormalOperation
    }

    fn affects_j1939_network3(&self) -> bool {
        let action = self.get_j1939_network3_action();
        action == NetworkAction::StopBroadcast || action == NetworkAction::ResumeNormalOperation
    }

    fn get_suspend_duration_seconds(&self) -> u64 {
        self.suspend_duration.into()
    }

    fn has_hold_signal(&self) -> bool {
        self.hold_signal != 0
    }

    fn is_valid_command(&self) -> bool {
        // Check suspend duration is within range
        if self.suspend_duration > 64255 {
            return false;
        }

        // Check that at least one network has a valid action
        

        self.affects_j1939_network1()
            || self.affects_j1939_network2()
            || self.affects_j1939_network3()
    }

    fn get_j1939_network1_action(&self) -> NetworkAction {
        self.j_1939_network_1.into()
    }

    fn get_j1939_network2_action(&self) -> NetworkAction {
        self.j_1939_network_2.into()
    }

    fn get_j1939_network3_action(&self) -> NetworkAction {
        self.j_1939_network_3.into()
    }

    fn diagnostic_workflow_context(&self) -> String {
        let action = if self.is_stop_command() {
            "STOP_BROADCAST"
        } else if self.is_resume_command() {
            "RESUME_OPERATION"
        } else {
            "NETWORK_CONTROL"
        };

        format!(
            "NETWORK_CONTROL: Device={:?}, Action={}, Duration={}s, Command=DM13",
            self.device_id,
            action,
            self.get_suspend_duration_seconds()
        )
    }
}
