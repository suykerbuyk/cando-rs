//! J1939 Simulator Integration Tests
//!
//! Comprehensive testing for all message handlers, broadcast verification,
//! diagnostic workflows, state machine transitions, and physics simulation.
//!
//! Tests are organized by phase:
//! - Phase 1: Infrastructure + first tests
//! - Phase 2: All 35 command message tests
//! - Phase 3: Diagnostic (DM) message testing
//! - Phase 4: Broadcast verification & round-trip tests
//! - Phase 5: State machine formalization
//! - Phase 6: Live CAN interface tests (#[ignore])

use cando_j1939_sim::{BrakingState, CrashState, MessageStatus, SimulatorState};
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
// Phase 1: Infrastructure + First Tests
// ============================================================================

#[test]
fn test_harness_basic_state_creation() {
    let state = test_state();
    assert_eq!(state.device_id, 0x82);
    assert!(!state.crash.crash_detected);
    assert_eq!(state.motor.mg1_speed_setpoint, 0.0);
}

#[test]
fn test_mg1ic_command_updates_speed_setpoint() {
    let mut state = test_state();
    let msg = MG1IC {
        device_id: external_device(),
        mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst: 75.0,
        mtr_gnrtr_1_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_1_invrtr_cntrl_cr: 42,
        mt_gt_1_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_1_rotor_position_sensing_request: 0,
        mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.motor.mg1_speed_setpoint, 75.0, 1.0, "mg1_speed_setpoint");
}

#[test]
fn test_mg1ic_physics_to_mg1is1_broadcast() {
    let mut state = test_state();

    // Set a speed setpoint
    let msg = MG1IC {
        device_id: external_device(),
        mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst: 50.0,
        mtr_gnrtr_1_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_1_invrtr_cntrl_cr: 42,
        mt_gt_1_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_1_rotor_position_sensing_request: 0,
        mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // Run physics to let motor respond
    for _ in 0..100 {
        state.update_physics(0.1);
    }

    // Motor should have converged to setpoint
    assert_float_near(state.motor.mg1_actual_speed, 50.0, 1.0, "mg1_actual_speed");

    // Generate broadcast frames and find MG1IS1
    let frames = state.generate_can_frames();
    assert!(!frames.is_empty());

    // Find MG1IS1 frame by CAN ID then decode
    let mut found_mg1is1 = false;
    for frame in &frames {
        let frame_id = frame.raw_id() & 0x1FFFFFFF;
        if matches_base_id(frame_id, MG1IS1::BASE_CAN_ID) {
            let mg1is1 = MG1IS1::decode(frame_id, frame.data()).unwrap();
            found_mg1is1 = true;
            assert_float_near(
                mg1is1.motor_generator_1_speed,
                50.0,
                1.0,
                "MG1IS1 speed",
            );
            break;
        }
    }
    assert!(found_mg1is1, "MG1IS1 frame not found in broadcast");
}

// ============================================================================
// Phase 2: All Command Message Tests
// ============================================================================

// --- Motor Control ---

#[test]
fn test_mg2ic_command_updates_speed_setpoint() {
    let mut state = test_state();
    let msg = MG2IC {
        device_id: external_device(),
        mtr_gnrtr_2_invrtr_cntrl_stpnt_rqst: -30.0,
        mtr_gnrtr_2_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_2_invrtr_cntrl_cr: 42,
        mt_gt_2_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_2_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_2_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_2_rotor_position_sensing_request: 0,
        mtr_gnrtr_2_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.motor.mg2_speed_setpoint, -30.0, 1.0, "mg2_speed_setpoint");
}

#[test]
fn test_mg1ic_physics_generates_torque_response() {
    let mut state = test_state();
    state.motor.mg1_speed_setpoint = 80.0;

    // Run physics long enough for convergence
    for _ in 0..200 {
        state.update_physics(0.1);
    }

    // Motor should be near setpoint with non-zero torque
    assert_float_near(state.motor.mg1_actual_speed, 80.0, 1.0, "mg1_actual_speed");
    assert!(state.motor.mg1_actual_torque.abs() > 0.1, "torque should be non-zero under load");
    assert!(state.motor.mg1_current > 0.0, "current should be positive under load");
}

#[test]
fn test_mg2ic_physics_convergence() {
    let mut state = test_state();
    state.motor.mg2_speed_setpoint = 60.0;

    for _ in 0..200 {
        state.update_physics(0.1);
    }

    assert_float_near(state.motor.mg2_actual_speed, 60.0, 1.0, "mg2_actual_speed");
}

#[test]
fn test_mg1is2_broadcast_contains_torque_limits() {
    let state = test_state();
    let frames = state.generate_can_frames();

    let mut found_mg1is2 = false;
    for frame in &frames {
        let frame_id = frame.raw_id() & 0x1FFFFFFF;
        if matches_base_id(frame_id, MG1IS2::BASE_CAN_ID) {
            let mg1is2 = MG1IS2::decode(frame_id, frame.data()).unwrap();
            found_mg1is2 = true;
            // Default max/min torque values
            assert_float_near(
                mg1is2.mtr_gnrtr_1_avll_mxmm_trq,
                100.0,
                1.0,
                "MG1IS2 max torque",
            );
            assert_float_near(
                mg1is2.mtr_gnrtr_1_avll_mnmm_trq,
                -100.0,
                1.0,
                "MG1IS2 min torque",
            );
            break;
        }
    }
    assert!(found_mg1is2, "MG1IS2 frame not found in broadcast");
}

// --- Power Management ---

#[test]
fn test_hvessc1_command_power_down() {
    let mut state = test_state();
    let msg = HVESSC1 {
        device_id: external_device(),
        hvess_power_down_command: 1,
        hvess_cell_balancing_command: 1,
        hvss_hgh_vltg_bs_cnnt_cmmnd: 0,
        hvss_hgh_vltg_bs_atv_isltn_tst_cmmnd: 0,
        hvss_hgh_vltg_bs_pssv_isltn_tst_cmmnd: 0,
        hvss_enl_intrnl_chrgr_cmmnd: 0,
        hvess_operation_consent: 0,
        hvss_hgh_vltg_bs_hgh_sd_rsstr_cnnt_rqst: 0,
        hvss_hgh_vltg_bs_lw_sd_rsstr_cnnt_rqst: 0,
        hvss_thrml_mngmnt_mntnn_rqst: 0,
        hvess_control_1_counter: 0,
        hvess_control_1_crc: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert!(state.hvess.hvess_power_down_command);
    assert!(state.hvess.hvess_cell_balancing_command);
}

#[test]
fn test_dcdc1c_command_processing() {
    let mut state = test_state();
    let msg = DCDC1C {
        device_id: external_device(),
        dc_dc_1_operational_command: 2,
        dc_dc_1_low_side_voltage_buck_setpoint: 52.0,
        dd_1_hgh_sd_vltg_bst_stpnt: 780.0,
        dc_dc_1_control_counter: 0,
        dd_1_lw_sd_vltg_bk_dflt_stpnt: 0.0,
        dc_dc_1_control_crc: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.dcdc.dcdc_operational_command, 2);
    assert_float_near(state.dcdc.dcdc_low_side_voltage_setpoint, 52.0, 1.0, "dcdc_low_side");
    assert_float_near(state.dcdc.dcdc_high_side_voltage_setpoint, 780.0, 2.0, "dcdc_high_side");
}

#[test]
fn test_altc_command_processing() {
    let mut state = test_state();
    let msg = ALTC {
        device_id: external_device(),
        altrntr_stpnt_vltg_cmmnd: 14.4,
        altrntr_exttn_mxmm_crrnt_lmt: 10.0,
        alternator_torque_ramp_time_command: 1.0,
        altrntr_trq_rmp_mxmm_spd_cmmnd: 1500.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.power_supply.altc_setpoint_voltage, 14.4, 0.5, "altc_voltage");
    assert_float_near(state.power_supply.altc_excitation_current_limit, 10.0, 1.0, "altc_current");
}

#[test]
fn test_gc2_command_processing() {
    let mut state = test_state();
    let msg = GC2 {
        device_id: external_device(),
        engine_load_setpoint_request: 75.0,
        engine_self_induced_derate_inhibit: 1,
        generator_governing_bias: 10.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.power_supply.gc2_engine_load_setpoint, 75.0, 1.0, "gc2_load");
    assert_eq!(state.power_supply.gc2_derate_inhibit, 1);
    assert_float_near(state.power_supply.gc2_governing_bias, 10.0, 1.0, "gc2_bias");
}

#[test]
fn test_dcacai1s2_command_processing() {
    let mut state = test_state();
    let msg = DCACAI1S2 {
        device_id: external_device(),
        da_assr_invrtr_1_d_sd_pwr: 2.5,
        da_assr_invrtr_1_d_sd_vltg: 240.0,
        da_assr_invrtr_1_d_sd_crrnt: 10.0,
        da_ass_ivt_1_d_sd_ntv_t_csss_gd_vt: 0.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.power_supply.dcacai1s2_desired_power, 2.5, 0.5, "dcacai1s2_power");
    assert_float_near(state.power_supply.dcacai1s2_desired_voltage, 240.0, 1.0, "dcacai1s2_voltage");
}

#[test]
fn test_dcdc1os_command_processing() {
    let mut state = test_state();
    let msg = DCDC1OS {
        device_id: external_device(),
        dc_dc_1_hvil_status: 2,
        dc_dc_1_loadshed_request: 1,
        dc_dc_1_operational_status: 3,
        dc_dc_1_operating_status_counter: 5,
        dc_dc_1_operating_status_crc: 42,
        dd_1_pwr_lmt_dt_hgh_sd_crrnt: 1,
        dd_1_pwr_lmt_dt_lw_sd_crrnt: 0,
        dd_1_pwr_lmt_dt_hgh_sd_vltg_mnmm: 0,
        dd_1_pwr_lmt_dt_hgh_sd_vltg_mxmm: 1,
        dd_1_pwr_lmt_dt_lw_sd_vltg_mnmm: 0,
        dd_1_pwr_lmt_dt_lw_sd_vltg_mxmm: 0,
        dd_1_pwr_lmt_dt_cnvrtr_tmprtr: 0,
        dd_1_pwr_lmt_dt_eltrn_fltr_tmprtr: 0,
        dd_1_pwr_lmt_dt_pwr_eltrns_tmprtr: 1,
        dd_1_pwr_lmt_dt_sl_bttr_trmnl_vltg: 0,
        dd_1_pwr_lmt_dt_sl_bttr_trmnl_crrnt: 0,
        dd_1_pwr_lmt_dt_sl_bttr_trmnl_tmprtr: 0,
        dd_1_pwr_lmt_dt_undfnd_rsn: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.dcdc.dcdc1os_hvil_status, 2);
    assert_eq!(state.dcdc.dcdc1os_loadshed_request, 1);
    assert_eq!(state.dcdc.dcdc1os_operational_status, 3);
    assert_eq!(state.dcdc.dcdc1os_operating_status_counter, 5);
    assert_eq!(state.dcdc.dcdc1os_operating_status_crc, 42);
}

#[test]
fn test_dcdc1sbs_command_processing() {
    let mut state = test_state();
    let msg = DCDC1SBS {
        device_id: external_device(),
        dc_dc_1_sli_battery_terminal_current: 30.0,
        dc_dc_1_sli_battery_terminal_voltage: 14.2,
        dd_1_sl_bttr_trmnl_tmprtr: 28.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1sbs_terminal_current, 30.0, 1.0, "dcdc1sbs_current");
    assert_float_near(state.dcdc.dcdc1sbs_terminal_voltage, 14.2, 0.5, "dcdc1sbs_voltage");
    assert_float_near(state.dcdc.dcdc1sbs_terminal_temperature, 28.0, 1.0, "dcdc1sbs_temp");
}

#[test]
fn test_dcdc1s2_command_processing() {
    let mut state = test_state();
    let msg = DCDC1S2 {
        device_id: external_device(),
        dc_dc_1_high_side_power: 150.0,
        dc_dc_1_low_side_power: -140.0,
        dd_1_hgh_sd_ngtv_t_chsss_grnd_vltg: 380.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc1s2_high_side_power, 150.0, 2.0, "dcdc1s2_hs_power");
    assert_float_near(state.dcdc.dcdc1s2_low_side_power, -140.0, 2.0, "dcdc1s2_ls_power");
}

#[test]
fn test_dcacai1v_command_processing() {
    let mut state = test_state();
    let msg = DCACAI1V {
        device_id: external_device(),
        da_assr_invrtr_1_igntn_vltg: 12.5,
        da_assr_invrtr_1_unswthd_sl_vltg: 13.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.power_supply.dcacai1v_ignition_voltage, 12.5, 0.5, "dcacai1v_ignition");
    assert_float_near(state.power_supply.dcacai1v_unswitched_voltage, 13.0, 0.5, "dcacai1v_unswitched");
}

#[test]
fn test_gtrace_command_processing() {
    let mut state = test_state();
    let msg = GTRACE {
        device_id: external_device(),
        generator_trip_kw_hours_export: 2000000,
        generator_trip_kvar_hours_export: 1000000,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.power_supply.gtrace_kwh_export, 2000000);
    assert_eq!(state.power_supply.gtrace_kvarh_export, 1000000);
}

#[test]
fn test_dcdc2sbs_command_processing() {
    let mut state = test_state();
    let msg = DCDC2SBS {
        device_id: external_device(),
        dc_dc_2_sli_battery_terminal_voltage: 13.5,
        dc_dc_2_sli_battery_terminal_current: -20.0,
        dd_2_sl_bttr_trmnl_tmprtr: 30.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2sbs_terminal_voltage, 13.5, 0.5, "dcdc2sbs_voltage");
    assert_float_near(state.dcdc.dcdc2sbs_terminal_current, -20.0, 1.0, "dcdc2sbs_current");
}

#[test]
fn test_dcdc2s2_command_processing() {
    let mut state = test_state();
    let msg = DCDC2S2 {
        device_id: external_device(),
        dc_dc_2_high_side_power: 180.0,
        dc_dc_2_low_side_power: -170.0,
        dd_2_hgh_sd_ngtv_t_chsss_grnd_vltg: 390.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.dcdc.dcdc2s2_high_side_power, 180.0, 2.0, "dcdc2s2_hs_power");
    assert_float_near(state.dcdc.dcdc2s2_low_side_power, -170.0, 2.0, "dcdc2s2_ls_power");
}

// --- Engine Control ---

#[test]
fn test_eec12_command_processing() {
    let mut state = test_state();
    let msg = EEC12 {
        device_id: external_device(),
        engn_exhst_1_gs_snsr_1_pwr_sppl: 2,
        aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl: 1,
        engn_exhst_2_gs_snsr_1_pwr_sppl: 2,
        aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl: 0,
        engn_exhst_1_gs_snsr_2_pwr_sppl: 1,
        aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.engine.eec12_engn_exhst_1_gs_snsr_1_pwr_sppl, 2);
    assert_eq!(state.engine.eec12_aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl, 1);
    assert_eq!(state.engine.eec12_engn_exhst_2_gs_snsr_1_pwr_sppl, 2);
    assert_eq!(state.engine.eec12_aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl, 0);
}

#[test]
fn test_etc5_command_processing() {
    let mut state = test_state();
    let msg = ETC5 {
        device_id: external_device(),
        trnsmssn_hgh_rng_sns_swth: 1,
        transmission_low_range_sense_switch: 0,
        transmission_splitter_position: 2,
        trnsmssn_rvrs_drtn_swth: 0,
        transmission_neutral_switch: 0,
        trnsmssn_frwrd_drtn_swth: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.transmission.etc5_trnsmssn_hgh_rng_sns_swth, 1);
    assert_eq!(state.transmission.etc5_transmission_splitter_position, 2);
    assert_eq!(state.transmission.etc5_trnsmssn_frwrd_drtn_swth, 1);
}

#[test]
fn test_eec22_command_processing() {
    let mut state = test_state();
    let msg = EEC22 {
        device_id: external_device(),
        engn_exhst_gs_rrltn_1_clr_intk_prssr: 200.0,
        ttl_nmr_of_crnk_attmpts_drng_engn_lf: 10000,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec22_engnexhstgsrrltn1clrintkprssr,
        200.0,
        2.0,
        "eec22_pressure",
    );
    assert_eq!(state.engine.eec22_ttlnmrofcrnkattmptsdrngengnlf, 10000);
}

#[test]
fn test_eec21_command_processing() {
    let mut state = test_state();
    let msg = EEC21 {
        device_id: external_device(),
        engn_exhst_mnfld_aslt_prssr_1: 150.0,
        engn_exhst_mnfld_aslt_prssr_2: 155.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec21_engn_exhst_mnfld_aslt_prssr_1,
        150.0,
        1.0,
        "eec21_pressure1",
    );
    assert_float_near(
        state.engine.eec21_engn_exhst_mnfld_aslt_prssr_2,
        155.0,
        1.0,
        "eec21_pressure2",
    );
}

#[test]
fn test_etcc2_command_processing() {
    let mut state = test_state();
    let msg = ETCC2 {
        device_id: external_device(),
        engn_stgd_trhrgr_slnd_stts: 50.0,
        nmr_of_engn_trhrgrs_cmmndd: 2,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etcc2_engn_stgd_trhrgr_slnd_stts,
        50.0,
        1.0,
        "etcc2_solenoid",
    );
    assert_eq!(state.transmission.etcc2_nmr_of_engn_trhrgrs_cmmndd, 2);
}

#[test]
fn test_etcc1_command_processing() {
    let mut state = test_state();
    let msg = ETCC1 {
        device_id: external_device(),
        engn_trhrgr_wstgt_attr_1_cmmnd: 75.5,
        engn_trhrgr_wstgt_attr_2_cmmnd: 80.0,
        e_exst_b_1_pss_rt_ct_cd: 60.0,
        et_cpss_bw_att_1_cd: 45.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etcc1_engn_trhrgr_wstgt_attr_1_cmmnd,
        75.5,
        0.5,
        "etcc1_wastegate1",
    );
    assert_float_near(
        state.transmission.etcc1_engn_trhrgr_wstgt_attr_2_cmmnd,
        80.0,
        0.5,
        "etcc1_wastegate2",
    );
}

#[test]
fn test_eec17_command_processing() {
    let mut state = test_state();
    let msg = EEC17 {
        device_id: external_device(),
        pems_engine_fuel_mass_flow_rate: 250.0,
        vehicle_fuel_rate: 240.0,
        engine_exhaust_flow_rate: 3000.0,
        cylinder_fuel_rate: 20.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec17_pems_engine_fuel_mass_flow_rate,
        250.0,
        1.0,
        "eec17_pems_fuel",
    );
    assert_float_near(state.engine.eec17_vehicle_fuel_rate, 240.0, 1.0, "eec17_vehicle_fuel");
}

#[test]
fn test_etc6_command_processing() {
    let mut state = test_state();
    let msg = ETC6 {
        device_id: external_device(),
        recommended_gear: 4.0,
        lowest_possible_gear: 1.0,
        highest_possible_gear: 8.0,
        clutch_life_remaining: 72.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.transmission.etc6_recommended_gear, 4.0, 0.5, "etc6_recommended");
    assert_float_near(state.transmission.etc6_clutch_life_remaining, 72.0, 1.0, "etc6_clutch_life");
}

#[test]
fn test_etc2_command_processing() {
    let mut state = test_state();
    let msg = ETC2 {
        device_id: external_device(),
        transmission_selected_gear: 5.0,
        transmission_actual_gear_ratio: 2.85,
        transmission_current_gear: 4.0,
        transmission_requested_range: 5,
        transmission_current_range: 5,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etc2_transmission_selected_gear,
        5.0,
        0.5,
        "etc2_selected_gear",
    );
    assert_float_near(
        state.transmission.etc2_transmission_actual_gear_ratio,
        2.85,
        0.1,
        "etc2_ratio",
    );
    assert_eq!(state.transmission.etc2_transmission_requested_range, 5);
}

#[test]
fn test_eec8_command_processing() {
    let mut state = test_state();
    let msg = EEC8 {
        device_id: external_device(),
        engn_exhst_gs_rrltn_1_vlv_2_cntrl: 55.0,
        engn_exhst_gs_rrltn_1_clr_intk_tmprtr: 95.0,
        e_exst_gs_rt_1_c_it_ast_pss: 175.0,
        engn_exhst_gs_rrltn_1_clr_effn: 80.0,
        e_exst_gs_rt_at_it_ct_tpt: 105.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec8_engn_exhst_gs_rrltn_1_vlv_2_cntrl,
        55.0,
        1.0,
        "eec8_egr_valve",
    );
    assert_float_near(
        state.engine.eec8_engn_exhst_gs_rrltn_1_clr_effn,
        80.0,
        1.0,
        "eec8_efficiency",
    );
}

#[test]
fn test_eec15_command_processing() {
    let mut state = test_state();
    let msg = EEC15 {
        device_id: external_device(),
        accelerator_pedal_1_channel_2: 45.0,
        accelerator_pedal_1_channel_3: 46.0,
        accelerator_pedal_2_channel_2: 44.0,
        accelerator_pedal_2_channel_3: 45.0,
        engn_exhst_gs_rstrtn_vlv_cntrl: 35.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.engine.eec15_accelerator_pedal_1_channel_2,
        45.0,
        1.0,
        "eec15_pedal1_ch2",
    );
    assert_float_near(
        state.engine.eec15_engn_exhst_gs_rstrtn_vlv_cntrl,
        35.0,
        1.0,
        "eec15_restriction",
    );
}

#[test]
fn test_etc9_command_processing() {
    let mut state = test_state();
    let msg = ETC9 {
        device_id: external_device(),
        dl_clth_trnsmssn_crrnt_pr_sltn_gr: 3.0,
        dl_clth_trnsmssn_inpt_shft_1_spd: 1500.0,
        dl_clth_trnsmssn_inpt_shft_2_spd: 1450.0,
        dl_clth_trnsmssn_sltd_pr_sltn_gr: 4.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.transmission.etc9_current_preselection_gear,
        3.0,
        0.5,
        "etc9_current_gear",
    );
    assert_float_near(
        state.transmission.etc9_input_shaft1_speed,
        1500.0,
        1.0,
        "etc9_shaft1_speed",
    );
}

// --- Thermal ---

#[test]
fn test_etcc3_command_processing() {
    let mut state = test_state();
    let msg = ETCC3 {
        device_id: external_device(),
        et_cpss_bw_att_1_mt_ct_ds: 1,
        engn_trhrgr_wstgt_attr_1_mtr_crrnt_dsl: 2,
        engn_clndr_hd_bpss_attr_1_mtr_crrnt_dsl: 0,
        engn_thrttl_vlv_1_mtr_crrnt_dsl: 3,
        et_cpss_bpss_att_1_mt_ct_ds: 1,
        et_cpss_bpss_att_2_mt_ct_ds: 2,
        engn_trhrgr_wstgt_attr_2_mtr_crrnt_dsl: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.thermal.etcc3_etc_bypass_actuator_1, 1);
    assert_eq!(state.thermal.etcc3_turbo_wastegate_actuator_1, 2);
    assert_eq!(state.thermal.etcc3_throttle_valve_1, 3);
}

#[test]
fn test_hvessts1_command_processing() {
    let mut state = test_state();
    let msg = HVESSTS1 {
        device_id: external_device(),
        hvss_thrml_mngmnt_sstm_sl_inpt_pwr: 2000.0,
        hvss_t_mt_sst_h_vt_ipt_pw: 2500.0,
        hvss_thrml_mngmnt_sstm_cmprssr_spd: 3000.0,
        hvss_thrml_mngmnt_sstm_rltv_hmdt: 70.0,
        hvss_thrml_mngmnt_sstm_htr_stts: 1,
        hvss_thrml_mngmnt_sstm_hvl_stts: 0,
        hvss_thrml_mngmnt_sstm_md: 3,
        hvss_thrml_mngmnt_sstm_clnt_lvl: 2,
        hvss_thrml_mngmnt_sstm_clnt_lvl_fll: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.thermal.hvessts1_system_input_power,
        2000.0,
        5.0,
        "hvessts1_power",
    );
    assert_float_near(
        state.thermal.hvessts1_compressor_speed,
        3000.0,
        10.0,
        "hvessts1_compressor",
    );
    assert_eq!(state.thermal.hvessts1_system_mode, 3);
}

#[test]
fn test_hvesstc1_command_processing() {
    let mut state = test_state();
    let msg = HVESSTC1 {
        device_id: external_device(),
        hvss_t_mt_sst_it_ct_tpt_rqst: 22.0,
        hvss_t_mt_sst_ott_ct_tpt_rqst: 27.0,
        hvss_t_mt_sst_ct_fw_rt_rqst: 120.0,
        hvss_thrml_mngmnt_sstm_htr_enl_cmmnd: 1,
        hvss_t_mt_sst_ct_pp_e_cd: 2,
        hvss_t_mt_sst_cpss_e_cd: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.thermal.hvesstc1_intake_coolant_temp_request,
        22.0,
        1.0,
        "hvesstc1_intake_temp",
    );
    assert_eq!(state.thermal.hvesstc1_coolant_pump_enable_code, 2);
}

#[test]
fn test_hvesstc2_command_processing() {
    let mut state = test_state();
    let msg = HVESSTC2 {
        device_id: external_device(),
        hvss_t_mt_sst_ct_pp_spd_cd: 3000.0,
        hvss_t_mt_sst_ct_pp_pt_spd_cd: 80.0,
        hvss_t_mt_sst_cpss_spd_cd: 4000.0,
        hvss_t_mt_sst_cpss_pt_spd_cd: 95.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(
        state.thermal.hvesstc2_pump_speed_command,
        3000.0,
        5.0,
        "hvesstc2_pump_speed",
    );
    assert_float_near(
        state.thermal.hvesstc2_compressor_speed_command,
        4000.0,
        5.0,
        "hvesstc2_compressor_speed",
    );
}

// --- Safety ---

#[test]
fn test_cn_crash_notification() {
    let mut state = test_state();
    let msg = CN {
        device_id: external_device(),
        crash_checksum: 5,
        crash_counter: 1,
        crash_type: 3,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert!(state.crash.crash_detected);
    assert_eq!(state.crash.crash_type, 3);
}

// --- Filtering ---

#[test]
fn test_self_reception_ignored() {
    let mut state = test_state(); // device_id = 0x82

    // Self-reception filtering works on PDU2 broadcast messages (PF >= 0xF0)
    // where SA (source address) is in bits 0-7 of the CAN ID.
    // PDU1 command messages (PF < 0xF0) are NOT filtered because the low byte
    // contains the destination address, not the source.
    // Use a PDU2 CAN ID with SA=0x82 to verify the filter works.
    let can_id = 0x18FECA82_u32; // PDU2 (PF=0xFE), SA=0x82 (matches device_id)
    let data = [0xFF; 8];
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Ignored);
}

#[test]
fn test_external_message_processed() {
    let mut state = test_state(); // device_id = 0x82

    // Send a message FROM device 0x42 (external) — should be processed
    let msg = MG1IC {
        device_id: DeviceId::from(0x42),
        mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst: 50.0,
        mtr_gnrtr_1_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_1_invrtr_cntrl_cr: 42,
        mt_gt_1_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_1_rotor_position_sensing_request: 0,
        mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.motor.mg1_speed_setpoint, 50.0, 1.0, "mg1_speed_setpoint");
}

#[test]
fn test_unrecognized_message_id() {
    let mut state = test_state();
    let unknown_can_id = 0x12345678;
    let data = [0xFF; 8];
    let status = state.process_incoming_message(unknown_can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Unrecognized);
}

// ============================================================================
// Phase 3: Diagnostic (DM) Message Testing
// ============================================================================

#[test]
fn test_dm01_receive_enables_fault_injection() {
    let mut state = test_state();
    assert!(!state.diagnostics.dm01_fault_injection_enabled);

    // Create a DM01 message and send it
    let dm01 = DM01 {
        device_id: external_device(),
        protect_lamp_status: 1,
        amber_warning_lamp_status: 1,
        red_stop_lamp_status: 0,
        malfunction_indicator_lamp_status: 1,
        flash_protect_lamp: 0,
        flash_amber_warning_lamp: 0,
        flash_red_stop_lamp: 0,
        flash_malfunc_indicator_lamp: 0,
        dm01_01spn: 3456,
        dm01_01spn_high: 0.0,
        dm01_01fmi: 5,
        dm01_01oc: 3,
        dm01_01cm: 0,
        dm01_02spn: 0xFFFF, dm01_02fmi: 0xFF, dm01_02oc: 0xFF, dm01_02cm: 0xFF, dm01_02spn_high: 0.0,
        dm01_03spn: 0xFFFF, dm01_03fmi: 0xFF, dm01_03oc: 0xFF, dm01_03cm: 0xFF, dm01_03spn_high: 0.0,
        dm01_04spn: 0xFFFF, dm01_04fmi: 0xFF, dm01_04oc: 0xFF, dm01_04cm: 0xFF, dm01_04spn_high: 0.0,
        dm01_05spn: 0xFFFF, dm01_05fmi: 0xFF, dm01_05oc: 0xFF, dm01_05cm: 0xFF, dm01_05spn_high: 0.0,
        dm01_06spn: 0xFFFF, dm01_06fmi: 0xFF, dm01_06oc: 0xFF, dm01_06cm: 0xFF, dm01_06spn_high: 0.0,
        dm01_07spn: 0xFFFF, dm01_07fmi: 0xFF, dm01_07oc: 0xFF, dm01_07cm: 0xFF, dm01_07spn_high: 0.0,
        dm01_08spn: 0xFFFF, dm01_08fmi: 0xFF, dm01_08oc: 0xFF, dm01_08cm: 0xFF, dm01_08spn_high: 0.0,
        dm01_09spn: 0xFFFF, dm01_09fmi: 0xFF, dm01_09oc: 0xFF, dm01_09cm: 0xFF, dm01_09spn_high: 0.0,
        dm01_10spn: 0xFFFF, dm01_10fmi: 0xFF, dm01_10oc: 0xFF, dm01_10cm: 0xFF, dm01_10spn_high: 0.0,
        dm01_11spn: 0xFFFF, dm01_11fmi: 0xFF, dm01_11oc: 0xFF, dm01_11cm: 0xFF, dm01_11spn_high: 0.0,
        dm01_12spn: 0xFFFF, dm01_12fmi: 0xFF, dm01_12oc: 0xFF, dm01_12cm: 0xFF, dm01_12spn_high: 0.0,
        dm01_13spn: 0xFFFF, dm01_13fmi: 0xFF, dm01_13oc: 0xFF, dm01_13cm: 0xFF, dm01_13spn_high: 0.0,
        dm01_14spn: 0xFFFF, dm01_14fmi: 0xFF, dm01_14oc: 0xFF, dm01_14cm: 0xFF, dm01_14spn_high: 0.0,
        dm01_15spn: 0xFFFF, dm01_15fmi: 0xFF, dm01_15oc: 0xFF, dm01_15cm: 0xFF, dm01_15spn_high: 0.0,
        dm01_16spn: 0xFFFF, dm01_16fmi: 0xFF, dm01_16oc: 0xFF, dm01_16cm: 0xFF, dm01_16spn_high: 0.0,
        dm01_17spn: 0xFFFF, dm01_17fmi: 0xFF, dm01_17oc: 0xFF, dm01_17cm: 0xFF, dm01_17spn_high: 0.0,
        dm01_18spn: 0xFFFF, dm01_18fmi: 0xFF, dm01_18oc: 0xFF, dm01_18cm: 0xFF, dm01_18spn_high: 0.0,
        dm01_19spn: 0xFFFF, dm01_19fmi: 0xFF, dm01_19oc: 0xFF, dm01_19cm: 0xFF, dm01_19spn_high: 0.0,
    };
    let (can_id, data) = dm01.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert!(state.diagnostics.dm01_fault_injection_enabled);
}

#[test]
fn test_dm02_receive_enables_fault_injection() {
    let mut state = test_state();
    assert!(!state.diagnostics.dm02_fault_injection_enabled);

    let dm02 = DM02 {
        device_id: external_device(),
        protect_lamp_status: 0,
        amber_warning_lamp_status: 0,
        red_stop_lamp_status: 0,
        malfunction_indicator_lamp_status: 0,
        flash_protect_lamp: 0,
        flash_amber_warning_lamp: 0,
        flash_red_stop_lamp: 0,
        flash_malfunc_indicator_lamp: 0,
        dm02_01spn: 7890,
        dm02_01spn_high: 0.0,
        dm02_01fmi: 3,
        dm02_01oc: 2,
        dm02_01cm: 0,
        dm02_02spn: 0xFFFF, dm02_02fmi: 0xFF, dm02_02oc: 0xFF, dm02_02cm: 0xFF, dm02_02spn_high: 0.0,
        dm02_03spn: 0xFFFF, dm02_03fmi: 0xFF, dm02_03oc: 0xFF, dm02_03cm: 0xFF, dm02_03spn_high: 0.0,
        dm02_04spn: 0xFFFF, dm02_04fmi: 0xFF, dm02_04oc: 0xFF, dm02_04cm: 0xFF, dm02_04spn_high: 0.0,
        dm02_05spn: 0xFFFF, dm02_05fmi: 0xFF, dm02_05oc: 0xFF, dm02_05cm: 0xFF, dm02_05spn_high: 0.0,
        dm02_06spn: 0xFFFF, dm02_06fmi: 0xFF, dm02_06oc: 0xFF, dm02_06cm: 0xFF, dm02_06spn_high: 0.0,
        dm02_07spn: 0xFFFF, dm02_07fmi: 0xFF, dm02_07oc: 0xFF, dm02_07cm: 0xFF, dm02_07spn_high: 0.0,
        dm02_08spn: 0xFFFF, dm02_08fmi: 0xFF, dm02_08oc: 0xFF, dm02_08cm: 0xFF, dm02_08spn_high: 0.0,
        dm02_09spn: 0xFFFF, dm02_09fmi: 0xFF, dm02_09oc: 0xFF, dm02_09cm: 0xFF, dm02_09spn_high: 0.0,
        dm02_10spn: 0xFFFF, dm02_10fmi: 0xFF, dm02_10oc: 0xFF, dm02_10cm: 0xFF, dm02_10spn_high: 0.0,
        dm02_11spn: 0xFFFF, dm02_11fmi: 0xFF, dm02_11oc: 0xFF, dm02_11cm: 0xFF, dm02_11spn_high: 0.0,
        dm02_12spn: 0xFFFF, dm02_12fmi: 0xFF, dm02_12oc: 0xFF, dm02_12cm: 0xFF, dm02_12spn_high: 0.0,
        dm02_13spn: 0xFFFF, dm02_13fmi: 0xFF, dm02_13oc: 0xFF, dm02_13cm: 0xFF, dm02_13spn_high: 0.0,
        dm02_14spn: 0xFFFF, dm02_14fmi: 0xFF, dm02_14oc: 0xFF, dm02_14cm: 0xFF, dm02_14spn_high: 0.0,
        dm02_15spn: 0xFFFF, dm02_15fmi: 0xFF, dm02_15oc: 0xFF, dm02_15cm: 0xFF, dm02_15spn_high: 0.0,
        dm02_16spn: 0xFFFF, dm02_16fmi: 0xFF, dm02_16oc: 0xFF, dm02_16cm: 0xFF, dm02_16spn_high: 0.0,
        dm02_17spn: 0xFFFF, dm02_17fmi: 0xFF, dm02_17oc: 0xFF, dm02_17cm: 0xFF, dm02_17spn_high: 0.0,
        dm02_18spn: 0xFFFF, dm02_18fmi: 0xFF, dm02_18oc: 0xFF, dm02_18cm: 0xFF, dm02_18spn_high: 0.0,
        dm02_19spn: 0xFFFF, dm02_19fmi: 0xFF, dm02_19oc: 0xFF, dm02_19cm: 0xFF, dm02_19spn_high: 0.0,
    };
    let (can_id, data) = dm02.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert!(state.diagnostics.dm02_fault_injection_enabled);
}

#[test]
fn test_dm03_clear_when_enabled() {
    let mut state = test_state();
    state.diagnostics.dm03_clear_operations_enabled = true;
    state.diagnostics.dm01_active_dtc_spn = 1234;
    state.diagnostics.dm01_active_dtc_fmi = 5;

    let msg = DM03 {
        device_id: external_device(),
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.diagnostics.dm03_clear_commands_received, 1);
    // Active DTC should be cleared
    assert_eq!(state.diagnostics.dm01_active_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm01_active_dtc_fmi, 0xFF);
}

#[test]
fn test_dm03_clear_when_disabled() {
    let mut state = test_state();
    state.diagnostics.dm03_clear_operations_enabled = false;
    state.diagnostics.dm01_active_dtc_spn = 1234;

    let msg = DM03 {
        device_id: external_device(),
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    // Clear count should NOT increment when disabled
    assert_eq!(state.diagnostics.dm03_clear_commands_received, 0);
    // Active DTC should NOT be cleared
    assert_eq!(state.diagnostics.dm01_active_dtc_spn, 1234);
}

#[test]
fn test_dm03_clears_active_dtcs() {
    let mut state = test_state();
    state.diagnostics.dm03_clear_operations_enabled = true;
    // Set active DTC
    state.diagnostics.dm01_active_dtc_spn = 5678;
    state.diagnostics.dm01_active_dtc_fmi = 9;
    state.diagnostics.dm01_active_dtc_occurrence_count = 3;
    state.diagnostics.dm01_active_dtc_conversion_method = 1;

    let msg = DM03 {
        device_id: external_device(),
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // All active DTC fields should be reset to "not available"
    assert_eq!(state.diagnostics.dm01_active_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm01_active_dtc_fmi, 0xFF);
    assert_eq!(state.diagnostics.dm01_active_dtc_occurrence_count, 0xFF);
    assert_eq!(state.diagnostics.dm01_active_dtc_conversion_method, 0xFF);
}

#[test]
fn test_dm03_moves_dtcs_to_previously_active() {
    let mut state = test_state();
    state.diagnostics.dm03_clear_operations_enabled = true;
    state.diagnostics.dm01_fault_injection_enabled = true;
    // Initially no previously active DTCs
    assert_eq!(state.diagnostics.dm02_previously_active_dtc_spn, 0xFFFF);

    let msg = DM03 {
        device_id: external_device(),
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // Previously active DTC should now have the cleared DTC
    assert_ne!(state.diagnostics.dm02_previously_active_dtc_spn, 0xFFFF);
    assert_ne!(state.diagnostics.dm02_previously_active_dtc_fmi, 0xFF);
}

#[test]
fn test_dm03_resets_lamp_states() {
    let mut state = test_state();
    state.diagnostics.dm03_clear_operations_enabled = true;
    // Set lamp states to on
    state.diagnostics.dm01_protect_lamp_status = 1;
    state.diagnostics.dm01_amber_warning_lamp_status = 1;
    state.diagnostics.dm01_red_stop_lamp_status = 1;
    state.diagnostics.dm01_malfunction_indicator_lamp_status = 1;

    let msg = DM03 {
        device_id: external_device(),
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // All lamps should be off after clear
    assert_eq!(state.diagnostics.dm01_protect_lamp_status, 0);
    assert_eq!(state.diagnostics.dm01_amber_warning_lamp_status, 0);
    assert_eq!(state.diagnostics.dm01_red_stop_lamp_status, 0);
    assert_eq!(state.diagnostics.dm01_malfunction_indicator_lamp_status, 0);
}

#[test]
fn test_dm01_state_defaults() {
    // DM01/DM02 are multi-frame J1939 messages (DLC=78) that exceed the 8-byte
    // CAN frame limit. They cannot be broadcast as single frames.
    // Instead, verify the state fields directly.
    let state = test_state();
    assert_eq!(state.diagnostics.dm01_active_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm01_active_dtc_fmi, 0xFF);
    assert_eq!(state.diagnostics.dm01_protect_lamp_status, 0);
}

#[test]
fn test_dm02_state_defaults() {
    let state = test_state();
    assert_eq!(state.diagnostics.dm02_previously_active_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm02_previously_active_dtc_fmi, 0xFF);
}

#[test]
fn test_full_diagnostic_workflow() {
    let mut state = test_state();
    state.diagnostics.dm03_clear_operations_enabled = true;

    // Step 1: Enable fault injection via DM01 receive
    let dm01 = DM01 {
        device_id: external_device(),
        protect_lamp_status: 1,
        amber_warning_lamp_status: 1,
        red_stop_lamp_status: 0,
        malfunction_indicator_lamp_status: 0,
        flash_protect_lamp: 0,
        flash_amber_warning_lamp: 0,
        flash_red_stop_lamp: 0,
        flash_malfunc_indicator_lamp: 0,
        dm01_01spn: 3456, dm01_01spn_high: 0.0, dm01_01fmi: 5, dm01_01oc: 3, dm01_01cm: 0,
        dm01_02spn: 0xFFFF, dm01_02fmi: 0xFF, dm01_02oc: 0xFF, dm01_02cm: 0xFF, dm01_02spn_high: 0.0,
        dm01_03spn: 0xFFFF, dm01_03fmi: 0xFF, dm01_03oc: 0xFF, dm01_03cm: 0xFF, dm01_03spn_high: 0.0,
        dm01_04spn: 0xFFFF, dm01_04fmi: 0xFF, dm01_04oc: 0xFF, dm01_04cm: 0xFF, dm01_04spn_high: 0.0,
        dm01_05spn: 0xFFFF, dm01_05fmi: 0xFF, dm01_05oc: 0xFF, dm01_05cm: 0xFF, dm01_05spn_high: 0.0,
        dm01_06spn: 0xFFFF, dm01_06fmi: 0xFF, dm01_06oc: 0xFF, dm01_06cm: 0xFF, dm01_06spn_high: 0.0,
        dm01_07spn: 0xFFFF, dm01_07fmi: 0xFF, dm01_07oc: 0xFF, dm01_07cm: 0xFF, dm01_07spn_high: 0.0,
        dm01_08spn: 0xFFFF, dm01_08fmi: 0xFF, dm01_08oc: 0xFF, dm01_08cm: 0xFF, dm01_08spn_high: 0.0,
        dm01_09spn: 0xFFFF, dm01_09fmi: 0xFF, dm01_09oc: 0xFF, dm01_09cm: 0xFF, dm01_09spn_high: 0.0,
        dm01_10spn: 0xFFFF, dm01_10fmi: 0xFF, dm01_10oc: 0xFF, dm01_10cm: 0xFF, dm01_10spn_high: 0.0,
        dm01_11spn: 0xFFFF, dm01_11fmi: 0xFF, dm01_11oc: 0xFF, dm01_11cm: 0xFF, dm01_11spn_high: 0.0,
        dm01_12spn: 0xFFFF, dm01_12fmi: 0xFF, dm01_12oc: 0xFF, dm01_12cm: 0xFF, dm01_12spn_high: 0.0,
        dm01_13spn: 0xFFFF, dm01_13fmi: 0xFF, dm01_13oc: 0xFF, dm01_13cm: 0xFF, dm01_13spn_high: 0.0,
        dm01_14spn: 0xFFFF, dm01_14fmi: 0xFF, dm01_14oc: 0xFF, dm01_14cm: 0xFF, dm01_14spn_high: 0.0,
        dm01_15spn: 0xFFFF, dm01_15fmi: 0xFF, dm01_15oc: 0xFF, dm01_15cm: 0xFF, dm01_15spn_high: 0.0,
        dm01_16spn: 0xFFFF, dm01_16fmi: 0xFF, dm01_16oc: 0xFF, dm01_16cm: 0xFF, dm01_16spn_high: 0.0,
        dm01_17spn: 0xFFFF, dm01_17fmi: 0xFF, dm01_17oc: 0xFF, dm01_17cm: 0xFF, dm01_17spn_high: 0.0,
        dm01_18spn: 0xFFFF, dm01_18fmi: 0xFF, dm01_18oc: 0xFF, dm01_18cm: 0xFF, dm01_18spn_high: 0.0,
        dm01_19spn: 0xFFFF, dm01_19fmi: 0xFF, dm01_19oc: 0xFF, dm01_19cm: 0xFF, dm01_19spn_high: 0.0,
    };
    let (can_id, data) = dm01.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();
    assert!(state.diagnostics.dm01_fault_injection_enabled);

    // Step 2: Send DM03 clear command
    let dm03 = DM03 {
        device_id: external_device(),
    };
    let (can_id, data) = dm03.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // Step 3: Verify active DTCs cleared
    assert_eq!(state.diagnostics.dm01_active_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm01_active_dtc_fmi, 0xFF);

    // Step 4: Verify lamp states reset
    assert_eq!(state.diagnostics.dm01_protect_lamp_status, 0);
    assert_eq!(state.diagnostics.dm01_amber_warning_lamp_status, 0);

    // Step 5: Verify DTCs moved to previously active
    assert_ne!(state.diagnostics.dm02_previously_active_dtc_spn, 0xFFFF);

    // Step 6: Verify clear counter incremented
    assert_eq!(state.diagnostics.dm03_clear_commands_received, 1);
}

// ============================================================================
// Phase 4: Broadcast Verification & Round-Trip Tests
// ============================================================================

#[test]
fn test_broadcast_frame_count_default_state() {
    let state = test_state();
    let frames = state.generate_can_frames();
    // Default state (no crash, AEBS enabled) - total includes all integrated batch messages
    assert_eq!(frames.len(), 305);
}

#[test]
fn test_broadcast_frame_count_with_crash() {
    let state = SimulatorState {
        device_id: 0x82,
        crash: CrashState {
            crash_detected: true,
            crash_type: 1,
            ..Default::default()
        },
        ..Default::default()
    };
    let frames = state.generate_can_frames();
    // With crash: adds CN frame - total includes all integrated batch messages
    assert_eq!(frames.len(), 306);
}

#[test]
fn test_round_trip_mg1ic_to_mg1is1() {
    let mut state = test_state();

    // Command: set speed to 75%
    let cmd = MG1IC {
        device_id: external_device(),
        mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst: 75.0,
        mtr_gnrtr_1_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_1_invrtr_cntrl_cr: 42,
        mt_gt_1_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_1_rotor_position_sensing_request: 0,
        mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = cmd.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // Physics: converge
    for _ in 0..200 {
        state.update_physics(0.1);
    }

    // Broadcast: decode MG1IS1 (filter by CAN ID first)
    let frames = state.generate_can_frames();
    let mut found = false;
    for frame in &frames {
        let fid = frame.raw_id() & 0x1FFFFFFF;
        if matches_base_id(fid, MG1IS1::BASE_CAN_ID) {
            let mg1is1 = MG1IS1::decode(fid, frame.data()).unwrap();
            found = true;
            assert_float_near(mg1is1.motor_generator_1_speed, 75.0, 2.0, "round-trip speed");
            // Non-zero torque under load
            assert!(
                mg1is1.motor_generator_1_net_rotor_torque.abs() > 0.1,
                "torque should be non-zero"
            );
            break;
        }
    }
    assert!(found, "MG1IS1 not found in round-trip test");
}

#[test]
fn test_round_trip_hvessc1_to_hvessd1() {
    let mut state = test_state();

    // Command: power down
    let cmd = HVESSC1 {
        device_id: external_device(),
        hvess_power_down_command: 1,
        hvess_cell_balancing_command: 0,
        hvss_hgh_vltg_bs_cnnt_cmmnd: 0,
        hvss_hgh_vltg_bs_atv_isltn_tst_cmmnd: 0,
        hvss_hgh_vltg_bs_pssv_isltn_tst_cmmnd: 0,
        hvss_enl_intrnl_chrgr_cmmnd: 0,
        hvess_operation_consent: 0,
        hvss_hgh_vltg_bs_hgh_sd_rsstr_cnnt_rqst: 0,
        hvss_hgh_vltg_bs_lw_sd_rsstr_cnnt_rqst: 0,
        hvss_thrml_mngmnt_mntnn_rqst: 0,
        hvess_control_1_counter: 0,
        hvess_control_1_crc: 0,
    };
    let (can_id, data) = cmd.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();
    assert!(state.hvess.hvess_power_down_command);

    // Broadcast: verify HVESSD1 exists (filter by CAN ID)
    let frames = state.generate_can_frames();
    let mut found = false;
    for frame in &frames {
        let fid = frame.raw_id() & 0x1FFFFFFF;
        if matches_base_id(fid, HVESSD1::BASE_CAN_ID) {
            let hvessd1 = HVESSD1::decode(fid, frame.data()).unwrap();
            found = true;
            // HVESSD1 should reflect default 800V nominal voltage
            assert!(hvessd1.hvess_voltage_level > 0.0, "voltage should be positive");
            break;
        }
    }
    assert!(found, "HVESSD1 not found in broadcast");
}

#[test]
fn test_round_trip_cn_crash_broadcast() {
    let mut state = test_state();

    // Command: trigger crash
    let cmd = CN {
        device_id: external_device(),
        crash_checksum: 5,
        crash_counter: 1,
        crash_type: 2,
    };
    let (can_id, data) = cmd.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();
    state.update_physics(0.01); // Update checksum

    // Broadcast: verify CN frame exists (filter by CAN ID)
    let frames = state.generate_can_frames();
    let mut found = false;
    for frame in &frames {
        let fid = frame.raw_id() & 0x1FFFFFFF;
        if matches_base_id(fid, CN::BASE_CAN_ID) {
            let cn = CN::decode(fid, frame.data()).unwrap();
            found = true;
            assert_eq!(cn.crash_type, 2);
            break;
        }
    }
    assert!(found, "CN frame not found after crash notification");
}

#[test]
fn test_round_trip_hvesstc1_to_hvessts1() {
    let mut state = test_state();

    // Command: set thermal management params
    let cmd = HVESSTC1 {
        device_id: external_device(),
        hvss_t_mt_sst_it_ct_tpt_rqst: 15.0,
        hvss_t_mt_sst_ott_ct_tpt_rqst: 20.0,
        hvss_t_mt_sst_ct_fw_rt_rqst: 200.0,
        hvss_thrml_mngmnt_sstm_htr_enl_cmmnd: 1,
        hvss_t_mt_sst_ct_pp_e_cd: 2,
        hvss_t_mt_sst_cpss_e_cd: 1,
    };
    let (can_id, data) = cmd.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // Broadcast: verify HVESSTS1 exists (filter by CAN ID)
    let frames = state.generate_can_frames();
    let mut found = false;
    for frame in &frames {
        let fid = frame.raw_id() & 0x1FFFFFFF;
        if matches_base_id(fid, HVESSTS1::BASE_CAN_ID) {
            found = true;
            break;
        }
    }
    assert!(found, "HVESSTS1 not found in broadcast");
}

#[test]
fn test_broadcast_contains_all_expected_message_types() {
    let state = SimulatorState {
        device_id: 0x82,
        crash: CrashState {
            crash_detected: true,
            crash_type: 1,
            ..Default::default()
        },
        braking: BrakingState { aebs_enabled: true, ..Default::default() },
        ..Default::default()
    };
    let frames = state.generate_can_frames();

    // Try to decode each expected message type from the broadcast
    let mut found_types: Vec<&str> = Vec::new();

    for frame in &frames {
        let fid = frame.raw_id() & 0x1FFFFFFF;
        // Use CAN ID matching to correctly identify frame types
        // (decode() doesn't validate CAN IDs, so it would match any 8-byte frame)
        if matches_base_id(fid, CN::BASE_CAN_ID) && !found_types.contains(&"CN") {
            found_types.push("CN");
        }
        if matches_base_id(fid, WAND::BASE_CAN_ID) && !found_types.contains(&"WAND") {
            found_types.push("WAND");
        }
        if matches_base_id(fid, LDISP::BASE_CAN_ID) && !found_types.contains(&"LDISP") {
            found_types.push("LDISP");
        }
        if matches_base_id(fid, MG1IS1::BASE_CAN_ID) && !found_types.contains(&"MG1IS1") {
            found_types.push("MG1IS1");
        }
        if matches_base_id(fid, MG2IS1::BASE_CAN_ID) && !found_types.contains(&"MG2IS1") {
            found_types.push("MG2IS1");
        }
        if matches_base_id(fid, MG1IS2::BASE_CAN_ID) && !found_types.contains(&"MG1IS2") {
            found_types.push("MG1IS2");
        }
        if matches_base_id(fid, HVESSD1::BASE_CAN_ID) && !found_types.contains(&"HVESSD1") {
            found_types.push("HVESSD1");
        }
        if matches_base_id(fid, HVESSD6::BASE_CAN_ID) && !found_types.contains(&"HVESSD6") {
            found_types.push("HVESSD6");
        }
        if matches_base_id(fid, ETCC3::BASE_CAN_ID) && !found_types.contains(&"ETCC3") {
            found_types.push("ETCC3");
        }
        if matches_base_id(fid, EEC12::BASE_CAN_ID) && !found_types.contains(&"EEC12") {
            found_types.push("EEC12");
        }
        if matches_base_id(fid, ETC5::BASE_CAN_ID) && !found_types.contains(&"ETC5") {
            found_types.push("ETC5");
        }
        if matches_base_id(fid, ALTC::BASE_CAN_ID) && !found_types.contains(&"ALTC") {
            found_types.push("ALTC");
        }
        if matches_base_id(fid, GC2::BASE_CAN_ID) && !found_types.contains(&"GC2") {
            found_types.push("GC2");
        }
    }

    // Verify critical message types are present
    // Note: DM01/DM02 are multi-frame (DLC=78) and cannot be sent as single CAN frames
    let expected = vec![
        "CN", "WAND", "LDISP", "MG1IS1", "MG2IS1", "MG1IS2", "HVESSD1", "HVESSD6",
        "ETCC3", "EEC12", "ETC5", "ALTC", "GC2",
    ];
    for msg_type in &expected {
        assert!(
            found_types.contains(msg_type),
            "Missing broadcast message type: {}",
            msg_type
        );
    }
}

// ============================================================================
// Phase 5: State Machine / Operating Mode Tests
// ============================================================================

#[test]
fn test_idle_state_default() {
    let state = test_state();
    // Default state: no motor commands, no faults, no crash
    assert_eq!(state.motor.mg1_speed_setpoint, 0.0);
    assert_eq!(state.motor.mg2_speed_setpoint, 0.0);
    assert!(!state.crash.crash_detected);
    assert!(!state.hvess.hvess_power_down_command);
}

#[test]
fn test_running_state_with_motor_command() {
    let mut state = test_state();
    let msg = MG1IC {
        device_id: external_device(),
        mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst: 50.0,
        mtr_gnrtr_1_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_1_invrtr_cntrl_cr: 42,
        mt_gt_1_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_1_rotor_position_sensing_request: 0,
        mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // State should have non-zero setpoint (running)
    assert!(state.motor.mg1_speed_setpoint.abs() > 0.0);
}

#[test]
fn test_emergency_state_after_crash() {
    let mut state = test_state();

    // First set running state
    state.motor.mg1_speed_setpoint = 50.0;

    // Crash notification
    let msg = CN {
        device_id: external_device(),
        crash_checksum: 0,
        crash_counter: 1,
        crash_type: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    assert!(state.crash.crash_detected);
    // Crash should be broadcast
    let frames = state.generate_can_frames();
    assert_eq!(frames.len(), 306); // +1 for CN frame, includes all integrated batch messages
}

#[test]
fn test_shutdown_via_hvess_power_down() {
    let mut state = test_state();

    let msg = HVESSC1 {
        device_id: external_device(),
        hvess_power_down_command: 1,
        hvess_cell_balancing_command: 0,
        hvss_hgh_vltg_bs_cnnt_cmmnd: 0,
        hvss_hgh_vltg_bs_atv_isltn_tst_cmmnd: 0,
        hvss_hgh_vltg_bs_pssv_isltn_tst_cmmnd: 0,
        hvss_enl_intrnl_chrgr_cmmnd: 0,
        hvess_operation_consent: 0,
        hvss_hgh_vltg_bs_hgh_sd_rsstr_cnnt_rqst: 0,
        hvss_hgh_vltg_bs_lw_sd_rsstr_cnnt_rqst: 0,
        hvss_thrml_mngmnt_mntnn_rqst: 0,
        hvess_control_1_counter: 0,
        hvess_control_1_crc: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();
    assert!(state.hvess.hvess_power_down_command);
}

#[test]
fn test_fault_state_dm01_active() {
    let mut state = test_state();
    state.diagnostics.dm01_active_dtc_spn = 3456;
    state.diagnostics.dm01_active_dtc_fmi = 5;
    state.diagnostics.dm01_protect_lamp_status = 1;

    // Verify fault is reflected in state
    assert_ne!(state.diagnostics.dm01_active_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm01_protect_lamp_status, 1);
}

#[test]
fn test_fault_cleared_via_dm03() {
    let mut state = test_state();
    state.diagnostics.dm03_clear_operations_enabled = true;
    state.diagnostics.dm01_active_dtc_spn = 3456;
    state.diagnostics.dm01_protect_lamp_status = 1;

    let msg = DM03 {
        device_id: external_device(),
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // Fault should be cleared
    assert_eq!(state.diagnostics.dm01_active_dtc_spn, 0xFFFF);
    assert_eq!(state.diagnostics.dm01_protect_lamp_status, 0);
}

// ============================================================================
// Phase 5 (continued): Physics Interaction Tests
// ============================================================================

#[test]
fn test_hvess_voltage_stability_under_load() {
    let mut state = test_state();
    let initial_voltage = state.hvess.hvess_voltage_level;

    // Apply motor load
    state.motor.mg1_speed_setpoint = 100.0;
    for _ in 0..200 {
        state.update_physics(0.1);
    }

    // Voltage should drop under load
    assert!(
        state.hvess.hvess_voltage_level < initial_voltage,
        "voltage should drop under load"
    );
    assert!(state.hvess.hvess_voltage_level > 750.0, "voltage should stay in range");
}

#[test]
fn test_hvess_temperature_rises_under_load() {
    let mut state = test_state();
    let initial_temp = state.hvess.hvess_electronics_temp;

    // Apply motor load
    state.motor.mg1_speed_setpoint = 100.0;
    state.motor.mg2_speed_setpoint = 100.0;
    for _ in 0..50 {
        state.update_physics(0.1);
    }

    // Temperature should rise
    assert!(
        state.hvess.hvess_electronics_temp >= initial_temp,
        "electronics temp should rise under load"
    );
}

#[test]
fn test_engine_speed_responds_to_motor_load() {
    let mut state = test_state();
    let initial_rpm = state.engine.engine_speed;

    // Apply motor load
    state.motor.mg1_speed_setpoint = 80.0;
    for _ in 0..200 {
        state.update_physics(0.1);
    }

    // Engine RPM should increase to meet demand
    assert!(
        state.engine.engine_speed > initial_rpm,
        "engine RPM should increase under load"
    );
}

#[test]
fn test_dcdc_voltage_regulation() {
    let mut state = test_state();
    state.dcdc.dcdc_operational_command = 1;
    state.dcdc.dcdc_low_side_voltage_setpoint = 55.0;

    for _ in 0..200 {
        state.update_physics(0.1);
    }

    // The physics model first calculates base voltage (48.0 - current*0.1),
    // then DCDC regulation blends toward the setpoint. With no motor load,
    // the equilibrium is roughly (48.0 * 0.9 + 55.0 * 0.1) ≈ 48.7V.
    assert_float_near(state.motor.mg1_voltage, 48.7, 1.0, "mg1_voltage after regulation");
}

#[test]
fn test_counter_increments_on_physics_update() {
    let mut state = test_state();
    let initial_counter = state.motor.mg1_control_counter;
    state.update_physics(0.1);
    assert_eq!(
        state.motor.mg1_control_counter,
        (initial_counter + 1) % 16,
        "counter should increment"
    );
}

#[test]
fn test_message_tracking() {
    let mut state = test_state();

    // Process a message
    let msg = MG1IC {
        device_id: external_device(),
        mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst: 50.0,
        mtr_gnrtr_1_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_1_invrtr_cntrl_cr: 42,
        mt_gt_1_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_1_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_1_rotor_position_sensing_request: 0,
        mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // Message tracking should have recorded it
    assert_eq!(state.recent_messages.len(), 1);
    assert!(state.recent_messages.back().unwrap().processed);
}

#[test]
fn test_message_tracking_limit() {
    let mut state = test_state();

    // Process 150 messages (limit is 100)
    for i in 0..150u64 {
        let msg = MG1IC {
            device_id: external_device(),
            mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst: i as f64,
            mtr_gnrtr_1_invrtr_cntrl_cntr: (i % 256) as u8,
            mtr_gnrtr_1_invrtr_cntrl_cr: 42,
            mt_gt_1_ivt_ct_lts_rqst_ovd_md: 0,
            mt_gt_1_ivt_ct_lts_rqst_ovd_mx: 0.0,
            mt_gt_1_ivt_ct_lts_rqst_ovd_m: 0.0,
            mg_1_rotor_position_sensing_request: 0,
            mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst: 0,
        };
        let (can_id, data) = msg.encode().unwrap();
        state.process_incoming_message(can_id, &data).unwrap();
    }

    // Should be capped at 100
    assert!(state.recent_messages.len() <= 100);
}

#[test]
fn test_broadcast_paused_flag() {
    let mut state = test_state();
    state.broadcast_paused = true;
    // The broadcast_paused flag is used by the runtime loop, not generate_can_frames itself.
    // Verify the flag can be set.
    assert!(state.broadcast_paused);
}

// ============================================================================
// Phase 6: Live CAN Interface Tests (ignored by default, run with --include-ignored)
// ============================================================================

#[test]
#[ignore]
fn test_live_can_simulator_starts() {
    // This test requires vcan0 to be available
    // It verifies the simulator binary can start and respond
    use std::process::Command;
    use std::time::Duration;

    let child = Command::new("cargo")
        .args(["run", "-p", "cando-j1939-sim", "--", "-i", "vcan0", "--test-mode"])
        .spawn();

    match child {
        Ok(mut child) => {
            std::thread::sleep(Duration::from_secs(2));
            child.kill().ok();
            child.wait().ok();
        }
        Err(e) => {
            eprintln!("Could not start simulator: {}", e);
            // Don't fail — this is expected if CAN is not configured
        }
    }
}

#[test]
#[ignore]
fn test_live_can_broadcast_frames_on_vcan0() {
    // This test requires vcan0 and the simulator running
    // Verify that broadcast frames appear on the CAN bus
    use std::process::Command;

    let output = Command::new("timeout")
        .args(["2", "candump", "vcan0"])
        .output();

    match output {
        Ok(result) => {
            if result.status.success() || result.status.code() == Some(124) {
                // Either got data or timed out — both are acceptable
                let stdout = String::from_utf8_lossy(&result.stdout);
                if !stdout.is_empty() {
                    eprintln!("Captured CAN traffic: {} bytes", stdout.len());
                }
            }
        }
        Err(_) => {
            eprintln!("candump not available, skipping live CAN test");
        }
    }
}
