use crate::SimulatorState;
use cando_messages::common::DeviceId;
#[allow(unused_imports)]
use cando_messages::j1939::DM03;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_diagnostics_frames(
        &self,
        frames: &mut Vec<CanFrame>,
        device_id: DeviceId,
    ) {
        // ============================================================================
        // DM01 - Active Diagnostic Trouble Codes (J1939-73 Diagnostics)
        // ============================================================================

        let dm01 = DM01 {
            device_id,
            protect_lamp_status: self.diagnostics.dm01_protect_lamp_status,
            amber_warning_lamp_status: self.diagnostics.dm01_amber_warning_lamp_status,
            red_stop_lamp_status: self.diagnostics.dm01_red_stop_lamp_status,
            malfunction_indicator_lamp_status: self
                .diagnostics
                .dm01_malfunction_indicator_lamp_status,
            flash_protect_lamp: self.diagnostics.dm01_flash_protect_lamp,
            flash_amber_warning_lamp: self.diagnostics.dm01_flash_amber_warning_lamp,
            flash_red_stop_lamp: self.diagnostics.dm01_flash_red_stop_lamp,
            flash_malfunc_indicator_lamp: self.diagnostics.dm01_flash_malfunction_indicator_lamp,
            dm01_01spn: self.diagnostics.dm01_active_dtc_spn,
            dm01_01spn_high: self.diagnostics.dm01_active_dtc_spn_high,
            dm01_01fmi: self.diagnostics.dm01_active_dtc_fmi,
            dm01_01oc: self.diagnostics.dm01_active_dtc_occurrence_count,
            dm01_01cm: self.diagnostics.dm01_active_dtc_conversion_method,
            // Initialize all remaining DTCs to "not available"
            dm01_02spn: 0xFFFF,
            dm01_02fmi: 0xFF,
            dm01_02oc: 0xFF,
            dm01_02cm: 0xFF,
            dm01_02spn_high: 0.0,
            dm01_03spn: 0xFFFF,
            dm01_03fmi: 0xFF,
            dm01_03oc: 0xFF,
            dm01_03cm: 0xFF,
            dm01_03spn_high: 0.0,
            dm01_04spn: 0xFFFF,
            dm01_04fmi: 0xFF,
            dm01_04oc: 0xFF,
            dm01_04cm: 0xFF,
            dm01_04spn_high: 0.0,
            dm01_05spn: 0xFFFF,
            dm01_05fmi: 0xFF,
            dm01_05oc: 0xFF,
            dm01_05cm: 0xFF,
            dm01_05spn_high: 0.0,
            dm01_06spn: 0xFFFF,
            dm01_06fmi: 0xFF,
            dm01_06oc: 0xFF,
            dm01_06cm: 0xFF,
            dm01_06spn_high: 0.0,
            dm01_07spn: 0xFFFF,
            dm01_07fmi: 0xFF,
            dm01_07oc: 0xFF,
            dm01_07cm: 0xFF,
            dm01_07spn_high: 0.0,
            dm01_08spn: 0xFFFF,
            dm01_08fmi: 0xFF,
            dm01_08oc: 0xFF,
            dm01_08cm: 0xFF,
            dm01_08spn_high: 0.0,
            dm01_09spn: 0xFFFF,
            dm01_09fmi: 0xFF,
            dm01_09oc: 0xFF,
            dm01_09cm: 0xFF,
            dm01_09spn_high: 0.0,
            dm01_10spn: 0xFFFF,
            dm01_10fmi: 0xFF,
            dm01_10oc: 0xFF,
            dm01_10cm: 0xFF,
            dm01_10spn_high: 0.0,
            dm01_11spn: 0xFFFF,
            dm01_11fmi: 0xFF,
            dm01_11oc: 0xFF,
            dm01_11cm: 0xFF,
            dm01_11spn_high: 0.0,
            dm01_12spn: 0xFFFF,
            dm01_12fmi: 0xFF,
            dm01_12oc: 0xFF,
            dm01_12cm: 0xFF,
            dm01_12spn_high: 0.0,
            dm01_13spn: 0xFFFF,
            dm01_13fmi: 0xFF,
            dm01_13oc: 0xFF,
            dm01_13cm: 0xFF,
            dm01_13spn_high: 0.0,
            dm01_14spn: 0xFFFF,
            dm01_14fmi: 0xFF,
            dm01_14oc: 0xFF,
            dm01_14cm: 0xFF,
            dm01_14spn_high: 0.0,
            dm01_15spn: 0xFFFF,
            dm01_15fmi: 0xFF,
            dm01_15oc: 0xFF,
            dm01_15cm: 0xFF,
            dm01_15spn_high: 0.0,
            dm01_16spn: 0xFFFF,
            dm01_16fmi: 0xFF,
            dm01_16oc: 0xFF,
            dm01_16cm: 0xFF,
            dm01_16spn_high: 0.0,
            dm01_17spn: 0xFFFF,
            dm01_17fmi: 0xFF,
            dm01_17oc: 0xFF,
            dm01_17cm: 0xFF,
            dm01_17spn_high: 0.0,
            dm01_18spn: 0xFFFF,
            dm01_18fmi: 0xFF,
            dm01_18oc: 0xFF,
            dm01_18cm: 0xFF,
            dm01_18spn_high: 0.0,
            dm01_19spn: 0xFFFF,
            dm01_19fmi: 0xFF,
            dm01_19oc: 0xFF,
            dm01_19cm: 0xFF,
            dm01_19spn_high: 0.0,
        };

        let (can_id, data) = dm01.encode().unwrap();
        if let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
            frames.push(frame);
        }

        // ============================================================================
        // DM02 - Previously Active Diagnostic Trouble Codes (J1939-73 Diagnostics)
        // ============================================================================

        let dm02 = DM02 {
            device_id,
            protect_lamp_status: self.diagnostics.dm02_protect_lamp_status,
            amber_warning_lamp_status: self.diagnostics.dm02_amber_warning_lamp_status,
            red_stop_lamp_status: self.diagnostics.dm02_red_stop_lamp_status,
            malfunction_indicator_lamp_status: self
                .diagnostics
                .dm02_malfunction_indicator_lamp_status,
            flash_protect_lamp: self.diagnostics.dm02_flash_protect_lamp,
            flash_amber_warning_lamp: self.diagnostics.dm02_flash_amber_warning_lamp,
            flash_red_stop_lamp: self.diagnostics.dm02_flash_red_stop_lamp,
            flash_malfunc_indicator_lamp: self.diagnostics.dm02_flash_malfunction_indicator_lamp,
            dm02_01spn: self.diagnostics.dm02_previously_active_dtc_spn,
            dm02_01spn_high: self.diagnostics.dm02_previously_active_dtc_spn_high,
            dm02_01fmi: self.diagnostics.dm02_previously_active_dtc_fmi,
            dm02_01oc: self.diagnostics.dm02_previously_active_dtc_occurrence_count,
            dm02_01cm: self.diagnostics.dm02_previously_active_dtc_conversion_method,
            // Initialize all remaining DTCs to "not available"
            dm02_02spn: 0xFFFF,
            dm02_02fmi: 0xFF,
            dm02_02oc: 0xFF,
            dm02_02cm: 0xFF,
            dm02_02spn_high: 0.0,
            dm02_03spn: 0xFFFF,
            dm02_03fmi: 0xFF,
            dm02_03oc: 0xFF,
            dm02_03cm: 0xFF,
            dm02_03spn_high: 0.0,
            dm02_04spn: 0xFFFF,
            dm02_04fmi: 0xFF,
            dm02_04oc: 0xFF,
            dm02_04cm: 0xFF,
            dm02_04spn_high: 0.0,
            dm02_05spn: 0xFFFF,
            dm02_05fmi: 0xFF,
            dm02_05oc: 0xFF,
            dm02_05cm: 0xFF,
            dm02_05spn_high: 0.0,
            dm02_06spn: 0xFFFF,
            dm02_06fmi: 0xFF,
            dm02_06oc: 0xFF,
            dm02_06cm: 0xFF,
            dm02_06spn_high: 0.0,
            dm02_07spn: 0xFFFF,
            dm02_07fmi: 0xFF,
            dm02_07oc: 0xFF,
            dm02_07cm: 0xFF,
            dm02_07spn_high: 0.0,
            dm02_08spn: 0xFFFF,
            dm02_08fmi: 0xFF,
            dm02_08oc: 0xFF,
            dm02_08cm: 0xFF,
            dm02_08spn_high: 0.0,
            dm02_09spn: 0xFFFF,
            dm02_09fmi: 0xFF,
            dm02_09oc: 0xFF,
            dm02_09cm: 0xFF,
            dm02_09spn_high: 0.0,
            dm02_10spn: 0xFFFF,
            dm02_10fmi: 0xFF,
            dm02_10oc: 0xFF,
            dm02_10cm: 0xFF,
            dm02_10spn_high: 0.0,
            dm02_11spn: 0xFFFF,
            dm02_11fmi: 0xFF,
            dm02_11oc: 0xFF,
            dm02_11cm: 0xFF,
            dm02_11spn_high: 0.0,
            dm02_12spn: 0xFFFF,
            dm02_12fmi: 0xFF,
            dm02_12oc: 0xFF,
            dm02_12cm: 0xFF,
            dm02_12spn_high: 0.0,
            dm02_13spn: 0xFFFF,
            dm02_13fmi: 0xFF,
            dm02_13oc: 0xFF,
            dm02_13cm: 0xFF,
            dm02_13spn_high: 0.0,
            dm02_14spn: 0xFFFF,
            dm02_14fmi: 0xFF,
            dm02_14oc: 0xFF,
            dm02_14cm: 0xFF,
            dm02_14spn_high: 0.0,
            dm02_15spn: 0xFFFF,
            dm02_15fmi: 0xFF,
            dm02_15oc: 0xFF,
            dm02_15cm: 0xFF,
            dm02_15spn_high: 0.0,
            dm02_16spn: 0xFFFF,
            dm02_16fmi: 0xFF,
            dm02_16oc: 0xFF,
            dm02_16cm: 0xFF,
            dm02_16spn_high: 0.0,
            dm02_17spn: 0xFFFF,
            dm02_17fmi: 0xFF,
            dm02_17oc: 0xFF,
            dm02_17cm: 0xFF,
            dm02_17spn_high: 0.0,
            dm02_18spn: 0xFFFF,
            dm02_18fmi: 0xFF,
            dm02_18oc: 0xFF,
            dm02_18cm: 0xFF,
            dm02_18spn_high: 0.0,
            dm02_19spn: 0xFFFF,
            dm02_19fmi: 0xFF,
            dm02_19oc: 0xFF,
            dm02_19cm: 0xFF,
            dm02_19spn_high: 0.0,
        };

        let (can_id, data) = dm02.encode().unwrap();
        if let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
            frames.push(frame);
        }

        // Generate DM03 command if enabled and interval elapsed
        if self.diagnostics.dm03_command_generation_enabled
            && self.diagnostics.dm03_command_interval_seconds > 0
            && (self.uptime_seconds - self.diagnostics.dm03_last_send_timestamp)
                >= self.diagnostics.dm03_command_interval_seconds
        {
            let dm03_msg = DM03 {
                device_id: DeviceId::from(self.diagnostics.dm03_target_device_id),
            };

            if let Ok((can_id, data)) = dm03_msg.encode()
                && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
            {
                frames.push(frame);
            }
        }

        // ============================================================================
        // DM05 - OBD Readiness Monitors (J1939-73 Diagnostics)
        // ============================================================================
        let dm05 = DM05 {
            device_id,
            active_trouble_code_count: self.diagnostics.dm05_active_trouble_code_count,
            previously_active_trouble_code_count: self
                .diagnostics
                .dm05_previously_active_trouble_code_count,
            obd_compliance: self.diagnostics.dm05_obd_compliance,
            misfire_monitoring_support: 0,
            fuel_system_monitoring_support: 0,
            comprehensive_component_mon_supp: 0,
            misfire_monitoring_status: 0,
            fuel_system_monitoring_status: 0,
            comprehensive_comp_mon_status: 0,
            catalyst_mon_supp: 0,
            heated_catalyst_mon_supp: 0,
            evaporative_system_mon_supp: 0,
            second_air_system_mon_supp: 0,
            ac_system_refrigerant_mon_supp: 0,
            oxygen_sensor_mon_supp: 0,
            oxygen_sensor_heater_mon_supp: 0,
            egr_system_monitoring_supp: 0,
            cold_start_aid_system_mon_supp: 0,
            boost_pressure_control_system_suppor: 0,
            diesel_particulate_filter_support: 0,
            n_ox_converting_catalyst_adsorber_sup: 0,
            nmhc_converting_catalyst_support: 0,
            catalyst_mon_status: 0,
            heated_catalyst_mon_status: 0,
            evaporative_system_mon_status: 0,
            second_air_system_mon_status: 0,
            ac_system_refrigerant_mon_status: 0,
            oxygen_sensor_mon_status: 0,
            oxygen_sensor_heater_mon_status: 0,
            egr_system_monitoring_status: 0,
            cold_start_aid_system_mon_status: 0,
            boost_pressure_control_system_status: 0,
            diesel_particulate_filter_status: 0,
            n_ox_converting_catalyst_adsorber_sta: 0,
            nmhc_converting_catalyst_status: 0,
        };
        if let Ok((can_id, data)) = dm05.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
                frames.push(frame);
            }

        // ============================================================================
        // DM07 - Command Non-Continuously Monitored Test (J1939-73 Diagnostics)
        // ============================================================================
        let dm07 = DM07 {
            device_id,
            test_identifier: self.diagnostics.dm07_test_id,
            dm07_01spn: self.diagnostics.dm07_spn,
            dm07_01fmi: self.diagnostics.dm07_fmi,
            dm07_01spn_high: 0.0,
        };
        if let Ok((can_id, data)) = dm07.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
                frames.push(frame);
            }

        // ============================================================================
        // DM10 - Non-Continuously Monitored Test Identifiers Support (J1939-73 Diagnostics)
        // ============================================================================
        let dm10 = DM10 {
            device_id,
            test_identifier_supported: self.diagnostics.dm10_test_identifier_supported,
        };
        if let Ok((can_id, data)) = dm10.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
                frames.push(frame);
            }

        // ============================================================================
        // DM13 - Stop/Start Broadcast (J1939-73 Diagnostics)
        // ============================================================================
        let dm13 = DM13 {
            device_id,
            j_1939_network_1: self.diagnostics.dm13_j1939_network_1,
            sae_j1922: 3,
            sae_j1587: 3,
            current_data_link: 3,
            manufacturer_specific_port: 3,
            sae_j1850: 3,
            iso_9141: 3,
            j_1939_network_2: 3,
            j_1939_network_3: 3,
            suspend_signal: self.diagnostics.dm13_suspend_signal,
            hold_signal: self.diagnostics.dm13_hold_signal,
            suspend_duration: self.diagnostics.dm13_suspend_duration,
        };
        if let Ok((can_id, data)) = dm13.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
                frames.push(frame);
            }

        // ============================================================================
        // DM21 - Diagnostic Readiness 2 (J1939-73 Diagnostics)
        // ============================================================================
        let dm21 = DM21 {
            device_id,
            distance_while_mi_lis_activated: self.diagnostics.dm21_distance_while_mil_activated,
            distance_since_dt_cs_cleared: self.diagnostics.dm21_distance_since_dtcs_cleared,
            minutes_run_by_engine_mil_activated: self.diagnostics.dm21_minutes_run_mil_activated,
            time_since_dt_cs_cleared: self.diagnostics.dm21_time_since_dtcs_cleared,
        };
        if let Ok((can_id, data)) = dm21.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
                frames.push(frame);
            }

        // ============================================================================
        // DM29 - Regulated DTC Counts (J1939-73 Diagnostics)
        // ============================================================================
        let dm29 = DM29 {
            device_id,
            pending_dt_cs: self.diagnostics.dm29_pending_dtc_count,
            all_pending_dt_cs: self.diagnostics.dm29_all_pending_dtc_count,
            mil_on_dt_cs: self.diagnostics.dm29_mil_on_dtc_count,
            previously_mil_on_dt_cs: self.diagnostics.dm29_previously_mil_on_dtc_count,
            permanent_dt_cs: self.diagnostics.dm29_permanent_dtc_count,
        };
        if let Ok((can_id, data)) = dm29.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
                frames.push(frame);
            }

        // ============================================================================
        // DM31 - DTC to Lamp Association (J1939-73 Diagnostics)
        // ============================================================================
        let dm31 = DM31 {
            device_id,
            dm31_01spn: self.diagnostics.dm31_spn,
            dm31_01fmi: self.diagnostics.dm31_fmi,
            dm31_01spn_high: 0.0,
            dm31_01oc: 0xFF,
            dm31_01cm: 0xFF,
            dtc_protect_lamp_support_status: self.diagnostics.dm31_protect_lamp_status,
            dtc_warn_lamp_support_status: self.diagnostics.dm31_warn_lamp_status,
            dtc_stop_lamp_support_status: self.diagnostics.dm31_stop_lamp_status,
            dtc_mil_support_status: self.diagnostics.dm31_mil_status,
            dtc_protect_lamp_support_flash: 0,
            dtc_warn_lamp_support_flash: 0,
            dtc_stop_lamp_support_flash: 0,
            dtc_mil_support_flash: 0,
        };
        if let Ok((can_id, data)) = dm31.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
                frames.push(frame);
            }

        // ============================================================================
        // DM34 - NTE Status (J1939-73 Diagnostics)
        // ============================================================================
        let dm34 = DM34 {
            device_id,
            n_ox_nte_deficiency_area_status: self.diagnostics.dm34_nox_nte_deficiency_area_status,
            mnfc_n_ox_nte_carve_out_area_status: self
                .diagnostics
                .dm34_nox_nte_carve_out_area_status,
            n_ox_nte_control_area_status: self.diagnostics.dm34_nox_nte_control_area_status,
            pmnte_deficiency_area_status: self.diagnostics.dm34_pm_nte_deficiency_area_status,
            mnfc_pmnte_carve_out_area_status: self.diagnostics.dm34_pm_nte_carve_out_area_status,
            pmnte_control_area_status: self.diagnostics.dm34_pm_nte_control_area_status,
        };
        if let Ok((can_id, data)) = dm34.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended) {
                frames.push(frame);
            }

        // Note: DM04, DM06, DM11, DM12, DM19, DM20, DM25, DM27, DM28, DM33, DM35 are not
        // broadcast periodically - they have DLC > 8 bytes or are request-only messages.
        // They are handled via process_incoming_message for receive, and their state
        // is tracked for test verification.
    }
}
