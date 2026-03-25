use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineState {
    // Core engine
    pub engine_speed: f64,
    pub engine_torque: f64,
    pub engine_load: f64,
    pub engine_coolant_temp: f64,
    pub engine_oil_pressure: f64,
    pub engine_fuel_rate: f64,
    pub engine_intake_pressure: f64,
    pub engine_exhaust_temp: f64,
    pub turbo_speed: f64,
    pub engine_control_counters: [u64; 8],

    // EEC12
    pub eec12_engn_exhst_1_gs_snsr_1_pwr_sppl: u8,
    pub eec12_aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl: u8,
    pub eec12_engn_exhst_2_gs_snsr_1_pwr_sppl: u8,
    pub eec12_aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl: u8,
    pub eec12_engn_exhst_1_gs_snsr_2_pwr_sppl: u8,
    pub eec12_aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl: u8,

    // EEC22
    pub eec22_engnexhstgsrrltn1clrintkprssr: f64,
    pub eec22_ttlnmrofcrnkattmptsdrngengnlf: u32,

    // EEC21
    pub eec21_engn_exhst_mnfld_aslt_prssr_1: f64,
    pub eec21_engn_exhst_mnfld_aslt_prssr_2: f64,

    // EEC17
    pub eec17_pems_engine_fuel_mass_flow_rate: f64,
    pub eec17_vehicle_fuel_rate: f64,
    pub eec17_engine_exhaust_flow_rate: f64,
    pub eec17_cylinder_fuel_rate: f64,

    // EEC15
    pub eec15_accelerator_pedal_1_channel_2: f64,
    pub eec15_accelerator_pedal_1_channel_3: f64,
    pub eec15_accelerator_pedal_2_channel_2: f64,
    pub eec15_accelerator_pedal_2_channel_3: f64,
    pub eec15_engn_exhst_gs_rstrtn_vlv_cntrl: f64,

    // EEC8
    pub eec8_engn_exhst_gs_rrltn_1_vlv_2_cntrl: f64,
    pub eec8_engn_exhst_gs_rrltn_1_clr_intk_tmprtr: f64,
    pub eec8_e_exst_gs_rt_1_c_it_ast_pss: f64,
    pub eec8_engn_exhst_gs_rrltn_1_clr_effn: f64,
    pub eec8_e_exst_gs_rt_at_it_ct_tpt: f64,

    // EEC1 Electronic Engine Controller 1 States
    pub eec1_engine_torque_mode: u8,               // Engine torque mode (0-15)
    pub eec1_atl_engn_prnt_trq_frtnl: f64,        // Actual engine percent torque fractional (0-0.88%)
    pub eec1_drvr_s_dmnd_engn_prnt_trq: f64,      // Driver's demand engine percent torque (0-125%)
    pub eec1_actual_engine_percent_torque: f64,    // Actual engine percent torque (0-125%)
    pub eec1_engine_speed: f64,                    // Engine speed (0-8031.88 rpm)
    pub eec1_sr_addrss_of_cntrllng_dv_fr_engn_cntrl: u8, // Source address of controlling device (0-253)
    pub eec1_engine_starter_mode: u8,              // Engine starter mode (0-15)
    pub eec1_engine_demand_percent_torque: f64,    // Engine demand percent torque (-125 to 125%)

    // EEC2 Electronic Engine Controller 2 States
    pub eec2_accelerator_pedal_1_low_idle_switch: u8, // Accelerator pedal 1 low idle switch (0-3)
    pub eec2_accelerator_pedal_kickdown_switch: u8,   // Accelerator pedal kickdown switch (0-3)
    pub eec2_road_speed_limit_status: u8,             // Road speed limit status (0-3)
    pub eec2_accelerator_pedal_2_low_idle_switch: u8, // Accelerator pedal 2 low idle switch (0-3)
    pub eec2_accelerator_pedal_1_position: f64,       // Accelerator pedal 1 position (0-100%)
    pub eec2_engine_percent_load_at_current_speed: u8, // Engine percent load at current speed (0-125%)
    pub eec2_remote_accelerator_pedal_position: f64,  // Remote accelerator pedal position (0-100%)
    pub eec2_accelerator_pedal_2_position: f64,       // Accelerator pedal 2 position (0-100%)
    pub eec2_vhl_alrtn_rt_lmt_stts: u8,              // Vehicle acceleration rate limit status (0-3)
    pub eec2_mmntr_engn_mxmm_pwr_enl_fdk: u8,       // Momentary engine max power enable feedback (0-3)
    pub eec2_dpf_thermal_management_active: u8,       // DPF thermal management active (0-3)
    pub eec2_scr_thermal_management_active: u8,       // SCR thermal management active (0-3)
    pub eec2_atl_mxmm_avll_engn_prnt_trq: f64,      // Actual max available engine percent torque (0-100%)
    pub eec2_estimated_pumping_percent_torque: f64,   // Estimated pumping percent torque (-125 to 125%)

    // EEC3 Electronic Engine Controller 3 States
    pub eec3_nominal_friction_percent_torque: f64,    // Nominal friction percent torque (-125 to 125%)
    pub eec3_engine_s_desired_operating_speed: f64,   // Engine desired operating speed (0-8031.88 rpm)
    pub eec3_es_dsd_opt_spd_ast_adstt: u8,           // Engine desired operating speed asymmetry adjustment (0-250)
    pub eec3_estmtd_engn_prst_lsss_prnt_trq: f64,   // Estimated engine parasitic losses percent torque (0-125%)
    pub eec3_aftrtrtmnt_1_exhst_gs_mss_flw_rt: f64,  // Aftertreatment 1 exhaust gas mass flow rate (0-12851 kg/h)
    pub eec3_engine_exhaust_1_dew_point: u8,          // Engine exhaust 1 dew point (0-3)
    pub eec3_aftertreatment_1_exhaust_dew_point: u8,  // Aftertreatment 1 exhaust dew point (0-3)
    pub eec3_engine_exhaust_2_dew_point: u8,          // Engine exhaust 2 dew point (0-3)
    pub eec3_aftertreatment_2_exhaust_dew_point: u8,  // Aftertreatment 2 exhaust dew point (0-3)

    // EEC4 Electronic Engine Controller 4 States
    pub eec4_engine_rated_power: f64,                  // Engine rated power (0-32127.5 kW)
    pub eec4_engine_rated_speed: f64,                  // Engine rated speed (0-8031.88 rpm)
    pub eec4_engine_rotation_direction: u8,            // Engine rotation direction (0-3)
    pub eec4_engn_intk_mnfld_prssr_cntrl_md: u8,      // Engine intake manifold pressure control mode (0-3)
    pub eec4_crnk_attmpt_cnt_on_prsnt_strt_attmpt: u8, // Crank attempt count on present start attempt (0-250)
    pub eec4_engn_prl_ol_lw_prssr_thrshld: f64,       // Engine prelube oil low pressure threshold (0-1000 kPa)

    // EEC5 Electronic Engine Controller 5 States
    pub eec5_engn_trhrgr_1_clltd_trn_intk_tmprtr: f64, // Turbocharger 1 calc turbine intake temp (-273 to 1735 degC)
    pub eec5_engn_trhrgr_1_clltd_trn_otlt_tmprtr: f64, // Turbocharger 1 calc turbine outlet temp (-273 to 1735 degC)
    pub eec5_engn_exhst_gs_rrltn_1_vlv_1_cntrl_1: f64, // EGR 1 valve 1 control 1 (0-100%)
    pub eec5_ev_gt_t_vt_a_ct_st_vv: u8,               // VGT air control shutoff valve (0-3)
    pub eec5_engine_fuel_control_mode: u8,             // Engine fuel control mode (0-3)
    pub eec5_engn_vrl_gmtr_trhrgr_1_cntrl_md: u8,     // VGT 1 control mode (0-3)
    pub eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn: f64, // VGT 1 actuator position (0-100%)

    // EEC6 Electronic Engine Controller 6 States
    pub eec6_engn_trhrgr_cmprssr_bpss_attr_1_cmmnd: f64, // Turbo compressor bypass actuator 1 command (0-100%)
    pub eec6_engn_vrl_gmtr_trhrgr_attr_1: f64,            // VGT actuator #1 (0-100%)
    pub eec6_engn_trhrgr_cmprssr_bpss_attr_1_pstn: f64,   // Turbo compressor bypass actuator 1 position (0-100%)
    pub eec6_engn_trhrgr_cmprssr_bpss_attr_2_cmmnd: f64,  // Turbo compressor bypass actuator 2 command (0-160.64%)
    pub eec6_et_cpss_bpss_att_1_dsd_pst: f64,             // Turbo compressor bypass actuator 1 desired position (0-100%)
    pub eec6_et_cpss_bpss_att_1_pf: u8,                   // Turbo compressor bypass actuator 1 preliminary FMI (0-31)
    pub eec6_et_cpss_bpss_att_1_tpt_stts: u8,             // Turbo compressor bypass actuator 1 temp status (0-7)

    // EEC7 Electronic Engine Controller 7 States
    pub eec7_engn_exhst_gs_rrltn_1_vlv_pstn: f64,         // EGR 1 valve position (0-160.64%)
    pub eec7_engn_exhst_gs_rrltn_1_vlv_2_pstn: f64,       // EGR 1 valve 2 position (0-160.64%)
    pub eec7_engn_crnks_brthr_ol_sprtr_spd: u16,          // Crankcase breather oil separator speed (0-64255 rpm)
    pub eec7_engn_intk_mnfld_cmmndd_prssr: f64,           // Intake manifold commanded pressure (0-8031.88 kPa)

    // EEC9 Electronic Engine Controller 9 States
    pub eec9_engn_exhst_gs_rrltn_2_vlv_pstn: f64,         // EGR 2 valve position (0-160.64%)
    pub eec9_engn_exhst_gs_rrltn_2_vlv_2_pstn: f64,       // EGR 2 valve 2 position (0-160.64%)
    pub eec9_commanded_engine_fuel_rail_pressure: f64,     // Commanded engine fuel rail pressure (0-251 MPa)
    pub eec9_cmmndd_engn_fl_injtn_cntrl_prssr: f64,       // Commanded engine fuel injection control pressure (0-251 MPa)

    // EEC10 Electronic Engine Controller 10 States
    pub eec10_engn_exhst_gs_rrltn_2_clr_intk_tmprtr: f64, // EGR 2 cooler intake temp (-273 to 1735 degC)
    pub eec10_e_exst_gs_rt_2_c_it_ast_pss: f64,           // EGR 2 cooler intake absolute pressure (0-32127.5 kPa)
    pub eec10_engn_exhst_gs_rrltn_2_clr_effn: f64,        // EGR 2 cooler efficiency (0-100%)
    pub eec10_e_exst_gs_rt_2_c_bpss_att_pst: f64,         // EGR 2 cooler bypass actuator position (0-100%)
    pub eec10_engn_exhst_gs_rrltn_2_clr_intk_prssr: f64,  // EGR 2 cooler intake pressure (0-321275 kPa)

    // EEC11 Electronic Engine Controller 11 States
    pub eec11_engn_exhst_gs_rrltn_2_vlv_1_cntrl: f64,     // EGR 2 valve 1 control (0-160.64%)
    pub eec11_engn_exhst_gs_rrltn_2_vlv_2_cntrl: f64,     // EGR 2 valve 2 control (0-160.64%)
    pub eec11_engn_exhst_gs_rrltn_2_vlv_1_pstn_errr: f64, // EGR 2 valve 1 position error (-125 to 132%)
    pub eec11_engn_exhst_gs_rrltn_2_vlv_2_pstn_errr: f64, // EGR 2 valve 2 position error (-125 to 132%)

    // EEC13 Electronic Engine Controller 13 States
    pub eec13_feedback_engine_fueling_state: u8,           // Feedback engine fueling state (0-3)
    pub eec13_engine_fueling_inhibit_allowed: u8,          // Engine fueling inhibit allowed (0-3)
    pub eec13_engn_flng_inht_prvntd_rsn: u8,              // Engine fueling inhibit prevented reason (0-15)
    pub eec13_sr_addrss_of_cntrllng_dv_fr_flng_stt: u8,   // Source address controlling fueling state (0-250)
    pub eec13_engine_dual_fuel_mode: u8,                   // Engine dual fuel mode (0-3)
    pub eec13_engn_flng_inht_prvntd_rsn_extnsn: u8,       // Engine fueling inhibit prevented reason extension (0-63)
    pub eec13_engn_gs_sstttn_fl_prntg: f64,               // Engine gas substitution fuel percentage (0-125%)
    pub eec13_engn_flng_inht_rqst_cnt: u8,                // Engine fueling inhibit request count (0-15)
    pub eec13_engn_flng_dsrd_rqst_cnt: u8,                // Engine fueling desired request count (0-15)
    pub eec13_engn_prttn_drt_ovrrd_stts: u8,              // Engine protection derate override status (0-3)
    pub eec13_remaining_engine_motoring_time: u8,          // Remaining engine motoring time (0-61 s)
    pub eec13_engine_performance_bias_level: f64,          // Engine performance bias level (0-100%)
    pub eec13_minimum_engine_motoring_speed: f64,          // Minimum engine motoring speed (0-2500 rpm)

    // EEC14 Electronic Engine Controller 14 States
    pub eec14_engn_exhst_gs_rrltn_1_vlv_1_pstn_errr: f64, // EGR 1 valve 1 position error (-125 to 132%)
    pub eec14_engn_exhst_gs_rrltn_1_vlv_2_pstn_errr: f64, // EGR 1 valve 2 position error (-125 to 132%)
    pub eec14_engine_fuel_mass_flow_rate: f64,             // Engine fuel mass flow rate (0-321.27 g/s)
    pub eec14_fuel_type: u8,                               // Fuel type (0-250)
    pub eec14_engine_fuel_isolation_control: u8,           // Engine fuel isolation control (0-3)

    // EEC16 Electronic Engine Controller 16 States
    pub eec16_accelerator_pedal_3_position: f64,           // Accelerator pedal 3 position (0-100%)
    pub eec16_ready_for_clutch_engagement_status: u8,      // Ready for clutch engagement status (0-3)
    pub eec16_engine_clutch_engage_request_status: u8,     // Engine clutch engage request status (0-3)

    // EEC18 Electronic Engine Controller 18 States
    pub eec18_engn_clndr_hd_bpss_attr_1_cmmnd: f64,       // Cylinder head bypass actuator 1 command (0-160.64%)
    pub eec18_engine_intake_air_source_valve: u8,          // Engine intake air source valve (0-3)
    pub eec18_engn_exhst_gs_rstrtn_vlv_pstn: f64,         // Exhaust gas restriction valve position (0-160.64%)

    // EEC19 Electronic Engine Controller 19 States
    pub eec19_total_engine_energy: u32,                    // Total engine energy (0-4211081215 kWh)
    pub eec19_engn_exhst_flw_rt_extndd_rng: f64,          // Engine exhaust flow rate extended range (0-4211081.21 kg/h)

    // EEC20 Electronic Engine Controller 20 States
    pub eec20_esttd_e_pst_lsss_pt_tq_h_rst: f64,         // Estimated engine parasitic losses percent torque high resolution (0-100.40%)
    pub eec20_atl_mxmm_avll_engn_prnt_fl: f64,            // Actual max available engine percent fuel (0-100%)
    pub eec20_nmnl_frtn_prnt_trq_hgh_rsltn: f64,          // Nominal friction percent torque high resolution (0-100.40%)
    pub eec20_aslt_engn_ld_prnt_ar_mss: f64,              // Absolute engine load percent air mass (0-1606.38%)

    // EEC23 Electronic Engine Controller 23 States
    pub eec23_engn_crnks_prssr_cntrl_attr_1_cmmnd: f64,   // Crankcase pressure control actuator 1 command (0-160.64%)
    pub eec23_engn_crnks_prssr_cntrl_attr_2_cmmnd: f64,   // Crankcase pressure control actuator 2 command (0-160.64%)

    // EEC24 Electronic Engine Controller 24 States
    pub eec24_engn_crnks_prssr_cntrl_attr_1_tmprtr: f64,  // Crankcase pressure control actuator 1 temp (-40 to 210 degC)
    pub eec24_engn_crnks_prssr_cntrl_attr_1_pstn: f64,    // Crankcase pressure control actuator 1 position (0-100%)
    pub eec24_e_cs_pss_ct_att_1_dsd_pst: f64,             // Crankcase pressure control actuator 1 desired position (0-100%)
    pub eec24_e_cs_pss_ct_att_1_pf: u8,                   // Crankcase pressure control actuator 1 preliminary FMI (0-31)
    pub eec24_e_cs_pss_ct_att_1_tpt_stts: u8,             // Crankcase pressure control actuator 1 temp status (0-7)
    pub eec24_e_cs_pss_ct_att_1_opt_stts: u8,             // Crankcase pressure control actuator 1 operation status (0-15)

    // EEC25 Electronic Engine Controller 25 States
    pub eec25_engn_crnks_prssr_cntrl_attr_2_tmprtr: f64,  // Crankcase pressure control actuator 2 temp (-40 to 210 degC)
    pub eec25_engn_crnks_prssr_cntrl_attr_2_pstn: f64,    // Crankcase pressure control actuator 2 position (0-100%)
    pub eec25_e_cs_pss_ct_att_2_dsd_pst: f64,             // Crankcase pressure control actuator 2 desired position (0-100%)
    pub eec25_e_cs_pss_ct_att_2_pf: u8,                   // Crankcase pressure control actuator 2 preliminary FMI (0-31)
    pub eec25_e_cs_pss_ct_att_2_tpt_stts: u8,             // Crankcase pressure control actuator 2 temp status (0-7)
    pub eec25_e_cs_pss_ct_att_2_opt_stts: u8,             // Crankcase pressure control actuator 2 operation status (0-15)

    // ETC1 Electronic Transmission Controller 1 States
    pub etc1_transmission_driveline_engaged: u8,           // Transmission driveline engaged (0-3)
    pub etc1_trnsmssn_trq_cnvrtr_lkp_enggd: u8,           // Transmission torque converter lockup engaged (0-3)
    pub etc1_transmission_shift_in_process: u8,            // Transmission shift in process (0-3)
    pub etc1_tsss_tq_cvt_lp_tst_i_pss: u8,               // Torque converter lockup transition in process (0-3)
    pub etc1_transmission_output_shaft_speed: f64,         // Transmission output shaft speed (0-8031.88 rpm)
    pub etc1_percent_clutch_slip: f64,                     // Percent clutch slip (0-100%)
    pub etc1_engine_momentary_overspeed_enable: u8,        // Engine momentary overspeed enable (0-3)
    pub etc1_progressive_shift_disable: u8,                // Progressive shift disable (0-3)
    pub etc1_mmntr_engn_mxmm_pwr_enl: u8,                 // Momentary engine max power enable (0-3)
    pub etc1_transmission_input_shaft_speed: f64,          // Transmission input shaft speed (0-8031.88 rpm)
    pub etc1_s_addss_o_ct_dv_f_tsss_ct: u8,               // Source address controlling transmission (0-253)
}

impl Default for EngineState {
    fn default() -> Self {
        Self {
            engine_speed: 800.0,
            engine_torque: 0.0,
            engine_load: 5.0,
            engine_coolant_temp: 85.0,
            engine_oil_pressure: 40.0,
            engine_fuel_rate: 2.0,
            engine_intake_pressure: 100.0,
            engine_exhaust_temp: 200.0,
            turbo_speed: 0.0,
            engine_control_counters: [0; 8],
            eec12_engn_exhst_1_gs_snsr_1_pwr_sppl: 1,
            eec12_aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl: 1,
            eec12_engn_exhst_2_gs_snsr_1_pwr_sppl: 1,
            eec12_aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl: 1,
            eec12_engn_exhst_1_gs_snsr_2_pwr_sppl: 1,
            eec12_aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl: 1,
            eec22_engnexhstgsrrltn1clrintkprssr: 100.0,
            eec22_ttlnmrofcrnkattmptsdrngengnlf: 5000,
            eec21_engn_exhst_mnfld_aslt_prssr_1: 101.3,
            eec21_engn_exhst_mnfld_aslt_prssr_2: 102.5,
            eec17_pems_engine_fuel_mass_flow_rate: 100.0,
            eec17_vehicle_fuel_rate: 95.0,
            eec17_engine_exhaust_flow_rate: 2500.0,
            eec17_cylinder_fuel_rate: 50.0,
            eec15_accelerator_pedal_1_channel_2: 25.0,
            eec15_accelerator_pedal_1_channel_3: 26.0,
            eec15_accelerator_pedal_2_channel_2: 24.0,
            eec15_accelerator_pedal_2_channel_3: 25.0,
            eec15_engn_exhst_gs_rstrtn_vlv_cntrl: 20.0,
            eec8_engn_exhst_gs_rrltn_1_vlv_2_cntrl: 40.0,
            eec8_engn_exhst_gs_rrltn_1_clr_intk_tmprtr: 85.0,
            eec8_e_exst_gs_rt_1_c_it_ast_pss: 150.0,
            eec8_engn_exhst_gs_rrltn_1_clr_effn: 75.0,
            eec8_e_exst_gs_rt_at_it_ct_tpt: 90.0,

            // EEC1 Electronic Engine Controller 1 defaults
            eec1_engine_torque_mode: 0,               // No torque mode active
            eec1_atl_engn_prnt_trq_frtnl: 0.0,       // No fractional torque
            eec1_drvr_s_dmnd_engn_prnt_trq: 0.0,     // No driver demand
            eec1_actual_engine_percent_torque: 5.0,   // Small idle torque
            eec1_engine_speed: 800.0,                 // Idle RPM
            eec1_sr_addrss_of_cntrllng_dv_fr_engn_cntrl: 0, // No controlling device
            eec1_engine_starter_mode: 0,              // Start not requested
            eec1_engine_demand_percent_torque: 0.0,   // No demand torque

            // EEC2 Electronic Engine Controller 2 defaults
            eec2_accelerator_pedal_1_low_idle_switch: 1, // In low idle position
            eec2_accelerator_pedal_kickdown_switch: 0,   // Kickdown passive
            eec2_road_speed_limit_status: 1,             // Not active
            eec2_accelerator_pedal_2_low_idle_switch: 1, // In low idle position
            eec2_accelerator_pedal_1_position: 0.0,      // Released
            eec2_engine_percent_load_at_current_speed: 5, // Minimal idle load
            eec2_remote_accelerator_pedal_position: 0.0, // Released
            eec2_accelerator_pedal_2_position: 0.0,      // Released
            eec2_vhl_alrtn_rt_lmt_stts: 3,              // Not available
            eec2_mmntr_engn_mxmm_pwr_enl_fdk: 0,       // Disabled
            eec2_dpf_thermal_management_active: 0,       // Not active
            eec2_scr_thermal_management_active: 0,       // Not active
            eec2_atl_mxmm_avll_engn_prnt_trq: 100.0,   // Full torque available
            eec2_estimated_pumping_percent_torque: -5.0, // Small pumping loss at idle

            // EEC3 Electronic Engine Controller 3 defaults
            eec3_nominal_friction_percent_torque: 15.0,   // Typical friction torque
            eec3_engine_s_desired_operating_speed: 800.0, // Idle speed desired
            eec3_es_dsd_opt_spd_ast_adstt: 125,          // Neutral asymmetry (midpoint)
            eec3_estmtd_engn_prst_lsss_prnt_trq: 3.0,   // Small parasitic losses at idle
            eec3_aftrtrtmnt_1_exhst_gs_mss_flw_rt: 100.0, // Moderate exhaust flow
            eec3_engine_exhaust_1_dew_point: 0,           // Not exceeded
            eec3_aftertreatment_1_exhaust_dew_point: 0,   // Not exceeded
            eec3_engine_exhaust_2_dew_point: 0,           // Not exceeded
            eec3_aftertreatment_2_exhaust_dew_point: 0,   // Not exceeded

            // EEC4 Electronic Engine Controller 4 defaults
            eec4_engine_rated_power: 250.0,               // 250 kW rated power
            eec4_engine_rated_speed: 2200.0,              // 2200 rpm rated speed
            eec4_engine_rotation_direction: 1,            // Clockwise
            eec4_engn_intk_mnfld_prssr_cntrl_md: 0,      // Open loop
            eec4_crnk_attmpt_cnt_on_prsnt_strt_attmpt: 0, // No current crank attempt
            eec4_engn_prl_ol_lw_prssr_thrshld: 50.0,     // 50 kPa prelube threshold

            // EEC5 Electronic Engine Controller 5 defaults
            eec5_engn_trhrgr_1_clltd_trn_intk_tmprtr: 400.0, // Typical turbine intake temp
            eec5_engn_trhrgr_1_clltd_trn_otlt_tmprtr: 350.0, // Typical turbine outlet temp
            eec5_engn_exhst_gs_rrltn_1_vlv_1_cntrl_1: 15.0,  // 15% EGR valve opening at idle
            eec5_ev_gt_t_vt_a_ct_st_vv: 0,                    // VGT shutoff valve off
            eec5_engine_fuel_control_mode: 1,                  // Closed loop
            eec5_engn_vrl_gmtr_trhrgr_1_cntrl_md: 1,          // Closed loop
            eec5_engn_vrl_gmtr_trhrgr_vgt_1_attr_pstn: 30.0,  // 30% VGT position at idle

            // EEC6 Electronic Engine Controller 6 defaults
            eec6_engn_trhrgr_cmprssr_bpss_attr_1_cmmnd: 0.0,  // Bypass closed
            eec6_engn_vrl_gmtr_trhrgr_attr_1: 30.0,            // 30% VGT at idle
            eec6_engn_trhrgr_cmprssr_bpss_attr_1_pstn: 0.0,    // Bypass closed
            eec6_engn_trhrgr_cmprssr_bpss_attr_2_cmmnd: 0.0,   // Bypass closed
            eec6_et_cpss_bpss_att_1_dsd_pst: 0.0,              // Desired bypass closed
            eec6_et_cpss_bpss_att_1_pf: 31,                    // Not available
            eec6_et_cpss_bpss_att_1_tpt_stts: 7,               // Not available

            // EEC7 Electronic Engine Controller 7 defaults
            eec7_engn_exhst_gs_rrltn_1_vlv_pstn: 15.0,        // 15% EGR valve open at idle
            eec7_engn_exhst_gs_rrltn_1_vlv_2_pstn: 10.0,      // 10% EGR valve 2 open at idle
            eec7_engn_crnks_brthr_ol_sprtr_spd: 3000,         // Moderate oil separator speed
            eec7_engn_intk_mnfld_cmmndd_prssr: 100.0,         // Atmospheric pressure commanded

            // EEC9 Electronic Engine Controller 9 defaults
            eec9_engn_exhst_gs_rrltn_2_vlv_pstn: 0.0,         // EGR2 valve closed
            eec9_engn_exhst_gs_rrltn_2_vlv_2_pstn: 0.0,       // EGR2 valve 2 closed
            eec9_commanded_engine_fuel_rail_pressure: 30.0,    // 30 MPa idle fuel rail pressure
            eec9_cmmndd_engn_fl_injtn_cntrl_prssr: 25.0,      // 25 MPa idle injection pressure

            // EEC10 Electronic Engine Controller 10 defaults
            eec10_engn_exhst_gs_rrltn_2_clr_intk_tmprtr: 85.0, // 85 degC EGR2 cooler intake temp
            eec10_e_exst_gs_rt_2_c_it_ast_pss: 101.3,          // Atmospheric pressure
            eec10_engn_exhst_gs_rrltn_2_clr_effn: 80.0,        // 80% cooler efficiency
            eec10_e_exst_gs_rt_2_c_bpss_att_pst: 0.0,          // Bypass closed
            eec10_engn_exhst_gs_rrltn_2_clr_intk_prssr: 100.0, // Near atmospheric

            // EEC11 Electronic Engine Controller 11 defaults
            eec11_engn_exhst_gs_rrltn_2_vlv_1_cntrl: 0.0,     // EGR2 valve 1 closed
            eec11_engn_exhst_gs_rrltn_2_vlv_2_cntrl: 0.0,     // EGR2 valve 2 closed
            eec11_engn_exhst_gs_rrltn_2_vlv_1_pstn_errr: 0.0, // No position error
            eec11_engn_exhst_gs_rrltn_2_vlv_2_pstn_errr: 0.0, // No position error

            // EEC13 Electronic Engine Controller 13 defaults
            eec13_feedback_engine_fueling_state: 0,            // Disabled
            eec13_engine_fueling_inhibit_allowed: 0,           // Disabled
            eec13_engn_flng_inht_prvntd_rsn: 0,               // No reason
            eec13_sr_addrss_of_cntrllng_dv_fr_flng_stt: 254, // Not available (0xFE)
            eec13_engine_dual_fuel_mode: 0,                    // Dual fuel off
            eec13_engn_flng_inht_prvntd_rsn_extnsn: 0,        // No reason
            eec13_engn_gs_sstttn_fl_prntg: 0.0,               // No gas substitution
            eec13_engn_flng_inht_rqst_cnt: 0,                 // No inhibit requests
            eec13_engn_flng_dsrd_rqst_cnt: 0,                 // No desired requests
            eec13_engn_prttn_drt_ovrrd_stts: 0,               // No override
            eec13_remaining_engine_motoring_time: 0,           // No motoring time
            eec13_engine_performance_bias_level: 50.0,         // Neutral bias
            eec13_minimum_engine_motoring_speed: 0.0,          // No motoring speed

            // EEC14 Electronic Engine Controller 14 defaults
            eec14_engn_exhst_gs_rrltn_1_vlv_1_pstn_errr: 0.0, // No position error
            eec14_engn_exhst_gs_rrltn_1_vlv_2_pstn_errr: 0.0, // No position error
            eec14_engine_fuel_mass_flow_rate: 2.0,              // Low idle fuel flow
            eec14_fuel_type: 4,                                 // Diesel
            eec14_engine_fuel_isolation_control: 0,             // Off

            // EEC16 Electronic Engine Controller 16 defaults
            eec16_accelerator_pedal_3_position: 0.0,           // Released
            eec16_ready_for_clutch_engagement_status: 1,       // Ready for clutch
            eec16_engine_clutch_engage_request_status: 0,      // No external request

            // EEC18 Electronic Engine Controller 18 defaults
            eec18_engn_clndr_hd_bpss_attr_1_cmmnd: 50.0,     // 50% bypass position
            eec18_engine_intake_air_source_valve: 0,           // Off
            eec18_engn_exhst_gs_rstrtn_vlv_pstn: 0.0,         // Restriction valve closed

            // EEC19 Electronic Engine Controller 19 defaults
            eec19_total_engine_energy: 50000,                  // 50000 kWh accumulated
            eec19_engn_exhst_flw_rt_extndd_rng: 250.0,        // 250 kg/h exhaust flow

            // EEC20 Electronic Engine Controller 20 defaults
            eec20_esttd_e_pst_lsss_pt_tq_h_rst: 3.0,         // 3% parasitic losses
            eec20_atl_mxmm_avll_engn_prnt_fl: 100.0,          // Full fuel available
            eec20_nmnl_frtn_prnt_trq_hgh_rsltn: 15.0,         // 15% friction torque
            eec20_aslt_engn_ld_prnt_ar_mss: 25.0,             // 25% air mass at idle

            // EEC23 Electronic Engine Controller 23 defaults
            eec23_engn_crnks_prssr_cntrl_attr_1_cmmnd: 50.0,  // 50% crankcase pressure control
            eec23_engn_crnks_prssr_cntrl_attr_2_cmmnd: 50.0,  // 50% crankcase pressure control

            // EEC24 Electronic Engine Controller 24 defaults
            eec24_engn_crnks_prssr_cntrl_attr_1_tmprtr: 60.0, // 60 degC actuator temp
            eec24_engn_crnks_prssr_cntrl_attr_1_pstn: 50.0,   // 50% position
            eec24_e_cs_pss_ct_att_1_dsd_pst: 50.0,            // 50% desired position
            eec24_e_cs_pss_ct_att_1_pf: 31,                   // Not available
            eec24_e_cs_pss_ct_att_1_tpt_stts: 7,              // Not available
            eec24_e_cs_pss_ct_att_1_opt_stts: 0,              // Normal

            // EEC25 Electronic Engine Controller 25 defaults
            eec25_engn_crnks_prssr_cntrl_attr_2_tmprtr: 60.0, // 60 degC actuator temp
            eec25_engn_crnks_prssr_cntrl_attr_2_pstn: 50.0,   // 50% position
            eec25_e_cs_pss_ct_att_2_dsd_pst: 50.0,            // 50% desired position
            eec25_e_cs_pss_ct_att_2_pf: 31,                   // Not available
            eec25_e_cs_pss_ct_att_2_tpt_stts: 7,              // Not available
            eec25_e_cs_pss_ct_att_2_opt_stts: 0,              // Normal

            // ETC1 Electronic Transmission Controller 1 defaults
            etc1_transmission_driveline_engaged: 0,            // Disengaged at idle
            etc1_trnsmssn_trq_cnvrtr_lkp_enggd: 0,            // Torque converter unlocked
            etc1_transmission_shift_in_process: 0,             // No shift in progress
            etc1_tsss_tq_cvt_lp_tst_i_pss: 0,                // No lockup transition
            etc1_transmission_output_shaft_speed: 0.0,         // Stationary
            etc1_percent_clutch_slip: 0.0,                     // No clutch slip
            etc1_engine_momentary_overspeed_enable: 0,         // Overspeed not requested
            etc1_progressive_shift_disable: 0,                 // Progressive shift enabled
            etc1_mmntr_engn_mxmm_pwr_enl: 0,                  // Max power not requested
            etc1_transmission_input_shaft_speed: 0.0,          // Stationary
            etc1_s_addss_o_ct_dv_f_tsss_ct: 0,                // No controlling device
        }
    }
}
