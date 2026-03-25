use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    // ============================================================================
    // Batch 3: Engine Temps, Fluids & Sensors Handlers
    // ============================================================================

    /// Handle ET1 - Engine Temperature 1
    pub(crate) fn handle_et1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ET1::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.et1_coolant_temp = msg.engine_coolant_temperature;
                self.sensors.et1_fuel_temp = msg.engine_fuel_1_temperature_1;
                self.sensors.et1_oil_temp = msg.engine_oil_temperature_1;
                self.sensors.et1_turbo_oil_temp = msg.engn_trhrgr_1_ol_tmprtr;
                self.sensors.et1_intercooler_temp = msg.engine_intercooler_temperature;
                self.sensors.et1_charge_air_cooler_thermostat = msg.engn_chrg_ar_clr_thrmstt_opnng;
                println!("🌡️ Received ET1: Coolant={:.1}°C, Oil={:.1}°C", self.sensors.et1_coolant_temp, self.sensors.et1_oil_temp);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode ET1: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle ET2 - Engine Temperature 2
    pub(crate) fn handle_et2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ET2::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.et2_oil_temp_2 = msg.engine_oil_temperature_2;
                self.sensors.et2_ecu_temp = msg.engine_ecu_temperature;
                self.sensors.et2_egr_diff_pressure = msg.engn_exhst_gs_rrltn_1_dffrntl_prssr;
                self.sensors.et2_egr_temp = msg.engn_exhst_gs_rrltn_1_tmprtr;
                println!("🌡️ Received ET2: Oil2={:.1}°C, ECU={:.1}°C", self.sensors.et2_oil_temp_2, self.sensors.et2_ecu_temp);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode ET2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle ET3 - Engine Temperature 3
    pub(crate) fn handle_et3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ET3::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.et3_intake_manifold_temp_hr = msg.engn_intk_mnfld_1_tmprtr_hgh_rsltn;
                self.sensors.et3_coolant_temp_hr = msg.engn_clnt_tmprtr_hgh_rsltn;
                self.sensors.et3_intake_valve_oil_temp = msg.engn_intk_vlv_attn_sstm_ol_tmprtr;
                self.sensors.et3_charge_air_cooler_outlet_temp = msg.engn_chrg_ar_clr_1_otlt_tmprtr;
                println!("🌡️ Received ET3: Coolant HR={:.1}°C", self.sensors.et3_coolant_temp_hr);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode ET3: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle ET4 - Engine Temperature 4
    pub(crate) fn handle_et4(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ET4::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.et4_coolant_temp_2 = msg.engine_coolant_temperature_2;
                self.sensors.et4_coolant_pump_outlet_temp = msg.engn_clnt_pmp_otlt_tmprtr;
                self.sensors.et4_coolant_thermostat_opening = msg.engine_coolant_thermostat_opening;
                self.sensors.et4_exhaust_valve_oil_temp = msg.engn_exhst_vlv_attn_sstm_ol_tmprtr;
                self.sensors.et4_egr_mixer_intake_temp = msg.engn_exhst_gs_rrltn_1_mxr_intk_tmprtr;
                self.sensors.et4_coolant_temp_3 = msg.engine_coolant_temperature_3;
                println!("🌡️ Received ET4: Coolant2={:.1}°C", self.sensors.et4_coolant_temp_2);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode ET4: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle ET5 - Engine Temperature 5
    pub(crate) fn handle_et5(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ET5::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.et5_egr2_temp = msg.engn_exhst_gs_rrltn_2_tmprtr;
                self.sensors.et5_egr2_mixer_intake_temp = msg.engn_exhst_gs_rrltn_2_mxr_intk_tmprtr;
                self.sensors.et5_coolant_temp_2_hr = msg.e_ct_tpt_2_h_rst_extdd_r;
                println!("🌡️ Received ET5: EGR2 Temp={:.1}°C", self.sensors.et5_egr2_temp);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode ET5: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle ET6 - Engine Temperature 6
    pub(crate) fn handle_et6(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ET6::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.et6_charge_air_cooler_intake_coolant_temp = msg.engn_chrg_ar_clr_intk_clnt_tmprtr;
                self.sensors.et6_charge_air_cooler_outlet_coolant_temp = msg.engn_chrg_ar_clr_otlt_clnt_tmprtr;
                self.sensors.et6_intake_coolant_temp = msg.engine_intake_coolant_temperature;
                self.sensors.et6_intake_mixed_air_side_coolant_temp = msg.e_it_md_at_sd_ct_ct_ct_dt_tpt;
                println!("🌡️ Received ET6: Intake Coolant={:.1}°C", self.sensors.et6_intake_coolant_temp);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode ET6: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle LFE1 - Liquid Fuel Economy 1
    pub(crate) fn handle_lfe1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match LFE1::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.lfe1_fuel_rate = msg.engine_fuel_rate;
                self.sensors.lfe1_instant_fuel_economy = msg.engine_instantaneous_fuel_economy;
                self.sensors.lfe1_average_fuel_economy = msg.engine_average_fuel_economy;
                self.sensors.lfe1_throttle_valve_1_pos = msg.engine_throttle_valve_1_position_1;
                self.sensors.lfe1_throttle_valve_2_pos = msg.engine_throttle_valve_2_position;
                println!("⛽ Received LFE1: Fuel Rate={:.1} L/h", self.sensors.lfe1_fuel_rate);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode LFE1: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle LFE2 - Liquid Fuel Economy 2
    pub(crate) fn handle_lfe2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match LFE2::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.lfe2_fuel_rate_hr = msg.engine_fuel_rate_high_resolution;
                self.sensors.lfe2_diesel_fuel_demand_rate = msg.engine_diesel_fuel_demand_rate;
                println!("⛽ Received LFE2: HR Fuel Rate={:.1} L/h", self.sensors.lfe2_fuel_rate_hr);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode LFE2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle IC1 - Intake/Exhaust Conditions 1
    pub(crate) fn handle_ic1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match IC1::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.ic1_aftertreatment_intake_pressure = msg.atttt_1_ds_ptt_ft_it_pss_us_sp_3609;
                self.sensors.ic1_intake_manifold_pressure = msg.engine_intake_manifold_1_pressure;
                self.sensors.ic1_intake_manifold_temp = msg.engn_intk_mnfld_1_tmprtr;
                self.sensors.ic1_intake_air_pressure = msg.engine_intake_air_pressure;
                self.sensors.ic1_air_filter_diff_pressure = msg.engn_ar_fltr_1_dffrntl_prssr;
                self.sensors.ic1_exhaust_temp = msg.engine_exhaust_temperature;
                self.sensors.ic1_coolant_filter_diff_pressure = msg.engn_clnt_fltr_dffrntl_prssr;
                println!("🌬️ Received IC1: Manifold P={:.1} kPa", self.sensors.ic1_intake_manifold_pressure);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode IC1: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle IC2 - Intake/Exhaust Conditions 2
    pub(crate) fn handle_ic2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match IC2::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.ic2_air_filter_2_diff_pressure = msg.engn_ar_fltr_2_dffrntl_prssr;
                self.sensors.ic2_air_filter_3_diff_pressure = msg.engn_ar_fltr_3_dffrntl_prssr;
                self.sensors.ic2_air_filter_4_diff_pressure = msg.engn_ar_fltr_4_dffrntl_prssr;
                self.sensors.ic2_intake_manifold_2_pressure = msg.engine_intake_manifold_2_pressure;
                self.sensors.ic2_intake_manifold_1_abs_pressure = msg.engn_intk_mnfld_1_aslt_prssr;
                self.sensors.ic2_intake_manifold_1_abs_pressure_hr = msg.engn_intk_mnfld_1_aslt_prssr_hgh_rsltn;
                self.sensors.ic2_intake_manifold_2_abs_pressure = msg.engn_intk_mnfld_2_aslt_prssr;
                println!("🌬️ Received IC2: Manifold2 P={:.1} kPa", self.sensors.ic2_intake_manifold_2_pressure);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode IC2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle IC3 - Intake/Exhaust Conditions 3
    pub(crate) fn handle_ic3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match IC3::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.ic3_mixer_1_intake_pressure = msg.engine_mixer_1_intake_pressure;
                self.sensors.ic3_mixer_2_intake_pressure = msg.engine_mixer_2_intake_pressure;
                self.sensors.ic3_intake_manifold_2_abs_pressure_hr = msg.engn_intk_mnfld_2_aslt_prssr_hgh_rsltn;
                self.sensors.ic3_desired_intake_manifold_pressure_high_limit = msg.dsrd_engn_intk_mnfld_prssr_hgh_lmt;
                println!("🌬️ Received IC3: Mixer1 P={:.1} kPa", self.sensors.ic3_mixer_1_intake_pressure);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode IC3: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle AMB - Ambient Conditions
    pub(crate) fn handle_amb(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AMB::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.amb_barometric_pressure = msg.barometric_pressure;
                self.sensors.amb_cab_interior_temp = msg.cab_interior_temperature;
                self.sensors.amb_ambient_temp = msg.ambient_air_temperature;
                self.sensors.amb_intake_air_temp = msg.engine_intake_1_air_temperature;
                self.sensors.amb_road_surface_temp = msg.road_surface_temperature;
                println!("🌡️ Received AMB: Ambient={:.1}°C, Baro={:.1} kPa", self.sensors.amb_ambient_temp, self.sensors.amb_barometric_pressure);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode AMB: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle AMB2 - Ambient Conditions 2
    pub(crate) fn handle_amb2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AMB2::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.amb2_solar_intensity = msg.solar_intensity_percent;
                self.sensors.amb2_solar_sensor_max = msg.solar_sensor_maximum;
                self.sensors.amb2_specific_humidity = msg.specific_humidity;
                self.sensors.amb2_calculated_ambient_temp = msg.calculated_ambient_air_temperature;
                self.sensors.amb2_barometric_abs_pressure_hr = msg.brmtr_aslt_prssr_hgh_rsltn;
                println!("🌡️ Received AMB2: Solar={:.1}%", self.sensors.amb2_solar_intensity);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode AMB2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle AMB3 - Ambient Conditions 3
    pub(crate) fn handle_amb3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AMB3::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.amb3_barometric_abs_pressure_2 = msg.barometric_absolute_pressure_2;
                self.sensors.amb3_intake_2_air_temp = msg.engine_intake_2_air_temperature;
                self.sensors.amb3_power_derate_humidity_diff = msg.engn_pwr_drt_rltv_hmdt_dffrn;
                println!("🌡️ Received AMB3: Baro2={:.1} kPa", self.sensors.amb3_barometric_abs_pressure_2);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode AMB3: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle AMB4 - Ambient Conditions 4
    pub(crate) fn handle_amb4(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AMB4::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.amb4_fuel_specific_humidity = msg.fuel_specific_humidity;
                self.sensors.amb4_charge_air_specific_humidity = msg.engine_charge_air_specific_humidity;
                self.sensors.amb4_fuel_relative_humidity = msg.fuel_relative_humidity;
                self.sensors.amb4_charge_air_relative_humidity = msg.engine_charge_air_relative_humidity;
                println!("🌡️ Received AMB4: Fuel Humidity={:.1}%", self.sensors.amb4_fuel_relative_humidity);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode AMB4: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle FD2 - Fan Drive 2
    pub(crate) fn handle_fd2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match FD2::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.fd2_estimated_fan_2_speed_pct = msg.estimated_percent_fan_2_speed;
                self.sensors.fd2_fan_2_drive_state = msg.fan_2_drive_state;
                self.sensors.fd2_fan_2_speed = msg.fan_2_speed;
                self.sensors.fd2_hydraulic_fan_2_pressure = msg.hydraulic_fan_2_motor_pressure;
                self.sensors.fd2_fan_2_bypass_command_status = msg.fan_2_drive_bypass_command_status;
                println!("🌀 Received FD2: Fan Speed={:.0} rpm", self.sensors.fd2_fan_2_speed);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode FD2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DD2 - Dash Display 2
    pub(crate) fn handle_dd2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DD2::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.dd2_oil_filter_diff_pressure_ext = msg.engn_ol_fltr_dffrntl_prssr_extndd_rng;
                self.sensors.dd2_fuel_2_tank_1_level = msg.engine_fuel_2_tank_1_level;
                self.sensors.dd2_fuel_2_tank_2_level = msg.engine_fuel_2_tank_2_level;
                self.sensors.dd2_fuel_2_tank_3_level = msg.engine_fuel_2_tank_3_level;
                self.sensors.dd2_fuel_2_tank_4_level = msg.engine_fuel_2_tank_4_level;
                self.sensors.dd2_display_remain_powered = msg.display_remain_powered;
                self.sensors.dd2_oil_level_high_low = msg.engine_oil_level_high_low;
                println!("📊 Received DD2: Fuel Level={:.1}%", self.sensors.dd2_fuel_2_tank_1_level);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DD2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DD3 - Dash Display 3
    pub(crate) fn handle_dd3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DD3::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.dd3_predictive_speed_adj_indicator_state = msg.prdtv_vhl_spd_adjstmnt_indtr_stt;
                self.sensors.dd3_predictive_speed_adj_speed = msg.prdtv_vhl_spd_adjstmnt_spd;
                println!("📊 Received DD3: Speed Adj={:.1}", self.sensors.dd3_predictive_speed_adj_speed);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DD3: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle HOURS - Engine Hours
    pub(crate) fn handle_hours(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HOURS::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.hours_engine_total_hours = msg.engine_total_hours_of_operation;
                self.sensors.hours_total_revolutions = msg.engine_total_revolutions;
                println!("⏱️ Received HOURS: Total={:.1} hr", self.sensors.hours_engine_total_hours);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode HOURS: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle HOURS2 - Engine Hours 2
    pub(crate) fn handle_hours2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HOURS2::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.hours2_idle_management_active_total_time = msg.engn_idl_mngmnt_atv_ttl_tm;
                println!("⏱️ Received HOURS2: Idle Mgmt={:.1} hr", self.sensors.hours2_idle_management_active_total_time);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode HOURS2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle IO - Idle Operation
    pub(crate) fn handle_io(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match IO::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.io_total_idle_fuel_used = msg.engine_total_idle_fuel_used;
                self.sensors.io_total_idle_hours = msg.engine_total_idle_hours;
                println!("⏱️ Received IO: Idle Fuel={:.1} L", self.sensors.io_total_idle_fuel_used);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode IO: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle FL - Fuel Leakage
    pub(crate) fn handle_fl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match FL::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.fl_fuel_leakage_1 = msg.engine_fuel_leakage_1;
                self.sensors.fl_fuel_leakage_2 = msg.engine_fuel_leakage_2;
                self.sensors.fl_fluid_bund_level = msg.engine_fluid_bund_level;
                println!("🔍 Received FL: Leak1={}, Leak2={}", self.sensors.fl_fuel_leakage_1, self.sensors.fl_fuel_leakage_2);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode FL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle LFC1 - Lifetime Fuel Consumption 1
    pub(crate) fn handle_lfc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match LFC1::decode(can_id, data) {
            Ok(msg) => {
                self.sensors.lfc1_trip_fuel = msg.engine_trip_fuel;
                self.sensors.lfc1_total_fuel_used = msg.engine_total_fuel_used;
                println!("⛽ Received LFC1: Trip={:.1} L, Total={:.1} L", self.sensors.lfc1_trip_fuel, self.sensors.lfc1_total_fuel_used);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode LFC1: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }
}
