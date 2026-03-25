use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_thermal_frames(&self, frames: &mut Vec<CanFrame>, device_id: DeviceId) {
        // HVESSTS1 - HVESS Thermal Management System Status 1 (Phase 2 Pumps)
        let hvessts1 = HVESSTS1 {
            device_id,
            hvss_thrml_mngmnt_sstm_sl_inpt_pwr: self.thermal.hvessts1_system_input_power,
            hvss_t_mt_sst_h_vt_ipt_pw: self.thermal.hvessts1_hv_input_power,
            hvss_thrml_mngmnt_sstm_cmprssr_spd: self.thermal.hvessts1_compressor_speed,
            hvss_thrml_mngmnt_sstm_rltv_hmdt: self.thermal.hvessts1_relative_humidity,
            hvss_thrml_mngmnt_sstm_htr_stts: self.thermal.hvessts1_heater_status,
            hvss_thrml_mngmnt_sstm_hvl_stts: self.thermal.hvessts1_hvil_status,
            hvss_thrml_mngmnt_sstm_md: self.thermal.hvessts1_system_mode,
            hvss_thrml_mngmnt_sstm_clnt_lvl: self.thermal.hvessts1_coolant_level,
            hvss_thrml_mngmnt_sstm_clnt_lvl_fll: self.thermal.hvessts1_coolant_level_full,
        };

        if let Ok((can_id, data)) = hvessts1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSTC1 - HVESS Thermal Management System Temperature Control (Phase 2 Pumps)
        let hvesstc1 = HVESSTC1 {
            device_id,
            hvss_t_mt_sst_it_ct_tpt_rqst: self.thermal.hvesstc1_intake_coolant_temp_request,
            hvss_t_mt_sst_ott_ct_tpt_rqst: self.thermal.hvesstc1_outlet_coolant_temp_request,
            hvss_t_mt_sst_ct_fw_rt_rqst: self.thermal.hvesstc1_coolant_flow_rate_request,
            hvss_thrml_mngmnt_sstm_htr_enl_cmmnd: self.thermal.hvesstc1_heater_enable_command,
            hvss_t_mt_sst_ct_pp_e_cd: self.thermal.hvesstc1_coolant_pump_enable_code,
            hvss_t_mt_sst_cpss_e_cd: self.thermal.hvesstc1_compressor_enable_code,
        };

        if let Ok((can_id, data)) = hvesstc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSTC2 - HVESS Thermal Management System Temperature Control 2 (Phase 2 Pumps)
        let hvesstc2 = HVESSTC2 {
            device_id,
            hvss_t_mt_sst_ct_pp_spd_cd: self.thermal.hvesstc2_pump_speed_command,
            hvss_t_mt_sst_ct_pp_pt_spd_cd: self.thermal.hvesstc2_pump_speed_command_percent,
            hvss_t_mt_sst_cpss_spd_cd: self.thermal.hvesstc2_compressor_speed_command,
            hvss_t_mt_sst_cpss_pt_spd_cd: self
                .thermal
                .hvesstc2_compressor_speed_command_percent,
        };

        if let Ok((can_id, data)) = hvesstc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ETCC3 - Electronic Transmission Controller Clutch 3: Engine Thermal Control
        let etcc3 = ETCC3 {
            device_id,
            et_cpss_bw_att_1_mt_ct_ds: self.thermal.etcc3_etc_bypass_actuator_1,
            engn_trhrgr_wstgt_attr_1_mtr_crrnt_dsl: self
                .thermal
                .etcc3_turbo_wastegate_actuator_1,
            engn_clndr_hd_bpss_attr_1_mtr_crrnt_dsl: self
                .thermal
                .etcc3_cylinder_head_bypass_actuator,
            engn_thrttl_vlv_1_mtr_crrnt_dsl: self.thermal.etcc3_throttle_valve_1,
            et_cpss_bpss_att_1_mt_ct_ds: self.thermal.etcc3_etc_bypass_pass_actuator_1,
            et_cpss_bpss_att_2_mt_ct_ds: self.thermal.etcc3_etc_bypass_pass_actuator_2,
            engn_trhrgr_wstgt_attr_2_mtr_crrnt_dsl: self
                .thermal
                .etcc3_turbo_wastegate_actuator_2,
        };

        if let Ok((can_id, data)) = etcc3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
