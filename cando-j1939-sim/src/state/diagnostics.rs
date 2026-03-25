use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsState {
    // DM01 Active DTCs
    pub dm01_protect_lamp_status: u8,
    pub dm01_amber_warning_lamp_status: u8,
    pub dm01_red_stop_lamp_status: u8,
    pub dm01_malfunction_indicator_lamp_status: u8,
    pub dm01_flash_protect_lamp: u8,
    pub dm01_flash_amber_warning_lamp: u8,
    pub dm01_flash_red_stop_lamp: u8,
    pub dm01_flash_malfunction_indicator_lamp: u8,
    pub dm01_active_dtc_spn: u16,
    pub dm01_active_dtc_spn_high: f64,
    pub dm01_active_dtc_fmi: u8,
    pub dm01_active_dtc_occurrence_count: u8,
    pub dm01_active_dtc_conversion_method: u8,
    pub dm01_fault_injection_enabled: bool,

    // DM02 Previously Active DTCs
    pub dm02_protect_lamp_status: u8,
    pub dm02_amber_warning_lamp_status: u8,
    pub dm02_red_stop_lamp_status: u8,
    pub dm02_malfunction_indicator_lamp_status: u8,
    pub dm02_flash_protect_lamp: u8,
    pub dm02_flash_amber_warning_lamp: u8,
    pub dm02_flash_red_stop_lamp: u8,
    pub dm02_flash_malfunction_indicator_lamp: u8,
    pub dm02_previously_active_dtc_spn: u16,
    pub dm02_previously_active_dtc_spn_high: f64,
    pub dm02_previously_active_dtc_fmi: u8,
    pub dm02_previously_active_dtc_occurrence_count: u8,
    pub dm02_previously_active_dtc_conversion_method: u8,
    pub dm02_fault_injection_enabled: bool,

    // DM03 Clear/Reset
    pub dm03_clear_commands_received: u64,
    pub dm03_last_clear_timestamp: u64,
    pub dm03_clear_operations_enabled: bool,
    pub dm03_auto_response_enabled: bool,

    // DM03 Command Generation
    pub dm03_command_generation_enabled: bool,
    pub dm03_target_device_id: u8,
    pub dm03_command_interval_seconds: u64,
    pub dm03_commands_sent: u64,
    pub dm03_last_send_timestamp: u64,

    // DM04 Freeze Frame Parameters States (J1939-73 Diagnostics)
    pub dm04_freeze_frame_length: u8,
    pub dm04_spn: u16,
    pub dm04_fmi: u8,
    pub dm04_eng_speed: f64,
    pub dm04_eng_load: u8,
    pub dm04_coolant_temp: f64,
    pub dm04_vehicle_speed: f64,

    // DM05 OBD Readiness Monitors States (J1939-73 Diagnostics)
    pub dm05_active_trouble_code_count: u8,
    pub dm05_previously_active_trouble_code_count: u8,
    pub dm05_obd_compliance: u8,

    // DM06 Pending DTC States (J1939-73 Diagnostics)
    pub dm06_pending_dtc_spn: u16,
    pub dm06_pending_dtc_fmi: u8,

    // DM07 Command Non-Continuously Monitored Test States (J1939-73 Diagnostics)
    pub dm07_test_id: u8,
    pub dm07_spn: u16,
    pub dm07_fmi: u8,

    // DM10 Non-Continuously Monitored Test Identifiers Support (J1939-73 Diagnostics)
    pub dm10_test_identifier_supported: u64,

    // DM11 Diagnostic Data Clear/Reset for Active DTCs (J1939-73 Diagnostics)
    pub dm11_clear_requested: bool,
    pub dm11_clear_count: u64,

    // DM12 Emissions Related Active DTCs (J1939-73 Diagnostics)
    pub dm12_active_dtc_spn: u16,
    pub dm12_active_dtc_fmi: u8,

    // DM13 Stop/Start Broadcast (J1939-73 Diagnostics)
    pub dm13_j1939_network_1: u8,
    pub dm13_suspend_duration: u16,
    pub dm13_suspend_signal: u8,
    pub dm13_hold_signal: u8,

    // DM19 Calibration Information (J1939-73 Diagnostics)
    pub dm19_calibration_verification_number: u32,
    pub dm19_calibration_id_1: u32,

    // DM20 Monitor Performance Ratio (J1939-73 Diagnostics)
    pub dm20_ignition_cycle_counter: u16,
    pub dm20_obd_monitoring_cond_encountered: u16,
    pub dm20_spn_of_appl_sys_monitor: u32,
    pub dm20_appl_sys_monitor_numerator: u16,
    pub dm20_appl_sys_monitor_denominator: u16,

    // DM21 Diagnostic Readiness 2 (J1939-73 Diagnostics)
    pub dm21_distance_while_mil_activated: u16,
    pub dm21_distance_since_dtcs_cleared: u16,
    pub dm21_minutes_run_mil_activated: u16,
    pub dm21_time_since_dtcs_cleared: u16,

    // DM25 Expanded Freeze Frame (J1939-73 Diagnostics)
    pub dm25_expanded_freeze_frame_length: u8,
    pub dm25_spn: u16,
    pub dm25_fmi: u8,

    // DM27 All Pending DTCs (J1939-73 Diagnostics)
    pub dm27_pending_dtc_spn: u16,
    pub dm27_pending_dtc_fmi: u8,

    // DM28 Permanent DTCs (J1939-73 Diagnostics)
    pub dm28_permanent_dtc_spn: u16,
    pub dm28_permanent_dtc_fmi: u8,

    // DM29 Regulated DTC Counts (J1939-73 Diagnostics)
    pub dm29_pending_dtc_count: u8,
    pub dm29_all_pending_dtc_count: u8,
    pub dm29_mil_on_dtc_count: u8,
    pub dm29_previously_mil_on_dtc_count: u8,
    pub dm29_permanent_dtc_count: u8,

    // DM31 DTC to Lamp Association (J1939-73 Diagnostics)
    pub dm31_spn: u16,
    pub dm31_fmi: u8,
    pub dm31_protect_lamp_status: u8,
    pub dm31_warn_lamp_status: u8,
    pub dm31_stop_lamp_status: u8,
    pub dm31_mil_status: u8,

    // DM33 Emission Increasing AECD Active Time (J1939-73 Diagnostics)
    pub dm33_aecd_number: u8,
    pub dm33_aecd_timer_1: u32,
    pub dm33_aecd_timer_2: u32,

    // DM34 NTE Status (J1939-73 Diagnostics)
    pub dm34_nox_nte_deficiency_area_status: u8,
    pub dm34_nox_nte_carve_out_area_status: u8,
    pub dm34_nox_nte_control_area_status: u8,
    pub dm34_pm_nte_deficiency_area_status: u8,
    pub dm34_pm_nte_carve_out_area_status: u8,
    pub dm34_pm_nte_control_area_status: u8,

    // DM35 Exhaust Gas Recirculation Diagnostics (J1939-73 Diagnostics)
    pub dm35_dtc_spn: u16,
    pub dm35_dtc_fmi: u8,
}

impl Default for DiagnosticsState {
    fn default() -> Self {
        Self {
            dm01_protect_lamp_status: 0,
            dm01_amber_warning_lamp_status: 0,
            dm01_red_stop_lamp_status: 0,
            dm01_malfunction_indicator_lamp_status: 0,
            dm01_flash_protect_lamp: 0,
            dm01_flash_amber_warning_lamp: 0,
            dm01_flash_red_stop_lamp: 0,
            dm01_flash_malfunction_indicator_lamp: 0,
            dm01_active_dtc_spn: 0xFFFF,
            dm01_active_dtc_spn_high: 0.0,
            dm01_active_dtc_fmi: 0xFF,
            dm01_active_dtc_occurrence_count: 0xFF,
            dm01_active_dtc_conversion_method: 0xFF,
            dm01_fault_injection_enabled: false,
            dm02_protect_lamp_status: 0,
            dm02_amber_warning_lamp_status: 0,
            dm02_red_stop_lamp_status: 0,
            dm02_malfunction_indicator_lamp_status: 0,
            dm02_flash_protect_lamp: 0,
            dm02_flash_amber_warning_lamp: 0,
            dm02_flash_red_stop_lamp: 0,
            dm02_flash_malfunction_indicator_lamp: 0,
            dm02_previously_active_dtc_spn: 0xFFFF,
            dm02_previously_active_dtc_spn_high: 0.0,
            dm02_previously_active_dtc_fmi: 0xFF,
            dm02_previously_active_dtc_occurrence_count: 0xFF,
            dm02_previously_active_dtc_conversion_method: 0xFF,
            dm02_fault_injection_enabled: false,
            dm03_clear_commands_received: 0,
            dm03_last_clear_timestamp: 0,
            dm03_clear_operations_enabled: true,
            dm03_auto_response_enabled: true,
            dm03_command_generation_enabled: false,
            dm03_target_device_id: 0x82,
            dm03_command_interval_seconds: 0,
            dm03_commands_sent: 0,
            dm03_last_send_timestamp: 0,

            // DM04 Freeze Frame Parameters defaults (J1939-73 Diagnostics)
            dm04_freeze_frame_length: 0,
            dm04_spn: 0xFFFF,
            dm04_fmi: 0xFF,
            dm04_eng_speed: 0.0,
            dm04_eng_load: 0,
            dm04_coolant_temp: -40.0,
            dm04_vehicle_speed: 0.0,

            // DM05 OBD Readiness Monitors defaults (J1939-73 Diagnostics)
            dm05_active_trouble_code_count: 0,
            dm05_previously_active_trouble_code_count: 0,
            dm05_obd_compliance: 0,

            // DM06 Pending DTC defaults (J1939-73 Diagnostics)
            dm06_pending_dtc_spn: 0xFFFF,
            dm06_pending_dtc_fmi: 0xFF,

            // DM07 Command Non-Continuously Monitored Test defaults (J1939-73 Diagnostics)
            dm07_test_id: 0,
            dm07_spn: 0xFFFF,
            dm07_fmi: 0xFF,

            // DM10 Non-Continuously Monitored Test Identifiers Support defaults
            dm10_test_identifier_supported: 0,

            // DM11 Diagnostic Data Clear/Reset defaults (J1939-73 Diagnostics)
            dm11_clear_requested: false,
            dm11_clear_count: 0,

            // DM12 Emissions Related Active DTCs defaults (J1939-73 Diagnostics)
            dm12_active_dtc_spn: 0xFFFF,
            dm12_active_dtc_fmi: 0xFF,

            // DM13 Stop/Start Broadcast defaults (J1939-73 Diagnostics)
            dm13_j1939_network_1: 3, // Don't care
            dm13_suspend_duration: 0,
            dm13_suspend_signal: 0,
            dm13_hold_signal: 0,

            // DM19 Calibration Information defaults (J1939-73 Diagnostics)
            dm19_calibration_verification_number: 0,
            dm19_calibration_id_1: 0,

            // DM20 Monitor Performance Ratio defaults (J1939-73 Diagnostics)
            dm20_ignition_cycle_counter: 0,
            dm20_obd_monitoring_cond_encountered: 0,
            dm20_spn_of_appl_sys_monitor: 0,
            dm20_appl_sys_monitor_numerator: 0,
            dm20_appl_sys_monitor_denominator: 0,

            // DM21 Diagnostic Readiness 2 defaults (J1939-73 Diagnostics)
            dm21_distance_while_mil_activated: 0,
            dm21_distance_since_dtcs_cleared: 0,
            dm21_minutes_run_mil_activated: 0,
            dm21_time_since_dtcs_cleared: 0,

            // DM25 Expanded Freeze Frame defaults (J1939-73 Diagnostics)
            dm25_expanded_freeze_frame_length: 0,
            dm25_spn: 0xFFFF,
            dm25_fmi: 0xFF,

            // DM27 All Pending DTCs defaults (J1939-73 Diagnostics)
            dm27_pending_dtc_spn: 0xFFFF,
            dm27_pending_dtc_fmi: 0xFF,

            // DM28 Permanent DTCs defaults (J1939-73 Diagnostics)
            dm28_permanent_dtc_spn: 0xFFFF,
            dm28_permanent_dtc_fmi: 0xFF,

            // DM29 Regulated DTC Counts defaults (J1939-73 Diagnostics)
            dm29_pending_dtc_count: 0,
            dm29_all_pending_dtc_count: 0,
            dm29_mil_on_dtc_count: 0,
            dm29_previously_mil_on_dtc_count: 0,
            dm29_permanent_dtc_count: 0,

            // DM31 DTC to Lamp Association defaults (J1939-73 Diagnostics)
            dm31_spn: 0xFFFF,
            dm31_fmi: 0xFF,
            dm31_protect_lamp_status: 3, // Unavailable
            dm31_warn_lamp_status: 3,    // Unavailable
            dm31_stop_lamp_status: 3,    // Unavailable
            dm31_mil_status: 3,          // Unavailable

            // DM33 Emission Increasing AECD Active Time defaults (J1939-73 Diagnostics)
            dm33_aecd_number: 0,
            dm33_aecd_timer_1: 0,
            dm33_aecd_timer_2: 0,

            // DM34 NTE Status defaults (J1939-73 Diagnostics)
            dm34_nox_nte_deficiency_area_status: 3, // Not available
            dm34_nox_nte_carve_out_area_status: 3,  // Not available
            dm34_nox_nte_control_area_status: 3,    // Not available
            dm34_pm_nte_deficiency_area_status: 3,  // Not available
            dm34_pm_nte_carve_out_area_status: 3,   // Not available
            dm34_pm_nte_control_area_status: 3,     // Not available

            // DM35 Exhaust Gas Recirculation Diagnostics defaults (J1939-73 Diagnostics)
            dm35_dtc_spn: 0xFFFF,
            dm35_dtc_fmi: 0xFF,
        }
    }
}
