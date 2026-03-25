//! Transmission Batch 2 Tests
//!
//! Tests for the 14 new transmission and drivetrain messages:
//! ETC3, ETC4, ETC7, ETC8, ETC10, ETC11, ETC12, ETC13, ETC14, ETC15, ETCC4, ETCBI, TC1, TC2

use cando_j1939_sim::{MessageStatus, SimulatorState};
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use socketcan::{EmbeddedFrame, Frame};

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

/// Assert float values are approximately equal with CAN signal tolerance
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

/// Check if a CAN frame ID matches a message type's BASE_CAN_ID (masking out the source address byte)
fn matches_base_id(frame_id: u32, base_can_id: u32) -> bool {
    (frame_id & 0xFFFFFF00) == base_can_id
}

// ============================================================================
// ETC3 - Electronic Transmission Controller 3 (Shift Finger & Actuators)
// ============================================================================

#[test]
fn test_etc3_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC3 {
        device_id: external_device(),
        trnsmssn_shft_fngr_gr_pstn: 75.0,
        trnsmssn_shft_fngr_rl_pstn: 60.0,
        trnsmssn_shft_fngr_ntrl_indtr: 0,
        trnsmssn_shft_fngr_enggmnt_indtr: 1,
        trnsmssn_shft_fngr_cntr_rl_indtr: 0,
        trnsmssn_shft_fngr_rl_attr_1: 1,
        trnsmssn_shft_fngr_gr_attr_1: 1,
        trnsmssn_shft_fngr_rl_attr_2: 0,
        trnsmssn_shft_fngr_gr_attr_2: 0,
        transmission_range_high_actuator: 1,
        transmission_range_low_actuator: 0,
        trnsmssn_splttr_drt_attr: 1,
        trnsmssn_splttr_indrt_attr: 0,
        transmission_clutch_actuator: 1,
        trnsmssn_trq_cnvrtr_lkp_clth_attr: 0,
        transmission_defuel_actuator: 0,
        trnsmssn_inrt_brk_attr: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etc3_shift_finger_gear_position,
        75.0,
        1.0,
        "etc3_shift_finger_gear_position",
    );
    assert_float_near(
        state.transmission.etc3_shift_finger_rail_position,
        60.0,
        1.0,
        "etc3_shift_finger_rail_position",
    );
    assert_eq!(state.transmission.etc3_shift_finger_engagement_indicator, 1);
    assert_eq!(state.transmission.etc3_clutch_actuator, 1);
    assert_eq!(state.transmission.etc3_range_high_actuator, 1);
}

#[test]
fn test_etc3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc3 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC3::BASE_CAN_ID));
    assert!(has_etc3, "ETC3 broadcast frame should be present");
}

// ============================================================================
// ETC4 - Electronic Transmission Controller 4 (Synchronizer)
// ============================================================================

#[test]
fn test_etc4_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC4 {
        device_id: external_device(),
        trnsmssn_snhrnzr_clth_vl: 80.0,
        trnsmssn_snhrnzr_brk_vl: 30.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etc4_synchronizer_clutch_value,
        80.0,
        1.0,
        "etc4_synchronizer_clutch_value",
    );
    assert_float_near(
        state.transmission.etc4_synchronizer_brake_value,
        30.0,
        1.0,
        "etc4_synchronizer_brake_value",
    );
}

#[test]
fn test_etc4_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc4 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC4::BASE_CAN_ID));
    assert!(has_etc4, "ETC4 broadcast frame should be present");
}

// ============================================================================
// ETC7 - Electronic Transmission Controller 7 (Display & Indicators)
// ============================================================================

#[test]
fn test_etc7_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC7 {
        device_id: external_device(),
        trnsmssn_crrnt_rng_dspl_blnk_stt: 0,
        transmission_service_indicator: 1,
        trnsmssn_rqstd_rng_dspl_blnk_stt: 0,
        trnsmssn_rqstd_rng_dspl_flsh_stt: 1,
        trnsmssn_rd_fr_brk_rls: 1,
        active_shift_console_indicator: 0,
        transmission_engine_crank_enable: 1,
        trnsmssn_shft_inht_indtr: 0,
        transmission_mode_1_indicator: 1,
        transmission_mode_2_indicator: 0,
        transmission_mode_3_indicator: 0,
        transmission_mode_4_indicator: 0,
        transmission_mode_5_indicator: 0,
        transmission_mode_6_indicator: 0,
        transmission_mode_7_indicator: 0,
        transmission_mode_8_indicator: 0,
        trnsmssn_rqstd_gr_fdk: 3.0,
        trnsmssn_rvrs_gr_shft_inht_stts: 0,
        transmission_warning_indicator: 0,
        transmission_mode_9_indicator: 0,
        transmission_mode_10_indicator: 0,
        trnsmssn_ar_sppl_prssr_indtr: 0,
        trnsmssn_at_ntrl_mnl_rtrn_stt: 0,
        transmission_manual_mode_indicator: 0,
        trnsmssn_ld_rdtn_indtr: 0,
        trnsmssn_pr_dfnd_rng_lmt_indtr: 0,
        transmission_coast_mode_indicator: 0,
        trnsmssn_otpt_shft_brk_indtr: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.transmission.etc7_service_indicator, 1);
    assert_eq!(state.transmission.etc7_ready_for_brake_release, 1);
    assert_eq!(state.transmission.etc7_engine_crank_enable, 1);
    assert_eq!(state.transmission.etc7_mode_1_indicator, 1);
    assert_float_near(
        state.transmission.etc7_requested_gear_feedback,
        3.0,
        1.0,
        "etc7_requested_gear_feedback",
    );
}

#[test]
fn test_etc7_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc7 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC7::BASE_CAN_ID));
    assert!(has_etc7, "ETC7 broadcast frame should be present");
}

// ============================================================================
// ETC8 - Electronic Transmission Controller 8 (Torque Converter)
// ============================================================================

#[test]
fn test_etc8_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC8 {
        device_id: external_device(),
        trnsmssn_trq_cnvrtr_rt: 2.5,
        trnsmssn_clth_cnvrtr_inpt_spd: 1500.0,
        transmission_shift_inhibit_reason: 3,
        trnsmssn_trq_cnvrtr_lkp_inht_rsn: 0,
        trnsmssn_trq_cnvrtr_lkp_inht_indtr: 0,
        trnsmssn_explt_mnl_md_indtr: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etc8_torque_converter_ratio,
        2.5,
        0.01,
        "etc8_torque_converter_ratio",
    );
    assert_float_near(
        state.transmission.etc8_clutch_converter_input_speed,
        1500.0,
        1.0,
        "etc8_clutch_converter_input_speed",
    );
    assert_eq!(state.transmission.etc8_shift_inhibit_reason, 3);
    assert_eq!(state.transmission.etc8_explicit_manual_mode_indicator, 1);
}

#[test]
fn test_etc8_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc8 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC8::BASE_CAN_ID));
    assert!(has_etc8, "ETC8 broadcast frame should be present");
}

// ============================================================================
// ETC10 - Electronic Transmission Controller 10 (Clutch & Actuator Positions)
// ============================================================================

#[test]
fn test_etc10_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC10 {
        device_id: external_device(),
        trnsmssn_clth_1_attr_pstn: 85.0,
        trnsmssn_clth_2_attr_pstn: 20.0,
        trnsmssn_hdrl_pmp_attr_1_pstn: 60.0,
        trnsmssn_1_shft_attr_1_pstn: 40.0,
        trnsmssn_1_shft_attr_2_pstn: 30.0,
        trnsmssn_clth_1_clng_attr_stts: 1,
        trnsmssn_clth_2_clng_attr_stts: 0,
        trnsmssn_shft_rl_1_attr_stts: 1,
        trnsmssn_shft_rl_2_attr_stts: 0,
        trnsmssn_shft_rl_3_attr_stts: 0,
        trnsmssn_shft_rl_4_attr_stts: 0,
        trnsmssn_shft_rl_5_attr_stts: 0,
        trnsmssn_shft_rl_6_attr_stts: 0,
        trnsmssn_hdrl_pmp_attr_2_prnt: 45.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etc10_clutch_1_actuator_position,
        85.0,
        1.0,
        "etc10_clutch_1_actuator_position",
    );
    assert_float_near(
        state.transmission.etc10_clutch_2_actuator_position,
        20.0,
        1.0,
        "etc10_clutch_2_actuator_position",
    );
    assert_eq!(state.transmission.etc10_clutch_1_cooling_actuator_status, 1);
    assert_eq!(state.transmission.etc10_shift_rail_1_actuator_status, 1);
}

#[test]
fn test_etc10_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc10 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC10::BASE_CAN_ID));
    assert!(has_etc10, "ETC10 broadcast frame should be present");
}

// ============================================================================
// ETC11 - Electronic Transmission Controller 11 (Shift Rail Positions)
// ============================================================================

#[test]
fn test_etc11_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC11 {
        device_id: external_device(),
        transmission_shift_rail_1_position: 20.0,
        transmission_shift_rail_2_position: 40.0,
        transmission_shift_rail_3_position: 60.0,
        transmission_shift_rail_4_position: 80.0,
        transmission_shift_rail_5_position: 100.0,
        transmission_shift_rail_6_position: 0.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etc11_shift_rail_1_position,
        20.0,
        1.0,
        "etc11_shift_rail_1_position",
    );
    assert_float_near(
        state.transmission.etc11_shift_rail_4_position,
        80.0,
        1.0,
        "etc11_shift_rail_4_position",
    );
    assert_float_near(
        state.transmission.etc11_shift_rail_5_position,
        100.0,
        1.0,
        "etc11_shift_rail_5_position",
    );
}

#[test]
fn test_etc11_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc11 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC11::BASE_CAN_ID));
    assert!(has_etc11, "ETC11 broadcast frame should be present");
}

// ============================================================================
// ETC12 - Electronic Transmission Controller 12 (Hydrostatic Loop)
// ============================================================================

#[test]
fn test_etc12_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC12 {
        device_id: external_device(),
        trnsmssn_hdrstt_lp_1_prssr: 8000,
        trnsmssn_hdrstt_lp_2_prssr: 7500,
        trnsmssn_drtnl_otpt_shft_spd: 1200.0,
        trnsmssn_intrmdt_shft_spd: 2400.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.transmission.etc12_hydrostatic_loop_1_pressure, 8000);
    assert_eq!(state.transmission.etc12_hydrostatic_loop_2_pressure, 7500);
    assert_float_near(
        state.transmission.etc12_directional_output_shaft_speed,
        1200.0,
        1.0,
        "etc12_directional_output_shaft_speed",
    );
}

#[test]
fn test_etc12_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc12 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC12::BASE_CAN_ID));
    assert!(has_etc12, "ETC12 broadcast frame should be present");
}

// ============================================================================
// ETC13 - Electronic Transmission Controller 13 (Max Speeds & Mode Indicators)
// ============================================================================

#[test]
fn test_etc13_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC13 {
        device_id: external_device(),
        mxmm_frwrd_trnsmssn_otpt_shft_spd: 5000.0,
        mxmm_rvrs_trnsmssn_otpt_shft_spd: 2500.0,
        s_addss_o_atv_o_pd_tsss_rqstd_g: 0x42,
        transmission_mode_11_indicator: 1,
        transmission_mode_12_indicator: 0,
        transmission_mode_13_indicator: 0,
        transmission_mode_14_indicator: 0,
        transmission_mode_15_indicator: 0,
        transmission_mode_16_indicator: 0,
        transmission_mode_17_indicator: 0,
        transmission_mode_18_indicator: 0,
        transmission_mode_19_indicator: 0,
        transmission_mode_20_indicator: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etc13_max_forward_output_shaft_speed,
        5000.0,
        1.0,
        "etc13_max_forward_output_shaft_speed",
    );
    assert_float_near(
        state.transmission.etc13_max_reverse_output_shaft_speed,
        2500.0,
        1.0,
        "etc13_max_reverse_output_shaft_speed",
    );
    assert_eq!(state.transmission.etc13_source_address_requested_gear, 0x42);
    assert_eq!(state.transmission.etc13_mode_11_indicator, 1);
}

#[test]
fn test_etc13_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc13 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC13::BASE_CAN_ID));
    assert!(has_etc13, "ETC13 broadcast frame should be present");
}

// ============================================================================
// ETC14 - Electronic Transmission Controller 14 (Clutch Temp & Capability)
// ============================================================================

#[test]
fn test_etc14_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC14 {
        device_id: external_device(),
        transmission_clutch_1_temperature: 120.0,
        trnsmssn_clth_1_ovrht_indtr: 0,
        transmission_launch_capability: 0,
        transmission_gear_shift_capability: 0,
        trnsmssn_dmg_thrshld_stts: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etc14_clutch_1_temperature,
        120.0,
        1.0,
        "etc14_clutch_1_temperature",
    );
    assert_eq!(state.transmission.etc14_clutch_1_overheat_indicator, 0);
}

#[test]
fn test_etc14_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc14 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC14::BASE_CAN_ID));
    assert!(has_etc14, "ETC14 broadcast frame should be present");
}

// ============================================================================
// ETC15 - Electronic Transmission Controller 15 (CRC, Counter, Auto-Neutral)
// ============================================================================

#[test]
fn test_etc15_handler_updates_state() {
    let mut state = test_state();
    let msg = ETC15 {
        device_id: external_device(),
        eltrn_trnsmssn_cntrl_15_cr: 42,
        eltrn_trnsmssn_cntrl_15_cntr: 7,
        trnsmssn_at_ntrl_at_rtrn_rqst_fdk: 1,
        launch_process_status: 2,
        trnsmssn_at_ntrl_at_rtrn_fntn_stt: 3,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.transmission.etc15_crc, 42);
    assert_eq!(state.transmission.etc15_counter, 7);
    assert_eq!(state.transmission.etc15_launch_process_status, 2);
    assert_eq!(
        state.transmission.etc15_auto_neutral_auto_return_function_state,
        3
    );
}

#[test]
fn test_etc15_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etc15 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETC15::BASE_CAN_ID));
    assert!(has_etc15, "ETC15 broadcast frame should be present");
}

// ============================================================================
// ETCC4 - Engine Turbocharger Control 4
// ============================================================================

#[test]
fn test_etcc4_handler_updates_state() {
    let mut state = test_state();
    let msg = ETCC4 {
        device_id: external_device(),
        engn_trhrgr_wstgt_attr_3_cmmnd: 80.0,
        engn_trhrgr_wstgt_attr_4_cmmnd: 65.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etcc4_wastegate_actuator_3_command,
        80.0,
        0.1,
        "etcc4_wastegate_actuator_3_command",
    );
    assert_float_near(
        state.transmission.etcc4_wastegate_actuator_4_command,
        65.0,
        0.1,
        "etcc4_wastegate_actuator_4_command",
    );
}

#[test]
fn test_etcc4_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etcc4 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETCC4::BASE_CAN_ID));
    assert!(has_etcc4, "ETCC4 broadcast frame should be present");
}

// ============================================================================
// ETCBI - Engine Turbocharger Compressor Bypass Information
// ============================================================================

#[test]
fn test_etcbi_handler_updates_state() {
    let mut state = test_state();
    let msg = ETCBI {
        device_id: external_device(),
        engn_trhrgr_cmprssr_bpss_attr_2_pstn: 40.0,
        et_cpss_bpss_att_2_dsd_pst: 45.0,
        et_cpss_bpss_att_2_pf: 5,
        et_cpss_bpss_att_2_tpt_stts: 1,
        et_cpss_bpss_att_1_opt_stts: 0,
        et_cpss_bpss_att_2_opt_stts: 1,
        et_cpss_bpss_att_1_tpt: 70.0,
        et_cpss_bpss_att_2_tpt: 75.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etcbi_compressor_bypass_actuator_2_position,
        40.0,
        1.0,
        "etcbi_compressor_bypass_actuator_2_position",
    );
    assert_float_near(
        state.transmission.etcbi_compressor_bypass_actuator_1_temperature,
        70.0,
        1.0,
        "etcbi_compressor_bypass_actuator_1_temperature",
    );
    assert_eq!(
        state.transmission.etcbi_compressor_bypass_actuator_2_preliminary_fmi,
        5
    );
}

#[test]
fn test_etcbi_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_etcbi = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), ETCBI::BASE_CAN_ID));
    assert!(has_etcbi, "ETCBI broadcast frame should be present");
}

// ============================================================================
// TC1 - Transmission Control 1 (Gear Shift Commands)
// ============================================================================

#[test]
fn test_tc1_handler_updates_state() {
    let mut state = test_state();
    let msg = TC1 {
        device_id: external_device(),
        trnsmssn_gr_shft_inht_rqst: 0,
        trnsmssn_trq_cnvrtr_lkp_rqst: 1,
        disengage_driveline_request: 0,
        trnsmssn_rvrs_gr_shft_inht_rqst: 0,
        requested_percent_clutch_slip: 15.0,
        transmission_requested_gear: 4.0,
        dsngg_dffrntl_lk_rqst_frnt_axl_1: 3,
        dsngg_dffrntl_lk_rqst_frnt_axl_2: 3,
        dsngg_dffrntl_lk_rqst_rr_axl_1: 3,
        dsngg_dffrntl_lk_rqst_rr_axl_2: 3,
        dsngg_dffrntl_lk_rqst_cntrl: 3,
        dsngg_dffrntl_lk_rqst_cntrl_frnt: 3,
        dsngg_dffrntl_lk_rqst_cntrl_rr: 3,
        trnsmssn_ld_rdtn_inht_rqst: 0,
        transmission_mode_1: 1,
        transmission_mode_2: 0,
        transmission_mode_3: 0,
        transmission_mode_4: 0,
        trnsmssn_at_ntrl_mnl_rtrn_rqst: 3,
        transmission_requested_launch_gear: 2,
        trnsmssn_shft_sltr_dspl_md_swth: 0,
        transmission_mode_5: 0,
        transmission_mode_6: 0,
        transmission_mode_7: 0,
        transmission_mode_8: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.tc1_transmission_requested_gear,
        4.0,
        1.0,
        "tc1_transmission_requested_gear",
    );
    assert_float_near(
        state.transmission.tc1_requested_percent_clutch_slip,
        15.0,
        1.0,
        "tc1_requested_percent_clutch_slip",
    );
    assert_eq!(state.transmission.tc1_torque_converter_lockup_request, 1);
    assert_eq!(state.transmission.tc1_mode_1, 1);
    assert_eq!(state.transmission.tc1_requested_launch_gear, 2);
    // Physics: TC1 should update ETC7 gear feedback and ETC2 selected gear
    assert_float_near(
        state.transmission.etc7_requested_gear_feedback,
        4.0,
        1.0,
        "etc7_requested_gear_feedback",
    );
    assert_float_near(
        state.transmission.etc2_transmission_selected_gear,
        4.0,
        1.0,
        "etc2_transmission_selected_gear",
    );
}

#[test]
fn test_tc1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_tc1 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), TC1::BASE_CAN_ID));
    assert!(has_tc1, "TC1 broadcast frame should be present");
}

// ============================================================================
// TC2 - Transmission Control 2 (Extended Mode Commands)
// ============================================================================

#[test]
fn test_tc2_handler_updates_state() {
    let mut state = test_state();
    let msg = TC2 {
        device_id: external_device(),
        transmission_mode_9: 1,
        transmission_mode_10: 0,
        trnsmssn_pr_dfnd_mxmm_gr_atvtn_rqst: 1,
        trnsmssn_otpt_shft_brk_rqst: 1,
        trnsmssn_rqstd_rvrs_lnh_gr: 2,
        sltd_mxmm_gr_lmt_atvtn_rqst: 0,
        transmission_mode_11: 0,
        transmission_mode_12: 0,
        transmission_mode_13: 0,
        transmission_mode_14: 0,
        transmission_mode_15: 0,
        transmission_mode_16: 0,
        transmission_mode_17: 0,
        transmission_mode_18: 0,
        transmission_mode_19: 0,
        transmission_mode_20: 0,
        dsngg_dffrntl_lk_rqst_rr_axl_3: 0,
        trnsmssn_cst_md_dsl_rqst: 1,
        trnsmssn_explt_mnl_md_rqst: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.transmission.tc2_mode_9, 1);
    assert_eq!(state.transmission.tc2_output_shaft_brake_request, 1);
    assert_eq!(state.transmission.tc2_requested_reverse_launch_gear, 2);
    assert_eq!(state.transmission.tc2_coast_mode_disable_request, 1);
    assert_eq!(state.transmission.tc2_explicit_manual_mode_request, 1);
    assert_eq!(
        state.transmission.tc2_pre_defined_max_gear_activation_request,
        1
    );
}

#[test]
fn test_tc2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let has_tc2 = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), TC2::BASE_CAN_ID));
    assert!(has_tc2, "TC2 broadcast frame should be present");
}

// ============================================================================
// Physics Tests
// ============================================================================

#[test]
fn test_tc1_gear_shift_physics() {
    let mut state = test_state();
    // Request gear 5 via TC1
    let msg = TC1 {
        device_id: external_device(),
        trnsmssn_gr_shft_inht_rqst: 0,
        trnsmssn_trq_cnvrtr_lkp_rqst: 3,
        disengage_driveline_request: 0,
        trnsmssn_rvrs_gr_shft_inht_rqst: 0,
        requested_percent_clutch_slip: 0.0,
        transmission_requested_gear: 5.0,
        dsngg_dffrntl_lk_rqst_frnt_axl_1: 3,
        dsngg_dffrntl_lk_rqst_frnt_axl_2: 3,
        dsngg_dffrntl_lk_rqst_rr_axl_1: 3,
        dsngg_dffrntl_lk_rqst_rr_axl_2: 3,
        dsngg_dffrntl_lk_rqst_cntrl: 3,
        dsngg_dffrntl_lk_rqst_cntrl_frnt: 3,
        dsngg_dffrntl_lk_rqst_cntrl_rr: 3,
        trnsmssn_ld_rdtn_inht_rqst: 0,
        transmission_mode_1: 0,
        transmission_mode_2: 0,
        transmission_mode_3: 0,
        transmission_mode_4: 0,
        trnsmssn_at_ntrl_mnl_rtrn_rqst: 3,
        transmission_requested_launch_gear: 1,
        trnsmssn_shft_sltr_dspl_md_swth: 0,
        transmission_mode_5: 0,
        transmission_mode_6: 0,
        transmission_mode_7: 0,
        transmission_mode_8: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // ETC2 current gear starts at 3.0 (default) and should gradually shift toward 5.0
    let initial_gear = state.transmission.etc2_transmission_current_gear;

    // Run physics for small time step - shift should be in progress
    state.update_physics(0.5);
    let mid_gear = state.transmission.etc2_transmission_current_gear;

    // Gear should have moved toward the target
    assert!(
        (mid_gear - 5.0).abs() < (initial_gear - 5.0).abs(),
        "After physics update, current gear ({}) should be closer to 5.0 than initial ({})",
        mid_gear,
        initial_gear
    );

    // After enough time, gear should reach target
    for _ in 0..20 {
        state.update_physics(0.5);
    }
    assert_float_near(
        state.transmission.etc2_transmission_current_gear,
        5.0,
        0.2,
        "etc2_transmission_current_gear after shift complete",
    );

    // After shift completes, clutch actuator should be off
    assert_eq!(
        state.transmission.etc3_clutch_actuator, 0,
        "Clutch actuator should be off after shift completes"
    );
}

#[test]
fn test_etc14_clutch_temperature_physics() {
    let mut state = test_state();
    let initial_temp = state.transmission.etc14_clutch_1_temperature;

    // Under high engine load, clutch temperature should rise
    // Set engine_speed high so engine_load is computed high by physics
    state.engine.engine_speed = 2600.0; // Should give ~90% engine_load
    state.update_physics(1.0);
    let heated_temp = state.transmission.etc14_clutch_1_temperature;
    assert!(
        heated_temp > initial_temp,
        "Clutch temp ({}) should increase under load from initial ({})",
        heated_temp,
        initial_temp
    );

    // Under no load, clutch temperature should cool
    // Set engine_speed to idle so engine_load drops to ~0%
    state.engine.engine_speed = 800.0;
    state.motor.mg1_actual_speed = 0.0;
    state.motor.mg2_actual_speed = 0.0;
    for _ in 0..100 {
        state.update_physics(1.0);
    }
    let cooled_temp = state.transmission.etc14_clutch_1_temperature;
    assert!(
        cooled_temp < heated_temp,
        "Clutch temp ({}) should decrease with no load from heated ({})",
        cooled_temp,
        heated_temp
    );
}

#[test]
fn test_etc8_input_speed_follows_engine() {
    let mut state = test_state();
    state.engine.engine_speed = 2500.0;
    state.update_physics(0.1);
    // ETC8 input speed should follow engine speed
    assert_float_near(
        state.transmission.etc8_clutch_converter_input_speed,
        state.engine.engine_speed,
        10.0,
        "etc8_clutch_converter_input_speed should follow engine_speed",
    );
}

// ============================================================================
// Round-Trip Tests (encode -> decode -> verify state)
// ============================================================================

#[test]
fn test_etc3_round_trip() {
    let mut state = test_state();
    state.transmission.etc3_shift_finger_gear_position = 75.0;
    state.transmission.etc3_clutch_actuator = 1;
    let frames = state.generate_can_frames();
    let etc3_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), ETC3::BASE_CAN_ID))
        .expect("ETC3 frame should be in broadcast");

    let decoded = ETC3::decode(etc3_frame.raw_id(), etc3_frame.data()).unwrap();
    assert_float_near(
        decoded.trnsmssn_shft_fngr_gr_pstn,
        75.0,
        1.0,
        "round-trip etc3 gear position",
    );
    assert_eq!(decoded.transmission_clutch_actuator, 1);
}

#[test]
fn test_etcc4_round_trip() {
    let mut state = test_state();
    state.transmission.etcc4_wastegate_actuator_3_command = 80.0;
    state.transmission.etcc4_wastegate_actuator_4_command = 65.0;
    let frames = state.generate_can_frames();
    let etcc4_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), ETCC4::BASE_CAN_ID))
        .expect("ETCC4 frame should be in broadcast");

    let decoded = ETCC4::decode(etcc4_frame.raw_id(), etcc4_frame.data()).unwrap();
    assert_float_near(
        decoded.engn_trhrgr_wstgt_attr_3_cmmnd,
        80.0,
        0.1,
        "round-trip etcc4 wastegate 3",
    );
    assert_float_near(
        decoded.engn_trhrgr_wstgt_attr_4_cmmnd,
        65.0,
        0.1,
        "round-trip etcc4 wastegate 4",
    );
}

#[test]
fn test_tc1_round_trip() {
    let mut state = test_state();
    state.transmission.tc1_transmission_requested_gear = 4.0;
    state.transmission.tc1_requested_percent_clutch_slip = 15.0;
    state.transmission.tc1_mode_1 = 1;
    let frames = state.generate_can_frames();
    let tc1_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), TC1::BASE_CAN_ID))
        .expect("TC1 frame should be in broadcast");

    let decoded = TC1::decode(tc1_frame.raw_id(), tc1_frame.data()).unwrap();
    assert_float_near(
        decoded.transmission_requested_gear,
        4.0,
        1.0,
        "round-trip tc1 requested gear",
    );
    assert_float_near(
        decoded.requested_percent_clutch_slip,
        15.0,
        1.0,
        "round-trip tc1 clutch slip",
    );
    assert_eq!(decoded.transmission_mode_1, 1);
}

// ============================================================================
// Default State Verification
// ============================================================================

#[test]
fn test_default_state_values() {
    let state = SimulatorState::default();

    // ETC3 defaults
    assert_float_near(
        state.transmission.etc3_shift_finger_gear_position,
        50.0,
        0.1,
        "etc3 default gear position",
    );
    assert_eq!(state.transmission.etc3_shift_finger_neutral_indicator, 1); // Should be in neutral

    // ETC8 defaults
    assert_float_near(
        state.transmission.etc8_torque_converter_ratio,
        1.0,
        0.01,
        "etc8 default TC ratio",
    );
    assert_float_near(
        state.transmission.etc8_clutch_converter_input_speed,
        800.0,
        0.1,
        "etc8 default input speed",
    );

    // ETC14 defaults
    assert_float_near(
        state.transmission.etc14_clutch_1_temperature,
        85.0,
        0.1,
        "etc14 default clutch temp",
    );
    assert_eq!(state.transmission.etc14_clutch_1_overheat_indicator, 0);

    // TC1 defaults
    assert_float_near(
        state.transmission.tc1_transmission_requested_gear,
        0.0,
        0.1,
        "tc1 default requested gear",
    );

    // ETCC4 defaults
    assert_float_near(
        state.transmission.etcc4_wastegate_actuator_3_command,
        50.0,
        0.1,
        "etcc4 default wastegate 3",
    );
}

// ============================================================================
// All 14 Messages Broadcast Test
// ============================================================================

#[test]
fn test_all_batch2_messages_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();

    let message_ids: Vec<(&str, u32)> = vec![
        ("ETC3", ETC3::BASE_CAN_ID),
        ("ETC4", ETC4::BASE_CAN_ID),
        ("ETC7", ETC7::BASE_CAN_ID),
        ("ETC8", ETC8::BASE_CAN_ID),
        ("ETC10", ETC10::BASE_CAN_ID),
        ("ETC11", ETC11::BASE_CAN_ID),
        ("ETC12", ETC12::BASE_CAN_ID),
        ("ETC13", ETC13::BASE_CAN_ID),
        ("ETC14", ETC14::BASE_CAN_ID),
        ("ETC15", ETC15::BASE_CAN_ID),
        ("ETCC4", ETCC4::BASE_CAN_ID),
        ("ETCBI", ETCBI::BASE_CAN_ID),
        ("TC1", TC1::BASE_CAN_ID),
        ("TC2", TC2::BASE_CAN_ID),
    ];

    for (name, base_id) in &message_ids {
        let found = frames.iter().any(|f| matches_base_id(f.raw_id(), *base_id));
        assert!(
            found,
            "{} (BASE_CAN_ID=0x{:08X}) should be present in broadcast frames",
            name, base_id
        );
    }
}

// ============================================================================
// Self-Reception & DecodeFailed Tests
// ============================================================================

#[test]
fn test_etc3_self_reception_ignored() {
    let mut state = test_state();
    let msg = ETC3 {
        device_id: DeviceId::from(0x82),
        trnsmssn_shft_fngr_gr_pstn: 75.0,
        trnsmssn_shft_fngr_rl_pstn: 60.0,
        trnsmssn_shft_fngr_ntrl_indtr: 0,
        trnsmssn_shft_fngr_enggmnt_indtr: 1,
        trnsmssn_shft_fngr_cntr_rl_indtr: 0,
        trnsmssn_shft_fngr_rl_attr_1: 1,
        trnsmssn_shft_fngr_gr_attr_1: 1,
        trnsmssn_shft_fngr_rl_attr_2: 0,
        trnsmssn_shft_fngr_gr_attr_2: 0,
        transmission_range_high_actuator: 1,
        transmission_range_low_actuator: 0,
        trnsmssn_splttr_drt_attr: 1,
        trnsmssn_splttr_indrt_attr: 0,
        transmission_clutch_actuator: 1,
        trnsmssn_trq_cnvrtr_lkp_clth_attr: 0,
        transmission_defuel_actuator: 0,
        trnsmssn_inrt_brk_attr: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Ignored);
}

#[test]
fn test_batch2_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = ETC3::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
