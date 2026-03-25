use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_dcdc_frames(&self, frames: &mut Vec<CanFrame>, device_id: DeviceId) {
        // DCDC1OS - DC/DC Converter 1 Operating Status (Phase 1 Power Supply)
        let dcdc1os = DCDC1OS {
            device_id,
            dc_dc_1_hvil_status: self.dcdc.dcdc1os_hvil_status,
            dc_dc_1_loadshed_request: self.dcdc.dcdc1os_loadshed_request,
            dc_dc_1_operational_status: self.dcdc.dcdc1os_operational_status,
            dc_dc_1_operating_status_counter: self.dcdc.dcdc1os_operating_status_counter,
            dc_dc_1_operating_status_crc: self.dcdc.dcdc1os_operating_status_crc,
            dd_1_pwr_lmt_dt_hgh_sd_crrnt: self.dcdc.dcdc1os_power_limit_high_side_current,
            dd_1_pwr_lmt_dt_lw_sd_crrnt: self.dcdc.dcdc1os_power_limit_low_side_current,
            dd_1_pwr_lmt_dt_hgh_sd_vltg_mnmm: self.dcdc.dcdc1os_power_limit_high_side_voltage_min,
            dd_1_pwr_lmt_dt_hgh_sd_vltg_mxmm: self.dcdc.dcdc1os_power_limit_high_side_voltage_max,
            dd_1_pwr_lmt_dt_lw_sd_vltg_mnmm: self.dcdc.dcdc1os_power_limit_low_side_voltage_min,
            dd_1_pwr_lmt_dt_lw_sd_vltg_mxmm: self.dcdc.dcdc1os_power_limit_low_side_voltage_max,
            dd_1_pwr_lmt_dt_cnvrtr_tmprtr: self.dcdc.dcdc1os_power_limit_converter_temperature,
            dd_1_pwr_lmt_dt_eltrn_fltr_tmprtr: self
                .dcdc
                .dcdc1os_power_limit_electronic_filter_temperature,
            dd_1_pwr_lmt_dt_pwr_eltrns_tmprtr: self
                .dcdc
                .dcdc1os_power_limit_power_electronics_temperature,
            dd_1_pwr_lmt_dt_sl_bttr_trmnl_vltg: self
                .dcdc
                .dcdc1os_power_limit_sli_battery_terminal_voltage,
            dd_1_pwr_lmt_dt_sl_bttr_trmnl_crrnt: self
                .dcdc
                .dcdc1os_power_limit_sli_battery_terminal_current,
            dd_1_pwr_lmt_dt_sl_bttr_trmnl_tmprtr: self
                .dcdc
                .dcdc1os_power_limit_sli_battery_terminal_temperature,
            dd_1_pwr_lmt_dt_undfnd_rsn: self.dcdc.dcdc1os_power_limit_undefined_reason,
        };

        if let Ok((can_id, data)) = dcdc1os.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC1SBS - DC/DC Converter 1 SLI Battery Status (Phase 1 Power Supply)
        let dcdc1sbs = DCDC1SBS {
            device_id,
            dc_dc_1_sli_battery_terminal_current: self.dcdc.dcdc1sbs_terminal_current,
            dc_dc_1_sli_battery_terminal_voltage: self.dcdc.dcdc1sbs_terminal_voltage,
            dd_1_sl_bttr_trmnl_tmprtr: self.dcdc.dcdc1sbs_terminal_temperature,
        };

        if let Ok((can_id, data)) = dcdc1sbs.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC1S2 - DC/DC Converter 1 Status 2 (Phase 1 Power Supply)
        let dcdc1s2 = DCDC1S2 {
            device_id,
            dc_dc_1_high_side_power: self.dcdc.dcdc1s2_high_side_power,
            dc_dc_1_low_side_power: self.dcdc.dcdc1s2_low_side_power,
            dd_1_hgh_sd_ngtv_t_chsss_grnd_vltg: self.dcdc.dcdc1s2_high_side_ground_voltage,
        };

        if let Ok((can_id, data)) = dcdc1s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2SBS - DC/DC Converter 2 SLI Battery Status (Phase 1 Power Supply)
        let dcdc2sbs = DCDC2SBS {
            device_id,
            dc_dc_2_sli_battery_terminal_voltage: self.dcdc.dcdc2sbs_terminal_voltage,
            dc_dc_2_sli_battery_terminal_current: self.dcdc.dcdc2sbs_terminal_current,
            dd_2_sl_bttr_trmnl_tmprtr: self.dcdc.dcdc2sbs_terminal_temperature,
        };

        if let Ok((can_id, data)) = dcdc2sbs.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2S2 - DC/DC Converter 2 Status 2 (Phase 1 Power Supply)
        let dcdc2s2 = DCDC2S2 {
            device_id,
            dc_dc_2_high_side_power: self.dcdc.dcdc2s2_high_side_power,
            dc_dc_2_low_side_power: self.dcdc.dcdc2s2_low_side_power,
            dd_2_hgh_sd_ngtv_t_chsss_grnd_vltg: self.dcdc.dcdc2s2_high_side_ground_voltage,
        };

        if let Ok((can_id, data)) = dcdc2s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // === Batch 9: Extended Power Conversion Broadcasts ===

        // DCDC1HL - DC/DC Converter 1 High Side Limits
        let dcdc1hl = DCDC1HL {
            device_id,
            dd_1_hgh_sd_vltg_mnmm_lmt_rqst: self.dcdc.dcdc1hl_high_side_voltage_min_limit,
            dd_1_hgh_sd_vltg_mxmm_lmt_rqst: self.dcdc.dcdc1hl_high_side_voltage_max_limit,
            dd_1_hgh_sd_crrnt_mxmm_lmt_rqst: self.dcdc.dcdc1hl_high_side_current_max_limit,
            dd_1_hgh_sd_crrnt_mnmm_lmt_rqst: self.dcdc.dcdc1hl_high_side_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc1hl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC1LL - DC/DC Converter 1 Low Side Limits
        let dcdc1ll = DCDC1LL {
            device_id,
            dd_1_lw_sd_vltg_mnmm_lmt_rqst: self.dcdc.dcdc1ll_low_side_voltage_min_limit,
            dd_1_lw_sd_vltg_mxmm_lmt_rqst: self.dcdc.dcdc1ll_low_side_voltage_max_limit,
            dd_1_lw_sd_crrnt_mxmm_lmt_rqst: self.dcdc.dcdc1ll_low_side_current_max_limit,
            dd_1_lw_sd_crrnt_mnmm_lmt_rqst: self.dcdc.dcdc1ll_low_side_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc1ll.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC1T - DC/DC Converter 1 Temperature
        let dcdc1t = DCDC1T {
            device_id,
            dc_dc_1_converter_temperature: self.dcdc.dcdc1t_converter_temperature,
            dd_1_cnvrtr_eltrn_fltr_tmprtr: self.dcdc.dcdc1t_electronic_filter_temperature,
            dd_1_pwr_eltrns_tmprtr: self.dcdc.dcdc1t_power_electronics_temperature,
            dc_dc_1_coolant_in_temperature: self.dcdc.dcdc1t_coolant_in_temperature,
            dc_dc_1_coolant_out_temperature: self.dcdc.dcdc1t_coolant_out_temperature,
        };
        if let Ok((can_id, data)) = dcdc1t.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC1V - DC/DC Converter 1 Voltage
        let dcdc1v = DCDC1V {
            device_id,
            dd_1_cntrllr_inpt_igntn_vltg: self.dcdc.dcdc1v_ignition_voltage,
            dd_1_cntrllr_inpt_unswthd_sl_vltg: self.dcdc.dcdc1v_unswitched_sli_voltage,
        };
        if let Ok((can_id, data)) = dcdc1v.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC1VC - DC/DC Converter 1 Voltage/Current
        let dcdc1vc = DCDC1VC {
            device_id,
            dc_dc_1_low_side_voltage: self.dcdc.dcdc1vc_low_side_voltage,
            dc_dc_1_low_side_current: self.dcdc.dcdc1vc_low_side_current,
            dc_dc_1_high_side_voltage: self.dcdc.dcdc1vc_high_side_voltage,
            dc_dc_1_high_side_current: self.dcdc.dcdc1vc_high_side_current,
        };
        if let Ok((can_id, data)) = dcdc1vc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC1LD - DC/DC Converter 1 Lifetime Data
        let dcdc1ld = DCDC1LD {
            device_id,
            dc_dc_1_total_high_side_energy: self.dcdc.dcdc1ld_total_high_side_energy,
            dc_dc_1_total_low_side_energy: self.dcdc.dcdc1ld_total_low_side_energy,
            dc_dc_1_total_high_side_charge: self.dcdc.dcdc1ld_total_high_side_charge,
            dc_dc_1_total_low_side_charge: self.dcdc.dcdc1ld_total_low_side_charge,
        };
        if let Ok((can_id, data)) = dcdc1ld.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC1SBL - DC/DC Converter 1 SLI Battery Limits
        let dcdc1sbl = DCDC1SBL {
            device_id,
            dd_1_sl_bttr_trmnl_vltg_mxmm_lmt_rqst: self.dcdc.dcdc1sbl_voltage_max_limit,
            dd_1_s_btt_tc_ct_mx_lt_rqst: self.dcdc.dcdc1sbl_current_max_limit,
            dd_1_sl_bttr_tmprtr_mxmm_lmt_rqst: self.dcdc.dcdc1sbl_temperature_max_limit,
        };
        if let Ok((can_id, data)) = dcdc1sbl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC1CFG1 - DC/DC Converter 1 Configuration 1
        let dcdc1cfg1 = DCDC1CFG1 {
            device_id,
            dd_1_hgh_sd_vltg_mnmm_lmt_sttng: self.dcdc.dcdc1cfg1_hs_voltage_min_limit,
            dd_1_hgh_sd_vltg_mxmm_lmt_sttng: self.dcdc.dcdc1cfg1_hs_voltage_max_limit,
            dd_1_hgh_sd_crrnt_mxmm_lmt_sttng: self.dcdc.dcdc1cfg1_hs_current_max_limit,
            dd_1_lw_sd_vltg_mnmm_lmt_sttng: self.dcdc.dcdc1cfg1_ls_voltage_min_limit,
            dd_1_lw_sd_vltg_mxmm_lmt_sttng: self.dcdc.dcdc1cfg1_ls_voltage_max_limit,
            dd_1_lw_sd_crrnt_mxmm_lmt_sttng: self.dcdc.dcdc1cfg1_ls_current_max_limit,
            dd_1_sl_bttr_trmnl_vltg_mxmm_lmt_sttng: self.dcdc.dcdc1cfg1_sli_voltage_max_limit,
            dd_1_s_btt_tc_ct_mx_lt_stt: self.dcdc.dcdc1cfg1_sli_current_max_limit,
            dd_1_sl_bttr_tmprtr_mxmm_lmt_sttng: self.dcdc.dcdc1cfg1_sli_temperature_max_limit,
            dd_1_lw_sd_vltg_bk_dflt_sttng: self.dcdc.dcdc1cfg1_ls_voltage_buck_default,
            dd_1_lw_sd_crrnt_mnmm_lmt_sttng: self.dcdc.dcdc1cfg1_ls_current_min_limit,
            dd_1_hgh_sd_crrnt_mnmm_lmt_sttng: self.dcdc.dcdc1cfg1_hs_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc1cfg1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2C - DC/DC Converter 2 Control
        let dcdc2c = DCDC2C {
            device_id,
            dc_dc_2_operational_command: self.dcdc.dcdc2c_operational_command,
            dc_dc_2_control_counter: self.dcdc.dcdc2c_control_counter,
            dc_dc_2_low_side_voltage_buck_setpoint: self.dcdc.dcdc2c_low_side_voltage_buck_setpoint,
            dd_2_hgh_sd_vltg_bst_stpnt: self.dcdc.dcdc2c_high_side_voltage_boost_setpoint,
            dd_2_lw_sd_vltg_bk_dflt_stpnt: self.dcdc.dcdc2c_low_side_voltage_buck_default_setpoint,
            dc_dc_2_control_crc: self.dcdc.dcdc2c_control_crc,
        };
        if let Ok((can_id, data)) = dcdc2c.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2OS - DC/DC Converter 2 Operating Status
        let dcdc2os = DCDC2OS {
            device_id,
            dc_dc_2_operational_status: self.dcdc.dcdc2os_operational_status,
            dc_dc_2_hvil_status: self.dcdc.dcdc2os_hvil_status,
            dc_dc_2_loadshed_request: self.dcdc.dcdc2os_loadshed_request,
            dd_2_pwr_lmt_dt_hgh_sd_crrnt: self.dcdc.dcdc2os_power_limit_high_side_current,
            dd_2_pwr_lmt_dt_lw_sd_crrnt: self.dcdc.dcdc2os_power_limit_low_side_current,
            dd_2_pwr_lmt_dt_hgh_sd_vltg_mnmm: self.dcdc.dcdc2os_power_limit_high_side_voltage_min,
            dd_2_pwr_lmt_dt_hgh_sd_vltg_mxmm: self.dcdc.dcdc2os_power_limit_high_side_voltage_max,
            dd_2_pwr_lmt_dt_lw_sd_vltg_mnmm: self.dcdc.dcdc2os_power_limit_low_side_voltage_min,
            dd_2_pwr_lmt_dt_lw_sd_vltg_mxmm: self.dcdc.dcdc2os_power_limit_low_side_voltage_max,
            dd_2_pwr_lmt_dt_cnvrtr_tmprtr: self.dcdc.dcdc2os_power_limit_converter_temperature,
            dd_2_pwr_lmt_dt_eltrn_fltr_tmprtr: self.dcdc.dcdc2os_power_limit_electronic_filter_temperature,
            dd_2_pwr_lmt_dt_pwr_eltrns_tmprtr: self.dcdc.dcdc2os_power_limit_power_electronics_temperature,
            dd_2_pwr_lmt_dt_sl_bttr_trmnl_vltg: self.dcdc.dcdc2os_power_limit_sli_battery_terminal_voltage,
            dd_2_pwr_lmt_dt_sl_bttr_trmnl_crrnt: self.dcdc.dcdc2os_power_limit_sli_battery_terminal_current,
            dd_2_pwr_lmt_dt_sl_bttr_trmnl_tmprtr: self.dcdc.dcdc2os_power_limit_sli_battery_terminal_temperature,
            dd_2_pwr_lmt_dt_undfnd_rsn: self.dcdc.dcdc2os_power_limit_undefined_reason,
            dc_dc_2_operating_status_counter: self.dcdc.dcdc2os_operating_status_counter,
            dc_dc_2_operating_status_crc: self.dcdc.dcdc2os_operating_status_crc,
        };
        if let Ok((can_id, data)) = dcdc2os.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2HL - DC/DC Converter 2 High Side Limits
        let dcdc2hl = DCDC2HL {
            device_id,
            dd_2_hgh_sd_vltg_mnmm_lmt_rqst: self.dcdc.dcdc2hl_high_side_voltage_min_limit,
            dd_2_hgh_sd_vltg_mxmm_lmt_rqst: self.dcdc.dcdc2hl_high_side_voltage_max_limit,
            dd_2_hgh_sd_crrnt_mxmm_lmt_rqst: self.dcdc.dcdc2hl_high_side_current_max_limit,
            dd_2_hgh_sd_crrnt_mnmm_lmt_rqst: self.dcdc.dcdc2hl_high_side_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc2hl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2LL - DC/DC Converter 2 Low Side Limits
        let dcdc2ll = DCDC2LL {
            device_id,
            dd_2_lw_sd_vltg_mnmm_lmt_rqst: self.dcdc.dcdc2ll_low_side_voltage_min_limit,
            dd_2_lw_sd_vltg_mxmm_lmt_rqst: self.dcdc.dcdc2ll_low_side_voltage_max_limit,
            dd_2_lw_sd_crrnt_mxmm_lmt_rqst: self.dcdc.dcdc2ll_low_side_current_max_limit,
            dd_2_lw_sd_crrnt_mnmm_lmt_rqst: self.dcdc.dcdc2ll_low_side_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc2ll.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2T - DC/DC Converter 2 Temperature
        let dcdc2t = DCDC2T {
            device_id,
            dc_dc_2_converter_temperature: self.dcdc.dcdc2t_converter_temperature,
            dd_2_cnvrtr_eltrn_fltr_tmprtr: self.dcdc.dcdc2t_electronic_filter_temperature,
            dd_2_pwr_eltrns_tmprtr: self.dcdc.dcdc2t_power_electronics_temperature,
            dc_dc_2_coolant_in_temperature: self.dcdc.dcdc2t_coolant_in_temperature,
            dc_dc_2_coolant_out_temperature: self.dcdc.dcdc2t_coolant_out_temperature,
        };
        if let Ok((can_id, data)) = dcdc2t.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2V - DC/DC Converter 2 Voltage
        let dcdc2v = DCDC2V {
            device_id,
            dd_2_cntrllr_inpt_igntn_vltg: self.dcdc.dcdc2v_ignition_voltage,
            dd_2_cntrllr_inpt_unswthd_sl_vltg: self.dcdc.dcdc2v_unswitched_sli_voltage,
        };
        if let Ok((can_id, data)) = dcdc2v.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2VC - DC/DC Converter 2 Voltage/Current
        let dcdc2vc = DCDC2VC {
            device_id,
            dc_dc_2_low_side_voltage: self.dcdc.dcdc2vc_low_side_voltage,
            dc_dc_2_low_side_current: self.dcdc.dcdc2vc_low_side_current,
            dc_dc_2_high_side_voltage: self.dcdc.dcdc2vc_high_side_voltage,
            dc_dc_2_high_side_current: self.dcdc.dcdc2vc_high_side_current,
        };
        if let Ok((can_id, data)) = dcdc2vc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2LD - DC/DC Converter 2 Lifetime Data
        let dcdc2ld = DCDC2LD {
            device_id,
            dc_dc_2_total_high_side_energy: self.dcdc.dcdc2ld_total_high_side_energy,
            dc_dc_2_total_low_side_energy: self.dcdc.dcdc2ld_total_low_side_energy,
            dc_dc_2_total_high_side_charge: self.dcdc.dcdc2ld_total_high_side_charge,
            dc_dc_2_total_low_side_charge: self.dcdc.dcdc2ld_total_low_side_charge,
        };
        if let Ok((can_id, data)) = dcdc2ld.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2SBL - DC/DC Converter 2 SLI Battery Limits
        let dcdc2sbl = DCDC2SBL {
            device_id,
            dd_2_sl_bttr_trmnl_vltg_mxmm_lmt_rqst: self.dcdc.dcdc2sbl_voltage_max_limit,
            dd_2_s_btt_tc_ct_mx_lt_rqst: self.dcdc.dcdc2sbl_current_max_limit,
            dd_2_sl_bttr_tmprtr_mxmm_lmt_rqst: self.dcdc.dcdc2sbl_temperature_max_limit,
        };
        if let Ok((can_id, data)) = dcdc2sbl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC2CFG1 - DC/DC Converter 2 Configuration 1
        let dcdc2cfg1 = DCDC2CFG1 {
            device_id,
            dd_2_hgh_sd_vltg_mnmm_lmt_sttng: self.dcdc.dcdc2cfg1_hs_voltage_min_limit,
            dd_2_hgh_sd_vltg_mxmm_lmt_sttng: self.dcdc.dcdc2cfg1_hs_voltage_max_limit,
            dd_2_hgh_sd_crrnt_mxmm_lmt_sttng: self.dcdc.dcdc2cfg1_hs_current_max_limit,
            dd_2_lw_sd_vltg_mnmm_lmt_sttng: self.dcdc.dcdc2cfg1_ls_voltage_min_limit,
            dd_2_lw_sd_vltg_mxmm_lmt_sttng: self.dcdc.dcdc2cfg1_ls_voltage_max_limit,
            dd_2_lw_sd_crrnt_mxmm_lmt_sttng: self.dcdc.dcdc2cfg1_ls_current_max_limit,
            dd_2_sl_bttr_trmnl_vltg_mxmm_lmt_sttng: self.dcdc.dcdc2cfg1_sli_voltage_max_limit,
            dd_2_s_btt_tc_ct_mx_lt_stt: self.dcdc.dcdc2cfg1_sli_current_max_limit,
            dd_2_sl_bttr_tmprtr_mxmm_lmt_sttng: self.dcdc.dcdc2cfg1_sli_temperature_max_limit,
            dd_2_lw_sd_vltg_bk_dflt_sttng: self.dcdc.dcdc2cfg1_ls_voltage_buck_default,
            dd_2_lw_sd_crrnt_mnmm_lmt_sttng: self.dcdc.dcdc2cfg1_ls_current_min_limit,
            dd_2_hgh_sd_crrnt_mnmm_lmt_sttng: self.dcdc.dcdc2cfg1_hs_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc2cfg1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3C - DC/DC Converter 3 Control
        let dcdc3c = DCDC3C {
            device_id,
            dc_dc_3_operational_command: self.dcdc.dcdc3c_operational_command,
            dc_dc_3_control_counter: self.dcdc.dcdc3c_control_counter,
            dc_dc_3_low_side_voltage_buck_setpoint: self.dcdc.dcdc3c_low_side_voltage_buck_setpoint,
            dd_3_hgh_sd_vltg_bst_stpnt: self.dcdc.dcdc3c_high_side_voltage_boost_setpoint,
            dd_3_lw_sd_vltg_bk_dflt_stpnt: self.dcdc.dcdc3c_low_side_voltage_buck_default_setpoint,
            dc_dc_3_control_crc: self.dcdc.dcdc3c_control_crc,
        };
        if let Ok((can_id, data)) = dcdc3c.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3OS - DC/DC Converter 3 Operating Status
        let dcdc3os = DCDC3OS {
            device_id,
            dc_dc_3_operational_status: self.dcdc.dcdc3os_operational_status,
            dc_dc_3_hvil_status: self.dcdc.dcdc3os_hvil_status,
            dc_dc_3_loadshed_request: self.dcdc.dcdc3os_loadshed_request,
            dd_3_pwr_lmt_dt_hgh_sd_crrnt: self.dcdc.dcdc3os_power_limit_high_side_current,
            dd_3_pwr_lmt_dt_lw_sd_crrnt: self.dcdc.dcdc3os_power_limit_low_side_current,
            dd_3_pwr_lmt_dt_hgh_sd_vltg_mnmm: self.dcdc.dcdc3os_power_limit_high_side_voltage_min,
            dd_3_pwr_lmt_dt_hgh_sd_vltg_mxmm: self.dcdc.dcdc3os_power_limit_high_side_voltage_max,
            dd_3_pwr_lmt_dt_lw_sd_vltg_mnmm: self.dcdc.dcdc3os_power_limit_low_side_voltage_min,
            dd_3_pwr_lmt_dt_lw_sd_vltg_mxmm: self.dcdc.dcdc3os_power_limit_low_side_voltage_max,
            dd_3_pwr_lmt_dt_cnvrtr_tmprtr: self.dcdc.dcdc3os_power_limit_converter_temperature,
            dd_3_pwr_lmt_dt_eltrn_fltr_tmprtr: self.dcdc.dcdc3os_power_limit_electronic_filter_temperature,
            dd_3_pwr_lmt_dt_pwr_eltrns_tmprtr: self.dcdc.dcdc3os_power_limit_power_electronics_temperature,
            dd_3_pwr_lmt_dt_sl_bttr_trmnl_vltg: self.dcdc.dcdc3os_power_limit_sli_battery_terminal_voltage,
            dd_3_pwr_lmt_dt_sl_bttr_trmnl_crrnt: self.dcdc.dcdc3os_power_limit_sli_battery_terminal_current,
            dd_3_pwr_lmt_dt_sl_bttr_trmnl_tmprtr: self.dcdc.dcdc3os_power_limit_sli_battery_terminal_temperature,
            dd_3_pwr_lmt_dt_undfnd_rsn: self.dcdc.dcdc3os_power_limit_undefined_reason,
            dc_dc_3_operating_status_counter: self.dcdc.dcdc3os_operating_status_counter,
            dc_dc_3_operating_status_crc: self.dcdc.dcdc3os_operating_status_crc,
        };
        if let Ok((can_id, data)) = dcdc3os.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3S2 - DC/DC Converter 3 Status 2
        let dcdc3s2 = DCDC3S2 {
            device_id,
            dc_dc_3_low_side_power: self.dcdc.dcdc3s2_low_side_power,
            dc_dc_3_high_side_power: self.dcdc.dcdc3s2_high_side_power,
            dd_3_hgh_sd_ngtv_t_chsss_grnd_vltg: self.dcdc.dcdc3s2_high_side_ground_voltage,
        };
        if let Ok((can_id, data)) = dcdc3s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3SBS - DC/DC Converter 3 SLI Battery Status
        let dcdc3sbs = DCDC3SBS {
            device_id,
            dc_dc_3_sli_battery_terminal_voltage: self.dcdc.dcdc3sbs_terminal_voltage,
            dc_dc_3_sli_battery_terminal_current: self.dcdc.dcdc3sbs_terminal_current,
            dd_3_sl_bttr_trmnl_tmprtr: self.dcdc.dcdc3sbs_terminal_temperature,
        };
        if let Ok((can_id, data)) = dcdc3sbs.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3T - DC/DC Converter 3 Temperature
        let dcdc3t = DCDC3T {
            device_id,
            dc_dc_3_converter_temperature: self.dcdc.dcdc3t_converter_temperature,
            dd_3_cnvrtr_eltrn_fltr_tmprtr: self.dcdc.dcdc3t_electronic_filter_temperature,
            dd_3_pwr_eltrns_tmprtr: self.dcdc.dcdc3t_power_electronics_temperature,
            dc_dc_3_coolant_in_temperature: self.dcdc.dcdc3t_coolant_in_temperature,
            dc_dc_3_coolant_out_temperature: self.dcdc.dcdc3t_coolant_out_temperature,
        };
        if let Ok((can_id, data)) = dcdc3t.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3V - DC/DC Converter 3 Voltage
        let dcdc3v = DCDC3V {
            device_id,
            dd_3_cntrllr_inpt_igntn_vltg: self.dcdc.dcdc3v_ignition_voltage,
            dd_3_cntrllr_inpt_unswthd_sl_vltg: self.dcdc.dcdc3v_unswitched_sli_voltage,
        };
        if let Ok((can_id, data)) = dcdc3v.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3VC - DC/DC Converter 3 Voltage/Current
        let dcdc3vc = DCDC3VC {
            device_id,
            dc_dc_3_low_side_voltage: self.dcdc.dcdc3vc_low_side_voltage,
            dc_dc_3_low_side_current: self.dcdc.dcdc3vc_low_side_current,
            dc_dc_3_high_side_voltage: self.dcdc.dcdc3vc_high_side_voltage,
            dc_dc_3_high_side_current: self.dcdc.dcdc3vc_high_side_current,
        };
        if let Ok((can_id, data)) = dcdc3vc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3SBL - DC/DC Converter 3 SLI Battery Limits
        let dcdc3sbl = DCDC3SBL {
            device_id,
            dd_3_sl_bttr_trmnl_vltg_mxmm_lmt_rqst: self.dcdc.dcdc3sbl_voltage_max_limit,
            dd_3_s_btt_tc_ct_mx_lt_rqst: self.dcdc.dcdc3sbl_current_max_limit,
            dd_3_sl_bttr_tmprtr_mxmm_lmt_rqst: self.dcdc.dcdc3sbl_temperature_max_limit,
        };
        if let Ok((can_id, data)) = dcdc3sbl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3LL - DC/DC Converter 3 Low Side Limits
        let dcdc3ll = DCDC3LL {
            device_id,
            dd_3_lw_sd_vltg_mnmm_lmt_rqst: self.dcdc.dcdc3ll_low_side_voltage_min_limit,
            dd_3_lw_sd_vltg_mxmm_lmt_rqst: self.dcdc.dcdc3ll_low_side_voltage_max_limit,
            dd_3_lw_sd_crrnt_mxmm_lmt_rqst: self.dcdc.dcdc3ll_low_side_current_max_limit,
            dd_3_lw_sd_crrnt_mnmm_lmt_rqst: self.dcdc.dcdc3ll_low_side_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc3ll.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3HL - DC/DC Converter 3 High Side Limits
        let dcdc3hl = DCDC3HL {
            device_id,
            dd_3_hgh_sd_vltg_mnmm_lmt_rqst: self.dcdc.dcdc3hl_high_side_voltage_min_limit,
            dd_3_hgh_sd_vltg_mxmm_lmt_rqst: self.dcdc.dcdc3hl_high_side_voltage_max_limit,
            dd_3_hgh_sd_crrnt_mxmm_lmt_rqst: self.dcdc.dcdc3hl_high_side_current_max_limit,
            dd_3_hgh_sd_crrnt_mnmm_lmt_rqst: self.dcdc.dcdc3hl_high_side_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc3hl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3LD - DC/DC Converter 3 Lifetime Data
        let dcdc3ld = DCDC3LD {
            device_id,
            dc_dc_3_total_high_side_energy: self.dcdc.dcdc3ld_total_high_side_energy,
            dc_dc_3_total_low_side_energy: self.dcdc.dcdc3ld_total_low_side_energy,
            dc_dc_3_total_high_side_charge: self.dcdc.dcdc3ld_total_high_side_charge,
            dc_dc_3_total_low_side_charge: self.dcdc.dcdc3ld_total_low_side_charge,
        };
        if let Ok((can_id, data)) = dcdc3ld.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC3CFG1 - DC/DC Converter 3 Configuration 1
        let dcdc3cfg1 = DCDC3CFG1 {
            device_id,
            dd_3_hgh_sd_vltg_mnmm_lmt_sttng: self.dcdc.dcdc3cfg1_hs_voltage_min_limit,
            dd_3_hgh_sd_vltg_mxmm_lmt_sttng: self.dcdc.dcdc3cfg1_hs_voltage_max_limit,
            dd_3_hgh_sd_crrnt_mxmm_lmt_sttng: self.dcdc.dcdc3cfg1_hs_current_max_limit,
            dd_3_lw_sd_vltg_mnmm_lmt_sttng: self.dcdc.dcdc3cfg1_ls_voltage_min_limit,
            dd_3_lw_sd_vltg_mxmm_lmt_sttng: self.dcdc.dcdc3cfg1_ls_voltage_max_limit,
            dd_3_lw_sd_crrnt_mxmm_lmt_sttng: self.dcdc.dcdc3cfg1_ls_current_max_limit,
            dd_3_sl_bttr_trmnl_vltg_mxmm_lmt_sttng: self.dcdc.dcdc3cfg1_sli_voltage_max_limit,
            dd_3_s_btt_tc_ct_mx_lt_stt: self.dcdc.dcdc3cfg1_sli_current_max_limit,
            dd_3_sl_bttr_tmprtr_mxmm_lmt_sttng: self.dcdc.dcdc3cfg1_sli_temperature_max_limit,
            dd_3_lw_sd_vltg_bk_dflt_sttng: self.dcdc.dcdc3cfg1_ls_voltage_buck_default,
            dd_3_lw_sd_crrnt_mnmm_lmt_sttng: self.dcdc.dcdc3cfg1_ls_current_min_limit,
            dd_3_hgh_sd_crrnt_mnmm_lmt_sttng: self.dcdc.dcdc3cfg1_hs_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc3cfg1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4C - DC/DC Converter 4 Control
        let dcdc4c = DCDC4C {
            device_id,
            dc_dc_4_operational_command: self.dcdc.dcdc4c_operational_command,
            dc_dc_4_control_counter: self.dcdc.dcdc4c_control_counter,
            dc_dc_4_low_side_voltage_buck_setpoint: self.dcdc.dcdc4c_low_side_voltage_buck_setpoint,
            dd_4_hgh_sd_vltg_bst_stpnt: self.dcdc.dcdc4c_high_side_voltage_boost_setpoint,
            dd_4_lw_sd_vltg_bk_dflt_stpnt: self.dcdc.dcdc4c_low_side_voltage_buck_default_setpoint,
            dc_dc_4_control_crc: self.dcdc.dcdc4c_control_crc,
        };
        if let Ok((can_id, data)) = dcdc4c.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4OS - DC/DC Converter 4 Operating Status
        let dcdc4os = DCDC4OS {
            device_id,
            dc_dc_4_operational_status: self.dcdc.dcdc4os_operational_status,
            dc_dc_4_hvil_status: self.dcdc.dcdc4os_hvil_status,
            dc_dc_4_loadshed_request: self.dcdc.dcdc4os_loadshed_request,
            dd_4_pwr_lmt_dt_hgh_sd_crrnt: self.dcdc.dcdc4os_power_limit_high_side_current,
            dd_4_pwr_lmt_dt_lw_sd_crrnt: self.dcdc.dcdc4os_power_limit_low_side_current,
            dd_4_pwr_lmt_dt_hgh_sd_vltg_mnmm: self.dcdc.dcdc4os_power_limit_high_side_voltage_min,
            dd_4_pwr_lmt_dt_hgh_sd_vltg_mxmm: self.dcdc.dcdc4os_power_limit_high_side_voltage_max,
            dd_4_pwr_lmt_dt_lw_sd_vltg_mnmm: self.dcdc.dcdc4os_power_limit_low_side_voltage_min,
            dd_4_pwr_lmt_dt_lw_sd_vltg_mxmm: self.dcdc.dcdc4os_power_limit_low_side_voltage_max,
            dd_4_pwr_lmt_dt_cnvrtr_tmprtr: self.dcdc.dcdc4os_power_limit_converter_temperature,
            dd_4_pwr_lmt_dt_eltrn_fltr_tmprtr: self.dcdc.dcdc4os_power_limit_electronic_filter_temperature,
            dd_4_pwr_lmt_dt_pwr_eltrns_tmprtr: self.dcdc.dcdc4os_power_limit_power_electronics_temperature,
            dd_4_pwr_lmt_dt_sl_bttr_trmnl_vltg: self.dcdc.dcdc4os_power_limit_sli_battery_terminal_voltage,
            dd_4_pwr_lmt_dt_sl_bttr_trmnl_crrnt: self.dcdc.dcdc4os_power_limit_sli_battery_terminal_current,
            dd_4_pwr_lmt_dt_sl_bttr_trmnl_tmprtr: self.dcdc.dcdc4os_power_limit_sli_battery_terminal_temperature,
            dd_4_pwr_lmt_dt_undfnd_rsn: self.dcdc.dcdc4os_power_limit_undefined_reason,
            dc_dc_4_operating_status_counter: self.dcdc.dcdc4os_operating_status_counter,
            dc_dc_4_operating_status_crc: self.dcdc.dcdc4os_operating_status_crc,
        };
        if let Ok((can_id, data)) = dcdc4os.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4S2 - DC/DC Converter 4 Status 2
        let dcdc4s2 = DCDC4S2 {
            device_id,
            dc_dc_4_low_side_power: self.dcdc.dcdc4s2_low_side_power,
            dc_dc_4_high_side_power: self.dcdc.dcdc4s2_high_side_power,
            dd_4_hgh_sd_ngtv_t_chsss_grnd_vltg: self.dcdc.dcdc4s2_high_side_ground_voltage,
        };
        if let Ok((can_id, data)) = dcdc4s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4SBS - DC/DC Converter 4 SLI Battery Status
        let dcdc4sbs = DCDC4SBS {
            device_id,
            dc_dc_4_sli_battery_terminal_voltage: self.dcdc.dcdc4sbs_terminal_voltage,
            dc_dc_4_sli_battery_terminal_current: self.dcdc.dcdc4sbs_terminal_current,
            dd_4_sl_bttr_trmnl_tmprtr: self.dcdc.dcdc4sbs_terminal_temperature,
        };
        if let Ok((can_id, data)) = dcdc4sbs.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4T - DC/DC Converter 4 Temperature
        let dcdc4t = DCDC4T {
            device_id,
            dc_dc_4_converter_temperature: self.dcdc.dcdc4t_converter_temperature,
            dd_4_cnvrtr_eltrn_fltr_tmprtr: self.dcdc.dcdc4t_electronic_filter_temperature,
            dd_4_pwr_eltrns_tmprtr: self.dcdc.dcdc4t_power_electronics_temperature,
            dc_dc_4_coolant_in_temperature: self.dcdc.dcdc4t_coolant_in_temperature,
            dc_dc_4_coolant_out_temperature: self.dcdc.dcdc4t_coolant_out_temperature,
        };
        if let Ok((can_id, data)) = dcdc4t.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4V - DC/DC Converter 4 Voltage
        let dcdc4v = DCDC4V {
            device_id,
            dd_4_cntrllr_inpt_igntn_vltg: self.dcdc.dcdc4v_ignition_voltage,
            dd_4_cntrllr_inpt_unswthd_sl_vltg: self.dcdc.dcdc4v_unswitched_sli_voltage,
        };
        if let Ok((can_id, data)) = dcdc4v.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4VC - DC/DC Converter 4 Voltage/Current
        let dcdc4vc = DCDC4VC {
            device_id,
            dc_dc_4_low_side_voltage: self.dcdc.dcdc4vc_low_side_voltage,
            dc_dc_4_low_side_current: self.dcdc.dcdc4vc_low_side_current,
            dc_dc_4_high_side_voltage: self.dcdc.dcdc4vc_high_side_voltage,
            dc_dc_4_high_side_current: self.dcdc.dcdc4vc_high_side_current,
        };
        if let Ok((can_id, data)) = dcdc4vc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4SBL - DC/DC Converter 4 SLI Battery Limits
        let dcdc4sbl = DCDC4SBL {
            device_id,
            dd_4_sl_bttr_trmnl_vltg_mxmm_lmt_rqst: self.dcdc.dcdc4sbl_voltage_max_limit,
            dd_4_s_btt_tc_ct_mx_lt_rqst: self.dcdc.dcdc4sbl_current_max_limit,
            dd_4_sl_bttr_tmprtr_mxmm_lmt_rqst: self.dcdc.dcdc4sbl_temperature_max_limit,
        };
        if let Ok((can_id, data)) = dcdc4sbl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4LL - DC/DC Converter 4 Low Side Limits
        let dcdc4ll = DCDC4LL {
            device_id,
            dd_4_lw_sd_vltg_mnmm_lmt_rqst: self.dcdc.dcdc4ll_low_side_voltage_min_limit,
            dd_4_lw_sd_vltg_mxmm_lmt_rqst: self.dcdc.dcdc4ll_low_side_voltage_max_limit,
            dd_4_lw_sd_crrnt_mxmm_lmt_rqst: self.dcdc.dcdc4ll_low_side_current_max_limit,
            dd_4_lw_sd_crrnt_mnmm_lmt_rqst: self.dcdc.dcdc4ll_low_side_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc4ll.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4HL - DC/DC Converter 4 High Side Limits
        let dcdc4hl = DCDC4HL {
            device_id,
            dd_4_hgh_sd_vltg_mnmm_lmt_rqst: self.dcdc.dcdc4hl_high_side_voltage_min_limit,
            dd_4_hgh_sd_vltg_mxmm_lmt_rqst: self.dcdc.dcdc4hl_high_side_voltage_max_limit,
            dd_4_hgh_sd_crrnt_mxmm_lmt_rqst: self.dcdc.dcdc4hl_high_side_current_max_limit,
            dd_4_hgh_sd_crrnt_mnmm_lmt_rqst: self.dcdc.dcdc4hl_high_side_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc4hl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4LD - DC/DC Converter 4 Lifetime Data
        let dcdc4ld = DCDC4LD {
            device_id,
            dc_dc_4_total_high_side_energy: self.dcdc.dcdc4ld_total_high_side_energy,
            dc_dc_4_total_low_side_energy: self.dcdc.dcdc4ld_total_low_side_energy,
            dc_dc_4_total_high_side_charge: self.dcdc.dcdc4ld_total_high_side_charge,
            dc_dc_4_total_low_side_charge: self.dcdc.dcdc4ld_total_low_side_charge,
        };
        if let Ok((can_id, data)) = dcdc4ld.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCDC4CFG1 - DC/DC Converter 4 Configuration 1
        let dcdc4cfg1 = DCDC4CFG1 {
            device_id,
            dd_4_hgh_sd_vltg_mnmm_lmt_sttng: self.dcdc.dcdc4cfg1_hs_voltage_min_limit,
            dd_4_hgh_sd_vltg_mxmm_lmt_sttng: self.dcdc.dcdc4cfg1_hs_voltage_max_limit,
            dd_4_hgh_sd_crrnt_mxmm_lmt_sttng: self.dcdc.dcdc4cfg1_hs_current_max_limit,
            dd_4_lw_sd_vltg_mnmm_lmt_sttng: self.dcdc.dcdc4cfg1_ls_voltage_min_limit,
            dd_4_lw_sd_vltg_mxmm_lmt_sttng: self.dcdc.dcdc4cfg1_ls_voltage_max_limit,
            dd_4_lw_sd_crrnt_mxmm_lmt_sttng: self.dcdc.dcdc4cfg1_ls_current_max_limit,
            dd_4_sl_bttr_trmnl_vltg_mxmm_lmt_sttng: self.dcdc.dcdc4cfg1_sli_voltage_max_limit,
            dd_4_s_btt_tc_ct_mx_lt_stt: self.dcdc.dcdc4cfg1_sli_current_max_limit,
            dd_4_sl_bttr_tmprtr_mxmm_lmt_sttng: self.dcdc.dcdc4cfg1_sli_temperature_max_limit,
            dd_4_lw_sd_vltg_bk_dflt_sttng: self.dcdc.dcdc4cfg1_ls_voltage_buck_default,
            dd_4_lw_sd_crrnt_mnmm_lmt_sttng: self.dcdc.dcdc4cfg1_ls_current_min_limit,
            dd_4_hgh_sd_crrnt_mnmm_lmt_sttng: self.dcdc.dcdc4cfg1_hs_current_min_limit,
        };
        if let Ok((can_id, data)) = dcdc4cfg1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
