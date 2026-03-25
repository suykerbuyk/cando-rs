//! Batch 7: HVESS (High Voltage Energy Storage System) Extended Message Tests
//!
//! Tests for 36 HVESS messages:
//! HVESSD4-D15, IS1-IS7, MS1-MS3, S1-S2, FS2, FC, CFG, CP1C, CP1S1-S2,
//! CP2C, CP2S1-S2, TCH1-TCH3, HIST

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
// HVESSD4 - High Voltage Energy Storage System Data 4
// ============================================================================

#[test]
fn test_hvessd4_handler() {
    let mut state = test_state();
    let msg = HVESSD4 {
        device_id: external_device(),
        hvess_discharge_capacity: 180.0,
        hvess_charge_capacity: 120.0,
        hvess_cell_balancing_count: 42,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessd4_discharge_capacity, 180.0, 1.0, "discharge_capacity");
    assert_float_near(state.hvess.hvessd4_charge_capacity, 120.0, 1.0, "charge_capacity");
    assert_eq!(state.hvess.hvessd4_cell_balancing_count, 42);
}

#[test]
fn test_hvessd4_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD4::BASE_CAN_ID));
    assert!(found, "HVESSD4 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD5 - High Voltage Energy Storage System Data 5
// ============================================================================

#[test]
fn test_hvessd5_handler() {
    let mut state = test_state();
    let msg = HVESSD5 {
        device_id: external_device(),
        hvss_mxmm_instntns_dshrg_crrnt_lmt: -180.0,
        hvss_mxmm_instntns_chrg_crrnt_lmt: 130.0,
        hvess_minimum_cell_state_of_charge: 68.0,
        hvess_maximum_cell_state_of_charge: 82.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessd5_max_discharge_current_limit, -180.0, 2.0, "max_discharge_current_limit");
    assert_float_near(state.hvess.hvessd5_max_charge_current_limit, 130.0, 2.0, "max_charge_current_limit");
    assert_float_near(state.hvess.hvessd5_min_cell_soc, 68.0, 1.0, "min_cell_soc");
    assert_float_near(state.hvess.hvessd5_max_cell_soc, 82.0, 1.0, "max_cell_soc");
}

#[test]
fn test_hvessd5_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD5::BASE_CAN_ID));
    assert!(found, "HVESSD5 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD7 - High Voltage Energy Storage System Data 7
// ============================================================================

#[test]
fn test_hvessd7_handler() {
    let mut state = test_state();
    let msg = HVESSD7 {
        device_id: external_device(),
        hvess_discharge_energy_capacity: 35.0,
        hvess_charge_energy_capacity: 18.0,
        hvess_maximum_charge_voltage_limit: 840.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessd7_discharge_energy_capacity, 35.0, 1.0, "discharge_energy_capacity");
    assert_float_near(state.hvess.hvessd7_charge_energy_capacity, 18.0, 1.0, "charge_energy_capacity");
    assert_float_near(state.hvess.hvessd7_max_charge_voltage_limit, 840.0, 2.0, "max_charge_voltage_limit");
}

#[test]
fn test_hvessd7_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD7::BASE_CAN_ID));
    assert!(found, "HVESSD7 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD8 - High Voltage Energy Storage System Data 8
// ============================================================================

#[test]
fn test_hvessd8_handler() {
    let mut state = test_state();
    let msg = HVESSD8 {
        device_id: external_device(),
        hvss_hghst_cll_vltg_mdl_nmr: 5,
        hvss_hghst_cll_vltg_cll_nmr: 14,
        hvss_lwst_cll_vltg_mdl_nmr: 9,
        hvss_lwst_cll_vltg_cll_nmr: 7,
        hvess_average_cell_voltage: 3.9,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessd8_highest_cell_voltage_module, 5);
    assert_eq!(state.hvess.hvessd8_highest_cell_voltage_cell, 14);
    assert_eq!(state.hvess.hvessd8_lowest_cell_voltage_module, 9);
    assert_eq!(state.hvess.hvessd8_lowest_cell_voltage_cell, 7);
    assert_float_near(state.hvess.hvessd8_average_cell_voltage, 3.9, 0.1, "average_cell_voltage");
}

#[test]
fn test_hvessd8_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD8::BASE_CAN_ID));
    assert!(found, "HVESSD8 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD9 - High Voltage Energy Storage System Data 9
// ============================================================================

#[test]
fn test_hvessd9_handler() {
    let mut state = test_state();
    let msg = HVESSD9 {
        device_id: external_device(),
        hvss_hghst_cll_tmprtr_mdl_nmr: 4,
        hvss_hghst_cll_tmprtr_cll_nmr: 10,
        hvss_lwst_cll_tmprtr_mdl_nmr: 12,
        hvss_lwst_cll_tmprtr_cll_nmr: 5,
        hvess_thermal_event_detected: 1,
        hvss_dt_9_emddd_intgrt_spprt: 0,
        hvess_data_9_counter: 7,
        hvess_data_9_crc: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessd9_highest_cell_temp_module, 4);
    assert_eq!(state.hvess.hvessd9_highest_cell_temp_cell, 10);
    assert_eq!(state.hvess.hvessd9_lowest_cell_temp_module, 12);
    assert_eq!(state.hvess.hvessd9_lowest_cell_temp_cell, 5);
    assert_eq!(state.hvess.hvessd9_thermal_event_detected, 1);
}

#[test]
fn test_hvessd9_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD9::BASE_CAN_ID));
    assert!(found, "HVESSD9 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD10 - High Voltage Energy Storage System Data 10
// ============================================================================

#[test]
fn test_hvessd10_handler() {
    let mut state = test_state();
    let msg = HVESSD10 {
        device_id: external_device(),
        hvss_hghst_cll_stt_of_chrg_mdl_nmr: 3,
        hvss_hghst_cll_stt_of_chrg_cll_nmr: 8,
        hvss_lwst_cll_stt_of_chrg_mdl_nmr: 11,
        hvss_lwst_cll_stt_of_chrg_cll_nmr: 13,
        hvss_hgh_vltg_bs_atv_isltn_tst_rslts: 480.0,
        hvss_hgh_vltg_bs_pssv_isltn_tst_rslts: 420.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessd10_highest_cell_soc_module, 3);
    assert_eq!(state.hvess.hvessd10_highest_cell_soc_cell, 8);
    assert_eq!(state.hvess.hvessd10_lowest_cell_soc_module, 11);
    assert_eq!(state.hvess.hvessd10_lowest_cell_soc_cell, 13);
    assert_float_near(state.hvess.hvessd10_active_isolation_test, 480.0, 5.0, "active_isolation_test");
    assert_float_near(state.hvess.hvessd10_passive_isolation_test, 420.0, 5.0, "passive_isolation_test");
}

#[test]
fn test_hvessd10_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD10::BASE_CAN_ID));
    assert!(found, "HVESSD10 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD11 - High Voltage Energy Storage System Data 11
// ============================================================================

#[test]
fn test_hvessd11_handler() {
    let mut state = test_state();
    let msg = HVESSD11 {
        device_id: external_device(),
        hvss_bs_vltg_ngtv_t_chsss_grnd_vltg: 380.0,
        hvss_vltg_lvl_ngtv_t_chsss_grnd_vltg: 385.0,
        hvess_actual_charge_rate: 25.0,
        hvss_ttl_strd_enrg_sr_lvl: 55.0,
        hvss_pwr_mdl_eltrns_tmprtr: 42.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessd11_bus_voltage_neg_to_chassis, 380.0, 5.0, "bus_voltage_neg_to_chassis");
    assert_float_near(state.hvess.hvessd11_voltage_neg_to_chassis, 385.0, 5.0, "voltage_neg_to_chassis");
    assert_float_near(state.hvess.hvessd11_actual_charge_rate, 25.0, 1.0, "actual_charge_rate");
    assert_float_near(state.hvess.hvessd11_total_stored_energy, 55.0, 1.0, "total_stored_energy");
    assert_float_near(state.hvess.hvessd11_power_module_electronics_temp, 42.0, 1.0, "power_module_electronics_temp");
}

#[test]
fn test_hvessd11_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD11::BASE_CAN_ID));
    assert!(found, "HVESSD11 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD12 - High Voltage Energy Storage System Data 12
// ============================================================================

#[test]
fn test_hvessd12_handler() {
    let mut state = test_state();
    let msg = HVESSD12 {
        device_id: external_device(),
        hvess_intake_coolant_pressure: 140.0,
        hvss_estmtd_dshrg_tm_rmnng: 100.0,
        hvss_estmtd_chrg_tm_rmnng: 45.0,
        hvss_hgh_vltg_expsr_indtr: 1,
        hvess_power_hold_relay_status: 1,
        hvss_hgh_vltg_bs_pstv_pr_chrg_rl_stt: 2,
        hvss_hgh_vltg_bs_ngtv_pr_chrg_rl_stt: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessd12_intake_coolant_pressure, 140.0, 5.0, "intake_coolant_pressure");
    assert_float_near(state.hvess.hvessd12_estimated_discharge_time, 100.0, 5.0, "estimated_discharge_time");
    assert_float_near(state.hvess.hvessd12_estimated_charge_time, 45.0, 5.0, "estimated_charge_time");
    assert_eq!(state.hvess.hvessd12_hv_exposure_indicator, 1);
    assert_eq!(state.hvess.hvessd12_power_hold_relay_status, 1);
    assert_eq!(state.hvess.hvessd12_positive_precharge_relay, 2);
    assert_eq!(state.hvess.hvessd12_negative_precharge_relay, 1);
}

#[test]
fn test_hvessd12_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD12::BASE_CAN_ID));
    assert!(found, "HVESSD12 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD13 - High Voltage Energy Storage System Data 13
// ============================================================================

#[test]
fn test_hvessd13_handler() {
    let mut state = test_state();
    let msg = HVESSD13 {
        device_id: external_device(),
        hvss_avll_dshrg_pwr_extndd_rng: 45.0,
        hvss_avll_chrg_pwr_extndd_rng: 40.0,
        hvess_voltage_level_extended_range: 790.0,
        hvess_current_extended_range: 10.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessd13_discharge_power_extended, 45.0, 2.0, "discharge_power_extended");
    assert_float_near(state.hvess.hvessd13_charge_power_extended, 40.0, 2.0, "charge_power_extended");
    assert_float_near(state.hvess.hvessd13_voltage_extended, 790.0, 5.0, "voltage_extended");
    assert_float_near(state.hvess.hvessd13_current_extended, 10.0, 2.0, "current_extended");
}

#[test]
fn test_hvessd13_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD13::BASE_CAN_ID));
    assert!(found, "HVESSD13 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD14 - High Voltage Energy Storage System Data 14
// ============================================================================

#[test]
fn test_hvessd14_handler() {
    let mut state = test_state();
    let msg = HVESSD14 {
        device_id: external_device(),
        hvss_mx_istts_ds_ct_lt_extdd_r: -190.0,
        hvss_mx_istts_c_ct_lt_extdd_r: 140.0,
        hvess_bus_voltage_extended_range: 795.0,
        hvss_mnmm_dshrg_vltg_lmt: 640.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessd14_max_discharge_current_extended, -190.0, 5.0, "max_discharge_current_extended");
    assert_float_near(state.hvess.hvessd14_max_charge_current_extended, 140.0, 5.0, "max_charge_current_extended");
    assert_float_near(state.hvess.hvessd14_bus_voltage_extended, 795.0, 5.0, "bus_voltage_extended");
    assert_float_near(state.hvess.hvessd14_min_discharge_voltage_limit, 640.0, 5.0, "min_discharge_voltage_limit");
}

#[test]
fn test_hvessd14_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD14::BASE_CAN_ID));
    assert!(found, "HVESSD14 frame should be present in broadcasts");
}

// ============================================================================
// HVESSD15 - High Voltage Energy Storage System Data 15
// ============================================================================

#[test]
fn test_hvessd15_handler() {
    let mut state = test_state();
    let msg = HVESSD15 {
        device_id: external_device(),
        hvss_nmnl_dshrg_crrnt_lmt: -140.0,
        hvess_nominal_charge_current_limit: 90.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessd15_nominal_discharge_current_limit, -140.0, 5.0, "nominal_discharge_current_limit");
    assert_float_near(state.hvess.hvessd15_nominal_charge_current_limit, 90.0, 2.0, "nominal_charge_current_limit");
}

#[test]
fn test_hvessd15_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD15::BASE_CAN_ID));
    assert!(found, "HVESSD15 frame should be present in broadcasts");
}

// ============================================================================
// HVESSIS1 - HVESS Internal Segment 1
// ============================================================================

#[test]
fn test_hvessis1_handler() {
    let mut state = test_state();
    let msg = HVESSIS1 {
        device_id: external_device(),
        hvss_hgh_vltg_intrnl_vltg_lvl_1: 390.0,
        hvss_hgh_vltg_intrnl_crrnt_1: 5.0,
        hvss_hgh_vltg_intrnl_vltg_lvl_2: 395.0,
        hvss_hgh_vltg_intrnl_crrnt_2: 4.5,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessis1_internal_voltage_1, 390.0, 5.0, "internal_voltage_1");
    assert_float_near(state.hvess.hvessis1_internal_current_1, 5.0, 1.0, "internal_current_1");
    assert_float_near(state.hvess.hvessis1_internal_voltage_2, 395.0, 5.0, "internal_voltage_2");
    assert_float_near(state.hvess.hvessis1_internal_current_2, 4.5, 1.0, "internal_current_2");
}

#[test]
fn test_hvessis1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSIS1::BASE_CAN_ID));
    assert!(found, "HVESSIS1 frame should be present in broadcasts");
}

// ============================================================================
// HVESSIS2 - HVESS Internal Segment 2
// ============================================================================

#[test]
fn test_hvessis2_handler() {
    let mut state = test_state();
    let msg = HVESSIS2 {
        device_id: external_device(),
        hvss_hgh_vltg_intrnl_vltg_lvl_3: 388.0,
        hvss_hgh_vltg_intrnl_crrnt_3: 3.0,
        hvss_hgh_vltg_intrnl_vltg_lvl_4: 392.0,
        hvss_hgh_vltg_intrnl_crrnt_4: 3.5,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessis2_internal_voltage_3, 388.0, 5.0, "internal_voltage_3");
    assert_float_near(state.hvess.hvessis2_internal_current_3, 3.0, 1.0, "internal_current_3");
    assert_float_near(state.hvess.hvessis2_internal_voltage_4, 392.0, 5.0, "internal_voltage_4");
    assert_float_near(state.hvess.hvessis2_internal_current_4, 3.5, 1.0, "internal_current_4");
}

#[test]
fn test_hvessis2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSIS2::BASE_CAN_ID));
    assert!(found, "HVESSIS2 frame should be present in broadcasts");
}

// ============================================================================
// HVESSIS3 - HVESS Internal Segment 3
// ============================================================================

#[test]
fn test_hvessis3_handler() {
    let mut state = test_state();
    let msg = HVESSIS3 {
        device_id: external_device(),
        hvss_hgh_vltg_intrnl_vltg_lvl_5: 385.0,
        hvss_hgh_vltg_intrnl_crrnt_5: 2.0,
        hvss_hgh_vltg_intrnl_vltg_lvl_6: 387.0,
        hvss_hgh_vltg_intrnl_crrnt_6: 2.5,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessis3_internal_voltage_5, 385.0, 5.0, "internal_voltage_5");
    assert_float_near(state.hvess.hvessis3_internal_current_5, 2.0, 1.0, "internal_current_5");
    assert_float_near(state.hvess.hvessis3_internal_voltage_6, 387.0, 5.0, "internal_voltage_6");
    assert_float_near(state.hvess.hvessis3_internal_current_6, 2.5, 1.0, "internal_current_6");
}

#[test]
fn test_hvessis3_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSIS3::BASE_CAN_ID));
    assert!(found, "HVESSIS3 frame should be present in broadcasts");
}

// ============================================================================
// HVESSIS4 - HVESS Internal Segment 4
// ============================================================================

#[test]
fn test_hvessis4_handler() {
    let mut state = test_state();
    let msg = HVESSIS4 {
        device_id: external_device(),
        hvss_hgh_vltg_intrnl_vltg_lvl_7: 382.0,
        hvss_hgh_vltg_intrnl_crrnt_7: 1.5,
        hvss_hgh_vltg_intrnl_vltg_lvl_8: 384.0,
        hvss_hgh_vltg_intrnl_crrnt_8: 1.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessis4_internal_voltage_7, 382.0, 5.0, "internal_voltage_7");
    assert_float_near(state.hvess.hvessis4_internal_current_7, 1.5, 1.0, "internal_current_7");
    assert_float_near(state.hvess.hvessis4_internal_voltage_8, 384.0, 5.0, "internal_voltage_8");
    assert_float_near(state.hvess.hvessis4_internal_current_8, 1.0, 1.0, "internal_current_8");
}

#[test]
fn test_hvessis4_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSIS4::BASE_CAN_ID));
    assert!(found, "HVESSIS4 frame should be present in broadcasts");
}

// ============================================================================
// HVESSIS5 - HVESS Internal Segment 5
// ============================================================================

#[test]
fn test_hvessis5_handler() {
    let mut state = test_state();
    let msg = HVESSIS5 {
        device_id: external_device(),
        hvss_hgh_vltg_intrnl_pstv_cnttr_1_stt: 1,
        hvss_hgh_vltg_intrnl_ngtv_cnttr_1_stt: 1,
        hvss_hgh_vltg_intrnl_prhrg_rl_1_stt: 2,
        hvss_t_mt_sst_it_ht_1_stts: 1,
        hvss_hgh_vltg_intrnl_bs_vltg_lvl_1: 395.0,
        hvss_hgh_vltg_intrnl_pstv_cnttr_2_stt: 1,
        hvss_hgh_vltg_intrnl_ngtv_cnttr_2_stt: 0,
        hvss_hgh_vltg_intrnl_prhrg_rl_2_stt: 1,
        hvss_t_mt_sst_it_ht_2_stts: 0,
        hvss_hgh_vltg_intrnl_bs_vltg_lvl_2: 390.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessis5_positive_contactor_1_state, 1);
    assert_eq!(state.hvess.hvessis5_negative_contactor_1_state, 1);
    assert_eq!(state.hvess.hvessis5_precharge_relay_1_state, 2);
    assert_eq!(state.hvess.hvessis5_inline_heater_1_status, 1);
    assert_float_near(state.hvess.hvessis5_bus_voltage_1, 395.0, 5.0, "bus_voltage_1");
    assert_eq!(state.hvess.hvessis5_positive_contactor_2_state, 1);
    assert_eq!(state.hvess.hvessis5_negative_contactor_2_state, 0);
    assert_eq!(state.hvess.hvessis5_precharge_relay_2_state, 1);
    assert_eq!(state.hvess.hvessis5_inline_heater_2_status, 0);
    assert_float_near(state.hvess.hvessis5_bus_voltage_2, 390.0, 5.0, "bus_voltage_2");
}

#[test]
fn test_hvessis5_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSIS5::BASE_CAN_ID));
    assert!(found, "HVESSIS5 frame should be present in broadcasts");
}

// ============================================================================
// HVESSIS6 - HVESS Internal Segment 6
// ============================================================================

#[test]
fn test_hvessis6_handler() {
    let mut state = test_state();
    let msg = HVESSIS6 {
        device_id: external_device(),
        hvss_hgh_vltg_intrnl_pstv_cnttr_3_stt: 1,
        hvss_hgh_vltg_intrnl_ngtv_cnttr_3_stt: 1,
        hvss_hgh_vltg_intrnl_prhrg_rl_3_stt: 0,
        hvss_t_mt_sst_it_ht_3_stts: 1,
        hvss_hgh_vltg_intrnl_bs_vltg_lvl_3: 388.0,
        hvss_hgh_vltg_intrnl_pstv_cnttr_4_stt: 0,
        hvss_hgh_vltg_intrnl_ngtv_cnttr_4_stt: 1,
        hvss_hgh_vltg_intrnl_prhrg_rl_4_stt: 2,
        hvss_t_mt_sst_it_ht_4_stts: 0,
        hvss_hgh_vltg_intrnl_bs_vltg_lvl_4: 392.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessis6_positive_contactor_3_state, 1);
    assert_eq!(state.hvess.hvessis6_negative_contactor_3_state, 1);
    assert_eq!(state.hvess.hvessis6_precharge_relay_3_state, 0);
    assert_eq!(state.hvess.hvessis6_inline_heater_3_status, 1);
    assert_float_near(state.hvess.hvessis6_bus_voltage_3, 388.0, 5.0, "bus_voltage_3");
    assert_eq!(state.hvess.hvessis6_positive_contactor_4_state, 0);
    assert_eq!(state.hvess.hvessis6_negative_contactor_4_state, 1);
    assert_eq!(state.hvess.hvessis6_precharge_relay_4_state, 2);
    assert_eq!(state.hvess.hvessis6_inline_heater_4_status, 0);
    assert_float_near(state.hvess.hvessis6_bus_voltage_4, 392.0, 5.0, "bus_voltage_4");
}

#[test]
fn test_hvessis6_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSIS6::BASE_CAN_ID));
    assert!(found, "HVESSIS6 frame should be present in broadcasts");
}

// ============================================================================
// HVESSIS7 - HVESS Internal Segment 7
// ============================================================================

#[test]
fn test_hvessis7_handler() {
    let mut state = test_state();
    let msg = HVESSIS7 {
        device_id: external_device(),
        hvss_nmr_of_intrnl_crts_rd: 6,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessis7_number_of_internal_circuits, 6);
}

#[test]
fn test_hvessis7_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSIS7::BASE_CAN_ID));
    assert!(found, "HVESSIS7 frame should be present in broadcasts");
}

// ============================================================================
// HVESSMS1 - HVESS Module Status 1
// ============================================================================

#[test]
fn test_hvessms1_handler() {
    let mut state = test_state();
    let msg = HVESSMS1 {
        device_id: external_device(),
        hvess_module_1_operational_status: 2,
        hvess_module_2_operational_status: 1,
        hvess_module_3_operational_status: 3,
        hvess_module_4_operational_status: 0,
        hvess_module_5_operational_status: 1,
        hvess_module_6_operational_status: 1,
        hvess_module_7_operational_status: 1,
        hvess_module_8_operational_status: 1,
        hvess_module_9_operational_status: 1,
        hvess_module_10_operational_status: 1,
        hvess_module_11_operational_status: 1,
        hvess_module_12_operational_status: 1,
        hvess_module_13_operational_status: 1,
        hvess_module_14_operational_status: 1,
        hvess_module_15_operational_status: 1,
        hvess_module_16_operational_status: 1,
        hvess_module_17_operational_status: 1,
        hvess_module_18_operational_status: 1,
        hvess_module_19_operational_status: 1,
        hvess_module_20_operational_status: 1,
        hvess_module_21_operational_status: 1,
        hvess_module_22_operational_status: 1,
        hvess_module_23_operational_status: 1,
        hvess_module_24_operational_status: 1,
        hvess_module_25_operational_status: 1,
        hvess_module_26_operational_status: 1,
        hvess_module_27_operational_status: 1,
        hvess_module_28_operational_status: 1,
        hvess_module_29_operational_status: 1,
        hvess_module_30_operational_status: 1,
        hvess_module_31_operational_status: 1,
        hvess_module_32_operational_status: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessms1_module_status[0], 2);
    assert_eq!(state.hvess.hvessms1_module_status[1], 1);
    assert_eq!(state.hvess.hvessms1_module_status[2], 3);
    assert_eq!(state.hvess.hvessms1_module_status[3], 0);
}

#[test]
fn test_hvessms1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSMS1::BASE_CAN_ID));
    assert!(found, "HVESSMS1 frame should be present in broadcasts");
}

// ============================================================================
// HVESSMS2 - HVESS Module Status 2
// ============================================================================

#[test]
fn test_hvessms2_handler() {
    let mut state = test_state();
    let msg = HVESSMS2 {
        device_id: external_device(),
        hvess_module_33_operational_status: 2,
        hvess_module_34_operational_status: 3,
        hvess_module_35_operational_status: 1,
        hvess_module_36_operational_status: 0,
        hvess_module_37_operational_status: 1,
        hvess_module_38_operational_status: 1,
        hvess_module_39_operational_status: 1,
        hvess_module_40_operational_status: 1,
        hvess_module_41_operational_status: 1,
        hvess_module_42_operational_status: 1,
        hvess_module_43_operational_status: 1,
        hvess_module_44_operational_status: 1,
        hvess_module_45_operational_status: 1,
        hvess_module_46_operational_status: 1,
        hvess_module_47_operational_status: 1,
        hvess_module_48_operational_status: 1,
        hvess_module_49_operational_status: 1,
        hvess_module_50_operational_status: 1,
        hvess_module_51_operational_status: 1,
        hvess_module_52_operational_status: 1,
        hvess_module_53_operational_status: 1,
        hvess_module_54_operational_status: 1,
        hvess_module_55_operational_status: 1,
        hvess_module_56_operational_status: 1,
        hvess_module_57_operational_status: 1,
        hvess_module_58_operational_status: 1,
        hvess_module_59_operational_status: 1,
        hvess_module_60_operational_status: 1,
        hvess_module_61_operational_status: 1,
        hvess_module_62_operational_status: 1,
        hvess_module_63_operational_status: 1,
        hvess_module_64_operational_status: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessms2_module_status[0], 2);
    assert_eq!(state.hvess.hvessms2_module_status[1], 3);
    assert_eq!(state.hvess.hvessms2_module_status[2], 1);
    assert_eq!(state.hvess.hvessms2_module_status[3], 0);
}

#[test]
fn test_hvessms2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSMS2::BASE_CAN_ID));
    assert!(found, "HVESSMS2 frame should be present in broadcasts");
}

// ============================================================================
// HVESSMS3 - HVESS Module Status 3
// ============================================================================

#[test]
fn test_hvessms3_handler() {
    let mut state = test_state();
    let msg = HVESSMS3 {
        device_id: external_device(),
        hvess_module_65_operational_status: 3,
        hvess_module_66_operational_status: 2,
        hvess_module_67_operational_status: 0,
        hvess_module_68_operational_status: 1,
        hvess_module_69_operational_status: 1,
        hvess_module_70_operational_status: 1,
        hvess_module_71_operational_status: 1,
        hvess_module_72_operational_status: 1,
        hvess_module_73_operational_status: 1,
        hvess_module_74_operational_status: 1,
        hvess_module_75_operational_status: 1,
        hvess_module_76_operational_status: 1,
        hvess_module_77_operational_status: 1,
        hvess_module_78_operational_status: 1,
        hvess_module_79_operational_status: 1,
        hvess_module_80_operational_status: 1,
        hvess_module_81_operational_status: 1,
        hvess_module_82_operational_status: 1,
        hvess_module_83_operational_status: 1,
        hvess_module_84_operational_status: 1,
        hvess_module_85_operational_status: 1,
        hvess_module_86_operational_status: 1,
        hvess_module_87_operational_status: 1,
        hvess_module_88_operational_status: 1,
        hvess_module_89_operational_status: 1,
        hvess_module_90_operational_status: 1,
        hvess_module_91_operational_status: 1,
        hvess_module_92_operational_status: 1,
        hvess_module_93_operational_status: 1,
        hvess_module_94_operational_status: 1,
        hvess_module_95_operational_status: 1,
        hvess_module_96_operational_status: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessms3_module_status[0], 3);
    assert_eq!(state.hvess.hvessms3_module_status[1], 2);
    assert_eq!(state.hvess.hvessms3_module_status[2], 0);
    assert_eq!(state.hvess.hvessms3_module_status[3], 1);
}

#[test]
fn test_hvessms3_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSMS3::BASE_CAN_ID));
    assert!(found, "HVESSMS3 frame should be present in broadcasts");
}

// ============================================================================
// HVESSS1 - HVESS System Status 1
// ============================================================================

#[test]
fn test_hvesss1_handler() {
    let mut state = test_state();
    let msg = HVESSS1 {
        device_id: external_device(),
        hvss_hgh_vltg_bs_pstv_cnttr_stt: 1,
        hvss_hgh_vltg_bs_ngtv_cnttr_stt: 1,
        hvss_hgh_vltg_bs_dsnnt_frwrnng: 0,
        hvss_hgh_vltg_bs_pr_chrg_rl_stt: 0,
        hvess_center_of_pack_contactor_state: 0,
        hvss_hgh_vltg_bs_atv_isltn_tst_stts: 1,
        hvss_hgh_vltg_bs_pssv_isltn_tst_stts: 1,
        hvess_hvil_status: 2,
        hvess_inertia_switch_status: 0,
        hvess_state_of_charge_status: 1,
        hvess_cell_balance_status: 0,
        hvess_cell_balancing_active: 0,
        hvess_internal_charger_status: 0,
        hvess_status_1_counter: 5,
        hvss_hgh_vltg_bs_cnntn_stts: 1,
        hvess_operational_status: 2,
        hvess_number_of_hvesps_ready: 1,
        hvess_status_1_crc: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvesss1_positive_contactor_state, 1);
    assert_eq!(state.hvess.hvesss1_negative_contactor_state, 1);
    assert_eq!(state.hvess.hvesss1_hvil_status, 2);
    assert_eq!(state.hvess.hvesss1_soc_status, 1);
    assert_eq!(state.hvess.hvesss1_operational_status, 2);
}

#[test]
fn test_hvesss1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSS1::BASE_CAN_ID));
    assert!(found, "HVESSS1 frame should be present in broadcasts");
}

// ============================================================================
// HVESSS2 - HVESS System Status 2
// ============================================================================

#[test]
fn test_hvesss2_handler() {
    let mut state = test_state();
    let msg = HVESSS2 {
        device_id: external_device(),
        hvss_dshrg_pwr_lmt_dt_stt_of_chrg: 1,
        hvss_dshrg_pwr_lmt_dt_bttr_tmprtr: 2,
        hvss_dshrg_pwr_lmt_dt_bttr_dgnst_cndtn: 0,
        hvss_dshrg_pwr_lmt_dt_bttr_or_cll_vltg: 0,
        hvss_dshrg_pwr_lmt_dt_bttr_crrnt: 0,
        hvss_dshrg_pwr_lmt_dt_an_undfnd_cs: 0,
        hvss_ds_pw_lt_dt_pw_md_ets_tpt: 0,
        hvss_chrg_pwr_lmt_dt_stt_of_chrg: 1,
        hvss_chrg_pwr_lmt_dt_bttr_tmprtr: 3,
        hvss_chrg_pwr_lmt_dt_bttr_dgnst_cndtn: 0,
        hvss_chrg_pwr_lmt_dt_bttr_or_cll_vltg: 0,
        hvss_chrg_pwr_lmt_dt_bttr_crrnt: 0,
        hvss_chrg_pwr_lmt_dt_an_undfnd_cs: 0,
        hvss_c_pw_lt_dt_pw_md_ets_tpt: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvesss2_discharge_limit_soc, 1);
    assert_eq!(state.hvess.hvesss2_discharge_limit_temp, 2);
    assert_eq!(state.hvess.hvesss2_charge_limit_soc, 1);
    assert_eq!(state.hvess.hvesss2_charge_limit_temp, 3);
}

#[test]
fn test_hvesss2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSS2::BASE_CAN_ID));
    assert!(found, "HVESSS2 frame should be present in broadcasts");
}

// ============================================================================
// HVESSFS2 - HVESS Fan Status 2
// ============================================================================

#[test]
fn test_hvessfs2_handler() {
    let mut state = test_state();
    let msg = HVESSFS2 {
        device_id: external_device(),
        hvess_fan_voltage: 11.5,
        hvess_fan_current: 4.0,
        hvess_fan_hvil_status: 1,
        hvess_fan_status_2_instance: 1,
        hvess_fan_percent_speed_status: 1,
        hvess_fan_percent_speed: 55.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvessfs2_fan_voltage, 11.5, 1.0, "fan_voltage");
    assert_float_near(state.hvess.hvessfs2_fan_current, 4.0, 1.0, "fan_current");
    assert_eq!(state.hvess.hvessfs2_fan_hvil_status, 1);
    assert_float_near(state.hvess.hvessfs2_fan_percent_speed, 55.0, 1.0, "fan_percent_speed");
}

#[test]
fn test_hvessfs2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSFS2::BASE_CAN_ID));
    assert!(found, "HVESSFS2 frame should be present in broadcasts");
}

// ============================================================================
// HVESSFC - HVESS Fan Command
// ============================================================================

#[test]
fn test_hvessfc_handler() {
    let mut state = test_state();
    let msg = HVESSFC {
        device_id: external_device(),
        hvess_fan_enable_command: 1,
        hvess_fan_power_hold: 1,
        hvess_fan_speed_command: 3000.0,
        hvess_fan_percent_speed_command: 75.0,
        hvess_fan_instance_1: 1,
        hvess_fan_instance_2: 0,
        hvess_fan_instance_3: 0,
        hvess_fan_instance_4: 0,
        hvess_fan_instance_5: 0,
        hvess_fan_instance_6: 0,
        hvess_fan_instance_7: 0,
        hvess_fan_instance_8: 0,
        hvess_fan_instance_9: 0,
        hvess_fan_instance_10: 0,
        hvess_fan_instance_11: 0,
        hvess_fan_instance_12: 0,
        hvess_fan_instance_13: 0,
        hvess_fan_instance_14: 0,
        hvess_fan_instance_15: 0,
        h_vt_e_st_sst_f_cds_eddd_itt_sppt: 0,
        hgh_vltg_enrg_strg_sstm_fn_cmmnds_cntr: 0,
        hgh_vltg_enrg_strg_sstm_fn_cmmnds_cr: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvessfc_fan_enable_command, 1);
    assert_eq!(state.hvess.hvessfc_fan_power_hold, 1);
    assert_float_near(state.hvess.hvessfc_fan_speed_command, 3000.0, 10.0, "fan_speed_command");
    assert_float_near(state.hvess.hvessfc_fan_percent_speed_command, 75.0, 1.0, "fan_percent_speed_command");
}

#[test]
fn test_hvessfc_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSFC::BASE_CAN_ID));
    assert!(found, "HVESSFC frame should be present in broadcasts");
}

// ============================================================================
// HVESSCFG - HVESS Configuration (DLC=20, handler only)
// ============================================================================

#[test]
fn test_hvesscfg_handler() {
    let mut state = test_state();
    let msg = HVESSCFG {
        device_id: external_device(),
        hvess_nominal_voltage: 750.0,
        hvss_rmmndd_mnmm_oprtng_vltg: 620.0,
        hvss_rmmndd_mxmm_oprtng_vltg: 830.0,
        hvss_rmmndd_mnmm_stt_of_chrg: 15.0,
        hvss_rmmndd_mxmm_stt_of_chrg: 90.0,
        hvss_rmmndd_mxmm_oprtng_tmprtr: 50.0,
        hvss_rmmndd_mnmm_oprtng_tmprtr: -15.0,
        hvess_cell_maximum_voltage_limit: 4.1,
        hvess_cell_minimum_voltage_limit: 2.8,
        hvess_number_of_hvesps_configured: 2,
        hvess_nominal_rated_capacity: 280.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvesscfg_nominal_voltage, 750.0, 5.0, "nominal_voltage");
    assert_float_near(state.hvess.hvesscfg_min_operating_voltage, 620.0, 5.0, "min_operating_voltage");
    assert_float_near(state.hvess.hvesscfg_max_operating_voltage, 830.0, 5.0, "max_operating_voltage");
    assert_float_near(state.hvess.hvesscfg_nominal_capacity, 280.0, 5.0, "nominal_capacity");
    assert_eq!(state.hvess.hvesscfg_num_packs, 2);
}

#[test]
fn test_hvesscfg_broadcast_skipped_due_to_dlc() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSCFG::BASE_CAN_ID));
    assert!(!found, "HVESSCFG frame should NOT be in broadcasts (DLC=20 exceeds 8-byte CAN limit)");
}

// ============================================================================
// HVESSCP1C - HVESS Coolant Pump 1 Command
// ============================================================================

#[test]
fn test_hvesscp1c_handler() {
    let mut state = test_state();
    let msg = HVESSCP1C {
        device_id: external_device(),
        hvess_coolant_pump_1_enable_command: 1,
        hvess_coolant_pump_1_power_hold: 1,
        hvess_coolant_pump_1_speed_command: 2500.0,
        hvss_clnt_pmp_1_prnt_spd_cmmnd: 70.0,
        hvss_ct_pp_1_cd_eddd_itt_sppt: 0,
        hvess_coolant_pump_1_command_counter: 0,
        hvess_coolant_pump_1_command_crc: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvesscp1c_enable_command, 1);
    assert_eq!(state.hvess.hvesscp1c_power_hold, 1);
    assert_float_near(state.hvess.hvesscp1c_speed_command, 2500.0, 10.0, "pump1_speed_command");
    assert_float_near(state.hvess.hvesscp1c_percent_speed_command, 70.0, 1.0, "pump1_percent_speed_command");
}

#[test]
fn test_hvesscp1c_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSCP1C::BASE_CAN_ID));
    assert!(found, "HVESSCP1C frame should be present in broadcasts");
}

// ============================================================================
// HVESSCP1S1 - HVESS Coolant Pump 1 Status 1
// ============================================================================

#[test]
fn test_hvesscp1s1_handler() {
    let mut state = test_state();
    let msg = HVESSCP1S1 {
        device_id: external_device(),
        hvss_clnt_pmp_1_mtr_spd_stts: 1,
        hvss_clnt_pmp_1_cntrllr_stts_rsn_cd: 2,
        hvss_clnt_pmp_1_cntrllr_cmmnd_stts: 1,
        hvess_coolant_pump_1_motor_speed: 1900.0,
        hvss_clnt_pmp_1_cntrl_tmprtr: 38.0,
        hvess_coolant_pump_1_power: 110.0,
        hvss_clnt_pmp_1_srv_indtr: 0,
        hvss_clnt_pmp_1_oprtng_stts: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvesscp1s1_motor_speed, 1900.0, 10.0, "pump1_motor_speed");
    assert_float_near(state.hvess.hvesscp1s1_power, 110.0, 5.0, "pump1_power");
    assert_eq!(state.hvess.hvesscp1s1_motor_speed_status, 1);
    assert_eq!(state.hvess.hvesscp1s1_operating_status, 1);
}

#[test]
fn test_hvesscp1s1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSCP1S1::BASE_CAN_ID));
    assert!(found, "HVESSCP1S1 frame should be present in broadcasts");
}

// ============================================================================
// HVESSCP1S2 - HVESS Coolant Pump 1 Status 2
// ============================================================================

#[test]
fn test_hvesscp1s2_handler() {
    let mut state = test_state();
    let msg = HVESSCP1S2 {
        device_id: external_device(),
        hvess_coolant_pump_1_voltage: 11.8,
        hvess_coolant_pump_1_current: 9.0,
        hvess_coolant_pump_1_hvil_status: 1,
        hvss_clnt_pmp_1_prnt_spd_stts: 1,
        hvess_coolant_pump_1_percent_speed: 62.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvesscp1s2_voltage, 11.8, 1.0, "pump1_voltage");
    assert_float_near(state.hvess.hvesscp1s2_current, 9.0, 1.0, "pump1_current");
    assert_eq!(state.hvess.hvesscp1s2_hvil_status, 1);
    assert_float_near(state.hvess.hvesscp1s2_percent_speed, 62.0, 1.0, "pump1_percent_speed");
}

#[test]
fn test_hvesscp1s2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSCP1S2::BASE_CAN_ID));
    assert!(found, "HVESSCP1S2 frame should be present in broadcasts");
}

// ============================================================================
// HVESSCP2C - HVESS Coolant Pump 2 Command
// ============================================================================

#[test]
fn test_hvesscp2c_handler() {
    let mut state = test_state();
    let msg = HVESSCP2C {
        device_id: external_device(),
        hvess_coolant_pump_2_enable_command: 1,
        hvess_coolant_pump_2_power_hold: 0,
        hvess_coolant_pump_2_speed_command: 2200.0,
        hvss_clnt_pmp_2_prnt_spd_cmmnd: 60.0,
        hvss_ct_pp_2_cd_eddd_itt_sppt: 0,
        hvess_coolant_pump_2_command_counter: 0,
        hvess_coolant_pump_2_command_crc: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvesscp2c_enable_command, 1);
    assert_eq!(state.hvess.hvesscp2c_power_hold, 0);
    assert_float_near(state.hvess.hvesscp2c_speed_command, 2200.0, 10.0, "pump2_speed_command");
    assert_float_near(state.hvess.hvesscp2c_percent_speed_command, 60.0, 1.0, "pump2_percent_speed_command");
}

#[test]
fn test_hvesscp2c_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSCP2C::BASE_CAN_ID));
    assert!(found, "HVESSCP2C frame should be present in broadcasts");
}

// ============================================================================
// HVESSCP2S1 - HVESS Coolant Pump 2 Status 1
// ============================================================================

#[test]
fn test_hvesscp2s1_handler() {
    let mut state = test_state();
    let msg = HVESSCP2S1 {
        device_id: external_device(),
        hvss_clnt_pmp_2_mtr_spd_stts: 1,
        hvss_clnt_pmp_2_cntrllr_stts_rsn_cd: 0,
        hvss_clnt_pmp_2_cntrllr_cmmnd_stts: 1,
        hvess_coolant_pump_2_motor_speed: 1700.0,
        hvss_clnt_pmp_2_cntrl_tmprtr: 36.0,
        hvess_coolant_pump_2_power: 95.0,
        hvss_clnt_pmp_2_srv_indtr: 0,
        hvss_clnt_pmp_2_oprtng_stts: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvesscp2s1_motor_speed, 1700.0, 10.0, "pump2_motor_speed");
    assert_float_near(state.hvess.hvesscp2s1_power, 95.0, 5.0, "pump2_power");
    assert_eq!(state.hvess.hvesscp2s1_motor_speed_status, 1);
    assert_eq!(state.hvess.hvesscp2s1_operating_status, 1);
}

#[test]
fn test_hvesscp2s1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSCP2S1::BASE_CAN_ID));
    assert!(found, "HVESSCP2S1 frame should be present in broadcasts");
}

// ============================================================================
// HVESSCP2S2 - HVESS Coolant Pump 2 Status 2
// ============================================================================

#[test]
fn test_hvesscp2s2_handler() {
    let mut state = test_state();
    let msg = HVESSCP2S2 {
        device_id: external_device(),
        hvess_coolant_pump_2_voltage: 11.5,
        hvess_coolant_pump_2_current: 7.5,
        hvess_coolant_pump_2_hvil_status: 0,
        hvss_clnt_pmp_2_prnt_spd_stts: 1,
        hvess_coolant_pump_2_percent_speed: 52.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvesscp2s2_voltage, 11.5, 1.0, "pump2_voltage");
    assert_float_near(state.hvess.hvesscp2s2_current, 7.5, 1.0, "pump2_current");
    assert_eq!(state.hvess.hvesscp2s2_hvil_status, 0);
    assert_float_near(state.hvess.hvesscp2s2_percent_speed, 52.0, 1.0, "pump2_percent_speed");
}

#[test]
fn test_hvesscp2s2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSCP2S2::BASE_CAN_ID));
    assert!(found, "HVESSCP2S2 frame should be present in broadcasts");
}

// ============================================================================
// HVESSTCH1 - HVESS Thermal Channel 1
// ============================================================================

#[test]
fn test_hvesstch1_handler() {
    let mut state = test_state();
    let msg = HVESSTCH1 {
        device_id: external_device(),
        hvss_t_mt_sst_c_1_cpss_ds_ast_pss: 1100,
        hvss_t_mt_sst_c_1_cpss_st_ast_pss: 380,
        hvss_t_mt_sst_c_1_ott_ct_tpt: 28.0,
        hvss_t_mt_sst_c_1_c_vv_pst: 55.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvesstch1_compressor_discharge_abs_pressure, 1100);
    assert_eq!(state.hvess.hvesstch1_compressor_suction_abs_pressure, 380);
    assert_float_near(state.hvess.hvesstch1_outlet_coolant_temp, 28.0, 1.0, "tch1_outlet_coolant_temp");
    assert_float_near(state.hvess.hvesstch1_condenser_valve_position, 55.0, 1.0, "tch1_condenser_valve_position");
}

#[test]
fn test_hvesstch1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSTCH1::BASE_CAN_ID));
    assert!(found, "HVESSTCH1 frame should be present in broadcasts");
}

// ============================================================================
// HVESSTCH2 - HVESS Thermal Channel 2
// ============================================================================

#[test]
fn test_hvesstch2_handler() {
    let mut state = test_state();
    let msg = HVESSTCH2 {
        device_id: external_device(),
        hvss_t_mt_sst_c_2_cpss_ds_ast_pss: 1050,
        hvss_t_mt_sst_c_2_cpss_st_ast_pss: 360,
        hvss_t_mt_sst_c_2_ott_ct_tpt: 29.0,
        hvss_t_mt_sst_c_2_c_vv_pst: 52.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvesstch2_compressor_discharge_abs_pressure, 1050);
    assert_eq!(state.hvess.hvesstch2_compressor_suction_abs_pressure, 360);
    assert_float_near(state.hvess.hvesstch2_outlet_coolant_temp, 29.0, 1.0, "tch2_outlet_coolant_temp");
    assert_float_near(state.hvess.hvesstch2_condenser_valve_position, 52.0, 1.0, "tch2_condenser_valve_position");
}

#[test]
fn test_hvesstch2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSTCH2::BASE_CAN_ID));
    assert!(found, "HVESSTCH2 frame should be present in broadcasts");
}

// ============================================================================
// HVESSTCH3 - HVESS Thermal Channel 3
// ============================================================================

#[test]
fn test_hvesstch3_handler() {
    let mut state = test_state();
    let msg = HVESSTCH3 {
        device_id: external_device(),
        hvss_t_mt_sst_c_3_cpss_ds_ast_pss: 980,
        hvss_t_mt_sst_c_3_cpss_st_ast_pss: 340,
        hvss_t_mt_sst_c_3_ott_ct_tpt: 30.0,
        hvss_t_mt_sst_c_3_c_vv_pst: 48.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.hvess.hvesstch3_compressor_discharge_abs_pressure, 980);
    assert_eq!(state.hvess.hvesstch3_compressor_suction_abs_pressure, 340);
    assert_float_near(state.hvess.hvesstch3_outlet_coolant_temp, 30.0, 1.0, "tch3_outlet_coolant_temp");
    assert_float_near(state.hvess.hvesstch3_condenser_valve_position, 48.0, 1.0, "tch3_condenser_valve_position");
}

#[test]
fn test_hvesstch3_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSTCH3::BASE_CAN_ID));
    assert!(found, "HVESSTCH3 frame should be present in broadcasts");
}

// ============================================================================
// HVESSHIST - HVESS History/Lifetime Data (DLC=20, handler only)
// ============================================================================

#[test]
fn test_hvesshist_handler() {
    let mut state = test_state();
    let msg = HVESSHIST {
        device_id: external_device(),
        hvess_state_of_health: 92.0,
        hvss_cnttr_opn_undr_ld_cnt: 5,
        hvess_total_energy_throughput: 4500.0,
        hvess_total_accumulated_charge: 5500.0,
        hvess_total_lifetime_energy_input: 230000,
        hvess_total_lifetime_energy_output: 220000,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.hvess.hvesshist_state_of_health, 92.0, 1.0, "state_of_health");
    assert_eq!(state.hvess.hvesshist_contactor_open_under_load, 5);
    assert_float_near(state.hvess.hvesshist_total_energy_throughput, 4500.0, 10.0, "total_energy_throughput");
    assert_float_near(state.hvess.hvesshist_total_accumulated_charge, 5500.0, 10.0, "total_accumulated_charge");
    assert_eq!(state.hvess.hvesshist_lifetime_energy_input, 230000);
    assert_eq!(state.hvess.hvesshist_lifetime_energy_output, 220000);
}

#[test]
fn test_hvesshist_broadcast_skipped_due_to_dlc() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSHIST::BASE_CAN_ID));
    assert!(!found, "HVESSHIST frame should NOT be in broadcasts (DLC=20 exceeds 8-byte CAN limit)");
}

// ============================================================================
// Round-trip Tests (Encode -> Broadcast -> Decode)
// ============================================================================

#[test]
fn test_hvessd4_roundtrip() {
    let mut state = test_state();
    state.hvess.hvessd4_discharge_capacity = 175.0;
    state.hvess.hvessd4_charge_capacity = 125.0;
    state.hvess.hvessd4_cell_balancing_count = 10;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSD4::BASE_CAN_ID))
        .expect("HVESSD4 frame should exist");

    let decoded = HVESSD4::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_float_near(decoded.hvess_discharge_capacity, 175.0, 1.0, "roundtrip discharge_capacity");
    assert_float_near(decoded.hvess_charge_capacity, 125.0, 1.0, "roundtrip charge_capacity");
    assert_eq!(decoded.hvess_cell_balancing_count, 10);
}

#[test]
fn test_hvessis1_roundtrip() {
    let mut state = test_state();
    state.hvess.hvessis1_internal_voltage_1 = 380.0;
    state.hvess.hvessis1_internal_current_1 = 8.0;
    state.hvess.hvessis1_internal_voltage_2 = 385.0;
    state.hvess.hvessis1_internal_current_2 = 7.5;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSIS1::BASE_CAN_ID))
        .expect("HVESSIS1 frame should exist");

    let decoded = HVESSIS1::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_float_near(decoded.hvss_hgh_vltg_intrnl_vltg_lvl_1, 380.0, 5.0, "roundtrip internal_voltage_1");
    assert_float_near(decoded.hvss_hgh_vltg_intrnl_crrnt_1, 8.0, 1.0, "roundtrip internal_current_1");
    assert_float_near(decoded.hvss_hgh_vltg_intrnl_vltg_lvl_2, 385.0, 5.0, "roundtrip internal_voltage_2");
    assert_float_near(decoded.hvss_hgh_vltg_intrnl_crrnt_2, 7.5, 1.0, "roundtrip internal_current_2");
}

#[test]
fn test_hvessfc_roundtrip() {
    let mut state = test_state();
    state.hvess.hvessfc_fan_enable_command = 1;
    state.hvess.hvessfc_fan_power_hold = 0;
    state.hvess.hvessfc_fan_speed_command = 2800.0;
    state.hvess.hvessfc_fan_percent_speed_command = 70.0;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSFC::BASE_CAN_ID))
        .expect("HVESSFC frame should exist");

    let decoded = HVESSFC::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_eq!(decoded.hvess_fan_enable_command, 1);
    assert_eq!(decoded.hvess_fan_power_hold, 0);
    assert_float_near(decoded.hvess_fan_speed_command, 2800.0, 10.0, "roundtrip fan_speed_command");
    assert_float_near(decoded.hvess_fan_percent_speed_command, 70.0, 1.0, "roundtrip fan_percent_speed_command");
}

#[test]
fn test_hvesscp1s1_roundtrip() {
    let mut state = test_state();
    state.hvess.hvesscp1s1_motor_speed = 1850.0;
    state.hvess.hvesscp1s1_power = 115.0;
    state.hvess.hvesscp1s1_motor_speed_status = 1;
    state.hvess.hvesscp1s1_operating_status = 1;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSCP1S1::BASE_CAN_ID))
        .expect("HVESSCP1S1 frame should exist");

    let decoded = HVESSCP1S1::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_float_near(decoded.hvess_coolant_pump_1_motor_speed, 1850.0, 10.0, "roundtrip pump1_motor_speed");
    assert_float_near(decoded.hvess_coolant_pump_1_power, 115.0, 5.0, "roundtrip pump1_power");
    assert_eq!(decoded.hvss_clnt_pmp_1_mtr_spd_stts, 1);
    assert_eq!(decoded.hvss_clnt_pmp_1_oprtng_stts, 1);
}

#[test]
fn test_hvesstch1_roundtrip() {
    let mut state = test_state();
    state.hvess.hvesstch1_compressor_discharge_abs_pressure = 1150;
    state.hvess.hvesstch1_compressor_suction_abs_pressure = 420;
    state.hvess.hvesstch1_outlet_coolant_temp = 26.0;
    state.hvess.hvesstch1_condenser_valve_position = 52.0;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HVESSTCH1::BASE_CAN_ID))
        .expect("HVESSTCH1 frame should exist");

    let decoded = HVESSTCH1::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_eq!(decoded.hvss_t_mt_sst_c_1_cpss_ds_ast_pss, 1150);
    assert_eq!(decoded.hvss_t_mt_sst_c_1_cpss_st_ast_pss, 420);
    assert_float_near(decoded.hvss_t_mt_sst_c_1_ott_ct_tpt, 26.0, 1.0, "roundtrip tch1_outlet_coolant_temp");
    assert_float_near(decoded.hvss_t_mt_sst_c_1_c_vv_pst, 52.0, 1.0, "roundtrip tch1_condenser_valve_position");
}

// ============================================================================
// Physics Tests
// ============================================================================

#[test]
fn test_physics_pump1_speed_lag() {
    // Pump 1 physics: speed approaches command with 10%/dt lag
    let mut state = test_state();
    state.hvess.hvesscp1c_enable_command = 1;
    state.hvess.hvesscp1c_speed_command = 3000.0;
    state.hvess.hvesscp1s1_motor_speed = 0.0;

    // One physics step with dt=1.0
    state.update_physics(1.0);

    // speed_diff = 3000 - 0 = 3000, step = 3000 * 0.1 * 1.0 = 300
    // new speed should be ~300
    assert_float_near(
        state.hvess.hvesscp1s1_motor_speed,
        300.0,
        50.0,
        "pump1 speed after one step should approach target with 10% lag",
    );
    // Should not yet be at target
    assert!(
        state.hvess.hvesscp1s1_motor_speed < 3000.0,
        "pump1 speed should still lag behind target"
    );
}

#[test]
fn test_physics_pump2_speed_lag() {
    // Pump 2 physics: speed approaches command with 10%/dt lag
    let mut state = test_state();
    state.hvess.hvesscp2c_enable_command = 1;
    state.hvess.hvesscp2c_speed_command = 2500.0;
    state.hvess.hvesscp2s1_motor_speed = 0.0;

    state.update_physics(1.0);

    // speed_diff = 2500 - 0 = 2500, step = 2500 * 0.1 * 1.0 = 250
    assert_float_near(
        state.hvess.hvesscp2s1_motor_speed,
        250.0,
        50.0,
        "pump2 speed after one step should approach target with 10% lag",
    );
    assert!(
        state.hvess.hvesscp2s1_motor_speed < 2500.0,
        "pump2 speed should still lag behind target"
    );
}

#[test]
fn test_physics_fan_response_lag() {
    // Fan physics: speed responds to fan command with 15%/dt lag
    let mut state = test_state();
    state.hvess.hvessfc_fan_enable_command = 1;
    state.hvess.hvessfc_fan_speed_command = 4000.0;
    state.hvess.hvess_fan_speed = 1000.0;

    state.update_physics(1.0);

    // fan_diff = 4000 - 1000 = 3000, step = 3000 * 0.15 * 1.0 = 450
    // new fan speed should be ~1450
    assert_float_near(
        state.hvess.hvess_fan_speed,
        1450.0,
        150.0,
        "fan speed after one step should approach target with 15% lag",
    );
    assert!(
        state.hvess.hvess_fan_speed < 4000.0,
        "fan speed should still lag behind target"
    );
}

#[test]
fn test_physics_d4_capacity_from_soc() {
    // HVESSD4 capacity = nominal_capacity * SOC / 100
    let mut state = test_state();
    state.hvess.hvesscfg_nominal_capacity = 300.0;
    state.hvess.hvess_fast_update_state_of_charge = 60.0;

    state.update_physics(0.1);

    // discharge_capacity = 300 * 60 / 100 = 180
    assert_float_near(
        state.hvess.hvessd4_discharge_capacity,
        180.0,
        10.0,
        "discharge capacity should track SOC * nominal_capacity / 100",
    );
    // charge_capacity = 300 * (100 - 60) / 100 = 120
    assert_float_near(
        state.hvess.hvessd4_charge_capacity,
        120.0,
        10.0,
        "charge capacity should track remaining SOC * nominal_capacity / 100",
    );
}

#[test]
fn test_physics_segment_voltage_from_bus() {
    // Internal segment voltage = bus_voltage / 4
    let mut state = test_state();
    state.hvess.hvess_voltage_level = 800.0;

    state.update_physics(0.1);

    // segment_voltage = 800 / 4 = 200
    assert_float_near(
        state.hvess.hvessis1_internal_voltage_1,
        200.0,
        5.0,
        "segment voltage should be bus_voltage / 4",
    );
    assert_float_near(
        state.hvess.hvessis2_internal_voltage_3,
        200.0,
        5.0,
        "segment 2 voltage should also be bus_voltage / 4",
    );
    assert_float_near(
        state.hvess.hvessis5_bus_voltage_1,
        200.0,
        5.0,
        "IS5 bus voltage 1 should track segment voltage",
    );
}

// ============================================================================
// Self-reception Test
// ============================================================================

#[test]
fn test_hvessd4_self_reception_ignored() {
    let mut state = test_state();
    let msg = HVESSD4 {
        device_id: DeviceId::from(0x82),
        hvess_discharge_capacity: 999.0,
        hvess_charge_capacity: 999.0,
        hvess_cell_balancing_count: 999,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Ignored);
    // State should NOT have been updated (should remain at default 200.0)
    assert_float_near(state.hvess.hvessd4_discharge_capacity, 200.0, 1.0, "d4 should remain at default after self-reception");
}

// ============================================================================
// DecodeFailed Test
// ============================================================================

#[test]
fn test_batch7_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = HVESSD4::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated -- should trigger DecodeFailed
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
