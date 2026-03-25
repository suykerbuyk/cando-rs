//! Property-based round-trip tests for encode/decode primitives and J1939 messages.
//!
//! These tests verify that encoding followed by decoding produces the original
//! value (or a value within acceptable tolerance for scaled signals).

use proptest::prelude::*;

use cando_messages::common::DeviceId;
use cando_messages::encoder::{
    apply_inverse_scaling, apply_scaling, embed_device_id, extract_device_id, extract_signal,
    pack_signal,
};
use cando_messages::j1939::{AMB, CCVS1, EEC1, EEC2};

// ============================================================================
// Signal Packing Round-Trip
// ============================================================================

proptest! {
    #[test]
    fn pack_extract_roundtrip_8bit(value in 0u64..256) {
        let mut data = [0u8; 8];
        pack_signal(&mut data, 0, 8, value).unwrap();
        let extracted = extract_signal(&data, 0, 8).unwrap();
        prop_assert_eq!(extracted, value);
    }

    #[test]
    fn pack_extract_roundtrip_16bit(value in 0u64..65536) {
        let mut data = [0u8; 8];
        pack_signal(&mut data, 0, 16, value).unwrap();
        let extracted = extract_signal(&data, 0, 16).unwrap();
        prop_assert_eq!(extracted, value);
    }

    #[test]
    fn pack_extract_roundtrip_arbitrary(
        start_bit in 0usize..32,
        length in 1usize..17,
        value in any::<u64>(),
    ) {
        // Ensure we fit within 8 bytes
        prop_assume!((start_bit + length + 7) / 8 <= 8);
        let max_val = if length >= 64 { u64::MAX } else { (1u64 << length) - 1 };
        let clamped = value & max_val;

        let mut data = [0u8; 8];
        pack_signal(&mut data, start_bit, length, clamped).unwrap();
        let extracted = extract_signal(&data, start_bit, length).unwrap();
        prop_assert_eq!(extracted, clamped);
    }
}

// ============================================================================
// Scaling Round-Trip
// ============================================================================

proptest! {
    #[test]
    fn scaling_roundtrip_unsigned(
        raw in 0u64..256,
        factor in 0.1f64..10.0,
        offset in -100.0f64..100.0,
    ) {
        prop_assume!(factor.abs() > 0.001);
        let physical = apply_scaling(raw, factor, offset, false, 8);
        let recovered = apply_inverse_scaling(physical, factor, offset, 8);
        // Should recover original value (within rounding)
        let diff = (recovered as i64 - raw as i64).unsigned_abs();
        prop_assert!(diff <= 1, "raw={}, physical={}, recovered={}", raw, physical, recovered);
    }

    #[test]
    fn scaling_roundtrip_16bit(
        raw in 0u64..65536,
    ) {
        // Engine speed: factor=0.125, offset=0, unsigned 16-bit
        let physical = apply_scaling(raw, 0.125, 0.0, false, 16);
        let recovered = apply_inverse_scaling(physical, 0.125, 0.0, 16);
        let diff = (recovered as i64 - raw as i64).unsigned_abs();
        prop_assert!(diff <= 1, "raw={}, physical={}, recovered={}", raw, physical, recovered);
    }
}

// ============================================================================
// Device ID Round-Trip
// ============================================================================

proptest! {
    #[test]
    fn device_id_roundtrip_pdu2(device_raw in 0u8..=255) {
        // PDU2 base ID: PF=0xF3 (>= 240), so source address is in lower 8 bits
        let base_id = 0x18F37000u32;
        let device = DeviceId::from(device_raw);
        let can_id = embed_device_id(base_id, device, None);
        let extracted = extract_device_id(can_id);
        prop_assert_eq!(extracted, device,
            "PDU2 roundtrip failed: embedded {:02X}, extracted {:02X}",
            device_raw, extracted.as_u8());
    }

    #[test]
    fn device_id_roundtrip_pdu1(
        dest in 0u8..240,   // Normal device range for true PDU1
        source in 0u8..240,
    ) {
        // PDU1: PF < 240, destination in bits 15-8, source in bits 7-0
        prop_assume!(dest != source);  // PDU1 heuristic requires DA != SA
        let base_id = 0x187D0000u32;  // PF=0x7D (125 < 240)
        let device = DeviceId::from(dest);
        let can_id = embed_device_id(base_id, device, Some(source));
        let extracted = extract_device_id(can_id);
        prop_assert_eq!(extracted, device,
            "PDU1 roundtrip failed: embedded dest={:02X} src={:02X}, extracted {:02X}",
            dest, source, extracted.as_u8());
    }
}

// ============================================================================
// Full Message Encode/Decode Round-Trips
// ============================================================================

proptest! {
    #[test]
    fn eec1_roundtrip(
        device_id in 0u8..=255,
        torque_mode in 0u8..16,
        speed in 0.0f64..8031.0,
        percent_torque in 0.0f64..125.0,
    ) {
        let msg = EEC1 {
            device_id: DeviceId::from(device_id),
            engine_torque_mode: torque_mode,
            atl_engn_prnt_trq_frtnl: 0.0,
            drvr_s_dmnd_engn_prnt_trq: percent_torque,
            actual_engine_percent_torque: percent_torque,
            engine_speed: speed,
            sr_addrss_of_cntrllng_dv_fr_engn_cntrl: 0,
            engine_starter_mode: 0,
            engine_demand_percent_torque: 0.0,
        };

        let (can_id, data) = msg.encode().unwrap();
        let decoded = EEC1::decode(can_id, &data).unwrap();

        // Engine speed has 0.125 resolution, so allow tolerance
        let speed_diff = (decoded.engine_speed - msg.engine_speed).abs();
        prop_assert!(speed_diff < 0.13,
            "Speed mismatch: {} vs {}", msg.engine_speed, decoded.engine_speed);

        prop_assert_eq!(decoded.engine_torque_mode, msg.engine_torque_mode);
    }

    #[test]
    fn eec2_roundtrip(
        device_id in 0u8..=255,
        accel_pedal in 0.0f64..100.0,
    ) {
        let msg = EEC2 {
            device_id: DeviceId::from(device_id),
            accelerator_pedal_1_low_idle_switch: 0,
            accelerator_pedal_kickdown_switch: 0,
            road_speed_limit_status: 0,
            accelerator_pedal_2_low_idle_switch: 0,
            accelerator_pedal_1_position: accel_pedal,
            engine_percent_load_at_current_speed: 0,
            remote_accelerator_pedal_position: 0.0,
            accelerator_pedal_2_position: 0.0,
            vhl_alrtn_rt_lmt_stts: 0,
            mmntr_engn_mxmm_pwr_enl_fdk: 0,
            dpf_thermal_management_active: 0,
            scr_thermal_management_active: 0,
            atl_mxmm_avll_engn_prnt_trq: 0.0,
            estimated_pumping_percent_torque: 0.0,
        };

        let (can_id, data) = msg.encode().unwrap();
        let decoded = EEC2::decode(can_id, &data).unwrap();

        let pedal_diff = (decoded.accelerator_pedal_1_position - msg.accelerator_pedal_1_position).abs();
        prop_assert!(pedal_diff < 0.5,
            "Pedal mismatch: {} vs {}", msg.accelerator_pedal_1_position, decoded.accelerator_pedal_1_position);
    }

    #[test]
    fn amb_roundtrip(
        device_id in 0u8..=255,
        baro_pressure in 0.0f64..125.0,
        ambient_temp in -40.0f64..210.0,
    ) {
        let msg = AMB {
            device_id: DeviceId::from(device_id),
            barometric_pressure: baro_pressure,
            cab_interior_temperature: 22.0,
            ambient_air_temperature: ambient_temp,
            engine_intake_1_air_temperature: 25.0,
            road_surface_temperature: 20.0,
        };

        let (can_id, data) = msg.encode().unwrap();
        let decoded = AMB::decode(can_id, &data).unwrap();

        let baro_diff = (decoded.barometric_pressure - msg.barometric_pressure).abs();
        prop_assert!(baro_diff < 0.6,
            "Baro mismatch: {} vs {}", msg.barometric_pressure, decoded.barometric_pressure);

        let temp_diff = (decoded.ambient_air_temperature - msg.ambient_air_temperature).abs();
        prop_assert!(temp_diff < 1.1,
            "Temp mismatch: {} vs {}", msg.ambient_air_temperature, decoded.ambient_air_temperature);
    }

    #[test]
    fn ccvs1_roundtrip(
        device_id in 0u8..=255,
        wheel_speed in 0.0f64..250.0,
    ) {
        let msg = CCVS1 {
            device_id: DeviceId::from(device_id),
            two_speed_axle_switch: 0,
            parking_brake_switch: 0,
            cruise_control_pause_switch: 0,
            park_brake_release_inhibit_request: 0,
            wheel_based_vehicle_speed: wheel_speed,
            cruise_control_active: 0,
            cruise_control_enable_switch: 0,
            brake_switch: 0,
            clutch_switch: 0,
            cruise_control_set_switch: 0,
            crs_cntrl_cst_dlrt_swth: 0,
            cruise_control_resume_switch: 0,
            cruise_control_accelerate_switch: 0,
            cruise_control_set_speed: 0,
            pto_governor_state: 0,
            cruise_control_states: 0,
            engine_idle_increment_switch: 0,
            engine_idle_decrement_switch: 0,
            engine_diagnostic_test_mode_switch: 0,
            engine_shutdown_override_switch: 0,
        };

        let (can_id, data) = msg.encode().unwrap();
        let decoded = CCVS1::decode(can_id, &data).unwrap();

        let speed_diff = (decoded.wheel_based_vehicle_speed - msg.wheel_based_vehicle_speed).abs();
        prop_assert!(speed_diff < 0.5,
            "Speed mismatch: {} vs {}", msg.wheel_based_vehicle_speed, decoded.wheel_based_vehicle_speed);
    }
}
