use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_transmission_frames(
        &self,
        frames: &mut Vec<CanFrame>,
        device_id: DeviceId,
    ) {
        // ETC5 - Transmission Control Status
        let etc5 = ETC5 {
            device_id,
            trnsmssn_hgh_rng_sns_swth: self.transmission.etc5_trnsmssn_hgh_rng_sns_swth,
            transmission_low_range_sense_switch: self
                .transmission
                .etc5_transmission_low_range_sense_switch,
            transmission_splitter_position: self.transmission.etc5_transmission_splitter_position,
            trnsmssn_rvrs_drtn_swth: self.transmission.etc5_trnsmssn_rvrs_drtn_swth,
            transmission_neutral_switch: self.transmission.etc5_transmission_neutral_switch,
            trnsmssn_frwrd_drtn_swth: self.transmission.etc5_trnsmssn_frwrd_drtn_swth,
        };

        if let Ok((can_id, data)) = etc5.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC9 - Electronic Transmission Controller 9 (Dual Clutch Transmission)
        let etc9 = ETC9 {
            device_id,
            dl_clth_trnsmssn_crrnt_pr_sltn_gr: self.transmission.etc9_current_preselection_gear,
            dl_clth_trnsmssn_inpt_shft_1_spd: self.transmission.etc9_input_shaft1_speed,
            dl_clth_trnsmssn_inpt_shft_2_spd: self.transmission.etc9_input_shaft2_speed,
            dl_clth_trnsmssn_sltd_pr_sltn_gr: self.transmission.etc9_selected_preselection_gear,
        };

        if let Ok((can_id, data)) = etc9.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC6 - Electronic Transmission Controller 6
        let etc6 = ETC6 {
            device_id,
            recommended_gear: self.transmission.etc6_recommended_gear,
            lowest_possible_gear: self.transmission.etc6_lowest_possible_gear,
            highest_possible_gear: self.transmission.etc6_highest_possible_gear,
            clutch_life_remaining: self.transmission.etc6_clutch_life_remaining,
        };

        if let Ok((can_id, data)) = etc6.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC2 - Electronic Transmission Controller 2
        let etc2 = ETC2 {
            device_id,
            transmission_selected_gear: self.transmission.etc2_transmission_selected_gear,
            transmission_actual_gear_ratio: self.transmission.etc2_transmission_actual_gear_ratio,
            transmission_current_gear: self.transmission.etc2_transmission_current_gear,
            transmission_requested_range: self.transmission.etc2_transmission_requested_range,
            transmission_current_range: self.transmission.etc2_transmission_current_range,
        };

        if let Ok((can_id, data)) = etc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETCC2 - Engine Turbocharger Control 2
        let etcc2 = ETCC2 {
            device_id,
            engn_stgd_trhrgr_slnd_stts: self.transmission.etcc2_engn_stgd_trhrgr_slnd_stts,
            nmr_of_engn_trhrgrs_cmmndd: self.transmission.etcc2_nmr_of_engn_trhrgrs_cmmndd,
        };

        if let Ok((can_id, data)) = etcc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETCC1 - Engine Turbocharger Control 1
        let etcc1 = ETCC1 {
            device_id,
            engn_trhrgr_wstgt_attr_1_cmmnd: self
                .transmission
                .etcc1_engn_trhrgr_wstgt_attr_1_cmmnd,
            engn_trhrgr_wstgt_attr_2_cmmnd: self
                .transmission
                .etcc1_engn_trhrgr_wstgt_attr_2_cmmnd,
            e_exst_b_1_pss_rt_ct_cd: self.transmission.etcc1_e_exst_b_1_pss_rt_ct_cd,
            et_cpss_bw_att_1_cd: self.transmission.etcc1_et_cpss_bw_att_1_cd,
        };

        if let Ok((can_id, data)) = etcc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // Batch 2: Transmission & Drivetrain Broadcast Messages
        // ============================================================================

        // ETC3 - Electronic Transmission Controller 3 (Shift Finger & Actuators)
        let etc3 = ETC3 {
            device_id,
            trnsmssn_shft_fngr_gr_pstn: self.transmission.etc3_shift_finger_gear_position,
            trnsmssn_shft_fngr_rl_pstn: self.transmission.etc3_shift_finger_rail_position,
            trnsmssn_shft_fngr_ntrl_indtr: self.transmission.etc3_shift_finger_neutral_indicator,
            trnsmssn_shft_fngr_enggmnt_indtr: self.transmission.etc3_shift_finger_engagement_indicator,
            trnsmssn_shft_fngr_cntr_rl_indtr: self.transmission.etc3_shift_finger_center_rail_indicator,
            trnsmssn_shft_fngr_rl_attr_1: self.transmission.etc3_shift_finger_rail_actuator_1,
            trnsmssn_shft_fngr_gr_attr_1: self.transmission.etc3_shift_finger_gear_actuator_1,
            trnsmssn_shft_fngr_rl_attr_2: self.transmission.etc3_shift_finger_rail_actuator_2,
            trnsmssn_shft_fngr_gr_attr_2: self.transmission.etc3_shift_finger_gear_actuator_2,
            transmission_range_high_actuator: self.transmission.etc3_range_high_actuator,
            transmission_range_low_actuator: self.transmission.etc3_range_low_actuator,
            trnsmssn_splttr_drt_attr: self.transmission.etc3_splitter_direct_actuator,
            trnsmssn_splttr_indrt_attr: self.transmission.etc3_splitter_indirect_actuator,
            transmission_clutch_actuator: self.transmission.etc3_clutch_actuator,
            trnsmssn_trq_cnvrtr_lkp_clth_attr: self.transmission.etc3_torque_converter_lockup_clutch_actuator,
            transmission_defuel_actuator: self.transmission.etc3_defuel_actuator,
            trnsmssn_inrt_brk_attr: self.transmission.etc3_inertia_brake_actuator,
        };

        if let Ok((can_id, data)) = etc3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC4 - Electronic Transmission Controller 4 (Synchronizer)
        let etc4 = ETC4 {
            device_id,
            trnsmssn_snhrnzr_clth_vl: self.transmission.etc4_synchronizer_clutch_value,
            trnsmssn_snhrnzr_brk_vl: self.transmission.etc4_synchronizer_brake_value,
        };

        if let Ok((can_id, data)) = etc4.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC7 - Electronic Transmission Controller 7 (Display & Indicators)
        let etc7 = ETC7 {
            device_id,
            trnsmssn_crrnt_rng_dspl_blnk_stt: self.transmission.etc7_current_range_display_blank_state,
            transmission_service_indicator: self.transmission.etc7_service_indicator,
            trnsmssn_rqstd_rng_dspl_blnk_stt: self.transmission.etc7_requested_range_display_blank_state,
            trnsmssn_rqstd_rng_dspl_flsh_stt: self.transmission.etc7_requested_range_display_flash_state,
            trnsmssn_rd_fr_brk_rls: self.transmission.etc7_ready_for_brake_release,
            active_shift_console_indicator: self.transmission.etc7_active_shift_console_indicator,
            transmission_engine_crank_enable: self.transmission.etc7_engine_crank_enable,
            trnsmssn_shft_inht_indtr: self.transmission.etc7_shift_inhibit_indicator,
            transmission_mode_4_indicator: self.transmission.etc7_mode_4_indicator,
            transmission_mode_3_indicator: self.transmission.etc7_mode_3_indicator,
            transmission_mode_2_indicator: self.transmission.etc7_mode_2_indicator,
            transmission_mode_1_indicator: self.transmission.etc7_mode_1_indicator,
            trnsmssn_rqstd_gr_fdk: self.transmission.etc7_requested_gear_feedback,
            transmission_mode_5_indicator: self.transmission.etc7_mode_5_indicator,
            transmission_mode_6_indicator: self.transmission.etc7_mode_6_indicator,
            transmission_mode_7_indicator: self.transmission.etc7_mode_7_indicator,
            transmission_mode_8_indicator: self.transmission.etc7_mode_8_indicator,
            trnsmssn_rvrs_gr_shft_inht_stts: self.transmission.etc7_reverse_gear_shift_inhibit_status,
            transmission_warning_indicator: self.transmission.etc7_warning_indicator,
            transmission_mode_9_indicator: self.transmission.etc7_mode_9_indicator,
            transmission_mode_10_indicator: self.transmission.etc7_mode_10_indicator,
            trnsmssn_ar_sppl_prssr_indtr: self.transmission.etc7_air_supply_pressure_indicator,
            trnsmssn_at_ntrl_mnl_rtrn_stt: self.transmission.etc7_auto_neutral_manual_return_state,
            transmission_manual_mode_indicator: self.transmission.etc7_manual_mode_indicator,
            trnsmssn_ld_rdtn_indtr: self.transmission.etc7_load_reduction_indicator,
            trnsmssn_pr_dfnd_rng_lmt_indtr: self.transmission.etc7_pre_defined_range_limit_indicator,
            transmission_coast_mode_indicator: self.transmission.etc7_coast_mode_indicator,
            trnsmssn_otpt_shft_brk_indtr: self.transmission.etc7_output_shaft_brake_indicator,
        };

        if let Ok((can_id, data)) = etc7.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC8 - Electronic Transmission Controller 8 (Torque Converter)
        let etc8 = ETC8 {
            device_id,
            trnsmssn_trq_cnvrtr_rt: self.transmission.etc8_torque_converter_ratio,
            trnsmssn_clth_cnvrtr_inpt_spd: self.transmission.etc8_clutch_converter_input_speed,
            transmission_shift_inhibit_reason: self.transmission.etc8_shift_inhibit_reason,
            trnsmssn_trq_cnvrtr_lkp_inht_rsn: self.transmission.etc8_torque_converter_lockup_inhibit_reason,
            trnsmssn_trq_cnvrtr_lkp_inht_indtr: self.transmission.etc8_torque_converter_lockup_inhibit_indicator,
            trnsmssn_explt_mnl_md_indtr: self.transmission.etc8_explicit_manual_mode_indicator,
        };

        if let Ok((can_id, data)) = etc8.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC10 - Electronic Transmission Controller 10 (Clutch & Actuator Positions)
        let etc10 = ETC10 {
            device_id,
            trnsmssn_clth_1_attr_pstn: self.transmission.etc10_clutch_1_actuator_position,
            trnsmssn_clth_2_attr_pstn: self.transmission.etc10_clutch_2_actuator_position,
            trnsmssn_hdrl_pmp_attr_1_pstn: self.transmission.etc10_hydraulic_pump_actuator_1_position,
            trnsmssn_1_shft_attr_1_pstn: self.transmission.etc10_shift_actuator_1_position,
            trnsmssn_1_shft_attr_2_pstn: self.transmission.etc10_shift_actuator_2_position,
            trnsmssn_clth_1_clng_attr_stts: self.transmission.etc10_clutch_1_cooling_actuator_status,
            trnsmssn_clth_2_clng_attr_stts: self.transmission.etc10_clutch_2_cooling_actuator_status,
            trnsmssn_shft_rl_1_attr_stts: self.transmission.etc10_shift_rail_1_actuator_status,
            trnsmssn_shft_rl_2_attr_stts: self.transmission.etc10_shift_rail_2_actuator_status,
            trnsmssn_shft_rl_3_attr_stts: self.transmission.etc10_shift_rail_3_actuator_status,
            trnsmssn_shft_rl_4_attr_stts: self.transmission.etc10_shift_rail_4_actuator_status,
            trnsmssn_shft_rl_5_attr_stts: self.transmission.etc10_shift_rail_5_actuator_status,
            trnsmssn_shft_rl_6_attr_stts: self.transmission.etc10_shift_rail_6_actuator_status,
            trnsmssn_hdrl_pmp_attr_2_prnt: self.transmission.etc10_hydraulic_pump_actuator_2_percent,
        };

        if let Ok((can_id, data)) = etc10.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC11 - Electronic Transmission Controller 11 (Shift Rail Positions)
        let etc11 = ETC11 {
            device_id,
            transmission_shift_rail_1_position: self.transmission.etc11_shift_rail_1_position,
            transmission_shift_rail_2_position: self.transmission.etc11_shift_rail_2_position,
            transmission_shift_rail_3_position: self.transmission.etc11_shift_rail_3_position,
            transmission_shift_rail_4_position: self.transmission.etc11_shift_rail_4_position,
            transmission_shift_rail_5_position: self.transmission.etc11_shift_rail_5_position,
            transmission_shift_rail_6_position: self.transmission.etc11_shift_rail_6_position,
        };

        if let Ok((can_id, data)) = etc11.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC12 - Electronic Transmission Controller 12 (Hydrostatic Loop)
        let etc12 = ETC12 {
            device_id,
            trnsmssn_hdrstt_lp_1_prssr: self.transmission.etc12_hydrostatic_loop_1_pressure,
            trnsmssn_hdrstt_lp_2_prssr: self.transmission.etc12_hydrostatic_loop_2_pressure,
            trnsmssn_drtnl_otpt_shft_spd: self.transmission.etc12_directional_output_shaft_speed,
            trnsmssn_intrmdt_shft_spd: self.transmission.etc12_intermediate_shaft_speed,
        };

        if let Ok((can_id, data)) = etc12.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC13 - Electronic Transmission Controller 13 (Max Speeds & Mode Indicators)
        let etc13 = ETC13 {
            device_id,
            mxmm_frwrd_trnsmssn_otpt_shft_spd: self.transmission.etc13_max_forward_output_shaft_speed,
            mxmm_rvrs_trnsmssn_otpt_shft_spd: self.transmission.etc13_max_reverse_output_shaft_speed,
            s_addss_o_atv_o_pd_tsss_rqstd_g: self.transmission.etc13_source_address_requested_gear,
            transmission_mode_11_indicator: self.transmission.etc13_mode_11_indicator,
            transmission_mode_12_indicator: self.transmission.etc13_mode_12_indicator,
            transmission_mode_13_indicator: self.transmission.etc13_mode_13_indicator,
            transmission_mode_14_indicator: self.transmission.etc13_mode_14_indicator,
            transmission_mode_15_indicator: self.transmission.etc13_mode_15_indicator,
            transmission_mode_16_indicator: self.transmission.etc13_mode_16_indicator,
            transmission_mode_17_indicator: self.transmission.etc13_mode_17_indicator,
            transmission_mode_18_indicator: self.transmission.etc13_mode_18_indicator,
            transmission_mode_19_indicator: self.transmission.etc13_mode_19_indicator,
            transmission_mode_20_indicator: self.transmission.etc13_mode_20_indicator,
        };

        if let Ok((can_id, data)) = etc13.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC14 - Electronic Transmission Controller 14 (Clutch Temp & Capability)
        let etc14 = ETC14 {
            device_id,
            transmission_clutch_1_temperature: self.transmission.etc14_clutch_1_temperature,
            trnsmssn_clth_1_ovrht_indtr: self.transmission.etc14_clutch_1_overheat_indicator,
            transmission_launch_capability: self.transmission.etc14_launch_capability,
            transmission_gear_shift_capability: self.transmission.etc14_gear_shift_capability,
            trnsmssn_dmg_thrshld_stts: self.transmission.etc14_damage_threshold_status,
        };

        if let Ok((can_id, data)) = etc14.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC15 - Electronic Transmission Controller 15 (CRC, Counter, Auto-Neutral)
        let etc15 = ETC15 {
            device_id,
            eltrn_trnsmssn_cntrl_15_cr: self.transmission.etc15_crc,
            eltrn_trnsmssn_cntrl_15_cntr: self.transmission.etc15_counter,
            trnsmssn_at_ntrl_at_rtrn_rqst_fdk: self.transmission.etc15_auto_neutral_auto_return_request_feedback,
            launch_process_status: self.transmission.etc15_launch_process_status,
            trnsmssn_at_ntrl_at_rtrn_fntn_stt: self.transmission.etc15_auto_neutral_auto_return_function_state,
        };

        if let Ok((can_id, data)) = etc15.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETCC4 - Engine Turbocharger Control 4
        let etcc4 = ETCC4 {
            device_id,
            engn_trhrgr_wstgt_attr_3_cmmnd: self.transmission.etcc4_wastegate_actuator_3_command,
            engn_trhrgr_wstgt_attr_4_cmmnd: self.transmission.etcc4_wastegate_actuator_4_command,
        };

        if let Ok((can_id, data)) = etcc4.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETCBI - Engine Turbocharger Compressor Bypass Information
        let etcbi = ETCBI {
            device_id,
            engn_trhrgr_cmprssr_bpss_attr_2_pstn: self.transmission.etcbi_compressor_bypass_actuator_2_position,
            et_cpss_bpss_att_2_dsd_pst: self.transmission.etcbi_compressor_bypass_actuator_2_desired_position,
            et_cpss_bpss_att_2_pf: self.transmission.etcbi_compressor_bypass_actuator_2_preliminary_fmi,
            et_cpss_bpss_att_2_tpt_stts: self.transmission.etcbi_compressor_bypass_actuator_2_temp_status,
            et_cpss_bpss_att_1_opt_stts: self.transmission.etcbi_compressor_bypass_actuator_1_operation_status,
            et_cpss_bpss_att_2_opt_stts: self.transmission.etcbi_compressor_bypass_actuator_2_operation_status,
            et_cpss_bpss_att_1_tpt: self.transmission.etcbi_compressor_bypass_actuator_1_temperature,
            et_cpss_bpss_att_2_tpt: self.transmission.etcbi_compressor_bypass_actuator_2_temperature,
        };

        if let Ok((can_id, data)) = etcbi.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // TC1 - Transmission Control 1 (Gear Shift Commands)
        let tc1 = TC1 {
            device_id,
            trnsmssn_gr_shft_inht_rqst: self.transmission.tc1_gear_shift_inhibit_request,
            trnsmssn_trq_cnvrtr_lkp_rqst: self.transmission.tc1_torque_converter_lockup_request,
            disengage_driveline_request: self.transmission.tc1_disengage_driveline_request,
            trnsmssn_rvrs_gr_shft_inht_rqst: self.transmission.tc1_reverse_gear_shift_inhibit_request,
            requested_percent_clutch_slip: self.transmission.tc1_requested_percent_clutch_slip,
            transmission_requested_gear: self.transmission.tc1_transmission_requested_gear,
            dsngg_dffrntl_lk_rqst_frnt_axl_1: self.transmission.tc1_disengage_differential_lock_front_axle_1,
            dsngg_dffrntl_lk_rqst_frnt_axl_2: self.transmission.tc1_disengage_differential_lock_front_axle_2,
            dsngg_dffrntl_lk_rqst_rr_axl_1: self.transmission.tc1_disengage_differential_lock_rear_axle_1,
            dsngg_dffrntl_lk_rqst_rr_axl_2: self.transmission.tc1_disengage_differential_lock_rear_axle_2,
            dsngg_dffrntl_lk_rqst_cntrl: self.transmission.tc1_disengage_differential_lock_central,
            dsngg_dffrntl_lk_rqst_cntrl_frnt: self.transmission.tc1_disengage_differential_lock_central_front,
            dsngg_dffrntl_lk_rqst_cntrl_rr: self.transmission.tc1_disengage_differential_lock_central_rear,
            trnsmssn_ld_rdtn_inht_rqst: self.transmission.tc1_load_reduction_inhibit_request,
            transmission_mode_1: self.transmission.tc1_mode_1,
            transmission_mode_2: self.transmission.tc1_mode_2,
            transmission_mode_3: self.transmission.tc1_mode_3,
            transmission_mode_4: self.transmission.tc1_mode_4,
            trnsmssn_at_ntrl_mnl_rtrn_rqst: self.transmission.tc1_auto_neutral_manual_return_request,
            transmission_requested_launch_gear: self.transmission.tc1_requested_launch_gear,
            trnsmssn_shft_sltr_dspl_md_swth: self.transmission.tc1_shift_selector_display_mode_switch,
            transmission_mode_5: self.transmission.tc1_mode_5,
            transmission_mode_6: self.transmission.tc1_mode_6,
            transmission_mode_7: self.transmission.tc1_mode_7,
            transmission_mode_8: self.transmission.tc1_mode_8,
        };

        if let Ok((can_id, data)) = tc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // TC2 - Transmission Control 2 (Extended Mode Commands)
        let tc2 = TC2 {
            device_id,
            transmission_mode_9: self.transmission.tc2_mode_9,
            transmission_mode_10: self.transmission.tc2_mode_10,
            trnsmssn_pr_dfnd_mxmm_gr_atvtn_rqst: self.transmission.tc2_pre_defined_max_gear_activation_request,
            trnsmssn_otpt_shft_brk_rqst: self.transmission.tc2_output_shaft_brake_request,
            trnsmssn_rqstd_rvrs_lnh_gr: self.transmission.tc2_requested_reverse_launch_gear,
            sltd_mxmm_gr_lmt_atvtn_rqst: self.transmission.tc2_selected_max_gear_limit_activation_request,
            transmission_mode_11: self.transmission.tc2_mode_11,
            transmission_mode_12: self.transmission.tc2_mode_12,
            transmission_mode_13: self.transmission.tc2_mode_13,
            transmission_mode_14: self.transmission.tc2_mode_14,
            transmission_mode_15: self.transmission.tc2_mode_15,
            transmission_mode_16: self.transmission.tc2_mode_16,
            transmission_mode_17: self.transmission.tc2_mode_17,
            transmission_mode_18: self.transmission.tc2_mode_18,
            transmission_mode_19: self.transmission.tc2_mode_19,
            transmission_mode_20: self.transmission.tc2_mode_20,
            dsngg_dffrntl_lk_rqst_rr_axl_3: self.transmission.tc2_disengage_differential_lock_rear_axle_3,
            trnsmssn_cst_md_dsl_rqst: self.transmission.tc2_coast_mode_disable_request,
            trnsmssn_explt_mnl_md_rqst: self.transmission.tc2_explicit_manual_mode_request,
        };

        if let Ok((can_id, data)) = tc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
