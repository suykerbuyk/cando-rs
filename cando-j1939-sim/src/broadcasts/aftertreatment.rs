use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::create_can_frame;
use cando_simulator_common::FrameType;
use socketcan::CanFrame;

impl SimulatorState {
    pub(super) fn generate_aftertreatment_frames(
        &self,
        frames: &mut Vec<CanFrame>,
        device_id: DeviceId,
    ) {
        // AT1S1 - Aftertreatment 1 Service 1
        let at1s1 = AT1S1 {
            device_id,
            aftrtrtmnt_1_dsl_prtlt_fltr_st_ld_prnt: self.aftertreatment.at1s1_dpf_soot_load_percent,
            atttt_1_ds_ptt_ft_as_ld_pt: self.aftertreatment.at1s1_dpf_ash_load_percent,
            atttt_1_ds_ptt_ft_ts_lst_atv_rt: self.aftertreatment.at1s1_dpf_time_since_last_regen,
            atttt_1_ds_ptt_ft_st_ld_rt_tsd: self.aftertreatment.at1s1_dpf_soot_load_regen_threshold,
        };
        if let Ok((can_id, data)) = at1s1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1S2 - Aftertreatment 1 Service 2
        let at1s2 = AT1S2 {
            device_id,
            atttt_1_ds_ptt_ft_tt_nxt_atv_rt: self.aftertreatment.at1s2_dpf_time_to_next_regen,
            atttt_1_s_sst_ts_lst_sst_c_evt: self.aftertreatment.at1s2_scr_time_since_cleaning,
        };
        if let Ok((can_id, data)) = at1s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1T1I1 - Aftertreatment 1 Diesel Exhaust Fluid Tank 1 Information 1
        let at1t1i1 = AT1T1I1 {
            device_id,
            aftrtrtmnt_1_dsl_exhst_fld_tnk_vlm: self.aftertreatment.at1t1i1_def_tank_volume,
            atttt_1_ds_exst_fd_t_tpt_1: self.aftertreatment.at1t1i1_def_tank_temp,
            aftrtrtmnt_1_dsl_exhst_fld_tnk_lvl: self.aftertreatment.at1t1i1_def_tank_level,
            atttt_1_ds_exst_fd_t_lv_vpf: self.aftertreatment.at1t1i1_def_tank_level_prelim_fmi,
            atttt_ds_exst_fd_t_lw_lv_idt: self.aftertreatment.at1t1i1_def_tank_low_level_indicator,
            atttt_1_ds_exst_fd_t_1_tpt_pf: self.aftertreatment.at1t1i1_def_tank_temp_prelim_fmi,
            aftrtrtmnt_sr_oprtr_indmnt_svrt: self.aftertreatment.at1t1i1_scr_operator_inducement_severity,
            aftrtrtmnt_1_dsl_exhst_fld_tnk_htr: self.aftertreatment.at1t1i1_def_tank_heater,
            atttt_1_ds_exst_fd_t_1_ht_pf: self.aftertreatment.at1t1i1_def_tank_heater_prelim_fmi,
        };
        if let Ok((can_id, data)) = at1t1i1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1T1I2 - Aftertreatment 1 Diesel Exhaust Fluid Tank 1 Information 2
        let at1t1i2 = AT1T1I2 {
            device_id,
            aftrtrtmnt_1_dsl_exhst_fld_tnk_vlm_2: self.aftertreatment.at1t1i2_def_tank_volume_2,
            atttt_1_ds_exst_fd_t_tpt_2: self.aftertreatment.at1t1i2_def_tank_temp_2,
            aftrtrtmnt_1_dsl_exhst_fld_tnk_htr_2: self.aftertreatment.at1t1i2_def_tank_heater_2,
        };
        if let Ok((can_id, data)) = at1t1i2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1TI - Aftertreatment 1 Trip Information
        let at1ti = AT1TI {
            device_id,
            aftrtrtmnt_1_dsl_prtlt_fltr_trp_fl_usd: self.aftertreatment.at1ti_dpf_trip_fuel_used,
            atttt_1_ds_ptt_ft_tp_atv_rt_t: self.aftertreatment.at1ti_dpf_trip_active_regen_time,
            atttt_1_ds_ptt_ft_tp_dsd_t: self.aftertreatment.at1ti_dpf_trip_disabled_time,
            atttt_1_ds_ptt_ft_tp_no_atv_rts: self.aftertreatment.at1ti_dpf_trip_num_active_regens,
            atttt_1_ds_ptt_ft_tp_pssv_rt_t: self.aftertreatment.at1ti_dpf_trip_passive_regen_time,
            atttt_1_ds_ptt_ft_tp_no_pssv_rts: self.aftertreatment.at1ti_dpf_trip_num_passive_regens,
            atttt_1_ds_ptt_ft_tp_no_atv_rt_it_rqsts: self.aftertreatment.at1ti_dpf_trip_num_regen_inhibit_requests,
            atttt_1_ds_ptt_ft_tp_no_atv_rt_m_rqsts: self.aftertreatment.at1ti_dpf_trip_num_regen_manual_requests,
        };
        if let Ok((can_id, data)) = at1ti.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1OG1 - Aftertreatment 1 Outlet Gas 1
        let at1og1 = AT1OG1 {
            device_id,
            aftertreatment_1_outlet_nox_1: self.aftertreatment.at1og1_outlet_nox,
            aftrtrtmnt_1_otlt_prnt_oxgn_1: self.aftertreatment.at1og1_outlet_oxygen,
            aftrtrtmnt_1_otlt_gs_snsr_1_pwr_in_rng: self.aftertreatment.at1og1_outlet_gas_sensor_power_in_range,
            aftrtrtmnt_1_otlt_gs_snsr_1_at_tmprtr: self.aftertreatment.at1og1_outlet_gas_sensor_at_temp,
            aftrtrtmnt_1_otlt_nx_1_rdng_stl: self.aftertreatment.at1og1_outlet_nox_reading_stable,
            atttt_1_ott_wd_r_pt_ox_1_rd_st: self.aftertreatment.at1og1_outlet_oxygen_reading_stable,
            atttt_1_ott_gs_ss_1_ht_pf: self.aftertreatment.at1og1_outlet_gas_sensor_heater_prelim_fmi,
            aftrtrtmnt_1_otlt_gs_snsr_1_htr_cntrl: self.aftertreatment.at1og1_outlet_gas_sensor_heater_control,
            aftrtrtmnt_1_otlt_nx_snsr_1_prlmnr_fm: self.aftertreatment.at1og1_outlet_nox_sensor_prelim_fmi,
            atttt_1_ott_nx_ss_1_s_dss_stts: self.aftertreatment.at1og1_outlet_nox_sensor_self_diag,
            atttt_1_ott_ox_ss_1_pf: self.aftertreatment.at1og1_outlet_oxygen_sensor_prelim_fmi,
        };
        if let Ok((can_id, data)) = at1og1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1OG2 - Aftertreatment 1 Outlet Gas 2
        let at1og2 = AT1OG2 {
            device_id,
            aftrtrtmnt_1_exhst_tmprtr_3: self.aftertreatment.at1og2_exhaust_temp_3,
            atttt_1_ds_ptt_ft_ott_tpt: self.aftertreatment.at1og2_dpf_outlet_temp,
            aftrtrtmnt_1_exhst_tmprtr_3_prlmnr_fm: self.aftertreatment.at1og2_exhaust_temp_3_prelim_fmi,
            atttt_1_ds_ptt_ft_ott_exst_tpt_pf: self.aftertreatment.at1og2_dpf_outlet_temp_prelim_fmi,
            aftrtrtmnt_exhst_1_dw_pnt_dttd: self.aftertreatment.at1og2_exhaust_dew_point_detected,
        };
        if let Ok((can_id, data)) = at1og2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1IG1 - Aftertreatment 1 Intake Gas 1
        let at1ig1 = AT1IG1 {
            device_id,
            engine_exhaust_1_nox_1: self.aftertreatment.at1ig1_inlet_nox,
            engine_exhaust_1_percent_oxygen_1: self.aftertreatment.at1ig1_inlet_oxygen,
            engn_exhst_1_gs_snsr_1_pwr_in_rng: self.aftertreatment.at1ig1_inlet_gas_sensor_power_in_range,
            engn_exhst_1_gs_snsr_1_at_tmprtr: self.aftertreatment.at1ig1_inlet_gas_sensor_at_temp,
            engine_exhaust_1_nox_1_reading_stable: self.aftertreatment.at1ig1_inlet_nox_reading_stable,
            engn_exhst_1_wd_rng_prnt_oxgn_1_rdng_stl: self.aftertreatment.at1ig1_inlet_oxygen_reading_stable,
            engn_exhst_1_gs_snsr_1_htr_prlmnr_fm: self.aftertreatment.at1ig1_inlet_gas_sensor_heater_prelim_fmi,
            engn_exhst_1_gs_snsr_1_htr_cntrl: self.aftertreatment.at1ig1_inlet_gas_sensor_heater_control,
            engn_exhst_1_nx_snsr_1_prlmnr_fm: self.aftertreatment.at1ig1_inlet_nox_sensor_prelim_fmi,
            engn_exhst_1_nx_snsr_1_slf_dgnss_stts: self.aftertreatment.at1ig1_inlet_nox_sensor_self_diag,
            engn_exhst_1_oxgn_snsr_1_prlmnr_fm: self.aftertreatment.at1ig1_inlet_oxygen_sensor_prelim_fmi,
        };
        if let Ok((can_id, data)) = at1ig1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1IG2 - Aftertreatment 1 Intake Gas 2
        let at1ig2 = AT1IG2 {
            device_id,
            aftrtrtmnt_1_exhst_tmprtr_1: self.aftertreatment.at1ig2_exhaust_temp_1,
            atttt_1_ds_ptt_ft_it_tpt: self.aftertreatment.at1ig2_dpf_intake_temp,
            aftrtrtmnt_1_exhst_tmprtr_1_prlmnr_fm: self.aftertreatment.at1ig2_exhaust_temp_1_prelim_fmi,
            atttt_1_ds_ptt_ft_it_tpt_pf: self.aftertreatment.at1ig2_dpf_intake_temp_prelim_fmi,
            engine_exhaust_1_dew_point_detected: self.aftertreatment.at1ig2_engine_exhaust_dew_point,
        };
        if let Ok((can_id, data)) = at1ig2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1HI1 - Aftertreatment 1 Historical Information 1
        let at1hi1 = AT1HI1 {
            device_id,
            aftertreatment_1_total_fuel_used: self.aftertreatment.at1hi1_total_fuel_used,
            aftrtrtmnt_1_ttl_rgnrtn_tm: self.aftertreatment.at1hi1_total_regen_time,
            aftrtrtmnt_1_ttl_dsld_tm: self.aftertreatment.at1hi1_total_disabled_time,
            aftrtrtmnt_1_ttl_nmr_of_atv_rgnrtns: self.aftertreatment.at1hi1_total_num_active_regens,
            atttt_1_ds_ptt_ft_tt_pssv_rt_t: self.aftertreatment.at1hi1_dpf_total_passive_regen_time,
            atttt_1_ds_ptt_ft_tt_no_pssv_rts: self.aftertreatment.at1hi1_dpf_total_num_passive_regens,
            atttt_1_ds_ptt_ft_tt_no_atv_rt_it_rqsts: self.aftertreatment.at1hi1_dpf_total_num_regen_inhibit_requests,
            atttt_1_ds_ptt_ft_tt_no_atv_rt_m_rqsts: self.aftertreatment.at1hi1_dpf_total_num_regen_manual_requests,
            atttt_1_ds_ptt_ft_av_t_btw_atv_rts: self.aftertreatment.at1hi1_dpf_avg_time_between_regens,
            atttt_1_ds_ptt_ft_av_dst_btw_atv_rts: self.aftertreatment.at1hi1_dpf_avg_distance_between_regens,
            atttt_1_ds_ptt_ft_no_atv_rts: self.aftertreatment.at1hi1_dpf_num_active_regens,
        };
        if let Ok((can_id, data)) = at1hi1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1GP - Aftertreatment 1 Gas Pressure
        let at1gp = AT1GP {
            device_id,
            atttt_1_ds_ptt_ft_it_pss: self.aftertreatment.at1gp_dpf_intake_pressure,
            atttt_1_ds_ptt_ft_ott_pss: self.aftertreatment.at1gp_dpf_outlet_pressure,
        };
        if let Ok((can_id, data)) = at1gp.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1FC1 - Aftertreatment 1 Fuel Control 1
        let at1fc1 = AT1FC1 {
            device_id,
            aftertreatment_1_fuel_pressure_1: self.aftertreatment.at1fc1_fuel_pressure_1,
            aftertreatment_1_fuel_rate: self.aftertreatment.at1fc1_fuel_rate,
            aftrtrtmnt_1_fl_prssr_1_cntrl: self.aftertreatment.at1fc1_fuel_pressure_1_control,
            aftrtrtmnt_1_fl_drn_attr: self.aftertreatment.at1fc1_fuel_drain_actuator,
            aftertreatment_1_ignition: self.aftertreatment.at1fc1_ignition,
            aftrtrtmnt_1_rgnrtn_stts: self.aftertreatment.at1fc1_regen_status,
            aftrtrtmnt_1_fl_enl_attr: self.aftertreatment.at1fc1_fuel_enable_actuator,
            aftrtrtmnt_1_fl_injtr_1_htr_cntrl: self.aftertreatment.at1fc1_fuel_injector_heater_control,
        };
        if let Ok((can_id, data)) = at1fc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1FC2 - Aftertreatment 1 Fuel Control 2
        let at1fc2 = AT1FC2 {
            device_id,
            aftertreatment_1_fuel_pressure_2: self.aftertreatment.at1fc2_fuel_pressure_2,
            aftrtrtmnt_1_fl_pmp_rl_cntrl: self.aftertreatment.at1fc2_fuel_pump_relay_control,
            aftrtrtmnt_1_fl_flw_dvrtr_vlv_cntrl: self.aftertreatment.at1fc2_fuel_flow_diverter_valve,
            aftrtrtmnt_1_fl_prssr_2_cntrl: self.aftertreatment.at1fc2_fuel_pressure_2_control,
            aftrtrtmnt_1_hdrrn_dsr_intk_fl_tmprtr: self.aftertreatment.at1fc2_hc_doser_intake_fuel_temp,
        };
        if let Ok((can_id, data)) = at1fc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT1AC1 - Aftertreatment 1 Air Control 1
        let at1ac1 = AT1AC1 {
            device_id,
            aftrtrtmnt_1_sppl_ar_prssr: self.aftertreatment.at1ac1_supply_air_pressure,
            aftertreatment_1_purge_air_pressure: self.aftertreatment.at1ac1_purge_air_pressure,
            aftrtrtmnt_1_ar_prssr_cntrl: self.aftertreatment.at1ac1_air_pressure_control,
            aftrtrtmnt_1_ar_prssr_attr_pstn: self.aftertreatment.at1ac1_air_pressure_actuator_pos,
            aftertreatment_1_air_system_relay: self.aftertreatment.at1ac1_air_system_relay,
            aftrtrtmnt_1_atmztn_ar_attr: self.aftertreatment.at1ac1_atomization_air_actuator,
            aftertreatment_1_purge_air_actuator: self.aftertreatment.at1ac1_purge_air_actuator,
            aftrtrtmnt_1_ar_enl_attr: self.aftertreatment.at1ac1_air_enable_actuator,
        };
        if let Ok((can_id, data)) = at1ac1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A1DOC1 - Aftertreatment 1 Diesel Oxidation Catalyst 1
        let a1doc1 = A1DOC1 {
            device_id,
            atttt_1_ds_oxdt_ctst_it_tpt: self.aftertreatment.a1doc1_intake_temp,
            atttt_1_ds_oxdt_ctst_ott_tpt: self.aftertreatment.a1doc1_outlet_temp,
            atttt_1_ds_oxdt_ctst_dt_pss: self.aftertreatment.a1doc1_delta_pressure,
            atttt_1_ds_oxdt_ctst_it_tpt_pf: self.aftertreatment.a1doc1_intake_temp_prelim_fmi,
            atttt_1_ds_oxdt_ctst_ott_tpt_pf: self.aftertreatment.a1doc1_outlet_temp_prelim_fmi,
            atttt_1_ds_oxdt_ctst_dt_pss_pf: self.aftertreatment.a1doc1_delta_pressure_prelim_fmi,
        };
        if let Ok((can_id, data)) = a1doc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A1DOC2 - Aftertreatment 1 Diesel Oxidation Catalyst 2
        let a1doc2 = A1DOC2 {
            device_id,
            atttt_1_ds_oxdt_ctst_it_pss: self.aftertreatment.a1doc2_intake_pressure,
            atttt_1_ds_oxdt_ctst_ott_pss: self.aftertreatment.a1doc2_outlet_pressure,
            atttt_1_d_it_t_dp_ott_dt_pss: self.aftertreatment.a1doc2_intake_to_dpf_outlet_delta,
        };
        if let Ok((can_id, data)) = a1doc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A1SCRAI - Aftertreatment 1 SCR Ammonia Information
        let a1scrai = A1SCRAI {
            device_id,
            aftertreatment_1_outlet_nh_3: self.aftertreatment.a1scrai_outlet_nh3,
            aftrtrtmnt_1_otlt_nh_3_snsr_prlmnr_fm: self.aftertreatment.a1scrai_outlet_nh3_prelim_fmi,
            aftrtrtmnt_1_otlt_nh_3_rdng_stl: self.aftertreatment.a1scrai_outlet_nh3_reading_stable,
            atttt_1_ott_n_3_gs_ss_pw_ir: self.aftertreatment.a1scrai_outlet_nh3_sensor_power_in_range,
            atttt_1_ott_n_3_gs_ss_at_tpt: self.aftertreatment.a1scrai_outlet_nh3_sensor_at_temp,
            atttt_1_ott_n_3_gs_ss_ht_pf: self.aftertreatment.a1scrai_outlet_nh3_sensor_heater_prelim_fmi,
            atttt_1_ott_n_3_gs_ss_ht_ct: self.aftertreatment.a1scrai_outlet_nh3_sensor_heater_control,
        };
        if let Ok((can_id, data)) = a1scrai.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A1SCRSI1 - Aftertreatment 1 SCR Status Information 1
        let a1scrsi1 = A1SCRSI1 {
            device_id,
            atttt_1_ds_exst_fd_av_cspt: self.aftertreatment.a1scrsi1_def_avg_consumption,
            atttt_1_s_cdd_ds_exst_fd_cspt: self.aftertreatment.a1scrsi1_scr_commanded_def_consumption,
            aftrtrtmnt_1_sr_cnvrsn_effn: self.aftertreatment.a1scrsi1_scr_conversion_efficiency,
            atttt_s_opt_idt_atv_tvd_dst: self.aftertreatment.a1scrsi1_scr_inducement_travel_distance,
            aftrtrtmnt_1_sr_sstm_slftn_lvl: self.aftertreatment.a1scrsi1_scr_sulfation_level,
        };
        if let Ok((can_id, data)) = a1scrsi1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A1SCRSI2 - Aftertreatment 1 SCR Status Information 2
        let a1scrsi2 = A1SCRSI2 {
            device_id,
            aftrtrtmnt_1_ttl_dsl_exhst_fld_usd: self.aftertreatment.a1scrsi2_total_def_used,
            aftrtrtmnt_trp_dsl_exhst_fld: self.aftertreatment.a1scrsi2_trip_def_used,
        };
        if let Ok((can_id, data)) = a1scrsi2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DPF1S - Diesel Particulate Filter 1 Soot Status
        let dpf1s = DPF1S {
            device_id,
            aftrtrtmnt_1_dsl_prtlt_fltr_st_mss: self.aftertreatment.dpf1s_soot_mass,
            aftrtrtmnt_1_dsl_prtlt_fltr_st_dnst: self.aftertreatment.dpf1s_soot_density,
            aftrtrtmnt_1_dsl_prtlt_fltr_mn_st_sgnl: self.aftertreatment.dpf1s_mean_soot_signal,
            atttt_1_ds_ptt_ft_md_st_s: self.aftertreatment.dpf1s_median_soot_signal,
            atttt_1_ds_ptt_ft_st_ss_pf: self.aftertreatment.dpf1s_soot_sensor_prelim_fmi,
            ds_ptt_ft_1_st_ss_e_it_tpt: self.aftertreatment.dpf1s_soot_sensor_ecu_temp,
        };
        if let Ok((can_id, data)) = dpf1s.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DPF1S2 - Diesel Particulate Filter 1 Soot Status 2
        let dpf1s2 = DPF1S2 {
            device_id,
            atttt_1_ds_ptt_ft_st_s_stdd_dvt: self.aftertreatment.dpf1s2_soot_signal_std_dev,
            atttt_1_ds_ptt_ft_st_s_mx: self.aftertreatment.dpf1s2_soot_signal_max,
            atttt_1_ds_ptt_ft_st_sm: self.aftertreatment.dpf1s2_soot_signal_min,
        };
        if let Ok((can_id, data)) = dpf1s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DPFC1 - Diesel Particulate Filter Control 1
        let dpfc1 = DPFC1 {
            device_id,
            dsl_prtlt_fltr_lmp_cmmnd: self.aftertreatment.dpfc1_dpf_lamp_command,
            dsl_prtlt_fltr_atv_rgnrtn_avllt_stts: self.aftertreatment.dpfc1_dpf_active_regen_availability,
            atttt_ds_ptt_ft_pssv_rt_stts: self.aftertreatment.dpfc1_dpf_passive_regen_status,
            atttt_ds_ptt_ft_atv_rt_stts: self.aftertreatment.dpfc1_dpf_active_regen_status,
            aftrtrtmnt_dsl_prtlt_fltr_stts: self.aftertreatment.dpfc1_dpf_status,
            dsl_prtlt_fltr_atv_rgnrtn_inhtd_stts: self.aftertreatment.dpfc1_dpf_active_regen_inhibited,
            ds_ptt_ft_atv_rt_itd_dt_it_swt: self.aftertreatment.dpfc1_dpf_regen_inhibited_switch,
            ds_ptt_ft_atv_rt_itd_dt_ct_dsd: self.aftertreatment.dpfc1_dpf_regen_inhibited_clutch,
            ds_ptt_ft_atv_rt_itd_dt_sv_b_atv: self.aftertreatment.dpfc1_dpf_regen_inhibited_brake,
            ds_ptt_ft_atv_rt_itd_dt_pt_atv: self.aftertreatment.dpfc1_dpf_regen_inhibited_pto,
            ds_ptt_ft_atv_rt_itd_dt_at_pd_o_id: self.aftertreatment.dpfc1_dpf_regen_inhibited_accel,
            ds_ptt_ft_atv_rt_itd_dt_ot_o_nt: self.aftertreatment.dpfc1_dpf_regen_inhibited_neutral,
            ds_ptt_ft_atv_rt_itd_dtv_spd_av_awd_spd: self.aftertreatment.dpfc1_dpf_regen_inhibited_speed,
            ds_ptt_ft_atv_rt_itd_dtpb_nt_st: self.aftertreatment.dpfc1_dpf_regen_inhibited_parking,
            ds_ptt_ft_atv_rt_itd_dt_lw_exst_tpt: self.aftertreatment.dpfc1_dpf_regen_inhibited_low_temp,
            ds_ptt_ft_atv_rt_itd_dt_sst_ft_atv: self.aftertreatment.dpfc1_dpf_regen_inhibited_fault,
            ds_ptt_ft_atv_rt_itd_dt_sst_tt: self.aftertreatment.dpfc1_dpf_regen_inhibited_timeout,
            ds_ptt_ft_atv_rt_itd_dt_tp_sst_lt: self.aftertreatment.dpfc1_dpf_regen_inhibited_temp_lockout,
            ds_ptt_ft_atv_rt_itd_dt_pt_sst_lt: self.aftertreatment.dpfc1_dpf_regen_inhibited_perm_lockout,
            ds_ptt_ft_atv_rt_itd_dte_nt_wd_up: self.aftertreatment.dpfc1_dpf_regen_inhibited_engine_not_warm,
            ds_ptt_ft_atv_rt_itd_dtv_spd_bw_awd_spd: self.aftertreatment.dpfc1_dpf_regen_inhibited_speed_below,
            ds_ptt_ft_att_atv_rt_itt_ct: self.aftertreatment.dpfc1_dpf_auto_regen_config,
            exhst_sstm_hgh_tmprtr_lmp_cmmnd: self.aftertreatment.dpfc1_exhaust_high_temp_lamp,
            dsl_prtlt_fltr_atv_rgnrtn_frd_stts: self.aftertreatment.dpfc1_dpf_regen_forced_status,
            hydrocarbon_doser_purging_enable: self.aftertreatment.dpfc1_hc_doser_purging_enable,
            ds_ptt_ft_atv_rt_itd_dt_lw_exst_pss: self.aftertreatment.dpfc1_dpf_regen_inhibited_low_pressure,
            atttt_1_ds_ptt_ft_cdts_nt_mt_f_atv_rt: self.aftertreatment.dpfc1_dpf_conditions_not_met,
            ds_ptt_ft_atv_rt_itd_dt_ts: self.aftertreatment.dpfc1_dpf_regen_inhibited_thresher,
        };
        if let Ok((can_id, data)) = dpfc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DPFC2 - Diesel Particulate Filter Control 2
        let dpfc2 = DPFC2 {
            device_id,
            atttt_1_ds_ptt_ft_it_tpt_st_pt: self.aftertreatment.dpfc2_dpf_intake_temp_setpoint,
            engine_unburned_fuel_percentage: self.aftertreatment.dpfc2_engine_unburned_fuel_pct,
            aftertreatment_1_fuel_mass_rate: self.aftertreatment.dpfc2_at1_fuel_mass_rate,
            aftertreatment_2_fuel_mass_rate: self.aftertreatment.dpfc2_at2_fuel_mass_rate,
        };
        if let Ok((can_id, data)) = dpfc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // Batch 11: Aftertreatment Bank 2 + EGR Broadcast Messages
        // ============================================================================

        // AT2S1 - Aftertreatment 2 DPF Soot Status 1
        let at2s1 = AT2S1 {
            device_id,
            aftrtrtmnt_2_dsl_prtlt_fltr_st_ld_prnt: self.aftertreatment.at2s1_dpf_soot_load_percent,
            atttt_2_ds_ptt_ft_as_ld_pt: self.aftertreatment.at2s1_dpf_ash_load_percent,
            atttt_2_ds_ptt_ft_ts_lst_atv_rt: self.aftertreatment.at2s1_dpf_time_since_last_regen,
            atttt_2_ds_ptt_ft_st_ld_rt_tsd: self.aftertreatment.at2s1_dpf_soot_load_regen_threshold,
        };
        if let Ok((can_id, data)) = at2s1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT2S2 - Aftertreatment 2 DPF Status 2
        let at2s2 = AT2S2 {
            device_id,
            atttt_2_ds_ptt_ft_tt_nxt_atv_rt: self.aftertreatment.at2s2_dpf_time_to_next_regen,
            atttt_2_s_sst_ts_lst_sst_c_evt: self.aftertreatment.at2s2_scr_time_since_last_clean,
        };
        if let Ok((can_id, data)) = at2s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT2OG1 - Aftertreatment 2 Outlet Gas Sensor 1
        let at2og1 = AT2OG1 {
            device_id,
            aftertreatment_2_outlet_nox_1: self.aftertreatment.at2og1_outlet_nox,
            aftrtrtmnt_2_otlt_prnt_oxgn_1: self.aftertreatment.at2og1_outlet_percent_oxygen,
            aftrtrtmnt_2_otlt_gs_snsr_1_pwr_in_rng: self.aftertreatment.at2og1_outlet_sensor_power_in_range,
            aftrtrtmnt_2_otlt_gs_snsr_1_at_tmprtr: self.aftertreatment.at2og1_outlet_sensor_at_temp,
            aftrtrtmnt_2_otlt_nx_1_rdng_stl: 1,       // Reading stable
            atttt_2_ott_wd_r_pt_ox_1_rd_st: 1,        // Reading stable
            atttt_2_ott_gs_ss_1_ht_pf: 0,             // No fault
            aftrtrtmnt_2_otlt_gs_snsr_1_htr_cntrl: 3, // Automatic
            aftrtrtmnt_2_otlt_nx_snsr_1_prlmnr_fm: 0, // No fault
            atttt_2_ott_nx_ss_1_s_dss_stts: 0,        // Diagnosis not active
            atttt_2_ott_ox_ss_1_pf: 0,                // No fault
            aftrtrtmnt_2_otlt_2_gs_snsr_pwr_sppl: 1,  // On
        };
        if let Ok((can_id, data)) = at2og1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT2IG1 - Aftertreatment 2 Inlet Gas Sensor 1
        let at2ig1 = AT2IG1 {
            device_id,
            engine_exhaust_2_nox_1: self.aftertreatment.at2ig1_inlet_nox,
            engine_exhaust_2_percent_oxygen_1: self.aftertreatment.at2ig1_inlet_percent_oxygen,
            engn_exhst_2_gs_snsr_1_pwr_in_rng: self.aftertreatment.at2ig1_inlet_sensor_power_in_range,
            engn_exhst_2_gs_snsr_1_at_tmprtr: self.aftertreatment.at2ig1_inlet_sensor_at_temp,
            engine_exhaust_2_nox_1_reading_stable: 1,        // Reading stable
            engn_exhst_2_wd_rng_prnt_oxgn_1_rdng_stl: 1,   // Reading stable
            engn_exhst_2_gs_snsr_1_htr_prlmnr_fm: 0,        // No fault
            engn_exhst_2_gs_snsr_1_htr_cntrl: 3,            // Automatic
            engn_exhst_2_nx_snsr_1_prlmnr_fm: 0,            // No fault
            engn_exhst_2_nx_snsr_1_slf_dgnss_stts: 0,       // Not active
            engn_exhst_2_oxgn_snsr_1_prlmnr_fm: 0,          // No fault
            engn_exhst_2_gs_snsr_2_pwr_sppl: 1,             // On
        };
        if let Ok((can_id, data)) = at2ig1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT2GP - Aftertreatment 2 Gas Pressures
        let at2gp = AT2GP {
            device_id,
            atttt_2_ds_ptt_ft_it_pss: self.aftertreatment.at2gp_dpf_intake_pressure,
            atttt_2_ds_ptt_ft_ott_pss: self.aftertreatment.at2gp_dpf_outlet_pressure,
        };
        if let Ok((can_id, data)) = at2gp.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT2FC1 - Aftertreatment 2 Fuel Control 1
        let at2fc1 = AT2FC1 {
            device_id,
            aftertreatment_2_fuel_pressure_1: self.aftertreatment.at2fc1_fuel_pressure,
            aftertreatment_2_fuel_rate: self.aftertreatment.at2fc1_fuel_rate,
            aftrtrtmnt_2_fl_prssr_1_cntrl: self.aftertreatment.at2fc1_fuel_pressure_control,
            aftrtrtmnt_2_fl_drn_attr: 0,               // Not active
            aftertreatment_2_ignition: 0,              // Not active
            aftrtrtmnt_2_rgnrtn_stts: self.aftertreatment.at2fc1_regen_status,
            aftrtrtmnt_2_fl_enl_attr: 1,               // Active
            aftrtrtmnt_2_fl_injtr_1_htr_cntrl: 0.0,   // Heater off
        };
        if let Ok((can_id, data)) = at2fc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // AT2AC1 - Aftertreatment 2 Air Control 1
        let at2ac1 = AT2AC1 {
            device_id,
            aftrtrtmnt_2_sppl_ar_prssr: self.aftertreatment.at2ac1_supply_air_pressure,
            aftertreatment_2_purge_air_pressure: self.aftertreatment.at2ac1_purge_air_pressure,
            aftrtrtmnt_2_ar_prssr_cntrl: self.aftertreatment.at2ac1_air_pressure_control,
            aftrtrtmnt_2_ar_prssr_attr_pstn: self.aftertreatment.at2ac1_air_pressure_actuator_position,
            aftertreatment_2_air_system_relay: 1,      // Active
            aftrtrtmnt_2_atmztn_ar_attr: 0,            // Not active
            aftertreatment_2_purge_air_actuator: 0,    // Not active
            aftrtrtmnt_2_ar_enl_attr: 1,               // Active
        };
        if let Ok((can_id, data)) = at2ac1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A2DOC1 - Aftertreatment 2 Diesel Oxidation Catalyst 1
        let a2doc1 = A2DOC1 {
            device_id,
            atttt_2_ds_oxdt_ctst_it_tpt: self.aftertreatment.a2doc1_inlet_temp,
            atttt_2_ds_oxdt_ctst_ott_tpt: self.aftertreatment.a2doc1_outlet_temp,
            atttt_2_ds_oxdt_ctst_dt_pss: self.aftertreatment.a2doc1_diff_pressure,
            atttt_2_ds_oxdt_ctst_it_tpt_pf: 0,  // No fault
            atttt_2_ds_oxdt_ctst_ott_tpt_pf: 0, // No fault
            atttt_2_ds_oxdt_ctst_dt_pss_pf: 0,  // No fault
        };
        if let Ok((can_id, data)) = a2doc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A2SCRAI - Aftertreatment 2 SCR Ammonia Information
        let a2scrai = A2SCRAI {
            device_id,
            aftertreatment_2_outlet_nh_3: self.aftertreatment.a2scrai_outlet_nh3,
            aftrtrtmnt_2_otlt_nh_3_snsr_prlmnr_fm: 0, // No fault
            aftrtrtmnt_2_otlt_nh_3_rdng_stl: 1,       // Reading stable
            atttt_2_ott_n_3_gs_ss_pw_ir: 1,            // In range
            atttt_2_ott_n_3_gs_ss_at_tpt: 1,           // At temperature
            atttt_2_ott_n_3_gs_ss_ht_pf: 0,            // No fault
            atttt_2_ott_n_3_gs_ss_ht_ct: 3,            // Automatic
        };
        if let Ok((can_id, data)) = a2scrai.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A2SCRSI1 - Aftertreatment 2 SCR Status Information 1
        let a2scrsi1 = A2SCRSI1 {
            device_id,
            atttt_2_ds_exst_fd_av_cspt: self.aftertreatment.a2scrsi1_def_avg_consumption,
            atttt_2_s_cdd_ds_exst_fd_cspt: self.aftertreatment.a2scrsi1_scr_commanded_consumption,
            aftrtrtmnt_2_sr_cnvrsn_effn: self.aftertreatment.a2scrsi1_scr_conversion_efficiency,
            aftrtrtmnt_2_sr_sstm_slftn_lvl: 5,        // 5% sulfation level (low)
            atttt_2_ds_exst_fd_usd_ts_opt_c: 10.0,    // 10 liters used this cycle
        };
        if let Ok((can_id, data)) = a2scrsi1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A1SCRDSI1 - Aftertreatment 1 SCR Dosing System Information 1
        let a1scrdsi1 = A1SCRDSI1 {
            device_id,
            atttt_1_ds_exst_fd_at_ds_qtt: self.aftertreatment.a1scrdsi1_dosing_rate,
            aftertreatment_1_scr_system_1_state: self.aftertreatment.a1scrdsi1_scr_system_1_state,
            aftertreatment_1_scr_system_2_state: 0,    // Dormant
            atttt_1_ds_exst_fd_at_qtt_o_itt: 100.0,   // 100 g integrator
            atttt_1_ds_exst_fd_ds_1_ast_pss: self.aftertreatment.a1scrdsi1_doser_1_abs_pressure,
            atttt_1_ds_exst_fd_at_ds_qtt_hr: 0.0,     // Not using high range
        };
        if let Ok((can_id, data)) = a1scrdsi1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A1SCRDSI2 - Aftertreatment 1 SCR Dosing System Information 2
        let a1scrdsi2 = A1SCRDSI2 {
            device_id,
            atttt_1_s_ds_a_assst_ast_pss: self.aftertreatment.a1scrdsi2_air_assist_pressure,
            aftrtrtmnt_1_sr_dsng_ar_assst_vlv: self.aftertreatment.a1scrdsi2_air_assist_valve,
            atttt_1_ds_exst_fd_ds_1_tpt: self.aftertreatment.a1scrdsi2_doser_1_temp,
            atttt_1_s_ds_vv_exst_tpt_rdt_rqst: 0,     // No reduction request
            aftrtrtmnt_1_sr_fdk_cntrl_stts: 1,         // Closed loop active
            aftrtrtmnt_1_dsl_exhst_fld_ln_htr_1_stt: 0, // Heater inactive
            atttt_1_ds_exst_fd_l_ht_1_pf: 0,          // No fault
            aftrtrtmnt_1_dsl_exhst_fld_ln_htr_2_stt: 0, // Heater inactive
            atttt_1_ds_exst_fd_l_ht_2_pf: 0,          // No fault
            aftrtrtmnt_1_dsl_exhst_fld_ln_htr_3_stt: 0, // Heater inactive
            atttt_1_ds_exst_fd_l_ht_3_pf: 0,          // No fault
            aftrtrtmnt_1_dsl_exhst_fld_ln_htr_4_stt: 0, // Heater inactive
            atttt_1_ds_exst_fd_l_ht_4_pf: 0,          // No fault
        };
        if let Ok((can_id, data)) = a1scrdsi2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A1SCRDSI3 - Aftertreatment 1 SCR Dosing System Information 3
        let a1scrdsi3 = A1SCRDSI3 {
            device_id,
            aftrtrtmnt_1_dsl_exhst_fld_dsr_1_prssr: self.aftertreatment.a1scrdsi3_doser_1_pressure,
            atttt_1_ds_exst_fd_ds_2_ast_pss: self.aftertreatment.a1scrdsi3_doser_2_abs_pressure,
            atttt_1_ds_exst_fd_ds_2_tpt: self.aftertreatment.a1scrdsi3_doser_2_temp,
            aftrtrtmnt_1_dsl_exhst_fld_dsr_2_prssr: 350.0, // 350 kPa doser 2 pressure
            atttt_1_ds_exst_fd_ds_1_pss_extdd_r: 400.0,    // 400 kPa extended range
        };
        if let Ok((can_id, data)) = a1scrdsi3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A2SCRDSI1 - Aftertreatment 2 SCR Dosing System Information 1
        let a2scrdsi1 = A2SCRDSI1 {
            device_id,
            atttt_2_ds_exst_fd_at_ds_qtt: self.aftertreatment.a2scrdsi1_dosing_rate,
            aftertreatment_2_scr_system_1_state: self.aftertreatment.a2scrdsi1_scr_system_1_state,
            aftertreatment_2_scr_system_2_state: 0,    // Dormant
            atttt_2_ds_exst_fd_at_qtt_o_itt: 95.0,    // 95 g integrator
            atttt_2_ds_exst_fd_ds_1_ast_pss: self.aftertreatment.a2scrdsi1_doser_1_abs_pressure,
            atttt_2_ds_exst_fd_at_ds_qtt_hr: 0.0,     // Not using high range
        };
        if let Ok((can_id, data)) = a2scrdsi1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A2SCRDSI2 - Aftertreatment 2 SCR Dosing System Information 2
        let a2scrdsi2 = A2SCRDSI2 {
            device_id,
            atttt_2_s_ds_a_assst_ast_pss: self.aftertreatment.a2scrdsi2_air_assist_pressure,
            aftrtrtmnt_2_sr_dsng_ar_assst_vlv: self.aftertreatment.a2scrdsi2_air_assist_valve,
            atttt_2_ds_exst_fd_ds_1_tpt: self.aftertreatment.a2scrdsi2_doser_1_temp,
            atttt_2_s_ds_vv_exst_tpt_rdt_rqst: 0,     // No reduction request
            aftrtrtmnt_2_sr_fdk_cntrl_stts: 1,         // Closed loop active
            aftrtrtmnt_2_dsl_exhst_fld_ln_htr_1_stt: 0, // Heater inactive
            atttt_2_ds_exst_fd_l_ht_1_pf: 0,          // No fault
            aftrtrtmnt_2_dsl_exhst_fld_ln_htr_2_stt: 0, // Heater inactive
            atttt_2_ds_exst_fd_l_ht_2_pf: 0,          // No fault
            aftrtrtmnt_2_dsl_exhst_fld_ln_htr_3_stt: 0, // Heater inactive
            atttt_2_ds_exst_fd_l_ht_3_pf: 0,          // No fault
            aftrtrtmnt_2_dsl_exhst_fld_ln_htr_4_stt: 0, // Heater inactive
            atttt_2_ds_exst_fd_l_ht_4_pf: 0,          // No fault
        };
        if let Ok((can_id, data)) = a2scrdsi2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // A2SCRDSI3 - Aftertreatment 2 SCR Dosing System Information 3
        let a2scrdsi3 = A2SCRDSI3 {
            device_id,
            aftrtrtmnt_2_dsl_exhst_fld_dsr_1_prssr: self.aftertreatment.a2scrdsi3_doser_1_pressure,
            atttt_2_ds_exst_fd_ds_2_ast_pss: self.aftertreatment.a2scrdsi3_doser_2_abs_pressure,
            atttt_2_ds_exst_fd_ds_2_tpt: self.aftertreatment.a2scrdsi3_doser_2_temp,
            aftrtrtmnt_2_dsl_exhst_fld_dsr_2_prssr: 330.0, // 330 kPa doser 2 pressure
            atttt_2_ds_exst_fd_ds_1_pss_extdd_r: 380.0,    // 380 kPa extended range
        };
        if let Ok((can_id, data)) = a2scrdsi3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEGR1A - Engine EGR 1 Actuator
        let eegr1a = EEGR1A {
            device_id,
            engn_exhst_gs_rrltn_1_attr_1_prlmnr_fm: 0, // No fault
            e_exst_gs_rt_1_att_1_tpt_stts: 0,           // Normal
            engn_exhst_gs_rrltn_1_attr_1_tmprtr: self.aftertreatment.eegr1a_actuator_1_temp,
            engn_exhst_gs_rrltn_1_attr_1_dsrd_pstn: self.aftertreatment.eegr1a_actuator_1_desired_position,
            engn_exhst_gs_rrltn_1_attr_2_prlmnr_fm: 0,  // No fault
            e_exst_gs_rt_1_att_2_tpt_stts: 0,           // Normal
            engn_exhst_gs_rrltn_1_attr_2_tmprtr: self.aftertreatment.eegr1a_actuator_2_temp,
            engn_exhst_gs_rrltn_1_attr_2_dsrd_pstn: self.aftertreatment.eegr1a_actuator_2_desired_position,
            engn_exhst_gs_rrltn_1_attr_1_oprtn_stts: 0, // Normal
            engn_exhst_gs_rrltn_1_attr_2_oprtn_stts: 0, // Normal
        };
        if let Ok((can_id, data)) = eegr1a.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EEGR2A - Engine EGR 2 Actuator
        let eegr2a = EEGR2A {
            device_id,
            engn_exhst_gs_rrltn_2_attr_1_prlmnr_fm: 0, // No fault
            e_exst_gs_rt_2_att_1_tpt_stts: 0,           // Normal
            engn_exhst_gs_rrltn_2_attr_1_tmprtr: self.aftertreatment.eegr2a_actuator_1_temp,
            engn_exhst_gs_rrltn_2_attr_1_dsrd_pstn: self.aftertreatment.eegr2a_actuator_1_desired_position,
            engn_exhst_gs_rrltn_2_attr_2_prlmnr_fm: 0,  // No fault
            e_exst_gs_rt_2_att_2_tpt_stts: 0,           // Normal
            engn_exhst_gs_rrltn_2_attr_2_tmprtr: self.aftertreatment.eegr2a_actuator_2_temp,
            engn_exhst_gs_rrltn_2_attr_2_dsrd_pstn: self.aftertreatment.eegr2a_actuator_2_desired_position,
            engn_exhst_gs_rrltn_2_attr_1_oprtn_stts: 0, // Normal
            engn_exhst_gs_rrltn_2_attr_2_oprtn_stts: 0, // Normal
        };
        if let Ok((can_id, data)) = eegr2a.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DPF2S - Diesel Particulate Filter 2 Soot
        let dpf2s = DPF2S {
            device_id,
            aftrtrtmnt_2_dsl_prtlt_fltr_st_mss: self.aftertreatment.dpf2s_soot_mass,
            aftrtrtmnt_2_dsl_prtlt_fltr_st_dnst: self.aftertreatment.dpf2s_soot_density,
            aftrtrtmnt_2_dsl_prtlt_fltr_mn_st_sgnl: self.aftertreatment.dpf2s_mean_soot_signal,
            atttt_2_ds_ptt_ft_md_st_s: self.aftertreatment.dpf2s_median_soot_signal,
            atttt_2_ds_ptt_ft_st_ss_pf: 0,            // No fault
            ds_ptt_ft_2_st_ss_e_it_tpt: 45.0,         // 45 degC ECU internal temp
        };
        if let Ok((can_id, data)) = dpf2s.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
