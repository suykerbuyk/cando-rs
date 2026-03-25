//! Batch 3: Engine Temps, Fluids & Sensors Test Suite
//!
//! Tests for all 23 implemented messages:
//! ET1, ET2, ET3, ET4, ET5, ET6, LFE1, LFE2, IC1, IC2, IC3,
//! AMB, AMB2, AMB3, AMB4, FD2, DD2, DD3, HOURS, HOURS2, IO, FL, LFC1
//!
//! Skipped messages (not in generated code): FD, DD

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

/// Check if a CAN frame ID matches a message type's BASE_CAN_ID (masking out source address)
fn matches_base_id(frame_id: u32, base_can_id: u32) -> bool {
    (frame_id & 0xFFFFFF00) == base_can_id
}

// ============================================================================
// ET1 - Engine Temperature 1 Tests
// ============================================================================

#[test]
fn test_et1_handler_updates_state() {
    let mut state = test_state();
    let msg = ET1 {
        device_id: external_device(),
        engine_coolant_temperature: 92.0,
        engine_fuel_1_temperature_1: 55.0,
        engine_oil_temperature_1: 105.0,
        engn_trhrgr_1_ol_tmprtr: 120.0,
        engine_intercooler_temperature: 60.0,
        engn_chrg_ar_clr_thrmstt_opnng: 75.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.et1_coolant_temp, 92.0, 1.0, "et1_coolant_temp");
    assert_float_near(state.sensors.et1_fuel_temp, 55.0, 1.0, "et1_fuel_temp");
    assert_float_near(state.sensors.et1_oil_temp, 105.0, 1.0, "et1_oil_temp");
    assert_float_near(state.sensors.et1_turbo_oil_temp, 120.0, 1.0, "et1_turbo_oil_temp");
    assert_float_near(state.sensors.et1_intercooler_temp, 60.0, 1.0, "et1_intercooler_temp");
    assert_float_near(state.sensors.et1_charge_air_cooler_thermostat, 75.0, 1.0, "et1_thermostat");
}

#[test]
fn test_et1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let et1_frames: Vec<_> = frames.iter()
        .filter(|f| matches_base_id(f.raw_id(), ET1::BASE_CAN_ID))
        .collect();
    assert!(!et1_frames.is_empty(), "ET1 frame should be present in broadcast");
}

#[test]
fn test_et1_round_trip() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let et1_frame = frames.iter()
        .find(|f| matches_base_id(f.raw_id(), ET1::BASE_CAN_ID))
        .expect("ET1 frame not found");
    let decoded = ET1::decode(et1_frame.raw_id(), et1_frame.data()).unwrap();
    assert_float_near(decoded.engine_coolant_temperature, state.sensors.et1_coolant_temp, 1.0, "ET1 round-trip coolant");
    assert_float_near(decoded.engine_oil_temperature_1, state.sensors.et1_oil_temp, 2.0, "ET1 round-trip oil");
}

// ============================================================================
// ET2 - Engine Temperature 2 Tests
// ============================================================================

#[test]
fn test_et2_handler_updates_state() {
    let mut state = test_state();
    let msg = ET2 {
        device_id: external_device(),
        engine_oil_temperature_2: 88.0,
        engine_ecu_temperature: 65.0,
        engn_exhst_gs_rrltn_1_dffrntl_prssr: 12.0,
        engn_exhst_gs_rrltn_1_tmprtr: 190.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.et2_oil_temp_2, 88.0, 2.0, "et2_oil_temp_2");
    assert_float_near(state.sensors.et2_ecu_temp, 65.0, 1.0, "et2_ecu_temp");
}

#[test]
fn test_et2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), ET2::BASE_CAN_ID)), "ET2 frame missing");
}

// ============================================================================
// ET3 - Engine Temperature 3 Tests
// ============================================================================

#[test]
fn test_et3_handler_updates_state() {
    let mut state = test_state();
    let msg = ET3 {
        device_id: external_device(),
        engn_intk_mnfld_1_tmprtr_hgh_rsltn: 50.0,
        engn_clnt_tmprtr_hgh_rsltn: 86.0,
        engn_intk_vlv_attn_sstm_ol_tmprtr: 78.0,
        engn_chrg_ar_clr_1_otlt_tmprtr: 38.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.et3_coolant_temp_hr, 86.0, 1.0, "et3_coolant_temp_hr");
}

#[test]
fn test_et3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), ET3::BASE_CAN_ID)), "ET3 frame missing");
}

// ============================================================================
// ET4 - Engine Temperature 4 Tests
// ============================================================================

#[test]
fn test_et4_handler_updates_state() {
    let mut state = test_state();
    let msg = ET4 {
        device_id: external_device(),
        engine_coolant_temperature_2: 83.0,
        engn_clnt_pmp_otlt_tmprtr: 80.0,
        engine_coolant_thermostat_opening: 70.0,
        engn_exhst_vlv_attn_sstm_ol_tmprtr: 95.0,
        engn_exhst_gs_rrltn_1_mxr_intk_tmprtr: 125.0,
        engine_coolant_temperature_3: 82.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.et4_coolant_temp_2, 83.0, 1.0, "et4_coolant_temp_2");
    assert_float_near(state.sensors.et4_coolant_thermostat_opening, 70.0, 1.0, "et4_thermostat");
}

#[test]
fn test_et4_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), ET4::BASE_CAN_ID)), "ET4 frame missing");
}

// ============================================================================
// ET5 - Engine Temperature 5 Tests
// ============================================================================

#[test]
fn test_et5_handler_updates_state() {
    let mut state = test_state();
    let msg = ET5 {
        device_id: external_device(),
        engn_exhst_gs_rrltn_2_tmprtr: 170.0,
        engn_exhst_gs_rrltn_2_mxr_intk_tmprtr: 110.0,
        e_ct_tpt_2_h_rst_extdd_r: 82.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.et5_egr2_temp, 170.0, 1.0, "et5_egr2_temp");
}

#[test]
fn test_et5_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), ET5::BASE_CAN_ID)), "ET5 frame missing");
}

// ============================================================================
// ET6 - Engine Temperature 6 Tests
// ============================================================================

#[test]
fn test_et6_handler_updates_state() {
    let mut state = test_state();
    let msg = ET6 {
        device_id: external_device(),
        engn_chrg_ar_clr_intk_clnt_tmprtr: 30.0,
        engn_chrg_ar_clr_otlt_clnt_tmprtr: 40.0,
        engine_intake_coolant_temperature: 35.0,
        e_it_md_at_sd_ct_ct_ct_dt_tpt: 37.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.et6_intake_coolant_temp, 35.0, 1.0, "et6_intake_coolant_temp");
}

#[test]
fn test_et6_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), ET6::BASE_CAN_ID)), "ET6 frame missing");
}

// ============================================================================
// LFE1 - Liquid Fuel Economy 1 Tests
// ============================================================================

#[test]
fn test_lfe1_handler_updates_state() {
    let mut state = test_state();
    let msg = LFE1 {
        device_id: external_device(),
        engine_fuel_rate: 15.5,
        engine_instantaneous_fuel_economy: 6.2,
        engine_average_fuel_economy: 7.0,
        engine_throttle_valve_1_position_1: 45.0,
        engine_throttle_valve_2_position: 20.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.lfe1_fuel_rate, 15.5, 0.5, "lfe1_fuel_rate");
    assert_float_near(state.sensors.lfe1_instant_fuel_economy, 6.2, 0.5, "lfe1_instant_fuel_economy");
    assert_float_near(state.sensors.lfe1_throttle_valve_1_pos, 45.0, 1.0, "lfe1_throttle_valve_1");
}

#[test]
fn test_lfe1_broadcast_round_trip() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let lfe1_frame = frames.iter()
        .find(|f| matches_base_id(f.raw_id(), LFE1::BASE_CAN_ID))
        .expect("LFE1 frame not found");
    let decoded = LFE1::decode(lfe1_frame.raw_id(), lfe1_frame.data()).unwrap();
    assert_float_near(decoded.engine_fuel_rate, state.sensors.lfe1_fuel_rate, 0.5, "LFE1 round-trip fuel rate");
}

// ============================================================================
// LFE2 - Liquid Fuel Economy 2 Tests
// ============================================================================

#[test]
fn test_lfe2_handler_updates_state() {
    let mut state = test_state();
    let msg = LFE2 {
        device_id: external_device(),
        engine_fuel_rate_high_resolution: 12.3,
        engine_diesel_fuel_demand_rate: 11.5,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.lfe2_fuel_rate_hr, 12.3, 0.5, "lfe2_fuel_rate_hr");
}

#[test]
fn test_lfe2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), LFE2::BASE_CAN_ID)), "LFE2 frame missing");
}

// ============================================================================
// IC1 - Intake/Exhaust Conditions 1 Tests
// ============================================================================

#[test]
fn test_ic1_handler_updates_state() {
    let mut state = test_state();
    let msg = IC1 {
        device_id: external_device(),
        atttt_1_ds_ptt_ft_it_pss_us_sp_3609: 110.0,
        engine_intake_manifold_1_pressure: 150.0,
        engn_intk_mnfld_1_tmprtr: 55.0,
        engine_intake_air_pressure: 101.0,
        engn_ar_fltr_1_dffrntl_prssr: 2.5,
        engine_exhaust_temperature: 400.0,
        engn_clnt_fltr_dffrntl_prssr: 10.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.ic1_intake_manifold_pressure, 150.0, 2.0, "ic1_manifold_pressure");
    assert_float_near(state.sensors.ic1_exhaust_temp, 400.0, 5.0, "ic1_exhaust_temp");
}

#[test]
fn test_ic1_broadcast_round_trip() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let ic1_frame = frames.iter()
        .find(|f| matches_base_id(f.raw_id(), IC1::BASE_CAN_ID))
        .expect("IC1 frame not found");
    let decoded = IC1::decode(ic1_frame.raw_id(), ic1_frame.data()).unwrap();
    assert_float_near(decoded.engine_intake_manifold_1_pressure, state.sensors.ic1_intake_manifold_pressure, 2.0, "IC1 round-trip");
}

// ============================================================================
// IC2 - Intake/Exhaust Conditions 2 Tests
// ============================================================================

#[test]
fn test_ic2_handler_updates_state() {
    let mut state = test_state();
    let msg = IC2 {
        device_id: external_device(),
        engn_ar_fltr_2_dffrntl_prssr: 1.5,
        engn_ar_fltr_3_dffrntl_prssr: 1.2,
        engn_ar_fltr_4_dffrntl_prssr: 0.9,
        engine_intake_manifold_2_pressure: 105.0,
        engn_intk_mnfld_1_aslt_prssr: 102.0,
        engn_intk_mnfld_1_aslt_prssr_hgh_rsltn: 102.5,
        engn_intk_mnfld_2_aslt_prssr: 101.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.ic2_intake_manifold_2_pressure, 105.0, 2.0, "ic2_manifold_2_pressure");
}

#[test]
fn test_ic2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), IC2::BASE_CAN_ID)), "IC2 frame missing");
}

// ============================================================================
// IC3 - Intake/Exhaust Conditions 3 Tests
// ============================================================================

#[test]
fn test_ic3_handler_updates_state() {
    let mut state = test_state();
    let msg = IC3 {
        device_id: external_device(),
        engine_mixer_1_intake_pressure: 98.0,
        engine_mixer_2_intake_pressure: 97.0,
        engn_intk_mnfld_2_aslt_prssr_hgh_rsltn: 99.0,
        dsrd_engn_intk_mnfld_prssr_hgh_lmt: 210.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.ic3_mixer_1_intake_pressure, 98.0, 2.0, "ic3_mixer_1");
}

#[test]
fn test_ic3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), IC3::BASE_CAN_ID)), "IC3 frame missing");
}

// ============================================================================
// AMB - Ambient Conditions Tests
// ============================================================================

#[test]
fn test_amb_handler_updates_state() {
    let mut state = test_state();
    let msg = AMB {
        device_id: external_device(),
        barometric_pressure: 100.5,
        cab_interior_temperature: 24.0,
        ambient_air_temperature: 28.0,
        engine_intake_1_air_temperature: 33.0,
        road_surface_temperature: 35.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.amb_barometric_pressure, 100.5, 1.0, "amb_barometric_pressure");
    assert_float_near(state.sensors.amb_ambient_temp, 28.0, 2.0, "amb_ambient_temp");
}

#[test]
fn test_amb_broadcast_round_trip() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let amb_frame = frames.iter()
        .find(|f| matches_base_id(f.raw_id(), AMB::BASE_CAN_ID))
        .expect("AMB frame not found");
    let decoded = AMB::decode(amb_frame.raw_id(), amb_frame.data()).unwrap();
    assert_float_near(decoded.barometric_pressure, state.sensors.amb_barometric_pressure, 1.0, "AMB round-trip baro");
}

// ============================================================================
// AMB2 - Ambient Conditions 2 Tests
// ============================================================================

#[test]
fn test_amb2_handler_updates_state() {
    let mut state = test_state();
    let msg = AMB2 {
        device_id: external_device(),
        solar_intensity_percent: 75.0,
        solar_sensor_maximum: 100.0,
        specific_humidity: 10.0,
        calculated_ambient_air_temperature: 27.0,
        brmtr_aslt_prssr_hgh_rsltn: 101.5,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.amb2_solar_intensity, 75.0, 1.0, "amb2_solar_intensity");
}

#[test]
fn test_amb2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), AMB2::BASE_CAN_ID)), "AMB2 frame missing");
}

// ============================================================================
// AMB3 - Ambient Conditions 3 Tests
// ============================================================================

#[test]
fn test_amb3_handler_updates_state() {
    let mut state = test_state();
    let msg = AMB3 {
        device_id: external_device(),
        barometric_absolute_pressure_2: 100.0,
        engine_intake_2_air_temperature: 34.0,
        engn_pwr_drt_rltv_hmdt_dffrn: 5.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.amb3_barometric_abs_pressure_2, 100.0, 1.0, "amb3_baro_2");
}

#[test]
fn test_amb3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), AMB3::BASE_CAN_ID)), "AMB3 frame missing");
}

// ============================================================================
// AMB4 - Ambient Conditions 4 Tests
// ============================================================================

#[test]
fn test_amb4_handler_updates_state() {
    let mut state = test_state();
    let msg = AMB4 {
        device_id: external_device(),
        fuel_specific_humidity: 6.0,
        engine_charge_air_specific_humidity: 9.0,
        fuel_relative_humidity: 35.0,
        engine_charge_air_relative_humidity: 50.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.amb4_fuel_relative_humidity, 35.0, 1.0, "amb4_fuel_rh");
}

#[test]
fn test_amb4_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), AMB4::BASE_CAN_ID)), "AMB4 frame missing");
}

// ============================================================================
// FD2 - Fan Drive 2 Tests
// ============================================================================

#[test]
fn test_fd2_handler_updates_state() {
    let mut state = test_state();
    let msg = FD2 {
        device_id: external_device(),
        estimated_percent_fan_2_speed: 60.0,
        fan_2_drive_state: 2,
        fan_2_speed: 3000.0,
        hydraulic_fan_2_motor_pressure: 800.0,
        fan_2_drive_bypass_command_status: 1.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.fd2_fan_2_speed, 3000.0, 50.0, "fd2_fan_2_speed");
    assert_eq!(state.sensors.fd2_fan_2_drive_state, 2);
}

#[test]
fn test_fd2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), FD2::BASE_CAN_ID)), "FD2 frame missing");
}

// ============================================================================
// DD2 - Dash Display 2 Tests
// ============================================================================

#[test]
fn test_dd2_handler_updates_state() {
    let mut state = test_state();
    let msg = DD2 {
        device_id: external_device(),
        engn_ol_fltr_dffrntl_prssr_extndd_rng: 20.0,
        engine_fuel_2_tank_1_level: 60.0,
        engine_fuel_2_tank_2_level: 50.0,
        engine_fuel_2_tank_3_level: 0.0,
        engine_fuel_2_tank_4_level: 0.0,
        display_remain_powered: 1,
        engine_oil_level_high_low: 2.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.dd2_fuel_2_tank_1_level, 60.0, 1.0, "dd2_fuel_level");
    assert_eq!(state.sensors.dd2_display_remain_powered, 1);
}

#[test]
fn test_dd2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), DD2::BASE_CAN_ID)), "DD2 frame missing");
}

// ============================================================================
// DD3 - Dash Display 3 Tests
// ============================================================================

#[test]
fn test_dd3_handler_updates_state() {
    let mut state = test_state();
    let msg = DD3 {
        device_id: external_device(),
        prdtv_vhl_spd_adjstmnt_indtr_stt: 1,
        prdtv_vhl_spd_adjstmnt_spd: 85.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.sensors.dd3_predictive_speed_adj_indicator_state, 1);
    assert_float_near(state.sensors.dd3_predictive_speed_adj_speed, 85.0, 1.0, "dd3_speed");
}

#[test]
fn test_dd3_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), DD3::BASE_CAN_ID)), "DD3 frame missing");
}

// ============================================================================
// HOURS - Engine Hours Tests
// ============================================================================

#[test]
fn test_hours_handler_updates_state() {
    let mut state = test_state();
    let msg = HOURS {
        device_id: external_device(),
        engine_total_hours_of_operation: 15000.0,
        engine_total_revolutions: 900000000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.hours_engine_total_hours, 15000.0, 1.0, "hours_total");
}

#[test]
fn test_hours_broadcast_round_trip() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let hours_frame = frames.iter()
        .find(|f| matches_base_id(f.raw_id(), HOURS::BASE_CAN_ID))
        .expect("HOURS frame not found");
    let decoded = HOURS::decode(hours_frame.raw_id(), hours_frame.data()).unwrap();
    assert_float_near(decoded.engine_total_hours_of_operation, state.sensors.hours_engine_total_hours, 1.0, "HOURS round-trip");
}

// ============================================================================
// HOURS2 - Engine Hours 2 Tests
// ============================================================================

#[test]
fn test_hours2_handler_updates_state() {
    let mut state = test_state();
    let msg = HOURS2 {
        device_id: external_device(),
        engn_idl_mngmnt_atv_ttl_tm: 4000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.hours2_idle_management_active_total_time, 4000.0, 1.0, "hours2_idle_mgmt");
}

#[test]
fn test_hours2_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), HOURS2::BASE_CAN_ID)), "HOURS2 frame missing");
}

// ============================================================================
// IO - Idle Operation Tests
// ============================================================================

#[test]
fn test_io_handler_updates_state() {
    let mut state = test_state();
    let msg = IO {
        device_id: external_device(),
        engine_total_idle_fuel_used: 1500.0,
        engine_total_idle_hours: 5000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.io_total_idle_fuel_used, 1500.0, 1.0, "io_idle_fuel");
    assert_float_near(state.sensors.io_total_idle_hours, 5000.0, 1.0, "io_idle_hours");
}

#[test]
fn test_io_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), IO::BASE_CAN_ID)), "IO frame missing");
}

// ============================================================================
// FL - Fuel Leakage Tests
// ============================================================================

#[test]
fn test_fl_handler_updates_state() {
    let mut state = test_state();
    let msg = FL {
        device_id: external_device(),
        engine_fuel_leakage_1: 1,
        engine_fuel_leakage_2: 0,
        engine_fluid_bund_level: 15.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.sensors.fl_fuel_leakage_1, 1);
    assert_eq!(state.sensors.fl_fuel_leakage_2, 0);
    assert_float_near(state.sensors.fl_fluid_bund_level, 15.0, 1.0, "fl_bund_level");
}

#[test]
fn test_fl_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    assert!(frames.iter().any(|f| matches_base_id(f.raw_id(), FL::BASE_CAN_ID)), "FL frame missing");
}

// ============================================================================
// LFC1 - Lifetime Fuel Consumption 1 Tests
// ============================================================================

#[test]
fn test_lfc1_handler_updates_state() {
    let mut state = test_state();
    let msg = LFC1 {
        device_id: external_device(),
        engine_trip_fuel: 200.0,
        engine_total_fuel_used: 100000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.sensors.lfc1_trip_fuel, 200.0, 1.0, "lfc1_trip_fuel");
    assert_float_near(state.sensors.lfc1_total_fuel_used, 100000.0, 10.0, "lfc1_total_fuel");
}

#[test]
fn test_lfc1_broadcast_round_trip() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let lfc1_frame = frames.iter()
        .find(|f| matches_base_id(f.raw_id(), LFC1::BASE_CAN_ID))
        .expect("LFC1 frame not found");
    let decoded = LFC1::decode(lfc1_frame.raw_id(), lfc1_frame.data()).unwrap();
    assert_float_near(decoded.engine_trip_fuel, state.sensors.lfc1_trip_fuel, 1.0, "LFC1 round-trip trip fuel");
}

// ============================================================================
// Physics Simulation Tests
// ============================================================================

#[test]
fn test_physics_et1_coolant_rises_with_load() {
    let mut state = test_state();
    // Set high RPM to drive engine_load through existing engine physics
    // engine_load = ((RPM - 800) / 2000 * 100), so 2800 RPM => 100% load
    state.engine.engine_speed = 2800.0;
    state.motor.mg1_actual_speed = 50.0; // Drive motor load to keep RPM high
    state.motor.mg1_speed_setpoint = 50.0;
    state.sensors.et1_coolant_temp = 75.0; // Start cool
    // Run physics for a while
    for _ in 0..200 {
        state.update_physics(0.1);
    }
    // Engine load should be driven high by RPM, which raises coolant target
    assert!(
        state.sensors.et1_coolant_temp > 76.0,
        "Coolant temp should rise under load: got {:.1}",
        state.sensors.et1_coolant_temp
    );
}

#[test]
fn test_physics_lfe1_fuel_rate_tracks_load() {
    let mut state = test_state();
    // engine_load is derived from RPM: load = ((RPM - 800) / 2000 * 100)
    // At idle (800 RPM), load = 0%, fuel_rate = 2.0
    // At 1800 RPM, load = 50%, fuel_rate = 7.0
    state.engine.engine_speed = 1800.0;
    state.motor.mg1_actual_speed = 25.0; // Enough motor load to hold RPM
    state.motor.mg1_speed_setpoint = 25.0;
    // Run several cycles to let RPM stabilize
    for _ in 0..50 {
        state.update_physics(0.1);
    }
    // After physics, engine_load should be driven by RPM
    // LFE1 fuel rate = 2.0 + load * 0.1
    assert!(
        state.sensors.lfe1_fuel_rate > 2.5,
        "LFE1 fuel rate should increase above idle: got {:.1}",
        state.sensors.lfe1_fuel_rate
    );

    // Now drop to idle
    state.engine.engine_speed = 800.0;
    state.motor.mg1_actual_speed = 0.0;
    state.motor.mg1_speed_setpoint = 0.0;
    for _ in 0..50 {
        state.update_physics(0.1);
    }
    assert_float_near(state.sensors.lfe1_fuel_rate, 2.0, 0.5, "LFE1 fuel rate at idle");
}

#[test]
fn test_physics_dd2_fuel_level_decreases() {
    let mut state = test_state();
    let initial_level = state.sensors.dd2_fuel_2_tank_1_level;
    state.sensors.lfe1_fuel_rate = 10.0; // 10 L/h
    // Run for 360 seconds (0.1 hours)
    for _ in 0..360 {
        state.update_physics(1.0);
    }
    // Should consume 1.0L in 360s at 10L/h, from 200L tank = 0.5% decrease
    assert!(
        state.sensors.dd2_fuel_2_tank_1_level < initial_level,
        "Fuel level should decrease: initial={:.2}, now={:.2}",
        initial_level,
        state.sensors.dd2_fuel_2_tank_1_level
    );
}

#[test]
fn test_physics_hours_increment() {
    let mut state = test_state();
    state.uptime_seconds = 3600; // 1 hour
    state.update_physics(1.0);
    // hours = 12500.5 + 3600/3600 = 12501.5
    assert_float_near(state.sensors.hours_engine_total_hours, 12501.5, 0.5, "HOURS after 1hr uptime");
}

#[test]
fn test_physics_fd2_fan_ramps_above_threshold() {
    let mut state = test_state();
    state.sensors.et1_coolant_temp = 100.0; // Above 90C thermostat threshold
    state.sensors.fd2_estimated_fan_2_speed_pct = 10.0; // Start low
    for _ in 0..50 {
        state.update_physics(0.1);
    }
    // Fan should ramp up significantly with 10C above threshold
    assert!(
        state.sensors.fd2_estimated_fan_2_speed_pct > 20.0,
        "Fan should ramp with high coolant temp: got {:.1}%",
        state.sensors.fd2_estimated_fan_2_speed_pct
    );
}

#[test]
fn test_physics_ic1_manifold_pressure_tracks_turbo() {
    let mut state = test_state();
    state.engine.turbo_speed = 20000.0;
    state.update_physics(1.0);
    // Manifold pressure = 101.3 + (20000/1000) * 5 = 201.3
    assert!(
        state.sensors.ic1_intake_manifold_pressure > 150.0,
        "Manifold pressure should rise with turbo: got {:.1}",
        state.sensors.ic1_intake_manifold_pressure
    );
}

#[test]
fn test_physics_io_idle_hours_accumulate_at_low_load() {
    let mut state = test_state();
    state.engine.engine_load = 5.0; // Low load (idle)
    let initial_idle_hours = state.sensors.io_total_idle_hours;
    // Run for 360 seconds at idle
    for _ in 0..360 {
        state.update_physics(1.0);
    }
    assert!(
        state.sensors.io_total_idle_hours > initial_idle_hours,
        "Idle hours should accumulate at low load"
    );
}

#[test]
fn test_physics_lfc1_total_fuel_increases() {
    let mut state = test_state();
    let initial_total = state.sensors.lfc1_total_fuel_used;
    state.sensors.lfe1_fuel_rate = 10.0;
    for _ in 0..100 {
        state.update_physics(1.0);
    }
    assert!(
        state.sensors.lfc1_total_fuel_used > initial_total,
        "Total fuel used should increase over time"
    );
}

// ============================================================================
// Comprehensive Broadcast Count Test
// ============================================================================

#[test]
fn test_all_batch3_broadcasts_present() {
    let state = test_state();
    let frames = state.generate_can_frames();

    // Check all 23 Batch 3 message types are present
    let batch3_messages: Vec<(&str, u32)> = vec![
        ("ET1", ET1::BASE_CAN_ID),
        ("ET2", ET2::BASE_CAN_ID),
        ("ET3", ET3::BASE_CAN_ID),
        ("ET4", ET4::BASE_CAN_ID),
        ("ET5", ET5::BASE_CAN_ID),
        ("ET6", ET6::BASE_CAN_ID),
        ("LFE1", LFE1::BASE_CAN_ID),
        ("LFE2", LFE2::BASE_CAN_ID),
        ("IC1", IC1::BASE_CAN_ID),
        ("IC2", IC2::BASE_CAN_ID),
        ("IC3", IC3::BASE_CAN_ID),
        ("AMB", AMB::BASE_CAN_ID),
        ("AMB2", AMB2::BASE_CAN_ID),
        ("AMB3", AMB3::BASE_CAN_ID),
        ("AMB4", AMB4::BASE_CAN_ID),
        ("FD2", FD2::BASE_CAN_ID),
        ("DD2", DD2::BASE_CAN_ID),
        ("DD3", DD3::BASE_CAN_ID),
        ("HOURS", HOURS::BASE_CAN_ID),
        ("HOURS2", HOURS2::BASE_CAN_ID),
        ("IO", IO::BASE_CAN_ID),
        ("FL", FL::BASE_CAN_ID),
        ("LFC1", LFC1::BASE_CAN_ID),
    ];

    for (name, base_id) in &batch3_messages {
        assert!(
            frames.iter().any(|f| matches_base_id(f.raw_id(), *base_id)),
            "Missing broadcast frame for {}",
            name
        );
    }
}

// ============================================================================
// Self-reception filtering test
// ============================================================================

#[test]
fn test_batch3_self_reception_ignored() {
    let mut state = test_state();
    // Send an ET1 from the simulator's own device ID (0x82) - should be ignored
    let msg = ET1 {
        device_id: DeviceId::from(0x82), // Same as simulator
        engine_coolant_temperature: 200.0, // Extreme value to detect if processed
        engine_fuel_1_temperature_1: 200.0,
        engine_oil_temperature_1: 200.0,
        engn_trhrgr_1_ol_tmprtr: 200.0,
        engine_intercooler_temperature: 200.0,
        engn_chrg_ar_clr_thrmstt_opnng: 100.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Ignored);
    // State should not change from defaults
    assert_float_near(state.sensors.et1_coolant_temp, 85.0, 1.0, "et1 should remain at default after self-reception");
}

// ============================================================================
// DecodeFailed test
// ============================================================================

#[test]
fn test_batch3_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = ET1::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
