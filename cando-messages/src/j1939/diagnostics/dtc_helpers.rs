//! Helper methods for DTC-style diagnostic messages (DM01, DM02, DM06, DM12, DM23)
//!
//! These messages all share the same structure:
//! - 8 lamp status fields (4 lamp states + 4 flash states)
//! - 19 DTC slots, each with 5 fields: SPN, FMI, SPN_High, OC, CM
//!
//! This module provides a common trait and helper types for working with these messages.

use crate::common::DeviceId;

/// Represents a single Diagnostic Trouble Code (DTC)
#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticTroubleCode {
    /// Suspect Parameter Number (19-bit value)
    pub spn: u32,
    /// Failure Mode Identifier (5-bit value)
    pub fmi: u8,
    /// Occurrence Count (7-bit value)
    pub occurrence_count: u8,
    /// Conversion Method (1-bit flag)
    pub conversion_method: bool,
}

impl DiagnosticTroubleCode {
    /// Create a new DTC from individual components
    pub fn new(spn: u32, fmi: u8, occurrence_count: u8, conversion_method: bool) -> Self {
        Self {
            spn,
            fmi,
            occurrence_count,
            conversion_method,
        }
    }

    /// Check if this DTC represents "no fault" (all zeros)
    pub fn is_empty(&self) -> bool {
        self.spn == 0 && self.fmi == 0 && self.occurrence_count == 0
    }

    /// Get a human-readable description of the FMI
    pub fn fmi_description(&self) -> &'static str {
        match self.fmi {
            0 => "Above normal - most severe",
            1 => "Below normal - most severe",
            2 => "Data erratic",
            3 => "Voltage above normal",
            4 => "Voltage below normal",
            5 => "Current below normal",
            6 => "Current above normal",
            7 => "Mechanical system not responding",
            8 => "Abnormal frequency",
            9 => "Abnormal update rate",
            10 => "Abnormal rate of change",
            11 => "Root cause not known",
            12 => "Bad intelligent device",
            13 => "Out of calibration",
            14 => "Special instructions",
            15 => "Above normal - least severe",
            16 => "Above normal - moderately severe",
            17 => "Below normal - least severe",
            18 => "Below normal - moderately severe",
            19 => "Received network data in error",
            31 => "Not available",
            _ => "Reserved",
        }
    }
}

impl std::fmt::Display for DiagnosticTroubleCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SPN={} FMI={} ({}) OC={} CM={}",
            self.spn,
            self.fmi,
            self.fmi_description(),
            self.occurrence_count,
            if self.conversion_method {
                "J1939-73"
            } else {
                "J1587"
            }
        )
    }
}

/// Lamp status values as defined in J1939-73
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LampState {
    /// Lamp is off
    Off = 0,
    /// Lamp is on
    On = 1,
    /// Reserved
    Reserved = 2,
    /// Not available / don't care
    NotAvailable = 3,
}

impl From<u8> for LampState {
    fn from(value: u8) -> Self {
        match value {
            0 => LampState::Off,
            1 => LampState::On,
            2 => LampState::Reserved,
            _ => LampState::NotAvailable,
        }
    }
}

impl std::fmt::Display for LampState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LampState::Off => write!(f, "OFF"),
            LampState::On => write!(f, "ON"),
            LampState::Reserved => write!(f, "RESERVED"),
            LampState::NotAvailable => write!(f, "N/A"),
        }
    }
}

/// Summary of all lamp states in a diagnostic message
#[derive(Debug, Clone, PartialEq)]
pub struct LampStatus {
    /// Malfunction Indicator Lamp (MIL) - emissions-related
    pub mil: LampState,
    /// Red Stop Lamp (RSL) - severe conditions
    pub red_stop: LampState,
    /// Amber Warning Lamp (AWL) - warnings
    pub amber_warning: LampState,
    /// Protect Lamp - non-electronic subsystem issues
    pub protect: LampState,
    /// Flash Malfunction Indicator Lamp
    pub flash_mil: LampState,
    /// Flash Red Stop Lamp
    pub flash_red_stop: LampState,
    /// Flash Amber Warning Lamp
    pub flash_amber_warning: LampState,
    /// Flash Protect Lamp
    pub flash_protect: LampState,
}

impl LampStatus {
    /// Check if any lamp is on
    pub fn any_lamp_on(&self) -> bool {
        self.mil == LampState::On
            || self.red_stop == LampState::On
            || self.amber_warning == LampState::On
            || self.protect == LampState::On
    }

    /// Check if any flash is active
    pub fn any_flash_active(&self) -> bool {
        self.flash_mil == LampState::On
            || self.flash_red_stop == LampState::On
            || self.flash_amber_warning == LampState::On
            || self.flash_protect == LampState::On
    }

    /// Check if MIL (Malfunction Indicator Lamp) is on
    pub fn mil_on(&self) -> bool {
        self.mil == LampState::On
    }

    /// Check if any critical lamp is on (Red Stop or Protect)
    pub fn critical_lamp_on(&self) -> bool {
        self.red_stop == LampState::On || self.protect == LampState::On
    }
}

impl std::fmt::Display for LampStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Lamp Status:")?;
        writeln!(f, "  MIL (Malfunction Indicator): {}", self.mil)?;
        writeln!(f, "  RSL (Red Stop Lamp):          {}", self.red_stop)?;
        writeln!(f, "  AWL (Amber Warning Lamp):     {}", self.amber_warning)?;
        writeln!(f, "  Protect Lamp:                 {}", self.protect)?;
        if self.any_flash_active() {
            writeln!(f, "  Flash MIL:                    {}", self.flash_mil)?;
            writeln!(f, "  Flash RSL:                    {}", self.flash_red_stop)?;
            writeln!(
                f,
                "  Flash AWL:                    {}",
                self.flash_amber_warning
            )?;
            writeln!(f, "  Flash Protect:                {}", self.flash_protect)?;
        }
        Ok(())
    }
}

/// Trait for DTC-style diagnostic messages with 19 fault slots
///
/// This trait provides common helper methods for diagnostic messages that follow
/// the standard J1939-73 pattern of lamp status fields + 19 DTC slots.
///
/// Implemented for: DM01, DM02, DM06, DM12, DM23
pub trait DtcMessage {
    /// Get the device ID that sent this message
    fn device_id(&self) -> DeviceId;

    /// Get lamp status fields as a tuple: (mil, red_stop, amber_warning, protect, flash_mil, flash_red_stop, flash_amber_warning, flash_protect)
    fn lamp_fields(&self) -> (u8, u8, u8, u8, u8, u8, u8, u8);

    /// Get DTC slot N (1-19) as a tuple: (spn, fmi, spn_high, oc, cm)
    fn dtc_slot(&self, slot: usize) -> Option<(u16, u8, f64, u8, u8)>;

    /// Extract a single DTC from a slot (1-19)
    fn get_dtc(&self, slot: usize) -> Option<DiagnosticTroubleCode> {
        if !(1..=19).contains(&slot) {
            return None;
        }

        let (spn_low, fmi, spn_high, oc, cm) = self.dtc_slot(slot)?;

        // Reconstruct full 19-bit SPN from low 16 bits + high 3 bits
        let spn = spn_low as u32 + ((spn_high as u32) * 65536);

        // Check if this slot is empty (all zeros)
        if spn == 0 && fmi == 0 && oc == 0 && cm == 0 {
            return None;
        }

        Some(DiagnosticTroubleCode::new(
            spn,
            fmi,
            oc,
            cm != 0,
        ))
    }

    /// Get all active DTCs (non-empty slots)
    fn get_active_dtcs(&self) -> Vec<DiagnosticTroubleCode> {
        (1..=19)
            .filter_map(|slot| self.get_dtc(slot))
            .filter(|dtc| !dtc.is_empty())
            .collect()
    }

    /// Check if there are any active faults
    fn has_active_faults(&self) -> bool {
        (1..=19).any(|slot| {
            if let Some(dtc) = self.get_dtc(slot) {
                !dtc.is_empty()
            } else {
                false
            }
        })
    }

    /// Get the count of active faults
    fn active_fault_count(&self) -> usize {
        self.get_active_dtcs().len()
    }

    /// Get lamp status summary
    fn lamp_status(&self) -> LampStatus {
        let (
            mil,
            red_stop,
            amber_warning,
            protect,
            flash_mil,
            flash_red_stop,
            flash_amber_warning,
            flash_protect,
        ) = self.lamp_fields();

        LampStatus {
            mil: LampState::from(mil),
            red_stop: LampState::from(red_stop),
            amber_warning: LampState::from(amber_warning),
            protect: LampState::from(protect),
            flash_mil: LampState::from(flash_mil),
            flash_red_stop: LampState::from(flash_red_stop),
            flash_amber_warning: LampState::from(flash_amber_warning),
            flash_protect: LampState::from(flash_protect),
        }
    }

    /// Format a summary of this diagnostic message
    fn summary(&self) -> String {
        let dtcs = self.get_active_dtcs();
        let lamp_status = self.lamp_status();

        let mut output = String::new();
        output.push_str(&format!("Device: {:?}\n", self.device_id()));
        output.push_str(&format!("Active DTCs: {}\n", dtcs.len()));
        output.push_str(&format!("{}", lamp_status));

        if !dtcs.is_empty() {
            output.push_str("\nActive Faults:\n");
            for (i, dtc) in dtcs.iter().enumerate() {
                output.push_str(&format!("  {}. {}\n", i + 1, dtc));
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dtc_creation() {
        let dtc = DiagnosticTroubleCode::new(1234, 5, 10, true);
        assert_eq!(dtc.spn, 1234);
        assert_eq!(dtc.fmi, 5);
        assert_eq!(dtc.occurrence_count, 10);
        assert!(dtc.conversion_method);
    }

    #[test]
    fn test_dtc_is_empty() {
        let empty_dtc = DiagnosticTroubleCode::new(0, 0, 0, false);
        assert!(empty_dtc.is_empty());

        let active_dtc = DiagnosticTroubleCode::new(1234, 5, 1, false);
        assert!(!active_dtc.is_empty());
    }

    #[test]
    fn test_dtc_fmi_descriptions() {
        let dtc = DiagnosticTroubleCode::new(100, 0, 1, false);
        assert_eq!(dtc.fmi_description(), "Above normal - most severe");

        let dtc2 = DiagnosticTroubleCode::new(100, 31, 1, false);
        assert_eq!(dtc2.fmi_description(), "Not available");
    }

    #[test]
    fn test_dtc_display() {
        let dtc = DiagnosticTroubleCode::new(1234, 5, 10, true);
        let display = format!("{}", dtc);
        assert!(display.contains("SPN=1234"));
        assert!(display.contains("FMI=5"));
        assert!(display.contains("OC=10"));
        assert!(display.contains("J1939-73"));
    }

    #[test]
    fn test_lamp_state_from_u64() {
        assert_eq!(LampState::from(0), LampState::Off);
        assert_eq!(LampState::from(1), LampState::On);
        assert_eq!(LampState::from(2), LampState::Reserved);
        assert_eq!(LampState::from(3), LampState::NotAvailable);
        assert_eq!(LampState::from(99), LampState::NotAvailable);
    }

    #[test]
    fn test_lamp_status_checks() {
        let status = LampStatus {
            mil: LampState::On,
            red_stop: LampState::Off,
            amber_warning: LampState::Off,
            protect: LampState::Off,
            flash_mil: LampState::Off,
            flash_red_stop: LampState::Off,
            flash_amber_warning: LampState::Off,
            flash_protect: LampState::Off,
        };

        assert!(status.any_lamp_on());
        assert!(status.mil_on());
        assert!(!status.critical_lamp_on());
        assert!(!status.any_flash_active());
    }

    #[test]
    fn test_lamp_status_critical() {
        let status = LampStatus {
            mil: LampState::Off,
            red_stop: LampState::On,
            amber_warning: LampState::Off,
            protect: LampState::Off,
            flash_mil: LampState::Off,
            flash_red_stop: LampState::Off,
            flash_amber_warning: LampState::Off,
            flash_protect: LampState::Off,
        };

        assert!(status.critical_lamp_on());
        assert!(status.any_lamp_on());
    }

    #[test]
    fn test_lamp_status_flash() {
        let status = LampStatus {
            mil: LampState::Off,
            red_stop: LampState::Off,
            amber_warning: LampState::Off,
            protect: LampState::Off,
            flash_mil: LampState::On,
            flash_red_stop: LampState::Off,
            flash_amber_warning: LampState::Off,
            flash_protect: LampState::Off,
        };

        assert!(status.any_flash_active());
        assert!(!status.any_lamp_on());
    }
}
