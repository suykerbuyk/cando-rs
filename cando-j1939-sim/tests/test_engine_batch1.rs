//! Engine Electronics Batch 1 Tests
//!
//! Tests for 20 core engine electronics messages:
//! EEC1, EEC2, EEC3, EEC4, EEC5, EEC6, EEC7, EEC9, EEC10, EEC11,
//! EEC13, EEC14, EEC16, EEC18, EEC19, EEC20, EEC23, EEC24, EEC25, ETC1

use cando_j1939_sim::{MessageStatus, SimulatorState};
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use socketcan::{EmbeddedFrame, Frame};

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

// ============================================================================
// EEC1 - Electronic Engine Controller 1
// ============================================================================

#[test]
fn test_eec1_handler() {
    let mut state = test_state();
    let msg = EEC1 {
        device_id: external_device(),
        engine_torque_mode: 2,
        atl_engn_prnt_trq_frtnl: 0.5,
        drvr_s_dmnd_engn_prnt_trq: 75.0,
        actual_engine_percent_torque: 80.0,
        engine_speed: 2400.0,
        sr_addrss_of_cntrllng_dv_fr_engn_cntrl: 0x00,
        engine_starter_mode: 0,
        engine_demand_percent_torque: 70.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_eq!(state.engine.eec1_engine_torque_mode, 2);
    assert_float_near(state.engine.eec1_engine_speed, 2400.0, 1.0, "eec1_engine_speed");
    assert_float_near(
        state.engine.eec1_actual_engine_percent_torque,
        80.0,
        1.0,
        "eec1_actual_engine_percent_torque",
    );
    assert_float_near(
        state.engine.eec1_drvr_s_dmnd_engn_prnt_trq,
        75.0,
        1.0,
        "eec1_drvr_s_dmnd_engn_prnt_trq",
    );
    assert_float_near(
        state.engine.eec1_engine_demand_percent_torque,
        70.0,
        1.0,
        "eec1_engine_demand_percent_torque",
    );
}

// ============================================================================
// EEC2 - Electronic Engine Controller 2
// ============================================================================

#[test]
fn test_eec2_handler() {
    let mut state = test_state();
    let msg = EEC2 {
        device_id: external_device(),
        accelerator_pedal_1_low_idle_switch: 0,
        accelerator_pedal_kickdown_switch: 1,
        road_speed_limit_status: 0,
        accelerator_pedal_2_low_idle_switch: 0,
        accelerator_pedal_1_position: 65.0,
        engine_percent_load_at_current_speed: 80,
        remote_accelerator_pedal_position: 0.0,
        accelerator_pedal_2_position: 30.0,
        vhl_alrtn_rt_lmt_stts: 0,
        mmntr_engn_mxmm_pwr_enl_fdk: 0,
        dpf_thermal_management_active: 0,
        scr_thermal_management_active: 0,
        atl_mxmm_avll_engn_prnt_trq: 95.0,
        estimated_pumping_percent_torque: -10.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec2_accelerator_pedal_1_position,
        65.0,
        1.0,
        "eec2_accelerator_pedal_1_position",
    );
    assert_eq!(state.engine.eec2_engine_percent_load_at_current_speed, 80);
    assert_float_near(
        state.engine.eec2_accelerator_pedal_2_position,
        30.0,
        1.0,
        "eec2_accelerator_pedal_2_position",
    );
    assert_eq!(state.engine.eec2_accelerator_pedal_kickdown_switch, 1);
}

// ============================================================================
// EEC3 - Electronic Engine Controller 3
// ============================================================================

#[test]
fn test_eec3_handler() {
    let mut state = test_state();
    let msg = EEC3 {
        device_id: external_device(),
        nominal_friction_percent_torque: 20.0,
        engine_s_desired_operating_speed: 1800.0,
        es_dsd_opt_spd_ast_adstt: 130,
        estmtd_engn_prst_lsss_prnt_trq: 5.0,
        aftrtrtmnt_1_exhst_gs_mss_flw_rt: 500.0,
        engine_exhaust_1_dew_point: 1,
        aftertreatment_1_exhaust_dew_point: 0,
        engine_exhaust_2_dew_point: 0,
        aftertreatment_2_exhaust_dew_point: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec3_nominal_friction_percent_torque,
        20.0,
        1.0,
        "eec3_nominal_friction_percent_torque",
    );
    assert_float_near(
        state.engine.eec3_engine_s_desired_operating_speed,
        1800.0,
        1.0,
        "eec3_engine_s_desired_operating_speed",
    );
    assert_eq!(state.engine.eec3_engine_exhaust_1_dew_point, 1);
}

// ============================================================================
// EEC4 - Electronic Engine Controller 4
// ============================================================================

#[test]
fn test_eec4_handler() {
    let mut state = test_state();
    let msg = EEC4 {
        device_id: external_device(),
        engine_rated_power: 300.0,
        engine_rated_speed: 2200.0,
        engine_rotation_direction: 1,
        engn_intk_mnfld_prssr_cntrl_md: 1,
        crnk_attmpt_cnt_on_prsnt_strt_attmpt: 3,
        engn_prl_ol_lw_prssr_thrshld: 60.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec4_engine_rated_power,
        300.0,
        1.0,
        "eec4_engine_rated_power",
    );
    assert_float_near(
        state.engine.eec4_engine_rated_speed,
        2200.0,
        1.0,
        "eec4_engine_rated_speed",
    );
    assert_eq!(state.engine.eec4_engine_rotation_direction, 1);
    assert_eq!(state.engine.eec4_crnk_attmpt_cnt_on_prsnt_strt_attmpt, 3);
}

// ============================================================================
// EEC5 - Electronic Engine Controller 5
// ============================================================================

#[test]
fn test_eec5_handler() {
    let mut state = test_state();
    let msg = EEC5 {
        device_id: external_device(),
        engn_trhrgr_1_clltd_trn_intk_tmprtr: 450.0,
        engn_trhrgr_1_clltd_trn_otlt_tmprtr: 380.0,
        engn_exhst_gs_rrltn_1_vlv_1_cntrl_1: 25.0,
        ev_gt_t_vt_a_ct_st_vv: 0,
        engine_fuel_control_mode: 1,
        engn_vrl_gmtr_trhrgr_1_cntrl_md: 1,
        engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn: 55.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec5_engn_trhrgr_1_clltd_trn_intk_tmprtr,
        450.0,
        2.0,
        "eec5_engn_trhrgr_1_clltd_trn_intk_tmprtr",
    );
    assert_float_near(
        state.engine.eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn,
        55.0,
        1.0,
        "eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn",
    );
    assert_eq!(state.engine.eec5_engine_fuel_control_mode, 1);
}

// ============================================================================
// EEC6 - Electronic Engine Controller 6
// ============================================================================

#[test]
fn test_eec6_handler() {
    let mut state = test_state();
    let msg = EEC6 {
        device_id: external_device(),
        engn_trhrgr_cmprssr_bpss_attr_1_cmmnd: 40.0,
        engn_vrl_gmtr_trhrgr_attr_1: 60.0,
        engn_trhrgr_cmprssr_bpss_attr_1_pstn: 38.0,
        engn_trhrgr_cmprssr_bpss_attr_2_cmmnd: 20.0,
        et_cpss_bpss_att_1_dsd_pst: 40.0,
        et_cpss_bpss_att_1_pf: 0,
        et_cpss_bpss_att_1_tpt_stts: 3,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_cmmnd,
        40.0,
        1.0,
        "eec6_engn_trhrgr_cmprssr_bpss_attr_1_cmmnd",
    );
    assert_float_near(
        state.engine.eec6_engn_vrl_gmtr_trhrgr_attr_1,
        60.0,
        1.0,
        "eec6_engn_vrl_gmtr_trhrgr_attr_1",
    );
    assert_eq!(state.engine.eec6_et_cpss_bpss_att_1_pf, 0);
}

// ============================================================================
// EEC7 - Electronic Engine Controller 7
// ============================================================================

#[test]
fn test_eec7_handler() {
    let mut state = test_state();
    let msg = EEC7 {
        device_id: external_device(),
        engn_exhst_gs_rrltn_1_vlv_pstn: 30.0,
        engn_exhst_gs_rrltn_1_vlv_2_pstn: 20.0,
        engn_crnks_brthr_ol_sprtr_spd: 5000,
        engn_intk_mnfld_cmmndd_prssr: 200.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec7_engn_exhst_gs_rrltn_1_vlv_pstn,
        30.0,
        1.0,
        "eec7_engn_exhst_gs_rrltn_1_vlv_pstn",
    );
    assert_eq!(state.engine.eec7_engn_crnks_brthr_ol_sprtr_spd, 5000);
    assert_float_near(
        state.engine.eec7_engn_intk_mnfld_cmmndd_prssr,
        200.0,
        1.0,
        "eec7_engn_intk_mnfld_cmmndd_prssr",
    );
}

// ============================================================================
// EEC9 - Electronic Engine Controller 9
// ============================================================================

#[test]
fn test_eec9_handler() {
    let mut state = test_state();
    let msg = EEC9 {
        device_id: external_device(),
        engn_exhst_gs_rrltn_2_vlv_pstn: 15.0,
        engn_exhst_gs_rrltn_2_vlv_2_pstn: 10.0,
        commanded_engine_fuel_rail_pressure: 100.0,
        cmmndd_engn_fl_injtn_cntrl_prssr: 80.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec9_commanded_engine_fuel_rail_pressure,
        100.0,
        1.0,
        "eec9_commanded_engine_fuel_rail_pressure",
    );
    assert_float_near(
        state.engine.eec9_cmmndd_engn_fl_injtn_cntrl_prssr,
        80.0,
        1.0,
        "eec9_cmmndd_engn_fl_injtn_cntrl_prssr",
    );
}

// ============================================================================
// EEC10 - Electronic Engine Controller 10
// ============================================================================

#[test]
fn test_eec10_handler() {
    let mut state = test_state();
    let msg = EEC10 {
        device_id: external_device(),
        engn_exhst_gs_rrltn_2_clr_intk_tmprtr: 120.0,
        e_exst_gs_rt_2_c_it_ast_pss: 200.0,
        engn_exhst_gs_rrltn_2_clr_effn: 75.0,
        e_exst_gs_rt_2_c_bpss_att_pst: 10.0,
        engn_exhst_gs_rrltn_2_clr_intk_prssr: 150.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec10_engn_exhst_gs_rrltn_2_clr_effn,
        75.0,
        1.0,
        "eec10_engn_exhst_gs_rrltn_2_clr_effn",
    );
    assert_float_near(
        state.engine.eec10_e_exst_gs_rt_2_c_it_ast_pss,
        200.0,
        1.0,
        "eec10_e_exst_gs_rt_2_c_it_ast_pss",
    );
}

// ============================================================================
// EEC11 - Electronic Engine Controller 11
// ============================================================================

#[test]
fn test_eec11_handler() {
    let mut state = test_state();
    let msg = EEC11 {
        device_id: external_device(),
        engn_exhst_gs_rrltn_2_vlv_1_cntrl: 50.0,
        engn_exhst_gs_rrltn_2_vlv_2_cntrl: 40.0,
        engn_exhst_gs_rrltn_2_vlv_1_pstn_errr: 2.0,
        engn_exhst_gs_rrltn_2_vlv_2_pstn_errr: -1.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec11_engn_exhst_gs_rrltn_2_vlv_1_cntrl,
        50.0,
        1.0,
        "eec11_engn_exhst_gs_rrltn_2_vlv_1_cntrl",
    );
    assert_float_near(
        state.engine.eec11_engn_exhst_gs_rrltn_2_vlv_2_cntrl,
        40.0,
        1.0,
        "eec11_engn_exhst_gs_rrltn_2_vlv_2_cntrl",
    );
}

// ============================================================================
// EEC13 - Electronic Engine Controller 13
// ============================================================================

#[test]
fn test_eec13_handler() {
    let mut state = test_state();
    let msg = EEC13 {
        device_id: external_device(),
        feedback_engine_fueling_state: 1,
        engine_fueling_inhibit_allowed: 0,
        engn_flng_inht_prvntd_rsn: 0,
        sr_addrss_of_cntrllng_dv_fr_flng_stt: 100,
        engine_dual_fuel_mode: 1,
        engn_flng_inht_prvntd_rsn_extnsn: 0,
        engn_gs_sstttn_fl_prntg: 40.0,
        engn_flng_inht_rqst_cnt: 0,
        engn_flng_dsrd_rqst_cnt: 1,
        engn_prttn_drt_ovrrd_stts: 0,
        remaining_engine_motoring_time: 5,
        engine_performance_bias_level: 60.0,
        minimum_engine_motoring_speed: 400.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_eq!(state.engine.eec13_engine_dual_fuel_mode, 1);
    assert_float_near(
        state.engine.eec13_engn_gs_sstttn_fl_prntg,
        40.0,
        1.0,
        "eec13_engn_gs_sstttn_fl_prntg",
    );
    assert_float_near(
        state.engine.eec13_engine_performance_bias_level,
        60.0,
        1.0,
        "eec13_engine_performance_bias_level",
    );
    assert_eq!(state.engine.eec13_remaining_engine_motoring_time, 5);
}

// ============================================================================
// EEC14 - Electronic Engine Controller 14
// ============================================================================

#[test]
fn test_eec14_handler() {
    let mut state = test_state();
    let msg = EEC14 {
        device_id: external_device(),
        engn_exhst_gs_rrltn_1_vlv_1_pstn_errr: 3.0,
        engn_exhst_gs_rrltn_1_vlv_2_pstn_errr: -2.0,
        engine_fuel_mass_flow_rate: 15.0,
        fuel_type: 4,
        engine_fuel_isolation_control: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec14_engine_fuel_mass_flow_rate,
        15.0,
        1.0,
        "eec14_engine_fuel_mass_flow_rate",
    );
    assert_eq!(state.engine.eec14_fuel_type, 4);
    assert_eq!(state.engine.eec14_engine_fuel_isolation_control, 0);
}

// ============================================================================
// EEC16 - Electronic Engine Controller 16
// ============================================================================

#[test]
fn test_eec16_handler() {
    let mut state = test_state();
    let msg = EEC16 {
        device_id: external_device(),
        accelerator_pedal_3_position: 50.0,
        ready_for_clutch_engagement_status: 1,
        engine_clutch_engage_request_status: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec16_accelerator_pedal_3_position,
        50.0,
        1.0,
        "eec16_accelerator_pedal_3_position",
    );
    assert_eq!(state.engine.eec16_ready_for_clutch_engagement_status, 1);
    assert_eq!(state.engine.eec16_engine_clutch_engage_request_status, 1);
}

// ============================================================================
// EEC18 - Electronic Engine Controller 18
// ============================================================================

#[test]
fn test_eec18_handler() {
    let mut state = test_state();
    let msg = EEC18 {
        device_id: external_device(),
        engn_clndr_hd_bpss_attr_1_cmmnd: 70.0,
        engine_intake_air_source_valve: 1,
        engn_exhst_gs_rstrtn_vlv_pstn: 25.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec18_engn_clndr_hd_bpss_attr_1_cmmnd,
        70.0,
        1.0,
        "eec18_engn_clndr_hd_bpss_attr_1_cmmnd",
    );
    assert_eq!(state.engine.eec18_engine_intake_air_source_valve, 1);
    assert_float_near(
        state.engine.eec18_engn_exhst_gs_rstrtn_vlv_pstn,
        25.0,
        1.0,
        "eec18_engn_exhst_gs_rstrtn_vlv_pstn",
    );
}

// ============================================================================
// EEC19 - Electronic Engine Controller 19
// ============================================================================

#[test]
fn test_eec19_handler() {
    let mut state = test_state();
    let msg = EEC19 {
        device_id: external_device(),
        total_engine_energy: 100000,
        engn_exhst_flw_rt_extndd_rng: 500.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_eq!(state.engine.eec19_total_engine_energy, 100000);
    assert_float_near(
        state.engine.eec19_engn_exhst_flw_rt_extndd_rng,
        500.0,
        1.0,
        "eec19_engn_exhst_flw_rt_extndd_rng",
    );
}

// ============================================================================
// EEC20 - Electronic Engine Controller 20
// ============================================================================

#[test]
fn test_eec20_handler() {
    let mut state = test_state();
    let msg = EEC20 {
        device_id: external_device(),
        esttd_e_pst_lsss_pt_tq_h_rst: 5.0,
        atl_mxmm_avll_engn_prnt_fl: 90.0,
        nmnl_frtn_prnt_trq_hgh_rsltn: 18.0,
        aslt_engn_ld_prnt_ar_mss: 45.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec20_atl_mxmm_avll_engn_prnt_fl,
        90.0,
        1.0,
        "eec20_atl_mxmm_avll_engn_prnt_fl",
    );
    assert_float_near(
        state.engine.eec20_aslt_engn_ld_prnt_ar_mss,
        45.0,
        1.0,
        "eec20_aslt_engn_ld_prnt_ar_mss",
    );
}

// ============================================================================
// EEC23 - Electronic Engine Controller 23
// ============================================================================

#[test]
fn test_eec23_handler() {
    let mut state = test_state();
    let msg = EEC23 {
        device_id: external_device(),
        engn_crnks_prssr_cntrl_attr_1_cmmnd: 60.0,
        engn_crnks_prssr_cntrl_attr_2_cmmnd: 55.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec23_engn_crnks_prssr_cntrl_attr_1_cmmnd,
        60.0,
        1.0,
        "eec23_engn_crnks_prssr_cntrl_attr_1_cmmnd",
    );
    assert_float_near(
        state.engine.eec23_engn_crnks_prssr_cntrl_attr_2_cmmnd,
        55.0,
        1.0,
        "eec23_engn_crnks_prssr_cntrl_attr_2_cmmnd",
    );
}

// ============================================================================
// EEC24 - Electronic Engine Controller 24
// ============================================================================

#[test]
fn test_eec24_handler() {
    let mut state = test_state();
    let msg = EEC24 {
        device_id: external_device(),
        engn_crnks_prssr_cntrl_attr_1_tmprtr: 75.0,
        engn_crnks_prssr_cntrl_attr_1_pstn: 60.0,
        e_cs_pss_ct_att_1_dsd_pst: 65.0,
        e_cs_pss_ct_att_1_pf: 0,
        e_cs_pss_ct_att_1_tpt_stts: 3,
        e_cs_pss_ct_att_1_opt_stts: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec24_engn_crnks_prssr_cntrl_attr_1_tmprtr,
        75.0,
        2.0,
        "eec24_engn_crnks_prssr_cntrl_attr_1_tmprtr",
    );
    assert_float_near(
        state.engine.eec24_engn_crnks_prssr_cntrl_attr_1_pstn,
        60.0,
        1.0,
        "eec24_engn_crnks_prssr_cntrl_attr_1_pstn",
    );
    assert_eq!(state.engine.eec24_e_cs_pss_ct_att_1_opt_stts, 0);
}

// ============================================================================
// EEC25 - Electronic Engine Controller 25
// ============================================================================

#[test]
fn test_eec25_handler() {
    let mut state = test_state();
    let msg = EEC25 {
        device_id: external_device(),
        engn_crnks_prssr_cntrl_attr_2_tmprtr: 80.0,
        engn_crnks_prssr_cntrl_attr_2_pstn: 55.0,
        e_cs_pss_ct_att_2_dsd_pst: 60.0,
        e_cs_pss_ct_att_2_pf: 0,
        e_cs_pss_ct_att_2_tpt_stts: 3,
        e_cs_pss_ct_att_2_opt_stts: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec25_engn_crnks_prssr_cntrl_attr_2_tmprtr,
        80.0,
        2.0,
        "eec25_engn_crnks_prssr_cntrl_attr_2_tmprtr",
    );
    assert_float_near(
        state.engine.eec25_engn_crnks_prssr_cntrl_attr_2_pstn,
        55.0,
        1.0,
        "eec25_engn_crnks_prssr_cntrl_attr_2_pstn",
    );
    assert_eq!(state.engine.eec25_e_cs_pss_ct_att_2_opt_stts, 1);
}

// ============================================================================
// ETC1 - Electronic Transmission Controller 1
// ============================================================================

#[test]
fn test_etc1_handler() {
    let mut state = test_state();
    let msg = ETC1 {
        device_id: external_device(),
        transmission_driveline_engaged: 1,
        trnsmssn_trq_cnvrtr_lkp_enggd: 1,
        transmission_shift_in_process: 0,
        tsss_tq_cvt_lp_tst_i_pss: 0,
        transmission_output_shaft_speed: 1500.0,
        percent_clutch_slip: 5.0,
        engine_momentary_overspeed_enable: 0,
        progressive_shift_disable: 0,
        mmntr_engn_mxmm_pwr_enl: 0,
        transmission_input_shaft_speed: 2000.0,
        s_addss_o_ct_dv_f_tsss_ct: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Recognized);
    assert_eq!(state.engine.etc1_transmission_driveline_engaged, 1);
    assert_eq!(state.engine.etc1_trnsmssn_trq_cnvrtr_lkp_enggd, 1);
    assert_float_near(
        state.engine.etc1_transmission_output_shaft_speed,
        1500.0,
        1.0,
        "etc1_transmission_output_shaft_speed",
    );
    assert_float_near(
        state.engine.etc1_transmission_input_shaft_speed,
        2000.0,
        1.0,
        "etc1_transmission_input_shaft_speed",
    );
    assert_float_near(
        state.engine.etc1_percent_clutch_slip,
        5.0,
        1.0,
        "etc1_percent_clutch_slip",
    );
}

// ============================================================================
// Self-reception filtering tests
// ============================================================================

#[test]
fn test_eec1_self_reception_ignored() {
    let mut state = test_state();
    // Send from our own device ID (0x82)
    let msg = EEC1 {
        device_id: DeviceId::from(0x82),
        engine_torque_mode: 5,
        atl_engn_prnt_trq_frtnl: 0.0,
        drvr_s_dmnd_engn_prnt_trq: 50.0,
        actual_engine_percent_torque: 50.0,
        engine_speed: 3000.0,
        sr_addrss_of_cntrllng_dv_fr_engn_cntrl: 0,
        engine_starter_mode: 0,
        engine_demand_percent_torque: 50.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Ignored);
    // State should NOT have been updated
    assert_eq!(state.engine.eec1_engine_torque_mode, 0); // Default value
}

// ============================================================================
// Broadcast verification tests
// ============================================================================

#[test]
fn test_eec1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let eec1_found = frames
        .iter()
        .any(|f| (f.raw_id() & 0x1FFFFFFF & 0xFFFFFF00) == EEC1::BASE_CAN_ID);
    assert!(eec1_found, "EEC1 broadcast frame not found");
}

#[test]
fn test_etc1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let etc1_found = frames
        .iter()
        .any(|f| (f.raw_id() & 0x1FFFFFFF & 0xFFFFFF00) == ETC1::BASE_CAN_ID);
    assert!(etc1_found, "ETC1 broadcast frame not found");
}

// ============================================================================
// Physics relationship tests
// ============================================================================

#[test]
fn test_physics_eec1_speed_responds_to_eec2_pedal() {
    let mut state = test_state();
    // Set accelerator pedal to 80%
    state.engine.eec2_accelerator_pedal_1_position = 80.0;
    // Run physics for several steps
    for _ in 0..100 {
        state.update_physics(0.1);
    }
    // Engine speed should have increased well above idle
    assert!(
        state.engine.eec1_engine_speed > 1000.0,
        "Engine speed should be above 1000 rpm with 80% pedal, got {}",
        state.engine.eec1_engine_speed
    );
}

#[test]
fn test_physics_etc1_output_shaft_from_engine() {
    let mut state = test_state();
    state.engine.eec1_engine_speed = 2000.0;
    state.transmission.etc2_transmission_actual_gear_ratio = 4.0;
    state.update_physics(0.1);
    assert_float_near(
        state.engine.etc1_transmission_output_shaft_speed,
        500.0,
        15.0,
        "etc1_output_shaft should be engine_speed / gear_ratio",
    );
}

#[test]
fn test_physics_turbo_boost_responds_to_load() {
    let mut state = test_state();
    // High accelerator demand
    state.engine.eec2_accelerator_pedal_1_position = 90.0;
    state.update_physics(0.1);
    // VGT position should be high
    assert!(
        state.engine.eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn > 50.0,
        "VGT position should be > 50% at high load, got {}",
        state.engine.eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn
    );
    // Compressor bypass should be mostly closed
    assert!(
        state.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_pstn < 30.0,
        "Compressor bypass should be < 30% at high load, got {}",
        state.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_pstn
    );
}

// ============================================================================
// Round-trip tests (encode from state -> generate frames -> decode -> verify)
// ============================================================================

#[test]
fn test_eec1_roundtrip() {
    let mut state = test_state();
    state.engine.eec1_engine_speed = 2200.0;
    state.engine.eec1_engine_torque_mode = 2;
    state.engine.eec1_actual_engine_percent_torque = 75.0;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| (f.raw_id() & 0x1FFFFFFF & 0xFFFFFF00) == EEC1::BASE_CAN_ID)
        .expect("EEC1 frame should exist");

    let decoded = EEC1::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_float_near(decoded.engine_speed, 2200.0, 1.0, "roundtrip engine_speed");
    assert_eq!(decoded.engine_torque_mode, 2);
    assert_float_near(
        decoded.actual_engine_percent_torque,
        75.0,
        1.0,
        "roundtrip actual_torque",
    );
}

#[test]
fn test_eec3_roundtrip() {
    let mut state = test_state();
    state.engine.eec3_nominal_friction_percent_torque = 22.0;
    state.engine.eec3_engine_s_desired_operating_speed = 1850.0;
    state.engine.eec3_engine_exhaust_1_dew_point = 1;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| (f.raw_id() & 0x1FFFFFFF & 0xFFFFFF00) == EEC3::BASE_CAN_ID)
        .expect("EEC3 frame should exist");

    let decoded = EEC3::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_float_near(
        decoded.nominal_friction_percent_torque,
        22.0,
        1.0,
        "roundtrip nominal_friction_percent_torque",
    );
    assert_float_near(
        decoded.engine_s_desired_operating_speed,
        1850.0,
        1.0,
        "roundtrip engine_s_desired_operating_speed",
    );
    assert_eq!(decoded.engine_exhaust_1_dew_point, 1);
}

#[test]
fn test_eec6_roundtrip() {
    let mut state = test_state();
    state.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_cmmnd = 42.0;
    state.engine.eec6_engn_vrl_gmtr_trhrgr_attr_1 = 58.0;
    state.engine.eec6_et_cpss_bpss_att_1_pf = 1;

    let frames = state.generate_can_frames();
    let frame = frames
        .iter()
        .find(|f| (f.raw_id() & 0x1FFFFFFF & 0xFFFFFF00) == EEC6::BASE_CAN_ID)
        .expect("EEC6 frame should exist");

    let decoded = EEC6::decode(frame.raw_id() & 0x1FFFFFFF, frame.data()).unwrap();
    assert_float_near(
        decoded.engn_trhrgr_cmprssr_bpss_attr_1_cmmnd,
        42.0,
        1.0,
        "roundtrip engn_trhrgr_cmprssr_bpss_attr_1_cmmnd",
    );
    assert_float_near(
        decoded.engn_vrl_gmtr_trhrgr_attr_1,
        58.0,
        1.0,
        "roundtrip engn_vrl_gmtr_trhrgr_attr_1",
    );
    assert_eq!(decoded.et_cpss_bpss_att_1_pf, 1);
}

// ============================================================================
// DecodeFailed test
// ============================================================================

#[test]
fn test_batch1_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = EEC1::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated -- triggers decode error
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}

// ============================================================================
// Broadcast count verification
// ============================================================================

#[test]
fn test_batch1_broadcast_count() {
    let state = test_state();
    let frames = state.generate_can_frames();
    // All 26 engine messages that are broadcast in generate_engine_frames
    let engine_ids = [
        EEC1::BASE_CAN_ID,
        EEC2::BASE_CAN_ID,
        EEC3::BASE_CAN_ID,
        EEC4::BASE_CAN_ID,
        EEC5::BASE_CAN_ID,
        EEC6::BASE_CAN_ID,
        EEC7::BASE_CAN_ID,
        EEC8::BASE_CAN_ID,
        EEC9::BASE_CAN_ID,
        EEC10::BASE_CAN_ID,
        EEC11::BASE_CAN_ID,
        EEC12::BASE_CAN_ID,
        EEC13::BASE_CAN_ID,
        EEC14::BASE_CAN_ID,
        EEC15::BASE_CAN_ID,
        EEC16::BASE_CAN_ID,
        EEC17::BASE_CAN_ID,
        EEC18::BASE_CAN_ID,
        EEC19::BASE_CAN_ID,
        EEC20::BASE_CAN_ID,
        EEC21::BASE_CAN_ID,
        EEC22::BASE_CAN_ID,
        EEC23::BASE_CAN_ID,
        EEC24::BASE_CAN_ID,
        EEC25::BASE_CAN_ID,
        ETC1::BASE_CAN_ID,
    ];
    let found_count = engine_ids
        .iter()
        .filter(|&&base_id| {
            frames
                .iter()
                .any(|f| (f.raw_id() & 0x1FFFFFFF & 0xFFFFFF00) == base_id)
        })
        .count();
    assert_eq!(
        found_count,
        engine_ids.len(),
        "All B1 engine messages should be broadcast"
    );
}
