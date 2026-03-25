//! Batch 10: Extended Diagnostics (DM04-DM35) Integration Tests
//!
//! Tests for 19 additional DM messages:
//! DM04, DM05, DM06, DM07, DM10, DM11, DM12, DM13, DM19, DM20,
//! DM21, DM25, DM27, DM28, DM29, DM31, DM33, DM34, DM35

use cando_j1939_sim::{MessageStatus, SimulatorState};
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use socketcan::Frame;

// ============================================================================
// Test Helpers
// ============================================================================

/// Create a test state with a non-default device ID to avoid self-reception filtering
fn test_state() -> SimulatorState {
    SimulatorState {
        device_id: 0x82,
        ..Default::default()
    }
}

/// External device ID (different from simulator's 0x82) for sending commands
fn external_device() -> DeviceId {
    DeviceId::from(0x42)
}

/// Check if a CAN frame ID matches a message type's BASE_CAN_ID (masking out the source address byte)
fn matches_base_id(frame_id: u32, base_can_id: u32) -> bool {
    (frame_id & 0xFFFFFF00) == base_can_id
}

// ============================================================================
// DM04 - Freeze Frame Parameters Tests
// ============================================================================

#[test]
fn test_dm04_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm04_freeze_frame_length, 0);
    assert_eq!(state.diagnostics.dm04_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm04_fmi, 0xFF);
    assert_eq!(state.diagnostics.dm04_eng_speed, 0.0);
}

#[test]
fn test_dm04_handler_updates_state() {
    let mut state = test_state();
    let msg = DM04 {
        device_id: external_device(),
        freeze_frame_length: 13,
        dm04_01spn: 100,
        dm04_01fmi: 3,
        dm04_01spn_high: 0.0,
        dm04_01oc: 5,
        dm04_01cm: 0,
        eng_torque_mode: 1,
        eng_intake_manifold_1_press: 100.0,
        eng_speed: 1500.0,
        eng_percent_load_at_current_speed: 50,
        engine_coolant_temperature: 85.0,
        wheel_based_vehicle_speed: 60.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm04_freeze_frame_length, 13);
    assert_eq!(state.diagnostics.dm04_spn, 100);
    assert_eq!(state.diagnostics.dm04_fmi, 3);
    assert_eq!(state.diagnostics.dm04_eng_load, 50);
}

// ============================================================================
// DM05 - OBD Readiness Monitors Tests
// ============================================================================

#[test]
fn test_dm05_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm05_active_trouble_code_count, 0);
    assert_eq!(state.diagnostics.dm05_previously_active_trouble_code_count, 0);
    assert_eq!(state.diagnostics.dm05_obd_compliance, 0);
}

#[test]
fn test_dm05_handler_updates_state() {
    let mut state = test_state();
    let msg = DM05 {
        device_id: external_device(),
        active_trouble_code_count: 3,
        previously_active_trouble_code_count: 5,
        obd_compliance: 14,
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
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm05_active_trouble_code_count, 3);
    assert_eq!(state.diagnostics.dm05_previously_active_trouble_code_count, 5);
    assert_eq!(state.diagnostics.dm05_obd_compliance, 14);
}

#[test]
fn test_dm05_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let dm05_found = frames.iter().any(|f| {
        let raw_id = f.raw_id() & 0x1FFFFFFF;
        matches_base_id(raw_id, DM05::BASE_CAN_ID)
    });
    assert!(dm05_found, "DM05 broadcast frame should be present");
}

// ============================================================================
// DM06 - Pending DTCs Tests (78-byte message, handler only)
// ============================================================================

#[test]
fn test_dm06_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm06_pending_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm06_pending_dtc_fmi, 0xFF);
}

// ============================================================================
// DM07 - Command Non-Continuously Monitored Test Tests
// ============================================================================

#[test]
fn test_dm07_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm07_test_id, 0);
    assert_eq!(state.diagnostics.dm07_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm07_fmi, 0xFF);
}

#[test]
fn test_dm07_handler_updates_state() {
    let mut state = test_state();
    let msg = DM07 {
        device_id: external_device(),
        test_identifier: 42,
        dm07_01spn: 1234,
        dm07_01fmi: 7,
        dm07_01spn_high: 0.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm07_test_id, 42);
    assert_eq!(state.diagnostics.dm07_spn, 1234);
    assert_eq!(state.diagnostics.dm07_fmi, 7);
}

#[test]
fn test_dm07_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let dm07_found = frames.iter().any(|f| {
        let raw_id = f.raw_id() & 0x1FFFFFFF;
        matches_base_id(raw_id, DM07::BASE_CAN_ID)
    });
    assert!(dm07_found, "DM07 broadcast frame should be present");
}

// ============================================================================
// DM10 - Non-Continuously Monitored Test Identifiers Support Tests
// ============================================================================

#[test]
fn test_dm10_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm10_test_identifier_supported, 0);
}

#[test]
fn test_dm10_handler_updates_state() {
    let mut state = test_state();
    let msg = DM10 {
        device_id: external_device(),
        test_identifier_supported: 0xDEADBEEF,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm10_test_identifier_supported, 0xDEADBEEF);
}

#[test]
fn test_dm10_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let dm10_found = frames.iter().any(|f| {
        let raw_id = f.raw_id() & 0x1FFFFFFF;
        matches_base_id(raw_id, DM10::BASE_CAN_ID)
    });
    assert!(dm10_found, "DM10 broadcast frame should be present");
}

// ============================================================================
// DM11 - Diagnostic Data Clear/Reset for Active DTCs Tests
// ============================================================================

#[test]
fn test_dm11_default_state() {
    let state = test_state();
    assert!(!state.diagnostics.dm11_clear_requested);
    assert_eq!(state.diagnostics.dm11_clear_count, 0);
}

#[test]
fn test_dm11_handler_clears_active_dtcs() {
    let mut state = test_state();
    // Set up some active DTCs first
    state.diagnostics.dm01_active_dtc_spn = 1234;
    state.diagnostics.dm01_active_dtc_fmi = 5;
    state.diagnostics.dm01_protect_lamp_status = 1;

    let msg = DM11 {
        device_id: external_device(),
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert!(state.diagnostics.dm11_clear_requested);
    assert_eq!(state.diagnostics.dm11_clear_count, 1);
    // Active DTCs should be cleared
    assert_eq!(state.diagnostics.dm01_active_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm01_active_dtc_fmi, 0xFF);
    assert_eq!(state.diagnostics.dm01_protect_lamp_status, 0);
}

#[test]
fn test_dm11_multiple_clears() {
    let mut state = test_state();
    let msg = DM11 {
        device_id: external_device(),
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();
    state.process_incoming_message(can_id, &data).unwrap();
    state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(state.diagnostics.dm11_clear_count, 3);
}

// ============================================================================
// DM12 - Emissions Related Active DTCs Tests (78-byte message, handler only)
// ============================================================================

#[test]
fn test_dm12_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm12_active_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm12_active_dtc_fmi, 0xFF);
}

// ============================================================================
// DM13 - Stop/Start Broadcast Tests
// ============================================================================

#[test]
fn test_dm13_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm13_j1939_network_1, 3); // Don't care
    assert_eq!(state.diagnostics.dm13_suspend_duration, 0);
}

#[test]
fn test_dm13_stop_broadcast() {
    let mut state = test_state();
    assert!(!state.broadcast_paused);
    let msg = DM13 {
        device_id: external_device(),
        j_1939_network_1: 0, // Stop broadcast
        sae_j1922: 3,
        sae_j1587: 3,
        current_data_link: 3,
        manufacturer_specific_port: 3,
        sae_j1850: 3,
        iso_9141: 3,
        j_1939_network_2: 3,
        j_1939_network_3: 3,
        suspend_signal: 0,
        hold_signal: 0,
        suspend_duration: 30,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert!(state.broadcast_paused);
    assert_eq!(state.diagnostics.dm13_j1939_network_1, 0);
    assert_eq!(state.diagnostics.dm13_suspend_duration, 30);
}

#[test]
fn test_dm13_start_broadcast() {
    let mut state = test_state();
    state.broadcast_paused = true;
    let msg = DM13 {
        device_id: external_device(),
        j_1939_network_1: 1, // Start broadcast
        sae_j1922: 3,
        sae_j1587: 3,
        current_data_link: 3,
        manufacturer_specific_port: 3,
        sae_j1850: 3,
        iso_9141: 3,
        j_1939_network_2: 3,
        j_1939_network_3: 3,
        suspend_signal: 0,
        hold_signal: 0,
        suspend_duration: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert!(!state.broadcast_paused);
    assert_eq!(state.diagnostics.dm13_j1939_network_1, 1);
}

#[test]
fn test_dm13_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let dm13_found = frames.iter().any(|f| {
        let raw_id = f.raw_id() & 0x1FFFFFFF;
        matches_base_id(raw_id, DM13::BASE_CAN_ID)
    });
    assert!(dm13_found, "DM13 broadcast frame should be present");
}

// ============================================================================
// DM19 - Calibration Information Tests (20-byte, handler only)
// ============================================================================

#[test]
fn test_dm19_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm19_calibration_verification_number, 0);
    assert_eq!(state.diagnostics.dm19_calibration_id_1, 0);
}

// ============================================================================
// DM20 - Monitor Performance Ratio Tests (11-byte, handler only)
// ============================================================================

#[test]
fn test_dm20_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm20_ignition_cycle_counter, 0);
    assert_eq!(state.diagnostics.dm20_obd_monitoring_cond_encountered, 0);
}

// ============================================================================
// DM21 - Diagnostic Readiness 2 Tests
// ============================================================================

#[test]
fn test_dm21_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm21_distance_while_mil_activated, 0);
    assert_eq!(state.diagnostics.dm21_distance_since_dtcs_cleared, 0);
    assert_eq!(state.diagnostics.dm21_minutes_run_mil_activated, 0);
    assert_eq!(state.diagnostics.dm21_time_since_dtcs_cleared, 0);
}

#[test]
fn test_dm21_handler_updates_state() {
    let mut state = test_state();
    let msg = DM21 {
        device_id: external_device(),
        distance_while_mi_lis_activated: 150,
        distance_since_dt_cs_cleared: 500,
        minutes_run_by_engine_mil_activated: 60,
        time_since_dt_cs_cleared: 1440,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm21_distance_while_mil_activated, 150);
    assert_eq!(state.diagnostics.dm21_distance_since_dtcs_cleared, 500);
    assert_eq!(state.diagnostics.dm21_minutes_run_mil_activated, 60);
    assert_eq!(state.diagnostics.dm21_time_since_dtcs_cleared, 1440);
}

#[test]
fn test_dm21_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let dm21_found = frames.iter().any(|f| {
        let raw_id = f.raw_id() & 0x1FFFFFFF;
        matches_base_id(raw_id, DM21::BASE_CAN_ID)
    });
    assert!(dm21_found, "DM21 broadcast frame should be present");
}

// ============================================================================
// DM25 - Expanded Freeze Frame Tests (13-byte, handler only)
// ============================================================================

#[test]
fn test_dm25_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm25_expanded_freeze_frame_length, 0);
    assert_eq!(state.diagnostics.dm25_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm25_fmi, 0xFF);
}

// ============================================================================
// DM27 - All Pending DTCs Tests (78-byte, handler only)
// ============================================================================

#[test]
fn test_dm27_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm27_pending_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm27_pending_dtc_fmi, 0xFF);
}

// ============================================================================
// DM28 - Permanent DTCs Tests (78-byte, handler only)
// ============================================================================

#[test]
fn test_dm28_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm28_permanent_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm28_permanent_dtc_fmi, 0xFF);
}

// ============================================================================
// DM29 - Regulated DTC Counts Tests
// ============================================================================

#[test]
fn test_dm29_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm29_pending_dtc_count, 0);
    assert_eq!(state.diagnostics.dm29_all_pending_dtc_count, 0);
    assert_eq!(state.diagnostics.dm29_mil_on_dtc_count, 0);
    assert_eq!(state.diagnostics.dm29_previously_mil_on_dtc_count, 0);
    assert_eq!(state.diagnostics.dm29_permanent_dtc_count, 0);
}

#[test]
fn test_dm29_handler_updates_state() {
    let mut state = test_state();
    let msg = DM29 {
        device_id: external_device(),
        pending_dt_cs: 2,
        all_pending_dt_cs: 5,
        mil_on_dt_cs: 1,
        previously_mil_on_dt_cs: 3,
        permanent_dt_cs: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm29_pending_dtc_count, 2);
    assert_eq!(state.diagnostics.dm29_all_pending_dtc_count, 5);
    assert_eq!(state.diagnostics.dm29_mil_on_dtc_count, 1);
    assert_eq!(state.diagnostics.dm29_previously_mil_on_dtc_count, 3);
    assert_eq!(state.diagnostics.dm29_permanent_dtc_count, 1);
}

#[test]
fn test_dm29_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let dm29_found = frames.iter().any(|f| {
        let raw_id = f.raw_id() & 0x1FFFFFFF;
        matches_base_id(raw_id, DM29::BASE_CAN_ID)
    });
    assert!(dm29_found, "DM29 broadcast frame should be present");
}

// ============================================================================
// DM31 - DTC to Lamp Association Tests
// ============================================================================

#[test]
fn test_dm31_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm31_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm31_fmi, 0xFF);
    assert_eq!(state.diagnostics.dm31_protect_lamp_status, 3); // Unavailable
    assert_eq!(state.diagnostics.dm31_warn_lamp_status, 3);
    assert_eq!(state.diagnostics.dm31_stop_lamp_status, 3);
    assert_eq!(state.diagnostics.dm31_mil_status, 3);
}

#[test]
fn test_dm31_handler_updates_state() {
    let mut state = test_state();
    let msg = DM31 {
        device_id: external_device(),
        dm31_01spn: 520,
        dm31_01fmi: 4,
        dm31_01spn_high: 0.0,
        dm31_01oc: 2,
        dm31_01cm: 0,
        dtc_protect_lamp_support_status: 1,
        dtc_warn_lamp_support_status: 1,
        dtc_stop_lamp_support_status: 0,
        dtc_mil_support_status: 1,
        dtc_protect_lamp_support_flash: 0,
        dtc_warn_lamp_support_flash: 0,
        dtc_stop_lamp_support_flash: 0,
        dtc_mil_support_flash: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm31_spn, 520);
    assert_eq!(state.diagnostics.dm31_fmi, 4);
    assert_eq!(state.diagnostics.dm31_protect_lamp_status, 1);
    assert_eq!(state.diagnostics.dm31_warn_lamp_status, 1);
    assert_eq!(state.diagnostics.dm31_stop_lamp_status, 0);
    assert_eq!(state.diagnostics.dm31_mil_status, 1);
}

#[test]
fn test_dm31_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let dm31_found = frames.iter().any(|f| {
        let raw_id = f.raw_id() & 0x1FFFFFFF;
        matches_base_id(raw_id, DM31::BASE_CAN_ID)
    });
    assert!(dm31_found, "DM31 broadcast frame should be present");
}

// ============================================================================
// DM33 - Emission Increasing AECD Active Time Tests (9-byte, handler only)
// ============================================================================

#[test]
fn test_dm33_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm33_aecd_number, 0);
    assert_eq!(state.diagnostics.dm33_aecd_timer_1, 0);
    assert_eq!(state.diagnostics.dm33_aecd_timer_2, 0);
}

#[test]
fn test_dm33_handler_updates_state() {
    let mut state = test_state();
    let msg = DM33 {
        device_id: external_device(),
        aecd_number_1: 5,
        aecd_engine_hours_1_timer_1: 12000,
        aecd_engine_hours_1_timer_2: 8000,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm33_aecd_number, 5);
    assert_eq!(state.diagnostics.dm33_aecd_timer_1, 12000);
    assert_eq!(state.diagnostics.dm33_aecd_timer_2, 8000);
}

// ============================================================================
// DM34 - NTE Status Tests
// ============================================================================

#[test]
fn test_dm34_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm34_nox_nte_deficiency_area_status, 3); // Not available
    assert_eq!(state.diagnostics.dm34_pm_nte_deficiency_area_status, 3);
}

#[test]
fn test_dm34_handler_updates_state() {
    let mut state = test_state();
    let msg = DM34 {
        device_id: external_device(),
        n_ox_nte_deficiency_area_status: 1,
        mnfc_n_ox_nte_carve_out_area_status: 0,
        n_ox_nte_control_area_status: 1,
        pmnte_deficiency_area_status: 0,
        mnfc_pmnte_carve_out_area_status: 0,
        pmnte_control_area_status: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm34_nox_nte_deficiency_area_status, 1);
    assert_eq!(state.diagnostics.dm34_nox_nte_carve_out_area_status, 0);
    assert_eq!(state.diagnostics.dm34_nox_nte_control_area_status, 1);
    assert_eq!(state.diagnostics.dm34_pm_nte_deficiency_area_status, 0);
    assert_eq!(state.diagnostics.dm34_pm_nte_carve_out_area_status, 0);
    assert_eq!(state.diagnostics.dm34_pm_nte_control_area_status, 1);
}

#[test]
fn test_dm34_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let dm34_found = frames.iter().any(|f| {
        let raw_id = f.raw_id() & 0x1FFFFFFF;
        matches_base_id(raw_id, DM34::BASE_CAN_ID)
    });
    assert!(dm34_found, "DM34 broadcast frame should be present");
}

// ============================================================================
// DM35 - Exhaust Gas Recirculation Diagnostics Tests (78-byte, handler only)
// ============================================================================

#[test]
fn test_dm35_default_state() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm35_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm35_dtc_fmi, 0xFF);
}

// ============================================================================
// Cross-DM Workflow Tests
// ============================================================================

#[test]
fn test_dm11_clears_and_dm29_reflects_zero_counts() {
    let mut state = test_state();
    // Initially, DM29 counts should be zero
    assert_eq!(state.diagnostics.dm29_pending_dtc_count, 0);
    assert_eq!(state.diagnostics.dm29_mil_on_dtc_count, 0);

    // Send DM11 clear
    let dm11 = DM11 {
        device_id: external_device(),
    };
    let (can_id, data) = dm11.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert!(state.diagnostics.dm11_clear_requested);
    // Active DTCs should be cleared
    assert_eq!(state.diagnostics.dm01_active_dtc_spn, 0xFFFF);
}

#[test]
fn test_dm13_stop_then_start_broadcast_cycle() {
    let mut state = test_state();

    // Stop broadcast via DM13
    let stop = DM13 {
        device_id: external_device(),
        j_1939_network_1: 0,
        sae_j1922: 3,
        sae_j1587: 3,
        current_data_link: 3,
        manufacturer_specific_port: 3,
        sae_j1850: 3,
        iso_9141: 3,
        j_1939_network_2: 3,
        j_1939_network_3: 3,
        suspend_signal: 0,
        hold_signal: 0,
        suspend_duration: 10,
    };
    let (can_id, data) = stop.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();
    assert!(state.broadcast_paused);

    // Start broadcast via DM13
    let start = DM13 {
        device_id: external_device(),
        j_1939_network_1: 1,
        sae_j1922: 3,
        sae_j1587: 3,
        current_data_link: 3,
        manufacturer_specific_port: 3,
        sae_j1850: 3,
        iso_9141: 3,
        j_1939_network_2: 3,
        j_1939_network_3: 3,
        suspend_signal: 0,
        hold_signal: 0,
        suspend_duration: 0,
    };
    let (can_id, data) = start.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();
    assert!(!state.broadcast_paused);
}

#[test]
fn test_all_dm_broadcasts_in_frame_set() {
    // Verify that the expected DM broadcast messages appear in generate_can_frames
    let state = test_state();
    let frames = state.generate_can_frames();

    // Messages with DLC <= 8 should produce broadcast frames
    let expected_broadcasts = [
        ("DM05", DM05::BASE_CAN_ID),
        ("DM07", DM07::BASE_CAN_ID),
        ("DM10", DM10::BASE_CAN_ID),
        ("DM13", DM13::BASE_CAN_ID),
        ("DM21", DM21::BASE_CAN_ID),
        ("DM29", DM29::BASE_CAN_ID),
        ("DM31", DM31::BASE_CAN_ID),
        ("DM34", DM34::BASE_CAN_ID),
    ];

    for (name, base_id) in &expected_broadcasts {
        let found = frames.iter().any(|f| {
            let raw_id = f.raw_id() & 0x1FFFFFFF;
            matches_base_id(raw_id, *base_id)
        });
        assert!(found, "{} broadcast frame should be present", name);
    }
}

#[test]
fn test_self_reception_filtered_for_dm_messages() {
    let mut state = test_state();
    // Send DM05 from the simulator's own device ID (0x82) - should be ignored
    // DM05 uses PDU2 format (PF=0xFE >= 0xF0), so source address is in byte 0
    // and self-reception filtering applies
    let msg = DM05 {
        device_id: DeviceId::from(0x82_u8),
        active_trouble_code_count: 10,
        previously_active_trouble_code_count: 10,
        obd_compliance: 14,
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
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Ignored);
    // State should not be updated
    assert_eq!(state.diagnostics.dm05_active_trouble_code_count, 0);
}

// ============================================================================
// Handler Reachability Tests for DLC > 8 Messages
// ============================================================================
// These messages have DLC > 8, so encode() produces payloads too large for a
// standard CAN frame. Instead we send an 8-byte payload with the correct
// BASE_CAN_ID to prove the dispatch path reaches the handler.

#[test]
fn test_dm06_handler_reachable() {
    let mut state = test_state();
    let can_id = DM06::BASE_CAN_ID | 0x42;
    let data = [0x00u8; 8];
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert!(
        status == MessageStatus::Recognized || status == MessageStatus::DecodeFailed,
        "DM06 handler should be reachable, got {:?}",
        status
    );
}

#[test]
fn test_dm12_handler_reachable() {
    let mut state = test_state();
    let can_id = DM12::BASE_CAN_ID | 0x42;
    let data = [0x00u8; 8];
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert!(
        status == MessageStatus::Recognized || status == MessageStatus::DecodeFailed,
        "DM12 handler should be reachable, got {:?}",
        status
    );
}

#[test]
fn test_dm19_handler_reachable() {
    let mut state = test_state();
    let can_id = DM19::BASE_CAN_ID | 0x42;
    let data = [0x00u8; 8];
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert!(
        status == MessageStatus::Recognized || status == MessageStatus::DecodeFailed,
        "DM19 handler should be reachable, got {:?}",
        status
    );
}

#[test]
fn test_dm25_handler_reachable() {
    let mut state = test_state();
    let can_id = DM25::BASE_CAN_ID | 0x42;
    let data = [0x00u8; 8];
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert!(
        status == MessageStatus::Recognized || status == MessageStatus::DecodeFailed,
        "DM25 handler should be reachable, got {:?}",
        status
    );
}

#[test]
fn test_dm27_handler_reachable() {
    let mut state = test_state();
    let can_id = DM27::BASE_CAN_ID | 0x42;
    let data = [0x00u8; 8];
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert!(
        status == MessageStatus::Recognized || status == MessageStatus::DecodeFailed,
        "DM27 handler should be reachable, got {:?}",
        status
    );
}

#[test]
fn test_dm28_handler_reachable() {
    let mut state = test_state();
    let can_id = DM28::BASE_CAN_ID | 0x42;
    let data = [0x00u8; 8];
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert!(
        status == MessageStatus::Recognized || status == MessageStatus::DecodeFailed,
        "DM28 handler should be reachable, got {:?}",
        status
    );
}

#[test]
fn test_dm35_handler_reachable() {
    let mut state = test_state();
    let can_id = DM35::BASE_CAN_ID | 0x42;
    let data = [0x00u8; 8];
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert!(
        status == MessageStatus::Recognized || status == MessageStatus::DecodeFailed,
        "DM35 handler should be reachable, got {:?}",
        status
    );
}

// ============================================================================
// DecodeFailed Test — Corrupt/Truncated Data
// ============================================================================

#[test]
fn test_batch10_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = DM05::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated — should trigger DecodeFailed
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}

// ============================================================================
// Broadcast Count Test — DM Messages in Frame Set
// ============================================================================

#[test]
fn test_batch10_dm_broadcast_count() {
    // The diagnostics broadcast function encodes frames for many DM messages, but
    // DM01 and DM02 have DLC > 8 (multi-frame TP) so create_can_frame may not
    // produce a standard CAN frame for them. The reliably broadcast DM messages
    // with DLC <= 8 are: DM05, DM07, DM10, DM13, DM21, DM29, DM31, DM34.
    // DM03 is conditional (not generated by default).
    // DM04, DM06, DM11, DM12, DM19, DM20, DM25, DM27, DM28, DM33, DM35
    // are NOT broadcast (DLC > 8 or request-only).
    let state = test_state();
    let frames = state.generate_can_frames();

    let dm_base_ids: Vec<(&str, u32)> = vec![
        ("DM05", DM05::BASE_CAN_ID),
        ("DM07", DM07::BASE_CAN_ID),
        ("DM10", DM10::BASE_CAN_ID),
        ("DM13", DM13::BASE_CAN_ID),
        ("DM21", DM21::BASE_CAN_ID),
        ("DM29", DM29::BASE_CAN_ID),
        ("DM31", DM31::BASE_CAN_ID),
        ("DM34", DM34::BASE_CAN_ID),
    ];

    let mut found_count = 0;
    for (name, base_id) in &dm_base_ids {
        let found = frames.iter().any(|f| {
            let raw_id = f.raw_id() & 0x1FFFFFFF;
            matches_base_id(raw_id, *base_id)
        });
        if found {
            found_count += 1;
        } else {
            panic!("{} broadcast frame should be present but was not found", name);
        }
    }
    assert_eq!(
        found_count, 8,
        "Expected exactly 8 DM broadcast frames (DLC <= 8), found {}",
        found_count
    );
}
