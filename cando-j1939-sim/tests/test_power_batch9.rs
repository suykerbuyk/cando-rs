//! Batch 9: Extended Power Conversion Messages Tests
//!
//! Tests for 40+ extended DCDC converter and power supply messages:
//! - DCDC1 extended: HL, LL, T, V, VC, LD, SBL, CFG1
//! - DCDC2 extended: C, OS, HL, LL, T, V, VC, LD, SBL, CFG1
//! - DCDC3 core + extended: C, OS, S2, SBS, T, V, VC, SBL, LL, HL, LD, CFG1
//! - DCDC4 core + extended: C, OS, S2, SBS, T, V, VC, SBL, LL, HL, LD, CFG1
//! - Power supply: GC1, GTRACE2, GAAC

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
    ((frame_id & 0x1FFFFFFF) & 0xFFFFFF00) == base_can_id
}

fn frame_can_id(frame: &socketcan::CanFrame) -> u32 {
    frame.raw_id() & 0x1FFFFFFF
}

// ============================================================================
// DCDC1 Extended Messages
// ============================================================================

#[test]
fn test_dcdc1hl_handler() {
    let mut state = test_state();
    let msg = DCDC1HL {
        device_id: external_device(),
        dd_1_hgh_sd_vltg_mnmm_lmt_rqst: 700.0,
        dd_1_hgh_sd_vltg_mxmm_lmt_rqst: 850.0,
        dd_1_hgh_sd_crrnt_mxmm_lmt_rqst: 180.0,
        dd_1_hgh_sd_crrnt_mnmm_lmt_rqst: -40.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1hl_high_side_voltage_min_limit, 700.0, 1.0, "dcdc1hl_hs_vmin");
    assert_float_near(state.dcdc.dcdc1hl_high_side_voltage_max_limit, 850.0, 1.0, "dcdc1hl_hs_vmax");
}

#[test]
fn test_dcdc1ll_handler() {
    let mut state = test_state();
    let msg = DCDC1LL {
        device_id: external_device(),
        dd_1_lw_sd_vltg_mnmm_lmt_rqst: 12.0,
        dd_1_lw_sd_vltg_mxmm_lmt_rqst: 56.0,
        dd_1_lw_sd_crrnt_mxmm_lmt_rqst: 280.0,
        dd_1_lw_sd_crrnt_mnmm_lmt_rqst: -80.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1ll_low_side_voltage_min_limit, 12.0, 1.0, "dcdc1ll_ls_vmin");
    assert_float_near(state.dcdc.dcdc1ll_low_side_voltage_max_limit, 56.0, 1.0, "dcdc1ll_ls_vmax");
}

#[test]
fn test_dcdc1t_handler() {
    let mut state = test_state();
    let msg = DCDC1T {
        device_id: external_device(),
        dc_dc_1_converter_temperature: 50.0,
        dd_1_cnvrtr_eltrn_fltr_tmprtr: 45.0,
        dd_1_pwr_eltrns_tmprtr: 60.0,
        dc_dc_1_coolant_in_temperature: 28.0,
        dc_dc_1_coolant_out_temperature: 38.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1t_converter_temperature, 50.0, 1.5, "dcdc1t_conv_temp");
    assert_float_near(state.dcdc.dcdc1t_coolant_in_temperature, 28.0, 1.5, "dcdc1t_coolant_in");
}

#[test]
fn test_dcdc1v_handler() {
    let mut state = test_state();
    let msg = DCDC1V {
        device_id: external_device(),
        dd_1_cntrllr_inpt_igntn_vltg: 13.5,
        dd_1_cntrllr_inpt_unswthd_sl_vltg: 14.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1v_ignition_voltage, 13.5, 0.5, "dcdc1v_ign");
    assert_float_near(state.dcdc.dcdc1v_unswitched_sli_voltage, 14.0, 0.5, "dcdc1v_unswitched");
}

#[test]
fn test_dcdc1vc_handler() {
    let mut state = test_state();
    let msg = DCDC1VC {
        device_id: external_device(),
        dc_dc_1_low_side_voltage: 48.5,
        dc_dc_1_low_side_current: 30.0,
        dc_dc_1_high_side_voltage: 810.0,
        dc_dc_1_high_side_current: 2.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1vc_low_side_voltage, 48.5, 1.0, "dcdc1vc_ls_v");
    assert_float_near(state.dcdc.dcdc1vc_high_side_voltage, 810.0, 1.0, "dcdc1vc_hs_v");
}

#[test]
fn test_dcdc1ld_handler() {
    let mut state = test_state();
    let msg = DCDC1LD {
        device_id: external_device(),
        dc_dc_1_total_high_side_energy: 200000.0,
        dc_dc_1_total_low_side_energy: 190000.0,
        dc_dc_1_total_high_side_charge: 60000.0,
        dc_dc_1_total_low_side_charge: 58000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1ld_total_high_side_energy, 200000.0, 100.0, "dcdc1ld_hs_energy");
}

#[test]
fn test_dcdc1sbl_handler() {
    let mut state = test_state();
    let msg = DCDC1SBL {
        device_id: external_device(),
        dd_1_sl_bttr_trmnl_vltg_mxmm_lmt_rqst: 15.0,
        dd_1_s_btt_tc_ct_mx_lt_rqst: 90.0,
        dd_1_sl_bttr_tmprtr_mxmm_lmt_rqst: 55.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1sbl_voltage_max_limit, 15.0, 1.0, "dcdc1sbl_vmax");
}

#[test]
fn test_dcdc1cfg1_handler() {
    let mut state = test_state();
    let msg = DCDC1CFG1 {
        device_id: external_device(),
        dd_1_hgh_sd_vltg_mnmm_lmt_sttng: 660.0,
        dd_1_hgh_sd_vltg_mxmm_lmt_sttng: 880.0,
        dd_1_hgh_sd_crrnt_mxmm_lmt_sttng: 190.0,
        dd_1_lw_sd_vltg_mnmm_lmt_sttng: 11.0,
        dd_1_lw_sd_vltg_mxmm_lmt_sttng: 55.0,
        dd_1_lw_sd_crrnt_mxmm_lmt_sttng: 290.0,
        dd_1_sl_bttr_trmnl_vltg_mxmm_lmt_sttng: 15.5,
        dd_1_s_btt_tc_ct_mx_lt_stt: 95.0,
        dd_1_sl_bttr_tmprtr_mxmm_lmt_sttng: 58.0,
        dd_1_lw_sd_vltg_bk_dflt_sttng: 47.0,
        dd_1_lw_sd_crrnt_mnmm_lmt_sttng: -90.0,
        dd_1_hgh_sd_crrnt_mnmm_lmt_sttng: -45.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1cfg1_hs_voltage_min_limit, 660.0, 1.0, "dcdc1cfg1_hs_vmin");
    assert_float_near(state.dcdc.dcdc1cfg1_ls_voltage_buck_default, 47.0, 1.0, "dcdc1cfg1_ls_default");
}

// ============================================================================
// DCDC2 Messages
// ============================================================================

#[test]
fn test_dcdc2c_handler() {
    let mut state = test_state();
    let msg = DCDC2C {
        device_id: external_device(),
        dc_dc_2_operational_command: 2,
        dc_dc_2_control_counter: 5,
        dc_dc_2_low_side_voltage_buck_setpoint: 52.0,
        dd_2_hgh_sd_vltg_bst_stpnt: 820.0,
        dd_2_lw_sd_vltg_bk_dflt_stpnt: 48.0,
        dc_dc_2_control_crc: 42,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.dcdc.dcdc2c_operational_command, 2);
    assert_float_near(state.dcdc.dcdc2c_low_side_voltage_buck_setpoint, 52.0, 1.0, "dcdc2c_ls_v");
}

#[test]
fn test_dcdc2os_handler() {
    let mut state = test_state();
    let msg = DCDC2OS {
        device_id: external_device(),
        dc_dc_2_operational_status: 3,
        dc_dc_2_hvil_status: 1,
        dc_dc_2_loadshed_request: 0,
        dd_2_pwr_lmt_dt_hgh_sd_crrnt: 1,
        dd_2_pwr_lmt_dt_lw_sd_crrnt: 0,
        dd_2_pwr_lmt_dt_hgh_sd_vltg_mnmm: 0,
        dd_2_pwr_lmt_dt_hgh_sd_vltg_mxmm: 0,
        dd_2_pwr_lmt_dt_lw_sd_vltg_mnmm: 0,
        dd_2_pwr_lmt_dt_lw_sd_vltg_mxmm: 0,
        dd_2_pwr_lmt_dt_cnvrtr_tmprtr: 0,
        dd_2_pwr_lmt_dt_eltrn_fltr_tmprtr: 0,
        dd_2_pwr_lmt_dt_pwr_eltrns_tmprtr: 0,
        dd_2_pwr_lmt_dt_sl_bttr_trmnl_vltg: 0,
        dd_2_pwr_lmt_dt_sl_bttr_trmnl_crrnt: 0,
        dd_2_pwr_lmt_dt_sl_bttr_trmnl_tmprtr: 0,
        dd_2_pwr_lmt_dt_undfnd_rsn: 0,
        dc_dc_2_operating_status_counter: 7,
        dc_dc_2_operating_status_crc: 123,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.dcdc.dcdc2os_operational_status, 3);
    assert_eq!(state.dcdc.dcdc2os_hvil_status, 1);
    assert_eq!(state.dcdc.dcdc2os_operating_status_counter, 7);
}

#[test]
fn test_dcdc2hl_handler() {
    let mut state = test_state();
    let msg = DCDC2HL {
        device_id: external_device(),
        dd_2_hgh_sd_vltg_mnmm_lmt_rqst: 680.0,
        dd_2_hgh_sd_vltg_mxmm_lmt_rqst: 870.0,
        dd_2_hgh_sd_crrnt_mxmm_lmt_rqst: 175.0,
        dd_2_hgh_sd_crrnt_mnmm_lmt_rqst: -35.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2hl_high_side_voltage_min_limit, 680.0, 1.0, "dcdc2hl_hs_vmin");
}

#[test]
fn test_dcdc2ll_handler() {
    let mut state = test_state();
    let msg = DCDC2LL {
        device_id: external_device(),
        dd_2_lw_sd_vltg_mnmm_lmt_rqst: 11.0,
        dd_2_lw_sd_vltg_mxmm_lmt_rqst: 55.0,
        dd_2_lw_sd_crrnt_mxmm_lmt_rqst: 270.0,
        dd_2_lw_sd_crrnt_mnmm_lmt_rqst: -70.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2ll_low_side_voltage_min_limit, 11.0, 1.0, "dcdc2ll_ls_vmin");
}

#[test]
fn test_dcdc2t_handler() {
    let mut state = test_state();
    let msg = DCDC2T {
        device_id: external_device(),
        dc_dc_2_converter_temperature: 48.0,
        dd_2_cnvrtr_eltrn_fltr_tmprtr: 42.0,
        dd_2_pwr_eltrns_tmprtr: 58.0,
        dc_dc_2_coolant_in_temperature: 26.0,
        dc_dc_2_coolant_out_temperature: 36.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2t_converter_temperature, 48.0, 1.5, "dcdc2t_conv_temp");
}

#[test]
fn test_dcdc2v_handler() {
    let mut state = test_state();
    let msg = DCDC2V {
        device_id: external_device(),
        dd_2_cntrllr_inpt_igntn_vltg: 13.2,
        dd_2_cntrllr_inpt_unswthd_sl_vltg: 13.8,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2v_ignition_voltage, 13.2, 0.5, "dcdc2v_ign");
}

#[test]
fn test_dcdc2vc_handler() {
    let mut state = test_state();
    let msg = DCDC2VC {
        device_id: external_device(),
        dc_dc_2_low_side_voltage: 47.0,
        dc_dc_2_low_side_current: 28.0,
        dc_dc_2_high_side_voltage: 795.0,
        dc_dc_2_high_side_current: 1.8,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2vc_low_side_voltage, 47.0, 1.0, "dcdc2vc_ls_v");
}

#[test]
fn test_dcdc2ld_handler() {
    let mut state = test_state();
    let msg = DCDC2LD {
        device_id: external_device(),
        dc_dc_2_total_high_side_energy: 180000.0,
        dc_dc_2_total_low_side_energy: 170000.0,
        dc_dc_2_total_high_side_charge: 55000.0,
        dc_dc_2_total_low_side_charge: 53000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2ld_total_high_side_energy, 180000.0, 100.0, "dcdc2ld_hs_energy");
}

#[test]
fn test_dcdc2sbl_handler() {
    let mut state = test_state();
    let msg = DCDC2SBL {
        device_id: external_device(),
        dd_2_sl_bttr_trmnl_vltg_mxmm_lmt_rqst: 15.5,
        dd_2_s_btt_tc_ct_mx_lt_rqst: 85.0,
        dd_2_sl_bttr_tmprtr_mxmm_lmt_rqst: 58.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2sbl_voltage_max_limit, 15.5, 0.5, "dcdc2sbl_vmax");
}

#[test]
fn test_dcdc2cfg1_handler() {
    let mut state = test_state();
    let msg = DCDC2CFG1 {
        device_id: external_device(),
        dd_2_hgh_sd_vltg_mnmm_lmt_sttng: 660.0,
        dd_2_hgh_sd_vltg_mxmm_lmt_sttng: 880.0,
        dd_2_hgh_sd_crrnt_mxmm_lmt_sttng: 190.0,
        dd_2_lw_sd_vltg_mnmm_lmt_sttng: 11.0,
        dd_2_lw_sd_vltg_mxmm_lmt_sttng: 55.0,
        dd_2_lw_sd_crrnt_mxmm_lmt_sttng: 290.0,
        dd_2_sl_bttr_trmnl_vltg_mxmm_lmt_sttng: 15.5,
        dd_2_s_btt_tc_ct_mx_lt_stt: 95.0,
        dd_2_sl_bttr_tmprtr_mxmm_lmt_sttng: 58.0,
        dd_2_lw_sd_vltg_bk_dflt_sttng: 47.0,
        dd_2_lw_sd_crrnt_mnmm_lmt_sttng: -90.0,
        dd_2_hgh_sd_crrnt_mnmm_lmt_sttng: -45.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2cfg1_hs_voltage_min_limit, 660.0, 1.0, "dcdc2cfg1_hs_vmin");
}

// ============================================================================
// DCDC3 Core + Extended Messages
// ============================================================================

#[test]
fn test_dcdc3c_handler() {
    let mut state = test_state();
    let msg = DCDC3C {
        device_id: external_device(),
        dc_dc_3_operational_command: 1,
        dc_dc_3_control_counter: 3,
        dc_dc_3_low_side_voltage_buck_setpoint: 50.0,
        dd_3_hgh_sd_vltg_bst_stpnt: 815.0,
        dd_3_lw_sd_vltg_bk_dflt_stpnt: 48.0,
        dc_dc_3_control_crc: 77,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.dcdc.dcdc3c_operational_command, 1);
    assert_float_near(state.dcdc.dcdc3c_low_side_voltage_buck_setpoint, 50.0, 1.0, "dcdc3c_ls_v");
}

#[test]
fn test_dcdc3os_handler() {
    let mut state = test_state();
    let msg = DCDC3OS {
        device_id: external_device(),
        dc_dc_3_operational_status: 2,
        dc_dc_3_hvil_status: 1,
        dc_dc_3_loadshed_request: 0,
        dd_3_pwr_lmt_dt_hgh_sd_crrnt: 0,
        dd_3_pwr_lmt_dt_lw_sd_crrnt: 0,
        dd_3_pwr_lmt_dt_hgh_sd_vltg_mnmm: 0,
        dd_3_pwr_lmt_dt_hgh_sd_vltg_mxmm: 0,
        dd_3_pwr_lmt_dt_lw_sd_vltg_mnmm: 0,
        dd_3_pwr_lmt_dt_lw_sd_vltg_mxmm: 0,
        dd_3_pwr_lmt_dt_cnvrtr_tmprtr: 0,
        dd_3_pwr_lmt_dt_eltrn_fltr_tmprtr: 0,
        dd_3_pwr_lmt_dt_pwr_eltrns_tmprtr: 0,
        dd_3_pwr_lmt_dt_sl_bttr_trmnl_vltg: 0,
        dd_3_pwr_lmt_dt_sl_bttr_trmnl_crrnt: 0,
        dd_3_pwr_lmt_dt_sl_bttr_trmnl_tmprtr: 0,
        dd_3_pwr_lmt_dt_undfnd_rsn: 0,
        dc_dc_3_operating_status_counter: 4,
        dc_dc_3_operating_status_crc: 88,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.dcdc.dcdc3os_operational_status, 2);
    assert_eq!(state.dcdc.dcdc3os_hvil_status, 1);
}

#[test]
fn test_dcdc3s2_handler() {
    let mut state = test_state();
    let msg = DCDC3S2 {
        device_id: external_device(),
        dc_dc_3_low_side_power: -160.0,
        dc_dc_3_high_side_power: 180.0,
        dd_3_hgh_sd_ngtv_t_chsss_grnd_vltg: 395.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3s2_high_side_power, 180.0, 1.0, "dcdc3s2_hs_power");
}

#[test]
fn test_dcdc3sbs_handler() {
    let mut state = test_state();
    let msg = DCDC3SBS {
        device_id: external_device(),
        dc_dc_3_sli_battery_terminal_voltage: 14.2,
        dc_dc_3_sli_battery_terminal_current: 22.0,
        dd_3_sl_bttr_trmnl_tmprtr: 28.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3sbs_terminal_voltage, 14.2, 0.5, "dcdc3sbs_v");
}

#[test]
fn test_dcdc3t_handler() {
    let mut state = test_state();
    let msg = DCDC3T {
        device_id: external_device(),
        dc_dc_3_converter_temperature: 44.0,
        dd_3_cnvrtr_eltrn_fltr_tmprtr: 39.0,
        dd_3_pwr_eltrns_tmprtr: 53.0,
        dc_dc_3_coolant_in_temperature: 27.0,
        dc_dc_3_coolant_out_temperature: 34.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3t_converter_temperature, 44.0, 1.5, "dcdc3t_conv_temp");
}

#[test]
fn test_dcdc3v_handler() {
    let mut state = test_state();
    let msg = DCDC3V {
        device_id: external_device(),
        dd_3_cntrllr_inpt_igntn_vltg: 13.0,
        dd_3_cntrllr_inpt_unswthd_sl_vltg: 13.6,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3v_ignition_voltage, 13.0, 0.5, "dcdc3v_ign");
}

#[test]
fn test_dcdc3vc_handler() {
    let mut state = test_state();
    let msg = DCDC3VC {
        device_id: external_device(),
        dc_dc_3_low_side_voltage: 47.5,
        dc_dc_3_low_side_current: 24.0,
        dc_dc_3_high_side_voltage: 805.0,
        dc_dc_3_high_side_current: 1.4,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3vc_low_side_voltage, 47.5, 1.0, "dcdc3vc_ls_v");
}

#[test]
fn test_dcdc3sbl_handler() {
    let mut state = test_state();
    let msg = DCDC3SBL {
        device_id: external_device(),
        dd_3_sl_bttr_trmnl_vltg_mxmm_lmt_rqst: 15.8,
        dd_3_s_btt_tc_ct_mx_lt_rqst: 88.0,
        dd_3_sl_bttr_tmprtr_mxmm_lmt_rqst: 57.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3sbl_voltage_max_limit, 15.8, 0.5, "dcdc3sbl_vmax");
}

#[test]
fn test_dcdc3ll_handler() {
    let mut state = test_state();
    let msg = DCDC3LL {
        device_id: external_device(),
        dd_3_lw_sd_vltg_mnmm_lmt_rqst: 11.5,
        dd_3_lw_sd_vltg_mxmm_lmt_rqst: 56.0,
        dd_3_lw_sd_crrnt_mxmm_lmt_rqst: 280.0,
        dd_3_lw_sd_crrnt_mnmm_lmt_rqst: -85.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3ll_low_side_voltage_min_limit, 11.5, 1.0, "dcdc3ll_ls_vmin");
}

#[test]
fn test_dcdc3hl_handler() {
    let mut state = test_state();
    let msg = DCDC3HL {
        device_id: external_device(),
        dd_3_hgh_sd_vltg_mnmm_lmt_rqst: 670.0,
        dd_3_hgh_sd_vltg_mxmm_lmt_rqst: 860.0,
        dd_3_hgh_sd_crrnt_mxmm_lmt_rqst: 185.0,
        dd_3_hgh_sd_crrnt_mnmm_lmt_rqst: -42.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3hl_high_side_voltage_min_limit, 670.0, 1.0, "dcdc3hl_hs_vmin");
}

#[test]
fn test_dcdc3ld_handler() {
    let mut state = test_state();
    let msg = DCDC3LD {
        device_id: external_device(),
        dc_dc_3_total_high_side_energy: 160000.0,
        dc_dc_3_total_low_side_energy: 155000.0,
        dc_dc_3_total_high_side_charge: 52000.0,
        dc_dc_3_total_low_side_charge: 50000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3ld_total_high_side_energy, 160000.0, 100.0, "dcdc3ld_hs_energy");
}

#[test]
fn test_dcdc3cfg1_handler() {
    let mut state = test_state();
    let msg = DCDC3CFG1 {
        device_id: external_device(),
        dd_3_hgh_sd_vltg_mnmm_lmt_sttng: 655.0,
        dd_3_hgh_sd_vltg_mxmm_lmt_sttng: 875.0,
        dd_3_hgh_sd_crrnt_mxmm_lmt_sttng: 185.0,
        dd_3_lw_sd_vltg_mnmm_lmt_sttng: 10.5,
        dd_3_lw_sd_vltg_mxmm_lmt_sttng: 54.0,
        dd_3_lw_sd_crrnt_mxmm_lmt_sttng: 285.0,
        dd_3_sl_bttr_trmnl_vltg_mxmm_lmt_sttng: 15.2,
        dd_3_s_btt_tc_ct_mx_lt_stt: 92.0,
        dd_3_sl_bttr_tmprtr_mxmm_lmt_sttng: 56.0,
        dd_3_lw_sd_vltg_bk_dflt_sttng: 46.0,
        dd_3_lw_sd_crrnt_mnmm_lmt_sttng: -88.0,
        dd_3_hgh_sd_crrnt_mnmm_lmt_sttng: -43.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc3cfg1_hs_voltage_min_limit, 655.0, 1.0, "dcdc3cfg1_hs_vmin");
}

// ============================================================================
// DCDC4 Core + Extended Messages
// ============================================================================

#[test]
fn test_dcdc4c_handler() {
    let mut state = test_state();
    let msg = DCDC4C {
        device_id: external_device(),
        dc_dc_4_operational_command: 1,
        dc_dc_4_control_counter: 2,
        dc_dc_4_low_side_voltage_buck_setpoint: 49.0,
        dd_4_hgh_sd_vltg_bst_stpnt: 810.0,
        dd_4_lw_sd_vltg_bk_dflt_stpnt: 48.0,
        dc_dc_4_control_crc: 55,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.dcdc.dcdc4c_operational_command, 1);
    assert_float_near(state.dcdc.dcdc4c_low_side_voltage_buck_setpoint, 49.0, 1.0, "dcdc4c_ls_v");
}

#[test]
fn test_dcdc4os_handler() {
    let mut state = test_state();
    let msg = DCDC4OS {
        device_id: external_device(),
        dc_dc_4_operational_status: 2,
        dc_dc_4_hvil_status: 1,
        dc_dc_4_loadshed_request: 0,
        dd_4_pwr_lmt_dt_hgh_sd_crrnt: 0,
        dd_4_pwr_lmt_dt_lw_sd_crrnt: 0,
        dd_4_pwr_lmt_dt_hgh_sd_vltg_mnmm: 0,
        dd_4_pwr_lmt_dt_hgh_sd_vltg_mxmm: 0,
        dd_4_pwr_lmt_dt_lw_sd_vltg_mnmm: 0,
        dd_4_pwr_lmt_dt_lw_sd_vltg_mxmm: 0,
        dd_4_pwr_lmt_dt_cnvrtr_tmprtr: 0,
        dd_4_pwr_lmt_dt_eltrn_fltr_tmprtr: 0,
        dd_4_pwr_lmt_dt_pwr_eltrns_tmprtr: 0,
        dd_4_pwr_lmt_dt_sl_bttr_trmnl_vltg: 0,
        dd_4_pwr_lmt_dt_sl_bttr_trmnl_crrnt: 0,
        dd_4_pwr_lmt_dt_sl_bttr_trmnl_tmprtr: 0,
        dd_4_pwr_lmt_dt_undfnd_rsn: 0,
        dc_dc_4_operating_status_counter: 6,
        dc_dc_4_operating_status_crc: 99,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.dcdc.dcdc4os_operational_status, 2);
}

#[test]
fn test_dcdc4s2_handler() {
    let mut state = test_state();
    let msg = DCDC4S2 {
        device_id: external_device(),
        dc_dc_4_low_side_power: -150.0,
        dc_dc_4_high_side_power: 170.0,
        dd_4_hgh_sd_ngtv_t_chsss_grnd_vltg: 390.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4s2_high_side_power, 170.0, 1.0, "dcdc4s2_hs_power");
}

#[test]
fn test_dcdc4sbs_handler() {
    let mut state = test_state();
    let msg = DCDC4SBS {
        device_id: external_device(),
        dc_dc_4_sli_battery_terminal_voltage: 14.0,
        dc_dc_4_sli_battery_terminal_current: 19.0,
        dd_4_sl_bttr_trmnl_tmprtr: 26.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4sbs_terminal_voltage, 14.0, 0.5, "dcdc4sbs_v");
}

#[test]
fn test_dcdc4t_handler() {
    let mut state = test_state();
    let msg = DCDC4T {
        device_id: external_device(),
        dc_dc_4_converter_temperature: 41.0,
        dd_4_cnvrtr_eltrn_fltr_tmprtr: 37.0,
        dd_4_pwr_eltrns_tmprtr: 51.0,
        dc_dc_4_coolant_in_temperature: 25.0,
        dc_dc_4_coolant_out_temperature: 32.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4t_converter_temperature, 41.0, 1.5, "dcdc4t_conv_temp");
}

#[test]
fn test_dcdc4v_handler() {
    let mut state = test_state();
    let msg = DCDC4V {
        device_id: external_device(),
        dd_4_cntrllr_inpt_igntn_vltg: 12.9,
        dd_4_cntrllr_inpt_unswthd_sl_vltg: 13.4,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4v_ignition_voltage, 12.9, 0.5, "dcdc4v_ign");
}

#[test]
fn test_dcdc4vc_handler() {
    let mut state = test_state();
    let msg = DCDC4VC {
        device_id: external_device(),
        dc_dc_4_low_side_voltage: 47.0,
        dc_dc_4_low_side_current: 21.0,
        dc_dc_4_high_side_voltage: 798.0,
        dc_dc_4_high_side_current: 1.3,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4vc_low_side_voltage, 47.0, 1.0, "dcdc4vc_ls_v");
}

#[test]
fn test_dcdc4sbl_handler() {
    let mut state = test_state();
    let msg = DCDC4SBL {
        device_id: external_device(),
        dd_4_sl_bttr_trmnl_vltg_mxmm_lmt_rqst: 15.6,
        dd_4_s_btt_tc_ct_mx_lt_rqst: 86.0,
        dd_4_sl_bttr_tmprtr_mxmm_lmt_rqst: 56.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4sbl_voltage_max_limit, 15.6, 0.5, "dcdc4sbl_vmax");
}

#[test]
fn test_dcdc4ll_handler() {
    let mut state = test_state();
    let msg = DCDC4LL {
        device_id: external_device(),
        dd_4_lw_sd_vltg_mnmm_lmt_rqst: 10.5,
        dd_4_lw_sd_vltg_mxmm_lmt_rqst: 55.0,
        dd_4_lw_sd_crrnt_mxmm_lmt_rqst: 275.0,
        dd_4_lw_sd_crrnt_mnmm_lmt_rqst: -80.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4ll_low_side_voltage_min_limit, 10.5, 1.0, "dcdc4ll_ls_vmin");
}

#[test]
fn test_dcdc4hl_handler() {
    let mut state = test_state();
    let msg = DCDC4HL {
        device_id: external_device(),
        dd_4_hgh_sd_vltg_mnmm_lmt_rqst: 660.0,
        dd_4_hgh_sd_vltg_mxmm_lmt_rqst: 855.0,
        dd_4_hgh_sd_crrnt_mxmm_lmt_rqst: 180.0,
        dd_4_hgh_sd_crrnt_mnmm_lmt_rqst: -38.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4hl_high_side_voltage_min_limit, 660.0, 1.0, "dcdc4hl_hs_vmin");
}

#[test]
fn test_dcdc4ld_handler() {
    let mut state = test_state();
    let msg = DCDC4LD {
        device_id: external_device(),
        dc_dc_4_total_high_side_energy: 140000.0,
        dc_dc_4_total_low_side_energy: 135000.0,
        dc_dc_4_total_high_side_charge: 48000.0,
        dc_dc_4_total_low_side_charge: 46000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4ld_total_high_side_energy, 140000.0, 100.0, "dcdc4ld_hs_energy");
}

#[test]
fn test_dcdc4cfg1_handler() {
    let mut state = test_state();
    let msg = DCDC4CFG1 {
        device_id: external_device(),
        dd_4_hgh_sd_vltg_mnmm_lmt_sttng: 640.0,
        dd_4_hgh_sd_vltg_mxmm_lmt_sttng: 870.0,
        dd_4_hgh_sd_crrnt_mxmm_lmt_sttng: 180.0,
        dd_4_lw_sd_vltg_mnmm_lmt_sttng: 10.0,
        dd_4_lw_sd_vltg_mxmm_lmt_sttng: 53.0,
        dd_4_lw_sd_crrnt_mxmm_lmt_sttng: 280.0,
        dd_4_sl_bttr_trmnl_vltg_mxmm_lmt_sttng: 15.0,
        dd_4_s_btt_tc_ct_mx_lt_stt: 90.0,
        dd_4_sl_bttr_tmprtr_mxmm_lmt_sttng: 55.0,
        dd_4_lw_sd_vltg_bk_dflt_sttng: 45.0,
        dd_4_lw_sd_crrnt_mnmm_lmt_sttng: -85.0,
        dd_4_hgh_sd_crrnt_mnmm_lmt_sttng: -40.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc4cfg1_hs_voltage_min_limit, 640.0, 1.0, "dcdc4cfg1_hs_vmin");
}

// ============================================================================
// Power Supply Messages (GC1, GTRACE2, GAAC)
// ============================================================================

#[test]
fn test_gc1_handler() {
    let mut state = test_state();
    let msg = GC1 {
        device_id: external_device(),
        requested_engine_control_mode: 2,
        gnrtr_cntrl_nt_in_atmt_strt_stt: 0,
        gnrtr_nt_rd_t_atmtll_prlll_stt: 1,
        generator_alternator_efficiency: 95.0,
        generator_governing_speed_command: 1,
        generator_frequency_selection: 0,
        engine_speed_governor_gain_adjust: 55.0,
        engine_speed_governor_droop: 4.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.power_supply.gc1_requested_engine_control_mode, 2);
    assert_float_near(state.power_supply.gc1_alternator_efficiency, 95.0, 1.0, "gc1_efficiency");
    assert_float_near(state.power_supply.gc1_speed_governor_droop, 4.0, 0.5, "gc1_droop");
}

#[test]
fn test_gtrace2_handler() {
    let mut state = test_state();
    let msg = GTRACE2 {
        device_id: external_device(),
        generator_trip_kvar_hours_import: 600000,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.power_supply.gtrace2_kvarh_import, 600000);
}

#[test]
fn test_gaac_handler() {
    let mut state = test_state();
    let msg = GAAC {
        device_id: external_device(),
        gnrtr_avrg_ln_ln_a_rms_vltg: 460,
        gnrtr_avrg_ln_ntrl_a_rms_vltg: 265,
        generator_average_ac_frequency: 59.5,
        generator_average_ac_rms_current: 110,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.power_supply.gaac_avg_line_line_voltage, 460);
    assert_eq!(state.power_supply.gaac_avg_line_neutral_voltage, 265);
    assert_float_near(state.power_supply.gaac_avg_frequency, 59.5, 0.5, "gaac_freq");
    assert_eq!(state.power_supply.gaac_avg_rms_current, 110);
}

// ============================================================================
// Broadcast Verification Tests
// ============================================================================

#[test]
fn test_batch9_broadcasts_generate_frames() {
    let state = test_state();
    let frames = state.generate_can_frames();

    // Check key Batch 9 frames appear in broadcast output
    // Note: LD (DLC=16) and CFG1 (DLC=23) messages are CAN FD only and cannot
    // be broadcast as standard 8-byte CAN frames, so they are excluded here.
    let batch9_base_ids = vec![
        DCDC1HL::BASE_CAN_ID,
        DCDC1LL::BASE_CAN_ID,
        DCDC1T::BASE_CAN_ID,
        DCDC1V::BASE_CAN_ID,
        DCDC1VC::BASE_CAN_ID,
        // DCDC1LD - DLC=16, CAN FD only
        DCDC1SBL::BASE_CAN_ID,
        // DCDC1CFG1 - DLC=23, CAN FD only
        DCDC2C::BASE_CAN_ID,
        DCDC2OS::BASE_CAN_ID,
        DCDC2HL::BASE_CAN_ID,
        DCDC2LL::BASE_CAN_ID,
        DCDC2T::BASE_CAN_ID,
        DCDC2V::BASE_CAN_ID,
        DCDC2VC::BASE_CAN_ID,
        // DCDC2LD - DLC=16, CAN FD only
        DCDC2SBL::BASE_CAN_ID,
        // DCDC2CFG1 - DLC=23, CAN FD only
        DCDC3C::BASE_CAN_ID,
        DCDC3OS::BASE_CAN_ID,
        DCDC3S2::BASE_CAN_ID,
        DCDC3SBS::BASE_CAN_ID,
        DCDC3T::BASE_CAN_ID,
        DCDC3V::BASE_CAN_ID,
        DCDC3VC::BASE_CAN_ID,
        DCDC3SBL::BASE_CAN_ID,
        DCDC3LL::BASE_CAN_ID,
        DCDC3HL::BASE_CAN_ID,
        // DCDC3LD - DLC=16, CAN FD only
        // DCDC3CFG1 - DLC=23, CAN FD only
        DCDC4C::BASE_CAN_ID,
        DCDC4OS::BASE_CAN_ID,
        DCDC4S2::BASE_CAN_ID,
        DCDC4SBS::BASE_CAN_ID,
        DCDC4T::BASE_CAN_ID,
        DCDC4V::BASE_CAN_ID,
        DCDC4VC::BASE_CAN_ID,
        DCDC4SBL::BASE_CAN_ID,
        DCDC4LL::BASE_CAN_ID,
        DCDC4HL::BASE_CAN_ID,
        // DCDC4LD - DLC=16, CAN FD only
        // DCDC4CFG1 - DLC=23, CAN FD only
        GC1::BASE_CAN_ID,
        GTRACE2::BASE_CAN_ID,
        GAAC::BASE_CAN_ID,
    ];

    for base_id in &batch9_base_ids {
        let found = frames.iter().any(|f| matches_base_id(f.raw_id(), *base_id));
        assert!(
            found,
            "Expected broadcast frame for BASE_CAN_ID 0x{:08X} not found",
            base_id
        );
    }
}

#[test]
fn test_dcdc1vc_round_trip() {
    // Set state -> broadcast -> decode -> verify
    let mut state = test_state();
    state.dcdc.dcdc1vc_low_side_voltage = 50.0;
    state.dcdc.dcdc1vc_low_side_current = 35.0;
    state.dcdc.dcdc1vc_high_side_voltage = 820.0;
    state.dcdc.dcdc1vc_high_side_current = 2.5;

    let frames = state.generate_can_frames();
    let vc_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), DCDC1VC::BASE_CAN_ID))
        .expect("DCDC1VC frame not found");

    let decoded = DCDC1VC::decode(frame_can_id(vc_frame), vc_frame.data()).unwrap();
    assert_float_near(decoded.dc_dc_1_low_side_voltage, 50.0, 1.0, "rt_dcdc1vc_ls_v");
    assert_float_near(decoded.dc_dc_1_high_side_voltage, 820.0, 1.0, "rt_dcdc1vc_hs_v");
}

#[test]
fn test_gc1_round_trip() {
    let mut state = test_state();
    state.power_supply.gc1_requested_engine_control_mode = 3;
    state.power_supply.gc1_alternator_efficiency = 88.0;
    state.power_supply.gc1_speed_governor_droop = 6.0;

    let frames = state.generate_can_frames();
    let gc1_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), GC1::BASE_CAN_ID))
        .expect("GC1 frame not found");

    let decoded = GC1::decode(frame_can_id(gc1_frame), gc1_frame.data()).unwrap();
    assert_eq!(decoded.requested_engine_control_mode, 3);
    assert_float_near(decoded.generator_alternator_efficiency, 88.0, 1.0, "rt_gc1_eff");
    assert_float_near(decoded.engine_speed_governor_droop, 6.0, 1.0, "rt_gc1_droop");
}

#[test]
fn test_gaac_round_trip() {
    let mut state = test_state();
    state.power_supply.gaac_avg_line_line_voltage = 500;
    state.power_supply.gaac_avg_frequency = 50.0;
    state.power_supply.gaac_avg_rms_current = 200;

    let frames = state.generate_can_frames();
    let gaac_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), GAAC::BASE_CAN_ID))
        .expect("GAAC frame not found");

    let decoded = GAAC::decode(frame_can_id(gaac_frame), gaac_frame.data()).unwrap();
    assert_eq!(decoded.gnrtr_avrg_ln_ln_a_rms_vltg, 500);
    assert_float_near(decoded.generator_average_ac_frequency, 50.0, 1.0, "rt_gaac_freq");
    assert_eq!(decoded.generator_average_ac_rms_current, 200);
}

// ============================================================================
// Physics Tests: DC-DC Voltage Regulation
// ============================================================================

#[test]
fn test_dcdc_voltage_regulation_converges() {
    let mut state = test_state();
    // Set operational command active and setpoint to 48V
    state.dcdc.dcdc_operational_command = 1;
    state.dcdc.dcdc_low_side_voltage_setpoint = 48.0;

    // Run physics multiple steps to let voltage converge
    for _ in 0..100 {
        state.update_physics(0.1);
    }

    // With setpoint = 48V and motor physics also driving toward 48V (48 - current*0.1,
    // current defaults to 0), the equilibrium should be very close to 48V.
    // The DCDC regulation formula is: voltage = (voltage * 0.9 + target * 0.1).clamp(40, 60)
    assert_float_near(state.motor.mg1_voltage, 48.0, 1.0, "mg1_voltage should converge near 48V");
    assert_float_near(state.motor.mg2_voltage, 48.0, 1.0, "mg2_voltage should converge near 48V");
    // Both must be within the clamp range [40, 60]
    assert!(state.motor.mg1_voltage >= 40.0 && state.motor.mg1_voltage <= 60.0,
        "mg1_voltage {} should be clamped within [40, 60]", state.motor.mg1_voltage);
    assert!(state.motor.mg2_voltage >= 40.0 && state.motor.mg2_voltage <= 60.0,
        "mg2_voltage {} should be clamped within [40, 60]", state.motor.mg2_voltage);
}

#[test]
fn test_dcdc_no_regulation_when_command_zero() {
    let mut state = test_state();
    state.dcdc.dcdc_operational_command = 0;
    // Set a setpoint that would cause change if regulation were active
    state.dcdc.dcdc_low_side_voltage_setpoint = 55.0;

    // Record initial voltage (default is 48.0)
    let initial_mg1 = state.motor.mg1_voltage;
    let initial_mg2 = state.motor.mg2_voltage;

    // Run physics a few steps
    for _ in 0..10 {
        state.update_physics(0.1);
    }

    // Motor physics still runs (sets voltage = 48 - current*0.1), but DCDC regulation
    // should NOT blend toward the 55V setpoint. With zero current, motor physics
    // keeps voltage at 48V, so the voltage should stay at the motor-physics equilibrium.
    assert_float_near(state.motor.mg1_voltage, initial_mg1, 1.0,
        "mg1_voltage should not regulate toward setpoint when command=0");
    assert_float_near(state.motor.mg2_voltage, initial_mg2, 1.0,
        "mg2_voltage should not regulate toward setpoint when command=0");
}

// ============================================================================
// Self-Reception Test
// ============================================================================

#[test]
fn test_dcdc1hl_self_reception_ignored() {
    let mut state = test_state();
    // Send DCDC1HL from our own device ID (0x82) - should be ignored
    let msg = DCDC1HL {
        device_id: DeviceId::from(0x82),
        dd_1_hgh_sd_vltg_mnmm_lmt_rqst: 999.0,
        dd_1_hgh_sd_vltg_mxmm_lmt_rqst: 999.0,
        dd_1_hgh_sd_crrnt_mxmm_lmt_rqst: 999.0,
        dd_1_hgh_sd_crrnt_mnmm_lmt_rqst: -999.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Ignored);
    // State should NOT have been updated - should remain at defaults
    assert_float_near(state.dcdc.dcdc1hl_high_side_voltage_min_limit, 650.0, 1.0,
        "dcdc1hl_hs_vmin should remain at default after self-reception");
}

// ============================================================================
// DecodeFailed Test
// ============================================================================

#[test]
fn test_batch9_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = DCDC1HL::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated -- triggers decode error
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
