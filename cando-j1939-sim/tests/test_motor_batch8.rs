//! Batch 8: Extended Motor/Generator Messages Tests
//!
//! Tests for 30 new MG messages: MG1IS3, MG2IS3, MG1IT, MG2IT, MG1II, MG2II,
//! MG1IR1, MG1IR2, MG2IR1, MG2IR2, MG1IRP, MG2IRP, MG1IAPL, MG2IAPL,
//! MG1IMF1, MG2IMF1, MG3IC, MG3IS1, MG3IS2, MG3IS3, MG3IT, MG3II,
//! MG3IR1, MG3IR2, MG3IRP, MG3IAPL, MG3IMF1, MG4IC, MG4IS1, MG4IS2

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
// MG3IC / MG4IC Command Handler Tests
// ============================================================================

#[test]
fn test_mg3ic_command_updates_speed_setpoint() {
    let mut state = test_state();
    let msg = MG3IC {
        device_id: external_device(),
        mtr_gnrtr_3_invrtr_cntrl_stpnt_rqst: 60.0,
        mtr_gnrtr_3_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_3_invrtr_cntrl_cr: 42,
        mt_gt_3_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_3_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_3_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_3_rotor_position_sensing_request: 0,
        mtr_gnrtr_3_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.motor.mg3_speed_setpoint, 60.0, 1.0, "mg3_speed_setpoint");
}

#[test]
fn test_mg4ic_command_updates_speed_setpoint() {
    let mut state = test_state();
    let msg = MG4IC {
        device_id: external_device(),
        mtr_gnrtr_4_invrtr_cntrl_stpnt_rqst: 45.0,
        mtr_gnrtr_4_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_4_invrtr_cntrl_cr: 42,
        mt_gt_4_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_4_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_4_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_4_rotor_position_sensing_request: 0,
        mtr_gnrtr_4_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.motor.mg4_speed_setpoint, 45.0, 1.0, "mg4_speed_setpoint");
}

// ============================================================================
// MG3/MG4 Physics Tests
// ============================================================================

#[test]
fn test_mg3_physics_speed_ramp() {
    let mut state = test_state();
    state.motor.mg3_speed_setpoint = 50.0;

    // Run physics for multiple steps
    for _ in 0..100 {
        state.update_physics(0.1);
    }

    // MG3 should approach the setpoint
    assert_float_near(state.motor.mg3_actual_speed, 50.0, 1.0, "mg3_actual_speed");
    // Torque should be non-zero when speed is non-zero
    assert!(state.motor.mg3_actual_torque.abs() > 0.1, "mg3 torque should be non-zero");
    // Current should be non-zero
    assert!(state.motor.mg3_current > 0.0, "mg3 current should be positive when running");
}

#[test]
fn test_mg4_physics_speed_ramp() {
    let mut state = test_state();
    state.motor.mg4_speed_setpoint = 30.0;

    for _ in 0..100 {
        state.update_physics(0.1);
    }

    assert_float_near(state.motor.mg4_actual_speed, 30.0, 1.0, "mg4_actual_speed");
    assert!(state.motor.mg4_actual_torque.abs() > 0.1, "mg4 torque should be non-zero");
}

#[test]
fn test_motor_angle_increments_with_speed() {
    let mut state = test_state();
    state.motor.mg1_speed_setpoint = 50.0;
    state.motor.mg2_speed_setpoint = 50.0;
    state.motor.mg3_speed_setpoint = 50.0;

    // Run physics to get speeds up
    for _ in 0..100 {
        state.update_physics(0.1);
    }

    // All motor angles should have advanced from 0
    assert!(state.motor.mg1_motor_angle > 0.0, "mg1 angle should advance");
    assert!(state.motor.mg2_motor_angle > 0.0, "mg2 angle should advance");
    assert!(state.motor.mg3_motor_angle > 0.0, "mg3 angle should advance");

    // Angles should be within 0-360 range
    assert!(state.motor.mg1_motor_angle < 360.0, "mg1 angle should be < 360");
    assert!(state.motor.mg2_motor_angle < 360.0, "mg2 angle should be < 360");
    assert!(state.motor.mg3_motor_angle < 360.0, "mg3 angle should be < 360");
}

#[test]
fn test_temperature_rises_with_current() {
    let mut state = test_state();
    let initial_temp1 = state.motor.mg1_inverter_temp1;
    let initial_temp3 = state.motor.mg3_inverter_temp1;

    // Set high speed to generate current
    state.motor.mg1_speed_setpoint = 100.0;
    state.motor.mg3_speed_setpoint = 100.0;

    // Run physics for many steps
    for _ in 0..200 {
        state.update_physics(0.1);
    }

    // Temperatures should have risen
    assert!(
        state.motor.mg1_inverter_temp1 > initial_temp1,
        "mg1 temp should rise: {} -> {}",
        initial_temp1,
        state.motor.mg1_inverter_temp1
    );
    assert!(
        state.motor.mg3_inverter_temp1 > initial_temp3,
        "mg3 temp should rise: {} -> {}",
        initial_temp3,
        state.motor.mg3_inverter_temp1
    );

    // Temp2-5 should be progressively lower
    assert!(state.motor.mg1_inverter_temp2 < state.motor.mg1_inverter_temp1);
    assert!(state.motor.mg1_inverter_temp3 < state.motor.mg1_inverter_temp2);
}

#[test]
fn test_temperature_cools_toward_ambient() {
    let mut state = test_state();
    // Start hot
    state.motor.mg1_inverter_temp1 = 100.0;
    state.motor.mg2_inverter_temp1 = 100.0;
    state.motor.mg3_inverter_temp1 = 100.0;

    // No load (speed = 0), let it cool
    for _ in 0..500 {
        state.update_physics(0.1);
    }

    // Should cool down toward ambient (25 degC)
    assert!(
        state.motor.mg1_inverter_temp1 < 80.0,
        "mg1 should cool: {}",
        state.motor.mg1_inverter_temp1
    );
    assert!(
        state.motor.mg3_inverter_temp1 < 80.0,
        "mg3 should cool: {}",
        state.motor.mg3_inverter_temp1
    );
}

// ============================================================================
// Broadcast Frame Generation Tests
// ============================================================================

#[test]
fn test_mg1is3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG1IS3::BASE_CAN_ID));
    assert!(found, "MG1IS3 frame should be in broadcast output");
}

#[test]
fn test_mg2is3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG2IS3::BASE_CAN_ID));
    assert!(found, "MG2IS3 frame should be in broadcast output");
}

#[test]
fn test_mg1it_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG1IT::BASE_CAN_ID));
    assert!(found, "MG1IT frame should be in broadcast output");
}

#[test]
fn test_mg2it_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG2IT::BASE_CAN_ID));
    assert!(found, "MG2IT frame should be in broadcast output");
}

#[test]
fn test_mg1ii_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG1II::BASE_CAN_ID));
    assert!(found, "MG1II frame should be in broadcast output");
}

#[test]
fn test_mg2ii_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG2II::BASE_CAN_ID));
    assert!(found, "MG2II frame should be in broadcast output");
}

#[test]
fn test_mg1ir1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG1IR1::BASE_CAN_ID));
    assert!(found, "MG1IR1 frame should be in broadcast output");
}

#[test]
fn test_mg1ir2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG1IR2::BASE_CAN_ID));
    assert!(found, "MG1IR2 frame should be in broadcast output");
}

#[test]
fn test_mg2ir1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG2IR1::BASE_CAN_ID));
    assert!(found, "MG2IR1 frame should be in broadcast output");
}

#[test]
fn test_mg2ir2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG2IR2::BASE_CAN_ID));
    assert!(found, "MG2IR2 frame should be in broadcast output");
}

#[test]
fn test_mg1irp_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG1IRP::BASE_CAN_ID));
    assert!(found, "MG1IRP frame should be in broadcast output");
}

#[test]
fn test_mg2irp_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG2IRP::BASE_CAN_ID));
    assert!(found, "MG2IRP frame should be in broadcast output");
}

#[test]
fn test_mg1iapl_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG1IAPL::BASE_CAN_ID));
    assert!(found, "MG1IAPL frame should be in broadcast output");
}

#[test]
fn test_mg2iapl_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG2IAPL::BASE_CAN_ID));
    assert!(found, "MG2IAPL frame should be in broadcast output");
}

#[test]
fn test_mg1imf1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG1IMF1::BASE_CAN_ID));
    assert!(found, "MG1IMF1 frame should be in broadcast output");
}

#[test]
fn test_mg2imf1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG2IMF1::BASE_CAN_ID));
    assert!(found, "MG2IMF1 frame should be in broadcast output");
}

#[test]
fn test_mg3is1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3IS1::BASE_CAN_ID));
    assert!(found, "MG3IS1 frame should be in broadcast output");
}

#[test]
fn test_mg3is2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3IS2::BASE_CAN_ID));
    assert!(found, "MG3IS2 frame should be in broadcast output");
}

#[test]
fn test_mg3is3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3IS3::BASE_CAN_ID));
    assert!(found, "MG3IS3 frame should be in broadcast output");
}

#[test]
fn test_mg3it_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3IT::BASE_CAN_ID));
    assert!(found, "MG3IT frame should be in broadcast output");
}

#[test]
fn test_mg3ii_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3II::BASE_CAN_ID));
    assert!(found, "MG3II frame should be in broadcast output");
}

#[test]
fn test_mg3ir1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3IR1::BASE_CAN_ID));
    assert!(found, "MG3IR1 frame should be in broadcast output");
}

#[test]
fn test_mg3ir2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3IR2::BASE_CAN_ID));
    assert!(found, "MG3IR2 frame should be in broadcast output");
}

#[test]
fn test_mg3irp_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3IRP::BASE_CAN_ID));
    assert!(found, "MG3IRP frame should be in broadcast output");
}

#[test]
fn test_mg3iapl_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3IAPL::BASE_CAN_ID));
    assert!(found, "MG3IAPL frame should be in broadcast output");
}

#[test]
fn test_mg3imf1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG3IMF1::BASE_CAN_ID));
    assert!(found, "MG3IMF1 frame should be in broadcast output");
}

#[test]
fn test_mg4is1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG4IS1::BASE_CAN_ID));
    assert!(found, "MG4IS1 frame should be in broadcast output");
}

#[test]
fn test_mg4is2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let found = frames
        .iter()
        .any(|f| matches_base_id(f.raw_id(), MG4IS2::BASE_CAN_ID));
    assert!(found, "MG4IS2 frame should be in broadcast output");
}

// ============================================================================
// Handler Dispatch Tests (all 30 messages recognized)
// ============================================================================

#[test]
fn test_all_batch8_messages_recognized() {
    let mut state = test_state();
    let dev = external_device();

    // Helper: encode, send, check recognized
    macro_rules! check_recognized {
        ($msg_type:ident, $msg:expr) => {
            let (can_id, data) = $msg.encode().unwrap();
            let status = state.process_incoming_message(can_id, &data).unwrap();
            assert_eq!(
                status,
                MessageStatus::Recognized,
                "{} should be recognized",
                stringify!($msg_type)
            );
        };
    }

    // MG1IS3
    check_recognized!(MG1IS3, MG1IS3 {
        device_id: dev,
        mtr_gnrtr_1_invrtr_cntrl_stts_3_cr: 0,
        mtr_gnrtr_1_invrtr_cntrl_stts_3_cntr: 0,
        motor_generator_1_motor_angle: 45.0,
    });

    // MG2IS3
    check_recognized!(MG2IS3, MG2IS3 {
        device_id: dev,
        mtr_gnrtr_2_invrtr_cntrl_stts_3_cr: 0,
        mtr_gnrtr_2_invrtr_cntrl_stts_3_cntr: 0,
        motor_generator_2_motor_angle: 90.0,
    });

    // MG1IT
    check_recognized!(MG1IT, MG1IT {
        device_id: dev,
        mtr_gnrtr_1_invrtr_tmprtr_1: 35.0,
        mtr_gnrtr_1_invrtr_tmprtr_2: 34.0,
        mtr_gnrtr_1_invrtr_tmprtr_3: 33.0,
        mtr_gnrtr_1_invrtr_tmprtr_4: 32.0,
        mtr_gnrtr_1_invrtr_tmprtr_5: 31.0,
    });

    // MG2IT
    check_recognized!(MG2IT, MG2IT {
        device_id: dev,
        mtr_gnrtr_2_invrtr_tmprtr_1: 35.0,
        mtr_gnrtr_2_invrtr_tmprtr_2: 34.0,
        mtr_gnrtr_2_invrtr_tmprtr_3: 33.0,
        mtr_gnrtr_2_invrtr_tmprtr_4: 32.0,
        mtr_gnrtr_2_invrtr_tmprtr_5: 31.0,
    });

    // MG1II
    check_recognized!(MG1II, MG1II {
        device_id: dev,
        mtr_gnrtr_1_invrtr_isltn_intgrt_cr: 0,
        mtr_gnrtr_1_invrtr_isltn_intgrt_cntr: 0,
        mt_gt_1_ivt_d_sd_ntv_t_csss_gd_vt: 0.0,
        mt_gt_1_ivt_h_vt_bs_atv_ist_tst_stts: 0,
        mt_gt_1_ivt_h_vt_bs_pssv_ist_tst_stts: 0,
        mt_gt_1_ivt_h_vt_bs_atv_ist_tst_rsts: 100.0,
        mt_gt_1_ivt_h_vt_bs_pssv_ist_tst_rsts: 100.0,
    });

    // MG2II
    check_recognized!(MG2II, MG2II {
        device_id: dev,
        mtr_gnrtr_2_invrtr_isltn_intgrt_cr: 0,
        mtr_gnrtr_2_invrtr_isltn_intgrt_cntr: 0,
        mt_gt_2_ivt_d_sd_ntv_t_csss_gd_vt: 0.0,
        mt_gt_2_ivt_h_vt_bs_atv_ist_tst_stts: 0,
        mt_gt_2_ivt_h_vt_bs_pssv_ist_tst_stts: 0,
        mt_gt_2_ivt_h_vt_bs_atv_ist_tst_rsts: 100.0,
        mt_gt_2_ivt_h_vt_bs_pssv_ist_tst_rsts: 100.0,
    });

    // MG1IR1
    check_recognized!(MG1IR1, MG1IR1 {
        device_id: dev,
        mtr_gnrtr_1_invrtr_rfrn_1_cr: 0,
        mtr_gnrtr_1_invrtr_rfrn_1_cntr: 0,
        mtr_gnrtr_1_invrtr_rfrn_trq: 400.0,
        mtr_gnrtr_1_invrtr_rfrn_spd: 8192.0,
        mtr_gnrtr_1_invrtr_rfrn_pwr: 100.0,
    });

    // MG1IR2
    check_recognized!(MG1IR2, MG1IR2 {
        device_id: dev,
        mtr_gnrtr_1_invrtr_rfrn_2_cr: 0,
        mtr_gnrtr_1_invrtr_rfrn_2_cntr: 0,
        mtr_gnrtr_1_invrtr_rfrn_crrnt: 200.0,
        mtr_gnrtr_1_invrtr_rfrn_vltg: 400.0,
    });

    // MG2IR1
    check_recognized!(MG2IR1, MG2IR1 {
        device_id: dev,
        mtr_gnrtr_2_invrtr_rfrn_1_cr: 0,
        mtr_gnrtr_2_invrtr_rfrn_1_cntr: 0,
        mtr_gnrtr_2_invrtr_rfrn_trq: 400.0,
        mtr_gnrtr_2_invrtr_rfrn_spd: 8192.0,
        mtr_gnrtr_2_invrtr_rfrn_pwr: 100.0,
    });

    // MG2IR2
    check_recognized!(MG2IR2, MG2IR2 {
        device_id: dev,
        mtr_gnrtr_2_invrtr_rfrn_2_cr: 0,
        mtr_gnrtr_2_invrtr_rfrn_2_cntr: 0,
        mtr_gnrtr_2_invrtr_rfrn_crrnt: 200.0,
        mtr_gnrtr_2_invrtr_rfrn_vltg: 400.0,
    });

    // MG1IRP
    check_recognized!(MG1IRP, MG1IRP {
        device_id: dev,
        mtr_gnrtr_1_invrtr_lmts_rqst_pwr_cr: 0,
        mtr_gnrtr_1_invrtr_lmts_rqst_pwr_cntr: 0,
        mt_gt_1_ivt_lts_rqst_m_pw_mx: 80.0,
        mt_gt_1_ivt_lts_rqst_m_pw_m: -80.0,
        mt_gt_1_ivt_lts_rqst_d_sd_pw_mx: 75.0,
        mt_gt_1_ivt_lts_rqst_d_sd_pw_m: -75.0,
    });

    // MG2IRP
    check_recognized!(MG2IRP, MG2IRP {
        device_id: dev,
        mtr_gnrtr_2_invrtr_lmts_rqst_pwr_cr: 0,
        mtr_gnrtr_2_invrtr_lmts_rqst_pwr_cntr: 0,
        mt_gt_2_ivt_lts_rqst_m_pw_mx: 80.0,
        mt_gt_2_ivt_lts_rqst_m_pw_m: -80.0,
        mt_gt_2_ivt_lts_rqst_d_sd_pw_mx: 75.0,
        mt_gt_2_ivt_lts_rqst_d_sd_pw_m: -75.0,
    });

    // MG1IAPL
    check_recognized!(MG1IAPL, MG1IAPL {
        device_id: dev,
        mt_gt_1_ivt_pw_ltd_dt_udd_rs: 0,
        mt_gt_1_ivt_pw_ltd_dtd_sd_ct_mx: 0,
        mt_gt_1_ivt_pw_ltd_dtd_sd_ct_m: 0,
        mt_gt_1_ivt_pw_ltd_dtd_sd_vt_mx: 0,
        mt_gt_1_ivt_pw_ltd_dtd_sd_vt_m: 0,
        mt_gt_1_ivt_pw_ltd_dtm_pw_mx: 0,
        mt_gt_1_ivt_pw_ltd_dtm_pw_m: 0,
        mt_gt_1_ivt_pw_ltd_dtd_sd_pw_mx: 0,
        mt_gt_1_ivt_pw_ltd_dtd_sd_pw_m: 0,
        mtr_gnrtr_1_invrtr_pwr_lmtd_dt_trq_mxmm: 0,
        mtr_gnrtr_1_invrtr_pwr_lmtd_dt_trq_mnmm: 0,
        mtr_gnrtr_1_invrtr_pwr_lmtd_dt_spd_mxmm: 0,
        mtr_gnrtr_1_invrtr_pwr_lmtd_dt_spd_mnmm: 0,
        mt_gt_1_ivt_pw_ltd_dt_ivt_tpt: 0,
        mt_gt_1_ivt_pw_ltd_dt_mt_tpt: 0,
        mt_gt_1_ivt_pw_ltd_dt_ft_cdt: 0,
        mt_gt_1_ivt_pw_ltd_dt_atv_tq_rt_lt: 0,
        mt_gt_1_ivt_pw_ltd_dt_atv_spd_rt_lt: 0,
        mt_gt_1_ivt_tq_ltd_dtm_ctsts: 0,
    });

    // MG2IAPL
    check_recognized!(MG2IAPL, MG2IAPL {
        device_id: dev,
        mt_gt_2_ivt_pw_ltd_dt_udd_rs: 0,
        mt_gt_2_ivt_pw_ltd_dtd_sd_ct_mx: 0,
        mt_gt_2_ivt_pw_ltd_dtd_sd_ct_m: 0,
        mt_gt_2_ivt_pw_ltd_dtd_sd_vt_mx: 0,
        mt_gt_2_ivt_pw_ltd_dtd_sd_vt_m: 0,
        mt_gt_2_ivt_pw_ltd_dtm_pw_mx: 0,
        mt_gt_2_ivt_pw_ltd_dtm_pw_m: 0,
        mt_gt_2_ivt_pw_ltd_dtd_sd_pw_mx: 0,
        mt_gt_2_ivt_pw_ltd_dtd_sd_pw_m: 0,
        mtr_gnrtr_2_invrtr_pwr_lmtd_dt_trq_mxmm: 0,
        mtr_gnrtr_2_invrtr_pwr_lmtd_dt_trq_mnmm: 0,
        mtr_gnrtr_2_invrtr_pwr_lmtd_dt_spd_mxmm: 0,
        mtr_gnrtr_2_invrtr_pwr_lmtd_dt_spd_mnmm: 0,
        mt_gt_2_ivt_pw_ltd_dt_ivt_tpt: 0,
        mt_gt_2_ivt_pw_ltd_dt_mt_tpt: 0,
        mt_gt_2_ivt_pw_ltd_dt_ft_cdt: 0,
        mt_gt_2_ivt_pw_ltd_dt_atv_tq_rt_lt: 0,
        mt_gt_2_ivt_pw_ltd_dt_atv_spd_rt_lt: 0,
        mt_gt_2_ivt_tq_ltd_dtm_ctsts: 0,
    });

    // MG1IMF1
    check_recognized!(MG1IMF1, MG1IMF1 {
        device_id: dev,
        mtr_gnrtr_1_invrtr_md_fdk_1_cr: 0,
        mtr_gnrtr_1_invrtr_md_fdk_1_cntr: 0,
        mtr_gnrtr_1_invrtr_cntrl_lmts_ovrrd_md: 0,
        mtr_gnrtr_1_invrtr_cntrl_stpnt_md: 3,
        mtr_gnrtr_1_invrtr_hvl_stts: 0,
        mg_1_rotor_position_sensing_status: 1,
    });

    // MG2IMF1
    check_recognized!(MG2IMF1, MG2IMF1 {
        device_id: dev,
        mtr_gnrtr_2_invrtr_md_fdk_1_cr: 0,
        mtr_gnrtr_2_invrtr_md_fdk_1_cntr: 0,
        mtr_gnrtr_2_invrtr_cntrl_lmts_ovrrd_md: 0,
        mtr_gnrtr_2_invrtr_cntrl_stpnt_md: 3,
        mtr_gnrtr_2_invrtr_hvl_stts: 0,
        mg_2_rotor_position_sensing_status: 1,
    });

    // MG3IC (already tested above, but include for completeness)
    check_recognized!(MG3IC, MG3IC {
        device_id: dev,
        mtr_gnrtr_3_invrtr_cntrl_cr: 0,
        mtr_gnrtr_3_invrtr_cntrl_cntr: 0,
        mt_gt_3_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_3_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_3_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_3_rotor_position_sensing_request: 0,
        mtr_gnrtr_3_invrtr_cntrl_stpnt_md_rqst: 0,
        mtr_gnrtr_3_invrtr_cntrl_stpnt_rqst: 25.0,
    });

    // MG3IS1
    check_recognized!(MG3IS1, MG3IS1 {
        device_id: dev,
        mtr_gnrtr_3_invrtr_stts_1_cr: 0,
        mtr_gnrtr_3_invrtr_stts_1_cntr: 0,
        motor_generator_3_speed: 50.0,
        motor_generator_3_net_rotor_torque: 25.0,
        mtr_gnrtr_3_invrtr_d_sd_crrnt: 10.0,
        mtr_gnrtr_3_invrtr_d_sd_vltg: 48.0,
    });

    // MG3IS2
    check_recognized!(MG3IS2, MG3IS2 {
        device_id: dev,
        mtr_gnrtr_3_invrtr_stts_2_cr: 0,
        mtr_gnrtr_3_invrtr_stts_2_cntr: 0,
        mtr_gnrtr_3_avll_mxmm_trq: 100.0,
        mtr_gnrtr_3_avll_mnmm_trq: -100.0,
    });

    // MG3IS3
    check_recognized!(MG3IS3, MG3IS3 {
        device_id: dev,
        mtr_gnrtr_3_invrtr_cntrl_stts_3_cr: 0,
        mtr_gnrtr_3_invrtr_cntrl_stts_3_cntr: 0,
        motor_generator_3_motor_angle: 180.0,
    });

    // MG3IT
    check_recognized!(MG3IT, MG3IT {
        device_id: dev,
        mtr_gnrtr_3_invrtr_tmprtr_1: 35.0,
        mtr_gnrtr_3_invrtr_tmprtr_2: 34.0,
        mtr_gnrtr_3_invrtr_tmprtr_3: 33.0,
        mtr_gnrtr_3_invrtr_tmprtr_4: 32.0,
        mtr_gnrtr_3_invrtr_tmprtr_5: 31.0,
    });

    // MG3II
    check_recognized!(MG3II, MG3II {
        device_id: dev,
        mtr_gnrtr_3_invrtr_isltn_intgrt_cr: 0,
        mtr_gnrtr_3_invrtr_isltn_intgrt_cntr: 0,
        mt_gt_3_ivt_d_sd_ntv_t_csss_gd_vt: 0.0,
        mt_gt_3_ivt_h_vt_bs_atv_ist_tst_stts: 0,
        mt_gt_3_ivt_h_vt_bs_pssv_ist_tst_stts: 0,
        mt_gt_3_ivt_h_vt_bs_atv_ist_tst_rsts: 100.0,
        mt_gt_3_ivt_h_vt_bs_pssv_ist_tst_rsts: 100.0,
    });

    // MG3IR1
    check_recognized!(MG3IR1, MG3IR1 {
        device_id: dev,
        mtr_gnrtr_3_invrtr_rfrn_1_cr: 0,
        mtr_gnrtr_3_invrtr_rfrn_1_cntr: 0,
        mtr_gnrtr_3_invrtr_rfrn_trq: 400.0,
        mtr_gnrtr_3_invrtr_rfrn_spd: 8192.0,
        mtr_gnrtr_3_invrtr_rfrn_pwr: 100.0,
    });

    // MG3IR2
    check_recognized!(MG3IR2, MG3IR2 {
        device_id: dev,
        mtr_gnrtr_3_invrtr_rfrn_2_cr: 0,
        mtr_gnrtr_3_invrtr_rfrn_2_cntr: 0,
        mtr_gnrtr_3_invrtr_rfrn_crrnt: 200.0,
        mtr_gnrtr_3_invrtr_rfrn_vltg: 400.0,
    });

    // MG3IRP
    check_recognized!(MG3IRP, MG3IRP {
        device_id: dev,
        mtr_gnrtr_3_invrtr_lmts_rqst_pwr_cr: 0,
        mtr_gnrtr_3_invrtr_lmts_rqst_pwr_cntr: 0,
        mt_gt_3_ivt_lts_rqst_m_pw_mx: 80.0,
        mt_gt_3_ivt_lts_rqst_m_pw_m: -80.0,
        mt_gt_3_ivt_lts_rqst_d_sd_pw_mx: 75.0,
        mt_gt_3_ivt_lts_rqst_d_sd_pw_m: -75.0,
    });

    // MG3IAPL
    check_recognized!(MG3IAPL, MG3IAPL {
        device_id: dev,
        mt_gt_3_ivt_pw_ltd_dt_udd_rs: 0,
        mt_gt_3_ivt_pw_ltd_dtd_sd_ct_mx: 0,
        mt_gt_3_ivt_pw_ltd_dtd_sd_ct_m: 0,
        mt_gt_3_ivt_pw_ltd_dtd_sd_vt_mx: 0,
        mt_gt_3_ivt_pw_ltd_dtd_sd_vt_m: 0,
        mt_gt_3_ivt_pw_ltd_dtm_pw_mx: 0,
        mt_gt_3_ivt_pw_ltd_dtm_pw_m: 0,
        mt_gt_3_ivt_pw_ltd_dtd_sd_pw_mx: 0,
        mt_gt_3_ivt_pw_ltd_dtd_sd_pw_m: 0,
        mtr_gnrtr_3_invrtr_pwr_lmtd_dt_trq_mxmm: 0,
        mtr_gnrtr_3_invrtr_pwr_lmtd_dt_trq_mnmm: 0,
        mtr_gnrtr_3_invrtr_pwr_lmtd_dt_spd_mxmm: 0,
        mtr_gnrtr_3_invrtr_pwr_lmtd_dt_spd_mnmm: 0,
        mt_gt_3_ivt_pw_ltd_dt_ivt_tpt: 0,
        mt_gt_3_ivt_pw_ltd_dt_mt_tpt: 0,
        mt_gt_3_ivt_pw_ltd_dt_ft_cdt: 0,
        mt_gt_3_ivt_pw_ltd_dt_atv_tq_rt_lt: 0,
        mt_gt_3_ivt_pw_ltd_dt_atv_spd_rt_lt: 0,
        mt_gt_3_ivt_tq_ltd_dtm_ctsts: 0,
    });

    // MG3IMF1
    check_recognized!(MG3IMF1, MG3IMF1 {
        device_id: dev,
        mtr_gnrtr_3_invrtr_md_fdk_1_cr: 0,
        mtr_gnrtr_3_invrtr_md_fdk_1_cntr: 0,
        mtr_gnrtr_3_invrtr_cntrl_lmts_ovrrd_md: 0,
        mtr_gnrtr_3_invrtr_cntrl_stpnt_md: 3,
        mtr_gnrtr_3_invrtr_hvl_stts: 0,
        mg_3_rotor_position_sensing_status: 1,
    });

    // MG4IC (already tested above)
    check_recognized!(MG4IC, MG4IC {
        device_id: dev,
        mtr_gnrtr_4_invrtr_cntrl_cr: 0,
        mtr_gnrtr_4_invrtr_cntrl_cntr: 0,
        mt_gt_4_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_4_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_4_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_4_rotor_position_sensing_request: 0,
        mtr_gnrtr_4_invrtr_cntrl_stpnt_md_rqst: 0,
        mtr_gnrtr_4_invrtr_cntrl_stpnt_rqst: 20.0,
    });

    // MG4IS1
    check_recognized!(MG4IS1, MG4IS1 {
        device_id: dev,
        mtr_gnrtr_4_invrtr_stts_1_cr: 0,
        mtr_gnrtr_4_invrtr_stts_1_cntr: 0,
        motor_generator_4_speed: 30.0,
        motor_generator_4_net_rotor_torque: 15.0,
        mtr_gnrtr_4_invrtr_d_sd_crrnt: 5.0,
        mtr_gnrtr_4_invrtr_d_sd_vltg: 47.0,
    });

    // MG4IS2
    check_recognized!(MG4IS2, MG4IS2 {
        device_id: dev,
        mtr_gnrtr_4_invrtr_stts_2_cr: 0,
        mtr_gnrtr_4_invrtr_stts_2_cntr: 0,
        mtr_gnrtr_4_avll_mxmm_trq: 100.0,
        mtr_gnrtr_4_avll_mnmm_trq: -100.0,
    });
}

// ============================================================================
// Round-Trip Tests (Command -> Physics -> Broadcast -> Decode)
// ============================================================================

#[test]
fn test_mg3ic_to_mg3is1_round_trip() {
    let mut state = test_state();

    // Send MG3IC command
    let msg = MG3IC {
        device_id: external_device(),
        mtr_gnrtr_3_invrtr_cntrl_stpnt_rqst: 70.0,
        mtr_gnrtr_3_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_3_invrtr_cntrl_cr: 42,
        mt_gt_3_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_3_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_3_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_3_rotor_position_sensing_request: 0,
        mtr_gnrtr_3_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    // Run physics to settle
    for _ in 0..200 {
        state.update_physics(0.1);
    }

    // Generate broadcast frames
    let frames = state.generate_can_frames();

    // Find and decode MG3IS1
    let mg3is1_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), MG3IS1::BASE_CAN_ID))
        .expect("MG3IS1 frame should be present");

    let decoded =
        MG3IS1::decode(mg3is1_frame.raw_id(), mg3is1_frame.data()).expect("MG3IS1 should decode");

    // Speed should be close to setpoint after physics convergence
    assert_float_near(
        decoded.motor_generator_3_speed,
        70.0,
        2.0,
        "MG3IS1 speed round-trip",
    );
}

#[test]
fn test_mg4ic_to_mg4is1_round_trip() {
    let mut state = test_state();

    let msg = MG4IC {
        device_id: external_device(),
        mtr_gnrtr_4_invrtr_cntrl_stpnt_rqst: 40.0,
        mtr_gnrtr_4_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_4_invrtr_cntrl_cr: 42,
        mt_gt_4_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_4_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_4_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_4_rotor_position_sensing_request: 0,
        mtr_gnrtr_4_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    state.process_incoming_message(can_id, &data).unwrap();

    for _ in 0..200 {
        state.update_physics(0.1);
    }

    let frames = state.generate_can_frames();

    let mg4is1_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id(), MG4IS1::BASE_CAN_ID))
        .expect("MG4IS1 frame should be present");

    let decoded =
        MG4IS1::decode(mg4is1_frame.raw_id(), mg4is1_frame.data()).expect("MG4IS1 should decode");

    assert_float_near(
        decoded.motor_generator_4_speed,
        40.0,
        2.0,
        "MG4IS1 speed round-trip",
    );
}

#[test]
fn test_mg1irp_command_updates_power_limits() {
    let mut state = test_state();
    let msg = MG1IRP {
        device_id: external_device(),
        mtr_gnrtr_1_invrtr_lmts_rqst_pwr_cr: 0,
        mtr_gnrtr_1_invrtr_lmts_rqst_pwr_cntr: 0,
        mt_gt_1_ivt_lts_rqst_m_pw_mx: 75.0,
        mt_gt_1_ivt_lts_rqst_m_pw_m: -60.0,
        mt_gt_1_ivt_lts_rqst_d_sd_pw_mx: 70.0,
        mt_gt_1_ivt_lts_rqst_d_sd_pw_m: -55.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.motor.mg1_power_limit_mech_max, 75.0, 1.0, "mg1_power_limit_mech_max");
    assert_float_near(state.motor.mg1_power_limit_mech_min, -60.0, 1.0, "mg1_power_limit_mech_min");
    assert_float_near(state.motor.mg1_power_limit_dc_max, 70.0, 1.0, "mg1_power_limit_dc_max");
    assert_float_near(state.motor.mg1_power_limit_dc_min, -55.0, 1.0, "mg1_power_limit_dc_min");
}

#[test]
fn test_mg3irp_command_updates_power_limits() {
    let mut state = test_state();
    let msg = MG3IRP {
        device_id: external_device(),
        mtr_gnrtr_3_invrtr_lmts_rqst_pwr_cr: 0,
        mtr_gnrtr_3_invrtr_lmts_rqst_pwr_cntr: 0,
        mt_gt_3_ivt_lts_rqst_m_pw_mx: 90.0,
        mt_gt_3_ivt_lts_rqst_m_pw_m: -85.0,
        mt_gt_3_ivt_lts_rqst_d_sd_pw_mx: 88.0,
        mt_gt_3_ivt_lts_rqst_d_sd_pw_m: -82.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.motor.mg3_power_limit_mech_max, 90.0, 1.0, "mg3_power_limit_mech_max");
    assert_float_near(state.motor.mg3_power_limit_mech_min, -85.0, 1.0, "mg3_power_limit_mech_min");
}

// ============================================================================
// MG3/MG4 Counter Increment Tests
// ============================================================================

#[test]
fn test_mg3_mg4_counters_increment() {
    let mut state = test_state();
    assert_eq!(state.motor.mg3_control_counter, 0);
    assert_eq!(state.motor.mg3_status_counter, 0);
    assert_eq!(state.motor.mg4_control_counter, 0);
    assert_eq!(state.motor.mg4_status_counter, 0);

    state.update_physics(0.1);

    assert_eq!(state.motor.mg3_control_counter, 1);
    assert_eq!(state.motor.mg3_status_counter, 1);
    assert_eq!(state.motor.mg4_control_counter, 1);
    assert_eq!(state.motor.mg4_status_counter, 1);

    // Counters should wrap at 16
    for _ in 0..15 {
        state.update_physics(0.1);
    }
    assert_eq!(state.motor.mg3_control_counter, 0); // 16 % 16 = 0
    assert_eq!(state.motor.mg4_control_counter, 0);
}

// ============================================================================
// Default State Tests
// ============================================================================

#[test]
fn test_batch8_default_state() {
    let state = test_state();

    // MG1 extended defaults
    assert_float_near(state.motor.mg1_motor_angle, 0.0, 0.01, "mg1_motor_angle");
    assert_float_near(state.motor.mg1_inverter_temp1, 35.0, 0.01, "mg1_inverter_temp1");
    assert_float_near(state.motor.mg1_ref_torque, 400.0, 0.01, "mg1_ref_torque");
    assert_float_near(state.motor.mg1_ref_speed, 8192.0, 0.01, "mg1_ref_speed");

    // MG2 extended defaults
    assert_float_near(state.motor.mg2_motor_angle, 0.0, 0.01, "mg2_motor_angle");
    assert_float_near(state.motor.mg2_max_torque, 100.0, 0.01, "mg2_max_torque");

    // MG3 defaults
    assert_float_near(state.motor.mg3_speed_setpoint, 0.0, 0.01, "mg3_speed_setpoint");
    assert_float_near(state.motor.mg3_actual_speed, 0.0, 0.01, "mg3_actual_speed");
    assert_float_near(state.motor.mg3_voltage, 48.0, 0.01, "mg3_voltage");
    assert_float_near(state.motor.mg3_max_torque, 100.0, 0.01, "mg3_max_torque");
    assert_eq!(state.motor.mg3_control_counter, 0);

    // MG4 defaults
    assert_float_near(state.motor.mg4_speed_setpoint, 0.0, 0.01, "mg4_speed_setpoint");
    assert_float_near(state.motor.mg4_voltage, 48.0, 0.01, "mg4_voltage");
    assert_eq!(state.motor.mg4_control_counter, 0);
}

// ============================================================================
// Broadcast count test - verify all 30 messages are broadcast
// ============================================================================

#[test]
fn test_batch8_all_30_broadcast_message_types() {
    let state = test_state();
    let frames = state.generate_can_frames();

    let batch8_base_ids: Vec<(&str, u32)> = vec![
        ("MG2IS2", MG2IS2::BASE_CAN_ID),
        ("MG1IS3", MG1IS3::BASE_CAN_ID),
        ("MG2IS3", MG2IS3::BASE_CAN_ID),
        ("MG1IT", MG1IT::BASE_CAN_ID),
        ("MG2IT", MG2IT::BASE_CAN_ID),
        ("MG1II", MG1II::BASE_CAN_ID),
        ("MG2II", MG2II::BASE_CAN_ID),
        ("MG1IR1", MG1IR1::BASE_CAN_ID),
        ("MG1IR2", MG1IR2::BASE_CAN_ID),
        ("MG2IR1", MG2IR1::BASE_CAN_ID),
        ("MG2IR2", MG2IR2::BASE_CAN_ID),
        ("MG1IRP", MG1IRP::BASE_CAN_ID),
        ("MG2IRP", MG2IRP::BASE_CAN_ID),
        ("MG1IAPL", MG1IAPL::BASE_CAN_ID),
        ("MG2IAPL", MG2IAPL::BASE_CAN_ID),
        ("MG1IMF1", MG1IMF1::BASE_CAN_ID),
        ("MG2IMF1", MG2IMF1::BASE_CAN_ID),
        ("MG3IS1", MG3IS1::BASE_CAN_ID),
        ("MG3IS2", MG3IS2::BASE_CAN_ID),
        ("MG3IS3", MG3IS3::BASE_CAN_ID),
        ("MG3IT", MG3IT::BASE_CAN_ID),
        ("MG3II", MG3II::BASE_CAN_ID),
        ("MG3IR1", MG3IR1::BASE_CAN_ID),
        ("MG3IR2", MG3IR2::BASE_CAN_ID),
        ("MG3IRP", MG3IRP::BASE_CAN_ID),
        ("MG3IAPL", MG3IAPL::BASE_CAN_ID),
        ("MG3IMF1", MG3IMF1::BASE_CAN_ID),
        ("MG4IS1", MG4IS1::BASE_CAN_ID),
        ("MG4IS2", MG4IS2::BASE_CAN_ID),
    ];

    for (name, base_id) in &batch8_base_ids {
        let found = frames
            .iter()
            .any(|f| matches_base_id(f.raw_id(), *base_id));
        assert!(found, "{} (0x{:08X}) should be in broadcast output", name, base_id);
    }
}

// ============================================================================
// Self-Reception and DecodeFailed Tests
// ============================================================================

#[test]
fn test_mg3ic_self_reception_ignored() {
    let mut state = test_state();
    let msg = MG3IC {
        device_id: DeviceId::from(0x82),
        mtr_gnrtr_3_invrtr_cntrl_stpnt_rqst: 60.0,
        mtr_gnrtr_3_invrtr_cntrl_cntr: 1,
        mtr_gnrtr_3_invrtr_cntrl_cr: 42,
        mt_gt_3_ivt_ct_lts_rqst_ovd_md: 0,
        mt_gt_3_ivt_ct_lts_rqst_ovd_mx: 0.0,
        mt_gt_3_ivt_ct_lts_rqst_ovd_m: 0.0,
        mg_3_rotor_position_sensing_request: 0,
        mtr_gnrtr_3_invrtr_cntrl_stpnt_md_rqst: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Ignored);
}

#[test]
fn test_batch8_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = MG3IC::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
