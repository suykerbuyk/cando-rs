//! Batch 4: Vehicle Speed, Distance & Wheels Tests
//!
//! Tests for 25 vehicle-related messages:
//! CCVS1-6, VD, VDS, VDS2, HRW, VW, TIRE1, TIRE2, SSI, VEP1-3,
//! AS1, AS2, EP, TD, OEL, SHUTDN, BSA, GFI1

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
// CCVS1 - Cruise Control / Vehicle Speed 1
// ============================================================================

#[test]
fn test_ccvs1_handler_updates_state() {
    let mut state = test_state();
    let msg = CCVS1 {
        device_id: external_device(),
        two_speed_axle_switch: 3,
        parking_brake_switch: 1,
        cruise_control_pause_switch: 0,
        park_brake_release_inhibit_request: 0,
        wheel_based_vehicle_speed: 88.5,
        cruise_control_active: 1,
        cruise_control_enable_switch: 1,
        brake_switch: 0,
        clutch_switch: 0,
        cruise_control_set_switch: 0,
        crs_cntrl_cst_dlrt_swth: 0,
        cruise_control_resume_switch: 0,
        cruise_control_accelerate_switch: 0,
        cruise_control_set_speed: 90,
        pto_governor_state: 0,
        cruise_control_states: 5,
        engine_idle_increment_switch: 0,
        engine_idle_decrement_switch: 0,
        engine_diagnostic_test_mode_switch: 0,
        engine_shutdown_override_switch: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.ccvs1_vehicle_speed, 88.5, 1.0, "ccvs1_vehicle_speed");
    assert_eq!(state.vehicle.ccvs1_parking_brake, 1);
    assert_eq!(state.vehicle.ccvs1_cruise_control_active, 1);
}

#[test]
fn test_ccvs1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let ccvs1_frames: Vec<_> = frames
        .iter()
        .filter(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, CCVS1::BASE_CAN_ID))
        .collect();
    assert!(
        !ccvs1_frames.is_empty(),
        "CCVS1 frame should be present in broadcast"
    );
}

// ============================================================================
// CCVS2 - Cruise Control / Vehicle Speed 2
// ============================================================================

#[test]
fn test_ccvs2_handler_updates_state() {
    let mut state = test_state();
    let msg = CCVS2 {
        device_id: external_device(),
        cruise_control_disable_command: 1,
        cruise_control_resume_command: 0,
        cruise_control_pause_command: 0,
        cruise_control_set_command: 0,
        idle_speed_request: 850.0,
        idle_control_enable_state: 0,
        idle_control_request_activation: 0,
        remote_vehicle_speed_limit_request: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.ccvs2_cruise_disable_command, 1);
    assert_float_near(state.vehicle.ccvs2_idle_speed_request, 850.0, 2.0, "ccvs2_idle_speed_request");
}

// ============================================================================
// CCVS3 - Cruise Control / Vehicle Speed 3
// ============================================================================

#[test]
fn test_ccvs3_handler_updates_state() {
    let mut state = test_state();
    let msg = CCVS3 {
        device_id: external_device(),
        adptv_crs_cntrl_rdnss_stts: 1,
        cruise_control_system_command_state: 2,
        prdtv_crs_cntrl_st_spd_offst_stts: 0,
        s_addss_o_ct_dv_f_ds_cs_ct: 0xFF,
        s_addss_o_ct_dv_f_ps_cs_ct: 0xFF,
        aebs_readiness_state: 0,
        crs_cntrl_drvr_cnlltn_stts: 0,
        pwrtrn_asr_as_rspns_rdnss_stts: 0,
        pwrtrn_rp_y_rspns_rdnss_stts: 0,
        crs_cntrl_st_spd_hgh_rsltn: 100.0,
        cruise_control_speed: 100.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.ccvs3_adaptive_cc_readiness, 1);
    assert_float_near(state.vehicle.ccvs3_cruise_control_speed, 100.0, 1.0, "ccvs3_cruise_control_speed");
}

// ============================================================================
// CCVS4 - Cruise Control / Vehicle Speed 4
// ============================================================================

#[test]
fn test_ccvs4_handler_updates_state() {
    let mut state = test_state();
    let msg = CCVS4 {
        device_id: external_device(),
        appld_vhl_spd_lmt_hgh_rsltn: 110.0,
        crs_cntrl_adjstd_mxmm_spd: 130.0,
        engn_extrnl_idl_rqst_fdk: 0,
        s_addss_o_ct_dv_f_stt_cs_ct: 0xFF,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.ccvs4_applied_speed_limit, 110.0, 1.0, "ccvs4_applied_speed_limit");
}

// ============================================================================
// CCVS5 - Cruise Control / Vehicle Speed 5
// ============================================================================

#[test]
fn test_ccvs5_handler_updates_state() {
    let mut state = test_state();
    let msg = CCVS5 {
        device_id: external_device(),
        directional_vehicle_speed: 65.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.ccvs5_directional_speed, 65.0, 1.0, "ccvs5_directional_speed");
}

// ============================================================================
// CCVS6 - Cruise Control / Vehicle Speed 6
// ============================================================================

#[test]
fn test_ccvs6_handler_updates_state() {
    let mut state = test_state();
    let msg = CCVS6 {
        device_id: external_device(),
        crrnt_rdw_vhl_spd_lmt_md: 2,
        sltd_rdw_vhl_spd_lmt: 80.0,
        current_roadway_vehicle_speed_limit: 80.0,
        map_based_vehicle_speed_limit: 80.0,
        crrnt_rdw_vhl_spd_lmt_dgrdtn_stts: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.ccvs6_roadway_speed_limit_mode, 2);
    assert_float_near(state.vehicle.ccvs6_roadway_speed_limit, 80.0, 1.0, "ccvs6_roadway_speed_limit");
}

// ============================================================================
// VD - Vehicle Distance
// ============================================================================

#[test]
fn test_vd_handler_updates_state() {
    let mut state = test_state();
    let msg = VD {
        device_id: external_device(),
        trip_distance: 150.5,
        total_vehicle_distance: 50000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.vd_trip_distance, 150.5, 1.0, "vd_trip_distance");
    assert_float_near(state.vehicle.vd_total_distance, 50000.0, 1.0, "vd_total_distance");
}

#[test]
fn test_vd_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let vd_frames: Vec<_> = frames
        .iter()
        .filter(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, VD::BASE_CAN_ID))
        .collect();
    assert!(!vd_frames.is_empty(), "VD frame should be present in broadcast");
}

#[test]
fn test_vd_distance_accumulates() {
    let mut state = test_state();
    let initial_trip = state.vehicle.vd_trip_distance;
    let initial_total = state.vehicle.vd_total_distance;

    // Set a vehicle speed (need gear ratio > 0 for speed calculation)
    state.transmission.etc2_transmission_actual_gear_ratio = 3.0;
    state.engine.engine_speed = 2000.0;

    // Run physics
    state.update_physics(1.0);

    // Speed should now be > 0 (engine_speed / gear_ratio * factor)
    assert!(state.vehicle.ccvs1_vehicle_speed > 0.0, "Vehicle speed should be > 0");

    // Run physics again so distance accumulates
    state.update_physics(1.0);

    assert!(
        state.vehicle.vd_trip_distance > initial_trip,
        "Trip distance should increase when moving"
    );
    assert!(
        state.vehicle.vd_total_distance > initial_total,
        "Total distance should increase when moving"
    );
}

// ============================================================================
// VDS - Vehicle Direction/Speed
// ============================================================================

#[test]
fn test_vds_handler_updates_state() {
    let mut state = test_state();
    let msg = VDS {
        device_id: external_device(),
        compass_bearing: 180.0,
        navigation_based_vehicle_speed: 60.0,
        pitch: 2.5,
        altitude: 500.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.vds_compass_bearing, 180.0, 1.0, "vds_compass_bearing");
    assert_float_near(state.vehicle.vds_nav_speed, 60.0, 1.0, "vds_nav_speed");
    assert_float_near(state.vehicle.vds_altitude, 500.0, 1.0, "vds_altitude");
}

// ============================================================================
// VDS2 - Vehicle Direction/Speed 2
// ============================================================================

#[test]
fn test_vds2_handler_updates_state() {
    let mut state = test_state();
    let msg = VDS2 {
        device_id: external_device(),
        vehicle_roll: 5.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.vds2_vehicle_roll, 5.0, 1.0, "vds2_vehicle_roll");
}

// ============================================================================
// HRW - High Resolution Wheel Speed
// ============================================================================

#[test]
fn test_hrw_handler_updates_state() {
    let mut state = test_state();
    let msg = HRW {
        device_id: external_device(),
        front_axle_left_wheel_speed: 80.0,
        front_axle_right_wheel_speed: 80.5,
        rear_axle_left_wheel_speed: 79.8,
        rear_axle_right_wheel_speed: 80.2,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.hrw_front_left_speed, 80.0, 1.0, "hrw_front_left_speed");
    assert_float_near(state.vehicle.hrw_front_right_speed, 80.5, 1.0, "hrw_front_right_speed");
}

#[test]
fn test_hrw_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let hrw_frames: Vec<_> = frames
        .iter()
        .filter(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, HRW::BASE_CAN_ID))
        .collect();
    assert!(
        !hrw_frames.is_empty(),
        "HRW frame should be present in broadcast"
    );
}

#[test]
fn test_hrw_tracks_vehicle_speed() {
    let mut state = test_state();
    state.transmission.etc2_transmission_actual_gear_ratio = 3.0;
    state.engine.engine_speed = 2000.0;

    state.update_physics(1.0);

    let base = state.vehicle.ccvs1_vehicle_speed;
    if base > 0.0 {
        // All wheel speeds should be approximately equal to vehicle speed
        assert_float_near(state.vehicle.hrw_front_left_speed, base, 1.0, "hrw_fl_tracks_speed");
        assert_float_near(state.vehicle.hrw_front_right_speed, base, 1.0, "hrw_fr_tracks_speed");
        assert_float_near(state.vehicle.hrw_rear_left_speed, base, 1.0, "hrw_rl_tracks_speed");
        assert_float_near(state.vehicle.hrw_rear_right_speed, base, 1.0, "hrw_rr_tracks_speed");
    }
}

// ============================================================================
// VW - Vehicle Weight
// ============================================================================

#[test]
fn test_vw_handler_updates_state() {
    let mut state = test_state();
    let msg = VW {
        device_id: external_device(),
        axle_location: 2,
        axle_weight: 7500.0,
        trailer_weight: 10000.0,
        cargo_weight: 3000.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.vw_axle_location, 2);
    assert_float_near(state.vehicle.vw_cargo_weight, 3000.0, 10.0, "vw_cargo_weight");
}

// ============================================================================
// TIRE1 - Tire Condition 1
// ============================================================================

#[test]
fn test_tire1_handler_updates_state() {
    let mut state = test_state();
    let msg = TIRE1 {
        device_id: external_device(),
        tire_location: 3,
        tire_pressure: 750.0,
        tire_temperature: 40.0,
        tire_sensor_enable_status: 1,
        tire_status: 0,
        tire_sensor_electrical_fault: 0,
        extended_tire_pressure_support: 0,
        tire_air_leakage_rate: 0.1,
        tire_pressure_threshold_detection: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.tire1_location, 3);
    assert_float_near(state.vehicle.tire1_pressure, 750.0, 5.0, "tire1_pressure");
    assert_float_near(state.vehicle.tire1_temperature, 40.0, 2.0, "tire1_temperature");
}

// ============================================================================
// TIRE2 - Tire Condition 2
// ============================================================================

#[test]
fn test_tire2_handler_updates_state() {
    let mut state = test_state();
    let msg = TIRE2 {
        device_id: external_device(),
        tire_location: 5,
        tire_pressure_extended_range: 900,
        required_tire_pressure: 850,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.tire2_location, 5);
    assert_eq!(state.vehicle.tire2_pressure_extended, 900);
    assert_eq!(state.vehicle.tire2_required_pressure, 850);
}

// ============================================================================
// SSI - Slope Sensor Information
// ============================================================================

#[test]
fn test_ssi_handler_updates_state() {
    let mut state = test_state();
    let msg = SSI {
        device_id: external_device(),
        pitch_angle: 3.5,
        roll_angle: -1.2,
        pitch_rate: 0.5,
        pitch_angle_figure_of_merit: 3,
        roll_angle_figure_of_merit: 3,
        pitch_rate_figure_of_merit: 3,
        pitch_and_roll_compensated: 1,
        roll_and_pitch_measurement_latency: 0.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.ssi_pitch_angle, 3.5, 1.0, "ssi_pitch_angle");
    assert_float_near(state.vehicle.ssi_roll_angle, -1.2, 1.0, "ssi_roll_angle");
}

#[test]
fn test_ssi_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let ssi_frames: Vec<_> = frames
        .iter()
        .filter(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, SSI::BASE_CAN_ID))
        .collect();
    assert!(
        !ssi_frames.is_empty(),
        "SSI frame should be present in broadcast"
    );
}

// ============================================================================
// VEP1 - Vehicle Electrical Power 1
// ============================================================================

#[test]
fn test_vep1_handler_updates_state() {
    let mut state = test_state();
    let msg = VEP1 {
        device_id: external_device(),
        sli_battery_1_net_current: 10.0,
        alternator_current: 50,
        charging_system_potential_voltage: 14.4,
        battery_potential_power_input_1: 12.9,
        key_switch_battery_potential: 12.7,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.vep1_battery_potential, 12.9, 0.5, "vep1_battery_potential");
    assert_eq!(state.vehicle.vep1_alternator_current, 50);
}

#[test]
fn test_vep1_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let vep1_frames: Vec<_> = frames
        .iter()
        .filter(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, VEP1::BASE_CAN_ID))
        .collect();
    assert!(
        !vep1_frames.is_empty(),
        "VEP1 frame should be present in broadcast"
    );
}

// ============================================================================
// VEP2 - Vehicle Electrical Power 2
// ============================================================================

#[test]
fn test_vep2_handler_updates_state() {
    let mut state = test_state();
    let msg = VEP2 {
        device_id: external_device(),
        battery_potential_power_input_2: 12.5,
        ecu_power_output_supply_voltage_1: 12.0,
        ecu_power_output_supply_voltage_2: 5.0,
        ecu_power_output_supply_voltage_3: 3.3,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.vep2_battery_potential_2, 12.5, 0.5, "vep2_battery_potential_2");
}

// ============================================================================
// VEP3 - Vehicle Electrical Power 3
// ============================================================================

#[test]
fn test_vep3_handler_updates_state() {
    let mut state = test_state();
    let msg = VEP3 {
        device_id: external_device(),
        altrntr_crrnt_hgh_rng_rsltn: 55.0,
        sl_bttr_1_nt_crrnt_hgh_rng_rsltn: 10.0,
        sli_battery_2_net_current: 5.0,
        ecu_key_switch_state: 2,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.vep3_alternator_current_hr, 55.0, 1.0, "vep3_alternator_current_hr");
    assert_eq!(state.vehicle.vep3_key_switch_state, 2);
}

// ============================================================================
// AS1 - Alternator Speed 1
// ============================================================================

#[test]
fn test_as1_handler_updates_state() {
    let mut state = test_state();
    let msg = AS1 {
        device_id: external_device(),
        alternator_speed: 3000.0,
        alternator_1_status: 1,
        alternator_2_status: 0,
        alternator_3_status: 0,
        alternator_4_status: 0,
        altrntr_eltrl_flr_stts: 0,
        altrntr_mhnl_flr_stts: 0,
        altrntr_hgh_tmprtr_wrnng_stts: 0,
        alternator_lin_timeout_detected: 0,
        altrntr_ln_cmmntn_flr_stts: 0,
        alternator_load_balancing_status: 0,
        alternator_excitation_status: 1,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.as1_alternator_speed, 3000.0, 10.0, "as1_alternator_speed");
    assert_eq!(state.vehicle.as1_alternator_1_status, 1);
}

// ============================================================================
// AS2 - Alternator Speed 2
// ============================================================================

#[test]
fn test_as2_handler_updates_state() {
    let mut state = test_state();
    let msg = AS2 {
        device_id: external_device(),
        altrntr_stpnt_vltg_fdk: 14.4,
        alternator_output_voltage: 14.2,
        altrntr_vltg_rgltr_tmprtr: 70.0,
        alternator_excitation_current: 4.0,
        alternator_excitation_duty_cycle: 60.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.as2_output_voltage, 14.2, 0.5, "as2_output_voltage");
    assert_float_near(state.vehicle.as2_excitation_duty_cycle, 60.0, 1.0, "as2_excitation_duty_cycle");
}

// ============================================================================
// EP - Electronic Process
// ============================================================================

#[test]
fn test_ep_handler_updates_state() {
    let mut state = test_state();
    let msg = EP {
        device_id: external_device(),
        keep_alive_battery_consumption: 10,
        data_memory_usage: 50.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.ep_keep_alive_consumption, 10);
    assert_float_near(state.vehicle.ep_data_memory_usage, 50.0, 1.0, "ep_data_memory_usage");
}

// ============================================================================
// TD - Time/Date
// ============================================================================

#[test]
fn test_td_handler_updates_state() {
    let mut state = test_state();
    let msg = TD {
        device_id: external_device(),
        seconds: 30.0,
        minutes: 45,
        hours: 14,
        month: 3,
        day: 24.0,
        year: 2026.0,
        local_minute_offset: 0.0,
        local_hour_offset: -5.0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.td_minutes, 45);
    assert_eq!(state.vehicle.td_hours, 14);
    assert_eq!(state.vehicle.td_month, 3);
    assert_float_near(state.vehicle.td_year, 2026.0, 1.0, "td_year");
}

#[test]
fn test_td_broadcast_present() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let td_frames: Vec<_> = frames
        .iter()
        .filter(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, TD::BASE_CAN_ID))
        .collect();
    assert!(
        !td_frames.is_empty(),
        "TD frame should be present in broadcast"
    );
}

// ============================================================================
// OEL - Operator External Light Controls
// ============================================================================

#[test]
fn test_oel_handler_updates_state() {
    let mut state = test_state();
    let msg = OEL {
        device_id: external_device(),
        work_light_switch: 1,
        main_light_switch: 1,
        turn_signal_switch: 2,
        hazard_light_switch: 0,
        high_low_beam_switch: 1,
        operators_desired_back_light: 0.0,
        oprtrs_dsrd_dld_lmp_off_tm: 0,
        exterior_lamp_check_switch: 0,
        headlamp_emergency_flash_switch: 0,
        auxiliary_lamp_group_switch: 0,
        auto_high_low_beam_enable_switch: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.oel_work_light, 1);
    assert_eq!(state.vehicle.oel_main_light, 1);
    assert_eq!(state.vehicle.oel_turn_signal, 2);
}

// ============================================================================
// SHUTDN - Shutdown
// ============================================================================

#[test]
fn test_shutdn_handler_updates_state() {
    let mut state = test_state();
    let msg = SHUTDN {
        device_id: external_device(),
        engn_idl_shtdwn_hs_shtdwn_engn: 1,
        engn_idl_shtdwn_drvr_alrt_md: 0,
        engine_idle_shutdown_timer_override: 0,
        engine_idle_shutdown_timer_state: 0,
        engine_idle_shutdown_timer_function: 0,
        ac_high_pressure_fan_switch: 0,
        refrigerant_low_pressure_switch: 0,
        refrigerant_high_pressure_switch: 0,
        engine_wait_to_start_lamp: 1,
        mhn_intvt_shtdwn_hs_shtdwn_engn: 0,
        engn_prttn_sstm_hs_shtdwn_engn: 0,
        engn_prttn_sstm_apprhng_shtdwn: 0,
        engn_prttn_sstm_tmr_ovrrd: 0,
        engn_prttn_sstm_tmr_stt: 0,
        engn_prttn_sstm_cnfgrtn: 0,
        engine_alarm_acknowledge: 0,
        engine_alarm_output_command_status: 0,
        engine_air_shutoff_command_status: 0,
        engine_overspeed_test: 0,
        engine_air_shutoff_status: 0,
        pto_shutdown_has_shutdown_engine: 0,
        clnt_lvl_engn_prttn_shtdwn_stts: 0,
        engine_oil_pressure_switch: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.shutdn_idle_shutdown, 1);
    assert_eq!(state.vehicle.shutdn_wait_to_start, 1);
}

// ============================================================================
// BSA - Brake Stroke Alert
// ============================================================================

#[test]
fn test_bsa_handler_updates_state() {
    let mut state = test_state();
    let msg = BSA {
        device_id: external_device(),
        tractor_brake_stroke_axle_1_left: 1,
        tractor_brake_stroke_axle_1_right: 0,
        tractor_brake_stroke_axle_2_left: 2,
        tractor_brake_stroke_axle_2_right: 1,
        tractor_brake_stroke_axle_3_left: 0,
        tractor_brake_stroke_axle_3_right: 0,
        tractor_brake_stroke_axle_4_left: 0,
        tractor_brake_stroke_axle_4_right: 0,
        tractor_brake_stroke_axle_5_left: 0,
        tractor_brake_stroke_axle_5_right: 0,
        trailer_brake_stroke_axle_1_left: 0,
        trailer_brake_stroke_axle_1_right: 0,
        trailer_brake_stroke_axle_2_left: 0,
        trailer_brake_stroke_axle_2_right: 0,
        trailer_brake_stroke_axle_3_left: 0,
        trailer_brake_stroke_axle_3_right: 0,
        trailer_brake_stroke_axle_4_left: 0,
        trailer_brake_stroke_axle_4_right: 0,
        trailer_brake_stroke_axle_5_left: 0,
        trailer_brake_stroke_axle_5_right: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_eq!(state.vehicle.bsa_axle1_left, 1);
    assert_eq!(state.vehicle.bsa_axle2_left, 2);
}

// ============================================================================
// GFI1 - Gaseous Fuel Information 1
// ============================================================================

#[test]
fn test_gfi1_handler_updates_state() {
    let mut state = test_state();
    let msg = GFI1 {
        device_id: external_device(),
        ttl_engn_pt_gvrnr_fl_usd_gss: 500.0,
        trip_average_fuel_rate_gaseous: 25.0,
        engine_fuel_specific_gravity: 0.82,
    };
    let (can_id, data) = msg.encode().unwrap();
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::Recognized);
    assert_float_near(state.vehicle.gfi1_total_fuel_used, 500.0, 5.0, "gfi1_total_fuel_used");
    assert_float_near(state.vehicle.gfi1_fuel_specific_gravity, 0.82, 0.05, "gfi1_fuel_specific_gravity");
}

// ============================================================================
// Physics Tests
// ============================================================================

#[test]
fn test_vehicle_speed_from_engine_and_gear() {
    let mut state = test_state();

    // In neutral (gear_ratio = 0), speed should be 0
    state.transmission.etc2_transmission_actual_gear_ratio = 0.0;
    state.engine.engine_speed = 3000.0;
    state.update_physics(0.1);
    assert_float_near(state.vehicle.ccvs1_vehicle_speed, 0.0, 0.1, "speed_in_neutral");

    // In gear, speed should be derived from engine speed
    state.transmission.etc2_transmission_actual_gear_ratio = 3.0;
    state.engine.engine_speed = 2000.0;
    state.update_physics(0.1);
    assert!(
        state.vehicle.ccvs1_vehicle_speed > 0.0,
        "Vehicle speed should be > 0 in gear"
    );
}

#[test]
fn test_vep1_battery_tracks_alternator() {
    let mut state = test_state();
    state.engine.engine_speed = 2000.0; // Engine running
    state.vehicle.vep1_battery_potential = 12.0; // Start low

    // Run physics several times to let voltage converge
    for _ in 0..100 {
        state.update_physics(0.1);
    }

    // Charging voltage should be in normal range
    assert!(
        state.vehicle.vep1_charging_voltage > 13.0,
        "Charging voltage should be above 13V with engine running"
    );
    // Battery should be charging toward alternator output
    assert!(
        state.vehicle.vep1_battery_potential > 12.0,
        "Battery potential should increase with charging"
    );
}

#[test]
fn test_alternator_speed_tracks_engine() {
    let mut state = test_state();
    state.engine.engine_speed = 2000.0;
    state.update_physics(0.1);

    // Alternator speed = engine_speed * 2.5
    // Engine speed may shift slightly due to physics, so check with wider tolerance
    let expected_alt_speed = state.engine.engine_speed * 2.5;
    assert_float_near(
        state.vehicle.as1_alternator_speed,
        expected_alt_speed,
        10.0,
        "as1_alternator_speed",
    );
}

#[test]
fn test_tire_temperature_increases_with_speed() {
    let mut state = test_state();
    state.transmission.etc2_transmission_actual_gear_ratio = 3.0;
    state.engine.engine_speed = 3000.0;
    state.update_physics(0.1);

    let speed = state.vehicle.ccvs1_vehicle_speed;
    let tire_temp = state.vehicle.tire1_temperature;

    // At speed > 0, tire temp should be above ambient 25C
    if speed > 0.0 {
        assert!(
            tire_temp > 25.0,
            "Tire temperature should increase with speed, got {}",
            tire_temp
        );
    }
}

// ============================================================================
// Broadcast Completeness Test
// ============================================================================

#[test]
fn test_all_batch4_messages_broadcast() {
    let state = test_state();
    let frames = state.generate_can_frames();
    let frame_ids: Vec<u32> = frames.iter().map(|f| f.raw_id() & 0x1FFFFFFF).collect();

    let expected_msgs: Vec<(&str, u32)> = vec![
        ("CCVS1", CCVS1::BASE_CAN_ID),
        ("CCVS2", CCVS2::BASE_CAN_ID),
        ("CCVS3", CCVS3::BASE_CAN_ID),
        ("CCVS4", CCVS4::BASE_CAN_ID),
        ("CCVS5", CCVS5::BASE_CAN_ID),
        ("CCVS6", CCVS6::BASE_CAN_ID),
        ("VD", VD::BASE_CAN_ID),
        ("VDS", VDS::BASE_CAN_ID),
        ("VDS2", VDS2::BASE_CAN_ID),
        ("HRW", HRW::BASE_CAN_ID),
        ("VW", VW::BASE_CAN_ID),
        ("TIRE1", TIRE1::BASE_CAN_ID),
        ("TIRE2", TIRE2::BASE_CAN_ID),
        ("SSI", SSI::BASE_CAN_ID),
        ("VEP1", VEP1::BASE_CAN_ID),
        ("VEP2", VEP2::BASE_CAN_ID),
        ("VEP3", VEP3::BASE_CAN_ID),
        ("AS1", AS1::BASE_CAN_ID),
        ("AS2", AS2::BASE_CAN_ID),
        ("EP", EP::BASE_CAN_ID),
        ("TD", TD::BASE_CAN_ID),
        ("OEL", OEL::BASE_CAN_ID),
        ("SHUTDN", SHUTDN::BASE_CAN_ID),
        ("BSA", BSA::BASE_CAN_ID),
        ("GFI1", GFI1::BASE_CAN_ID),
    ];

    for (name, base_id) in &expected_msgs {
        let found = frame_ids.iter().any(|&fid| (fid & 0xFFFFFF00) == *base_id);
        assert!(found, "{} (base 0x{:08X}) should be in broadcast", name, base_id);
    }
}

// ============================================================================
// Round-trip Tests (encode -> decode -> verify)
// ============================================================================

#[test]
fn test_ccvs1_roundtrip() {
    let mut state = test_state();
    state.vehicle.ccvs1_vehicle_speed = 72.0;
    state.vehicle.ccvs1_parking_brake = 0;
    state.vehicle.ccvs1_brake_switch = 1;

    let frames = state.generate_can_frames();
    let ccvs1_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, CCVS1::BASE_CAN_ID))
        .expect("CCVS1 frame should exist");

    let decoded =
        CCVS1::decode(ccvs1_frame.raw_id() & 0x1FFFFFFF, ccvs1_frame.data()).unwrap();
    assert_float_near(
        decoded.wheel_based_vehicle_speed,
        72.0,
        1.0,
        "ccvs1_roundtrip_speed",
    );
    assert_eq!(decoded.parking_brake_switch, 0);
    assert_eq!(decoded.brake_switch, 1);
}

#[test]
fn test_vd_roundtrip() {
    let mut state = test_state();
    state.vehicle.vd_trip_distance = 250.5;
    state.vehicle.vd_total_distance = 100000.0;

    let frames = state.generate_can_frames();
    let vd_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, VD::BASE_CAN_ID))
        .expect("VD frame should exist");

    let decoded = VD::decode(vd_frame.raw_id() & 0x1FFFFFFF, vd_frame.data()).unwrap();
    assert_float_near(decoded.trip_distance, 250.5, 1.0, "vd_roundtrip_trip");
    assert_float_near(
        decoded.total_vehicle_distance,
        100000.0,
        1.0,
        "vd_roundtrip_total",
    );
}

#[test]
fn test_vep1_roundtrip() {
    let mut state = test_state();
    state.vehicle.vep1_battery_potential = 13.2;
    state.vehicle.vep1_charging_voltage = 14.4;
    state.vehicle.vep1_alternator_current = 60;

    let frames = state.generate_can_frames();
    let vep1_frame = frames
        .iter()
        .find(|f| matches_base_id(f.raw_id() & 0x1FFFFFFF, VEP1::BASE_CAN_ID))
        .expect("VEP1 frame should exist");

    let decoded = VEP1::decode(vep1_frame.raw_id() & 0x1FFFFFFF, vep1_frame.data()).unwrap();
    assert_float_near(
        decoded.battery_potential_power_input_1,
        13.2,
        0.5,
        "vep1_roundtrip_battery",
    );
    assert_eq!(decoded.alternator_current, 60);
}

// ============================================================================
// Self-Reception & Error Handling Tests
// ============================================================================

#[test]
fn test_ccvs1_self_reception_ignored() {
    let mut state = test_state();
    let msg = CCVS1 {
        device_id: DeviceId::from(0x82),
        two_speed_axle_switch: 3,
        parking_brake_switch: 1,
        cruise_control_pause_switch: 0,
        park_brake_release_inhibit_request: 0,
        wheel_based_vehicle_speed: 88.5,
        cruise_control_active: 1,
        cruise_control_enable_switch: 1,
        brake_switch: 0,
        clutch_switch: 0,
        cruise_control_set_switch: 0,
        crs_cntrl_cst_dlrt_swth: 0,
        cruise_control_resume_switch: 0,
        cruise_control_accelerate_switch: 0,
        cruise_control_set_speed: 90,
        pto_governor_state: 0,
        cruise_control_states: 5,
        engine_idle_increment_switch: 0,
        engine_idle_decrement_switch: 0,
        engine_diagnostic_test_mode_switch: 0,
        engine_shutdown_override_switch: 0,
    };
    let (can_id, data) = msg.encode().unwrap();
    let result = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(result, MessageStatus::Ignored);
}

#[test]
fn test_batch4_decode_failed_on_corrupt_data() {
    let mut state = test_state();
    let can_id = CCVS1::BASE_CAN_ID | 0x42;
    let data = [0xFF, 0xFF]; // Truncated
    let status = state.process_incoming_message(can_id, &data).unwrap();
    assert_eq!(status, MessageStatus::DecodeFailed);
}
