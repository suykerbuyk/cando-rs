use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle HVESSTS1 - HVESS Thermal Management System Status 1
    pub(crate) fn handle_hvessts1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // HVESSTS1 - HVESS Thermal Management System Status 1 (Phase 2 Pumps)
        match HVESSTS1::decode(can_id, data) {
            Ok(msg) => {
                self.thermal.hvessts1_system_input_power = msg.hvss_thrml_mngmnt_sstm_sl_inpt_pwr;
                self.thermal.hvessts1_hv_input_power = msg.hvss_t_mt_sst_h_vt_ipt_pw;
                self.thermal.hvessts1_compressor_speed = msg.hvss_thrml_mngmnt_sstm_cmprssr_spd;
                self.thermal.hvessts1_relative_humidity = msg.hvss_thrml_mngmnt_sstm_rltv_hmdt;
                self.thermal.hvessts1_heater_status = msg.hvss_thrml_mngmnt_sstm_htr_stts;
                self.thermal.hvessts1_hvil_status = msg.hvss_thrml_mngmnt_sstm_hvl_stts;
                self.thermal.hvessts1_system_mode = msg.hvss_thrml_mngmnt_sstm_md;
                self.thermal.hvessts1_coolant_level = msg.hvss_thrml_mngmnt_sstm_clnt_lvl;
                self.thermal.hvessts1_coolant_level_full = msg.hvss_thrml_mngmnt_sstm_clnt_lvl_fll;
                println!(
                    "🌡️  Received HVESSTS1: Sys Power={:.1}W, HV Power={:.1}W, Compressor={:.0} rpm, Humidity={:.1}%, Mode={}, Coolant={}",
                    self.thermal.hvessts1_system_input_power,
                    self.thermal.hvessts1_hv_input_power,
                    self.thermal.hvessts1_compressor_speed,
                    self.thermal.hvessts1_relative_humidity,
                    self.thermal.hvessts1_system_mode,
                    self.thermal.hvessts1_coolant_level
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVESSTS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle HVESSTC1 - HVESS Thermal Management System Temperature Control
    pub(crate) fn handle_hvesstc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // HVESSTC1 - HVESS Thermal Management System Temperature Control (Phase 2 Pumps)
        match HVESSTC1::decode(can_id, data) {
            Ok(msg) => {
                self.thermal.hvesstc1_intake_coolant_temp_request =
                    msg.hvss_t_mt_sst_it_ct_tpt_rqst;
                self.thermal.hvesstc1_outlet_coolant_temp_request =
                    msg.hvss_t_mt_sst_ott_ct_tpt_rqst;
                self.thermal.hvesstc1_coolant_flow_rate_request = msg.hvss_t_mt_sst_ct_fw_rt_rqst;
                self.thermal.hvesstc1_heater_enable_command =
                    msg.hvss_thrml_mngmnt_sstm_htr_enl_cmmnd;
                self.thermal.hvesstc1_coolant_pump_enable_code = msg.hvss_t_mt_sst_ct_pp_e_cd;
                self.thermal.hvesstc1_compressor_enable_code = msg.hvss_t_mt_sst_cpss_e_cd;
                println!(
                    "🌡️  Received HVESSTC1: Intake={:.1}°C, Outlet={:.1}°C, Flow={:.1} l/min, Heater={}, Pump={}, Compressor={}",
                    self.thermal.hvesstc1_intake_coolant_temp_request,
                    self.thermal.hvesstc1_outlet_coolant_temp_request,
                    self.thermal.hvesstc1_coolant_flow_rate_request,
                    self.thermal.hvesstc1_heater_enable_command,
                    self.thermal.hvesstc1_coolant_pump_enable_code,
                    self.thermal.hvesstc1_compressor_enable_code
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVESSTC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle HVESSTC2 - HVESS Thermal Management System Temperature Control 2
    pub(crate) fn handle_hvesstc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // HVESSTC2 - HVESS Thermal Management System Temperature Control 2 (Phase 2 Pumps)
        match HVESSTC2::decode(can_id, data) {
            Ok(msg) => {
                self.thermal.hvesstc2_pump_speed_command = msg.hvss_t_mt_sst_ct_pp_spd_cd;
                self.thermal.hvesstc2_pump_speed_command_percent =
                    msg.hvss_t_mt_sst_ct_pp_pt_spd_cd;
                self.thermal.hvesstc2_compressor_speed_command = msg.hvss_t_mt_sst_cpss_spd_cd;
                self.thermal.hvesstc2_compressor_speed_command_percent =
                    msg.hvss_t_mt_sst_cpss_pt_spd_cd;
                println!(
                    "🌡️  Received HVESSTC2: Pump Speed={:.1} rpm ({:.1}%), Compressor Speed={:.1} rpm ({:.1}%)",
                    self.thermal.hvesstc2_pump_speed_command,
                    self.thermal.hvesstc2_pump_speed_command_percent,
                    self.thermal.hvesstc2_compressor_speed_command,
                    self.thermal.hvesstc2_compressor_speed_command_percent
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVESSTC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }
}
