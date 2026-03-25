use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    pub(crate) fn handle_at1s1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1S1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1s1_dpf_soot_load_percent = msg.aftrtrtmnt_1_dsl_prtlt_fltr_st_ld_prnt;
                self.aftertreatment.at1s1_dpf_ash_load_percent = msg.atttt_1_ds_ptt_ft_as_ld_pt;
                self.aftertreatment.at1s1_dpf_time_since_last_regen = msg.atttt_1_ds_ptt_ft_ts_lst_atv_rt;
                self.aftertreatment.at1s1_dpf_soot_load_regen_threshold = msg.atttt_1_ds_ptt_ft_st_ld_rt_tsd;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1S2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1s2_dpf_time_to_next_regen = msg.atttt_1_ds_ptt_ft_tt_nxt_atv_rt;
                self.aftertreatment.at1s2_scr_time_since_cleaning = msg.atttt_1_s_sst_ts_lst_sst_c_evt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1t1i1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1T1I1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1t1i1_def_tank_volume = msg.aftrtrtmnt_1_dsl_exhst_fld_tnk_vlm;
                self.aftertreatment.at1t1i1_def_tank_temp = msg.atttt_1_ds_exst_fd_t_tpt_1;
                self.aftertreatment.at1t1i1_def_tank_level = msg.aftrtrtmnt_1_dsl_exhst_fld_tnk_lvl;
                self.aftertreatment.at1t1i1_def_tank_heater = msg.aftrtrtmnt_1_dsl_exhst_fld_tnk_htr;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1t1i2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1T1I2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1t1i2_def_tank_volume_2 = msg.aftrtrtmnt_1_dsl_exhst_fld_tnk_vlm_2;
                self.aftertreatment.at1t1i2_def_tank_temp_2 = msg.atttt_1_ds_exst_fd_t_tpt_2;
                self.aftertreatment.at1t1i2_def_tank_heater_2 = msg.aftrtrtmnt_1_dsl_exhst_fld_tnk_htr_2;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1ti(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1TI::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1ti_dpf_trip_fuel_used = msg.aftrtrtmnt_1_dsl_prtlt_fltr_trp_fl_usd;
                self.aftertreatment.at1ti_dpf_trip_active_regen_time = msg.atttt_1_ds_ptt_ft_tp_atv_rt_t;
                self.aftertreatment.at1ti_dpf_trip_disabled_time = msg.atttt_1_ds_ptt_ft_tp_dsd_t;
                self.aftertreatment.at1ti_dpf_trip_num_active_regens = msg.atttt_1_ds_ptt_ft_tp_no_atv_rts;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1og1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1OG1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1og1_outlet_nox = msg.aftertreatment_1_outlet_nox_1;
                self.aftertreatment.at1og1_outlet_oxygen = msg.aftrtrtmnt_1_otlt_prnt_oxgn_1;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1og2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1OG2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1og2_exhaust_temp_3 = msg.aftrtrtmnt_1_exhst_tmprtr_3;
                self.aftertreatment.at1og2_dpf_outlet_temp = msg.atttt_1_ds_ptt_ft_ott_tpt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1ig1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1IG1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1ig1_inlet_nox = msg.engine_exhaust_1_nox_1;
                self.aftertreatment.at1ig1_inlet_oxygen = msg.engine_exhaust_1_percent_oxygen_1;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1ig2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1IG2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1ig2_exhaust_temp_1 = msg.aftrtrtmnt_1_exhst_tmprtr_1;
                self.aftertreatment.at1ig2_dpf_intake_temp = msg.atttt_1_ds_ptt_ft_it_tpt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1hi1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1HI1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1hi1_total_fuel_used = msg.aftertreatment_1_total_fuel_used;
                self.aftertreatment.at1hi1_total_regen_time = msg.aftrtrtmnt_1_ttl_rgnrtn_tm;
                self.aftertreatment.at1hi1_total_disabled_time = msg.aftrtrtmnt_1_ttl_dsld_tm;
                self.aftertreatment.at1hi1_total_num_active_regens = msg.aftrtrtmnt_1_ttl_nmr_of_atv_rgnrtns;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1gp(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1GP::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1gp_dpf_intake_pressure = msg.atttt_1_ds_ptt_ft_it_pss;
                self.aftertreatment.at1gp_dpf_outlet_pressure = msg.atttt_1_ds_ptt_ft_ott_pss;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1fc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1FC1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1fc1_fuel_pressure_1 = msg.aftertreatment_1_fuel_pressure_1;
                self.aftertreatment.at1fc1_fuel_rate = msg.aftertreatment_1_fuel_rate;
                self.aftertreatment.at1fc1_regen_status = msg.aftrtrtmnt_1_rgnrtn_stts;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1fc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1FC2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1fc2_fuel_pressure_2 = msg.aftertreatment_1_fuel_pressure_2;
                self.aftertreatment.at1fc2_fuel_pressure_2_control = msg.aftrtrtmnt_1_fl_prssr_2_cntrl;
                self.aftertreatment.at1fc2_hc_doser_intake_fuel_temp = msg.aftrtrtmnt_1_hdrrn_dsr_intk_fl_tmprtr;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at1ac1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT1AC1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at1ac1_supply_air_pressure = msg.aftrtrtmnt_1_sppl_ar_prssr;
                self.aftertreatment.at1ac1_purge_air_pressure = msg.aftertreatment_1_purge_air_pressure;
                self.aftertreatment.at1ac1_air_pressure_control = msg.aftrtrtmnt_1_ar_prssr_cntrl;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a1doc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A1DOC1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a1doc1_intake_temp = msg.atttt_1_ds_oxdt_ctst_it_tpt;
                self.aftertreatment.a1doc1_outlet_temp = msg.atttt_1_ds_oxdt_ctst_ott_tpt;
                self.aftertreatment.a1doc1_delta_pressure = msg.atttt_1_ds_oxdt_ctst_dt_pss;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a1doc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A1DOC2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a1doc2_intake_pressure = msg.atttt_1_ds_oxdt_ctst_it_pss;
                self.aftertreatment.a1doc2_outlet_pressure = msg.atttt_1_ds_oxdt_ctst_ott_pss;
                self.aftertreatment.a1doc2_intake_to_dpf_outlet_delta = msg.atttt_1_d_it_t_dp_ott_dt_pss;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a1scrai(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A1SCRAI::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a1scrai_outlet_nh3 = msg.aftertreatment_1_outlet_nh_3;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a1scrsi1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A1SCRSI1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a1scrsi1_def_avg_consumption = msg.atttt_1_ds_exst_fd_av_cspt;
                self.aftertreatment.a1scrsi1_scr_commanded_def_consumption = msg.atttt_1_s_cdd_ds_exst_fd_cspt;
                self.aftertreatment.a1scrsi1_scr_conversion_efficiency = msg.aftrtrtmnt_1_sr_cnvrsn_effn;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a1scrsi2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A1SCRSI2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a1scrsi2_total_def_used = msg.aftrtrtmnt_1_ttl_dsl_exhst_fld_usd;
                self.aftertreatment.a1scrsi2_trip_def_used = msg.aftrtrtmnt_trp_dsl_exhst_fld;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_dpf1s(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DPF1S::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.dpf1s_soot_mass = msg.aftrtrtmnt_1_dsl_prtlt_fltr_st_mss;
                self.aftertreatment.dpf1s_soot_density = msg.aftrtrtmnt_1_dsl_prtlt_fltr_st_dnst;
                self.aftertreatment.dpf1s_mean_soot_signal = msg.aftrtrtmnt_1_dsl_prtlt_fltr_mn_st_sgnl;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_dpf1s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DPF1S2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.dpf1s2_soot_signal_std_dev = msg.atttt_1_ds_ptt_ft_st_s_stdd_dvt;
                self.aftertreatment.dpf1s2_soot_signal_max = msg.atttt_1_ds_ptt_ft_st_s_mx;
                self.aftertreatment.dpf1s2_soot_signal_min = msg.atttt_1_ds_ptt_ft_st_sm;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_dpfc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DPFC1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.dpfc1_dpf_lamp_command = msg.dsl_prtlt_fltr_lmp_cmmnd;
                self.aftertreatment.dpfc1_dpf_active_regen_status = msg.atttt_ds_ptt_ft_atv_rt_stts;
                self.aftertreatment.dpfc1_dpf_passive_regen_status = msg.atttt_ds_ptt_ft_pssv_rt_stts;
                self.aftertreatment.dpfc1_dpf_status = msg.aftrtrtmnt_dsl_prtlt_fltr_stts;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_dpfc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DPFC2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.dpfc2_dpf_intake_temp_setpoint = msg.atttt_1_ds_ptt_ft_it_tpt_st_pt;
                self.aftertreatment.dpfc2_engine_unburned_fuel_pct = msg.engine_unburned_fuel_percentage;
                self.aftertreatment.dpfc2_at1_fuel_mass_rate = msg.aftertreatment_1_fuel_mass_rate;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    // ============================================================================
    // Batch 11: Aftertreatment Bank 2 + EGR Handlers
    // ============================================================================

    pub(crate) fn handle_at2s1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT2S1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at2s1_dpf_soot_load_percent = msg.aftrtrtmnt_2_dsl_prtlt_fltr_st_ld_prnt;
                self.aftertreatment.at2s1_dpf_ash_load_percent = msg.atttt_2_ds_ptt_ft_as_ld_pt;
                self.aftertreatment.at2s1_dpf_time_since_last_regen = msg.atttt_2_ds_ptt_ft_ts_lst_atv_rt;
                self.aftertreatment.at2s1_dpf_soot_load_regen_threshold = msg.atttt_2_ds_ptt_ft_st_ld_rt_tsd;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at2s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT2S2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at2s2_dpf_time_to_next_regen = msg.atttt_2_ds_ptt_ft_tt_nxt_atv_rt;
                self.aftertreatment.at2s2_scr_time_since_last_clean = msg.atttt_2_s_sst_ts_lst_sst_c_evt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at2og1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT2OG1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at2og1_outlet_nox = msg.aftertreatment_2_outlet_nox_1;
                self.aftertreatment.at2og1_outlet_percent_oxygen = msg.aftrtrtmnt_2_otlt_prnt_oxgn_1;
                self.aftertreatment.at2og1_outlet_sensor_power_in_range = msg.aftrtrtmnt_2_otlt_gs_snsr_1_pwr_in_rng;
                self.aftertreatment.at2og1_outlet_sensor_at_temp = msg.aftrtrtmnt_2_otlt_gs_snsr_1_at_tmprtr;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at2ig1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT2IG1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at2ig1_inlet_nox = msg.engine_exhaust_2_nox_1;
                self.aftertreatment.at2ig1_inlet_percent_oxygen = msg.engine_exhaust_2_percent_oxygen_1;
                self.aftertreatment.at2ig1_inlet_sensor_power_in_range = msg.engn_exhst_2_gs_snsr_1_pwr_in_rng;
                self.aftertreatment.at2ig1_inlet_sensor_at_temp = msg.engn_exhst_2_gs_snsr_1_at_tmprtr;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at2hi1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT2HI1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at2hi1_total_fuel_used = msg.aftertreatment_2_total_fuel_used;
                self.aftertreatment.at2hi1_total_regen_time = msg.aftrtrtmnt_2_ttl_rgnrtn_tm;
                self.aftertreatment.at2hi1_total_disabled_time = msg.aftrtrtmnt_2_ttl_dsld_tm;
                self.aftertreatment.at2hi1_total_active_regens = msg.aftrtrtmnt_2_ttl_nmr_of_atv_rgnrtns;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at2gp(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT2GP::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at2gp_dpf_intake_pressure = msg.atttt_2_ds_ptt_ft_it_pss;
                self.aftertreatment.at2gp_dpf_outlet_pressure = msg.atttt_2_ds_ptt_ft_ott_pss;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at2fc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT2FC1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at2fc1_fuel_pressure = msg.aftertreatment_2_fuel_pressure_1;
                self.aftertreatment.at2fc1_fuel_rate = msg.aftertreatment_2_fuel_rate;
                self.aftertreatment.at2fc1_fuel_pressure_control = msg.aftrtrtmnt_2_fl_prssr_1_cntrl;
                self.aftertreatment.at2fc1_regen_status = msg.aftrtrtmnt_2_rgnrtn_stts;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_at2ac1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match AT2AC1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.at2ac1_supply_air_pressure = msg.aftrtrtmnt_2_sppl_ar_prssr;
                self.aftertreatment.at2ac1_purge_air_pressure = msg.aftertreatment_2_purge_air_pressure;
                self.aftertreatment.at2ac1_air_pressure_control = msg.aftrtrtmnt_2_ar_prssr_cntrl;
                self.aftertreatment.at2ac1_air_pressure_actuator_position = msg.aftrtrtmnt_2_ar_prssr_attr_pstn;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a2doc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A2DOC1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a2doc1_inlet_temp = msg.atttt_2_ds_oxdt_ctst_it_tpt;
                self.aftertreatment.a2doc1_outlet_temp = msg.atttt_2_ds_oxdt_ctst_ott_tpt;
                self.aftertreatment.a2doc1_diff_pressure = msg.atttt_2_ds_oxdt_ctst_dt_pss;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a2scrai(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A2SCRAI::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a2scrai_outlet_nh3 = msg.aftertreatment_2_outlet_nh_3;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a2scrsi1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A2SCRSI1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a2scrsi1_def_avg_consumption = msg.atttt_2_ds_exst_fd_av_cspt;
                self.aftertreatment.a2scrsi1_scr_commanded_consumption = msg.atttt_2_s_cdd_ds_exst_fd_cspt;
                self.aftertreatment.a2scrsi1_scr_conversion_efficiency = msg.aftrtrtmnt_2_sr_cnvrsn_effn;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a1scrdsi1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A1SCRDSI1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a1scrdsi1_dosing_rate = msg.atttt_1_ds_exst_fd_at_ds_qtt;
                self.aftertreatment.a1scrdsi1_scr_system_1_state = msg.aftertreatment_1_scr_system_1_state;
                self.aftertreatment.a1scrdsi1_doser_1_abs_pressure = msg.atttt_1_ds_exst_fd_ds_1_ast_pss;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a1scrdsi2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A1SCRDSI2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a1scrdsi2_air_assist_pressure = msg.atttt_1_s_ds_a_assst_ast_pss;
                self.aftertreatment.a1scrdsi2_air_assist_valve = msg.aftrtrtmnt_1_sr_dsng_ar_assst_vlv;
                self.aftertreatment.a1scrdsi2_doser_1_temp = msg.atttt_1_ds_exst_fd_ds_1_tpt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a1scrdsi3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A1SCRDSI3::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a1scrdsi3_doser_1_pressure = msg.aftrtrtmnt_1_dsl_exhst_fld_dsr_1_prssr;
                self.aftertreatment.a1scrdsi3_doser_2_abs_pressure = msg.atttt_1_ds_exst_fd_ds_2_ast_pss;
                self.aftertreatment.a1scrdsi3_doser_2_temp = msg.atttt_1_ds_exst_fd_ds_2_tpt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a2scrdsi1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A2SCRDSI1::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a2scrdsi1_dosing_rate = msg.atttt_2_ds_exst_fd_at_ds_qtt;
                self.aftertreatment.a2scrdsi1_scr_system_1_state = msg.aftertreatment_2_scr_system_1_state;
                self.aftertreatment.a2scrdsi1_doser_1_abs_pressure = msg.atttt_2_ds_exst_fd_ds_1_ast_pss;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a2scrdsi2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A2SCRDSI2::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a2scrdsi2_air_assist_pressure = msg.atttt_2_s_ds_a_assst_ast_pss;
                self.aftertreatment.a2scrdsi2_air_assist_valve = msg.aftrtrtmnt_2_sr_dsng_ar_assst_vlv;
                self.aftertreatment.a2scrdsi2_doser_1_temp = msg.atttt_2_ds_exst_fd_ds_1_tpt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_a2scrdsi3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match A2SCRDSI3::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.a2scrdsi3_doser_1_pressure = msg.aftrtrtmnt_2_dsl_exhst_fld_dsr_1_prssr;
                self.aftertreatment.a2scrdsi3_doser_2_abs_pressure = msg.atttt_2_ds_exst_fd_ds_2_ast_pss;
                self.aftertreatment.a2scrdsi3_doser_2_temp = msg.atttt_2_ds_exst_fd_ds_2_tpt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_eegr1a(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEGR1A::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.eegr1a_actuator_1_desired_position = msg.engn_exhst_gs_rrltn_1_attr_1_dsrd_pstn;
                self.aftertreatment.eegr1a_actuator_1_temp = msg.engn_exhst_gs_rrltn_1_attr_1_tmprtr;
                self.aftertreatment.eegr1a_actuator_2_desired_position = msg.engn_exhst_gs_rrltn_1_attr_2_dsrd_pstn;
                self.aftertreatment.eegr1a_actuator_2_temp = msg.engn_exhst_gs_rrltn_1_attr_2_tmprtr;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_eegr2a(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EEGR2A::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.eegr2a_actuator_1_desired_position = msg.engn_exhst_gs_rrltn_2_attr_1_dsrd_pstn;
                self.aftertreatment.eegr2a_actuator_1_temp = msg.engn_exhst_gs_rrltn_2_attr_1_tmprtr;
                self.aftertreatment.eegr2a_actuator_2_desired_position = msg.engn_exhst_gs_rrltn_2_attr_2_dsrd_pstn;
                self.aftertreatment.eegr2a_actuator_2_temp = msg.engn_exhst_gs_rrltn_2_attr_2_tmprtr;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_dpf2s(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DPF2S::decode(can_id, data) {
            Ok(msg) => {
                self.aftertreatment.dpf2s_soot_mass = msg.aftrtrtmnt_2_dsl_prtlt_fltr_st_mss;
                self.aftertreatment.dpf2s_soot_density = msg.aftrtrtmnt_2_dsl_prtlt_fltr_st_dnst;
                self.aftertreatment.dpf2s_mean_soot_signal = msg.aftrtrtmnt_2_dsl_prtlt_fltr_mn_st_sgnl;
                self.aftertreatment.dpf2s_median_soot_signal = msg.atttt_2_ds_ptt_ft_md_st_s;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }
}
