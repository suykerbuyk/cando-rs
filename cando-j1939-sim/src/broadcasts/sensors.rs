use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_sensor_frames(&self, frames: &mut Vec<CanFrame>, device_id: DeviceId) {
        // WAND message (always send)
        let wand = WAND {
            device_id,
            wand_angle: self.sensors.wand_angle,
            wand_sensor_figure_of_merit: self.sensors.wand_quality,
        };

        if let Ok((can_id, data)) = wand.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // LDISP message (always send)
        let ldisp = LDISP {
            device_id,
            measured_linear_displacement: self.sensors.linear_displacement,
            lnr_dsplmnt_snsr_snsr_fgr_of_mrt: self.sensors.displacement_quality,
        };

        if let Ok((can_id, data)) = ldisp.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ====================================================================
        // Batch 3: Engine Temps, Fluids & Sensors Broadcasts
        // ====================================================================

        // ET1 - Engine Temperature 1
        let et1 = ET1 {
            device_id,
            engine_coolant_temperature: self.sensors.et1_coolant_temp,
            engine_fuel_1_temperature_1: self.sensors.et1_fuel_temp,
            engine_oil_temperature_1: self.sensors.et1_oil_temp,
            engn_trhrgr_1_ol_tmprtr: self.sensors.et1_turbo_oil_temp,
            engine_intercooler_temperature: self.sensors.et1_intercooler_temp,
            engn_chrg_ar_clr_thrmstt_opnng: self.sensors.et1_charge_air_cooler_thermostat,
        };
        if let Ok((can_id, data)) = et1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ET2 - Engine Temperature 2
        let et2 = ET2 {
            device_id,
            engine_oil_temperature_2: self.sensors.et2_oil_temp_2,
            engine_ecu_temperature: self.sensors.et2_ecu_temp,
            engn_exhst_gs_rrltn_1_dffrntl_prssr: self.sensors.et2_egr_diff_pressure,
            engn_exhst_gs_rrltn_1_tmprtr: self.sensors.et2_egr_temp,
        };
        if let Ok((can_id, data)) = et2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ET3 - Engine Temperature 3
        let et3 = ET3 {
            device_id,
            engn_intk_mnfld_1_tmprtr_hgh_rsltn: self.sensors.et3_intake_manifold_temp_hr,
            engn_clnt_tmprtr_hgh_rsltn: self.sensors.et3_coolant_temp_hr,
            engn_intk_vlv_attn_sstm_ol_tmprtr: self.sensors.et3_intake_valve_oil_temp,
            engn_chrg_ar_clr_1_otlt_tmprtr: self.sensors.et3_charge_air_cooler_outlet_temp,
        };
        if let Ok((can_id, data)) = et3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ET4 - Engine Temperature 4
        let et4 = ET4 {
            device_id,
            engine_coolant_temperature_2: self.sensors.et4_coolant_temp_2,
            engn_clnt_pmp_otlt_tmprtr: self.sensors.et4_coolant_pump_outlet_temp,
            engine_coolant_thermostat_opening: self.sensors.et4_coolant_thermostat_opening,
            engn_exhst_vlv_attn_sstm_ol_tmprtr: self.sensors.et4_exhaust_valve_oil_temp,
            engn_exhst_gs_rrltn_1_mxr_intk_tmprtr: self.sensors.et4_egr_mixer_intake_temp,
            engine_coolant_temperature_3: self.sensors.et4_coolant_temp_3,
        };
        if let Ok((can_id, data)) = et4.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ET5 - Engine Temperature 5
        let et5 = ET5 {
            device_id,
            engn_exhst_gs_rrltn_2_tmprtr: self.sensors.et5_egr2_temp,
            engn_exhst_gs_rrltn_2_mxr_intk_tmprtr: self.sensors.et5_egr2_mixer_intake_temp,
            e_ct_tpt_2_h_rst_extdd_r: self.sensors.et5_coolant_temp_2_hr,
        };
        if let Ok((can_id, data)) = et5.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ET6 - Engine Temperature 6
        let et6 = ET6 {
            device_id,
            engn_chrg_ar_clr_intk_clnt_tmprtr: self.sensors.et6_charge_air_cooler_intake_coolant_temp,
            engn_chrg_ar_clr_otlt_clnt_tmprtr: self.sensors.et6_charge_air_cooler_outlet_coolant_temp,
            engine_intake_coolant_temperature: self.sensors.et6_intake_coolant_temp,
            e_it_md_at_sd_ct_ct_ct_dt_tpt: self.sensors.et6_intake_mixed_air_side_coolant_temp,
        };
        if let Ok((can_id, data)) = et6.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // LFE1 - Liquid Fuel Economy 1
        let lfe1 = LFE1 {
            device_id,
            engine_fuel_rate: self.sensors.lfe1_fuel_rate,
            engine_instantaneous_fuel_economy: self.sensors.lfe1_instant_fuel_economy,
            engine_average_fuel_economy: self.sensors.lfe1_average_fuel_economy,
            engine_throttle_valve_1_position_1: self.sensors.lfe1_throttle_valve_1_pos,
            engine_throttle_valve_2_position: self.sensors.lfe1_throttle_valve_2_pos,
        };
        if let Ok((can_id, data)) = lfe1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // LFE2 - Liquid Fuel Economy 2
        let lfe2 = LFE2 {
            device_id,
            engine_fuel_rate_high_resolution: self.sensors.lfe2_fuel_rate_hr,
            engine_diesel_fuel_demand_rate: self.sensors.lfe2_diesel_fuel_demand_rate,
        };
        if let Ok((can_id, data)) = lfe2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // IC1 - Intake/Exhaust Conditions 1
        let ic1 = IC1 {
            device_id,
            atttt_1_ds_ptt_ft_it_pss_us_sp_3609: self.sensors.ic1_aftertreatment_intake_pressure,
            engine_intake_manifold_1_pressure: self.sensors.ic1_intake_manifold_pressure,
            engn_intk_mnfld_1_tmprtr: self.sensors.ic1_intake_manifold_temp,
            engine_intake_air_pressure: self.sensors.ic1_intake_air_pressure,
            engn_ar_fltr_1_dffrntl_prssr: self.sensors.ic1_air_filter_diff_pressure,
            engine_exhaust_temperature: self.sensors.ic1_exhaust_temp,
            engn_clnt_fltr_dffrntl_prssr: self.sensors.ic1_coolant_filter_diff_pressure,
        };
        if let Ok((can_id, data)) = ic1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // IC2 - Intake/Exhaust Conditions 2
        let ic2 = IC2 {
            device_id,
            engn_ar_fltr_2_dffrntl_prssr: self.sensors.ic2_air_filter_2_diff_pressure,
            engn_ar_fltr_3_dffrntl_prssr: self.sensors.ic2_air_filter_3_diff_pressure,
            engn_ar_fltr_4_dffrntl_prssr: self.sensors.ic2_air_filter_4_diff_pressure,
            engine_intake_manifold_2_pressure: self.sensors.ic2_intake_manifold_2_pressure,
            engn_intk_mnfld_1_aslt_prssr: self.sensors.ic2_intake_manifold_1_abs_pressure,
            engn_intk_mnfld_1_aslt_prssr_hgh_rsltn: self.sensors.ic2_intake_manifold_1_abs_pressure_hr,
            engn_intk_mnfld_2_aslt_prssr: self.sensors.ic2_intake_manifold_2_abs_pressure,
        };
        if let Ok((can_id, data)) = ic2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // IC3 - Intake/Exhaust Conditions 3
        let ic3 = IC3 {
            device_id,
            engine_mixer_1_intake_pressure: self.sensors.ic3_mixer_1_intake_pressure,
            engine_mixer_2_intake_pressure: self.sensors.ic3_mixer_2_intake_pressure,
            engn_intk_mnfld_2_aslt_prssr_hgh_rsltn: self.sensors.ic3_intake_manifold_2_abs_pressure_hr,
            dsrd_engn_intk_mnfld_prssr_hgh_lmt: self.sensors.ic3_desired_intake_manifold_pressure_high_limit,
        };
        if let Ok((can_id, data)) = ic3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AMB - Ambient Conditions
        let amb = AMB {
            device_id,
            barometric_pressure: self.sensors.amb_barometric_pressure,
            cab_interior_temperature: self.sensors.amb_cab_interior_temp,
            ambient_air_temperature: self.sensors.amb_ambient_temp,
            engine_intake_1_air_temperature: self.sensors.amb_intake_air_temp,
            road_surface_temperature: self.sensors.amb_road_surface_temp,
        };
        if let Ok((can_id, data)) = amb.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AMB2 - Ambient Conditions 2
        let amb2 = AMB2 {
            device_id,
            solar_intensity_percent: self.sensors.amb2_solar_intensity,
            solar_sensor_maximum: self.sensors.amb2_solar_sensor_max,
            specific_humidity: self.sensors.amb2_specific_humidity,
            calculated_ambient_air_temperature: self.sensors.amb2_calculated_ambient_temp,
            brmtr_aslt_prssr_hgh_rsltn: self.sensors.amb2_barometric_abs_pressure_hr,
        };
        if let Ok((can_id, data)) = amb2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AMB3 - Ambient Conditions 3
        let amb3 = AMB3 {
            device_id,
            barometric_absolute_pressure_2: self.sensors.amb3_barometric_abs_pressure_2,
            engine_intake_2_air_temperature: self.sensors.amb3_intake_2_air_temp,
            engn_pwr_drt_rltv_hmdt_dffrn: self.sensors.amb3_power_derate_humidity_diff,
        };
        if let Ok((can_id, data)) = amb3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AMB4 - Ambient Conditions 4
        let amb4 = AMB4 {
            device_id,
            fuel_specific_humidity: self.sensors.amb4_fuel_specific_humidity,
            engine_charge_air_specific_humidity: self.sensors.amb4_charge_air_specific_humidity,
            fuel_relative_humidity: self.sensors.amb4_fuel_relative_humidity,
            engine_charge_air_relative_humidity: self.sensors.amb4_charge_air_relative_humidity,
        };
        if let Ok((can_id, data)) = amb4.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // FD2 - Fan Drive 2
        let fd2 = FD2 {
            device_id,
            estimated_percent_fan_2_speed: self.sensors.fd2_estimated_fan_2_speed_pct,
            fan_2_drive_state: self.sensors.fd2_fan_2_drive_state,
            fan_2_speed: self.sensors.fd2_fan_2_speed,
            hydraulic_fan_2_motor_pressure: self.sensors.fd2_hydraulic_fan_2_pressure,
            fan_2_drive_bypass_command_status: self.sensors.fd2_fan_2_bypass_command_status,
        };
        if let Ok((can_id, data)) = fd2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DD2 - Dash Display 2
        let dd2 = DD2 {
            device_id,
            engn_ol_fltr_dffrntl_prssr_extndd_rng: self.sensors.dd2_oil_filter_diff_pressure_ext,
            engine_fuel_2_tank_1_level: self.sensors.dd2_fuel_2_tank_1_level,
            engine_fuel_2_tank_2_level: self.sensors.dd2_fuel_2_tank_2_level,
            engine_fuel_2_tank_3_level: self.sensors.dd2_fuel_2_tank_3_level,
            engine_fuel_2_tank_4_level: self.sensors.dd2_fuel_2_tank_4_level,
            display_remain_powered: self.sensors.dd2_display_remain_powered,
            engine_oil_level_high_low: self.sensors.dd2_oil_level_high_low,
        };
        if let Ok((can_id, data)) = dd2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DD3 - Dash Display 3
        let dd3 = DD3 {
            device_id,
            prdtv_vhl_spd_adjstmnt_indtr_stt: self.sensors.dd3_predictive_speed_adj_indicator_state,
            prdtv_vhl_spd_adjstmnt_spd: self.sensors.dd3_predictive_speed_adj_speed,
        };
        if let Ok((can_id, data)) = dd3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HOURS - Engine Hours
        let hours = HOURS {
            device_id,
            engine_total_hours_of_operation: self.sensors.hours_engine_total_hours,
            engine_total_revolutions: self.sensors.hours_total_revolutions,
        };
        if let Ok((can_id, data)) = hours.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HOURS2 - Engine Hours 2
        let hours2 = HOURS2 {
            device_id,
            engn_idl_mngmnt_atv_ttl_tm: self.sensors.hours2_idle_management_active_total_time,
        };
        if let Ok((can_id, data)) = hours2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // IO - Idle Operation
        let io = IO {
            device_id,
            engine_total_idle_fuel_used: self.sensors.io_total_idle_fuel_used,
            engine_total_idle_hours: self.sensors.io_total_idle_hours,
        };
        if let Ok((can_id, data)) = io.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // FL - Fuel Leakage
        let fl = FL {
            device_id,
            engine_fuel_leakage_1: self.sensors.fl_fuel_leakage_1,
            engine_fuel_leakage_2: self.sensors.fl_fuel_leakage_2,
            engine_fluid_bund_level: self.sensors.fl_fluid_bund_level,
        };
        if let Ok((can_id, data)) = fl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // LFC1 - Lifetime Fuel Consumption 1
        let lfc1 = LFC1 {
            device_id,
            engine_trip_fuel: self.sensors.lfc1_trip_fuel,
            engine_total_fuel_used: self.sensors.lfc1_total_fuel_used,
        };
        if let Ok((can_id, data)) = lfc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
