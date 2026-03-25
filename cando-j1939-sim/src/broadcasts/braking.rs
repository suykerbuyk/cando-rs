use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_braking_frames(&self, frames: &mut Vec<CanFrame>, device_id: DeviceId) {
        // AEBS2 - Advanced Emergency Braking System 2 (Simplified - keeping existing fields)
        if self.braking.aebs_enabled {
            let aebs2 = AEBS2 {
                device_id,
                aebs_2_message_counter: (self.uptime_seconds % 16) as u8,
                aebs_2_message_checksum: ((self.uptime_seconds * 7) % 256) as u8,
                dv_atvt_dd_f_advd_eb_sst: if self.braking.aebs_brake_demand > 0.0 {
                    1
                } else {
                    0
                },
            };

            if let Ok((can_id, data)) = aebs2.encode()
                && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
            {
                frames.push(frame);
            }
        }

        // ============================================================================
        // EBC1 - Electronic Brake Controller 1
        // ============================================================================

        let ebc1 = EBC1 {
            device_id,
            asr_engine_control_active: self.braking.ebc1_asr_engine_control_active,
            asr_brake_control_active: self.braking.ebc1_asr_brake_control_active,
            anti_lock_braking_abs_active: self.braking.ebc1_abs_active,
            ebs_brake_switch: self.braking.ebc1_ebs_brake_switch,
            brake_pedal_position: self.braking.ebc1_brake_pedal_position,
            abs_off_road_switch: self.braking.ebc1_abs_off_road_switch,
            asr_off_road_switch: self.braking.ebc1_asr_off_road_switch,
            asr_hill_holder_switch: self.braking.ebc1_asr_hill_holder_switch,
            traction_control_override_switch: self.braking.ebc1_traction_control_override,
            accelerator_interlock_switch: self.braking.ebc1_accelerator_interlock,
            engine_derate_switch: self.braking.ebc1_engine_derate_switch,
            engine_auxiliary_shutdown_switch: self.braking.ebc1_aux_engine_shutdown_switch,
            remote_accelerator_enable_switch: self.braking.ebc1_remote_accel_enable_switch,
            engine_retarder_selection: self.braking.ebc1_engine_retarder_selection,
            abs_fully_operational: self.braking.ebc1_abs_fully_operational,
            ebs_red_warning_signal: self.braking.ebc1_ebs_red_warning,
            as_es_amr_wrnng_sgnl_pwrd_vhl: self.braking.ebc1_abs_ebs_amber_warning,
            atc_asr_information_signal: self.braking.ebc1_atc_asr_information_signal,
            sr_addrss_of_cntrllng_dv_fr_brk_cntrl: self.braking.ebc1_source_address_brake_control,
            railroad_mode_switch: 0,
            halt_brake_switch: self.braking.ebc1_halt_brake_switch,
            trailer_abs_status: self.braking.ebc1_trailer_abs_status,
            trtr_mntd_trlr_as_wrnng_sgnl: self.braking.ebc1_tractor_trailer_abs_warning,
        };

        if let Ok((can_id, data)) = ebc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // EBC2 - Electronic Brake Controller 2 (Wheel Speeds)
        // ============================================================================

        let ebc2 = EBC2 {
            device_id,
            front_axle_speed: self.braking.ebc2_front_axle_speed,
            relative_speed_front_axle_left_wheel: self.braking.ebc2_rel_speed_front_left,
            rltv_spd_frnt_axl_rght_whl: self.braking.ebc2_rel_speed_front_right,
            relative_speed_rear_axle_1_left_wheel: self.braking.ebc2_rel_speed_rear1_left,
            rltv_spd_rr_axl_1_rght_whl: self.braking.ebc2_rel_speed_rear1_right,
            relative_speed_rear_axle_2_left_wheel: self.braking.ebc2_rel_speed_rear2_left,
            rltv_spd_rr_axl_2_rght_whl: self.braking.ebc2_rel_speed_rear2_right,
        };

        if let Ok((can_id, data)) = ebc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // EBC3 - Electronic Brake Controller 3 (Brake Pressures)
        // ============================================================================

        let ebc3 = EBC3 {
            device_id,
            b_appt_pss_hr_ft_ax_lt_w: self.braking.ebc3_pressure_front_left,
            b_appt_pss_hr_ft_ax_rt_w: self.braking.ebc3_pressure_front_right,
            b_appt_pss_hrr_ax_1_lt_w: self.braking.ebc3_pressure_rear1_left,
            b_appt_pss_hrr_ax_1_rt_w: self.braking.ebc3_pressure_rear1_right,
            b_appt_pss_hrr_ax_2_lt_w: self.braking.ebc3_pressure_rear2_left,
            b_appt_pss_hrr_ax_2_rt_w: self.braking.ebc3_pressure_rear2_right,
            b_appt_pss_hrr_ax_3_lt_w: self.braking.ebc3_pressure_rear3_left,
            b_appt_pss_hrr_ax_3_rt_w: self.braking.ebc3_pressure_rear3_right,
        };

        if let Ok((can_id, data)) = ebc3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // EBC4 - Electronic Brake Controller 4 (Brake Lining Axles 1-3)
        // ============================================================================

        let ebc4 = EBC4 {
            device_id,
            brk_lnng_rmnng_frnt_axl_lft_whl: self.braking.ebc4_lining_front_left,
            brk_lnng_rmnng_frnt_axl_rght_whl: self.braking.ebc4_lining_front_right,
            brk_lnng_rmnng_rr_axl_1_lft_whl: self.braking.ebc4_lining_rear1_left,
            brk_lnng_rmnng_rr_axl_1_rght_whl: self.braking.ebc4_lining_rear1_right,
            brk_lnng_rmnng_rr_axl_2_lft_whl: self.braking.ebc4_lining_rear2_left,
            brk_lnng_rmnng_rr_axl_2_rght_whl: self.braking.ebc4_lining_rear2_right,
            brk_lnng_rmnng_rr_axl_3_lft_whl: self.braking.ebc4_lining_rear3_left,
            brk_lnng_rmnng_rr_axl_3_rght_whl: self.braking.ebc4_lining_rear3_right,
        };

        if let Ok((can_id, data)) = ebc4.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // EBC5 - Electronic Brake Controller 5 (Brake Status)
        // ============================================================================

        let ebc5 = EBC5 {
            device_id,
            brake_temperature_warning: self.braking.ebc5_brake_temp_warning,
            halt_brake_mode: self.braking.ebc5_halt_brake_mode,
            hill_holder_mode: self.braking.ebc5_hill_holder_mode,
            foundation_brake_use: self.braking.ebc5_foundation_brake_use,
            xbr_system_state: self.braking.ebc5_xbr_system_state,
            xbr_active_control_mode: self.braking.ebc5_xbr_active_control_mode,
            xbr_acceleration_limit: self.braking.ebc5_xbr_acceleration_limit,
            prkng_brk_attr_fll_atvtd: self.braking.ebc5_parking_brake_actuator,
            emergency_braking_active: self.braking.ebc5_emergency_braking_active,
            railroad_mode: self.braking.ebc5_railroad_mode,
            xbr_brake_hold_mode: self.braking.ebc5_xbr_brake_hold_mode,
            driver_brake_demand: self.braking.ebc5_driver_brake_demand,
            ovrll_intndd_brk_alrtn: self.braking.ebc5_overall_brake_demand,
        };

        if let Ok((can_id, data)) = ebc5.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // EBC6 - Electronic Brake Controller 6 (Brake Lining Axles 4-7)
        // ============================================================================

        let ebc6 = EBC6 {
            device_id,
            brk_lnng_rmnng_rr_axl_4_lft_whl: self.braking.ebc6_lining_rear4_left,
            brk_lnng_rmnng_rr_axl_4_rght_whl: self.braking.ebc6_lining_rear4_right,
            brk_lnng_rmnng_rr_axl_5_lft_whl: self.braking.ebc6_lining_rear5_left,
            brk_lnng_rmnng_rr_axl_5_rght_whl: self.braking.ebc6_lining_rear5_right,
            brk_lnng_rmnng_rr_axl_6_lft_whl: self.braking.ebc6_lining_rear6_left,
            brk_lnng_rmnng_rr_axl_6_rght_whl: self.braking.ebc6_lining_rear6_right,
            brk_lnng_rmnng_rr_axl_7_lft_whl: self.braking.ebc6_lining_rear7_left,
            brk_lnng_rmnng_rr_axl_7_rght_whl: self.braking.ebc6_lining_rear7_right,
        };

        if let Ok((can_id, data)) = ebc6.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // EBC7 - Electronic Brake Controller 7 (Brake Lining Axles 8-10)
        // ============================================================================

        let ebc7 = EBC7 {
            device_id,
            brk_lnng_rmnng_rr_axl_8_lft_whl: self.braking.ebc7_lining_rear8_left,
            brk_lnng_rmnng_rr_axl_8_rght_whl: self.braking.ebc7_lining_rear8_right,
            brk_lnng_rmnng_rr_axl_9_lft_whl: self.braking.ebc7_lining_rear9_left,
            brk_lnng_rmnng_rr_axl_9_rght_whl: self.braking.ebc7_lining_rear9_right,
            brk_lnng_rmnng_rr_axl_10_lft_whl: self.braking.ebc7_lining_rear10_left,
            brk_lnng_rmnng_rr_axl_10_rght_whl: self.braking.ebc7_lining_rear10_right,
        };

        if let Ok((can_id, data)) = ebc7.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // EBCC - Engine Brake Continuous Control
        // ============================================================================

        let ebcc = EBCC {
            device_id,
            engn_trhrgr_1_trn_otlt_prssr: self.braking.ebcc_turbo1_outlet_pressure,
            engn_trhrgr_1_trn_dsrd_otlt_prssr: self.braking.ebcc_turbo1_desired_outlet_pressure,
            engn_exhst_brk_attr_cmmnd: self.braking.ebcc_exhaust_brake_command,
            engn_trhrgr_2_trn_otlt_prssr: self.braking.ebcc_turbo2_outlet_pressure,
            engn_trhrgr_2_trn_dsrd_otlt_prssr: self.braking.ebcc_turbo2_desired_outlet_pressure,
        };

        if let Ok((can_id, data)) = ebcc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // XBR - External Brake Request
        // ============================================================================

        let xbr = XBR {
            device_id,
            external_acceleration_demand: self.braking.xbr_acceleration_demand,
            xbr_ebi_mode: self.braking.xbr_ebi_mode,
            xbr_priority: self.braking.xbr_priority,
            xbr_control_mode: self.braking.xbr_control_mode,
            xbr_compensation_mode: self.braking.xbr_compensation_mode,
            xbr_urgency: self.braking.xbr_urgency,
            xbr_brake_hold_request: self.braking.xbr_brake_hold_request,
            xbr_reason: self.braking.xbr_reason,
            xbr_message_counter: self.braking.xbr_message_counter,
            xbr_message_checksum: self.braking.xbr_message_checksum,
        };

        if let Ok((can_id, data)) = xbr.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // AEBS1 - Advanced Emergency Braking System 1
        // ============================================================================

        let aebs1 = AEBS1 {
            device_id,
            fwd_cs_advd_eb_sst_stt: self.braking.aebs1_forward_collision_status,
            collision_warning_level: self.braking.aebs1_collision_warning_level,
            rvt_ot_dttd_f_advd_eb_sst: self.braking.aebs1_relevant_object_detected,
            bnd_off_prlt_of_rlvnt_ojt: self.braking.aebs1_bound_offset,
            tm_t_cllsn_wth_rlvnt_ojt: self.braking.aebs1_time_to_collision,
            rd_dprtr_advnd_emrgn_brkng_sstm_stt: self.braking.aebs1_road_departure_status,
        };

        if let Ok((can_id, data)) = aebs1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // ACC1 - Adaptive Cruise Control 1
        // ============================================================================

        let acc1 = ACC1 {
            device_id,
            speed_of_forward_vehicle: self.braking.acc1_speed_of_forward_vehicle,
            distance_to_forward_vehicle: self.braking.acc1_distance_to_forward_vehicle,
            adaptive_cruise_control_set_speed: self.braking.acc1_set_speed,
            adaptive_cruise_control_mode: self.braking.acc1_mode,
            adptv_crs_cntrl_st_dstn_md: self.braking.acc1_set_distance_mode,
            road_curvature: self.braking.acc1_road_curvature,
            acc_target_detected: self.braking.acc1_target_detected,
            acc_system_shutoff_warning: self.braking.acc1_system_shutoff_warning,
            acc_distance_alert_signal: self.braking.acc1_distance_alert,
            forward_collision_warning: self.braking.acc1_forward_collision_warning,
        };

        if let Ok((can_id, data)) = acc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // ACC2 - Adaptive Cruise Control 2
        // ============================================================================

        let acc2 = ACC2 {
            device_id,
            acc_usage_demand: self.braking.acc2_usage_demand,
            requested_acc_distance_mode: self.braking.acc2_distance_mode,
        };

        if let Ok((can_id, data)) = acc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // ACCS - Acceleration Sensor
        // ============================================================================

        let accs = ACCS {
            device_id,
            ltrl_alrtn_extndd_rng: self.braking.accs_lateral_acceleration,
            lngtdnl_alrtn_extndd_rng: self.braking.accs_longitudinal_acceleration,
            vrtl_alrtn_extndd_rng: self.braking.accs_vertical_acceleration,
            ltrl_alrtn_fgr_of_mrt_extndd_rng: self.braking.accs_lateral_fom,
            lngtdnl_alrtn_fgr_of_mrt_extndd_rng: self.braking.accs_longitudinal_fom,
            vrtl_alrtn_fgr_of_mrt_extndd_rng: self.braking.accs_vertical_fom,
            sppt_v_tsss_rptt_rt_f_at_ss: self.braking.accs_support_report_rate,
        };

        if let Ok((can_id, data)) = accs.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // ACCVC - Aftercooler Coolant Valve Control
        // ============================================================================

        let accvc = ACCVC {
            device_id,
            engn_aftrlr_clnt_thrmstt_md: self.braking.accvc_aftercooler_thermostat_mode,
            engn_dsrd_aftrlr_clnt_intk_tmprtr: self.braking.accvc_desired_aftercooler_temp,
            engn_dsrd_aftrlr_clnt_thrmstt_opnng: self.braking.accvc_desired_thermostat_opening,
            engn_chrg_ar_clr_bpss_vlv_1_cmmnd: self.braking.accvc_charge_air_bypass_valve1_cmd,
            engn_chrg_ar_clr_bpss_vlv_2_cmmnd: self.braking.accvc_charge_air_bypass_valve2_cmd,
            engn_aftrlr_clnt_dvrtr_vlv_cmmnd: self.braking.accvc_aftercooler_diverter_valve_cmd,
        };

        if let Ok((can_id, data)) = accvc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // ERC1 - Electronic Retarder Controller 1
        // ============================================================================

        let erc1 = ERC1 {
            device_id,
            retarder_torque_mode: self.braking.erc1_retarder_torque_mode,
            retarder_enable_brake_assist_switch: self.braking.erc1_enable_brake_assist,
            retarder_enable_shift_assist_switch: self.braking.erc1_enable_shift_assist,
            actual_retarder_percent_torque: self.braking.erc1_actual_retarder_torque,
            intended_retarder_percent_torque: self.braking.erc1_intended_retarder_torque,
            engine_coolant_load_increase: self.braking.erc1_coolant_load_increase,
            retarder_requesting_brake_light: self.braking.erc1_requesting_brake_light,
            retarder_road_speed_limit_switch: self.braking.erc1_road_speed_limit_switch,
            retarder_road_speed_exceeded_status: self.braking.erc1_road_speed_exceeded,
            s_addss_o_ct_dv_f_rtd_ct: self.braking.erc1_source_address,
            drvrs_dmnd_rtrdr_prnt_trq: self.braking.erc1_drivers_demand_torque,
            retarder_selection_non_engine: self.braking.erc1_selection_non_engine,
            atl_mxmm_avll_rtrdr_prnt_trq: self.braking.erc1_max_available_torque,
        };

        if let Ok((can_id, data)) = erc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // ERC2 - Electronic Retarder Controller 2
        // ============================================================================

        let erc2 = ERC2 {
            device_id,
            transmission_output_retarder: self.braking.erc2_transmission_output_retarder,
            retarder_road_speed_limit_enable: self.braking.erc2_road_speed_limit_enable,
            retarder_road_speed_limit_active: self.braking.erc2_road_speed_limit_active,
            trnsmssn_rtrdr_enl_swth: self.braking.erc2_transmission_retarder_enable,
            crs_cntrl_rtrdr_atv_spd_offst: self.braking.erc2_cruise_control_speed_offset,
            retarder_road_speed_limit_set_speed: self.braking.erc2_road_speed_limit_set_speed,
            retarder_road_speed_limit_readiness: self.braking.erc2_road_speed_limit_readiness,
            retarder_derate_status: self.braking.erc2_retarder_derate_status,
        };

        if let Ok((can_id, data)) = erc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // RC - Retarder Configuration
        // ============================================================================

        let rc = RC {
            device_id,
            retarder_type: self.braking.rc_retarder_type,
            retarder_location: self.braking.rc_retarder_location,
            retarder_control_method: self.braking.rc_control_method,
            retarder_speed_at_idle_point_1: self.braking.rc_speed_at_idle,
            rtrdr_prnt_trq_at_idl_pnt_1: self.braking.rc_torque_at_idle,
            maximum_retarder_speed_point_2: self.braking.rc_max_speed,
            rtrdr_prnt_trq_at_mxmm_spd_pnt_2: self.braking.rc_torque_at_max_speed,
            retarder_speed_at_point_3: self.braking.rc_speed_at_point3,
            retarder_percent_torque_at_point_3: self.braking.rc_torque_at_point3,
            retarder_speed_at_point_4: self.braking.rc_speed_at_point4,
            retarder_percent_torque_at_point_4: self.braking.rc_torque_at_point4,
            retarder_speed_at_peak_torque_point_5: self.braking.rc_speed_at_peak_torque,
            retarder_reference_torque: self.braking.rc_reference_torque,
            rtrdr_prnt_trq_at_pk_trq_pnt_5: self.braking.rc_torque_at_peak,
        };

        if let Ok((can_id, data)) = rc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // LMP - Mast Position
        // ============================================================================

        let lmp = LMP {
            device_id,
            mast_position: self.braking.lmp_mast_position,
        };

        if let Ok((can_id, data)) = lmp.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
