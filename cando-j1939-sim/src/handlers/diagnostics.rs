use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle DM01 - Active Diagnostic Trouble Codes (J1939-73 Diagnostics)
    pub(crate) fn handle_dm01(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DM01 - Active Diagnostic Trouble Codes (J1939-73 Diagnostics)
        match DM01::decode(can_id, data) {
            Ok(_msg) => {
                // Update diagnostic state from received commands
                self.diagnostics.dm01_fault_injection_enabled = true; // Enable fault simulation
                println!("🔧 Received DM01 diagnostic command");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DM01: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM02 - Previously Active Diagnostic Trouble Codes (J1939-73 Diagnostics)
    pub(crate) fn handle_dm02(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DM02 - Previously Active Diagnostic Trouble Codes (J1939-73 Diagnostics)
        match DM02::decode(can_id, data) {
            Ok(_msg) => {
                // Update diagnostic state from received commands
                self.diagnostics.dm02_fault_injection_enabled = true; // Enable fault simulation
                println!("🔧 Received DM02 diagnostic command");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DM02: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM04 - Freeze Frame Parameters (J1939-73 Diagnostics)
    pub(crate) fn handle_dm04(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM04::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm04_freeze_frame_length = msg.freeze_frame_length;
                self.diagnostics.dm04_spn = msg.dm04_01spn;
                self.diagnostics.dm04_fmi = msg.dm04_01fmi;
                self.diagnostics.dm04_eng_speed = msg.eng_speed;
                self.diagnostics.dm04_eng_load = msg.eng_percent_load_at_current_speed;
                self.diagnostics.dm04_coolant_temp = msg.engine_coolant_temperature;
                self.diagnostics.dm04_vehicle_speed = msg.wheel_based_vehicle_speed;
                println!("Received DM04 freeze frame data");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM04: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM05 - OBD Readiness Monitors (J1939-73 Diagnostics)
    pub(crate) fn handle_dm05(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM05::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm05_active_trouble_code_count = msg.active_trouble_code_count;
                self.diagnostics.dm05_previously_active_trouble_code_count =
                    msg.previously_active_trouble_code_count;
                self.diagnostics.dm05_obd_compliance = msg.obd_compliance;
                println!("Received DM05 OBD readiness monitors");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM05: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM06 - Pending Diagnostic Trouble Codes (J1939-73 Diagnostics)
    pub(crate) fn handle_dm06(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM06::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm06_pending_dtc_spn = msg.dm06_01spn;
                self.diagnostics.dm06_pending_dtc_fmi = msg.dm06_01fmi;
                println!("Received DM06 pending DTCs");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM06: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM07 - Command Non-Continuously Monitored Test (J1939-73 Diagnostics)
    pub(crate) fn handle_dm07(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM07::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm07_test_id = msg.test_identifier;
                self.diagnostics.dm07_spn = msg.dm07_01spn;
                self.diagnostics.dm07_fmi = msg.dm07_01fmi;
                println!("Received DM07 test command: test_id={}", msg.test_identifier);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM07: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM10 - Non-Continuously Monitored Test Identifiers Support (J1939-73 Diagnostics)
    pub(crate) fn handle_dm10(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM10::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm10_test_identifier_supported = msg.test_identifier_supported;
                println!("Received DM10 test identifier support");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM10: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM11 - Diagnostic Data Clear/Reset for Active DTCs (J1939-73 Diagnostics)
    pub(crate) fn handle_dm11(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM11::decode(can_id, data) {
            Ok(_msg) => {
                self.diagnostics.dm11_clear_requested = true;
                self.diagnostics.dm11_clear_count += 1;
                // Clear active DTCs
                self.diagnostics.dm01_active_dtc_spn = 0xFFFF;
                self.diagnostics.dm01_active_dtc_fmi = 0xFF;
                self.diagnostics.dm01_active_dtc_occurrence_count = 0xFF;
                self.diagnostics.dm01_protect_lamp_status = 0;
                self.diagnostics.dm01_amber_warning_lamp_status = 0;
                self.diagnostics.dm01_red_stop_lamp_status = 0;
                self.diagnostics.dm01_malfunction_indicator_lamp_status = 0;
                println!("Received DM11 clear/reset command for active DTCs");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM11: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM12 - Emissions Related Active DTCs (J1939-73 Diagnostics)
    pub(crate) fn handle_dm12(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM12::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm12_active_dtc_spn = msg.dm12_01spn;
                self.diagnostics.dm12_active_dtc_fmi = msg.dm12_01fmi;
                println!("Received DM12 emissions active DTCs");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM12: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM13 - Stop/Start Broadcast (J1939-73 Diagnostics)
    pub(crate) fn handle_dm13(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM13::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm13_j1939_network_1 = msg.j_1939_network_1;
                self.diagnostics.dm13_suspend_duration = msg.suspend_duration;
                self.diagnostics.dm13_suspend_signal = msg.suspend_signal;
                self.diagnostics.dm13_hold_signal = msg.hold_signal;
                if msg.j_1939_network_1 == 0 {
                    self.broadcast_paused = true;
                    println!("Received DM13 stop broadcast command");
                } else if msg.j_1939_network_1 == 1 {
                    self.broadcast_paused = false;
                    println!("Received DM13 start broadcast command");
                } else {
                    println!("Received DM13 broadcast control");
                }
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM13: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM19 - Calibration Information (J1939-73 Diagnostics)
    pub(crate) fn handle_dm19(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM19::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm19_calibration_verification_number =
                    msg.calibration_verification_number;
                self.diagnostics.dm19_calibration_id_1 = msg.calibration_id_1;
                println!(
                    "Received DM19 calibration info: CVN=0x{:08X}",
                    msg.calibration_verification_number
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM19: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM20 - Monitor Performance Ratio (J1939-73 Diagnostics)
    pub(crate) fn handle_dm20(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM20::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm20_ignition_cycle_counter = msg.ignition_cycle_counter;
                self.diagnostics.dm20_obd_monitoring_cond_encountered =
                    msg.obd_monitoring_cond_encountered;
                self.diagnostics.dm20_spn_of_appl_sys_monitor = msg.sp_nof_appl_sys_monitor;
                self.diagnostics.dm20_appl_sys_monitor_numerator = msg.appl_sys_monitor_numerator;
                self.diagnostics.dm20_appl_sys_monitor_denominator =
                    msg.appl_sys_monitor_denominator;
                println!("Received DM20 monitor performance ratio");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM20: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM21 - Diagnostic Readiness 2 (J1939-73 Diagnostics)
    pub(crate) fn handle_dm21(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM21::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm21_distance_while_mil_activated =
                    msg.distance_while_mi_lis_activated;
                self.diagnostics.dm21_distance_since_dtcs_cleared =
                    msg.distance_since_dt_cs_cleared;
                self.diagnostics.dm21_minutes_run_mil_activated =
                    msg.minutes_run_by_engine_mil_activated;
                self.diagnostics.dm21_time_since_dtcs_cleared = msg.time_since_dt_cs_cleared;
                println!("Received DM21 diagnostic readiness 2");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM21: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM25 - Expanded Freeze Frame (J1939-73 Diagnostics)
    pub(crate) fn handle_dm25(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM25::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm25_expanded_freeze_frame_length =
                    msg.expanded_freeze_frame_length;
                self.diagnostics.dm25_spn = msg.dm25_01spn;
                self.diagnostics.dm25_fmi = msg.dm25_01fmi;
                println!("Received DM25 expanded freeze frame");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM25: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM27 - All Pending DTCs (J1939-73 Diagnostics)
    pub(crate) fn handle_dm27(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM27::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm27_pending_dtc_spn = msg.dm27_01spn;
                self.diagnostics.dm27_pending_dtc_fmi = msg.dm27_01fmi;
                println!("Received DM27 all pending DTCs");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM27: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM28 - Permanent DTCs (J1939-73 Diagnostics)
    pub(crate) fn handle_dm28(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM28::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm28_permanent_dtc_spn = msg.dm28_01spn;
                self.diagnostics.dm28_permanent_dtc_fmi = msg.dm28_01fmi;
                println!("Received DM28 permanent DTCs");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM28: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM29 - Regulated DTC Counts (J1939-73 Diagnostics)
    pub(crate) fn handle_dm29(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM29::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm29_pending_dtc_count = msg.pending_dt_cs;
                self.diagnostics.dm29_all_pending_dtc_count = msg.all_pending_dt_cs;
                self.diagnostics.dm29_mil_on_dtc_count = msg.mil_on_dt_cs;
                self.diagnostics.dm29_previously_mil_on_dtc_count = msg.previously_mil_on_dt_cs;
                self.diagnostics.dm29_permanent_dtc_count = msg.permanent_dt_cs;
                println!(
                    "Received DM29 DTC counts: pending={}, active={}, permanent={}",
                    msg.pending_dt_cs, msg.mil_on_dt_cs, msg.permanent_dt_cs
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM29: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM31 - DTC to Lamp Association (J1939-73 Diagnostics)
    pub(crate) fn handle_dm31(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM31::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm31_spn = msg.dm31_01spn;
                self.diagnostics.dm31_fmi = msg.dm31_01fmi;
                self.diagnostics.dm31_protect_lamp_status = msg.dtc_protect_lamp_support_status;
                self.diagnostics.dm31_warn_lamp_status = msg.dtc_warn_lamp_support_status;
                self.diagnostics.dm31_stop_lamp_status = msg.dtc_stop_lamp_support_status;
                self.diagnostics.dm31_mil_status = msg.dtc_mil_support_status;
                println!("Received DM31 DTC-lamp association");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM31: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM33 - Emission Increasing AECD Active Time (J1939-73 Diagnostics)
    pub(crate) fn handle_dm33(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM33::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm33_aecd_number = msg.aecd_number_1;
                self.diagnostics.dm33_aecd_timer_1 = msg.aecd_engine_hours_1_timer_1;
                self.diagnostics.dm33_aecd_timer_2 = msg.aecd_engine_hours_1_timer_2;
                println!("Received DM33 AECD active time: AECD#={}", msg.aecd_number_1);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM33: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM34 - NTE Status (J1939-73 Diagnostics)
    pub(crate) fn handle_dm34(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM34::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm34_nox_nte_deficiency_area_status =
                    msg.n_ox_nte_deficiency_area_status;
                self.diagnostics.dm34_nox_nte_carve_out_area_status =
                    msg.mnfc_n_ox_nte_carve_out_area_status;
                self.diagnostics.dm34_nox_nte_control_area_status =
                    msg.n_ox_nte_control_area_status;
                self.diagnostics.dm34_pm_nte_deficiency_area_status =
                    msg.pmnte_deficiency_area_status;
                self.diagnostics.dm34_pm_nte_carve_out_area_status =
                    msg.mnfc_pmnte_carve_out_area_status;
                self.diagnostics.dm34_pm_nte_control_area_status = msg.pmnte_control_area_status;
                println!("Received DM34 NTE status");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM34: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM35 - Exhaust Gas Recirculation Diagnostics (J1939-73 Diagnostics)
    pub(crate) fn handle_dm35(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DM35::decode(can_id, data) {
            Ok(msg) => {
                self.diagnostics.dm35_dtc_spn = msg.dm35_01spn;
                self.diagnostics.dm35_dtc_fmi = msg.dm35_01fmi;
                println!("Received DM35 EGR diagnostics");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode DM35: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DM03 - Diagnostic Data Clear/Reset (J1939-73 Diagnostics)
    pub(crate) fn handle_dm03(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DM03 - Diagnostic Data Clear/Reset (J1939-73 Diagnostics)
        // Note: base_id already masks device byte, so this matches all source addresses
        match DM03::decode(can_id, data) {
            Ok(dm03_msg) => {
                if self.diagnostics.dm03_clear_operations_enabled {
                    // Process clear command
                    self.diagnostics.dm03_clear_commands_received += 1;
                    self.diagnostics.dm03_last_clear_timestamp = self.uptime_seconds;

                    // Clear active DTCs (DM01)
                    self.diagnostics.dm01_active_dtc_spn = 0xFFFF;
                    self.diagnostics.dm01_active_dtc_fmi = 0xFF;
                    self.diagnostics.dm01_active_dtc_occurrence_count = 0xFF;
                    self.diagnostics.dm01_active_dtc_conversion_method = 0xFF;

                    // Move active DTCs to previously active (DM02)
                    if self.diagnostics.dm02_previously_active_dtc_spn == 0xFFFF {
                        // Only move if there was an active DTC to clear
                        if self.diagnostics.dm01_fault_injection_enabled {
                            self.diagnostics.dm02_previously_active_dtc_spn = 7945; // Example cleared DTC
                            self.diagnostics.dm02_previously_active_dtc_fmi = 9;
                            self.diagnostics.dm02_previously_active_dtc_occurrence_count = 5;
                            self.diagnostics.dm02_previously_active_dtc_conversion_method = 0;
                        }
                    }

                    // Reset lamp states to off
                    self.diagnostics.dm01_protect_lamp_status = 0;
                    self.diagnostics.dm01_amber_warning_lamp_status = 0;
                    self.diagnostics.dm01_red_stop_lamp_status = 0;
                    self.diagnostics.dm01_malfunction_indicator_lamp_status = 0;

                    println!(
                        "🔧 Received DM03 clear command from device 0x{:02X} - DTCs cleared",
                        dm03_msg.device_id.as_u8()
                    );

                    if self.diagnostics.dm03_auto_response_enabled {
                        println!(
                            "   └─ Auto-response enabled: DM01/DM02 will reflect cleared state"
                        );
                    }
                } else {
                    println!(
                        "🔧 Received DM03 clear command but clear operations are disabled"
                    );
                }
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DM03: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }
}
