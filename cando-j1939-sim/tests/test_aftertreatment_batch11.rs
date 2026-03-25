//! Batch 11: Aftertreatment Bank 2 + EGR Messages Tests
//!
//! Tests for 20 aftertreatment bank 2 and EGR messages:
//! AT2S1, AT2S2, AT2OG1, AT2IG1, AT2HI1, AT2GP, AT2FC1, AT2AC1,
//! A2DOC1, A2SCRAI, A2SCRSI1, A1SCRDSI1, A1SCRDSI2, A1SCRDSI3,
//! A2SCRDSI1, A2SCRDSI2, A2SCRDSI3, EEGR1A, EEGR2A, DPF2S

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
// AT2S1 - Aftertreatment 2 DPF Soot Status 1
// ============================================================================

#[test]
fn test_at2s1_handler_updates_state() {
    let mut state = test_state();
    let msg = AT2S1 {
        device_id: external_device(),
        aftrtrtmnt_2_dsl_prtlt_fltr_st_ld_prnt: 45,
        atttt_2_ds_ptt_ft_as_ld_pt: 12,
        atttt_2_ds_ptt_ft_ts_lst_atv_rt: 7200,
        atttt_2_ds_ptt_ft_st_ld_rt_tsd: 90.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.aftertreatment.at2s1_dpf_soot_load_percent, 45);
    assert_eq!(state.aftertreatment.at2s1_dpf_ash_load_percent, 12);
    assert_eq!(state.aftertreatment.at2s1_dpf_time_since_last_regen, 7200);
    assert_float_near(state.aftertreatment.at2s1_dpf_soot_load_regen_threshold, 90.0, 1.0, "regen_threshold");
}

#[test]
fn test_at2s1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT2S1::BASE_CAN_ID));
    assert!(found, "AT2S1 broadcast frame should be present");
}

// ============================================================================
// AT2S2 - Aftertreatment 2 DPF Status 2
// ============================================================================

#[test]
fn test_at2s2_handler_updates_state() {
    let mut state = test_state();
    let msg = AT2S2 {
        device_id: external_device(),
        atttt_2_ds_ptt_ft_tt_nxt_atv_rt: 14400,
        atttt_2_s_sst_ts_lst_sst_c_evt: 172800,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.aftertreatment.at2s2_dpf_time_to_next_regen, 14400);
    assert_eq!(state.aftertreatment.at2s2_scr_time_since_last_clean, 172800);
}

#[test]
fn test_at2s2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT2S2::BASE_CAN_ID));
    assert!(found, "AT2S2 broadcast frame should be present");
}

// ============================================================================
// AT2OG1 - Aftertreatment 2 Outlet Gas Sensor 1
// ============================================================================

#[test]
fn test_at2og1_handler_updates_state() {
    let mut state = test_state();
    let msg = AT2OG1 {
        device_id: external_device(),
        aftertreatment_2_outlet_nox_1: 35.0,
        aftrtrtmnt_2_otlt_prnt_oxgn_1: 12.0,
        aftrtrtmnt_2_otlt_gs_snsr_1_pwr_in_rng: 1,
        aftrtrtmnt_2_otlt_gs_snsr_1_at_tmprtr: 1,
        aftrtrtmnt_2_otlt_nx_1_rdng_stl: 1,
        atttt_2_ott_wd_r_pt_ox_1_rd_st: 1,
        atttt_2_ott_gs_ss_1_ht_pf: 0,
        aftrtrtmnt_2_otlt_gs_snsr_1_htr_cntrl: 3,
        aftrtrtmnt_2_otlt_nx_snsr_1_prlmnr_fm: 0,
        atttt_2_ott_nx_ss_1_s_dss_stts: 0,
        atttt_2_ott_ox_ss_1_pf: 0,
        aftrtrtmnt_2_otlt_2_gs_snsr_pwr_sppl: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.at2og1_outlet_nox, 35.0, 1.0, "outlet_nox");
    assert_float_near(state.aftertreatment.at2og1_outlet_percent_oxygen, 12.0, 1.0, "outlet_oxygen");
}

#[test]
fn test_at2og1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT2OG1::BASE_CAN_ID));
    assert!(found, "AT2OG1 broadcast frame should be present");
}

// ============================================================================
// AT2IG1 - Aftertreatment 2 Inlet Gas Sensor 1
// ============================================================================

#[test]
fn test_at2ig1_handler_updates_state() {
    let mut state = test_state();
    let msg = AT2IG1 {
        device_id: external_device(),
        engine_exhaust_2_nox_1: 350.0,
        engine_exhaust_2_percent_oxygen_1: 15.0,
        engn_exhst_2_gs_snsr_1_pwr_in_rng: 1,
        engn_exhst_2_gs_snsr_1_at_tmprtr: 1,
        engine_exhaust_2_nox_1_reading_stable: 1,
        engn_exhst_2_wd_rng_prnt_oxgn_1_rdng_stl: 1,
        engn_exhst_2_gs_snsr_1_htr_prlmnr_fm: 0,
        engn_exhst_2_gs_snsr_1_htr_cntrl: 3,
        engn_exhst_2_nx_snsr_1_prlmnr_fm: 0,
        engn_exhst_2_nx_snsr_1_slf_dgnss_stts: 0,
        engn_exhst_2_oxgn_snsr_1_prlmnr_fm: 0,
        engn_exhst_2_gs_snsr_2_pwr_sppl: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.at2ig1_inlet_nox, 350.0, 1.0, "inlet_nox");
    assert_float_near(state.aftertreatment.at2ig1_inlet_percent_oxygen, 15.0, 1.0, "inlet_oxygen");
}

#[test]
fn test_at2ig1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT2IG1::BASE_CAN_ID));
    assert!(found, "AT2IG1 broadcast frame should be present");
}

// ============================================================================
// AT2HI1 - Aftertreatment 2 Historical Info 1
// ============================================================================

#[test]
fn test_at2hi1_handler_updates_state() {
    let mut state = test_state();
    let msg = AT2HI1 {
        device_id: external_device(),
        aftertreatment_2_total_fuel_used: 2500.0,
        aftrtrtmnt_2_ttl_rgnrtn_tm: 72000,
        aftrtrtmnt_2_ttl_dsld_tm: 3600,
        aftrtrtmnt_2_ttl_nmr_of_atv_rgnrtns: 1000,
        atttt_2_ds_ptt_ft_tt_pssv_rt_t: 18000,
        atttt_2_ds_ptt_ft_tt_no_pssv_rts: 200,
        atttt_2_ds_ptt_ft_tt_no_atv_rt_it_rqsts: 10,
        atttt_2_ds_ptt_ft_tt_no_atv_rt_m_rqsts: 5,
        atttt_2_ds_ptt_ft_av_t_btw_atv_rts: 14400,
        atttt_2_ds_ptt_ft_av_dst_btw_atv_rts: 500.0,
        atttt_2_ds_ptt_ft_no_atv_rts: 50,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.at2hi1_total_fuel_used, 2500.0, 1.0, "total_fuel_used");
    assert_eq!(state.aftertreatment.at2hi1_total_regen_time, 72000);
    assert_eq!(state.aftertreatment.at2hi1_total_active_regens, 1000);
}

#[test]
fn test_at2hi1_is_multiframe_no_broadcast() {
    // AT2HI1 has DLC=44 (multi-frame transport protocol message)
    // Cannot fit in a single 8-byte CAN frame, so no broadcast is generated.
    // Handler still processes incoming AT2HI1 messages.
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT2HI1::BASE_CAN_ID));
    assert!(!found, "AT2HI1 should NOT be in broadcasts (DLC=44, requires TP)");
}

// ============================================================================
// AT2GP - Aftertreatment 2 Gas Pressures
// ============================================================================

#[test]
fn test_at2gp_handler_updates_state() {
    let mut state = test_state();
    let msg = AT2GP {
        device_id: external_device(),
        atttt_2_ds_ptt_ft_it_pss: 150.0,
        atttt_2_ds_ptt_ft_ott_pss: 110.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.at2gp_dpf_intake_pressure, 150.0, 1.0, "dpf_intake_pressure");
    assert_float_near(state.aftertreatment.at2gp_dpf_outlet_pressure, 110.0, 1.0, "dpf_outlet_pressure");
}

#[test]
fn test_at2gp_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT2GP::BASE_CAN_ID));
    assert!(found, "AT2GP broadcast frame should be present");
}

// ============================================================================
// AT2FC1 - Aftertreatment 2 Fuel Control 1
// ============================================================================

#[test]
fn test_at2fc1_handler_updates_state() {
    let mut state = test_state();
    let msg = AT2FC1 {
        device_id: external_device(),
        aftertreatment_2_fuel_pressure_1: 400.0,
        aftertreatment_2_fuel_rate: 8.0,
        aftrtrtmnt_2_fl_prssr_1_cntrl: 55.0,
        aftrtrtmnt_2_fl_drn_attr: 0,
        aftertreatment_2_ignition: 0,
        aftrtrtmnt_2_rgnrtn_stts: 1,
        aftrtrtmnt_2_fl_enl_attr: 1,
        aftrtrtmnt_2_fl_injtr_1_htr_cntrl: 0.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.at2fc1_fuel_pressure, 400.0, 1.0, "fuel_pressure");
    assert_float_near(state.aftertreatment.at2fc1_fuel_rate, 8.0, 1.0, "fuel_rate");
    assert_eq!(state.aftertreatment.at2fc1_regen_status, 1);
}

#[test]
fn test_at2fc1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT2FC1::BASE_CAN_ID));
    assert!(found, "AT2FC1 broadcast frame should be present");
}

// ============================================================================
// AT2AC1 - Aftertreatment 2 Air Control 1
// ============================================================================

#[test]
fn test_at2ac1_handler_updates_state() {
    let mut state = test_state();
    let msg = AT2AC1 {
        device_id: external_device(),
        aftrtrtmnt_2_sppl_ar_prssr: 600.0,
        aftertreatment_2_purge_air_pressure: 250.0,
        aftrtrtmnt_2_ar_prssr_cntrl: 40.0,
        aftrtrtmnt_2_ar_prssr_attr_pstn: 35.0,
        aftertreatment_2_air_system_relay: 1,
        aftrtrtmnt_2_atmztn_ar_attr: 0,
        aftertreatment_2_purge_air_actuator: 0,
        aftrtrtmnt_2_ar_enl_attr: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.at2ac1_supply_air_pressure, 600.0, 1.0, "supply_air_pressure");
    assert_float_near(state.aftertreatment.at2ac1_purge_air_pressure, 250.0, 1.0, "purge_air_pressure");
}

#[test]
fn test_at2ac1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT2AC1::BASE_CAN_ID));
    assert!(found, "AT2AC1 broadcast frame should be present");
}

// ============================================================================
// A2DOC1 - Aftertreatment 2 Diesel Oxidation Catalyst 1
// ============================================================================

#[test]
fn test_a2doc1_handler_updates_state() {
    let mut state = test_state();
    let msg = A2DOC1 {
        device_id: external_device(),
        atttt_2_ds_oxdt_ctst_it_tpt: 300.0,
        atttt_2_ds_oxdt_ctst_ott_tpt: 340.0,
        atttt_2_ds_oxdt_ctst_dt_pss: 5.0,
        atttt_2_ds_oxdt_ctst_it_tpt_pf: 0,
        atttt_2_ds_oxdt_ctst_ott_tpt_pf: 0,
        atttt_2_ds_oxdt_ctst_dt_pss_pf: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.a2doc1_inlet_temp, 300.0, 1.0, "doc_inlet_temp");
    assert_float_near(state.aftertreatment.a2doc1_outlet_temp, 340.0, 1.0, "doc_outlet_temp");
    assert_float_near(state.aftertreatment.a2doc1_diff_pressure, 5.0, 1.0, "doc_diff_pressure");
}

#[test]
fn test_a2doc1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A2DOC1::BASE_CAN_ID));
    assert!(found, "A2DOC1 broadcast frame should be present");
}

// ============================================================================
// A2SCRAI - Aftertreatment 2 SCR Ammonia Info
// ============================================================================

#[test]
fn test_a2scrai_handler_updates_state() {
    let mut state = test_state();
    let msg = A2SCRAI {
        device_id: external_device(),
        aftertreatment_2_outlet_nh_3: 8.0,
        aftrtrtmnt_2_otlt_nh_3_snsr_prlmnr_fm: 0,
        aftrtrtmnt_2_otlt_nh_3_rdng_stl: 1,
        atttt_2_ott_n_3_gs_ss_pw_ir: 1,
        atttt_2_ott_n_3_gs_ss_at_tpt: 1,
        atttt_2_ott_n_3_gs_ss_ht_pf: 0,
        atttt_2_ott_n_3_gs_ss_ht_ct: 3,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.a2scrai_outlet_nh3, 8.0, 1.0, "outlet_nh3");
}

#[test]
fn test_a2scrai_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A2SCRAI::BASE_CAN_ID));
    assert!(found, "A2SCRAI broadcast frame should be present");
}

// ============================================================================
// A2SCRSI1 - Aftertreatment 2 SCR Status Info 1
// ============================================================================

#[test]
fn test_a2scrsi1_handler_updates_state() {
    let mut state = test_state();
    let msg = A2SCRSI1 {
        device_id: external_device(),
        atttt_2_ds_exst_fd_av_cspt: 3.0,
        atttt_2_s_cdd_ds_exst_fd_cspt: 3.5,
        aftrtrtmnt_2_sr_cnvrsn_effn: 92.0,
        aftrtrtmnt_2_sr_sstm_slftn_lvl: 8,
        atttt_2_ds_exst_fd_usd_ts_opt_c: 15.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.a2scrsi1_def_avg_consumption, 3.0, 1.0, "def_avg_consumption");
    assert_float_near(state.aftertreatment.a2scrsi1_scr_conversion_efficiency, 92.0, 1.0, "scr_efficiency");
}

#[test]
fn test_a2scrsi1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A2SCRSI1::BASE_CAN_ID));
    assert!(found, "A2SCRSI1 broadcast frame should be present");
}

// ============================================================================
// A1SCRDSI1 - Aftertreatment 1 SCR Dosing System Info 1
// ============================================================================

#[test]
fn test_a1scrdsi1_handler_updates_state() {
    let mut state = test_state();
    let msg = A1SCRDSI1 {
        device_id: external_device(),
        atttt_1_ds_exst_fd_at_ds_qtt: 750.0,
        aftertreatment_1_scr_system_1_state: 2,
        aftertreatment_1_scr_system_2_state: 0,
        atttt_1_ds_exst_fd_at_qtt_o_itt: 120.0,
        atttt_1_ds_exst_fd_ds_1_ast_pss: 900.0,
        atttt_1_ds_exst_fd_at_ds_qtt_hr: 0.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.a1scrdsi1_dosing_rate, 750.0, 5.0, "dosing_rate");
    assert_eq!(state.aftertreatment.a1scrdsi1_scr_system_1_state, 2);
}

#[test]
fn test_a1scrdsi1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A1SCRDSI1::BASE_CAN_ID));
    assert!(found, "A1SCRDSI1 broadcast frame should be present");
}

// ============================================================================
// A1SCRDSI2 - Aftertreatment 1 SCR Dosing System Info 2
// ============================================================================

#[test]
fn test_a1scrdsi2_handler_updates_state() {
    let mut state = test_state();
    let msg = A1SCRDSI2 {
        device_id: external_device(),
        atttt_1_s_ds_a_assst_ast_pss: 700.0,
        aftrtrtmnt_1_sr_dsng_ar_assst_vlv: 60.0,
        atttt_1_ds_exst_fd_ds_1_tpt: 70.0,
        atttt_1_s_ds_vv_exst_tpt_rdt_rqst: 0,
        aftrtrtmnt_1_sr_fdk_cntrl_stts: 1,
        aftrtrtmnt_1_dsl_exhst_fld_ln_htr_1_stt: 0,
        atttt_1_ds_exst_fd_l_ht_1_pf: 0,
        aftrtrtmnt_1_dsl_exhst_fld_ln_htr_2_stt: 0,
        atttt_1_ds_exst_fd_l_ht_2_pf: 0,
        aftrtrtmnt_1_dsl_exhst_fld_ln_htr_3_stt: 0,
        atttt_1_ds_exst_fd_l_ht_3_pf: 0,
        aftrtrtmnt_1_dsl_exhst_fld_ln_htr_4_stt: 0,
        atttt_1_ds_exst_fd_l_ht_4_pf: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.a1scrdsi2_air_assist_pressure, 700.0, 5.0, "air_assist_pressure");
    assert_float_near(state.aftertreatment.a1scrdsi2_air_assist_valve, 60.0, 1.0, "air_assist_valve");
}

#[test]
fn test_a1scrdsi2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A1SCRDSI2::BASE_CAN_ID));
    assert!(found, "A1SCRDSI2 broadcast frame should be present");
}

// ============================================================================
// A1SCRDSI3 - Aftertreatment 1 SCR Dosing System Info 3
// ============================================================================

#[test]
fn test_a1scrdsi3_handler_updates_state() {
    let mut state = test_state();
    let msg = A1SCRDSI3 {
        device_id: external_device(),
        aftrtrtmnt_1_dsl_exhst_fld_dsr_1_prssr: 500.0,
        atttt_1_ds_exst_fd_ds_2_ast_pss: 850.0,
        atttt_1_ds_exst_fd_ds_2_tpt: 68.0,
        aftrtrtmnt_1_dsl_exhst_fld_dsr_2_prssr: 400.0,
        atttt_1_ds_exst_fd_ds_1_pss_extdd_r: 500.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.a1scrdsi3_doser_1_pressure, 500.0, 10.0, "doser_1_pressure");
    assert_float_near(state.aftertreatment.a1scrdsi3_doser_2_abs_pressure, 850.0, 10.0, "doser_2_abs_pressure");
}

#[test]
fn test_a1scrdsi3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A1SCRDSI3::BASE_CAN_ID));
    assert!(found, "A1SCRDSI3 broadcast frame should be present");
}

// ============================================================================
// A2SCRDSI1 - Aftertreatment 2 SCR Dosing System Info 1
// ============================================================================

#[test]
fn test_a2scrdsi1_handler_updates_state() {
    let mut state = test_state();
    let msg = A2SCRDSI1 {
        device_id: external_device(),
        atttt_2_ds_exst_fd_at_ds_qtt: 600.0,
        aftertreatment_2_scr_system_1_state: 2,
        aftertreatment_2_scr_system_2_state: 0,
        atttt_2_ds_exst_fd_at_qtt_o_itt: 90.0,
        atttt_2_ds_exst_fd_ds_1_ast_pss: 820.0,
        atttt_2_ds_exst_fd_at_ds_qtt_hr: 0.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.a2scrdsi1_dosing_rate, 600.0, 5.0, "dosing_rate");
    assert_eq!(state.aftertreatment.a2scrdsi1_scr_system_1_state, 2);
}

#[test]
fn test_a2scrdsi1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A2SCRDSI1::BASE_CAN_ID));
    assert!(found, "A2SCRDSI1 broadcast frame should be present");
}

// ============================================================================
// A2SCRDSI2 - Aftertreatment 2 SCR Dosing System Info 2
// ============================================================================

#[test]
fn test_a2scrdsi2_handler_updates_state() {
    let mut state = test_state();
    let msg = A2SCRDSI2 {
        device_id: external_device(),
        atttt_2_s_ds_a_assst_ast_pss: 650.0,
        aftrtrtmnt_2_sr_dsng_ar_assst_vlv: 55.0,
        atttt_2_ds_exst_fd_ds_1_tpt: 68.0,
        atttt_2_s_ds_vv_exst_tpt_rdt_rqst: 0,
        aftrtrtmnt_2_sr_fdk_cntrl_stts: 1,
        aftrtrtmnt_2_dsl_exhst_fld_ln_htr_1_stt: 0,
        atttt_2_ds_exst_fd_l_ht_1_pf: 0,
        aftrtrtmnt_2_dsl_exhst_fld_ln_htr_2_stt: 0,
        atttt_2_ds_exst_fd_l_ht_2_pf: 0,
        aftrtrtmnt_2_dsl_exhst_fld_ln_htr_3_stt: 0,
        atttt_2_ds_exst_fd_l_ht_3_pf: 0,
        aftrtrtmnt_2_dsl_exhst_fld_ln_htr_4_stt: 0,
        atttt_2_ds_exst_fd_l_ht_4_pf: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.a2scrdsi2_air_assist_pressure, 650.0, 5.0, "air_assist_pressure");
    assert_float_near(state.aftertreatment.a2scrdsi2_air_assist_valve, 55.0, 1.0, "air_assist_valve");
}

#[test]
fn test_a2scrdsi2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A2SCRDSI2::BASE_CAN_ID));
    assert!(found, "A2SCRDSI2 broadcast frame should be present");
}

// ============================================================================
// A2SCRDSI3 - Aftertreatment 2 SCR Dosing System Info 3
// ============================================================================

#[test]
fn test_a2scrdsi3_handler_updates_state() {
    let mut state = test_state();
    let msg = A2SCRDSI3 {
        device_id: external_device(),
        aftrtrtmnt_2_dsl_exhst_fld_dsr_1_prssr: 450.0,
        atttt_2_ds_exst_fd_ds_2_ast_pss: 780.0,
        atttt_2_ds_exst_fd_ds_2_tpt: 62.0,
        aftrtrtmnt_2_dsl_exhst_fld_dsr_2_prssr: 380.0,
        atttt_2_ds_exst_fd_ds_1_pss_extdd_r: 450.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.a2scrdsi3_doser_1_pressure, 450.0, 10.0, "doser_1_pressure");
    assert_float_near(state.aftertreatment.a2scrdsi3_doser_2_abs_pressure, 780.0, 10.0, "doser_2_abs_pressure");
}

#[test]
fn test_a2scrdsi3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, A2SCRDSI3::BASE_CAN_ID));
    assert!(found, "A2SCRDSI3 broadcast frame should be present");
}

// ============================================================================
// EEGR1A - Engine EGR 1 Actuator
// ============================================================================

#[test]
fn test_eegr1a_handler_updates_state() {
    let mut state = test_state();
    let msg = EEGR1A {
        device_id: external_device(),
        engn_exhst_gs_rrltn_1_attr_1_prlmnr_fm: 0,
        e_exst_gs_rt_1_att_1_tpt_stts: 0,
        engn_exhst_gs_rrltn_1_attr_1_tmprtr: 80.0,
        engn_exhst_gs_rrltn_1_attr_1_dsrd_pstn: 45.0,
        engn_exhst_gs_rrltn_1_attr_2_prlmnr_fm: 0,
        e_exst_gs_rt_1_att_2_tpt_stts: 0,
        engn_exhst_gs_rrltn_1_attr_2_tmprtr: 78.0,
        engn_exhst_gs_rrltn_1_attr_2_dsrd_pstn: 40.0,
        engn_exhst_gs_rrltn_1_attr_1_oprtn_stts: 0,
        engn_exhst_gs_rrltn_1_attr_2_oprtn_stts: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.eegr1a_actuator_1_desired_position, 45.0, 1.0, "egr1_act1_position");
    assert_float_near(state.aftertreatment.eegr1a_actuator_1_temp, 80.0, 1.0, "egr1_act1_temp");
    assert_float_near(state.aftertreatment.eegr1a_actuator_2_desired_position, 40.0, 1.0, "egr1_act2_position");
}

#[test]
fn test_eegr1a_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EEGR1A::BASE_CAN_ID));
    assert!(found, "EEGR1A broadcast frame should be present");
}

// ============================================================================
// EEGR2A - Engine EGR 2 Actuator
// ============================================================================

#[test]
fn test_eegr2a_handler_updates_state() {
    let mut state = test_state();
    let msg = EEGR2A {
        device_id: external_device(),
        engn_exhst_gs_rrltn_2_attr_1_prlmnr_fm: 0,
        e_exst_gs_rt_2_att_1_tpt_stts: 0,
        engn_exhst_gs_rrltn_2_attr_1_tmprtr: 76.0,
        engn_exhst_gs_rrltn_2_attr_1_dsrd_pstn: 42.0,
        engn_exhst_gs_rrltn_2_attr_2_prlmnr_fm: 0,
        e_exst_gs_rt_2_att_2_tpt_stts: 0,
        engn_exhst_gs_rrltn_2_attr_2_tmprtr: 74.0,
        engn_exhst_gs_rrltn_2_attr_2_dsrd_pstn: 38.0,
        engn_exhst_gs_rrltn_2_attr_1_oprtn_stts: 0,
        engn_exhst_gs_rrltn_2_attr_2_oprtn_stts: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.eegr2a_actuator_1_desired_position, 42.0, 1.0, "egr2_act1_position");
    assert_float_near(state.aftertreatment.eegr2a_actuator_1_temp, 76.0, 1.0, "egr2_act1_temp");
    assert_float_near(state.aftertreatment.eegr2a_actuator_2_desired_position, 38.0, 1.0, "egr2_act2_position");
}

#[test]
fn test_eegr2a_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EEGR2A::BASE_CAN_ID));
    assert!(found, "EEGR2A broadcast frame should be present");
}

// ============================================================================
// DPF2S - Diesel Particulate Filter 2 Soot
// ============================================================================

#[test]
fn test_dpf2s_handler_updates_state() {
    let mut state = test_state();
    let msg = DPF2S {
        device_id: external_device(),
        aftrtrtmnt_2_dsl_prtlt_fltr_st_mss: 20.0,
        aftrtrtmnt_2_dsl_prtlt_fltr_st_dnst: 3.5,
        aftrtrtmnt_2_dsl_prtlt_fltr_mn_st_sgnl: 40.0,
        atttt_2_ds_ptt_ft_md_st_s: 38.0,
        atttt_2_ds_ptt_ft_st_ss_pf: 0,
        ds_ptt_ft_2_st_ss_e_it_tpt: 50.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.aftertreatment.dpf2s_soot_mass, 20.0, 1.0, "soot_mass");
    assert_float_near(state.aftertreatment.dpf2s_soot_density, 3.5, 0.5, "soot_density");
    assert_float_near(state.aftertreatment.dpf2s_mean_soot_signal, 40.0, 1.0, "mean_soot_signal");
    assert_float_near(state.aftertreatment.dpf2s_median_soot_signal, 38.0, 1.0, "median_soot_signal");
}

#[test]
fn test_dpf2s_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, DPF2S::BASE_CAN_ID));
    assert!(found, "DPF2S broadcast frame should be present");
}

// ============================================================================
// Round-trip Tests: Broadcast -> Decode -> Verify
// ============================================================================

#[test]
fn test_at2og1_round_trip() {
    let state = test_state();
    let frames = state.generate_can_frames();

    // Find the AT2OG1 frame
    let at2og1_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AT2OG1::BASE_CAN_ID))
        .expect("AT2OG1 frame should exist");

    // Decode it
    let decoded = AT2OG1::decode(at2og1_frame.raw_id() & 0x1FFFFFFF, at2og1_frame.data()).unwrap();
    assert_float_near(decoded.aftertreatment_2_outlet_nox_1, state.aftertreatment.at2og1_outlet_nox, 1.0, "round_trip_outlet_nox");
}

#[test]
fn test_eegr1a_round_trip() {
    let state = test_state();
    let frames = state.generate_can_frames();

    let eegr1a_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EEGR1A::BASE_CAN_ID))
        .expect("EEGR1A frame should exist");

    let decoded = EEGR1A::decode(eegr1a_frame.raw_id() & 0x1FFFFFFF, eegr1a_frame.data()).unwrap();
    assert_float_near(decoded.engn_exhst_gs_rrltn_1_attr_1_dsrd_pstn, state.aftertreatment.eegr1a_actuator_1_desired_position, 1.0, "round_trip_egr1_position");
}

#[test]
fn test_dpf2s_round_trip() {
    let state = test_state();
    let frames = state.generate_can_frames();

    let dpf2s_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, DPF2S::BASE_CAN_ID))
        .expect("DPF2S frame should exist");

    let decoded = DPF2S::decode(dpf2s_frame.raw_id() & 0x1FFFFFFF, dpf2s_frame.data()).unwrap();
    assert_float_near(decoded.aftrtrtmnt_2_dsl_prtlt_fltr_st_mss, state.aftertreatment.dpf2s_soot_mass, 2.0, "round_trip_soot_mass");
}

// ============================================================================
// Cross-system: Bank 2 independent from Bank 1 defaults
// ============================================================================

#[test]
fn test_bank2_independent_from_bank1() {
    let mut state = test_state();

    // Modify bank 2 state
    state.aftertreatment.a2scrdsi1_dosing_rate = 900.0;
    state.aftertreatment.eegr2a_actuator_1_desired_position = 75.0;

    // Bank 1 should remain at defaults
    assert_float_near(state.aftertreatment.a1scrdsi1_dosing_rate, 500.0, 1.0, "bank1_dosing_unchanged");
    assert_float_near(state.aftertreatment.eegr1a_actuator_1_desired_position, 30.0, 1.0, "bank1_egr_unchanged");

    // Bank 2 should reflect changes
    assert_float_near(state.aftertreatment.a2scrdsi1_dosing_rate, 900.0, 1.0, "bank2_dosing_changed");
    assert_float_near(state.aftertreatment.eegr2a_actuator_1_desired_position, 75.0, 1.0, "bank2_egr_changed");
}

// ============================================================================
// All 20 broadcast messages present
// ============================================================================

#[test]
fn test_all_batch11_broadcasts_present() {
    let state = test_state();
    let frames = state.generate_can_frames();

    // 19 single-frame messages that should be broadcast
    // AT2HI1 is excluded because it has DLC=44 (multi-frame, requires J1939 TP)
    let batch11_ids: Vec<(&str, u32)> = vec![
        ("AT2S1", AT2S1::BASE_CAN_ID),
        ("AT2S2", AT2S2::BASE_CAN_ID),
        ("AT2OG1", AT2OG1::BASE_CAN_ID),
        ("AT2IG1", AT2IG1::BASE_CAN_ID),
        // AT2HI1 excluded: DLC=44, requires Transport Protocol
        ("AT2GP", AT2GP::BASE_CAN_ID),
        ("AT2FC1", AT2FC1::BASE_CAN_ID),
        ("AT2AC1", AT2AC1::BASE_CAN_ID),
        ("A2DOC1", A2DOC1::BASE_CAN_ID),
        ("A2SCRAI", A2SCRAI::BASE_CAN_ID),
        ("A2SCRSI1", A2SCRSI1::BASE_CAN_ID),
        ("A1SCRDSI1", A1SCRDSI1::BASE_CAN_ID),
        ("A1SCRDSI2", A1SCRDSI2::BASE_CAN_ID),
        ("A1SCRDSI3", A1SCRDSI3::BASE_CAN_ID),
        ("A2SCRDSI1", A2SCRDSI1::BASE_CAN_ID),
        ("A2SCRDSI2", A2SCRDSI2::BASE_CAN_ID),
        ("A2SCRDSI3", A2SCRDSI3::BASE_CAN_ID),
        ("EEGR1A", EEGR1A::BASE_CAN_ID),
        ("EEGR2A", EEGR2A::BASE_CAN_ID),
        ("DPF2S", DPF2S::BASE_CAN_ID),
    ];

    for (name, base_id) in &batch11_ids {
        let found = frames.iter().any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, *base_id));
        assert!(found, "{} broadcast frame should be present in generate_can_frames()", name);
    }
}

#[test]
fn test_default_state_has_reasonable_aftertreatment_values() {
    let state = test_state();

    // DPF soot load should be moderate (not full, not empty)
    assert!(state.aftertreatment.at2s1_dpf_soot_load_percent > 0);
    assert!(state.aftertreatment.at2s1_dpf_soot_load_percent < 100);

    // Inlet NOx should be higher than outlet NOx (SCR conversion)
    assert!(state.aftertreatment.at2ig1_inlet_nox > state.aftertreatment.at2og1_outlet_nox,
        "Inlet NOx ({}) should be higher than outlet NOx ({}) due to SCR conversion",
        state.aftertreatment.at2ig1_inlet_nox, state.aftertreatment.at2og1_outlet_nox);

    // SCR conversion efficiency should be high
    assert!(state.aftertreatment.a2scrsi1_scr_conversion_efficiency > 85.0,
        "SCR conversion efficiency should be > 85%, got {}",
        state.aftertreatment.a2scrsi1_scr_conversion_efficiency);

    // DOC outlet temp should be >= inlet temp (exothermic reaction)
    assert!(state.aftertreatment.a2doc1_outlet_temp >= state.aftertreatment.a2doc1_inlet_temp,
        "DOC outlet temp ({}) should be >= inlet temp ({})",
        state.aftertreatment.a2doc1_outlet_temp, state.aftertreatment.a2doc1_inlet_temp);

    // EGR actuator positions should be reasonable (0-100%)
    assert!(state.aftertreatment.eegr1a_actuator_1_desired_position >= 0.0);
    assert!(state.aftertreatment.eegr1a_actuator_1_desired_position <= 100.0);
    assert!(state.aftertreatment.eegr2a_actuator_1_desired_position >= 0.0);
    assert!(state.aftertreatment.eegr2a_actuator_1_desired_position <= 100.0);
}

// ============================================================================
// Self-reception: messages from our own device_id should be ignored
// ============================================================================

#[test]
fn test_at2s1_self_reception_ignored() {
    let mut state = test_state();
    let msg = AT2S1 {
        device_id: DeviceId::from(0x82),
        aftrtrtmnt_2_dsl_prtlt_fltr_st_ld_prnt: 45,
        atttt_2_ds_ptt_ft_as_ld_pt: 12,
        atttt_2_ds_ptt_ft_ts_lst_atv_rt: 7200,
        atttt_2_ds_ptt_ft_st_ld_rt_tsd: 90.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Ignored);
}

// ============================================================================
// DecodeFailed: corrupt/truncated data should return DecodeFailed
// ============================================================================

#[test]
fn test_batch11_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = AT2S1::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
