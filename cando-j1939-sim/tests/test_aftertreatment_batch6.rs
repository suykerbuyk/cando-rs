//! Batch 6: Aftertreatment Bank 1 Message Tests
//!
//! Tests for 23 aftertreatment messages (Bank 1):
//! AT1S1, AT1S2, AT1T1I1, AT1T1I2, AT1TI, AT1OG1, AT1OG2, AT1IG1, AT1IG2,
//! AT1HI1, AT1GP, AT1FC1, AT1FC2, AT1AC1, A1DOC1, A1DOC2, A1SCRAI,
//! A1SCRSI1, A1SCRSI2, DPF1S, DPF1S2, DPFC1, DPFC2

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
// AT1S1 - Aftertreatment 1 Service 1
// ============================================================================

#[test]
fn test_at1s1_handler() {
    let mut state = test_state();
    let msg = AT1S1 {
        device_id: external_device(),
        aftrtrtmnt_1_dsl_prtlt_fltr_st_ld_prnt: 50,
        atttt_1_ds_ptt_ft_as_ld_pt: 15,
        atttt_1_ds_ptt_ft_ts_lst_atv_rt: 3600,
        atttt_1_ds_ptt_ft_st_ld_rt_tsd: 90.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.aftertreatment.at1s1_dpf_soot_load_percent, 50);
    assert_eq!(state.aftertreatment.at1s1_dpf_ash_load_percent, 15);
    assert_eq!(state.aftertreatment.at1s1_dpf_time_since_last_regen, 3600);
    assert_float_near(
        state.aftertreatment.at1s1_dpf_soot_load_regen_threshold,
        90.0,
        1.0,
        "dpf_soot_load_regen_threshold",
    );
}

#[test]
fn test_at1s1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1S1::BASE_CAN_ID));
    assert!(found, "AT1S1 frame should be present in broadcasts");
}

// ============================================================================
// AT1S2 - Aftertreatment 1 Service 2
// ============================================================================

#[test]
fn test_at1s2_handler() {
    let mut state = test_state();
    let msg = AT1S2 {
        device_id: external_device(),
        atttt_1_ds_ptt_ft_tt_nxt_atv_rt: 7200,
        atttt_1_s_sst_ts_lst_sst_c_evt: 43200,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.aftertreatment.at1s2_dpf_time_to_next_regen, 7200);
    assert_eq!(state.aftertreatment.at1s2_scr_time_since_cleaning, 43200);
}

#[test]
fn test_at1s2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1S2::BASE_CAN_ID));
    assert!(found, "AT1S2 frame should be present in broadcasts");
}

// ============================================================================
// AT1T1I1 - Aftertreatment 1 DEF Tank 1 Information 1
// ============================================================================

#[test]
fn test_at1t1i1_handler() {
    let mut state = test_state();
    let msg = AT1T1I1 {
        device_id: external_device(),
        aftrtrtmnt_1_dsl_exhst_fld_tnk_vlm: 80.0,
        atttt_1_ds_exst_fd_t_tpt_1: 30.0,
        aftrtrtmnt_1_dsl_exhst_fld_tnk_lvl: 500.0,
        atttt_1_ds_exst_fd_t_lv_vpf: 31,
        atttt_ds_exst_fd_t_lw_lv_idt: 0,
        atttt_1_ds_exst_fd_t_1_tpt_pf: 31,
        aftrtrtmnt_sr_oprtr_indmnt_svrt: 0,
        aftrtrtmnt_1_dsl_exhst_fld_tnk_htr: 25.0,
        atttt_1_ds_exst_fd_t_1_ht_pf: 31,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1t1i1_def_tank_volume,
        80.0,
        1.0,
        "def_tank_volume",
    );
    assert_float_near(
        state.aftertreatment.at1t1i1_def_tank_temp,
        30.0,
        1.0,
        "def_tank_temp",
    );
    assert_float_near(
        state.aftertreatment.at1t1i1_def_tank_level,
        500.0,
        5.0,
        "def_tank_level",
    );
    assert_float_near(
        state.aftertreatment.at1t1i1_def_tank_heater,
        25.0,
        1.0,
        "def_tank_heater",
    );
}

#[test]
fn test_at1t1i1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1T1I1::BASE_CAN_ID));
    assert!(found, "AT1T1I1 frame should be present in broadcasts");
}

// ============================================================================
// AT1T1I2 - Aftertreatment 1 DEF Tank 1 Information 2
// ============================================================================

#[test]
fn test_at1t1i2_handler() {
    let mut state = test_state();
    let msg = AT1T1I2 {
        device_id: external_device(),
        aftrtrtmnt_1_dsl_exhst_fld_tnk_vlm_2: 85.0,
        atttt_1_ds_exst_fd_t_tpt_2: 28.0,
        aftrtrtmnt_1_dsl_exhst_fld_tnk_htr_2: 30.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1t1i2_def_tank_volume_2,
        85.0,
        1.0,
        "def_tank_volume_2",
    );
    assert_float_near(
        state.aftertreatment.at1t1i2_def_tank_temp_2,
        28.0,
        1.0,
        "def_tank_temp_2",
    );
    assert_float_near(
        state.aftertreatment.at1t1i2_def_tank_heater_2,
        30.0,
        1.0,
        "def_tank_heater_2",
    );
}

#[test]
fn test_at1t1i2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1T1I2::BASE_CAN_ID));
    assert!(found, "AT1T1I2 frame should be present in broadcasts");
}

// ============================================================================
// AT1TI - Aftertreatment 1 Trip Information
// ============================================================================

#[test]
fn test_at1ti_handler() {
    let mut state = test_state();
    let msg = AT1TI {
        device_id: external_device(),
        aftrtrtmnt_1_dsl_prtlt_fltr_trp_fl_usd: 10.0,
        atttt_1_ds_ptt_ft_tp_atv_rt_t: 1200,
        atttt_1_ds_ptt_ft_tp_dsd_t: 300,
        atttt_1_ds_ptt_ft_tp_no_atv_rts: 5,
        atttt_1_ds_ptt_ft_tp_pssv_rt_t: 2400,
        atttt_1_ds_ptt_ft_tp_no_pssv_rts: 8,
        atttt_1_ds_ptt_ft_tp_no_atv_rt_it_rqsts: 2,
        atttt_1_ds_ptt_ft_tp_no_atv_rt_m_rqsts: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1ti_dpf_trip_fuel_used,
        10.0,
        1.0,
        "dpf_trip_fuel_used",
    );
    assert_eq!(state.aftertreatment.at1ti_dpf_trip_active_regen_time, 1200);
    assert_eq!(state.aftertreatment.at1ti_dpf_trip_disabled_time, 300);
    assert_eq!(state.aftertreatment.at1ti_dpf_trip_num_active_regens, 5);
}

#[test]
fn test_at1ti_broadcast_skipped_due_to_dlc() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1TI::BASE_CAN_ID));
    assert!(
        !found,
        "AT1TI frame should NOT be in broadcasts (DLC=32 exceeds 8-byte CAN limit)"
    );
}

// ============================================================================
// AT1OG1 - Aftertreatment 1 Outlet Gas 1
// ============================================================================

#[test]
fn test_at1og1_handler() {
    let mut state = test_state();
    let msg = AT1OG1 {
        device_id: external_device(),
        aftertreatment_1_outlet_nox_1: 50.0,
        aftrtrtmnt_1_otlt_prnt_oxgn_1: 12.0,
        aftrtrtmnt_1_otlt_gs_snsr_1_pwr_in_rng: 1,
        aftrtrtmnt_1_otlt_gs_snsr_1_at_tmprtr: 1,
        aftrtrtmnt_1_otlt_nx_1_rdng_stl: 1,
        atttt_1_ott_wd_r_pt_ox_1_rd_st: 1,
        atttt_1_ott_gs_ss_1_ht_pf: 31,
        aftrtrtmnt_1_otlt_gs_snsr_1_htr_cntrl: 3,
        aftrtrtmnt_1_otlt_nx_snsr_1_prlmnr_fm: 31,
        atttt_1_ott_nx_ss_1_s_dss_stts: 0,
        atttt_1_ott_ox_ss_1_pf: 31,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1og1_outlet_nox,
        50.0,
        1.0,
        "outlet_nox",
    );
    assert_float_near(
        state.aftertreatment.at1og1_outlet_oxygen,
        12.0,
        1.0,
        "outlet_oxygen",
    );
}

#[test]
fn test_at1og1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1OG1::BASE_CAN_ID));
    assert!(found, "AT1OG1 frame should be present in broadcasts");
}

// ============================================================================
// AT1OG2 - Aftertreatment 1 Outlet Gas 2
// ============================================================================

#[test]
fn test_at1og2_handler() {
    let mut state = test_state();
    let msg = AT1OG2 {
        device_id: external_device(),
        aftrtrtmnt_1_exhst_tmprtr_3: 300.0,
        atttt_1_ds_ptt_ft_ott_tpt: 310.0,
        aftrtrtmnt_1_exhst_tmprtr_3_prlmnr_fm: 31,
        atttt_1_ds_ptt_ft_ott_exst_tpt_pf: 31,
        aftrtrtmnt_exhst_1_dw_pnt_dttd: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1og2_exhaust_temp_3,
        300.0,
        1.0,
        "exhaust_temp_3",
    );
    assert_float_near(
        state.aftertreatment.at1og2_dpf_outlet_temp,
        310.0,
        1.0,
        "dpf_outlet_temp",
    );
}

#[test]
fn test_at1og2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1OG2::BASE_CAN_ID));
    assert!(found, "AT1OG2 frame should be present in broadcasts");
}

// ============================================================================
// AT1IG1 - Aftertreatment 1 Intake Gas 1
// ============================================================================

#[test]
fn test_at1ig1_handler() {
    let mut state = test_state();
    let msg = AT1IG1 {
        device_id: external_device(),
        engine_exhaust_1_nox_1: 500.0,
        engine_exhaust_1_percent_oxygen_1: 9.0,
        engn_exhst_1_gs_snsr_1_pwr_in_rng: 1,
        engn_exhst_1_gs_snsr_1_at_tmprtr: 1,
        engine_exhaust_1_nox_1_reading_stable: 1,
        engn_exhst_1_wd_rng_prnt_oxgn_1_rdng_stl: 1,
        engn_exhst_1_gs_snsr_1_htr_prlmnr_fm: 31,
        engn_exhst_1_gs_snsr_1_htr_cntrl: 3,
        engn_exhst_1_nx_snsr_1_prlmnr_fm: 31,
        engn_exhst_1_nx_snsr_1_slf_dgnss_stts: 0,
        engn_exhst_1_oxgn_snsr_1_prlmnr_fm: 31,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1ig1_inlet_nox,
        500.0,
        5.0,
        "inlet_nox",
    );
    assert_float_near(
        state.aftertreatment.at1ig1_inlet_oxygen,
        9.0,
        1.0,
        "inlet_oxygen",
    );
}

#[test]
fn test_at1ig1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1IG1::BASE_CAN_ID));
    assert!(found, "AT1IG1 frame should be present in broadcasts");
}

// ============================================================================
// AT1IG2 - Aftertreatment 1 Intake Gas 2
// ============================================================================

#[test]
fn test_at1ig2_handler() {
    let mut state = test_state();
    let msg = AT1IG2 {
        device_id: external_device(),
        aftrtrtmnt_1_exhst_tmprtr_1: 350.0,
        atttt_1_ds_ptt_ft_it_tpt: 360.0,
        aftrtrtmnt_1_exhst_tmprtr_1_prlmnr_fm: 31,
        atttt_1_ds_ptt_ft_it_tpt_pf: 31,
        engine_exhaust_1_dew_point_detected: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1ig2_exhaust_temp_1,
        350.0,
        1.0,
        "exhaust_temp_1",
    );
    assert_float_near(
        state.aftertreatment.at1ig2_dpf_intake_temp,
        360.0,
        1.0,
        "dpf_intake_temp",
    );
}

#[test]
fn test_at1ig2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1IG2::BASE_CAN_ID));
    assert!(found, "AT1IG2 frame should be present in broadcasts");
}

// ============================================================================
// AT1HI1 - Aftertreatment 1 Historical Information 1
// ============================================================================

#[test]
fn test_at1hi1_handler() {
    let mut state = test_state();
    let msg = AT1HI1 {
        device_id: external_device(),
        aftertreatment_1_total_fuel_used: 1000.0,
        aftrtrtmnt_1_ttl_rgnrtn_tm: 72000,
        aftrtrtmnt_1_ttl_dsld_tm: 3600,
        aftrtrtmnt_1_ttl_nmr_of_atv_rgnrtns: 400,
        atttt_1_ds_ptt_ft_tt_pssv_rt_t: 144000,
        atttt_1_ds_ptt_ft_tt_no_pssv_rts: 1000,
        atttt_1_ds_ptt_ft_tt_no_atv_rt_it_rqsts: 20,
        atttt_1_ds_ptt_ft_tt_no_atv_rt_m_rqsts: 10,
        atttt_1_ds_ptt_ft_av_t_btw_atv_rts: 86400,
        atttt_1_ds_ptt_ft_av_dst_btw_atv_rts: 800.0,
        atttt_1_ds_ptt_ft_no_atv_rts: 300,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1hi1_total_fuel_used,
        1000.0,
        10.0,
        "total_fuel_used",
    );
    assert_eq!(state.aftertreatment.at1hi1_total_regen_time, 72000);
    assert_eq!(state.aftertreatment.at1hi1_total_disabled_time, 3600);
    assert_eq!(state.aftertreatment.at1hi1_total_num_active_regens, 400);
}

#[test]
fn test_at1hi1_broadcast_skipped_due_to_dlc() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1HI1::BASE_CAN_ID));
    assert!(
        !found,
        "AT1HI1 frame should NOT be in broadcasts (DLC=44 exceeds 8-byte CAN limit)"
    );
}

// ============================================================================
// AT1GP - Aftertreatment 1 Gas Pressure
// ============================================================================

#[test]
fn test_at1gp_handler() {
    let mut state = test_state();
    let msg = AT1GP {
        device_id: external_device(),
        atttt_1_ds_ptt_ft_it_pss: 120.0,
        atttt_1_ds_ptt_ft_ott_pss: 110.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1gp_dpf_intake_pressure,
        120.0,
        1.0,
        "dpf_intake_pressure",
    );
    assert_float_near(
        state.aftertreatment.at1gp_dpf_outlet_pressure,
        110.0,
        1.0,
        "dpf_outlet_pressure",
    );
}

#[test]
fn test_at1gp_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1GP::BASE_CAN_ID));
    assert!(found, "AT1GP frame should be present in broadcasts");
}

// ============================================================================
// AT1FC1 - Aftertreatment 1 Fuel Control 1
// ============================================================================

#[test]
fn test_at1fc1_handler() {
    let mut state = test_state();
    let msg = AT1FC1 {
        device_id: external_device(),
        aftertreatment_1_fuel_pressure_1: 500.0,
        aftertreatment_1_fuel_rate: 10.0,
        aftrtrtmnt_1_fl_prssr_1_cntrl: 50.0,
        aftrtrtmnt_1_fl_drn_attr: 1,
        aftertreatment_1_ignition: 1,
        aftrtrtmnt_1_rgnrtn_stts: 2,
        aftrtrtmnt_1_fl_enl_attr: 1,
        aftrtrtmnt_1_fl_injtr_1_htr_cntrl: 75.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1fc1_fuel_pressure_1,
        500.0,
        5.0,
        "fuel_pressure_1",
    );
    assert_float_near(
        state.aftertreatment.at1fc1_fuel_rate,
        10.0,
        1.0,
        "fuel_rate",
    );
    assert_eq!(state.aftertreatment.at1fc1_regen_status, 2);
}

#[test]
fn test_at1fc1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1FC1::BASE_CAN_ID));
    assert!(found, "AT1FC1 frame should be present in broadcasts");
}

// ============================================================================
// AT1FC2 - Aftertreatment 1 Fuel Control 2
// ============================================================================

#[test]
fn test_at1fc2_handler() {
    let mut state = test_state();
    let msg = AT1FC2 {
        device_id: external_device(),
        aftertreatment_1_fuel_pressure_2: 450.0,
        aftrtrtmnt_1_fl_pmp_rl_cntrl: 1,
        aftrtrtmnt_1_fl_flw_dvrtr_vlv_cntrl: 1,
        aftrtrtmnt_1_fl_prssr_2_cntrl: 60.0,
        aftrtrtmnt_1_hdrrn_dsr_intk_fl_tmprtr: 70.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1fc2_fuel_pressure_2,
        450.0,
        5.0,
        "fuel_pressure_2",
    );
    assert_float_near(
        state.aftertreatment.at1fc2_fuel_pressure_2_control,
        60.0,
        1.0,
        "fuel_pressure_2_control",
    );
    assert_float_near(
        state.aftertreatment.at1fc2_hc_doser_intake_fuel_temp,
        70.0,
        1.0,
        "hc_doser_intake_fuel_temp",
    );
}

#[test]
fn test_at1fc2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1FC2::BASE_CAN_ID));
    assert!(found, "AT1FC2 frame should be present in broadcasts");
}

// ============================================================================
// AT1AC1 - Aftertreatment 1 Air Control 1
// ============================================================================

#[test]
fn test_at1ac1_handler() {
    let mut state = test_state();
    let msg = AT1AC1 {
        device_id: external_device(),
        aftrtrtmnt_1_sppl_ar_prssr: 900.0,
        aftertreatment_1_purge_air_pressure: 700.0,
        aftrtrtmnt_1_ar_prssr_cntrl: 60.0,
        aftrtrtmnt_1_ar_prssr_attr_pstn: 55.0,
        aftertreatment_1_air_system_relay: 1,
        aftrtrtmnt_1_atmztn_ar_attr: 1,
        aftertreatment_1_purge_air_actuator: 1,
        aftrtrtmnt_1_ar_enl_attr: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.at1ac1_supply_air_pressure,
        900.0,
        10.0,
        "supply_air_pressure",
    );
    assert_float_near(
        state.aftertreatment.at1ac1_purge_air_pressure,
        700.0,
        10.0,
        "purge_air_pressure",
    );
    assert_float_near(
        state.aftertreatment.at1ac1_air_pressure_control,
        60.0,
        1.0,
        "air_pressure_control",
    );
}

#[test]
fn test_at1ac1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1AC1::BASE_CAN_ID));
    assert!(found, "AT1AC1 frame should be present in broadcasts");
}

// ============================================================================
// A1DOC1 - Aftertreatment 1 Diesel Oxidation Catalyst 1
// ============================================================================

#[test]
fn test_a1doc1_handler() {
    let mut state = test_state();
    let msg = A1DOC1 {
        device_id: external_device(),
        atttt_1_ds_oxdt_ctst_it_tpt: 320.0,
        atttt_1_ds_oxdt_ctst_ott_tpt: 340.0,
        atttt_1_ds_oxdt_ctst_dt_pss: 5.0,
        atttt_1_ds_oxdt_ctst_it_tpt_pf: 31,
        atttt_1_ds_oxdt_ctst_ott_tpt_pf: 31,
        atttt_1_ds_oxdt_ctst_dt_pss_pf: 31,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.a1doc1_intake_temp,
        320.0,
        1.0,
        "doc_intake_temp",
    );
    assert_float_near(
        state.aftertreatment.a1doc1_outlet_temp,
        340.0,
        1.0,
        "doc_outlet_temp",
    );
    assert_float_near(
        state.aftertreatment.a1doc1_delta_pressure,
        5.0,
        0.5,
        "doc_delta_pressure",
    );
}

#[test]
fn test_a1doc1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A1DOC1::BASE_CAN_ID));
    assert!(found, "A1DOC1 frame should be present in broadcasts");
}

// ============================================================================
// A1DOC2 - Aftertreatment 1 Diesel Oxidation Catalyst 2
// ============================================================================

#[test]
fn test_a1doc2_handler() {
    let mut state = test_state();
    let msg = A1DOC2 {
        device_id: external_device(),
        atttt_1_ds_oxdt_ctst_it_pss: 115.0,
        atttt_1_ds_oxdt_ctst_ott_pss: 108.0,
        atttt_1_d_it_t_dp_ott_dt_pss: 4.5,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.a1doc2_intake_pressure,
        115.0,
        1.0,
        "doc_intake_pressure",
    );
    assert_float_near(
        state.aftertreatment.a1doc2_outlet_pressure,
        108.0,
        1.0,
        "doc_outlet_pressure",
    );
    assert_float_near(
        state.aftertreatment.a1doc2_intake_to_dpf_outlet_delta,
        4.5,
        0.5,
        "doc_intake_to_dpf_outlet_delta",
    );
}

#[test]
fn test_a1doc2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A1DOC2::BASE_CAN_ID));
    assert!(found, "A1DOC2 frame should be present in broadcasts");
}

// ============================================================================
// A1SCRAI - Aftertreatment 1 SCR Ammonia Information
// ============================================================================

#[test]
fn test_a1scrai_handler() {
    let mut state = test_state();
    let msg = A1SCRAI {
        device_id: external_device(),
        aftertreatment_1_outlet_nh_3: 10.0,
        aftrtrtmnt_1_otlt_nh_3_snsr_prlmnr_fm: 31,
        aftrtrtmnt_1_otlt_nh_3_rdng_stl: 1,
        atttt_1_ott_n_3_gs_ss_pw_ir: 1,
        atttt_1_ott_n_3_gs_ss_at_tpt: 1,
        atttt_1_ott_n_3_gs_ss_ht_pf: 31,
        atttt_1_ott_n_3_gs_ss_ht_ct: 3,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.a1scrai_outlet_nh3,
        10.0,
        1.0,
        "outlet_nh3",
    );
}

#[test]
fn test_a1scrai_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A1SCRAI::BASE_CAN_ID));
    assert!(found, "A1SCRAI frame should be present in broadcasts");
}

// ============================================================================
// A1SCRSI1 - Aftertreatment 1 SCR Status Information 1
// ============================================================================

#[test]
fn test_a1scrsi1_handler() {
    let mut state = test_state();
    let msg = A1SCRSI1 {
        device_id: external_device(),
        atttt_1_ds_exst_fd_av_cspt: 3.0,
        atttt_1_s_cdd_ds_exst_fd_cspt: 3.5,
        aftrtrtmnt_1_sr_cnvrsn_effn: 95.0,
        atttt_s_opt_idt_atv_tvd_dst: 100,
        aftrtrtmnt_1_sr_sstm_slftn_lvl: 10,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.a1scrsi1_def_avg_consumption,
        3.0,
        0.5,
        "def_avg_consumption",
    );
    assert_float_near(
        state.aftertreatment.a1scrsi1_scr_commanded_def_consumption,
        3.5,
        0.5,
        "scr_commanded_def_consumption",
    );
    assert_float_near(
        state.aftertreatment.a1scrsi1_scr_conversion_efficiency,
        95.0,
        1.0,
        "scr_conversion_efficiency",
    );
}

#[test]
fn test_a1scrsi1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A1SCRSI1::BASE_CAN_ID));
    assert!(found, "A1SCRSI1 frame should be present in broadcasts");
}

// ============================================================================
// A1SCRSI2 - Aftertreatment 1 SCR Status Information 2
// ============================================================================

#[test]
fn test_a1scrsi2_handler() {
    let mut state = test_state();
    let msg = A1SCRSI2 {
        device_id: external_device(),
        aftrtrtmnt_1_ttl_dsl_exhst_fld_usd: 3000.0,
        aftrtrtmnt_trp_dsl_exhst_fld: 15.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.a1scrsi2_total_def_used,
        3000.0,
        50.0,
        "total_def_used",
    );
    assert_float_near(
        state.aftertreatment.a1scrsi2_trip_def_used,
        15.0,
        1.0,
        "trip_def_used",
    );
}

#[test]
fn test_a1scrsi2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A1SCRSI2::BASE_CAN_ID));
    assert!(found, "A1SCRSI2 frame should be present in broadcasts");
}

// ============================================================================
// DPF1S - Diesel Particulate Filter 1 Soot Status
// ============================================================================

#[test]
fn test_dpf1s_handler() {
    let mut state = test_state();
    let msg = DPF1S {
        device_id: external_device(),
        aftrtrtmnt_1_dsl_prtlt_fltr_st_mss: 20.0,
        aftrtrtmnt_1_dsl_prtlt_fltr_st_dnst: 2.0,
        aftrtrtmnt_1_dsl_prtlt_fltr_mn_st_sgnl: 30.0,
        atttt_1_ds_ptt_ft_md_st_s: 28.0,
        atttt_1_ds_ptt_ft_st_ss_pf: 31,
        ds_ptt_ft_1_st_ss_e_it_tpt: 70.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.dpf1s_soot_mass,
        20.0,
        1.0,
        "soot_mass",
    );
    assert_float_near(
        state.aftertreatment.dpf1s_soot_density,
        2.0,
        0.5,
        "soot_density",
    );
    assert_float_near(
        state.aftertreatment.dpf1s_mean_soot_signal,
        30.0,
        1.0,
        "mean_soot_signal",
    );
}

#[test]
fn test_dpf1s_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, DPF1S::BASE_CAN_ID));
    assert!(found, "DPF1S frame should be present in broadcasts");
}

// ============================================================================
// DPF1S2 - Diesel Particulate Filter 1 Soot Status 2
// ============================================================================

#[test]
fn test_dpf1s2_handler() {
    let mut state = test_state();
    let msg = DPF1S2 {
        device_id: external_device(),
        atttt_1_ds_ptt_ft_st_s_stdd_dvt: 3.0,
        atttt_1_ds_ptt_ft_st_s_mx: 40.0,
        atttt_1_ds_ptt_ft_st_sm: 15.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.dpf1s2_soot_signal_std_dev,
        3.0,
        0.5,
        "soot_signal_std_dev",
    );
    assert_float_near(
        state.aftertreatment.dpf1s2_soot_signal_max,
        40.0,
        1.0,
        "soot_signal_max",
    );
    assert_float_near(
        state.aftertreatment.dpf1s2_soot_signal_min,
        15.0,
        1.0,
        "soot_signal_min",
    );
}

#[test]
fn test_dpf1s2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, DPF1S2::BASE_CAN_ID));
    assert!(found, "DPF1S2 frame should be present in broadcasts");
}

// ============================================================================
// DPFC1 - Diesel Particulate Filter Control 1
// ============================================================================

#[test]
fn test_dpfc1_handler() {
    let mut state = test_state();
    let msg = DPFC1 {
        device_id: external_device(),
        dsl_prtlt_fltr_lmp_cmmnd: 2,
        dsl_prtlt_fltr_atv_rgnrtn_avllt_stts: 1,
        atttt_ds_ptt_ft_pssv_rt_stts: 1,
        atttt_ds_ptt_ft_atv_rt_stts: 2,
        aftrtrtmnt_dsl_prtlt_fltr_stts: 3,
        dsl_prtlt_fltr_atv_rgnrtn_inhtd_stts: 0,
        ds_ptt_ft_atv_rt_itd_dt_it_swt: 0,
        ds_ptt_ft_atv_rt_itd_dt_ct_dsd: 0,
        ds_ptt_ft_atv_rt_itd_dt_sv_b_atv: 0,
        ds_ptt_ft_atv_rt_itd_dt_pt_atv: 0,
        ds_ptt_ft_atv_rt_itd_dt_at_pd_o_id: 0,
        ds_ptt_ft_atv_rt_itd_dt_ot_o_nt: 0,
        ds_ptt_ft_atv_rt_itd_dtv_spd_av_awd_spd: 0,
        ds_ptt_ft_atv_rt_itd_dtpb_nt_st: 0,
        ds_ptt_ft_atv_rt_itd_dt_lw_exst_tpt: 0,
        ds_ptt_ft_atv_rt_itd_dt_sst_ft_atv: 0,
        ds_ptt_ft_atv_rt_itd_dt_sst_tt: 0,
        ds_ptt_ft_atv_rt_itd_dt_tp_sst_lt: 0,
        ds_ptt_ft_atv_rt_itd_dt_pt_sst_lt: 0,
        ds_ptt_ft_atv_rt_itd_dte_nt_wd_up: 0,
        ds_ptt_ft_atv_rt_itd_dtv_spd_bw_awd_spd: 0,
        ds_ptt_ft_att_atv_rt_itt_ct: 1,
        exhst_sstm_hgh_tmprtr_lmp_cmmnd: 0,
        dsl_prtlt_fltr_atv_rgnrtn_frd_stts: 0,
        hydrocarbon_doser_purging_enable: 0,
        ds_ptt_ft_atv_rt_itd_dt_lw_exst_pss: 0,
        atttt_1_ds_ptt_ft_cdts_nt_mt_f_atv_rt: 0,
        ds_ptt_ft_atv_rt_itd_dt_ts: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.aftertreatment.dpfc1_dpf_lamp_command, 2);
    assert_eq!(state.aftertreatment.dpfc1_dpf_active_regen_status, 2);
    assert_eq!(state.aftertreatment.dpfc1_dpf_passive_regen_status, 1);
    assert_eq!(state.aftertreatment.dpfc1_dpf_status, 3);
}

#[test]
fn test_dpfc1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, DPFC1::BASE_CAN_ID));
    assert!(found, "DPFC1 frame should be present in broadcasts");
}

// ============================================================================
// DPFC2 - Diesel Particulate Filter Control 2
// ============================================================================

#[test]
fn test_dpfc2_handler() {
    let mut state = test_state();
    let msg = DPFC2 {
        device_id: external_device(),
        atttt_1_ds_ptt_ft_it_tpt_st_pt: 600.0,
        engine_unburned_fuel_percentage: 3.0,
        aftertreatment_1_fuel_mass_rate: 5.0,
        aftertreatment_2_fuel_mass_rate: 4.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.aftertreatment.dpfc2_dpf_intake_temp_setpoint,
        600.0,
        5.0,
        "dpf_intake_temp_setpoint",
    );
    assert_float_near(
        state.aftertreatment.dpfc2_engine_unburned_fuel_pct,
        3.0,
        0.5,
        "engine_unburned_fuel_pct",
    );
    assert_float_near(
        state.aftertreatment.dpfc2_at1_fuel_mass_rate,
        5.0,
        0.5,
        "at1_fuel_mass_rate",
    );
}

#[test]
fn test_dpfc2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, DPFC2::BASE_CAN_ID));
    assert!(found, "DPFC2 frame should be present in broadcasts");
}

// ============================================================================
// Round-trip Tests (Set state -> Broadcast -> Decode -> Verify)
// ============================================================================

#[test]
fn test_at1s1_roundtrip() {
    let mut state = test_state();
    state.aftertreatment.at1s1_dpf_soot_load_percent = 40;
    state.aftertreatment.at1s1_dpf_ash_load_percent = 12;
    state.aftertreatment.at1s1_dpf_time_since_last_regen = 5400;
    state.aftertreatment.at1s1_dpf_soot_load_regen_threshold = 85.0;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1S1::BASE_CAN_ID))
        .expect("AT1S1 frame should exist");

    let decoded = AT1S1::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_eq!(decoded.aftrtrtmnt_1_dsl_prtlt_fltr_st_ld_prnt, 40);
    assert_eq!(decoded.atttt_1_ds_ptt_ft_as_ld_pt, 12);
    assert_eq!(decoded.atttt_1_ds_ptt_ft_ts_lst_atv_rt, 5400);
    assert_float_near(
        decoded.atttt_1_ds_ptt_ft_st_ld_rt_tsd,
        85.0,
        1.0,
        "roundtrip soot_load_regen_threshold",
    );
}

#[test]
fn test_at1og1_roundtrip() {
    let mut state = test_state();
    state.aftertreatment.at1og1_outlet_nox = 35.0;
    state.aftertreatment.at1og1_outlet_oxygen = 11.0;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1OG1::BASE_CAN_ID))
        .expect("AT1OG1 frame should exist");

    let decoded = AT1OG1::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_float_near(
        decoded.aftertreatment_1_outlet_nox_1,
        35.0,
        1.0,
        "roundtrip outlet_nox",
    );
    assert_float_near(
        decoded.aftrtrtmnt_1_otlt_prnt_oxgn_1,
        11.0,
        1.0,
        "roundtrip outlet_oxygen",
    );
}

#[test]
fn test_at1fc1_roundtrip() {
    let mut state = test_state();
    state.aftertreatment.at1fc1_fuel_pressure_1 = 450.0;
    state.aftertreatment.at1fc1_fuel_rate = 8.0;
    state.aftertreatment.at1fc1_regen_status = 1;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT1FC1::BASE_CAN_ID))
        .expect("AT1FC1 frame should exist");

    let decoded = AT1FC1::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_float_near(
        decoded.aftertreatment_1_fuel_pressure_1,
        450.0,
        5.0,
        "roundtrip fuel_pressure_1",
    );
    assert_float_near(
        decoded.aftertreatment_1_fuel_rate,
        8.0,
        1.0,
        "roundtrip fuel_rate",
    );
    assert_eq!(decoded.aftrtrtmnt_1_rgnrtn_stts, 1);
}

#[test]
fn test_dpf1s_roundtrip() {
    let mut state = test_state();
    state.aftertreatment.dpf1s_soot_mass = 20.0;
    state.aftertreatment.dpf1s_soot_density = 1.6;
    state.aftertreatment.dpf1s_mean_soot_signal = 28.0;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, DPF1S::BASE_CAN_ID))
        .expect("DPF1S frame should exist");

    let decoded = DPF1S::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_float_near(
        decoded.aftrtrtmnt_1_dsl_prtlt_fltr_st_mss,
        20.0,
        1.0,
        "roundtrip soot_mass",
    );
    assert_float_near(
        decoded.aftrtrtmnt_1_dsl_prtlt_fltr_st_dnst,
        1.6,
        0.5,
        "roundtrip soot_density",
    );
    assert_float_near(
        decoded.aftrtrtmnt_1_dsl_prtlt_fltr_mn_st_sgnl,
        28.0,
        1.0,
        "roundtrip mean_soot_signal",
    );
}

#[test]
fn test_dpfc1_roundtrip() {
    let mut state = test_state();
    state.aftertreatment.dpfc1_dpf_lamp_command = 1;
    state.aftertreatment.dpfc1_dpf_active_regen_status = 2;
    state.aftertreatment.dpfc1_dpf_status = 3;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, DPFC1::BASE_CAN_ID))
        .expect("DPFC1 frame should exist");

    let decoded = DPFC1::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_eq!(decoded.dsl_prtlt_fltr_lmp_cmmnd, 1);
    assert_eq!(decoded.atttt_ds_ptt_ft_atv_rt_stts, 2);
    assert_eq!(decoded.aftrtrtmnt_dsl_prtlt_fltr_stts, 3);
}

// ============================================================================
// Self-reception Test
// ============================================================================

#[test]
fn test_at1s1_self_reception_ignored() {
    let mut state = test_state();
    let msg = AT1S1 {
        device_id: DeviceId::from(0x82),
        aftrtrtmnt_1_dsl_prtlt_fltr_st_ld_prnt: 50,
        atttt_1_ds_ptt_ft_as_ld_pt: 15,
        atttt_1_ds_ptt_ft_ts_lst_atv_rt: 3600,
        atttt_1_ds_ptt_ft_st_ld_rt_tsd: 90.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Ignored);
}

// ============================================================================
// DecodeFailed Test
// ============================================================================

#[test]
fn test_batch6_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = AT1S1::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
