use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle EEC12 - Engine Exhaust Sensor Power Supply
    pub(crate) fn handle_eec12(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // EEC12 - Engine Exhaust Sensor Power Supply
        match EEC12::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec12_engn_exhst_1_gs_snsr_1_pwr_sppl =
                    msg.engn_exhst_1_gs_snsr_1_pwr_sppl;
                self.engine.eec12_aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl =
                    msg.aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl;
                self.engine.eec12_engn_exhst_2_gs_snsr_1_pwr_sppl =
                    msg.engn_exhst_2_gs_snsr_1_pwr_sppl;
                self.engine.eec12_aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl =
                    msg.aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl;
                self.engine.eec12_engn_exhst_1_gs_snsr_2_pwr_sppl =
                    msg.engn_exhst_1_gs_snsr_2_pwr_sppl;
                self.engine.eec12_aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl =
                    msg.aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl;
                println!(
                    "🔧 Received EEC12: Exhaust sensors power = [{}, {}, {}, {}, {}, {}]",
                    self.engine.eec12_engn_exhst_1_gs_snsr_1_pwr_sppl,
                    self.engine.eec12_aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl,
                    self.engine.eec12_engn_exhst_2_gs_snsr_1_pwr_sppl,
                    self.engine.eec12_aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl,
                    self.engine.eec12_engn_exhst_1_gs_snsr_2_pwr_sppl,
                    self.engine.eec12_aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC12: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC22 - Electronic Engine Controller 22
    pub(crate) fn handle_eec22(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // EEC22 - Electronic Engine Controller 22
        match EEC22::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec22_engnexhstgsrrltn1clrintkprssr =
                    msg.engn_exhst_gs_rrltn_1_clr_intk_prssr;
                self.engine.eec22_ttlnmrofcrnkattmptsdrngengnlf =
                    msg.ttl_nmr_of_crnk_attmpts_drng_engn_lf;
                println!(
                    "🔧 Received EEC22: Cooler pressure = {:.1} kPa, Crank attempts = {}",
                    self.engine.eec22_engnexhstgsrrltn1clrintkprssr,
                    self.engine.eec22_ttlnmrofcrnkattmptsdrngengnlf
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC22: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC21 - Electronic Engine Controller 21
    pub(crate) fn handle_eec21(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // EEC21 - Electronic Engine Controller 21
        match EEC21::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec21_engn_exhst_mnfld_aslt_prssr_1 =
                    msg.engn_exhst_mnfld_aslt_prssr_1;
                self.engine.eec21_engn_exhst_mnfld_aslt_prssr_2 =
                    msg.engn_exhst_mnfld_aslt_prssr_2;
                println!(
                    "🔥 Received EEC21: Exhaust pressure = [{:.1} kPa, {:.1} kPa]",
                    self.engine.eec21_engn_exhst_mnfld_aslt_prssr_1,
                    self.engine.eec21_engn_exhst_mnfld_aslt_prssr_2
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC21: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC17 - Electronic Engine Controller 17
    pub(crate) fn handle_eec17(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // EEC17 - Electronic Engine Controller 17
        match EEC17::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec17_pems_engine_fuel_mass_flow_rate =
                    msg.pems_engine_fuel_mass_flow_rate;
                self.engine.eec17_vehicle_fuel_rate = msg.vehicle_fuel_rate;
                self.engine.eec17_engine_exhaust_flow_rate = msg.engine_exhaust_flow_rate;
                self.engine.eec17_cylinder_fuel_rate = msg.cylinder_fuel_rate;
                println!(
                    "🔧 Received EEC17: PEMS fuel = {:.1} g/s, Vehicle fuel = {:.1} g/s, Exhaust flow = {:.1} kg/h, Cylinder fuel = {:.1} mg/stroke",
                    self.engine.eec17_pems_engine_fuel_mass_flow_rate,
                    self.engine.eec17_vehicle_fuel_rate,
                    self.engine.eec17_engine_exhaust_flow_rate,
                    self.engine.eec17_cylinder_fuel_rate
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC17: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC15 - Electronic Engine Controller 15
    pub(crate) fn handle_eec15(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // EEC15 - Electronic Engine Controller 15
        match EEC15::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec15_accelerator_pedal_1_channel_2 =
                    msg.accelerator_pedal_1_channel_2;
                self.engine.eec15_accelerator_pedal_1_channel_3 =
                    msg.accelerator_pedal_1_channel_3;
                self.engine.eec15_accelerator_pedal_2_channel_2 =
                    msg.accelerator_pedal_2_channel_2;
                self.engine.eec15_accelerator_pedal_2_channel_3 =
                    msg.accelerator_pedal_2_channel_3;
                self.engine.eec15_engn_exhst_gs_rstrtn_vlv_cntrl =
                    msg.engn_exhst_gs_rstrtn_vlv_cntrl;
                println!(
                    "🔧 Received EEC15: Pedal1 Ch2 = {:.1}%, Pedal1 Ch3 = {:.1}%, Pedal2 Ch2 = {:.1}%, Pedal2 Ch3 = {:.1}%, Restriction valve = {:.1}%",
                    self.engine.eec15_accelerator_pedal_1_channel_2,
                    self.engine.eec15_accelerator_pedal_1_channel_3,
                    self.engine.eec15_accelerator_pedal_2_channel_2,
                    self.engine.eec15_accelerator_pedal_2_channel_3,
                    self.engine.eec15_engn_exhst_gs_rstrtn_vlv_cntrl
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC15: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC8 - Electronic Engine Controller 8
    pub(crate) fn handle_eec8(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // EEC8 - Electronic Engine Controller 8
        match EEC8::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec8_engn_exhst_gs_rrltn_1_vlv_2_cntrl =
                    msg.engn_exhst_gs_rrltn_1_vlv_2_cntrl;
                self.engine.eec8_engn_exhst_gs_rrltn_1_clr_intk_tmprtr =
                    msg.engn_exhst_gs_rrltn_1_clr_intk_tmprtr;
                self.engine.eec8_e_exst_gs_rt_1_c_it_ast_pss = msg.e_exst_gs_rt_1_c_it_ast_pss;
                self.engine.eec8_engn_exhst_gs_rrltn_1_clr_effn =
                    msg.engn_exhst_gs_rrltn_1_clr_effn;
                self.engine.eec8_e_exst_gs_rt_at_it_ct_tpt = msg.e_exst_gs_rt_at_it_ct_tpt;
                println!(
                    "🔧 Received EEC8: EGR valve = {:.1}%, Cooler temp = {:.1}°C, Pressure = {:.1} kPa, Efficiency = {:.1}%, Throttle = {}",
                    self.engine.eec8_engn_exhst_gs_rrltn_1_vlv_2_cntrl,
                    self.engine.eec8_engn_exhst_gs_rrltn_1_clr_intk_tmprtr,
                    self.engine.eec8_e_exst_gs_rt_1_c_it_ast_pss,
                    self.engine.eec8_engn_exhst_gs_rrltn_1_clr_effn,
                    self.engine.eec8_e_exst_gs_rt_at_it_ct_tpt
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC8: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC1 - Electronic Engine Controller 1
    pub(crate) fn handle_eec1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC1::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec1_engine_torque_mode = msg.engine_torque_mode;
                self.engine.eec1_atl_engn_prnt_trq_frtnl = msg.atl_engn_prnt_trq_frtnl;
                self.engine.eec1_drvr_s_dmnd_engn_prnt_trq = msg.drvr_s_dmnd_engn_prnt_trq;
                self.engine.eec1_actual_engine_percent_torque = msg.actual_engine_percent_torque;
                self.engine.eec1_engine_speed = msg.engine_speed;
                self.engine.eec1_sr_addrss_of_cntrllng_dv_fr_engn_cntrl = msg.sr_addrss_of_cntrllng_dv_fr_engn_cntrl;
                self.engine.eec1_engine_starter_mode = msg.engine_starter_mode;
                self.engine.eec1_engine_demand_percent_torque = msg.engine_demand_percent_torque;
                println!(
                    "🔧 Received EEC1: Speed={:.1} rpm, Torque={:.1}%, Demand={:.1}%",
                    self.engine.eec1_engine_speed,
                    self.engine.eec1_actual_engine_percent_torque,
                    self.engine.eec1_engine_demand_percent_torque
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC2 - Electronic Engine Controller 2
    pub(crate) fn handle_eec2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC2::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec2_accelerator_pedal_1_low_idle_switch = msg.accelerator_pedal_1_low_idle_switch;
                self.engine.eec2_accelerator_pedal_kickdown_switch = msg.accelerator_pedal_kickdown_switch;
                self.engine.eec2_road_speed_limit_status = msg.road_speed_limit_status;
                self.engine.eec2_accelerator_pedal_2_low_idle_switch = msg.accelerator_pedal_2_low_idle_switch;
                self.engine.eec2_accelerator_pedal_1_position = msg.accelerator_pedal_1_position;
                self.engine.eec2_engine_percent_load_at_current_speed = msg.engine_percent_load_at_current_speed;
                self.engine.eec2_remote_accelerator_pedal_position = msg.remote_accelerator_pedal_position;
                self.engine.eec2_accelerator_pedal_2_position = msg.accelerator_pedal_2_position;
                self.engine.eec2_vhl_alrtn_rt_lmt_stts = msg.vhl_alrtn_rt_lmt_stts;
                self.engine.eec2_mmntr_engn_mxmm_pwr_enl_fdk = msg.mmntr_engn_mxmm_pwr_enl_fdk;
                self.engine.eec2_dpf_thermal_management_active = msg.dpf_thermal_management_active;
                self.engine.eec2_scr_thermal_management_active = msg.scr_thermal_management_active;
                self.engine.eec2_atl_mxmm_avll_engn_prnt_trq = msg.atl_mxmm_avll_engn_prnt_trq;
                self.engine.eec2_estimated_pumping_percent_torque = msg.estimated_pumping_percent_torque;
                println!(
                    "🔧 Received EEC2: Pedal1={:.1}%, Load={}, Pedal2={:.1}%",
                    self.engine.eec2_accelerator_pedal_1_position,
                    self.engine.eec2_engine_percent_load_at_current_speed,
                    self.engine.eec2_accelerator_pedal_2_position
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC3 - Electronic Engine Controller 3
    pub(crate) fn handle_eec3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC3::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec3_nominal_friction_percent_torque = msg.nominal_friction_percent_torque;
                self.engine.eec3_engine_s_desired_operating_speed = msg.engine_s_desired_operating_speed;
                self.engine.eec3_es_dsd_opt_spd_ast_adstt = msg.es_dsd_opt_spd_ast_adstt;
                self.engine.eec3_estmtd_engn_prst_lsss_prnt_trq = msg.estmtd_engn_prst_lsss_prnt_trq;
                self.engine.eec3_aftrtrtmnt_1_exhst_gs_mss_flw_rt = msg.aftrtrtmnt_1_exhst_gs_mss_flw_rt;
                self.engine.eec3_engine_exhaust_1_dew_point = msg.engine_exhaust_1_dew_point;
                self.engine.eec3_aftertreatment_1_exhaust_dew_point = msg.aftertreatment_1_exhaust_dew_point;
                self.engine.eec3_engine_exhaust_2_dew_point = msg.engine_exhaust_2_dew_point;
                self.engine.eec3_aftertreatment_2_exhaust_dew_point = msg.aftertreatment_2_exhaust_dew_point;
                println!(
                    "🔧 Received EEC3: Friction={:.1}%, DesiredSpeed={:.1} rpm, ExhaustFlow={:.1} kg/h",
                    self.engine.eec3_nominal_friction_percent_torque,
                    self.engine.eec3_engine_s_desired_operating_speed,
                    self.engine.eec3_aftrtrtmnt_1_exhst_gs_mss_flw_rt
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC4 - Electronic Engine Controller 4
    pub(crate) fn handle_eec4(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC4::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec4_engine_rated_power = msg.engine_rated_power;
                self.engine.eec4_engine_rated_speed = msg.engine_rated_speed;
                self.engine.eec4_engine_rotation_direction = msg.engine_rotation_direction;
                self.engine.eec4_engn_intk_mnfld_prssr_cntrl_md = msg.engn_intk_mnfld_prssr_cntrl_md;
                self.engine.eec4_crnk_attmpt_cnt_on_prsnt_strt_attmpt = msg.crnk_attmpt_cnt_on_prsnt_strt_attmpt;
                self.engine.eec4_engn_prl_ol_lw_prssr_thrshld = msg.engn_prl_ol_lw_prssr_thrshld;
                println!(
                    "🔧 Received EEC4: RatedPower={:.1} kW, RatedSpeed={:.1} rpm, Rotation={}",
                    self.engine.eec4_engine_rated_power,
                    self.engine.eec4_engine_rated_speed,
                    self.engine.eec4_engine_rotation_direction
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC4: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC5 - Electronic Engine Controller 5
    pub(crate) fn handle_eec5(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC5::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec5_engn_trhrgr_1_clltd_trn_intk_tmprtr = msg.engn_trhrgr_1_clltd_trn_intk_tmprtr;
                self.engine.eec5_engn_trhrgr_1_clltd_trn_otlt_tmprtr = msg.engn_trhrgr_1_clltd_trn_otlt_tmprtr;
                self.engine.eec5_engn_exhst_gs_rrltn_1_vlv_1_cntrl_1 = msg.engn_exhst_gs_rrltn_1_vlv_1_cntrl_1;
                self.engine.eec5_ev_gt_t_vt_a_ct_st_vv = msg.ev_gt_t_vt_a_ct_st_vv;
                self.engine.eec5_engine_fuel_control_mode = msg.engine_fuel_control_mode;
                self.engine.eec5_engn_vrl_gmtr_trhrgr_1_cntrl_md = msg.engn_vrl_gmtr_trhrgr_1_cntrl_md;
                self.engine.eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn = msg.engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn;
                println!(
                    "🔧 Received EEC5: TurbineIntake={:.1}°C, TurbineOutlet={:.1}°C, EGR={:.1}%, VGT={:.1}%",
                    self.engine.eec5_engn_trhrgr_1_clltd_trn_intk_tmprtr,
                    self.engine.eec5_engn_trhrgr_1_clltd_trn_otlt_tmprtr,
                    self.engine.eec5_engn_exhst_gs_rrltn_1_vlv_1_cntrl_1,
                    self.engine.eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC5: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC6 - Electronic Engine Controller 6
    pub(crate) fn handle_eec6(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC6::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_cmmnd = msg.engn_trhrgr_cmprssr_bpss_attr_1_cmmnd;
                self.engine.eec6_engn_vrl_gmtr_trhrgr_attr_1 = msg.engn_vrl_gmtr_trhrgr_attr_1;
                self.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_pstn = msg.engn_trhrgr_cmprssr_bpss_attr_1_pstn;
                self.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_2_cmmnd = msg.engn_trhrgr_cmprssr_bpss_attr_2_cmmnd;
                self.engine.eec6_et_cpss_bpss_att_1_dsd_pst = msg.et_cpss_bpss_att_1_dsd_pst;
                self.engine.eec6_et_cpss_bpss_att_1_pf = msg.et_cpss_bpss_att_1_pf;
                self.engine.eec6_et_cpss_bpss_att_1_tpt_stts = msg.et_cpss_bpss_att_1_tpt_stts;
                println!(
                    "🔧 Received EEC6: Bypass1Cmd={:.1}%, VGT={:.1}%, Bypass1Pos={:.1}%",
                    self.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_cmmnd,
                    self.engine.eec6_engn_vrl_gmtr_trhrgr_attr_1,
                    self.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_pstn
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC6: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC7 - Electronic Engine Controller 7
    pub(crate) fn handle_eec7(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC7::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec7_engn_exhst_gs_rrltn_1_vlv_pstn = msg.engn_exhst_gs_rrltn_1_vlv_pstn;
                self.engine.eec7_engn_exhst_gs_rrltn_1_vlv_2_pstn = msg.engn_exhst_gs_rrltn_1_vlv_2_pstn;
                self.engine.eec7_engn_crnks_brthr_ol_sprtr_spd = msg.engn_crnks_brthr_ol_sprtr_spd;
                self.engine.eec7_engn_intk_mnfld_cmmndd_prssr = msg.engn_intk_mnfld_cmmndd_prssr;
                println!(
                    "🔧 Received EEC7: EGR1Pos={:.1}%, EGR1V2Pos={:.1}%, OilSepSpd={}, IntakePressCmd={:.1} kPa",
                    self.engine.eec7_engn_exhst_gs_rrltn_1_vlv_pstn,
                    self.engine.eec7_engn_exhst_gs_rrltn_1_vlv_2_pstn,
                    self.engine.eec7_engn_crnks_brthr_ol_sprtr_spd,
                    self.engine.eec7_engn_intk_mnfld_cmmndd_prssr
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC7: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC9 - Electronic Engine Controller 9
    pub(crate) fn handle_eec9(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC9::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec9_engn_exhst_gs_rrltn_2_vlv_pstn = msg.engn_exhst_gs_rrltn_2_vlv_pstn;
                self.engine.eec9_engn_exhst_gs_rrltn_2_vlv_2_pstn = msg.engn_exhst_gs_rrltn_2_vlv_2_pstn;
                self.engine.eec9_commanded_engine_fuel_rail_pressure = msg.commanded_engine_fuel_rail_pressure;
                self.engine.eec9_cmmndd_engn_fl_injtn_cntrl_prssr = msg.cmmndd_engn_fl_injtn_cntrl_prssr;
                println!(
                    "🔧 Received EEC9: EGR2Pos={:.1}%, EGR2V2Pos={:.1}%, FuelRail={:.1} MPa, InjCtrl={:.1} MPa",
                    self.engine.eec9_engn_exhst_gs_rrltn_2_vlv_pstn,
                    self.engine.eec9_engn_exhst_gs_rrltn_2_vlv_2_pstn,
                    self.engine.eec9_commanded_engine_fuel_rail_pressure,
                    self.engine.eec9_cmmndd_engn_fl_injtn_cntrl_prssr
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC9: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC10 - Electronic Engine Controller 10
    pub(crate) fn handle_eec10(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC10::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec10_engn_exhst_gs_rrltn_2_clr_intk_tmprtr = msg.engn_exhst_gs_rrltn_2_clr_intk_tmprtr;
                self.engine.eec10_e_exst_gs_rt_2_c_it_ast_pss = msg.e_exst_gs_rt_2_c_it_ast_pss;
                self.engine.eec10_engn_exhst_gs_rrltn_2_clr_effn = msg.engn_exhst_gs_rrltn_2_clr_effn;
                self.engine.eec10_e_exst_gs_rt_2_c_bpss_att_pst = msg.e_exst_gs_rt_2_c_bpss_att_pst;
                self.engine.eec10_engn_exhst_gs_rrltn_2_clr_intk_prssr = msg.engn_exhst_gs_rrltn_2_clr_intk_prssr;
                println!(
                    "🔧 Received EEC10: EGR2CoolerTemp={:.1}°C, Pressure={:.1} kPa, Efficiency={:.1}%",
                    self.engine.eec10_engn_exhst_gs_rrltn_2_clr_intk_tmprtr,
                    self.engine.eec10_e_exst_gs_rt_2_c_it_ast_pss,
                    self.engine.eec10_engn_exhst_gs_rrltn_2_clr_effn
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC10: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC11 - Electronic Engine Controller 11
    pub(crate) fn handle_eec11(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC11::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_1_cntrl = msg.engn_exhst_gs_rrltn_2_vlv_1_cntrl;
                self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_2_cntrl = msg.engn_exhst_gs_rrltn_2_vlv_2_cntrl;
                self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_1_pstn_errr = msg.engn_exhst_gs_rrltn_2_vlv_1_pstn_errr;
                self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_2_pstn_errr = msg.engn_exhst_gs_rrltn_2_vlv_2_pstn_errr;
                println!(
                    "🔧 Received EEC11: EGR2V1Ctrl={:.1}%, EGR2V2Ctrl={:.1}%, V1Err={:.1}%, V2Err={:.1}%",
                    self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_1_cntrl,
                    self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_2_cntrl,
                    self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_1_pstn_errr,
                    self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_2_pstn_errr
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC11: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC13 - Electronic Engine Controller 13
    pub(crate) fn handle_eec13(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC13::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec13_feedback_engine_fueling_state = msg.feedback_engine_fueling_state;
                self.engine.eec13_engine_fueling_inhibit_allowed = msg.engine_fueling_inhibit_allowed;
                self.engine.eec13_engn_flng_inht_prvntd_rsn = msg.engn_flng_inht_prvntd_rsn;
                self.engine.eec13_sr_addrss_of_cntrllng_dv_fr_flng_stt = msg.sr_addrss_of_cntrllng_dv_fr_flng_stt;
                self.engine.eec13_engine_dual_fuel_mode = msg.engine_dual_fuel_mode;
                self.engine.eec13_engn_flng_inht_prvntd_rsn_extnsn = msg.engn_flng_inht_prvntd_rsn_extnsn;
                self.engine.eec13_engn_gs_sstttn_fl_prntg = msg.engn_gs_sstttn_fl_prntg;
                self.engine.eec13_engn_flng_inht_rqst_cnt = msg.engn_flng_inht_rqst_cnt;
                self.engine.eec13_engn_flng_dsrd_rqst_cnt = msg.engn_flng_dsrd_rqst_cnt;
                self.engine.eec13_engn_prttn_drt_ovrrd_stts = msg.engn_prttn_drt_ovrrd_stts;
                self.engine.eec13_remaining_engine_motoring_time = msg.remaining_engine_motoring_time;
                self.engine.eec13_engine_performance_bias_level = msg.engine_performance_bias_level;
                self.engine.eec13_minimum_engine_motoring_speed = msg.minimum_engine_motoring_speed;
                println!(
                    "🔧 Received EEC13: FuelingState={}, DualFuel={}, GasSub={:.1}%, PerfBias={:.1}%",
                    self.engine.eec13_feedback_engine_fueling_state,
                    self.engine.eec13_engine_dual_fuel_mode,
                    self.engine.eec13_engn_gs_sstttn_fl_prntg,
                    self.engine.eec13_engine_performance_bias_level
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC13: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC14 - Electronic Engine Controller 14
    pub(crate) fn handle_eec14(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC14::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec14_engn_exhst_gs_rrltn_1_vlv_1_pstn_errr = msg.engn_exhst_gs_rrltn_1_vlv_1_pstn_errr;
                self.engine.eec14_engn_exhst_gs_rrltn_1_vlv_2_pstn_errr = msg.engn_exhst_gs_rrltn_1_vlv_2_pstn_errr;
                self.engine.eec14_engine_fuel_mass_flow_rate = msg.engine_fuel_mass_flow_rate;
                self.engine.eec14_fuel_type = msg.fuel_type;
                self.engine.eec14_engine_fuel_isolation_control = msg.engine_fuel_isolation_control;
                println!(
                    "🔧 Received EEC14: FuelFlow={:.1} g/s, FuelType={}, Isolation={}",
                    self.engine.eec14_engine_fuel_mass_flow_rate,
                    self.engine.eec14_fuel_type,
                    self.engine.eec14_engine_fuel_isolation_control
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC14: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC16 - Electronic Engine Controller 16
    pub(crate) fn handle_eec16(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC16::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec16_accelerator_pedal_3_position = msg.accelerator_pedal_3_position;
                self.engine.eec16_ready_for_clutch_engagement_status = msg.ready_for_clutch_engagement_status;
                self.engine.eec16_engine_clutch_engage_request_status = msg.engine_clutch_engage_request_status;
                println!(
                    "🔧 Received EEC16: Pedal3={:.1}%, ClutchReady={}, ClutchRequest={}",
                    self.engine.eec16_accelerator_pedal_3_position,
                    self.engine.eec16_ready_for_clutch_engagement_status,
                    self.engine.eec16_engine_clutch_engage_request_status
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC16: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC18 - Electronic Engine Controller 18
    pub(crate) fn handle_eec18(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC18::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec18_engn_clndr_hd_bpss_attr_1_cmmnd = msg.engn_clndr_hd_bpss_attr_1_cmmnd;
                self.engine.eec18_engine_intake_air_source_valve = msg.engine_intake_air_source_valve;
                self.engine.eec18_engn_exhst_gs_rstrtn_vlv_pstn = msg.engn_exhst_gs_rstrtn_vlv_pstn;
                println!(
                    "🔧 Received EEC18: CylinderBypass={:.1}%, AirSource={}, ExhaustRestriction={:.1}%",
                    self.engine.eec18_engn_clndr_hd_bpss_attr_1_cmmnd,
                    self.engine.eec18_engine_intake_air_source_valve,
                    self.engine.eec18_engn_exhst_gs_rstrtn_vlv_pstn
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC18: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC19 - Electronic Engine Controller 19
    pub(crate) fn handle_eec19(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC19::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec19_total_engine_energy = msg.total_engine_energy;
                self.engine.eec19_engn_exhst_flw_rt_extndd_rng = msg.engn_exhst_flw_rt_extndd_rng;
                println!(
                    "🔧 Received EEC19: TotalEnergy={} kWh, ExhaustFlow={:.1} kg/h",
                    self.engine.eec19_total_engine_energy,
                    self.engine.eec19_engn_exhst_flw_rt_extndd_rng
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC19: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC20 - Electronic Engine Controller 20
    pub(crate) fn handle_eec20(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC20::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec20_esttd_e_pst_lsss_pt_tq_h_rst = msg.esttd_e_pst_lsss_pt_tq_h_rst;
                self.engine.eec20_atl_mxmm_avll_engn_prnt_fl = msg.atl_mxmm_avll_engn_prnt_fl;
                self.engine.eec20_nmnl_frtn_prnt_trq_hgh_rsltn = msg.nmnl_frtn_prnt_trq_hgh_rsltn;
                self.engine.eec20_aslt_engn_ld_prnt_ar_mss = msg.aslt_engn_ld_prnt_ar_mss;
                println!(
                    "🔧 Received EEC20: ParasiticLoss={:.2}%, MaxFuel={:.1}%, Friction={:.2}%, AirMass={:.1}%",
                    self.engine.eec20_esttd_e_pst_lsss_pt_tq_h_rst,
                    self.engine.eec20_atl_mxmm_avll_engn_prnt_fl,
                    self.engine.eec20_nmnl_frtn_prnt_trq_hgh_rsltn,
                    self.engine.eec20_aslt_engn_ld_prnt_ar_mss
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC20: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC23 - Electronic Engine Controller 23
    pub(crate) fn handle_eec23(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC23::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec23_engn_crnks_prssr_cntrl_attr_1_cmmnd = msg.engn_crnks_prssr_cntrl_attr_1_cmmnd;
                self.engine.eec23_engn_crnks_prssr_cntrl_attr_2_cmmnd = msg.engn_crnks_prssr_cntrl_attr_2_cmmnd;
                println!(
                    "🔧 Received EEC23: CrankcaseCtrl1={:.1}%, CrankcaseCtrl2={:.1}%",
                    self.engine.eec23_engn_crnks_prssr_cntrl_attr_1_cmmnd,
                    self.engine.eec23_engn_crnks_prssr_cntrl_attr_2_cmmnd
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC23: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC24 - Electronic Engine Controller 24
    pub(crate) fn handle_eec24(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC24::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec24_engn_crnks_prssr_cntrl_attr_1_tmprtr = msg.engn_crnks_prssr_cntrl_attr_1_tmprtr;
                self.engine.eec24_engn_crnks_prssr_cntrl_attr_1_pstn = msg.engn_crnks_prssr_cntrl_attr_1_pstn;
                self.engine.eec24_e_cs_pss_ct_att_1_dsd_pst = msg.e_cs_pss_ct_att_1_dsd_pst;
                self.engine.eec24_e_cs_pss_ct_att_1_pf = msg.e_cs_pss_ct_att_1_pf;
                self.engine.eec24_e_cs_pss_ct_att_1_tpt_stts = msg.e_cs_pss_ct_att_1_tpt_stts;
                self.engine.eec24_e_cs_pss_ct_att_1_opt_stts = msg.e_cs_pss_ct_att_1_opt_stts;
                println!(
                    "🔧 Received EEC24: Temp={:.1}°C, Position={:.1}%, DesiredPos={:.1}%, Status={}",
                    self.engine.eec24_engn_crnks_prssr_cntrl_attr_1_tmprtr,
                    self.engine.eec24_engn_crnks_prssr_cntrl_attr_1_pstn,
                    self.engine.eec24_e_cs_pss_ct_att_1_dsd_pst,
                    self.engine.eec24_e_cs_pss_ct_att_1_opt_stts
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC24: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EEC25 - Electronic Engine Controller 25
    pub(crate) fn handle_eec25(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEC25::decode(can_id, data) {
            Ok(msg) => {
                self.engine.eec25_engn_crnks_prssr_cntrl_attr_2_tmprtr = msg.engn_crnks_prssr_cntrl_attr_2_tmprtr;
                self.engine.eec25_engn_crnks_prssr_cntrl_attr_2_pstn = msg.engn_crnks_prssr_cntrl_attr_2_pstn;
                self.engine.eec25_e_cs_pss_ct_att_2_dsd_pst = msg.e_cs_pss_ct_att_2_dsd_pst;
                self.engine.eec25_e_cs_pss_ct_att_2_pf = msg.e_cs_pss_ct_att_2_pf;
                self.engine.eec25_e_cs_pss_ct_att_2_tpt_stts = msg.e_cs_pss_ct_att_2_tpt_stts;
                self.engine.eec25_e_cs_pss_ct_att_2_opt_stts = msg.e_cs_pss_ct_att_2_opt_stts;
                println!(
                    "🔧 Received EEC25: Temp={:.1}°C, Position={:.1}%, DesiredPos={:.1}%, Status={}",
                    self.engine.eec25_engn_crnks_prssr_cntrl_attr_2_tmprtr,
                    self.engine.eec25_engn_crnks_prssr_cntrl_attr_2_pstn,
                    self.engine.eec25_e_cs_pss_ct_att_2_dsd_pst,
                    self.engine.eec25_e_cs_pss_ct_att_2_opt_stts
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EEC25: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC1 - Electronic Transmission Controller 1
    pub(crate) fn handle_etc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match ETC1::decode(can_id, data) {
            Ok(msg) => {
                self.engine.etc1_transmission_driveline_engaged = msg.transmission_driveline_engaged;
                self.engine.etc1_trnsmssn_trq_cnvrtr_lkp_enggd = msg.trnsmssn_trq_cnvrtr_lkp_enggd;
                self.engine.etc1_transmission_shift_in_process = msg.transmission_shift_in_process;
                self.engine.etc1_tsss_tq_cvt_lp_tst_i_pss = msg.tsss_tq_cvt_lp_tst_i_pss;
                self.engine.etc1_transmission_output_shaft_speed = msg.transmission_output_shaft_speed;
                self.engine.etc1_percent_clutch_slip = msg.percent_clutch_slip;
                self.engine.etc1_engine_momentary_overspeed_enable = msg.engine_momentary_overspeed_enable;
                self.engine.etc1_progressive_shift_disable = msg.progressive_shift_disable;
                self.engine.etc1_mmntr_engn_mxmm_pwr_enl = msg.mmntr_engn_mxmm_pwr_enl;
                self.engine.etc1_transmission_input_shaft_speed = msg.transmission_input_shaft_speed;
                self.engine.etc1_s_addss_o_ct_dv_f_tsss_ct = msg.s_addss_o_ct_dv_f_tsss_ct;
                println!(
                    "🔧 Received ETC1: OutputShaft={:.1} rpm, InputShaft={:.1} rpm, ClutchSlip={:.1}%, Engaged={}",
                    self.engine.etc1_transmission_output_shaft_speed,
                    self.engine.etc1_transmission_input_shaft_speed,
                    self.engine.etc1_percent_clutch_slip,
                    self.engine.etc1_transmission_driveline_engaged
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }
}
