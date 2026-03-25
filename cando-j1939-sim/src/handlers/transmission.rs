use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle ETC9 - Electronic Transmission Controller 9 (Dual Clutch Transmission)
    pub(crate) fn handle_etc9(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC9 - Electronic Transmission Controller 9 (Dual Clutch Transmission)
        match ETC9::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc9_current_preselection_gear = msg.dl_clth_trnsmssn_crrnt_pr_sltn_gr;
                self.transmission.etc9_input_shaft1_speed = msg.dl_clth_trnsmssn_inpt_shft_1_spd;
                self.transmission.etc9_input_shaft2_speed = msg.dl_clth_trnsmssn_inpt_shft_2_spd;
                self.transmission.etc9_selected_preselection_gear = msg.dl_clth_trnsmssn_sltd_pr_sltn_gr;
                println!(
                    "🔧 Received ETC9: Current gear = {}, Shaft1 = {:.1} rpm, Shaft2 = {:.1} rpm, Selected gear = {}",
                    self.transmission.etc9_current_preselection_gear,
                    self.transmission.etc9_input_shaft1_speed,
                    self.transmission.etc9_input_shaft2_speed,
                    self.transmission.etc9_selected_preselection_gear
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC9: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC5 - Transmission Control Status
    pub(crate) fn handle_etc5(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC5 - Transmission Control Status
        match ETC5::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc5_trnsmssn_hgh_rng_sns_swth = msg.trnsmssn_hgh_rng_sns_swth;
                self.transmission.etc5_transmission_low_range_sense_switch =
                    msg.transmission_low_range_sense_switch;
                self.transmission.etc5_transmission_splitter_position =
                    msg.transmission_splitter_position;
                self.transmission.etc5_trnsmssn_rvrs_drtn_swth = msg.trnsmssn_rvrs_drtn_swth;
                self.transmission.etc5_transmission_neutral_switch = msg.transmission_neutral_switch;
                self.transmission.etc5_trnsmssn_frwrd_drtn_swth = msg.trnsmssn_frwrd_drtn_swth;
                println!(
                    "🔧 Received ETC5: Transmission = [High:{}, Low:{}, Split:{}, Rev:{}, Neu:{}, Fwd:{}]",
                    self.transmission.etc5_trnsmssn_hgh_rng_sns_swth,
                    self.transmission.etc5_transmission_low_range_sense_switch,
                    self.transmission.etc5_transmission_splitter_position,
                    self.transmission.etc5_trnsmssn_rvrs_drtn_swth,
                    self.transmission.etc5_transmission_neutral_switch,
                    self.transmission.etc5_trnsmssn_frwrd_drtn_swth
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC5: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC6 - Electronic Transmission Controller 6
    pub(crate) fn handle_etc6(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC6 - Electronic Transmission Controller 6
        match ETC6::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc6_recommended_gear = msg.recommended_gear;
                self.transmission.etc6_lowest_possible_gear = msg.lowest_possible_gear;
                self.transmission.etc6_highest_possible_gear = msg.highest_possible_gear;
                self.transmission.etc6_clutch_life_remaining = msg.clutch_life_remaining;
                println!(
                    "🔧 Received ETC6: Recommended gear = {:.0}, Lowest gear = {:.0}, Highest gear = {:.0}, Clutch life = {:.1}%",
                    self.transmission.etc6_recommended_gear,
                    self.transmission.etc6_lowest_possible_gear,
                    self.transmission.etc6_highest_possible_gear,
                    self.transmission.etc6_clutch_life_remaining
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC6: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC2 - Electronic Transmission Controller 2
    pub(crate) fn handle_etc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC2 - Electronic Transmission Controller 2
        match ETC2::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc2_transmission_selected_gear = msg.transmission_selected_gear;
                self.transmission.etc2_transmission_actual_gear_ratio =
                    msg.transmission_actual_gear_ratio;
                self.transmission.etc2_transmission_current_gear = msg.transmission_current_gear;
                self.transmission.etc2_transmission_requested_range = msg.transmission_requested_range;
                self.transmission.etc2_transmission_current_range = msg.transmission_current_range;
                println!(
                    "🔧 Received ETC2: Selected gear = {:.0}, Gear ratio = {:.2}, Current gear = {:.0}, Requested range = {}, Current range = {}",
                    self.transmission.etc2_transmission_selected_gear,
                    self.transmission.etc2_transmission_actual_gear_ratio,
                    self.transmission.etc2_transmission_current_gear,
                    self.transmission.etc2_transmission_requested_range,
                    self.transmission.etc2_transmission_current_range
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETCC1 - Engine Turbocharger Control 1
    pub(crate) fn handle_etcc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETCC1 - Engine Turbocharger Control 1
        match ETCC1::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etcc1_engn_trhrgr_wstgt_attr_1_cmmnd =
                    msg.engn_trhrgr_wstgt_attr_1_cmmnd;
                self.transmission.etcc1_engn_trhrgr_wstgt_attr_2_cmmnd =
                    msg.engn_trhrgr_wstgt_attr_2_cmmnd;
                self.transmission.etcc1_e_exst_b_1_pss_rt_ct_cd = msg.e_exst_b_1_pss_rt_ct_cd;
                self.transmission.etcc1_et_cpss_bw_att_1_cd = msg.et_cpss_bw_att_1_cd;
                println!(
                    "🔥 Received ETCC1: Wastegate1={:.1}%, Wastegate2={:.1}%, Bank1={:.1}%, Blowoff={:.1}%",
                    self.transmission.etcc1_engn_trhrgr_wstgt_attr_1_cmmnd,
                    self.transmission.etcc1_engn_trhrgr_wstgt_attr_2_cmmnd,
                    self.transmission.etcc1_e_exst_b_1_pss_rt_ct_cd,
                    self.transmission.etcc1_et_cpss_bw_att_1_cd
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETCC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETCC2 - Engine Turbocharger Control 2
    pub(crate) fn handle_etcc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETCC2 - Engine Turbocharger Control 2
        match ETCC2::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etcc2_engn_stgd_trhrgr_slnd_stts = msg.engn_stgd_trhrgr_slnd_stts;
                self.transmission.etcc2_nmr_of_engn_trhrgrs_cmmndd = msg.nmr_of_engn_trhrgrs_cmmndd;
                println!(
                    "🔥 Received ETCC2: Turbo status = {:.1}%, Number commanded = {}",
                    self.transmission.etcc2_engn_stgd_trhrgr_slnd_stts,
                    self.transmission.etcc2_nmr_of_engn_trhrgrs_cmmndd
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETCC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC3 - Electronic Transmission Controller 3 (Shift Finger & Actuators)
    pub(crate) fn handle_etc3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC3 - Electronic Transmission Controller 3 (Shift Finger & Actuators)
        match ETC3::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc3_shift_finger_gear_position = msg.trnsmssn_shft_fngr_gr_pstn;
                self.transmission.etc3_shift_finger_rail_position = msg.trnsmssn_shft_fngr_rl_pstn;
                self.transmission.etc3_shift_finger_neutral_indicator = msg.trnsmssn_shft_fngr_ntrl_indtr;
                self.transmission.etc3_shift_finger_engagement_indicator = msg.trnsmssn_shft_fngr_enggmnt_indtr;
                self.transmission.etc3_shift_finger_center_rail_indicator = msg.trnsmssn_shft_fngr_cntr_rl_indtr;
                self.transmission.etc3_shift_finger_rail_actuator_1 = msg.trnsmssn_shft_fngr_rl_attr_1;
                self.transmission.etc3_shift_finger_gear_actuator_1 = msg.trnsmssn_shft_fngr_gr_attr_1;
                self.transmission.etc3_shift_finger_rail_actuator_2 = msg.trnsmssn_shft_fngr_rl_attr_2;
                self.transmission.etc3_shift_finger_gear_actuator_2 = msg.trnsmssn_shft_fngr_gr_attr_2;
                self.transmission.etc3_range_high_actuator = msg.transmission_range_high_actuator;
                self.transmission.etc3_range_low_actuator = msg.transmission_range_low_actuator;
                self.transmission.etc3_splitter_direct_actuator = msg.trnsmssn_splttr_drt_attr;
                self.transmission.etc3_splitter_indirect_actuator = msg.trnsmssn_splttr_indrt_attr;
                self.transmission.etc3_clutch_actuator = msg.transmission_clutch_actuator;
                self.transmission.etc3_torque_converter_lockup_clutch_actuator = msg.trnsmssn_trq_cnvrtr_lkp_clth_attr;
                self.transmission.etc3_defuel_actuator = msg.transmission_defuel_actuator;
                self.transmission.etc3_inertia_brake_actuator = msg.trnsmssn_inrt_brk_attr;
                println!(
                    "🔧 Received ETC3: Gear pos = {:.1}%, Rail pos = {:.1}%, Neutral = {}, Engaged = {}",
                    self.transmission.etc3_shift_finger_gear_position,
                    self.transmission.etc3_shift_finger_rail_position,
                    self.transmission.etc3_shift_finger_neutral_indicator,
                    self.transmission.etc3_shift_finger_engagement_indicator
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC4 - Electronic Transmission Controller 4 (Synchronizer)
    pub(crate) fn handle_etc4(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC4 - Electronic Transmission Controller 4 (Synchronizer)
        match ETC4::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc4_synchronizer_clutch_value = msg.trnsmssn_snhrnzr_clth_vl;
                self.transmission.etc4_synchronizer_brake_value = msg.trnsmssn_snhrnzr_brk_vl;
                println!(
                    "🔧 Received ETC4: Synchro clutch = {:.1}%, Synchro brake = {:.1}%",
                    self.transmission.etc4_synchronizer_clutch_value,
                    self.transmission.etc4_synchronizer_brake_value
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC4: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC7 - Electronic Transmission Controller 7 (Display & Indicators)
    pub(crate) fn handle_etc7(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC7 - Electronic Transmission Controller 7 (Display & Indicators)
        match ETC7::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc7_current_range_display_blank_state = msg.trnsmssn_crrnt_rng_dspl_blnk_stt;
                self.transmission.etc7_service_indicator = msg.transmission_service_indicator;
                self.transmission.etc7_requested_range_display_blank_state = msg.trnsmssn_rqstd_rng_dspl_blnk_stt;
                self.transmission.etc7_requested_range_display_flash_state = msg.trnsmssn_rqstd_rng_dspl_flsh_stt;
                self.transmission.etc7_ready_for_brake_release = msg.trnsmssn_rd_fr_brk_rls;
                self.transmission.etc7_active_shift_console_indicator = msg.active_shift_console_indicator;
                self.transmission.etc7_engine_crank_enable = msg.transmission_engine_crank_enable;
                self.transmission.etc7_shift_inhibit_indicator = msg.trnsmssn_shft_inht_indtr;
                self.transmission.etc7_mode_1_indicator = msg.transmission_mode_1_indicator;
                self.transmission.etc7_mode_2_indicator = msg.transmission_mode_2_indicator;
                self.transmission.etc7_mode_3_indicator = msg.transmission_mode_3_indicator;
                self.transmission.etc7_mode_4_indicator = msg.transmission_mode_4_indicator;
                self.transmission.etc7_mode_5_indicator = msg.transmission_mode_5_indicator;
                self.transmission.etc7_mode_6_indicator = msg.transmission_mode_6_indicator;
                self.transmission.etc7_mode_7_indicator = msg.transmission_mode_7_indicator;
                self.transmission.etc7_mode_8_indicator = msg.transmission_mode_8_indicator;
                self.transmission.etc7_requested_gear_feedback = msg.trnsmssn_rqstd_gr_fdk;
                self.transmission.etc7_reverse_gear_shift_inhibit_status = msg.trnsmssn_rvrs_gr_shft_inht_stts;
                self.transmission.etc7_warning_indicator = msg.transmission_warning_indicator;
                self.transmission.etc7_mode_9_indicator = msg.transmission_mode_9_indicator;
                self.transmission.etc7_mode_10_indicator = msg.transmission_mode_10_indicator;
                self.transmission.etc7_air_supply_pressure_indicator = msg.trnsmssn_ar_sppl_prssr_indtr;
                self.transmission.etc7_auto_neutral_manual_return_state = msg.trnsmssn_at_ntrl_mnl_rtrn_stt;
                self.transmission.etc7_manual_mode_indicator = msg.transmission_manual_mode_indicator;
                self.transmission.etc7_load_reduction_indicator = msg.trnsmssn_ld_rdtn_indtr;
                self.transmission.etc7_pre_defined_range_limit_indicator = msg.trnsmssn_pr_dfnd_rng_lmt_indtr;
                self.transmission.etc7_coast_mode_indicator = msg.transmission_coast_mode_indicator;
                self.transmission.etc7_output_shaft_brake_indicator = msg.trnsmssn_otpt_shft_brk_indtr;
                println!(
                    "🔧 Received ETC7: Service = {}, Brake release = {}, Crank enable = {}, Warning = {}",
                    self.transmission.etc7_service_indicator,
                    self.transmission.etc7_ready_for_brake_release,
                    self.transmission.etc7_engine_crank_enable,
                    self.transmission.etc7_warning_indicator
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC7: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC8 - Electronic Transmission Controller 8 (Torque Converter)
    pub(crate) fn handle_etc8(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC8 - Electronic Transmission Controller 8 (Torque Converter)
        match ETC8::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc8_torque_converter_ratio = msg.trnsmssn_trq_cnvrtr_rt;
                self.transmission.etc8_clutch_converter_input_speed = msg.trnsmssn_clth_cnvrtr_inpt_spd;
                self.transmission.etc8_shift_inhibit_reason = msg.transmission_shift_inhibit_reason;
                self.transmission.etc8_torque_converter_lockup_inhibit_reason = msg.trnsmssn_trq_cnvrtr_lkp_inht_rsn;
                self.transmission.etc8_torque_converter_lockup_inhibit_indicator = msg.trnsmssn_trq_cnvrtr_lkp_inht_indtr;
                self.transmission.etc8_explicit_manual_mode_indicator = msg.trnsmssn_explt_mnl_md_indtr;
                println!(
                    "🔧 Received ETC8: TC ratio = {:.3}, Input speed = {:.1} rpm, Shift inhibit = {}",
                    self.transmission.etc8_torque_converter_ratio,
                    self.transmission.etc8_clutch_converter_input_speed,
                    self.transmission.etc8_shift_inhibit_reason
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC8: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC10 - Electronic Transmission Controller 10 (Clutch & Actuator Positions)
    pub(crate) fn handle_etc10(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC10 - Electronic Transmission Controller 10 (Clutch & Actuator Positions)
        match ETC10::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc10_clutch_1_actuator_position = msg.trnsmssn_clth_1_attr_pstn;
                self.transmission.etc10_clutch_2_actuator_position = msg.trnsmssn_clth_2_attr_pstn;
                self.transmission.etc10_hydraulic_pump_actuator_1_position = msg.trnsmssn_hdrl_pmp_attr_1_pstn;
                self.transmission.etc10_shift_actuator_1_position = msg.trnsmssn_1_shft_attr_1_pstn;
                self.transmission.etc10_shift_actuator_2_position = msg.trnsmssn_1_shft_attr_2_pstn;
                self.transmission.etc10_clutch_1_cooling_actuator_status = msg.trnsmssn_clth_1_clng_attr_stts;
                self.transmission.etc10_clutch_2_cooling_actuator_status = msg.trnsmssn_clth_2_clng_attr_stts;
                self.transmission.etc10_shift_rail_1_actuator_status = msg.trnsmssn_shft_rl_1_attr_stts;
                self.transmission.etc10_shift_rail_2_actuator_status = msg.trnsmssn_shft_rl_2_attr_stts;
                self.transmission.etc10_shift_rail_3_actuator_status = msg.trnsmssn_shft_rl_3_attr_stts;
                self.transmission.etc10_shift_rail_4_actuator_status = msg.trnsmssn_shft_rl_4_attr_stts;
                self.transmission.etc10_shift_rail_5_actuator_status = msg.trnsmssn_shft_rl_5_attr_stts;
                self.transmission.etc10_shift_rail_6_actuator_status = msg.trnsmssn_shft_rl_6_attr_stts;
                self.transmission.etc10_hydraulic_pump_actuator_2_percent = msg.trnsmssn_hdrl_pmp_attr_2_prnt;
                println!(
                    "🔧 Received ETC10: Clutch1 = {:.1}%, Clutch2 = {:.1}%, Pump1 = {:.1}%",
                    self.transmission.etc10_clutch_1_actuator_position,
                    self.transmission.etc10_clutch_2_actuator_position,
                    self.transmission.etc10_hydraulic_pump_actuator_1_position
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC10: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC11 - Electronic Transmission Controller 11 (Shift Rail Positions)
    pub(crate) fn handle_etc11(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC11 - Electronic Transmission Controller 11 (Shift Rail Positions)
        match ETC11::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc11_shift_rail_1_position = msg.transmission_shift_rail_1_position;
                self.transmission.etc11_shift_rail_2_position = msg.transmission_shift_rail_2_position;
                self.transmission.etc11_shift_rail_3_position = msg.transmission_shift_rail_3_position;
                self.transmission.etc11_shift_rail_4_position = msg.transmission_shift_rail_4_position;
                self.transmission.etc11_shift_rail_5_position = msg.transmission_shift_rail_5_position;
                self.transmission.etc11_shift_rail_6_position = msg.transmission_shift_rail_6_position;
                println!(
                    "🔧 Received ETC11: Rails = [{:.1}, {:.1}, {:.1}, {:.1}, {:.1}, {:.1}]%",
                    self.transmission.etc11_shift_rail_1_position,
                    self.transmission.etc11_shift_rail_2_position,
                    self.transmission.etc11_shift_rail_3_position,
                    self.transmission.etc11_shift_rail_4_position,
                    self.transmission.etc11_shift_rail_5_position,
                    self.transmission.etc11_shift_rail_6_position
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC11: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC12 - Electronic Transmission Controller 12 (Hydrostatic Loop)
    pub(crate) fn handle_etc12(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC12 - Electronic Transmission Controller 12 (Hydrostatic Loop)
        match ETC12::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc12_hydrostatic_loop_1_pressure = msg.trnsmssn_hdrstt_lp_1_prssr;
                self.transmission.etc12_hydrostatic_loop_2_pressure = msg.trnsmssn_hdrstt_lp_2_prssr;
                self.transmission.etc12_directional_output_shaft_speed = msg.trnsmssn_drtnl_otpt_shft_spd;
                self.transmission.etc12_intermediate_shaft_speed = msg.trnsmssn_intrmdt_shft_spd;
                println!(
                    "🔧 Received ETC12: Loop1 = {} kPa, Loop2 = {} kPa, Output = {:.1} rpm",
                    self.transmission.etc12_hydrostatic_loop_1_pressure,
                    self.transmission.etc12_hydrostatic_loop_2_pressure,
                    self.transmission.etc12_directional_output_shaft_speed
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC12: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC13 - Electronic Transmission Controller 13 (Max Speeds & Mode Indicators)
    pub(crate) fn handle_etc13(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC13 - Electronic Transmission Controller 13 (Max Speeds & Mode Indicators)
        match ETC13::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc13_max_forward_output_shaft_speed = msg.mxmm_frwrd_trnsmssn_otpt_shft_spd;
                self.transmission.etc13_max_reverse_output_shaft_speed = msg.mxmm_rvrs_trnsmssn_otpt_shft_spd;
                self.transmission.etc13_source_address_requested_gear = msg.s_addss_o_atv_o_pd_tsss_rqstd_g;
                self.transmission.etc13_mode_11_indicator = msg.transmission_mode_11_indicator;
                self.transmission.etc13_mode_12_indicator = msg.transmission_mode_12_indicator;
                self.transmission.etc13_mode_13_indicator = msg.transmission_mode_13_indicator;
                self.transmission.etc13_mode_14_indicator = msg.transmission_mode_14_indicator;
                self.transmission.etc13_mode_15_indicator = msg.transmission_mode_15_indicator;
                self.transmission.etc13_mode_16_indicator = msg.transmission_mode_16_indicator;
                self.transmission.etc13_mode_17_indicator = msg.transmission_mode_17_indicator;
                self.transmission.etc13_mode_18_indicator = msg.transmission_mode_18_indicator;
                self.transmission.etc13_mode_19_indicator = msg.transmission_mode_19_indicator;
                self.transmission.etc13_mode_20_indicator = msg.transmission_mode_20_indicator;
                println!(
                    "🔧 Received ETC13: Max fwd = {:.1} rpm, Max rev = {:.1} rpm",
                    self.transmission.etc13_max_forward_output_shaft_speed,
                    self.transmission.etc13_max_reverse_output_shaft_speed
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC13: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC14 - Electronic Transmission Controller 14 (Clutch Temp & Capability)
    pub(crate) fn handle_etc14(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC14 - Electronic Transmission Controller 14 (Clutch Temp & Capability)
        match ETC14::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc14_clutch_1_temperature = msg.transmission_clutch_1_temperature;
                self.transmission.etc14_clutch_1_overheat_indicator = msg.trnsmssn_clth_1_ovrht_indtr;
                self.transmission.etc14_launch_capability = msg.transmission_launch_capability;
                self.transmission.etc14_gear_shift_capability = msg.transmission_gear_shift_capability;
                self.transmission.etc14_damage_threshold_status = msg.trnsmssn_dmg_thrshld_stts;
                println!(
                    "🔧 Received ETC14: Clutch temp = {:.1}°C, Overheat = {}, Launch cap = {}, Shift cap = {}",
                    self.transmission.etc14_clutch_1_temperature,
                    self.transmission.etc14_clutch_1_overheat_indicator,
                    self.transmission.etc14_launch_capability,
                    self.transmission.etc14_gear_shift_capability
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC14: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETC15 - Electronic Transmission Controller 15 (CRC, Counter, Auto-Neutral)
    pub(crate) fn handle_etc15(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETC15 - Electronic Transmission Controller 15 (CRC, Counter, Auto-Neutral)
        match ETC15::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etc15_crc = msg.eltrn_trnsmssn_cntrl_15_cr;
                self.transmission.etc15_counter = msg.eltrn_trnsmssn_cntrl_15_cntr;
                self.transmission.etc15_auto_neutral_auto_return_request_feedback = msg.trnsmssn_at_ntrl_at_rtrn_rqst_fdk;
                self.transmission.etc15_launch_process_status = msg.launch_process_status;
                self.transmission.etc15_auto_neutral_auto_return_function_state = msg.trnsmssn_at_ntrl_at_rtrn_fntn_stt;
                println!(
                    "🔧 Received ETC15: CRC = {}, Counter = {}, Launch status = {}",
                    self.transmission.etc15_crc,
                    self.transmission.etc15_counter,
                    self.transmission.etc15_launch_process_status
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETC15: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETCC4 - Engine Turbocharger Control 4
    pub(crate) fn handle_etcc4(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETCC4 - Engine Turbocharger Control 4
        match ETCC4::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etcc4_wastegate_actuator_3_command = msg.engn_trhrgr_wstgt_attr_3_cmmnd;
                self.transmission.etcc4_wastegate_actuator_4_command = msg.engn_trhrgr_wstgt_attr_4_cmmnd;
                println!(
                    "🔧 Received ETCC4: Wastegate3 = {:.1}%, Wastegate4 = {:.1}%",
                    self.transmission.etcc4_wastegate_actuator_3_command,
                    self.transmission.etcc4_wastegate_actuator_4_command
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETCC4: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETCBI - Engine Turbocharger Compressor Bypass Information
    pub(crate) fn handle_etcbi(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETCBI - Engine Turbocharger Compressor Bypass Information
        match ETCBI::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.etcbi_compressor_bypass_actuator_2_position = msg.engn_trhrgr_cmprssr_bpss_attr_2_pstn;
                self.transmission.etcbi_compressor_bypass_actuator_2_desired_position = msg.et_cpss_bpss_att_2_dsd_pst;
                self.transmission.etcbi_compressor_bypass_actuator_2_preliminary_fmi = msg.et_cpss_bpss_att_2_pf;
                self.transmission.etcbi_compressor_bypass_actuator_2_temp_status = msg.et_cpss_bpss_att_2_tpt_stts;
                self.transmission.etcbi_compressor_bypass_actuator_1_operation_status = msg.et_cpss_bpss_att_1_opt_stts;
                self.transmission.etcbi_compressor_bypass_actuator_2_operation_status = msg.et_cpss_bpss_att_2_opt_stts;
                self.transmission.etcbi_compressor_bypass_actuator_1_temperature = msg.et_cpss_bpss_att_1_tpt;
                self.transmission.etcbi_compressor_bypass_actuator_2_temperature = msg.et_cpss_bpss_att_2_tpt;
                println!(
                    "🔧 Received ETCBI: Bypass2 pos = {:.1}%, Bypass1 temp = {:.1}°C, Bypass2 temp = {:.1}°C",
                    self.transmission.etcbi_compressor_bypass_actuator_2_position,
                    self.transmission.etcbi_compressor_bypass_actuator_1_temperature,
                    self.transmission.etcbi_compressor_bypass_actuator_2_temperature
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETCBI: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle TC1 - Transmission Control 1 (Gear Shift Commands)
    pub(crate) fn handle_tc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // TC1 - Transmission Control 1 (Gear Shift Commands)
        match TC1::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.tc1_gear_shift_inhibit_request = msg.trnsmssn_gr_shft_inht_rqst;
                self.transmission.tc1_torque_converter_lockup_request = msg.trnsmssn_trq_cnvrtr_lkp_rqst;
                self.transmission.tc1_disengage_driveline_request = msg.disengage_driveline_request;
                self.transmission.tc1_reverse_gear_shift_inhibit_request = msg.trnsmssn_rvrs_gr_shft_inht_rqst;
                self.transmission.tc1_requested_percent_clutch_slip = msg.requested_percent_clutch_slip;
                self.transmission.tc1_transmission_requested_gear = msg.transmission_requested_gear;
                self.transmission.tc1_disengage_differential_lock_front_axle_1 = msg.dsngg_dffrntl_lk_rqst_frnt_axl_1;
                self.transmission.tc1_disengage_differential_lock_front_axle_2 = msg.dsngg_dffrntl_lk_rqst_frnt_axl_2;
                self.transmission.tc1_disengage_differential_lock_rear_axle_1 = msg.dsngg_dffrntl_lk_rqst_rr_axl_1;
                self.transmission.tc1_disengage_differential_lock_rear_axle_2 = msg.dsngg_dffrntl_lk_rqst_rr_axl_2;
                self.transmission.tc1_disengage_differential_lock_central = msg.dsngg_dffrntl_lk_rqst_cntrl;
                self.transmission.tc1_disengage_differential_lock_central_front = msg.dsngg_dffrntl_lk_rqst_cntrl_frnt;
                self.transmission.tc1_disengage_differential_lock_central_rear = msg.dsngg_dffrntl_lk_rqst_cntrl_rr;
                self.transmission.tc1_load_reduction_inhibit_request = msg.trnsmssn_ld_rdtn_inht_rqst;
                self.transmission.tc1_mode_1 = msg.transmission_mode_1;
                self.transmission.tc1_mode_2 = msg.transmission_mode_2;
                self.transmission.tc1_mode_3 = msg.transmission_mode_3;
                self.transmission.tc1_mode_4 = msg.transmission_mode_4;
                self.transmission.tc1_auto_neutral_manual_return_request = msg.trnsmssn_at_ntrl_mnl_rtrn_rqst;
                self.transmission.tc1_requested_launch_gear = msg.transmission_requested_launch_gear;
                self.transmission.tc1_shift_selector_display_mode_switch = msg.trnsmssn_shft_sltr_dspl_md_swth;
                self.transmission.tc1_mode_5 = msg.transmission_mode_5;
                self.transmission.tc1_mode_6 = msg.transmission_mode_6;
                self.transmission.tc1_mode_7 = msg.transmission_mode_7;
                self.transmission.tc1_mode_8 = msg.transmission_mode_8;

                // Physics: TC1 requested gear drives ETC7 feedback and ETC2 current gear
                self.transmission.etc7_requested_gear_feedback = msg.transmission_requested_gear;
                // Update ETC2 selected gear to match the request
                self.transmission.etc2_transmission_selected_gear = msg.transmission_requested_gear;

                println!(
                    "🔧 Received TC1: Requested gear = {:.0}, Clutch slip = {:.1}%, Shift inhibit = {}",
                    self.transmission.tc1_transmission_requested_gear,
                    self.transmission.tc1_requested_percent_clutch_slip,
                    self.transmission.tc1_gear_shift_inhibit_request
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode TC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle TC2 - Transmission Control 2 (Extended Mode Commands)
    pub(crate) fn handle_tc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // TC2 - Transmission Control 2 (Extended Mode Commands)
        match TC2::decode(can_id, data) {
            Ok(msg) => {
                self.transmission.tc2_mode_9 = msg.transmission_mode_9;
                self.transmission.tc2_mode_10 = msg.transmission_mode_10;
                self.transmission.tc2_pre_defined_max_gear_activation_request = msg.trnsmssn_pr_dfnd_mxmm_gr_atvtn_rqst;
                self.transmission.tc2_output_shaft_brake_request = msg.trnsmssn_otpt_shft_brk_rqst;
                self.transmission.tc2_requested_reverse_launch_gear = msg.trnsmssn_rqstd_rvrs_lnh_gr;
                self.transmission.tc2_selected_max_gear_limit_activation_request = msg.sltd_mxmm_gr_lmt_atvtn_rqst;
                self.transmission.tc2_mode_11 = msg.transmission_mode_11;
                self.transmission.tc2_mode_12 = msg.transmission_mode_12;
                self.transmission.tc2_mode_13 = msg.transmission_mode_13;
                self.transmission.tc2_mode_14 = msg.transmission_mode_14;
                self.transmission.tc2_mode_15 = msg.transmission_mode_15;
                self.transmission.tc2_mode_16 = msg.transmission_mode_16;
                self.transmission.tc2_mode_17 = msg.transmission_mode_17;
                self.transmission.tc2_mode_18 = msg.transmission_mode_18;
                self.transmission.tc2_mode_19 = msg.transmission_mode_19;
                self.transmission.tc2_mode_20 = msg.transmission_mode_20;
                self.transmission.tc2_disengage_differential_lock_rear_axle_3 = msg.dsngg_dffrntl_lk_rqst_rr_axl_3;
                self.transmission.tc2_coast_mode_disable_request = msg.trnsmssn_cst_md_dsl_rqst;
                self.transmission.tc2_explicit_manual_mode_request = msg.trnsmssn_explt_mnl_md_rqst;
                println!(
                    "🔧 Received TC2: Mode9={}, Mode10={}, Shaft brake={}, Manual mode={}",
                    self.transmission.tc2_mode_9,
                    self.transmission.tc2_mode_10,
                    self.transmission.tc2_output_shaft_brake_request,
                    self.transmission.tc2_explicit_manual_mode_request
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode TC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle ETCC3 - Electronic Transmission Controller Clutch 3: Engine Thermal Control
    pub(crate) fn handle_etcc3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ETCC3 - Electronic Transmission Controller Clutch 3: Engine Thermal Control
        match ETCC3::decode(can_id, data) {
            Ok(msg) => {
                self.thermal.etcc3_etc_bypass_actuator_1 = msg.et_cpss_bw_att_1_mt_ct_ds;
                self.thermal.etcc3_turbo_wastegate_actuator_1 =
                    msg.engn_trhrgr_wstgt_attr_1_mtr_crrnt_dsl;
                self.thermal.etcc3_cylinder_head_bypass_actuator =
                    msg.engn_clndr_hd_bpss_attr_1_mtr_crrnt_dsl;
                self.thermal.etcc3_throttle_valve_1 = msg.engn_thrttl_vlv_1_mtr_crrnt_dsl;
                self.thermal.etcc3_etc_bypass_pass_actuator_1 = msg.et_cpss_bpss_att_1_mt_ct_ds;
                self.thermal.etcc3_etc_bypass_pass_actuator_2 = msg.et_cpss_bpss_att_2_mt_ct_ds;
                self.thermal.etcc3_turbo_wastegate_actuator_2 =
                    msg.engn_trhrgr_wstgt_attr_2_mtr_crrnt_dsl;
                println!(
                    "🔥 Received ETCC3: Bypass1={}, Wastegate1={}, CylinderHead={}, Throttle={}, ETCPass1={}, ETCPass2={}, Wastegate2={}",
                    self.thermal.etcc3_etc_bypass_actuator_1,
                    self.thermal.etcc3_turbo_wastegate_actuator_1,
                    self.thermal.etcc3_cylinder_head_bypass_actuator,
                    self.thermal.etcc3_throttle_valve_1,
                    self.thermal.etcc3_etc_bypass_pass_actuator_1,
                    self.thermal.etcc3_etc_bypass_pass_actuator_2,
                    self.thermal.etcc3_turbo_wastegate_actuator_2
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ETCC3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }
}
