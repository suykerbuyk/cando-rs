use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle CCVS1 - Cruise Control / Vehicle Speed 1
    pub(crate) fn handle_ccvs1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match CCVS1::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.ccvs1_vehicle_speed = msg.wheel_based_vehicle_speed;
                self.vehicle.ccvs1_cruise_control_active = msg.cruise_control_active;
                self.vehicle.ccvs1_parking_brake = msg.parking_brake_switch;
                self.vehicle.ccvs1_brake_switch = msg.brake_switch;
                self.vehicle.ccvs1_clutch_switch = msg.clutch_switch;
                self.vehicle.ccvs1_cruise_control_set_speed = msg.cruise_control_set_speed;
                self.vehicle.ccvs1_cruise_control_states = msg.cruise_control_states;
                self.vehicle.ccvs1_cruise_control_enable = msg.cruise_control_enable_switch;
                self.vehicle.ccvs1_pto_governor_state = msg.pto_governor_state;
                println!(
                    "🚗 Received CCVS1: Speed = {:.1} km/h, Parking = {}, Brake = {}",
                    self.vehicle.ccvs1_vehicle_speed,
                    self.vehicle.ccvs1_parking_brake,
                    self.vehicle.ccvs1_brake_switch
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode CCVS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle CCVS2 - Cruise Control / Vehicle Speed 2
    pub(crate) fn handle_ccvs2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match CCVS2::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.ccvs2_cruise_disable_command = msg.cruise_control_disable_command;
                self.vehicle.ccvs2_idle_speed_request = msg.idle_speed_request;
                println!(
                    "🚗 Received CCVS2: Cruise disable = {}, Idle request = {:.1} rpm",
                    self.vehicle.ccvs2_cruise_disable_command,
                    self.vehicle.ccvs2_idle_speed_request
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode CCVS2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle CCVS3 - Cruise Control / Vehicle Speed 3
    pub(crate) fn handle_ccvs3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match CCVS3::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.ccvs3_adaptive_cc_readiness = msg.adptv_crs_cntrl_rdnss_stts;
                self.vehicle.ccvs3_cc_system_command_state =
                    msg.cruise_control_system_command_state;
                self.vehicle.ccvs3_cruise_control_speed = msg.cruise_control_speed;
                println!(
                    "🚗 Received CCVS3: CC speed = {:.1} km/h",
                    self.vehicle.ccvs3_cruise_control_speed
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode CCVS3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle CCVS4 - Cruise Control / Vehicle Speed 4
    pub(crate) fn handle_ccvs4(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match CCVS4::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.ccvs4_applied_speed_limit = msg.appld_vhl_spd_lmt_hgh_rsltn;
                println!(
                    "🚗 Received CCVS4: Speed limit = {:.1} km/h",
                    self.vehicle.ccvs4_applied_speed_limit
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode CCVS4: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle CCVS5 - Cruise Control / Vehicle Speed 5
    pub(crate) fn handle_ccvs5(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match CCVS5::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.ccvs5_directional_speed = msg.directional_vehicle_speed;
                println!(
                    "🚗 Received CCVS5: Directional speed = {:.1} km/h",
                    self.vehicle.ccvs5_directional_speed
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode CCVS5: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle CCVS6 - Cruise Control / Vehicle Speed 6
    pub(crate) fn handle_ccvs6(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match CCVS6::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.ccvs6_roadway_speed_limit_mode = msg.crrnt_rdw_vhl_spd_lmt_md;
                self.vehicle.ccvs6_roadway_speed_limit =
                    msg.current_roadway_vehicle_speed_limit;
                println!(
                    "🚗 Received CCVS6: Roadway limit = {:.1} km/h, Mode = {}",
                    self.vehicle.ccvs6_roadway_speed_limit,
                    self.vehicle.ccvs6_roadway_speed_limit_mode
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode CCVS6: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle VD - Vehicle Distance
    pub(crate) fn handle_vd(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match VD::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.vd_trip_distance = msg.trip_distance;
                self.vehicle.vd_total_distance = msg.total_vehicle_distance;
                println!(
                    "🚗 Received VD: Trip = {:.1} km, Total = {:.1} km",
                    self.vehicle.vd_trip_distance, self.vehicle.vd_total_distance
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode VD: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle VDS - Vehicle Direction/Speed
    pub(crate) fn handle_vds(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match VDS::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.vds_compass_bearing = msg.compass_bearing;
                self.vehicle.vds_nav_speed = msg.navigation_based_vehicle_speed;
                self.vehicle.vds_pitch = msg.pitch;
                self.vehicle.vds_altitude = msg.altitude;
                println!(
                    "🚗 Received VDS: Bearing = {:.1}, Nav speed = {:.1} km/h",
                    self.vehicle.vds_compass_bearing, self.vehicle.vds_nav_speed
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode VDS: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle VDS2 - Vehicle Direction/Speed 2
    pub(crate) fn handle_vds2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match VDS2::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.vds2_vehicle_roll = msg.vehicle_roll;
                println!(
                    "🚗 Received VDS2: Roll = {:.1} deg",
                    self.vehicle.vds2_vehicle_roll
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode VDS2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle HRW - High Resolution Wheel Speed
    pub(crate) fn handle_hrw(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HRW::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.hrw_front_left_speed = msg.front_axle_left_wheel_speed;
                self.vehicle.hrw_front_right_speed = msg.front_axle_right_wheel_speed;
                self.vehicle.hrw_rear_left_speed = msg.rear_axle_left_wheel_speed;
                self.vehicle.hrw_rear_right_speed = msg.rear_axle_right_wheel_speed;
                println!(
                    "🚗 Received HRW: FL={:.1}, FR={:.1}, RL={:.1}, RR={:.1} km/h",
                    self.vehicle.hrw_front_left_speed,
                    self.vehicle.hrw_front_right_speed,
                    self.vehicle.hrw_rear_left_speed,
                    self.vehicle.hrw_rear_right_speed
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HRW: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle VW - Vehicle Weight
    pub(crate) fn handle_vw(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match VW::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.vw_axle_location = msg.axle_location;
                self.vehicle.vw_axle_weight = msg.axle_weight;
                self.vehicle.vw_trailer_weight = msg.trailer_weight;
                self.vehicle.vw_cargo_weight = msg.cargo_weight;
                println!(
                    "🚗 Received VW: Axle={:.0} kg, Cargo={:.0} kg",
                    self.vehicle.vw_axle_weight, self.vehicle.vw_cargo_weight
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode VW: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle TIRE1 - Tire Condition 1
    pub(crate) fn handle_tire1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match TIRE1::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.tire1_location = msg.tire_location;
                self.vehicle.tire1_pressure = msg.tire_pressure;
                self.vehicle.tire1_temperature = msg.tire_temperature;
                self.vehicle.tire1_status = msg.tire_status;
                self.vehicle.tire1_leakage_rate = msg.tire_air_leakage_rate;
                println!(
                    "🚗 Received TIRE1: Pressure = {:.0} kPa, Temp = {:.1}°C",
                    self.vehicle.tire1_pressure, self.vehicle.tire1_temperature
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode TIRE1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle TIRE2 - Tire Condition 2
    pub(crate) fn handle_tire2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match TIRE2::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.tire2_location = msg.tire_location;
                self.vehicle.tire2_pressure_extended = msg.tire_pressure_extended_range;
                self.vehicle.tire2_required_pressure = msg.required_tire_pressure;
                println!(
                    "🚗 Received TIRE2: Extended pressure = {}, Required = {}",
                    self.vehicle.tire2_pressure_extended,
                    self.vehicle.tire2_required_pressure
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode TIRE2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle SSI - Slope Sensor Information
    pub(crate) fn handle_ssi(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match SSI::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.ssi_pitch_angle = msg.pitch_angle;
                self.vehicle.ssi_roll_angle = msg.roll_angle;
                self.vehicle.ssi_pitch_rate = msg.pitch_rate;
                println!(
                    "🚗 Received SSI: Pitch = {:.2}°, Roll = {:.2}°",
                    self.vehicle.ssi_pitch_angle, self.vehicle.ssi_roll_angle
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode SSI: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle VEP1 - Vehicle Electrical Power 1
    pub(crate) fn handle_vep1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match VEP1::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.vep1_battery_current = msg.sli_battery_1_net_current;
                self.vehicle.vep1_alternator_current = msg.alternator_current;
                self.vehicle.vep1_charging_voltage = msg.charging_system_potential_voltage;
                self.vehicle.vep1_battery_potential = msg.battery_potential_power_input_1;
                self.vehicle.vep1_key_switch_voltage = msg.key_switch_battery_potential;
                println!(
                    "🔋 Received VEP1: Battery = {:.1}V, Alternator = {}A",
                    self.vehicle.vep1_battery_potential,
                    self.vehicle.vep1_alternator_current
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode VEP1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle VEP2 - Vehicle Electrical Power 2
    pub(crate) fn handle_vep2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match VEP2::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.vep2_battery_potential_2 = msg.battery_potential_power_input_2;
                self.vehicle.vep2_ecu_supply_voltage_1 =
                    msg.ecu_power_output_supply_voltage_1;
                self.vehicle.vep2_ecu_supply_voltage_2 =
                    msg.ecu_power_output_supply_voltage_2;
                self.vehicle.vep2_ecu_supply_voltage_3 =
                    msg.ecu_power_output_supply_voltage_3;
                println!(
                    "🔋 Received VEP2: Battery2 = {:.1}V",
                    self.vehicle.vep2_battery_potential_2
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode VEP2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle VEP3 - Vehicle Electrical Power 3
    pub(crate) fn handle_vep3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match VEP3::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.vep3_alternator_current_hr = msg.altrntr_crrnt_hgh_rng_rsltn;
                self.vehicle.vep3_battery_current_hr =
                    msg.sl_bttr_1_nt_crrnt_hgh_rng_rsltn;
                self.vehicle.vep3_battery_2_current = msg.sli_battery_2_net_current;
                self.vehicle.vep3_key_switch_state = msg.ecu_key_switch_state;
                println!(
                    "🔋 Received VEP3: Alt current = {:.1}A, Key state = {}",
                    self.vehicle.vep3_alternator_current_hr,
                    self.vehicle.vep3_key_switch_state
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode VEP3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle AS1 - Alternator Speed 1
    pub(crate) fn handle_as1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AS1::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.as1_alternator_speed = msg.alternator_speed;
                self.vehicle.as1_alternator_1_status = msg.alternator_1_status;
                println!(
                    "⚡ Received AS1: Speed = {:.0} rpm, Status = {}",
                    self.vehicle.as1_alternator_speed,
                    self.vehicle.as1_alternator_1_status
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode AS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle AS2 - Alternator Speed 2
    pub(crate) fn handle_as2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AS2::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.as2_setpoint_voltage_feedback = msg.altrntr_stpnt_vltg_fdk;
                self.vehicle.as2_output_voltage = msg.alternator_output_voltage;
                self.vehicle.as2_regulator_temperature = msg.altrntr_vltg_rgltr_tmprtr;
                self.vehicle.as2_excitation_current = msg.alternator_excitation_current;
                self.vehicle.as2_excitation_duty_cycle = msg.alternator_excitation_duty_cycle;
                println!(
                    "⚡ Received AS2: Output = {:.1}V, Excitation = {:.1}A",
                    self.vehicle.as2_output_voltage,
                    self.vehicle.as2_excitation_current
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode AS2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EP - Electronic Process
    pub(crate) fn handle_ep(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EP::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.ep_keep_alive_consumption = msg.keep_alive_battery_consumption;
                self.vehicle.ep_data_memory_usage = msg.data_memory_usage;
                println!(
                    "⚡ Received EP: Keep-alive = {}W, Memory = {:.1}%",
                    self.vehicle.ep_keep_alive_consumption,
                    self.vehicle.ep_data_memory_usage
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EP: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle TD - Time/Date
    pub(crate) fn handle_td(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match TD::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.td_seconds = msg.seconds;
                self.vehicle.td_minutes = msg.minutes;
                self.vehicle.td_hours = msg.hours;
                self.vehicle.td_day = msg.day;
                self.vehicle.td_month = msg.month;
                self.vehicle.td_year = msg.year;
                self.vehicle.td_local_minute_offset = msg.local_minute_offset;
                self.vehicle.td_local_hour_offset = msg.local_hour_offset;
                println!(
                    "⏰ Received TD: {:02}:{:02}:{:.0} {:.0}/{}/{:.0}",
                    self.vehicle.td_hours,
                    self.vehicle.td_minutes,
                    self.vehicle.td_seconds,
                    self.vehicle.td_day,
                    self.vehicle.td_month,
                    self.vehicle.td_year
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode TD: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle OEL - Operator External Light Controls
    pub(crate) fn handle_oel(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match OEL::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.oel_work_light = msg.work_light_switch;
                self.vehicle.oel_main_light = msg.main_light_switch;
                self.vehicle.oel_turn_signal = msg.turn_signal_switch;
                self.vehicle.oel_hazard_light = msg.hazard_light_switch;
                self.vehicle.oel_high_low_beam = msg.high_low_beam_switch;
                println!(
                    "💡 Received OEL: Main={}, Turn={}, Hazard={}",
                    self.vehicle.oel_main_light,
                    self.vehicle.oel_turn_signal,
                    self.vehicle.oel_hazard_light
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode OEL: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle SHUTDN - Shutdown
    pub(crate) fn handle_shutdn(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match SHUTDN::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.shutdn_idle_shutdown = msg.engn_idl_shtdwn_hs_shtdwn_engn;
                self.vehicle.shutdn_wait_to_start = msg.engine_wait_to_start_lamp;
                println!(
                    "⛔ Received SHUTDN: Idle shutdown = {}, Wait to start = {}",
                    self.vehicle.shutdn_idle_shutdown, self.vehicle.shutdn_wait_to_start
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode SHUTDN: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle BSA - Brake Stroke Alert
    pub(crate) fn handle_bsa(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match BSA::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.bsa_axle1_left = msg.tractor_brake_stroke_axle_1_left;
                self.vehicle.bsa_axle1_right = msg.tractor_brake_stroke_axle_1_right;
                self.vehicle.bsa_axle2_left = msg.tractor_brake_stroke_axle_2_left;
                self.vehicle.bsa_axle2_right = msg.tractor_brake_stroke_axle_2_right;
                println!(
                    "🛑 Received BSA: A1L={}, A1R={}, A2L={}, A2R={}",
                    self.vehicle.bsa_axle1_left,
                    self.vehicle.bsa_axle1_right,
                    self.vehicle.bsa_axle2_left,
                    self.vehicle.bsa_axle2_right
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode BSA: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle GFI1 - Gaseous Fuel Information 1
    pub(crate) fn handle_gfi1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match GFI1::decode(can_id, data) {
            Ok(msg) => {
                self.vehicle.gfi1_total_fuel_used = msg.ttl_engn_pt_gvrnr_fl_usd_gss;
                self.vehicle.gfi1_trip_average_fuel_rate = msg.trip_average_fuel_rate_gaseous;
                self.vehicle.gfi1_fuel_specific_gravity = msg.engine_fuel_specific_gravity;
                println!(
                    "⛽ Received GFI1: Total fuel = {:.1} kg, Rate = {:.1} kg/h",
                    self.vehicle.gfi1_total_fuel_used,
                    self.vehicle.gfi1_trip_average_fuel_rate
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode GFI1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }
}
