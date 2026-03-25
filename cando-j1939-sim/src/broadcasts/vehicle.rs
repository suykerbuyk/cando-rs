use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_vehicle_frames(
        &self,
        frames: &mut Vec<CanFrame>,
        device_id: DeviceId,
    ) {
        // CCVS1 - Cruise Control / Vehicle Speed 1
        let ccvs1 = CCVS1 {
            device_id,
            two_speed_axle_switch: 3,       // Not available
            parking_brake_switch: self.vehicle.ccvs1_parking_brake,
            cruise_control_pause_switch: 0,
            park_brake_release_inhibit_request: 0,
            wheel_based_vehicle_speed: self.vehicle.ccvs1_vehicle_speed,
            cruise_control_active: self.vehicle.ccvs1_cruise_control_active,
            cruise_control_enable_switch: self.vehicle.ccvs1_cruise_control_enable,
            brake_switch: self.vehicle.ccvs1_brake_switch,
            clutch_switch: self.vehicle.ccvs1_clutch_switch,
            cruise_control_set_switch: 0,
            crs_cntrl_cst_dlrt_swth: 0,
            cruise_control_resume_switch: 0,
            cruise_control_accelerate_switch: 0,
            cruise_control_set_speed: self.vehicle.ccvs1_cruise_control_set_speed,
            pto_governor_state: self.vehicle.ccvs1_pto_governor_state,
            cruise_control_states: self.vehicle.ccvs1_cruise_control_states,
            engine_idle_increment_switch: 0,
            engine_idle_decrement_switch: 0,
            engine_diagnostic_test_mode_switch: 0,
            engine_shutdown_override_switch: 0,
        };

        if let Ok((can_id, data)) = ccvs1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // CCVS2 - Cruise Control / Vehicle Speed 2
        let ccvs2 = CCVS2 {
            device_id,
            cruise_control_disable_command: self.vehicle.ccvs2_cruise_disable_command,
            cruise_control_resume_command: 0,
            cruise_control_pause_command: 0,
            cruise_control_set_command: 0,
            idle_speed_request: self.vehicle.ccvs2_idle_speed_request,
            idle_control_enable_state: 0,
            idle_control_request_activation: 0,
            remote_vehicle_speed_limit_request: 0,
        };

        if let Ok((can_id, data)) = ccvs2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // CCVS3 - Cruise Control / Vehicle Speed 3
        let ccvs3 = CCVS3 {
            device_id,
            adptv_crs_cntrl_rdnss_stts: self.vehicle.ccvs3_adaptive_cc_readiness,
            cruise_control_system_command_state: self.vehicle.ccvs3_cc_system_command_state,
            prdtv_crs_cntrl_st_spd_offst_stts: 0,
            s_addss_o_ct_dv_f_ds_cs_ct: 0xFF,
            s_addss_o_ct_dv_f_ps_cs_ct: 0xFF,
            aebs_readiness_state: 0,
            crs_cntrl_drvr_cnlltn_stts: 0,
            pwrtrn_asr_as_rspns_rdnss_stts: 0,
            pwrtrn_rp_y_rspns_rdnss_stts: 0,
            crs_cntrl_st_spd_hgh_rsltn: self.vehicle.ccvs3_cruise_control_speed,
            cruise_control_speed: self.vehicle.ccvs3_cruise_control_speed,
        };

        if let Ok((can_id, data)) = ccvs3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // CCVS4 - Cruise Control / Vehicle Speed 4
        let ccvs4 = CCVS4 {
            device_id,
            appld_vhl_spd_lmt_hgh_rsltn: self.vehicle.ccvs4_applied_speed_limit,
            crs_cntrl_adjstd_mxmm_spd: self.vehicle.ccvs4_applied_speed_limit,
            engn_extrnl_idl_rqst_fdk: 0,
            s_addss_o_ct_dv_f_stt_cs_ct: 0xFF,
        };

        if let Ok((can_id, data)) = ccvs4.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // CCVS5 - Cruise Control / Vehicle Speed 5
        let ccvs5 = CCVS5 {
            device_id,
            directional_vehicle_speed: self.vehicle.ccvs5_directional_speed,
        };

        if let Ok((can_id, data)) = ccvs5.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // CCVS6 - Cruise Control / Vehicle Speed 6
        let ccvs6 = CCVS6 {
            device_id,
            crrnt_rdw_vhl_spd_lmt_md: self.vehicle.ccvs6_roadway_speed_limit_mode,
            sltd_rdw_vhl_spd_lmt: self.vehicle.ccvs6_roadway_speed_limit,
            current_roadway_vehicle_speed_limit: self.vehicle.ccvs6_roadway_speed_limit,
            map_based_vehicle_speed_limit: 0.0,
            crrnt_rdw_vhl_spd_lmt_dgrdtn_stts: 0,
        };

        if let Ok((can_id, data)) = ccvs6.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // VD - Vehicle Distance
        let vd = VD {
            device_id,
            trip_distance: self.vehicle.vd_trip_distance,
            total_vehicle_distance: self.vehicle.vd_total_distance,
        };

        if let Ok((can_id, data)) = vd.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // VDS - Vehicle Direction/Speed
        let vds = VDS {
            device_id,
            compass_bearing: self.vehicle.vds_compass_bearing,
            navigation_based_vehicle_speed: self.vehicle.vds_nav_speed,
            pitch: self.vehicle.vds_pitch,
            altitude: self.vehicle.vds_altitude,
        };

        if let Ok((can_id, data)) = vds.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // VDS2 - Vehicle Direction/Speed 2
        let vds2 = VDS2 {
            device_id,
            vehicle_roll: self.vehicle.vds2_vehicle_roll,
        };

        if let Ok((can_id, data)) = vds2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HRW - High Resolution Wheel Speed
        let hrw = HRW {
            device_id,
            front_axle_left_wheel_speed: self.vehicle.hrw_front_left_speed,
            front_axle_right_wheel_speed: self.vehicle.hrw_front_right_speed,
            rear_axle_left_wheel_speed: self.vehicle.hrw_rear_left_speed,
            rear_axle_right_wheel_speed: self.vehicle.hrw_rear_right_speed,
        };

        if let Ok((can_id, data)) = hrw.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // VW - Vehicle Weight
        let vw = VW {
            device_id,
            axle_location: self.vehicle.vw_axle_location,
            axle_weight: self.vehicle.vw_axle_weight,
            trailer_weight: self.vehicle.vw_trailer_weight,
            cargo_weight: self.vehicle.vw_cargo_weight,
        };

        if let Ok((can_id, data)) = vw.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // TIRE1 - Tire Condition 1
        let tire1 = TIRE1 {
            device_id,
            tire_location: self.vehicle.tire1_location,
            tire_pressure: self.vehicle.tire1_pressure,
            tire_temperature: self.vehicle.tire1_temperature,
            tire_sensor_enable_status: 1,   // Enabled
            tire_status: self.vehicle.tire1_status,
            tire_sensor_electrical_fault: 0,
            extended_tire_pressure_support: 0,
            tire_air_leakage_rate: self.vehicle.tire1_leakage_rate,
            tire_pressure_threshold_detection: 0,
        };

        if let Ok((can_id, data)) = tire1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // TIRE2 - Tire Condition 2
        let tire2 = TIRE2 {
            device_id,
            tire_location: self.vehicle.tire2_location,
            tire_pressure_extended_range: self.vehicle.tire2_pressure_extended,
            required_tire_pressure: self.vehicle.tire2_required_pressure,
        };

        if let Ok((can_id, data)) = tire2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // SSI - Slope Sensor Information
        let ssi = SSI {
            device_id,
            pitch_angle: self.vehicle.ssi_pitch_angle,
            roll_angle: self.vehicle.ssi_roll_angle,
            pitch_rate: self.vehicle.ssi_pitch_rate,
            pitch_angle_figure_of_merit: 3,     // Excellent
            roll_angle_figure_of_merit: 3,      // Excellent
            pitch_rate_figure_of_merit: 3,      // Excellent
            pitch_and_roll_compensated: 1,      // Compensated
            roll_and_pitch_measurement_latency: 0.0,
        };

        if let Ok((can_id, data)) = ssi.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // VEP1 - Vehicle Electrical Power 1
        let vep1 = VEP1 {
            device_id,
            sli_battery_1_net_current: self.vehicle.vep1_battery_current,
            alternator_current: self.vehicle.vep1_alternator_current,
            charging_system_potential_voltage: self.vehicle.vep1_charging_voltage,
            battery_potential_power_input_1: self.vehicle.vep1_battery_potential,
            key_switch_battery_potential: self.vehicle.vep1_key_switch_voltage,
        };

        if let Ok((can_id, data)) = vep1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // VEP2 - Vehicle Electrical Power 2
        let vep2 = VEP2 {
            device_id,
            battery_potential_power_input_2: self.vehicle.vep2_battery_potential_2,
            ecu_power_output_supply_voltage_1: self.vehicle.vep2_ecu_supply_voltage_1,
            ecu_power_output_supply_voltage_2: self.vehicle.vep2_ecu_supply_voltage_2,
            ecu_power_output_supply_voltage_3: self.vehicle.vep2_ecu_supply_voltage_3,
        };

        if let Ok((can_id, data)) = vep2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // VEP3 - Vehicle Electrical Power 3
        let vep3 = VEP3 {
            device_id,
            altrntr_crrnt_hgh_rng_rsltn: self.vehicle.vep3_alternator_current_hr,
            sl_bttr_1_nt_crrnt_hgh_rng_rsltn: self.vehicle.vep3_battery_current_hr,
            sli_battery_2_net_current: self.vehicle.vep3_battery_2_current,
            ecu_key_switch_state: self.vehicle.vep3_key_switch_state,
        };

        if let Ok((can_id, data)) = vep3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AS1 - Alternator Speed 1
        let as1 = AS1 {
            device_id,
            alternator_speed: self.vehicle.as1_alternator_speed,
            alternator_1_status: self.vehicle.as1_alternator_1_status,
            alternator_2_status: 0,
            alternator_3_status: 0,
            alternator_4_status: 0,
            altrntr_eltrl_flr_stts: 0,
            altrntr_mhnl_flr_stts: 0,
            altrntr_hgh_tmprtr_wrnng_stts: 0,
            alternator_lin_timeout_detected: 0,
            altrntr_ln_cmmntn_flr_stts: 0,
            alternator_load_balancing_status: 0,
            alternator_excitation_status: 1,    // Excited
        };

        if let Ok((can_id, data)) = as1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AS2 - Alternator Speed 2
        let as2 = AS2 {
            device_id,
            altrntr_stpnt_vltg_fdk: self.vehicle.as2_setpoint_voltage_feedback,
            alternator_output_voltage: self.vehicle.as2_output_voltage,
            altrntr_vltg_rgltr_tmprtr: self.vehicle.as2_regulator_temperature,
            alternator_excitation_current: self.vehicle.as2_excitation_current,
            alternator_excitation_duty_cycle: self.vehicle.as2_excitation_duty_cycle,
        };

        if let Ok((can_id, data)) = as2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EP - Electronic Process
        let ep = EP {
            device_id,
            keep_alive_battery_consumption: self.vehicle.ep_keep_alive_consumption,
            data_memory_usage: self.vehicle.ep_data_memory_usage,
        };

        if let Ok((can_id, data)) = ep.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // TD - Time/Date
        let td = TD {
            device_id,
            seconds: self.vehicle.td_seconds,
            minutes: self.vehicle.td_minutes,
            hours: self.vehicle.td_hours,
            month: self.vehicle.td_month,
            day: self.vehicle.td_day,
            year: self.vehicle.td_year,
            local_minute_offset: self.vehicle.td_local_minute_offset,
            local_hour_offset: self.vehicle.td_local_hour_offset,
        };

        if let Ok((can_id, data)) = td.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // OEL - Operator External Light Controls
        let oel = OEL {
            device_id,
            work_light_switch: self.vehicle.oel_work_light,
            main_light_switch: self.vehicle.oel_main_light,
            turn_signal_switch: self.vehicle.oel_turn_signal,
            hazard_light_switch: self.vehicle.oel_hazard_light,
            high_low_beam_switch: self.vehicle.oel_high_low_beam,
            operators_desired_back_light: 0.0,
            oprtrs_dsrd_dld_lmp_off_tm: 0,
            exterior_lamp_check_switch: 0,
            headlamp_emergency_flash_switch: 0,
            auxiliary_lamp_group_switch: 0,
            auto_high_low_beam_enable_switch: 0,
        };

        if let Ok((can_id, data)) = oel.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // SHUTDN - Shutdown
        let shutdn = SHUTDN {
            device_id,
            engn_idl_shtdwn_hs_shtdwn_engn: self.vehicle.shutdn_idle_shutdown,
            engn_idl_shtdwn_drvr_alrt_md: 0,
            engine_idle_shutdown_timer_override: 0,
            engine_idle_shutdown_timer_state: 0,
            engine_idle_shutdown_timer_function: 0,
            ac_high_pressure_fan_switch: 0,
            refrigerant_low_pressure_switch: 0,
            refrigerant_high_pressure_switch: 0,
            engine_wait_to_start_lamp: self.vehicle.shutdn_wait_to_start,
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

        if let Ok((can_id, data)) = shutdn.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // BSA - Brake Stroke Alert
        let bsa = BSA {
            device_id,
            tractor_brake_stroke_axle_1_left: self.vehicle.bsa_axle1_left,
            tractor_brake_stroke_axle_1_right: self.vehicle.bsa_axle1_right,
            tractor_brake_stroke_axle_2_left: self.vehicle.bsa_axle2_left,
            tractor_brake_stroke_axle_2_right: self.vehicle.bsa_axle2_right,
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

        if let Ok((can_id, data)) = bsa.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // GFI1 - Gaseous Fuel Information 1
        let gfi1 = GFI1 {
            device_id,
            ttl_engn_pt_gvrnr_fl_usd_gss: self.vehicle.gfi1_total_fuel_used,
            trip_average_fuel_rate_gaseous: self.vehicle.gfi1_trip_average_fuel_rate,
            engine_fuel_specific_gravity: self.vehicle.gfi1_fuel_specific_gravity,
        };

        if let Ok((can_id, data)) = gfi1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
