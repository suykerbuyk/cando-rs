//! Batch 5: Braking & Stability Message Tests
//!
//! Tests for 18 braking and stability messages:
//! EBC1-7, EBCC, XBR, AEBS1, ACC1, ACC2, ACCS, ACCVC, ERC1, ERC2, RC, LMP

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
// EBC1 - Electronic Brake Controller 1
// ============================================================================

#[test]
fn test_ebc1_handler() {
    let mut state = test_state();
    let msg = EBC1 {
        device_id: external_device(),
        brake_pedal_position: 45.0,
        ebs_brake_switch: 1,
        anti_lock_braking_abs_active: 0,
        asr_engine_control_active: 0,
        asr_brake_control_active: 0,
        abs_off_road_switch: 0,
        asr_off_road_switch: 0,
        asr_hill_holder_switch: 0,
        traction_control_override_switch: 0,
        accelerator_interlock_switch: 0,
        engine_derate_switch: 0,
        engine_auxiliary_shutdown_switch: 0,
        remote_accelerator_enable_switch: 0,
        engine_retarder_selection: 30.0,
        abs_fully_operational: 1,
        ebs_red_warning_signal: 0,
        as_es_amr_wrnng_sgnl_pwrd_vhl: 0,
        atc_asr_information_signal: 0,
        sr_addrss_of_cntrllng_dv_fr_brk_cntrl: 0,
        railroad_mode_switch: 0,
        halt_brake_switch: 0,
        trailer_abs_status: 0,
        trtr_mntd_trlr_as_wrnng_sgnl: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.braking.ebc1_brake_pedal_position,
        45.0,
        1.0,
        "brake_pedal_position",
    );
    assert_eq!(state.braking.ebc1_ebs_brake_switch, 1);
    assert_eq!(state.braking.ebc1_abs_fully_operational, 1);
    assert_float_near(
        state.braking.ebc1_engine_retarder_selection,
        30.0,
        1.0,
        "engine_retarder_selection",
    );
}

#[test]
fn test_ebc1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EBC1::BASE_CAN_ID));
    assert!(found, "EBC1 frame should be present in broadcasts");
}

// ============================================================================
// EBC2 - Electronic Brake Controller 2
// ============================================================================

#[test]
fn test_ebc2_handler() {
    let mut state = test_state();
    let msg = EBC2 {
        device_id: external_device(),
        front_axle_speed: 80.0,
        relative_speed_front_axle_left_wheel: 1.5,
        rltv_spd_frnt_axl_rght_whl: -1.5,
        relative_speed_rear_axle_1_left_wheel: 0.5,
        rltv_spd_rr_axl_1_rght_whl: -0.5,
        relative_speed_rear_axle_2_left_wheel: 0.0,
        rltv_spd_rr_axl_2_rght_whl: 0.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.braking.ebc2_front_axle_speed, 80.0, 1.0, "front_axle_speed");
    assert_float_near(
        state.braking.ebc2_rel_speed_front_left,
        1.5,
        0.1,
        "rel_speed_front_left",
    );
}

#[test]
fn test_ebc2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EBC2::BASE_CAN_ID));
    assert!(found, "EBC2 frame should be present in broadcasts");
}

// ============================================================================
// EBC3 - Electronic Brake Controller 3
// ============================================================================

#[test]
fn test_ebc3_handler() {
    let mut state = test_state();
    let msg = EBC3 {
        device_id: external_device(),
        b_appt_pss_hr_ft_ax_lt_w: 500.0,
        b_appt_pss_hr_ft_ax_rt_w: 500.0,
        b_appt_pss_hrr_ax_1_lt_w: 400.0,
        b_appt_pss_hrr_ax_1_rt_w: 400.0,
        b_appt_pss_hrr_ax_2_lt_w: 0.0,
        b_appt_pss_hrr_ax_2_rt_w: 0.0,
        b_appt_pss_hrr_ax_3_lt_w: 0.0,
        b_appt_pss_hrr_ax_3_rt_w: 0.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.braking.ebc3_pressure_front_left,
        500.0,
        10.0,
        "pressure_front_left",
    );
    assert_float_near(
        state.braking.ebc3_pressure_rear1_left,
        400.0,
        10.0,
        "pressure_rear1_left",
    );
}

#[test]
fn test_ebc3_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EBC3::BASE_CAN_ID));
    assert!(found, "EBC3 frame should be present in broadcasts");
}

// ============================================================================
// EBC4 - Electronic Brake Controller 4
// ============================================================================

#[test]
fn test_ebc4_handler() {
    let mut state = test_state();
    let msg = EBC4 {
        device_id: external_device(),
        brk_lnng_rmnng_frnt_axl_lft_whl: 75.0,
        brk_lnng_rmnng_frnt_axl_rght_whl: 72.0,
        brk_lnng_rmnng_rr_axl_1_lft_whl: 65.0,
        brk_lnng_rmnng_rr_axl_1_rght_whl: 60.0,
        brk_lnng_rmnng_rr_axl_2_lft_whl: 80.0,
        brk_lnng_rmnng_rr_axl_2_rght_whl: 80.0,
        brk_lnng_rmnng_rr_axl_3_lft_whl: 90.0,
        brk_lnng_rmnng_rr_axl_3_rght_whl: 90.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.braking.ebc4_lining_front_left,
        75.0,
        1.0,
        "lining_front_left",
    );
    assert_float_near(
        state.braking.ebc4_lining_rear1_right,
        60.0,
        1.0,
        "lining_rear1_right",
    );
}

#[test]
fn test_ebc4_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EBC4::BASE_CAN_ID));
    assert!(found, "EBC4 frame should be present in broadcasts");
}

// ============================================================================
// EBC5 - Electronic Brake Controller 5
// ============================================================================

#[test]
fn test_ebc5_handler() {
    let mut state = test_state();
    let msg = EBC5 {
        device_id: external_device(),
        brake_temperature_warning: 1,
        halt_brake_mode: 0,
        hill_holder_mode: 0,
        foundation_brake_use: 1,
        xbr_system_state: 0,
        xbr_active_control_mode: 2,
        xbr_acceleration_limit: -5.0,
        prkng_brk_attr_fll_atvtd: 0,
        emergency_braking_active: 0,
        railroad_mode: 0,
        xbr_brake_hold_mode: 0,
        driver_brake_demand: -3.0,
        ovrll_intndd_brk_alrtn: -4.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.braking.ebc5_brake_temp_warning, 1);
    assert_eq!(state.braking.ebc5_foundation_brake_use, 1);
    assert_float_near(
        state.braking.ebc5_xbr_acceleration_limit,
        -5.0,
        0.5,
        "xbr_acceleration_limit",
    );
    assert_float_near(
        state.braking.ebc5_driver_brake_demand,
        -3.0,
        0.5,
        "driver_brake_demand",
    );
}

#[test]
fn test_ebc5_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EBC5::BASE_CAN_ID));
    assert!(found, "EBC5 frame should be present in broadcasts");
}

// ============================================================================
// EBC6 - Electronic Brake Controller 6
// ============================================================================

#[test]
fn test_ebc6_handler() {
    let mut state = test_state();
    let msg = EBC6 {
        device_id: external_device(),
        brk_lnng_rmnng_rr_axl_4_lft_whl: 50.0,
        brk_lnng_rmnng_rr_axl_4_rght_whl: 55.0,
        brk_lnng_rmnng_rr_axl_5_lft_whl: 60.0,
        brk_lnng_rmnng_rr_axl_5_rght_whl: 65.0,
        brk_lnng_rmnng_rr_axl_6_lft_whl: 70.0,
        brk_lnng_rmnng_rr_axl_6_rght_whl: 75.0,
        brk_lnng_rmnng_rr_axl_7_lft_whl: 80.0,
        brk_lnng_rmnng_rr_axl_7_rght_whl: 85.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.braking.ebc6_lining_rear4_left,
        50.0,
        1.0,
        "lining_rear4_left",
    );
    assert_float_near(
        state.braking.ebc6_lining_rear7_right,
        85.0,
        1.0,
        "lining_rear7_right",
    );
}

#[test]
fn test_ebc6_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EBC6::BASE_CAN_ID));
    assert!(found, "EBC6 frame should be present in broadcasts");
}

// ============================================================================
// EBC7 - Electronic Brake Controller 7
// ============================================================================

#[test]
fn test_ebc7_handler() {
    let mut state = test_state();
    let msg = EBC7 {
        device_id: external_device(),
        brk_lnng_rmnng_rr_axl_8_lft_whl: 45.0,
        brk_lnng_rmnng_rr_axl_8_rght_whl: 50.0,
        brk_lnng_rmnng_rr_axl_9_lft_whl: 55.0,
        brk_lnng_rmnng_rr_axl_9_rght_whl: 60.0,
        brk_lnng_rmnng_rr_axl_10_lft_whl: 70.0,
        brk_lnng_rmnng_rr_axl_10_rght_whl: 75.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.braking.ebc7_lining_rear8_left,
        45.0,
        1.0,
        "lining_rear8_left",
    );
    assert_float_near(
        state.braking.ebc7_lining_rear10_right,
        75.0,
        1.0,
        "lining_rear10_right",
    );
}

#[test]
fn test_ebc7_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EBC7::BASE_CAN_ID));
    assert!(found, "EBC7 frame should be present in broadcasts");
}

// ============================================================================
// EBCC - Engine Brake Continuous Control
// ============================================================================

#[test]
fn test_ebcc_handler() {
    let mut state = test_state();
    let msg = EBCC {
        device_id: external_device(),
        engn_trhrgr_1_trn_otlt_prssr: 120.0,
        engn_trhrgr_1_trn_dsrd_otlt_prssr: 130.0,
        engn_exhst_brk_attr_cmmnd: 75.0,
        engn_trhrgr_2_trn_otlt_prssr: 110.0,
        engn_trhrgr_2_trn_dsrd_otlt_prssr: 125.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.braking.ebcc_exhaust_brake_command,
        75.0,
        1.0,
        "exhaust_brake_command",
    );
    assert_float_near(
        state.braking.ebcc_turbo1_outlet_pressure,
        120.0,
        5.0,
        "turbo1_outlet_pressure",
    );
}

#[test]
fn test_ebcc_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EBCC::BASE_CAN_ID));
    assert!(found, "EBCC frame should be present in broadcasts");
}

// ============================================================================
// XBR - External Brake Request
// ============================================================================

#[test]
fn test_xbr_handler() {
    let mut state = test_state();
    let msg = XBR {
        device_id: external_device(),
        external_acceleration_demand: -5.0,
        xbr_ebi_mode: 1,
        xbr_priority: 0,
        xbr_control_mode: 2,
        xbr_compensation_mode: 0,
        xbr_urgency: 80.0,
        xbr_brake_hold_request: 0,
        xbr_reason: 1,
        xbr_message_counter: 5,
        xbr_message_checksum: 42,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.braking.xbr_acceleration_demand,
        -5.0,
        0.5,
        "acceleration_demand",
    );
    assert_eq!(state.braking.xbr_priority, 0);
    assert_eq!(state.braking.xbr_control_mode, 2);
    assert_float_near(state.braking.xbr_urgency, 80.0, 2.0, "xbr_urgency");
}

#[test]
fn test_xbr_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, XBR::BASE_CAN_ID));
    assert!(found, "XBR frame should be present in broadcasts");
}

// ============================================================================
// AEBS1 - Advanced Emergency Braking System 1
// ============================================================================

#[test]
fn test_aebs1_handler() {
    let mut state = test_state();
    let msg = AEBS1 {
        device_id: external_device(),
        fwd_cs_advd_eb_sst_stt: 2,
        collision_warning_level: 1,
        rvt_ot_dttd_f_advd_eb_sst: 1,
        bnd_off_prlt_of_rlvnt_ojt: 0,
        tm_t_cllsn_wth_rlvnt_ojt: 5.0,
        rd_dprtr_advnd_emrgn_brkng_sstm_stt: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.braking.aebs1_forward_collision_status, 2);
    assert_eq!(state.braking.aebs1_collision_warning_level, 1);
    assert_eq!(state.braking.aebs1_relevant_object_detected, 1);
    assert_float_near(
        state.braking.aebs1_time_to_collision,
        5.0,
        0.5,
        "time_to_collision",
    );
}

#[test]
fn test_aebs1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, AEBS1::BASE_CAN_ID));
    assert!(found, "AEBS1 frame should be present in broadcasts");
}

// ============================================================================
// ACC1 - Adaptive Cruise Control 1
// ============================================================================

#[test]
fn test_acc1_handler() {
    let mut state = test_state();
    let msg = ACC1 {
        device_id: external_device(),
        speed_of_forward_vehicle: 100,
        distance_to_forward_vehicle: 50,
        adaptive_cruise_control_set_speed: 120,
        adaptive_cruise_control_mode: 2,
        adptv_crs_cntrl_st_dstn_md: 1,
        road_curvature: 10.0,
        acc_target_detected: 1,
        acc_system_shutoff_warning: 0,
        acc_distance_alert_signal: 0,
        forward_collision_warning: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.braking.acc1_speed_of_forward_vehicle, 100);
    assert_eq!(state.braking.acc1_distance_to_forward_vehicle, 50);
    assert_eq!(state.braking.acc1_set_speed, 120);
    assert_eq!(state.braking.acc1_mode, 2);
    assert_eq!(state.braking.acc1_target_detected, 1);
}

#[test]
fn test_acc1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, ACC1::BASE_CAN_ID));
    assert!(found, "ACC1 frame should be present in broadcasts");
}

// ============================================================================
// ACC2 - Adaptive Cruise Control 2
// ============================================================================

#[test]
fn test_acc2_handler() {
    let mut state = test_state();
    let msg = ACC2 {
        device_id: external_device(),
        acc_usage_demand: 2,
        requested_acc_distance_mode: 3,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.braking.acc2_usage_demand, 2);
    assert_eq!(state.braking.acc2_distance_mode, 3);
}

#[test]
fn test_acc2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, ACC2::BASE_CAN_ID));
    assert!(found, "ACC2 frame should be present in broadcasts");
}

// ============================================================================
// ACCS - Acceleration Sensor
// ============================================================================

#[test]
fn test_accs_handler() {
    let mut state = test_state();
    let msg = ACCS {
        device_id: external_device(),
        ltrl_alrtn_extndd_rng: 2.5,
        lngtdnl_alrtn_extndd_rng: -3.0,
        vrtl_alrtn_extndd_rng: 9.8,
        ltrl_alrtn_fgr_of_mrt_extndd_rng: 1,
        lngtdnl_alrtn_fgr_of_mrt_extndd_rng: 1,
        vrtl_alrtn_fgr_of_mrt_extndd_rng: 1,
        sppt_v_tsss_rptt_rt_f_at_ss: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.braking.accs_lateral_acceleration,
        2.5,
        0.5,
        "lateral_acceleration",
    );
    assert_float_near(
        state.braking.accs_longitudinal_acceleration,
        -3.0,
        0.5,
        "longitudinal_acceleration",
    );
    assert_float_near(
        state.braking.accs_vertical_acceleration,
        9.8,
        0.5,
        "vertical_acceleration",
    );
}

#[test]
fn test_accs_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, ACCS::BASE_CAN_ID));
    assert!(found, "ACCS frame should be present in broadcasts");
}

// ============================================================================
// ACCVC - Aftercooler Coolant Valve Control
// ============================================================================

#[test]
fn test_accvc_handler() {
    let mut state = test_state();
    let msg = ACCVC {
        device_id: external_device(),
        engn_aftrlr_clnt_thrmstt_md: 2,
        engn_dsrd_aftrlr_clnt_intk_tmprtr: 35.0,
        engn_dsrd_aftrlr_clnt_thrmstt_opnng: 60.0,
        engn_chrg_ar_clr_bpss_vlv_1_cmmnd: 25.0,
        engn_chrg_ar_clr_bpss_vlv_2_cmmnd: 30.0,
        engn_aftrlr_clnt_dvrtr_vlv_cmmnd: 50.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.braking.accvc_aftercooler_thermostat_mode, 2);
    assert_float_near(
        state.braking.accvc_desired_aftercooler_temp,
        35.0,
        1.0,
        "desired_aftercooler_temp",
    );
    assert_float_near(
        state.braking.accvc_desired_thermostat_opening,
        60.0,
        1.0,
        "desired_thermostat_opening",
    );
}

#[test]
fn test_accvc_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, ACCVC::BASE_CAN_ID));
    assert!(found, "ACCVC frame should be present in broadcasts");
}

// ============================================================================
// ERC1 - Electronic Retarder Controller 1
// ============================================================================

#[test]
fn test_erc1_handler() {
    let mut state = test_state();
    let msg = ERC1 {
        device_id: external_device(),
        retarder_torque_mode: 1,
        retarder_enable_brake_assist_switch: 1,
        retarder_enable_shift_assist_switch: 0,
        actual_retarder_percent_torque: 50.0,
        intended_retarder_percent_torque: 55.0,
        engine_coolant_load_increase: 1,
        retarder_requesting_brake_light: 1,
        retarder_road_speed_limit_switch: 0,
        retarder_road_speed_exceeded_status: 0,
        s_addss_o_ct_dv_f_rtd_ct: 0x42,
        drvrs_dmnd_rtrdr_prnt_trq: 40.0,
        retarder_selection_non_engine: 60.0,
        atl_mxmm_avll_rtrdr_prnt_trq: 100.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.braking.erc1_retarder_torque_mode, 1);
    assert_float_near(
        state.braking.erc1_actual_retarder_torque,
        50.0,
        2.0,
        "actual_retarder_torque",
    );
    assert_float_near(
        state.braking.erc1_selection_non_engine,
        60.0,
        2.0,
        "selection_non_engine",
    );
    assert_eq!(state.braking.erc1_requesting_brake_light, 1);
}

#[test]
fn test_erc1_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, ERC1::BASE_CAN_ID));
    assert!(found, "ERC1 frame should be present in broadcasts");
}

// ============================================================================
// ERC2 - Electronic Retarder Controller 2
// ============================================================================

#[test]
fn test_erc2_handler() {
    let mut state = test_state();
    let msg = ERC2 {
        device_id: external_device(),
        transmission_output_retarder: 1,
        retarder_road_speed_limit_enable: 1,
        retarder_road_speed_limit_active: 1,
        trnsmssn_rtrdr_enl_swth: 1,
        crs_cntrl_rtrdr_atv_spd_offst: 5.0,
        retarder_road_speed_limit_set_speed: 90.0,
        retarder_road_speed_limit_readiness: 1,
        retarder_derate_status: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.braking.erc2_transmission_output_retarder, 1);
    assert_eq!(state.braking.erc2_road_speed_limit_active, 1);
    assert_float_near(
        state.braking.erc2_road_speed_limit_set_speed,
        90.0,
        2.0,
        "road_speed_limit_set_speed",
    );
}

#[test]
fn test_erc2_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, ERC2::BASE_CAN_ID));
    assert!(found, "ERC2 frame should be present in broadcasts");
}

// ============================================================================
// RC - Retarder Configuration
// ============================================================================

#[test]
fn test_rc_handler() {
    let mut state = test_state();
    let msg = RC {
        device_id: external_device(),
        retarder_type: 2,
        retarder_location: 1,
        retarder_control_method: 1,
        retarder_speed_at_idle_point_1: 700.0,
        rtrdr_prnt_trq_at_idl_pnt_1: 5.0,
        maximum_retarder_speed_point_2: 3000.0,
        rtrdr_prnt_trq_at_mxmm_spd_pnt_2: 100.0,
        retarder_speed_at_point_3: 1200.0,
        retarder_percent_torque_at_point_3: 30.0,
        retarder_speed_at_point_4: 1800.0,
        retarder_percent_torque_at_point_4: 60.0,
        retarder_speed_at_peak_torque_point_5: 2500.0,
        retarder_reference_torque: 1500,
        rtrdr_prnt_trq_at_pk_trq_pnt_5: 100.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.braking.rc_retarder_type, 2);
    assert_eq!(state.braking.rc_retarder_location, 1);
    assert_eq!(state.braking.rc_reference_torque, 1500);
    assert_float_near(state.braking.rc_max_speed, 3000.0, 10.0, "rc_max_speed");
}

#[test]
fn test_rc_broadcast_skipped_due_to_dlc() {
    // RC has DLC=19 (multi-packet), cannot fit in a standard 8-byte CAN frame.
    // The broadcast silently skips it, which is correct behavior.
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, RC::BASE_CAN_ID));
    assert!(
        !found,
        "RC frame should NOT be in broadcasts (DLC=19 exceeds 8-byte CAN limit)"
    );
}

// ============================================================================
// LMP - Mast Position
// ============================================================================

#[test]
fn test_lmp_handler() {
    let mut state = test_state();
    let msg = LMP {
        device_id: external_device(),
        mast_position: 75.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.braking.lmp_mast_position, 75.0, 1.0, "mast_position");
}

#[test]
fn test_lmp_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, LMP::BASE_CAN_ID));
    assert!(found, "LMP frame should be present in broadcasts");
}

// ============================================================================
// Physics Tests
// ============================================================================

#[test]
fn test_brake_pedal_sets_pressure_and_switch() {
    let mut state = test_state();
    state.braking.ebc1_brake_pedal_position = 80.0;
    state.update_physics(0.1);

    // Brake switch should be on
    assert_eq!(state.braking.ebc1_ebs_brake_switch, 1);
    // Pressures should be set proportional to pedal position
    assert!(
        state.braking.ebc3_pressure_front_left > 0.0,
        "Front left pressure should be > 0 when braking"
    );
    assert!(
        state.braking.ebc3_pressure_rear1_left > 0.0,
        "Rear1 left pressure should be > 0 when braking"
    );
    // Foundation brakes in use
    assert_eq!(state.braking.ebc5_foundation_brake_use, 1);
    // Driver brake demand should be negative (deceleration)
    assert!(
        state.braking.ebc5_driver_brake_demand < 0.0,
        "Driver brake demand should be negative under braking"
    );
}

#[test]
fn test_brake_pedal_release_decays_pressure() {
    let mut state = test_state();
    // Apply brakes
    state.braking.ebc1_brake_pedal_position = 100.0;
    state.update_physics(0.1);
    let initial_pressure = state.braking.ebc3_pressure_front_left;
    assert!(initial_pressure > 0.0);

    // Release brakes
    state.braking.ebc1_brake_pedal_position = 0.0;
    state.update_physics(0.1);
    // Pressure should decay
    assert!(
        state.braking.ebc3_pressure_front_left < initial_pressure,
        "Pressure should decay after brake release"
    );
    assert_eq!(state.braking.ebc1_ebs_brake_switch, 0);
}

#[test]
fn test_xbr_emergency_braking_sets_ebc5() {
    let mut state = test_state();
    // Set XBR with highest priority (emergency)
    state.braking.xbr_control_mode = 2; // Addition mode
    state.braking.xbr_acceleration_demand = -8.0; // Strong braking
    state.braking.xbr_priority = 0; // Highest priority (emergency)
    state.update_physics(0.1);

    assert_eq!(
        state.braking.ebc5_emergency_braking_active, 1,
        "Emergency braking should be active on priority 0 XBR"
    );
    assert!(
        state.braking.ebc5_overall_brake_demand < 0.0,
        "Overall brake demand should be negative"
    );
}

#[test]
fn test_retarder_activates_above_speed() {
    let mut state = test_state();
    // Set engine speed high enough for vehicle speed > 30 km/h
    // vehicle_speed = (engine_speed - 600) * 0.05
    // Need > 30 km/h => engine_speed > 1200 RPM
    state.engine.engine_speed = 1500.0;
    state.braking.erc1_selection_non_engine = 50.0; // 50% retarder demand
    state.update_physics(0.1);

    assert_eq!(
        state.braking.erc1_retarder_torque_mode, 1,
        "Retarder should be active above 30 km/h"
    );
    assert!(
        state.braking.erc1_actual_retarder_torque > 0.0,
        "Retarder torque should be > 0"
    );
    assert_eq!(
        state.braking.erc1_requesting_brake_light, 1,
        "Brake light should be requested when retarder active"
    );
}

#[test]
fn test_retarder_inactive_at_low_speed() {
    let mut state = test_state();
    state.engine.engine_speed = 700.0; // Low speed
    state.braking.erc1_selection_non_engine = 50.0;
    state.update_physics(0.1);

    // At engine_speed = 700, vehicle_speed = (700-600)*0.05 = 5 km/h < 30
    // Retarder should not activate
    assert_eq!(
        state.braking.erc1_retarder_torque_mode, 0,
        "Retarder should not activate below 30 km/h"
    );
}

#[test]
fn test_wheel_speed_tracks_engine() {
    let mut state = test_state();
    state.engine.engine_speed = 2000.0;
    state.update_physics(0.1);

    // vehicle_speed = (2000-600) * 0.05 = 70 km/h (CAN quantization may shift slightly)
    assert_float_near(state.braking.ebc2_front_axle_speed, 70.0, 5.0, "front_axle_speed");
}

#[test]
fn test_accs_tracks_braking_deceleration() {
    let mut state = test_state();
    state.braking.ebc1_brake_pedal_position = 60.0;
    state.update_physics(0.1);

    // Longitudinal acceleration should track overall brake demand
    assert!(
        state.braking.accs_longitudinal_acceleration < 0.0,
        "Longitudinal accel should be negative under braking"
    );
}

// ============================================================================
// Round-trip Tests (Encode -> Broadcast -> Decode)
// ============================================================================

#[test]
fn test_ebc1_roundtrip() {
    let mut state = test_state();
    state.braking.ebc1_brake_pedal_position = 55.0;
    state.braking.ebc1_ebs_brake_switch = 1;
    state.braking.ebc1_abs_fully_operational = 1;

    let frames = state.generate_can_frames();
    let ebc1_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, EBC1::BASE_CAN_ID))
        .expect("EBC1 frame should exist");

    let decoded = EBC1::decode(ebc1_frame.raw_id() & 0x1FFFFFFF, ebc1_frame.data()).unwrap();
    assert_float_near(
        decoded.brake_pedal_position,
        55.0,
        1.0,
        "roundtrip brake_pedal_position",
    );
    assert_eq!(decoded.ebs_brake_switch, 1);
    assert_eq!(decoded.abs_fully_operational, 1);
}

#[test]
fn test_xbr_roundtrip() {
    let mut state = test_state();
    state.braking.xbr_acceleration_demand = -3.0;
    state.braking.xbr_priority = 1;
    state.braking.xbr_control_mode = 1;

    let frames = state.generate_can_frames();
    let xbr_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, XBR::BASE_CAN_ID))
        .expect("XBR frame should exist");

    let decoded = XBR::decode(xbr_frame.raw_id() & 0x1FFFFFFF, xbr_frame.data()).unwrap();
    assert_float_near(
        decoded.external_acceleration_demand,
        -3.0,
        0.5,
        "roundtrip xbr_demand",
    );
    assert_eq!(decoded.xbr_priority, 1);
    assert_eq!(decoded.xbr_control_mode, 1);
}

#[test]
fn test_erc1_roundtrip() {
    let mut state = test_state();
    state.braking.erc1_retarder_torque_mode = 1;
    state.braking.erc1_actual_retarder_torque = 40.0;
    state.braking.erc1_requesting_brake_light = 1;

    let frames = state.generate_can_frames();
    let erc1_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, ERC1::BASE_CAN_ID))
        .expect("ERC1 frame should exist");

    let decoded = ERC1::decode(erc1_frame.raw_id() & 0x1FFFFFFF, erc1_frame.data()).unwrap();
    assert_eq!(decoded.retarder_torque_mode, 1);
    assert_float_near(
        decoded.actual_retarder_percent_torque,
        40.0,
        2.0,
        "roundtrip retarder_torque",
    );
    assert_eq!(decoded.retarder_requesting_brake_light, 1);
}

// ============================================================================
// Self-Reception & Decode Failure Tests
// ============================================================================

#[test]
fn test_ebc1_self_reception_ignored() {
    let mut state = test_state();
    let msg = EBC1 {
        device_id: DeviceId::from(0x82),
        brake_pedal_position: 45.0,
        ebs_brake_switch: 1,
        anti_lock_braking_abs_active: 0,
        asr_engine_control_active: 0,
        asr_brake_control_active: 0,
        abs_off_road_switch: 0,
        asr_off_road_switch: 0,
        asr_hill_holder_switch: 0,
        traction_control_override_switch: 0,
        accelerator_interlock_switch: 0,
        engine_derate_switch: 0,
        engine_auxiliary_shutdown_switch: 0,
        remote_accelerator_enable_switch: 0,
        engine_retarder_selection: 30.0,
        abs_fully_operational: 1,
        ebs_red_warning_signal: 0,
        as_es_amr_wrnng_sgnl_pwrd_vhl: 0,
        atc_asr_information_signal: 0,
        sr_addrss_of_cntrllng_dv_fr_brk_cntrl: 0,
        railroad_mode_switch: 0,
        halt_brake_switch: 0,
        trailer_abs_status: 0,
        trtr_mntd_trlr_as_wrnng_sgnl: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Ignored);
    // State should NOT have been updated
    assert_eq!(state.braking.ebc1_ebs_brake_switch, 0); // Default value
}

#[test]
fn test_batch5_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = EBC1::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
