use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_engine_frames(&self, frames: &mut Vec<CanFrame>, device_id: DeviceId) {
        // EEC12 - Engine Exhaust Sensor Power Supply
        let eec12 = EEC12 {
            device_id,
            engn_exhst_1_gs_snsr_1_pwr_sppl: self.engine.eec12_engn_exhst_1_gs_snsr_1_pwr_sppl,
            aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl: self
                .engine
                .eec12_aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl,
            engn_exhst_2_gs_snsr_1_pwr_sppl: self.engine.eec12_engn_exhst_2_gs_snsr_1_pwr_sppl,
            aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl: self
                .engine
                .eec12_aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl,
            engn_exhst_1_gs_snsr_2_pwr_sppl: self.engine.eec12_engn_exhst_1_gs_snsr_2_pwr_sppl,
            aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl: self
                .engine
                .eec12_aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl,
        };

        if let Ok((can_id, data)) = eec12.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC22 - Electronic Engine Controller 22
        let eec22 = EEC22 {
            device_id,
            engn_exhst_gs_rrltn_1_clr_intk_prssr: self.engine.eec22_engnexhstgsrrltn1clrintkprssr,
            ttl_nmr_of_crnk_attmpts_drng_engn_lf: self
                .engine
                .eec22_ttlnmrofcrnkattmptsdrngengnlf,
        };

        if let Ok((can_id, data)) = eec22.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC21 - Electronic Engine Controller 21
        let eec21 = EEC21 {
            device_id,
            engn_exhst_mnfld_aslt_prssr_1: self.engine.eec21_engn_exhst_mnfld_aslt_prssr_1,
            engn_exhst_mnfld_aslt_prssr_2: self.engine.eec21_engn_exhst_mnfld_aslt_prssr_2,
        };

        if let Ok((can_id, data)) = eec21.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC17 - Electronic Engine Controller 17
        let eec17 = EEC17 {
            device_id,
            pems_engine_fuel_mass_flow_rate: self.engine.eec17_pems_engine_fuel_mass_flow_rate,
            vehicle_fuel_rate: self.engine.eec17_vehicle_fuel_rate,
            engine_exhaust_flow_rate: self.engine.eec17_engine_exhaust_flow_rate,
            cylinder_fuel_rate: self.engine.eec17_cylinder_fuel_rate,
        };

        if let Ok((can_id, data)) = eec17.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC15 - Electronic Engine Controller 15
        let eec15 = EEC15 {
            device_id,
            accelerator_pedal_1_channel_2: self.engine.eec15_accelerator_pedal_1_channel_2,
            accelerator_pedal_1_channel_3: self.engine.eec15_accelerator_pedal_1_channel_3,
            accelerator_pedal_2_channel_2: self.engine.eec15_accelerator_pedal_2_channel_2,
            accelerator_pedal_2_channel_3: self.engine.eec15_accelerator_pedal_2_channel_3,
            engn_exhst_gs_rstrtn_vlv_cntrl: self.engine.eec15_engn_exhst_gs_rstrtn_vlv_cntrl,
        };

        if let Ok((can_id, data)) = eec15.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC8 - Electronic Engine Controller 8
        let eec8 = EEC8 {
            device_id,
            engn_exhst_gs_rrltn_1_vlv_2_cntrl: self
                .engine
                .eec8_engn_exhst_gs_rrltn_1_vlv_2_cntrl,
            engn_exhst_gs_rrltn_1_clr_intk_tmprtr: self
                .engine
                .eec8_engn_exhst_gs_rrltn_1_clr_intk_tmprtr,
            e_exst_gs_rt_1_c_it_ast_pss: self.engine.eec8_e_exst_gs_rt_1_c_it_ast_pss,
            engn_exhst_gs_rrltn_1_clr_effn: self.engine.eec8_engn_exhst_gs_rrltn_1_clr_effn,
            e_exst_gs_rt_at_it_ct_tpt: self.engine.eec8_e_exst_gs_rt_at_it_ct_tpt,
        };

        if let Ok((can_id, data)) = eec8.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC1 - Electronic Engine Controller 1
        let eec1 = EEC1 {
            device_id,
            engine_torque_mode: self.engine.eec1_engine_torque_mode,
            atl_engn_prnt_trq_frtnl: self.engine.eec1_atl_engn_prnt_trq_frtnl,
            drvr_s_dmnd_engn_prnt_trq: self.engine.eec1_drvr_s_dmnd_engn_prnt_trq,
            actual_engine_percent_torque: self.engine.eec1_actual_engine_percent_torque,
            engine_speed: self.engine.eec1_engine_speed,
            sr_addrss_of_cntrllng_dv_fr_engn_cntrl: self.engine.eec1_sr_addrss_of_cntrllng_dv_fr_engn_cntrl,
            engine_starter_mode: self.engine.eec1_engine_starter_mode,
            engine_demand_percent_torque: self.engine.eec1_engine_demand_percent_torque,
        };
        if let Ok((can_id, data)) = eec1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC2 - Electronic Engine Controller 2
        let eec2 = EEC2 {
            device_id,
            accelerator_pedal_1_low_idle_switch: self.engine.eec2_accelerator_pedal_1_low_idle_switch,
            accelerator_pedal_kickdown_switch: self.engine.eec2_accelerator_pedal_kickdown_switch,
            road_speed_limit_status: self.engine.eec2_road_speed_limit_status,
            accelerator_pedal_2_low_idle_switch: self.engine.eec2_accelerator_pedal_2_low_idle_switch,
            accelerator_pedal_1_position: self.engine.eec2_accelerator_pedal_1_position,
            engine_percent_load_at_current_speed: self.engine.eec2_engine_percent_load_at_current_speed,
            remote_accelerator_pedal_position: self.engine.eec2_remote_accelerator_pedal_position,
            accelerator_pedal_2_position: self.engine.eec2_accelerator_pedal_2_position,
            vhl_alrtn_rt_lmt_stts: self.engine.eec2_vhl_alrtn_rt_lmt_stts,
            mmntr_engn_mxmm_pwr_enl_fdk: self.engine.eec2_mmntr_engn_mxmm_pwr_enl_fdk,
            dpf_thermal_management_active: self.engine.eec2_dpf_thermal_management_active,
            scr_thermal_management_active: self.engine.eec2_scr_thermal_management_active,
            atl_mxmm_avll_engn_prnt_trq: self.engine.eec2_atl_mxmm_avll_engn_prnt_trq,
            estimated_pumping_percent_torque: self.engine.eec2_estimated_pumping_percent_torque,
        };
        if let Ok((can_id, data)) = eec2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC3 - Electronic Engine Controller 3
        let eec3 = EEC3 {
            device_id,
            nominal_friction_percent_torque: self.engine.eec3_nominal_friction_percent_torque,
            engine_s_desired_operating_speed: self.engine.eec3_engine_s_desired_operating_speed,
            es_dsd_opt_spd_ast_adstt: self.engine.eec3_es_dsd_opt_spd_ast_adstt,
            estmtd_engn_prst_lsss_prnt_trq: self.engine.eec3_estmtd_engn_prst_lsss_prnt_trq,
            aftrtrtmnt_1_exhst_gs_mss_flw_rt: self.engine.eec3_aftrtrtmnt_1_exhst_gs_mss_flw_rt,
            engine_exhaust_1_dew_point: self.engine.eec3_engine_exhaust_1_dew_point,
            aftertreatment_1_exhaust_dew_point: self.engine.eec3_aftertreatment_1_exhaust_dew_point,
            engine_exhaust_2_dew_point: self.engine.eec3_engine_exhaust_2_dew_point,
            aftertreatment_2_exhaust_dew_point: self.engine.eec3_aftertreatment_2_exhaust_dew_point,
        };
        if let Ok((can_id, data)) = eec3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC4 - Electronic Engine Controller 4
        let eec4 = EEC4 {
            device_id,
            engine_rated_power: self.engine.eec4_engine_rated_power,
            engine_rated_speed: self.engine.eec4_engine_rated_speed,
            engine_rotation_direction: self.engine.eec4_engine_rotation_direction,
            engn_intk_mnfld_prssr_cntrl_md: self.engine.eec4_engn_intk_mnfld_prssr_cntrl_md,
            crnk_attmpt_cnt_on_prsnt_strt_attmpt: self.engine.eec4_crnk_attmpt_cnt_on_prsnt_strt_attmpt,
            engn_prl_ol_lw_prssr_thrshld: self.engine.eec4_engn_prl_ol_lw_prssr_thrshld,
        };
        if let Ok((can_id, data)) = eec4.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC5 - Electronic Engine Controller 5
        let eec5 = EEC5 {
            device_id,
            engn_trhrgr_1_clltd_trn_intk_tmprtr: self.engine.eec5_engn_trhrgr_1_clltd_trn_intk_tmprtr,
            engn_trhrgr_1_clltd_trn_otlt_tmprtr: self.engine.eec5_engn_trhrgr_1_clltd_trn_otlt_tmprtr,
            engn_exhst_gs_rrltn_1_vlv_1_cntrl_1: self.engine.eec5_engn_exhst_gs_rrltn_1_vlv_1_cntrl_1,
            ev_gt_t_vt_a_ct_st_vv: self.engine.eec5_ev_gt_t_vt_a_ct_st_vv,
            engine_fuel_control_mode: self.engine.eec5_engine_fuel_control_mode,
            engn_vrl_gmtr_trhrgr_1_cntrl_md: self.engine.eec5_engn_vrl_gmtr_trhrgr_1_cntrl_md,
            engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn: self.engine.eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn,
        };
        if let Ok((can_id, data)) = eec5.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC6 - Electronic Engine Controller 6
        let eec6 = EEC6 {
            device_id,
            engn_trhrgr_cmprssr_bpss_attr_1_cmmnd: self.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_cmmnd,
            engn_vrl_gmtr_trhrgr_attr_1: self.engine.eec6_engn_vrl_gmtr_trhrgr_attr_1,
            engn_trhrgr_cmprssr_bpss_attr_1_pstn: self.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_1_pstn,
            engn_trhrgr_cmprssr_bpss_attr_2_cmmnd: self.engine.eec6_engn_trhrgr_cmprssr_bpss_attr_2_cmmnd,
            et_cpss_bpss_att_1_dsd_pst: self.engine.eec6_et_cpss_bpss_att_1_dsd_pst,
            et_cpss_bpss_att_1_pf: self.engine.eec6_et_cpss_bpss_att_1_pf,
            et_cpss_bpss_att_1_tpt_stts: self.engine.eec6_et_cpss_bpss_att_1_tpt_stts,
        };
        if let Ok((can_id, data)) = eec6.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC7 - Electronic Engine Controller 7
        let eec7 = EEC7 {
            device_id,
            engn_exhst_gs_rrltn_1_vlv_pstn: self.engine.eec7_engn_exhst_gs_rrltn_1_vlv_pstn,
            engn_exhst_gs_rrltn_1_vlv_2_pstn: self.engine.eec7_engn_exhst_gs_rrltn_1_vlv_2_pstn,
            engn_crnks_brthr_ol_sprtr_spd: self.engine.eec7_engn_crnks_brthr_ol_sprtr_spd,
            engn_intk_mnfld_cmmndd_prssr: self.engine.eec7_engn_intk_mnfld_cmmndd_prssr,
        };
        if let Ok((can_id, data)) = eec7.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC9 - Electronic Engine Controller 9
        let eec9 = EEC9 {
            device_id,
            engn_exhst_gs_rrltn_2_vlv_pstn: self.engine.eec9_engn_exhst_gs_rrltn_2_vlv_pstn,
            engn_exhst_gs_rrltn_2_vlv_2_pstn: self.engine.eec9_engn_exhst_gs_rrltn_2_vlv_2_pstn,
            commanded_engine_fuel_rail_pressure: self.engine.eec9_commanded_engine_fuel_rail_pressure,
            cmmndd_engn_fl_injtn_cntrl_prssr: self.engine.eec9_cmmndd_engn_fl_injtn_cntrl_prssr,
        };
        if let Ok((can_id, data)) = eec9.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC10 - Electronic Engine Controller 10
        let eec10 = EEC10 {
            device_id,
            engn_exhst_gs_rrltn_2_clr_intk_tmprtr: self.engine.eec10_engn_exhst_gs_rrltn_2_clr_intk_tmprtr,
            e_exst_gs_rt_2_c_it_ast_pss: self.engine.eec10_e_exst_gs_rt_2_c_it_ast_pss,
            engn_exhst_gs_rrltn_2_clr_effn: self.engine.eec10_engn_exhst_gs_rrltn_2_clr_effn,
            e_exst_gs_rt_2_c_bpss_att_pst: self.engine.eec10_e_exst_gs_rt_2_c_bpss_att_pst,
            engn_exhst_gs_rrltn_2_clr_intk_prssr: self.engine.eec10_engn_exhst_gs_rrltn_2_clr_intk_prssr,
        };
        if let Ok((can_id, data)) = eec10.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC11 - Electronic Engine Controller 11
        let eec11 = EEC11 {
            device_id,
            engn_exhst_gs_rrltn_2_vlv_1_cntrl: self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_1_cntrl,
            engn_exhst_gs_rrltn_2_vlv_2_cntrl: self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_2_cntrl,
            engn_exhst_gs_rrltn_2_vlv_1_pstn_errr: self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_1_pstn_errr,
            engn_exhst_gs_rrltn_2_vlv_2_pstn_errr: self.engine.eec11_engn_exhst_gs_rrltn_2_vlv_2_pstn_errr,
        };
        if let Ok((can_id, data)) = eec11.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC13 - Electronic Engine Controller 13
        let eec13 = EEC13 {
            device_id,
            feedback_engine_fueling_state: self.engine.eec13_feedback_engine_fueling_state,
            engine_fueling_inhibit_allowed: self.engine.eec13_engine_fueling_inhibit_allowed,
            engn_flng_inht_prvntd_rsn: self.engine.eec13_engn_flng_inht_prvntd_rsn,
            sr_addrss_of_cntrllng_dv_fr_flng_stt: self.engine.eec13_sr_addrss_of_cntrllng_dv_fr_flng_stt,
            engine_dual_fuel_mode: self.engine.eec13_engine_dual_fuel_mode,
            engn_flng_inht_prvntd_rsn_extnsn: self.engine.eec13_engn_flng_inht_prvntd_rsn_extnsn,
            engn_gs_sstttn_fl_prntg: self.engine.eec13_engn_gs_sstttn_fl_prntg,
            engn_flng_inht_rqst_cnt: self.engine.eec13_engn_flng_inht_rqst_cnt,
            engn_flng_dsrd_rqst_cnt: self.engine.eec13_engn_flng_dsrd_rqst_cnt,
            engn_prttn_drt_ovrrd_stts: self.engine.eec13_engn_prttn_drt_ovrrd_stts,
            remaining_engine_motoring_time: self.engine.eec13_remaining_engine_motoring_time,
            engine_performance_bias_level: self.engine.eec13_engine_performance_bias_level,
            minimum_engine_motoring_speed: self.engine.eec13_minimum_engine_motoring_speed,
        };
        if let Ok((can_id, data)) = eec13.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC14 - Electronic Engine Controller 14
        let eec14 = EEC14 {
            device_id,
            engn_exhst_gs_rrltn_1_vlv_1_pstn_errr: self.engine.eec14_engn_exhst_gs_rrltn_1_vlv_1_pstn_errr,
            engn_exhst_gs_rrltn_1_vlv_2_pstn_errr: self.engine.eec14_engn_exhst_gs_rrltn_1_vlv_2_pstn_errr,
            engine_fuel_mass_flow_rate: self.engine.eec14_engine_fuel_mass_flow_rate,
            fuel_type: self.engine.eec14_fuel_type,
            engine_fuel_isolation_control: self.engine.eec14_engine_fuel_isolation_control,
        };
        if let Ok((can_id, data)) = eec14.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC16 - Electronic Engine Controller 16
        let eec16 = EEC16 {
            device_id,
            accelerator_pedal_3_position: self.engine.eec16_accelerator_pedal_3_position,
            ready_for_clutch_engagement_status: self.engine.eec16_ready_for_clutch_engagement_status,
            engine_clutch_engage_request_status: self.engine.eec16_engine_clutch_engage_request_status,
        };
        if let Ok((can_id, data)) = eec16.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC18 - Electronic Engine Controller 18
        let eec18 = EEC18 {
            device_id,
            engn_clndr_hd_bpss_attr_1_cmmnd: self.engine.eec18_engn_clndr_hd_bpss_attr_1_cmmnd,
            engine_intake_air_source_valve: self.engine.eec18_engine_intake_air_source_valve,
            engn_exhst_gs_rstrtn_vlv_pstn: self.engine.eec18_engn_exhst_gs_rstrtn_vlv_pstn,
        };
        if let Ok((can_id, data)) = eec18.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC19 - Electronic Engine Controller 19
        let eec19 = EEC19 {
            device_id,
            total_engine_energy: self.engine.eec19_total_engine_energy,
            engn_exhst_flw_rt_extndd_rng: self.engine.eec19_engn_exhst_flw_rt_extndd_rng,
        };
        if let Ok((can_id, data)) = eec19.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC20 - Electronic Engine Controller 20
        let eec20 = EEC20 {
            device_id,
            esttd_e_pst_lsss_pt_tq_h_rst: self.engine.eec20_esttd_e_pst_lsss_pt_tq_h_rst,
            atl_mxmm_avll_engn_prnt_fl: self.engine.eec20_atl_mxmm_avll_engn_prnt_fl,
            nmnl_frtn_prnt_trq_hgh_rsltn: self.engine.eec20_nmnl_frtn_prnt_trq_hgh_rsltn,
            aslt_engn_ld_prnt_ar_mss: self.engine.eec20_aslt_engn_ld_prnt_ar_mss,
        };
        if let Ok((can_id, data)) = eec20.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC23 - Electronic Engine Controller 23
        let eec23 = EEC23 {
            device_id,
            engn_crnks_prssr_cntrl_attr_1_cmmnd: self.engine.eec23_engn_crnks_prssr_cntrl_attr_1_cmmnd,
            engn_crnks_prssr_cntrl_attr_2_cmmnd: self.engine.eec23_engn_crnks_prssr_cntrl_attr_2_cmmnd,
        };
        if let Ok((can_id, data)) = eec23.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC24 - Electronic Engine Controller 24
        let eec24 = EEC24 {
            device_id,
            engn_crnks_prssr_cntrl_attr_1_tmprtr: self.engine.eec24_engn_crnks_prssr_cntrl_attr_1_tmprtr,
            engn_crnks_prssr_cntrl_attr_1_pstn: self.engine.eec24_engn_crnks_prssr_cntrl_attr_1_pstn,
            e_cs_pss_ct_att_1_dsd_pst: self.engine.eec24_e_cs_pss_ct_att_1_dsd_pst,
            e_cs_pss_ct_att_1_pf: self.engine.eec24_e_cs_pss_ct_att_1_pf,
            e_cs_pss_ct_att_1_tpt_stts: self.engine.eec24_e_cs_pss_ct_att_1_tpt_stts,
            e_cs_pss_ct_att_1_opt_stts: self.engine.eec24_e_cs_pss_ct_att_1_opt_stts,
        };
        if let Ok((can_id, data)) = eec24.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEC25 - Electronic Engine Controller 25
        let eec25 = EEC25 {
            device_id,
            engn_crnks_prssr_cntrl_attr_2_tmprtr: self.engine.eec25_engn_crnks_prssr_cntrl_attr_2_tmprtr,
            engn_crnks_prssr_cntrl_attr_2_pstn: self.engine.eec25_engn_crnks_prssr_cntrl_attr_2_pstn,
            e_cs_pss_ct_att_2_dsd_pst: self.engine.eec25_e_cs_pss_ct_att_2_dsd_pst,
            e_cs_pss_ct_att_2_pf: self.engine.eec25_e_cs_pss_ct_att_2_pf,
            e_cs_pss_ct_att_2_tpt_stts: self.engine.eec25_e_cs_pss_ct_att_2_tpt_stts,
            e_cs_pss_ct_att_2_opt_stts: self.engine.eec25_e_cs_pss_ct_att_2_opt_stts,
        };
        if let Ok((can_id, data)) = eec25.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETC1 - Electronic Transmission Controller 1
        let etc1 = ETC1 {
            device_id,
            transmission_driveline_engaged: self.engine.etc1_transmission_driveline_engaged,
            trnsmssn_trq_cnvrtr_lkp_enggd: self.engine.etc1_trnsmssn_trq_cnvrtr_lkp_enggd,
            transmission_shift_in_process: self.engine.etc1_transmission_shift_in_process,
            tsss_tq_cvt_lp_tst_i_pss: self.engine.etc1_tsss_tq_cvt_lp_tst_i_pss,
            transmission_output_shaft_speed: self.engine.etc1_transmission_output_shaft_speed,
            percent_clutch_slip: self.engine.etc1_percent_clutch_slip,
            engine_momentary_overspeed_enable: self.engine.etc1_engine_momentary_overspeed_enable,
            progressive_shift_disable: self.engine.etc1_progressive_shift_disable,
            mmntr_engn_mxmm_pwr_enl: self.engine.etc1_mmntr_engn_mxmm_pwr_enl,
            transmission_input_shaft_speed: self.engine.etc1_transmission_input_shaft_speed,
            s_addss_o_ct_dv_f_tsss_ct: self.engine.etc1_s_addss_o_ct_dv_f_tsss_ct,
        };
        if let Ok((can_id, data)) = etc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
