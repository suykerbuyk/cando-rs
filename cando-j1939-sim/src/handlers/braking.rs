use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle EBC1 - Electronic Brake Controller 1
    pub(crate) fn handle_ebc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EBC1::decode(can_id, data) {
            Ok(msg) => {
                self.braking.ebc1_brake_pedal_position = msg.brake_pedal_position;
                self.braking.ebc1_ebs_brake_switch = msg.ebs_brake_switch;
                self.braking.ebc1_abs_active = msg.anti_lock_braking_abs_active;
                self.braking.ebc1_asr_engine_control_active = msg.asr_engine_control_active;
                self.braking.ebc1_asr_brake_control_active = msg.asr_brake_control_active;
                self.braking.ebc1_engine_retarder_selection = msg.engine_retarder_selection;
                self.braking.ebc1_abs_fully_operational = msg.abs_fully_operational;
                self.braking.ebc1_ebs_red_warning = msg.ebs_red_warning_signal;
                self.braking.ebc1_abs_ebs_amber_warning = msg.as_es_amr_wrnng_sgnl_pwrd_vhl;
                self.braking.ebc1_halt_brake_switch = msg.halt_brake_switch;
                println!(
                    "🛑 Received EBC1: Pedal = {:.1}%, ABS = {}, ASR engine = {}",
                    self.braking.ebc1_brake_pedal_position,
                    self.braking.ebc1_abs_active,
                    self.braking.ebc1_asr_engine_control_active
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EBC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EBC2 - Electronic Brake Controller 2 (Wheel Speeds)
    pub(crate) fn handle_ebc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EBC2::decode(can_id, data) {
            Ok(msg) => {
                self.braking.ebc2_front_axle_speed = msg.front_axle_speed;
                self.braking.ebc2_rel_speed_front_left = msg.relative_speed_front_axle_left_wheel;
                self.braking.ebc2_rel_speed_front_right = msg.rltv_spd_frnt_axl_rght_whl;
                self.braking.ebc2_rel_speed_rear1_left = msg.relative_speed_rear_axle_1_left_wheel;
                self.braking.ebc2_rel_speed_rear1_right = msg.rltv_spd_rr_axl_1_rght_whl;
                self.braking.ebc2_rel_speed_rear2_left = msg.relative_speed_rear_axle_2_left_wheel;
                self.braking.ebc2_rel_speed_rear2_right = msg.rltv_spd_rr_axl_2_rght_whl;
                println!(
                    "🛑 Received EBC2: Front axle = {:.1} km/h",
                    self.braking.ebc2_front_axle_speed
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EBC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EBC3 - Electronic Brake Controller 3 (Brake Pressures)
    pub(crate) fn handle_ebc3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EBC3::decode(can_id, data) {
            Ok(msg) => {
                self.braking.ebc3_pressure_front_left = msg.b_appt_pss_hr_ft_ax_lt_w;
                self.braking.ebc3_pressure_front_right = msg.b_appt_pss_hr_ft_ax_rt_w;
                self.braking.ebc3_pressure_rear1_left = msg.b_appt_pss_hrr_ax_1_lt_w;
                self.braking.ebc3_pressure_rear1_right = msg.b_appt_pss_hrr_ax_1_rt_w;
                self.braking.ebc3_pressure_rear2_left = msg.b_appt_pss_hrr_ax_2_lt_w;
                self.braking.ebc3_pressure_rear2_right = msg.b_appt_pss_hrr_ax_2_rt_w;
                self.braking.ebc3_pressure_rear3_left = msg.b_appt_pss_hrr_ax_3_lt_w;
                self.braking.ebc3_pressure_rear3_right = msg.b_appt_pss_hrr_ax_3_rt_w;
                println!(
                    "🛑 Received EBC3: Front L = {:.0} kPa, Front R = {:.0} kPa",
                    self.braking.ebc3_pressure_front_left, self.braking.ebc3_pressure_front_right
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EBC3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EBC4 - Electronic Brake Controller 4 (Brake Lining Axles 1-3)
    pub(crate) fn handle_ebc4(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EBC4::decode(can_id, data) {
            Ok(msg) => {
                self.braking.ebc4_lining_front_left = msg.brk_lnng_rmnng_frnt_axl_lft_whl;
                self.braking.ebc4_lining_front_right = msg.brk_lnng_rmnng_frnt_axl_rght_whl;
                self.braking.ebc4_lining_rear1_left = msg.brk_lnng_rmnng_rr_axl_1_lft_whl;
                self.braking.ebc4_lining_rear1_right = msg.brk_lnng_rmnng_rr_axl_1_rght_whl;
                self.braking.ebc4_lining_rear2_left = msg.brk_lnng_rmnng_rr_axl_2_lft_whl;
                self.braking.ebc4_lining_rear2_right = msg.brk_lnng_rmnng_rr_axl_2_rght_whl;
                self.braking.ebc4_lining_rear3_left = msg.brk_lnng_rmnng_rr_axl_3_lft_whl;
                self.braking.ebc4_lining_rear3_right = msg.brk_lnng_rmnng_rr_axl_3_rght_whl;
                println!(
                    "🛑 Received EBC4: Lining FL = {:.1}%, FR = {:.1}%",
                    self.braking.ebc4_lining_front_left, self.braking.ebc4_lining_front_right
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EBC4: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EBC5 - Electronic Brake Controller 5 (Brake Status)
    pub(crate) fn handle_ebc5(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EBC5::decode(can_id, data) {
            Ok(msg) => {
                self.braking.ebc5_brake_temp_warning = msg.brake_temperature_warning;
                self.braking.ebc5_halt_brake_mode = msg.halt_brake_mode;
                self.braking.ebc5_hill_holder_mode = msg.hill_holder_mode;
                self.braking.ebc5_foundation_brake_use = msg.foundation_brake_use;
                self.braking.ebc5_xbr_system_state = msg.xbr_system_state;
                self.braking.ebc5_xbr_active_control_mode = msg.xbr_active_control_mode;
                self.braking.ebc5_xbr_acceleration_limit = msg.xbr_acceleration_limit;
                self.braking.ebc5_parking_brake_actuator = msg.prkng_brk_attr_fll_atvtd;
                self.braking.ebc5_emergency_braking_active = msg.emergency_braking_active;
                self.braking.ebc5_driver_brake_demand = msg.driver_brake_demand;
                self.braking.ebc5_overall_brake_demand = msg.ovrll_intndd_brk_alrtn;
                println!(
                    "🛑 Received EBC5: Temp warn = {}, Emergency = {}",
                    self.braking.ebc5_brake_temp_warning, self.braking.ebc5_emergency_braking_active
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EBC5: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EBC6 - Electronic Brake Controller 6 (Brake Lining Axles 4-7)
    pub(crate) fn handle_ebc6(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EBC6::decode(can_id, data) {
            Ok(msg) => {
                self.braking.ebc6_lining_rear4_left = msg.brk_lnng_rmnng_rr_axl_4_lft_whl;
                self.braking.ebc6_lining_rear4_right = msg.brk_lnng_rmnng_rr_axl_4_rght_whl;
                self.braking.ebc6_lining_rear5_left = msg.brk_lnng_rmnng_rr_axl_5_lft_whl;
                self.braking.ebc6_lining_rear5_right = msg.brk_lnng_rmnng_rr_axl_5_rght_whl;
                self.braking.ebc6_lining_rear6_left = msg.brk_lnng_rmnng_rr_axl_6_lft_whl;
                self.braking.ebc6_lining_rear6_right = msg.brk_lnng_rmnng_rr_axl_6_rght_whl;
                self.braking.ebc6_lining_rear7_left = msg.brk_lnng_rmnng_rr_axl_7_lft_whl;
                self.braking.ebc6_lining_rear7_right = msg.brk_lnng_rmnng_rr_axl_7_rght_whl;
                println!(
                    "🛑 Received EBC6: Lining R4L = {:.1}%, R4R = {:.1}%",
                    self.braking.ebc6_lining_rear4_left, self.braking.ebc6_lining_rear4_right
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EBC6: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EBC7 - Electronic Brake Controller 7 (Brake Lining Axles 8-10)
    pub(crate) fn handle_ebc7(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EBC7::decode(can_id, data) {
            Ok(msg) => {
                self.braking.ebc7_lining_rear8_left = msg.brk_lnng_rmnng_rr_axl_8_lft_whl;
                self.braking.ebc7_lining_rear8_right = msg.brk_lnng_rmnng_rr_axl_8_rght_whl;
                self.braking.ebc7_lining_rear9_left = msg.brk_lnng_rmnng_rr_axl_9_lft_whl;
                self.braking.ebc7_lining_rear9_right = msg.brk_lnng_rmnng_rr_axl_9_rght_whl;
                self.braking.ebc7_lining_rear10_left = msg.brk_lnng_rmnng_rr_axl_10_lft_whl;
                self.braking.ebc7_lining_rear10_right = msg.brk_lnng_rmnng_rr_axl_10_rght_whl;
                println!(
                    "🛑 Received EBC7: Lining R8L = {:.1}%, R8R = {:.1}%",
                    self.braking.ebc7_lining_rear8_left, self.braking.ebc7_lining_rear8_right
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EBC7: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EBCC - Engine Brake Continuous Control
    pub(crate) fn handle_ebcc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EBCC::decode(can_id, data) {
            Ok(msg) => {
                self.braking.ebcc_turbo1_outlet_pressure = msg.engn_trhrgr_1_trn_otlt_prssr;
                self.braking.ebcc_turbo1_desired_outlet_pressure = msg.engn_trhrgr_1_trn_dsrd_otlt_prssr;
                self.braking.ebcc_exhaust_brake_command = msg.engn_exhst_brk_attr_cmmnd;
                self.braking.ebcc_turbo2_outlet_pressure = msg.engn_trhrgr_2_trn_otlt_prssr;
                self.braking.ebcc_turbo2_desired_outlet_pressure = msg.engn_trhrgr_2_trn_dsrd_otlt_prssr;
                println!(
                    "🛑 Received EBCC: Exhaust brake = {:.1}%",
                    self.braking.ebcc_exhaust_brake_command
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EBCC: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle XBR - External Brake Request
    pub(crate) fn handle_xbr(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match XBR::decode(can_id, data) {
            Ok(msg) => {
                self.braking.xbr_acceleration_demand = msg.external_acceleration_demand;
                self.braking.xbr_ebi_mode = msg.xbr_ebi_mode;
                self.braking.xbr_priority = msg.xbr_priority;
                self.braking.xbr_control_mode = msg.xbr_control_mode;
                self.braking.xbr_compensation_mode = msg.xbr_compensation_mode;
                self.braking.xbr_urgency = msg.xbr_urgency;
                self.braking.xbr_brake_hold_request = msg.xbr_brake_hold_request;
                self.braking.xbr_reason = msg.xbr_reason;
                self.braking.xbr_message_counter = msg.xbr_message_counter;
                self.braking.xbr_message_checksum = msg.xbr_message_checksum;
                println!(
                    "🛑 Received XBR: Demand = {:.2} m/s^2, Priority = {}, Mode = {}",
                    self.braking.xbr_acceleration_demand, self.braking.xbr_priority, self.braking.xbr_control_mode
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode XBR: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle AEBS1 - Advanced Emergency Braking System 1
    pub(crate) fn handle_aebs1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AEBS1::decode(can_id, data) {
            Ok(msg) => {
                self.braking.aebs1_forward_collision_status = msg.fwd_cs_advd_eb_sst_stt;
                self.braking.aebs1_collision_warning_level = msg.collision_warning_level;
                self.braking.aebs1_relevant_object_detected = msg.rvt_ot_dttd_f_advd_eb_sst;
                self.braking.aebs1_bound_offset = msg.bnd_off_prlt_of_rlvnt_ojt;
                self.braking.aebs1_time_to_collision = msg.tm_t_cllsn_wth_rlvnt_ojt;
                self.braking.aebs1_road_departure_status = msg.rd_dprtr_advnd_emrgn_brkng_sstm_stt;
                println!(
                    "🛑 Received AEBS1: Collision status = {}, Warning = {}, TTC = {:.1}s",
                    self.braking.aebs1_forward_collision_status,
                    self.braking.aebs1_collision_warning_level,
                    self.braking.aebs1_time_to_collision
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode AEBS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ACC1 - Adaptive Cruise Control 1
    pub(crate) fn handle_acc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ACC1::decode(can_id, data) {
            Ok(msg) => {
                self.braking.acc1_speed_of_forward_vehicle = msg.speed_of_forward_vehicle;
                self.braking.acc1_distance_to_forward_vehicle = msg.distance_to_forward_vehicle;
                self.braking.acc1_set_speed = msg.adaptive_cruise_control_set_speed;
                self.braking.acc1_mode = msg.adaptive_cruise_control_mode;
                self.braking.acc1_set_distance_mode = msg.adptv_crs_cntrl_st_dstn_md;
                self.braking.acc1_road_curvature = msg.road_curvature;
                self.braking.acc1_target_detected = msg.acc_target_detected;
                self.braking.acc1_system_shutoff_warning = msg.acc_system_shutoff_warning;
                self.braking.acc1_distance_alert = msg.acc_distance_alert_signal;
                self.braking.acc1_forward_collision_warning = msg.forward_collision_warning;
                println!(
                    "🛑 Received ACC1: Set speed = {}, Mode = {}, Target = {}",
                    self.braking.acc1_set_speed, self.braking.acc1_mode, self.braking.acc1_target_detected
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ACC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ACC2 - Adaptive Cruise Control 2
    pub(crate) fn handle_acc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ACC2::decode(can_id, data) {
            Ok(msg) => {
                self.braking.acc2_usage_demand = msg.acc_usage_demand;
                self.braking.acc2_distance_mode = msg.requested_acc_distance_mode;
                println!(
                    "🛑 Received ACC2: Usage = {}, Distance mode = {}",
                    self.braking.acc2_usage_demand, self.braking.acc2_distance_mode
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ACC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ACCS - Acceleration Sensor
    pub(crate) fn handle_accs(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ACCS::decode(can_id, data) {
            Ok(msg) => {
                self.braking.accs_lateral_acceleration = msg.ltrl_alrtn_extndd_rng;
                self.braking.accs_longitudinal_acceleration = msg.lngtdnl_alrtn_extndd_rng;
                self.braking.accs_vertical_acceleration = msg.vrtl_alrtn_extndd_rng;
                self.braking.accs_lateral_fom = msg.ltrl_alrtn_fgr_of_mrt_extndd_rng;
                self.braking.accs_longitudinal_fom = msg.lngtdnl_alrtn_fgr_of_mrt_extndd_rng;
                self.braking.accs_vertical_fom = msg.vrtl_alrtn_fgr_of_mrt_extndd_rng;
                println!(
                    "🛑 Received ACCS: Lat = {:.2}, Long = {:.2}, Vert = {:.2} m/s^2",
                    self.braking.accs_lateral_acceleration,
                    self.braking.accs_longitudinal_acceleration,
                    self.braking.accs_vertical_acceleration
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ACCS: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ACCVC - Aftercooler Coolant Valve Control
    pub(crate) fn handle_accvc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ACCVC::decode(can_id, data) {
            Ok(msg) => {
                self.braking.accvc_aftercooler_thermostat_mode = msg.engn_aftrlr_clnt_thrmstt_md;
                self.braking.accvc_desired_aftercooler_temp = msg.engn_dsrd_aftrlr_clnt_intk_tmprtr;
                self.braking.accvc_desired_thermostat_opening = msg.engn_dsrd_aftrlr_clnt_thrmstt_opnng;
                self.braking.accvc_charge_air_bypass_valve1_cmd = msg.engn_chrg_ar_clr_bpss_vlv_1_cmmnd;
                self.braking.accvc_charge_air_bypass_valve2_cmd = msg.engn_chrg_ar_clr_bpss_vlv_2_cmmnd;
                self.braking.accvc_aftercooler_diverter_valve_cmd = msg.engn_aftrlr_clnt_dvrtr_vlv_cmmnd;
                println!(
                    "🛑 Received ACCVC: Thermostat mode = {}, Temp = {:.1}C",
                    self.braking.accvc_aftercooler_thermostat_mode,
                    self.braking.accvc_desired_aftercooler_temp
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ACCVC: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ERC1 - Electronic Retarder Controller 1
    pub(crate) fn handle_erc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ERC1::decode(can_id, data) {
            Ok(msg) => {
                self.braking.erc1_retarder_torque_mode = msg.retarder_torque_mode;
                self.braking.erc1_enable_brake_assist = msg.retarder_enable_brake_assist_switch;
                self.braking.erc1_enable_shift_assist = msg.retarder_enable_shift_assist_switch;
                self.braking.erc1_actual_retarder_torque = msg.actual_retarder_percent_torque;
                self.braking.erc1_intended_retarder_torque = msg.intended_retarder_percent_torque;
                self.braking.erc1_coolant_load_increase = msg.engine_coolant_load_increase;
                self.braking.erc1_requesting_brake_light = msg.retarder_requesting_brake_light;
                self.braking.erc1_drivers_demand_torque = msg.drvrs_dmnd_rtrdr_prnt_trq;
                self.braking.erc1_selection_non_engine = msg.retarder_selection_non_engine;
                self.braking.erc1_max_available_torque = msg.atl_mxmm_avll_rtrdr_prnt_trq;
                println!(
                    "🛑 Received ERC1: Actual torque = {:.1}%, Mode = {}",
                    self.braking.erc1_actual_retarder_torque, self.braking.erc1_retarder_torque_mode
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ERC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ERC2 - Electronic Retarder Controller 2
    pub(crate) fn handle_erc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ERC2::decode(can_id, data) {
            Ok(msg) => {
                self.braking.erc2_transmission_output_retarder = msg.transmission_output_retarder;
                self.braking.erc2_road_speed_limit_enable = msg.retarder_road_speed_limit_enable;
                self.braking.erc2_road_speed_limit_active = msg.retarder_road_speed_limit_active;
                self.braking.erc2_transmission_retarder_enable = msg.trnsmssn_rtrdr_enl_swth;
                self.braking.erc2_cruise_control_speed_offset = msg.crs_cntrl_rtrdr_atv_spd_offst;
                self.braking.erc2_road_speed_limit_set_speed = msg.retarder_road_speed_limit_set_speed;
                self.braking.erc2_road_speed_limit_readiness = msg.retarder_road_speed_limit_readiness;
                self.braking.erc2_retarder_derate_status = msg.retarder_derate_status;
                println!(
                    "🛑 Received ERC2: Speed limit = {:.1} km/h, Active = {}",
                    self.braking.erc2_road_speed_limit_set_speed, self.braking.erc2_road_speed_limit_active
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ERC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle RC - Retarder Configuration
    pub(crate) fn handle_rc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match RC::decode(can_id, data) {
            Ok(msg) => {
                self.braking.rc_retarder_type = msg.retarder_type;
                self.braking.rc_retarder_location = msg.retarder_location;
                self.braking.rc_control_method = msg.retarder_control_method;
                self.braking.rc_speed_at_idle = msg.retarder_speed_at_idle_point_1;
                self.braking.rc_torque_at_idle = msg.rtrdr_prnt_trq_at_idl_pnt_1;
                self.braking.rc_max_speed = msg.maximum_retarder_speed_point_2;
                self.braking.rc_torque_at_max_speed = msg.rtrdr_prnt_trq_at_mxmm_spd_pnt_2;
                self.braking.rc_speed_at_point3 = msg.retarder_speed_at_point_3;
                self.braking.rc_torque_at_point3 = msg.retarder_percent_torque_at_point_3;
                self.braking.rc_speed_at_point4 = msg.retarder_speed_at_point_4;
                self.braking.rc_torque_at_point4 = msg.retarder_percent_torque_at_point_4;
                self.braking.rc_speed_at_peak_torque = msg.retarder_speed_at_peak_torque_point_5;
                self.braking.rc_reference_torque = msg.retarder_reference_torque;
                self.braking.rc_torque_at_peak = msg.rtrdr_prnt_trq_at_pk_trq_pnt_5;
                println!(
                    "🛑 Received RC: Type = {}, Location = {}, Ref torque = {} Nm",
                    self.braking.rc_retarder_type, self.braking.rc_retarder_location, self.braking.rc_reference_torque
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode RC: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle LMP - Mast Position
    pub(crate) fn handle_lmp(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match LMP::decode(can_id, data) {
            Ok(msg) => {
                self.braking.lmp_mast_position = msg.mast_position;
                println!(
                    "🛑 Received LMP: Mast position = {:.1}%",
                    self.braking.lmp_mast_position
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode LMP: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }
}
