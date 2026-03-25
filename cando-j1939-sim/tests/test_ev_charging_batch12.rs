//! EV Charging & HV Bus Batch 12 Tests
//!
//! Tests for all 21 EV Charging and HV Bus messages:
//! EVDCS1, EVDCTGT, EVDCLIM1, EVDCLIM2, EVDCCIP, EVSE1CS1, EVSE1CC1,
//! EVSEC1, EVSEDCS1, EVSES1, EVSES2, EVC, EVEI, EVOI1,
//! HVBCS1, HVBCS2, HVBCS3, HVBCC1, HVBCC2, HVBI, EVVT

use cando_j1939_sim::{MessageStatus, SimulatorState};
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use socketcan::{EmbeddedFrame, Frame};

// ============================================================================
// Test Helpers
// ============================================================================

fn test_state() -> SimulatorState {
    SimulatorState {
        device_id: 0x82,
        ..Default::default()
    }
}

fn external_device() -> DeviceId {
    DeviceId::from(0x42)
}

fn assert_float_near(actual: f64, expected: f64, tolerance: f64, field_name: &str) {
    assert!(
        (actual - expected).abs() < tolerance,
        "{}: expected ~{}, got {} (tolerance {})",
        field_name,
        expected,
        actual,
        tolerance
    );
}

fn matches_base_id(frame_id: u32, base_can_id: u32) -> bool {
    (frame_id & 0xFFFFFF00) == base_can_id
}

// ============================================================================
// Handler Tests - EV DC Charging Messages
// ============================================================================

#[test]
fn test_evdcs1_handler() {
    let mut state = test_state();
    let msg = EVDCS1 {
        device_id: external_device(),
        ev_cabin_conditioning_flag: 1,
        ev_ress_conditioning_flag: 1,
        ev_error_code: 2,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.evdcs1_cabin_conditioning_flag, 1);
    assert_eq!(state.ev_charging.evdcs1_ress_conditioning_flag, 1);
    assert_eq!(state.ev_charging.evdcs1_error_code, 2);
}

#[test]
fn test_evdctgt_handler() {
    let mut state = test_state();
    let msg = EVDCTGT {
        device_id: external_device(),
        dc_charging_target_crc: 42,
        dc_charging_target_sequence_counter: 5,
        dc_target_charging_voltage: 800.0,
        dc_target_charging_current: 250.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.ev_charging.evdctgt_target_voltage, 800.0, 1.0, "target_voltage");
    assert_float_near(state.ev_charging.evdctgt_target_current, 250.0, 1.0, "target_current");
}

#[test]
fn test_evdclim1_handler() {
    let mut state = test_state();
    let msg = EVDCLIM1 {
        device_id: external_device(),
        hrd_or_ev_d_chrgng_vltg_mxmm: 920.0,
        hrd_or_ev_d_chrgng_crrnt_mxmm: 500.0,
        hrd_or_ev_d_chrgng_pwr_mxmm: 350.0,
        hrd_or_ev_rqstd_enrg_trnsfr_tp: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.ev_charging.evdclim1_max_voltage, 920.0, 1.0, "max_voltage");
    assert_float_near(state.ev_charging.evdclim1_max_current, 500.0, 1.0, "max_current");
    assert_float_near(state.ev_charging.evdclim1_max_power, 350.0, 1.0, "max_power");
}

#[test]
fn test_evdclim2_handler() {
    let mut state = test_state();
    let msg = EVDCLIM2 {
        device_id: external_device(),
        ev_bulk_state_of_charge: 80.0,
        ev_full_state_of_charge: 100.0,
        ev_energy_capacity: 100,
        ev_energy_requested: 75,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.ev_charging.evdclim2_bulk_soc, 80.0, 1.0, "bulk_soc");
    assert_float_near(state.ev_charging.evdclim2_full_soc, 100.0, 1.0, "full_soc");
    assert_eq!(state.ev_charging.evdclim2_energy_capacity, 100);
    assert_eq!(state.ev_charging.evdclim2_energy_requested, 75);
}

#[test]
fn test_evdccip_handler() {
    let mut state = test_state();
    let msg = EVDCCIP {
        device_id: external_device(),
        ev_bulk_charging_complete: 1,
        ev_full_charging_complete: 0,
        ev_bulk_charge_time_remaining: 1200.0,
        ev_full_charge_time_remaining: 3600.0,
        ev_departure_time: 7200.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.evdccip_bulk_charging_complete, 1);
    assert_eq!(state.ev_charging.evdccip_full_charging_complete, 0);
    assert_float_near(
        state.ev_charging.evdccip_full_charge_time_remaining,
        3600.0,
        100.0,
        "full_time_remaining",
    );
}

// ============================================================================
// Handler Tests - EVSE Messages
// ============================================================================

#[test]
fn test_evse1cs1_handler() {
    let mut state = test_state();
    let msg = EVSE1CS1 {
        device_id: external_device(),
        hrd_or_ev_d_cnttr_inpt_vltg: 400.0,
        hybrid_or_ev_dc_charging_bus_voltage: 800.0,
        hrd_or_ev_d_inlt_cnttr_1_stt: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.ev_charging.evse1cs1_contactor_input_voltage,
        400.0,
        1.0,
        "contactor_input_voltage",
    );
    assert_eq!(state.ev_charging.evse1cs1_contactor_1_state, 1);
}

#[test]
fn test_evse1cc1_handler() {
    let mut state = test_state();
    let msg = EVSE1CC1 {
        device_id: external_device(),
        hrd_or_ev_d_inlt_cnttr_1_cmmnd: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.evse1cc1_contactor_1_command, 1);
}

#[test]
fn test_evsec1_handler() {
    let mut state = test_state();
    let msg = EVSEC1 {
        device_id: external_device(),
        evse_connector_lock_request: 1,
        evse_dc_stage_request: 2,
        ev_ready: 1,
        evse_contactor_command: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.evsec1_connector_lock_request, 1);
    assert_eq!(state.ev_charging.evsec1_dc_stage_request, 2);
    assert_eq!(state.ev_charging.evsec1_ev_ready, 1);
    assert_eq!(state.ev_charging.evsec1_contactor_command, 1);
}

#[test]
fn test_evsedcs1_handler() {
    let mut state = test_state();
    let msg = EVSEDCS1 {
        device_id: external_device(),
        evse_dc_charging_state: 1,
        evse_isolation_status: 2,
        evse_present_dc_charging_voltage: 800.0,
        evse_present_dc_charging_current: 200.0,
        evse_voltage_limit_achieved: 0,
        evse_current_limit_achieved: 0,
        evse_power_limit_achieved: 0,
        evse_processing_state: 0,
        evse_status: 1,
        evse_response_code: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.evsedcs1_dc_charging_state, 1);
    assert_float_near(
        state.ev_charging.evsedcs1_present_voltage,
        800.0,
        1.0,
        "present_voltage",
    );
    assert_float_near(
        state.ev_charging.evsedcs1_present_current,
        200.0,
        1.0,
        "present_current",
    );
    assert_eq!(state.ev_charging.evsedcs1_status, 1);
}

#[test]
fn test_evses1_handler() {
    let mut state = test_state();
    let msg = EVSES1 {
        device_id: external_device(),
        evse_connector_release_latch: 0,
        evse_manual_override: 0,
        evse_connector_lock_state: 1,
        evse_connector_lock_permission: 1,
        inlet_contactor_state: 1,
        evse_inlet_state: 3,
        evse_connection_type: 2,
        evse_communications_physical_layer: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.evses1_connector_lock_state, 1);
    assert_eq!(state.ev_charging.evses1_inlet_state, 3);
    assert_eq!(state.ev_charging.evses1_connection_type, 2);
}

#[test]
fn test_evses2_handler() {
    let mut state = test_state();
    let msg = EVSES2 {
        device_id: external_device(),
        evse_temp_sensor_type: 1,
        evse_connector_temperature_status: 0,
        ev_chrgng_inlt_cnntr_tmprtr: 45.0,
        ev_c_it_ct_tpt_ss_rsst: 1200,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.ev_charging.evses2_inlet_connector_temperature,
        45.0,
        1.0,
        "connector_temp",
    );
    assert_eq!(state.ev_charging.evses2_temp_sensor_resistance, 1200);
}

// ============================================================================
// Handler Tests - EVC, EVEI, EVOI1
// ============================================================================

#[test]
fn test_evc_handler() {
    let mut state = test_state();
    let msg = EVC {
        device_id: external_device(),
        engn_vlv_cntrl_mdl_1_prlmnr_fm: 5,
        engn_vlv_cntrl_mdl_2_prlmnr_fm: 10,
        engn_vlv_cntrl_mdl_3_prlmnr_fm: 31,
        engn_vlv_cntrl_mdl_4_prlmnr_fm: 31,
        engn_vlv_cntrl_mdl_5_prlmnr_fm: 31,
        engn_vlv_cntrl_mdl_6_prlmnr_fm: 31,
        engn_vlv_cntrl_mdl_7_prlmnr_fm: 31,
        engn_vlv_cntrl_mdl_8_prlmnr_fm: 31,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.evc_valve_control_modules[0], 5);
    assert_eq!(state.ev_charging.evc_valve_control_modules[1], 10);
    assert_eq!(state.ev_charging.evc_valve_control_modules[2], 31);
}

#[test]
fn test_evei_handler() {
    let mut state = test_state();
    let msg = EVEI {
        device_id: external_device(),
        total_trip_energy_consumed: 25.5,
        trip_drive_energy_economy: 0.18,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.ev_charging.evei_total_trip_energy_consumed,
        25.5,
        1.0,
        "trip_energy",
    );
    assert_float_near(
        state.ev_charging.evei_trip_drive_energy_economy,
        0.18,
        0.1,
        "energy_economy",
    );
}

#[test]
fn test_evoi1_handler() {
    let mut state = test_state();
    let msg = EVOI1 {
        device_id: external_device(),
        hvess_estimated_remaining_distance: 250.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.ev_charging.evoi1_estimated_remaining_distance,
        250.0,
        1.0,
        "remaining_distance",
    );
}

// ============================================================================
// Handler Tests - HV Bus Messages
// ============================================================================

#[test]
fn test_hvbcs1_handler() {
    let mut state = test_state();
    let msg = HVBCS1 {
        device_id: external_device(),
        hgh_vltg_bs_intrf_1_pstv_cnttr_stt: 1,
        hgh_vltg_bs_intrf_1_ngtv_cnttr_stt: 1,
        hgh_vltg_bs_intrf_2_pstv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_2_ngtv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_3_pstv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_3_ngtv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_4_pstv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_4_ngtv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_5_pstv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_5_ngtv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_6_pstv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_6_ngtv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_7_pstv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_7_ngtv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_8_pstv_cnttr_stt: 0,
        hgh_vltg_bs_intrf_8_ngtv_cnttr_stt: 0,
        hvbcs_1_embedded_integrity_support: 0,
        hvbcs_1_counter: 5,
        hvbcs_1_crc: 35,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.hvbcs1_positive_contactor_states[0], 1);
    assert_eq!(state.ev_charging.hvbcs1_negative_contactor_states[0], 1);
    assert_eq!(state.ev_charging.hvbcs1_counter, 5);
}

#[test]
fn test_hvbcs2_handler() {
    let mut state = test_state();
    let msg = HVBCS2 {
        device_id: external_device(),
        hvbcs_2_embedded_integrity_support: 0,
        hvbcs_2_counter: 7,
        hvbcs_2_crc: 49,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.hvbcs2_counter, 7);
}

#[test]
fn test_hvbcs3_handler() {
    let mut state = test_state();
    let msg = HVBCS3 {
        device_id: external_device(),
        hvbcs_3_embedded_integrity_support: 0,
        hvbcs_3_counter: 3,
        hvbcs_3_crc: 21,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.hvbcs3_counter, 3);
}

#[test]
fn test_hvbcc1_handler() {
    let mut state = test_state();
    let msg = HVBCC1 {
        device_id: external_device(),
        hgh_vltg_bs_intrf_1_cnnt_cmmnd: 1,
        hgh_vltg_bs_intrf_2_cnnt_cmmnd: 1,
        hgh_vltg_bs_intrf_3_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_4_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_5_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_6_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_7_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_8_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_9_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_10_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_11_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_12_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_13_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_14_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_15_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_16_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_17_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_18_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_19_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_20_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_21_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_22_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_23_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_24_cnnt_cmmnd: 0,
        hvbcc_1_embedded_integrity_support: 0,
        hvbcc_1_counter: 2,
        hvbcc_1_crc: 14,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.hvbcc1_connect_commands[0], 1);
    assert_eq!(state.ev_charging.hvbcc1_connect_commands[1], 1);
    assert_eq!(state.ev_charging.hvbcc1_connect_commands[2], 0);
}

#[test]
fn test_hvbcc2_handler() {
    let mut state = test_state();
    let msg = HVBCC2 {
        device_id: external_device(),
        hgh_vltg_bs_intrf_25_cnnt_cmmnd: 1,
        hgh_vltg_bs_intrf_26_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_27_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_28_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_29_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_30_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_31_cnnt_cmmnd: 0,
        hgh_vltg_bs_intrf_32_cnnt_cmmnd: 0,
        hvbcc_2_embedded_integrity_support: 0,
        hvbcc_2_counter: 4,
        hvbcc_2_crc: 28,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.hvbcc2_connect_commands[0], 1);
    assert_eq!(state.ev_charging.hvbcc2_connect_commands[1], 0);
}

#[test]
fn test_hvbi_handler() {
    let mut state = test_state();
    let msg = HVBI {
        device_id: external_device(),
        high_voltage_dc_bus_availability: 1,
        hgh_vltg_bs_drvln_avllt: 1,
        hgh_vltg_bs_axlrs_avllt: 1,
        high_voltage_bus_epto_availability: 0,
        hgh_vltg_bs_on_brd_chrgr_avllt: 1,
        hgh_vltg_bs_off_brd_chrgr_avllt: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.ev_charging.hvbi_dc_bus_availability, 1);
    assert_eq!(state.ev_charging.hvbi_driveline_availability, 1);
    assert_eq!(state.ev_charging.hvbi_off_board_charger_availability, 1);
}

#[test]
fn test_evvt_handler() {
    let mut state = test_state();
    let msg = EVVT {
        device_id: external_device(),
        e_vvt_it_vv_cst_cdd_t_ost: -10.0,
        e_vvt_it_vv_cst_t_ost_pst: -9.5,
        e_vvt_exst_vv_cst_cdd_t_ost: 5.0,
        e_vvt_exst_vv_cst_t_ost_pst: 4.8,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.ev_charging.evvt_intake_commanded_offset,
        -10.0,
        1.0,
        "intake_cmd",
    );
    assert_float_near(
        state.ev_charging.evvt_exhaust_commanded_offset,
        5.0,
        1.0,
        "exhaust_cmd",
    );
}

// ============================================================================
// Broadcast Verification Tests
// ============================================================================

#[test]
fn test_ev_charging_broadcasts_present() {
    let state = test_state();
    let frames = state.generate_can_frames();

    // Check that all 21 EV Charging & HV Bus messages are broadcast
    let has_evdcs1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVDCS1::BASE_CAN_ID));
    let has_evdctgt = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVDCTGT::BASE_CAN_ID));
    let has_evdclim1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVDCLIM1::BASE_CAN_ID));
    let has_evdclim2 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVDCLIM2::BASE_CAN_ID));
    let has_evdccip = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVDCCIP::BASE_CAN_ID));
    let has_evse1cs1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVSE1CS1::BASE_CAN_ID));
    let has_evse1cc1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVSE1CC1::BASE_CAN_ID));
    let has_evsec1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVSEC1::BASE_CAN_ID));
    let has_evsedcs1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVSEDCS1::BASE_CAN_ID));
    let has_evses1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVSES1::BASE_CAN_ID));
    let has_evses2 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVSES2::BASE_CAN_ID));
    let has_evc = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVC::BASE_CAN_ID));
    let has_evei = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVEI::BASE_CAN_ID));
    let has_evoi1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVOI1::BASE_CAN_ID));
    let has_hvbcs1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), HVBCS1::BASE_CAN_ID));
    let has_hvbcs2 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), HVBCS2::BASE_CAN_ID));
    let has_hvbcs3 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), HVBCS3::BASE_CAN_ID));
    let has_hvbcc1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), HVBCC1::BASE_CAN_ID));
    let has_hvbcc2 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), HVBCC2::BASE_CAN_ID));
    let has_hvbi = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), HVBI::BASE_CAN_ID));
    let has_evvt = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), EVVT::BASE_CAN_ID));

    assert!(has_evdcs1, "EVDCS1 broadcast missing");
    assert!(has_evdctgt, "EVDCTGT broadcast missing");
    assert!(has_evdclim1, "EVDCLIM1 broadcast missing");
    assert!(has_evdclim2, "EVDCLIM2 broadcast missing");
    assert!(has_evdccip, "EVDCCIP broadcast missing");
    assert!(has_evse1cs1, "EVSE1CS1 broadcast missing");
    assert!(has_evse1cc1, "EVSE1CC1 broadcast missing");
    assert!(has_evsec1, "EVSEC1 broadcast missing");
    assert!(has_evsedcs1, "EVSEDCS1 broadcast missing");
    assert!(has_evses1, "EVSES1 broadcast missing");
    assert!(has_evses2, "EVSES2 broadcast missing");
    assert!(has_evc, "EVC broadcast missing");
    assert!(has_evei, "EVEI broadcast missing");
    assert!(has_evoi1, "EVOI1 broadcast missing");
    assert!(has_hvbcs1, "HVBCS1 broadcast missing");
    assert!(has_hvbcs2, "HVBCS2 broadcast missing");
    assert!(has_hvbcs3, "HVBCS3 broadcast missing");
    assert!(has_hvbcc1, "HVBCC1 broadcast missing");
    assert!(has_hvbcc2, "HVBCC2 broadcast missing");
    assert!(has_hvbi, "HVBI broadcast missing");
    assert!(has_evvt, "EVVT broadcast missing");
}

// ============================================================================
// Round-trip Tests (encode -> broadcast -> decode)
// ============================================================================

#[test]
fn test_evdctgt_round_trip() {
    let mut state = test_state();
    state.ev_charging.evdctgt_target_voltage = 850.0;
    state.ev_charging.evdctgt_target_current = 300.0;

    let frames = state.generate_can_frames();
    let evdctgt_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), EVDCTGT::BASE_CAN_ID))
        .expect("EVDCTGT frame not found");

    let decoded = EVDCTGT::decode(evdctgt_frame.raw_id(), evdctgt_frame.data()).unwrap();
    assert_float_near(
        decoded.dc_target_charging_voltage,
        850.0,
        1.0,
        "round_trip_voltage",
    );
    assert_float_near(
        decoded.dc_target_charging_current,
        300.0,
        1.0,
        "round_trip_current",
    );
}

#[test]
fn test_hvbi_round_trip() {
    let mut state = test_state();
    state.ev_charging.hvbi_dc_bus_availability = 1;
    state.ev_charging.hvbi_driveline_availability = 1;
    state.ev_charging.hvbi_auxiliaries_availability = 1;
    state.ev_charging.hvbi_off_board_charger_availability = 1;

    let frames = state.generate_can_frames();
    let hvbi_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), HVBI::BASE_CAN_ID))
        .expect("HVBI frame not found");

    let decoded = HVBI::decode(hvbi_frame.raw_id(), hvbi_frame.data()).unwrap();
    assert_eq!(decoded.high_voltage_dc_bus_availability, 1);
    assert_eq!(decoded.hgh_vltg_bs_drvln_avllt, 1);
    assert_eq!(decoded.hgh_vltg_bs_off_brd_chrgr_avllt, 1);
}

// ============================================================================
// Physics / State Machine Tests
// ============================================================================

#[test]
fn test_charging_state_machine_idle() {
    let mut state = test_state();
    assert_eq!(state.ev_charging.ev_charging_state, 0); // Starts idle

    // Physics update in idle state
    state.update_physics(0.1);
    assert_eq!(state.ev_charging.evsedcs1_dc_charging_state, 0); // Idle
    assert_eq!(state.ev_charging.hvbcs1_positive_contactor_states[0], 0); // Open
}

#[test]
fn test_charging_state_machine_idle_to_communication() {
    let mut state = test_state();
    assert_eq!(state.ev_charging.ev_charging_state, 0);

    // Set ev_ready to trigger transition
    state.ev_charging.evsec1_ev_ready = 1;
    state.update_physics(0.1);

    // Should transition to communication (1) then pre-charge (2) in one step
    assert!(
        state.ev_charging.ev_charging_state >= 1,
        "Should have advanced from idle"
    );
}

#[test]
fn test_charging_state_machine_full_cycle() {
    let mut state = test_state();

    // Set up for charging
    state.ev_charging.evsec1_ev_ready = 1;
    state.ev_charging.evdctgt_target_voltage = 800.0;
    state.ev_charging.evdctgt_target_current = 200.0;
    state.ev_charging.evdclim1_max_voltage = 920.0;
    state.ev_charging.evdclim1_max_current = 500.0;
    state.ev_charging.evdclim1_max_power = 350.0;
    state.ev_charging.evdclim2_bulk_soc = 80.0;
    state.ev_charging.evdclim2_full_soc = 100.0;
    state.ev_charging.evdclim2_energy_capacity = 100;
    state.hvess.hvess_fast_update_state_of_charge = 50.0; // 50% SOC

    // Run through idle -> communication -> pre-charge
    for _ in 0..5 {
        state.update_physics(0.1);
    }
    // Should be in pre-charge (2) or charging (3)
    assert!(
        state.ev_charging.ev_charging_state >= 2,
        "Should be in pre-charge or charging, got {}",
        state.ev_charging.ev_charging_state
    );

    // Run many cycles to ramp voltage up for pre-charge
    for _ in 0..100 {
        state.update_physics(0.1);
    }

    // Should have transitioned to charging (3) by now
    assert_eq!(
        state.ev_charging.ev_charging_state, 3,
        "Should be in charging state after voltage ramp"
    );

    // Contactors should be closed
    assert_eq!(state.ev_charging.hvbcs1_positive_contactor_states[0], 1);
    assert_eq!(state.ev_charging.hvbcs1_negative_contactor_states[0], 1);
    assert_eq!(state.ev_charging.evse1cs1_contactor_1_state, 1);

    // EVSEDCS1 should show charging
    assert_eq!(state.ev_charging.evsedcs1_dc_charging_state, 1);

    // Current should be ramping
    assert!(
        state.ev_charging.evsedcs1_present_current > 0.0,
        "Current should be flowing"
    );
}

#[test]
fn test_charging_soc_taper() {
    let mut state = test_state();

    // Set up at high SOC to test current tapering
    state.ev_charging.ev_charging_state = 3; // Already charging
    state.ev_charging.evdctgt_target_voltage = 800.0;
    state.ev_charging.evdctgt_target_current = 500.0;
    state.ev_charging.evdclim1_max_voltage = 920.0;
    state.ev_charging.evdclim1_max_current = 500.0;
    state.ev_charging.evdclim1_max_power = 350.0;
    state.ev_charging.evdclim2_bulk_soc = 80.0;
    state.ev_charging.evdclim2_full_soc = 100.0;
    state.hvess.hvess_fast_update_state_of_charge = 90.0; // High SOC

    state.update_physics(0.1);

    // Max current should be tapered at 90% SOC
    // taper_factor = 1.0 - (90-80)/20 = 0.5
    assert!(
        state.ev_charging.evdclim1_max_current < 300.0,
        "Current should be tapered at high SOC, got {:.1}",
        state.ev_charging.evdclim1_max_current
    );
}

#[test]
fn test_charging_complete_transition() {
    let mut state = test_state();

    // Set up in charging state near full
    state.ev_charging.ev_charging_state = 3;
    state.ev_charging.evdctgt_target_voltage = 800.0;
    state.ev_charging.evdctgt_target_current = 50.0;
    state.ev_charging.evdclim1_max_voltage = 920.0;
    state.ev_charging.evdclim1_max_current = 500.0;
    state.ev_charging.evdclim1_max_power = 350.0;
    state.ev_charging.evdclim2_full_soc = 100.0;
    state.hvess.hvess_fast_update_state_of_charge = 100.0; // Full

    state.update_physics(0.1);

    assert_eq!(state.ev_charging.ev_charging_state, 4, "Should transition to complete");
    assert_eq!(state.ev_charging.evdccip_full_charging_complete, 1);
}

#[test]
fn test_charging_ev_not_ready_stops() {
    let mut state = test_state();

    // Set up in charging state
    state.ev_charging.ev_charging_state = 3;
    state.ev_charging.evdctgt_target_voltage = 800.0;
    state.ev_charging.evdctgt_target_current = 200.0;
    state.ev_charging.evdclim1_max_voltage = 920.0;
    state.ev_charging.evdclim1_max_current = 500.0;
    state.ev_charging.evdclim1_max_power = 350.0;
    state.ev_charging.evsec1_ev_ready = 0; // Not ready
    state.hvess.hvess_fast_update_state_of_charge = 50.0;

    state.update_physics(0.1);

    assert_eq!(
        state.ev_charging.ev_charging_state, 4,
        "Should transition to complete when ev_ready goes low"
    );
}

#[test]
fn test_contactors_open_after_complete() {
    let mut state = test_state();

    // Set up in complete state with current already at zero
    state.ev_charging.ev_charging_state = 4;
    state.ev_charging.evsedcs1_present_current = 0.0;
    state.ev_charging.evsedcs1_present_voltage = 50.0; // Still has some voltage

    // Run several cycles to drain voltage
    for _ in 0..100 {
        state.update_physics(0.1);
    }

    // Contactors should be open
    assert_eq!(state.ev_charging.hvbcs1_positive_contactor_states[0], 0);
    assert_eq!(state.ev_charging.hvbcs1_negative_contactor_states[0], 0);
    assert_eq!(state.ev_charging.evse1cs1_contactor_1_state, 0);

    // Should return to idle
    assert_eq!(state.ev_charging.ev_charging_state, 0, "Should return to idle");
}

#[test]
fn test_hvbi_availability_during_charging() {
    let mut state = test_state();
    state.ev_charging.ev_charging_state = 3; // Charging
    state.ev_charging.evdctgt_target_voltage = 800.0;
    state.ev_charging.evdctgt_target_current = 200.0;
    state.ev_charging.evdclim1_max_voltage = 920.0;
    state.ev_charging.evdclim1_max_current = 500.0;
    state.ev_charging.evdclim1_max_power = 350.0;
    state.hvess.hvess_fast_update_state_of_charge = 50.0;

    state.update_physics(0.1);

    assert_eq!(
        state.ev_charging.hvbi_dc_bus_availability, 1,
        "DC bus should be available during charging"
    );
    assert_eq!(
        state.ev_charging.hvbi_off_board_charger_availability, 1,
        "Off-board charger should be available during charging"
    );
}

#[test]
fn test_energy_tracking_during_charging() {
    let mut state = test_state();

    // Simulate charging with known power
    state.ev_charging.evsedcs1_present_voltage = 800.0;
    state.ev_charging.evsedcs1_present_current = 100.0; // 80kW
    state.ev_charging.evei_total_trip_energy_consumed = 0.0;

    // Run EV charging physics for 1 second (10 x 0.1s)
    // We need to be in charging state for this to work
    state.ev_charging.ev_charging_state = 3;
    state.ev_charging.evdctgt_target_voltage = 800.0;
    state.ev_charging.evdctgt_target_current = 100.0;
    state.ev_charging.evdclim1_max_voltage = 920.0;
    state.ev_charging.evdclim1_max_current = 500.0;
    state.ev_charging.evdclim1_max_power = 350.0;
    state.hvess.hvess_fast_update_state_of_charge = 50.0;

    for _ in 0..10 {
        state.update_physics(0.1);
    }

    // Energy should have been tracked (approximate due to current ramping)
    assert!(
        state.ev_charging.evei_total_trip_energy_consumed > 0.0,
        "Energy consumed should be tracked during charging"
    );
}

#[test]
fn test_remaining_distance_tracks_soc() {
    let mut state = test_state();
    state.hvess.hvess_fast_update_state_of_charge = 80.0;

    // Trigger the EV charging physics (which updates range estimate)
    state.update_physics(0.1);

    // 80% SOC * 500km = 400km
    assert_float_near(
        state.ev_charging.evoi1_estimated_remaining_distance,
        400.0,
        10.0,
        "remaining_distance_at_80pct",
    );
}

// ============================================================================
// Self-reception filtering
// ============================================================================

#[test]
fn test_ev_charging_self_reception_filtered() {
    let mut state = test_state(); // device_id = 0x82

    // Send from the same device (0x82) - should be ignored
    let msg = EVDCTGT {
        device_id: DeviceId::from(0x82),
        dc_charging_target_crc: 42,
        dc_charging_target_sequence_counter: 5,
        dc_target_charging_voltage: 900.0,
        dc_target_charging_current: 400.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Ignored);
    // State should not have changed (stays at default 400V)
    assert_float_near(
        state.ev_charging.evdctgt_target_voltage,
        400.0,
        1.0,
        "voltage_unchanged",
    );
}

// ============================================================================
// DecodeFailed Tests
// ============================================================================

#[test]
fn test_batch12_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = EVDCS1::BASE_CAN_ID | 0x42;
    let data: [u8; 0] = []; // Empty payload triggers decode failure
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
