use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle DCDC1C - DC-DC Converter 1 Control
    pub(crate) fn handle_dcdc1c(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DCDC1C - DC-DC Converter 1 Control
        // Note: CAN frames are masked with CAN_EFF_MASK before matching
        match DCDC1C::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc_operational_command = msg.dc_dc_1_operational_command;
                self.dcdc.dcdc_low_side_voltage_setpoint =
                    msg.dc_dc_1_low_side_voltage_buck_setpoint;
                self.dcdc.dcdc_high_side_voltage_setpoint = msg.dd_1_hgh_sd_vltg_bst_stpnt;
                println!(
                    "⚡ Received DCDC1C: Op command = {}, Low V = {:.1}V, High V = {:.1}V",
                    self.dcdc.dcdc_operational_command,
                    self.dcdc.dcdc_low_side_voltage_setpoint,
                    self.dcdc.dcdc_high_side_voltage_setpoint
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DCDC1C: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DCDC1OS - DC/DC Converter 1 Operating Status
    pub(crate) fn handle_dcdc1os(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DCDC1OS - DC/DC Converter 1 Operating Status (Phase 1 Power Supply)
        match DCDC1OS::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1os_hvil_status = msg.dc_dc_1_hvil_status;
                self.dcdc.dcdc1os_loadshed_request = msg.dc_dc_1_loadshed_request;
                self.dcdc.dcdc1os_operational_status = msg.dc_dc_1_operational_status;
                self.dcdc.dcdc1os_operating_status_counter =
                    msg.dc_dc_1_operating_status_counter;
                self.dcdc.dcdc1os_operating_status_crc = msg.dc_dc_1_operating_status_crc;
                self.dcdc.dcdc1os_power_limit_high_side_current =
                    msg.dd_1_pwr_lmt_dt_hgh_sd_crrnt;
                self.dcdc.dcdc1os_power_limit_low_side_current = msg.dd_1_pwr_lmt_dt_lw_sd_crrnt;
                self.dcdc.dcdc1os_power_limit_high_side_voltage_min =
                    msg.dd_1_pwr_lmt_dt_hgh_sd_vltg_mnmm;
                self.dcdc.dcdc1os_power_limit_high_side_voltage_max =
                    msg.dd_1_pwr_lmt_dt_hgh_sd_vltg_mxmm;
                self.dcdc.dcdc1os_power_limit_low_side_voltage_min =
                    msg.dd_1_pwr_lmt_dt_lw_sd_vltg_mnmm;
                self.dcdc.dcdc1os_power_limit_low_side_voltage_max =
                    msg.dd_1_pwr_lmt_dt_lw_sd_vltg_mxmm;
                self.dcdc.dcdc1os_power_limit_converter_temperature =
                    msg.dd_1_pwr_lmt_dt_cnvrtr_tmprtr;
                self.dcdc.dcdc1os_power_limit_electronic_filter_temperature =
                    msg.dd_1_pwr_lmt_dt_eltrn_fltr_tmprtr;
                self.dcdc.dcdc1os_power_limit_power_electronics_temperature =
                    msg.dd_1_pwr_lmt_dt_pwr_eltrns_tmprtr;
                self.dcdc.dcdc1os_power_limit_sli_battery_terminal_voltage =
                    msg.dd_1_pwr_lmt_dt_sl_bttr_trmnl_vltg;
                self.dcdc.dcdc1os_power_limit_sli_battery_terminal_current =
                    msg.dd_1_pwr_lmt_dt_sl_bttr_trmnl_crrnt;
                self.dcdc.dcdc1os_power_limit_sli_battery_terminal_temperature =
                    msg.dd_1_pwr_lmt_dt_sl_bttr_trmnl_tmprtr;
                self.dcdc.dcdc1os_power_limit_undefined_reason = msg.dd_1_pwr_lmt_dt_undfnd_rsn;
                println!(
                    "⚡ Received DCDC1OS: Op={}, HVIL={}, Load={}, Counter={}, CRC={}",
                    self.dcdc.dcdc1os_operational_status,
                    self.dcdc.dcdc1os_hvil_status,
                    self.dcdc.dcdc1os_loadshed_request,
                    self.dcdc.dcdc1os_operating_status_counter,
                    self.dcdc.dcdc1os_operating_status_crc
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DCDC1OS: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DCDC1SBS - DC/DC Converter 1 SLI Battery Status
    pub(crate) fn handle_dcdc1sbs(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DCDC1SBS - DC/DC Converter 1 SLI Battery Status (Phase 1 Power Supply)
        match DCDC1SBS::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1sbs_terminal_current = msg.dc_dc_1_sli_battery_terminal_current;
                self.dcdc.dcdc1sbs_terminal_voltage = msg.dc_dc_1_sli_battery_terminal_voltage;
                self.dcdc.dcdc1sbs_terminal_temperature = msg.dd_1_sl_bttr_trmnl_tmprtr;
                println!(
                    "🔋 Received DCDC1SBS: Current={:.1}A, Voltage={:.1}V, Temp={:.1}°C",
                    self.dcdc.dcdc1sbs_terminal_current,
                    self.dcdc.dcdc1sbs_terminal_voltage,
                    self.dcdc.dcdc1sbs_terminal_temperature
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DCDC1SBS: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DCDC1S2 - DC/DC Converter 1 Status 2
    pub(crate) fn handle_dcdc1s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DCDC1S2 - DC/DC Converter 1 Status 2 (Phase 1 Power Supply)
        match DCDC1S2::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1s2_high_side_power = msg.dc_dc_1_high_side_power;
                self.dcdc.dcdc1s2_low_side_power = msg.dc_dc_1_low_side_power;
                self.dcdc.dcdc1s2_high_side_ground_voltage =
                    msg.dd_1_hgh_sd_ngtv_t_chsss_grnd_vltg;
                println!(
                    "⚡ Received DCDC1S2: HS Power={:.1}kW, LS Power={:.1}kW, Ground={:.1}V",
                    self.dcdc.dcdc1s2_high_side_power,
                    self.dcdc.dcdc1s2_low_side_power,
                    self.dcdc.dcdc1s2_high_side_ground_voltage
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DCDC1S2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DCDC2SBS - DC/DC Converter 2 SLI Battery Status
    pub(crate) fn handle_dcdc2sbs(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DCDC2SBS - DC/DC Converter 2 SLI Battery Status (Phase 1 Power Supply)
        match DCDC2SBS::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2sbs_terminal_voltage = msg.dc_dc_2_sli_battery_terminal_voltage;
                self.dcdc.dcdc2sbs_terminal_current = msg.dc_dc_2_sli_battery_terminal_current;
                self.dcdc.dcdc2sbs_terminal_temperature = msg.dd_2_sl_bttr_trmnl_tmprtr;
                println!(
                    "🔋 Received DCDC2SBS: Voltage={:.1}V, Current={:.1}A, Temp={:.1}°C",
                    self.dcdc.dcdc2sbs_terminal_voltage,
                    self.dcdc.dcdc2sbs_terminal_current,
                    self.dcdc.dcdc2sbs_terminal_temperature
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DCDC2SBS: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DCDC2S2 - DC/DC Converter 2 Status 2
    pub(crate) fn handle_dcdc2s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DCDC2S2 - DC/DC Converter 2 Status 2 (Phase 1 Power Supply)
        match DCDC2S2::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2s2_high_side_power = msg.dc_dc_2_high_side_power;
                self.dcdc.dcdc2s2_low_side_power = msg.dc_dc_2_low_side_power;
                self.dcdc.dcdc2s2_high_side_ground_voltage =
                    msg.dd_2_hgh_sd_ngtv_t_chsss_grnd_vltg;
                println!(
                    "⚡ Received DCDC2S2: HS Power={:.1}kW, LS Power={:.1}kW, Ground={:.1}V",
                    self.dcdc.dcdc2s2_high_side_power,
                    self.dcdc.dcdc2s2_low_side_power,
                    self.dcdc.dcdc2s2_high_side_ground_voltage
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DCDC2S2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    // === Batch 9: Extended Power Conversion Handlers ===

    /// Handle DCDC1HL - DC/DC Converter 1 High Side Limits
    pub(crate) fn handle_dcdc1hl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC1HL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1hl_high_side_voltage_min_limit = msg.dd_1_hgh_sd_vltg_mnmm_lmt_rqst;
                self.dcdc.dcdc1hl_high_side_voltage_max_limit = msg.dd_1_hgh_sd_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc1hl_high_side_current_max_limit = msg.dd_1_hgh_sd_crrnt_mxmm_lmt_rqst;
                self.dcdc.dcdc1hl_high_side_current_min_limit = msg.dd_1_hgh_sd_crrnt_mnmm_lmt_rqst;
                println!("⚡ Received DCDC1HL: HS VMin={:.1}V, VMax={:.1}V", self.dcdc.dcdc1hl_high_side_voltage_min_limit, self.dcdc.dcdc1hl_high_side_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC1HL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC1LL - DC/DC Converter 1 Low Side Limits
    pub(crate) fn handle_dcdc1ll(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC1LL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1ll_low_side_voltage_min_limit = msg.dd_1_lw_sd_vltg_mnmm_lmt_rqst;
                self.dcdc.dcdc1ll_low_side_voltage_max_limit = msg.dd_1_lw_sd_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc1ll_low_side_current_max_limit = msg.dd_1_lw_sd_crrnt_mxmm_lmt_rqst;
                self.dcdc.dcdc1ll_low_side_current_min_limit = msg.dd_1_lw_sd_crrnt_mnmm_lmt_rqst;
                println!("⚡ Received DCDC1LL: LS VMin={:.1}V, VMax={:.1}V", self.dcdc.dcdc1ll_low_side_voltage_min_limit, self.dcdc.dcdc1ll_low_side_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC1LL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC1T - DC/DC Converter 1 Temperature
    pub(crate) fn handle_dcdc1t(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC1T::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1t_converter_temperature = msg.dc_dc_1_converter_temperature;
                self.dcdc.dcdc1t_electronic_filter_temperature = msg.dd_1_cnvrtr_eltrn_fltr_tmprtr;
                self.dcdc.dcdc1t_power_electronics_temperature = msg.dd_1_pwr_eltrns_tmprtr;
                self.dcdc.dcdc1t_coolant_in_temperature = msg.dc_dc_1_coolant_in_temperature;
                self.dcdc.dcdc1t_coolant_out_temperature = msg.dc_dc_1_coolant_out_temperature;
                println!("🌡️  Received DCDC1T: Conv={:.1}°C, Coolant In={:.1}°C", self.dcdc.dcdc1t_converter_temperature, self.dcdc.dcdc1t_coolant_in_temperature);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC1T: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC1V - DC/DC Converter 1 Voltage
    pub(crate) fn handle_dcdc1v(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC1V::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1v_ignition_voltage = msg.dd_1_cntrllr_inpt_igntn_vltg;
                self.dcdc.dcdc1v_unswitched_sli_voltage = msg.dd_1_cntrllr_inpt_unswthd_sl_vltg;
                println!("⚡ Received DCDC1V: Ign={:.1}V, Unswitched={:.1}V", self.dcdc.dcdc1v_ignition_voltage, self.dcdc.dcdc1v_unswitched_sli_voltage);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC1V: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC1VC - DC/DC Converter 1 Voltage/Current
    pub(crate) fn handle_dcdc1vc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC1VC::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1vc_low_side_voltage = msg.dc_dc_1_low_side_voltage;
                self.dcdc.dcdc1vc_low_side_current = msg.dc_dc_1_low_side_current;
                self.dcdc.dcdc1vc_high_side_voltage = msg.dc_dc_1_high_side_voltage;
                self.dcdc.dcdc1vc_high_side_current = msg.dc_dc_1_high_side_current;
                println!("⚡ Received DCDC1VC: LS={:.1}V/{:.1}A, HS={:.1}V/{:.1}A", self.dcdc.dcdc1vc_low_side_voltage, self.dcdc.dcdc1vc_low_side_current, self.dcdc.dcdc1vc_high_side_voltage, self.dcdc.dcdc1vc_high_side_current);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC1VC: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC1LD - DC/DC Converter 1 Lifetime Data
    pub(crate) fn handle_dcdc1ld(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC1LD::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1ld_total_high_side_energy = msg.dc_dc_1_total_high_side_energy;
                self.dcdc.dcdc1ld_total_low_side_energy = msg.dc_dc_1_total_low_side_energy;
                self.dcdc.dcdc1ld_total_high_side_charge = msg.dc_dc_1_total_high_side_charge;
                self.dcdc.dcdc1ld_total_low_side_charge = msg.dc_dc_1_total_low_side_charge;
                println!("⚡ Received DCDC1LD: HS Energy={:.0}, LS Energy={:.0}", self.dcdc.dcdc1ld_total_high_side_energy, self.dcdc.dcdc1ld_total_low_side_energy);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC1LD: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC1SBL - DC/DC Converter 1 SLI Battery Limits
    pub(crate) fn handle_dcdc1sbl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC1SBL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1sbl_voltage_max_limit = msg.dd_1_sl_bttr_trmnl_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc1sbl_current_max_limit = msg.dd_1_s_btt_tc_ct_mx_lt_rqst;
                self.dcdc.dcdc1sbl_temperature_max_limit = msg.dd_1_sl_bttr_tmprtr_mxmm_lmt_rqst;
                println!("⚡ Received DCDC1SBL: VMax={:.1}V, IMax={:.1}A", self.dcdc.dcdc1sbl_voltage_max_limit, self.dcdc.dcdc1sbl_current_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC1SBL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC1CFG1 - DC/DC Converter 1 Configuration 1
    pub(crate) fn handle_dcdc1cfg1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC1CFG1::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc1cfg1_hs_voltage_min_limit = msg.dd_1_hgh_sd_vltg_mnmm_lmt_sttng;
                self.dcdc.dcdc1cfg1_hs_voltage_max_limit = msg.dd_1_hgh_sd_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc1cfg1_hs_current_max_limit = msg.dd_1_hgh_sd_crrnt_mxmm_lmt_sttng;
                self.dcdc.dcdc1cfg1_ls_voltage_min_limit = msg.dd_1_lw_sd_vltg_mnmm_lmt_sttng;
                self.dcdc.dcdc1cfg1_ls_voltage_max_limit = msg.dd_1_lw_sd_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc1cfg1_ls_current_max_limit = msg.dd_1_lw_sd_crrnt_mxmm_lmt_sttng;
                self.dcdc.dcdc1cfg1_sli_voltage_max_limit = msg.dd_1_sl_bttr_trmnl_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc1cfg1_sli_current_max_limit = msg.dd_1_s_btt_tc_ct_mx_lt_stt;
                self.dcdc.dcdc1cfg1_sli_temperature_max_limit = msg.dd_1_sl_bttr_tmprtr_mxmm_lmt_sttng;
                self.dcdc.dcdc1cfg1_ls_voltage_buck_default = msg.dd_1_lw_sd_vltg_bk_dflt_sttng;
                self.dcdc.dcdc1cfg1_ls_current_min_limit = msg.dd_1_lw_sd_crrnt_mnmm_lmt_sttng;
                self.dcdc.dcdc1cfg1_hs_current_min_limit = msg.dd_1_hgh_sd_crrnt_mnmm_lmt_sttng;
                println!("⚙️  Received DCDC1CFG1: HS V=[{:.1},{:.1}]", self.dcdc.dcdc1cfg1_hs_voltage_min_limit, self.dcdc.dcdc1cfg1_hs_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC1CFG1: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2C - DC/DC Converter 2 Control
    pub(crate) fn handle_dcdc2c(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2C::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2c_operational_command = msg.dc_dc_2_operational_command;
                self.dcdc.dcdc2c_control_counter = msg.dc_dc_2_control_counter;
                self.dcdc.dcdc2c_low_side_voltage_buck_setpoint = msg.dc_dc_2_low_side_voltage_buck_setpoint;
                self.dcdc.dcdc2c_high_side_voltage_boost_setpoint = msg.dd_2_hgh_sd_vltg_bst_stpnt;
                self.dcdc.dcdc2c_low_side_voltage_buck_default_setpoint = msg.dd_2_lw_sd_vltg_bk_dflt_stpnt;
                self.dcdc.dcdc2c_control_crc = msg.dc_dc_2_control_crc;
                println!("⚡ Received DCDC2C: Op={}, LS V={:.1}V, HS V={:.1}V", self.dcdc.dcdc2c_operational_command, self.dcdc.dcdc2c_low_side_voltage_buck_setpoint, self.dcdc.dcdc2c_high_side_voltage_boost_setpoint);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2C: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2OS - DC/DC Converter 2 Operating Status
    pub(crate) fn handle_dcdc2os(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2OS::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2os_operational_status = msg.dc_dc_2_operational_status;
                self.dcdc.dcdc2os_hvil_status = msg.dc_dc_2_hvil_status;
                self.dcdc.dcdc2os_loadshed_request = msg.dc_dc_2_loadshed_request;
                self.dcdc.dcdc2os_power_limit_high_side_current = msg.dd_2_pwr_lmt_dt_hgh_sd_crrnt;
                self.dcdc.dcdc2os_power_limit_low_side_current = msg.dd_2_pwr_lmt_dt_lw_sd_crrnt;
                self.dcdc.dcdc2os_power_limit_high_side_voltage_min = msg.dd_2_pwr_lmt_dt_hgh_sd_vltg_mnmm;
                self.dcdc.dcdc2os_power_limit_high_side_voltage_max = msg.dd_2_pwr_lmt_dt_hgh_sd_vltg_mxmm;
                self.dcdc.dcdc2os_power_limit_low_side_voltage_min = msg.dd_2_pwr_lmt_dt_lw_sd_vltg_mnmm;
                self.dcdc.dcdc2os_power_limit_low_side_voltage_max = msg.dd_2_pwr_lmt_dt_lw_sd_vltg_mxmm;
                self.dcdc.dcdc2os_power_limit_converter_temperature = msg.dd_2_pwr_lmt_dt_cnvrtr_tmprtr;
                self.dcdc.dcdc2os_power_limit_electronic_filter_temperature = msg.dd_2_pwr_lmt_dt_eltrn_fltr_tmprtr;
                self.dcdc.dcdc2os_power_limit_power_electronics_temperature = msg.dd_2_pwr_lmt_dt_pwr_eltrns_tmprtr;
                self.dcdc.dcdc2os_power_limit_sli_battery_terminal_voltage = msg.dd_2_pwr_lmt_dt_sl_bttr_trmnl_vltg;
                self.dcdc.dcdc2os_power_limit_sli_battery_terminal_current = msg.dd_2_pwr_lmt_dt_sl_bttr_trmnl_crrnt;
                self.dcdc.dcdc2os_power_limit_sli_battery_terminal_temperature = msg.dd_2_pwr_lmt_dt_sl_bttr_trmnl_tmprtr;
                self.dcdc.dcdc2os_power_limit_undefined_reason = msg.dd_2_pwr_lmt_dt_undfnd_rsn;
                self.dcdc.dcdc2os_operating_status_counter = msg.dc_dc_2_operating_status_counter;
                self.dcdc.dcdc2os_operating_status_crc = msg.dc_dc_2_operating_status_crc;
                println!("⚡ Received DCDC2OS: Op={}, HVIL={}", self.dcdc.dcdc2os_operational_status, self.dcdc.dcdc2os_hvil_status);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2OS: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2HL - DC/DC Converter 2 High Side Limits
    pub(crate) fn handle_dcdc2hl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2HL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2hl_high_side_voltage_min_limit = msg.dd_2_hgh_sd_vltg_mnmm_lmt_rqst;
                self.dcdc.dcdc2hl_high_side_voltage_max_limit = msg.dd_2_hgh_sd_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc2hl_high_side_current_max_limit = msg.dd_2_hgh_sd_crrnt_mxmm_lmt_rqst;
                self.dcdc.dcdc2hl_high_side_current_min_limit = msg.dd_2_hgh_sd_crrnt_mnmm_lmt_rqst;
                println!("⚡ Received DCDC2HL: HS VMin={:.1}V, VMax={:.1}V", self.dcdc.dcdc2hl_high_side_voltage_min_limit, self.dcdc.dcdc2hl_high_side_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2HL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2LL - DC/DC Converter 2 Low Side Limits
    pub(crate) fn handle_dcdc2ll(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2LL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2ll_low_side_voltage_min_limit = msg.dd_2_lw_sd_vltg_mnmm_lmt_rqst;
                self.dcdc.dcdc2ll_low_side_voltage_max_limit = msg.dd_2_lw_sd_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc2ll_low_side_current_max_limit = msg.dd_2_lw_sd_crrnt_mxmm_lmt_rqst;
                self.dcdc.dcdc2ll_low_side_current_min_limit = msg.dd_2_lw_sd_crrnt_mnmm_lmt_rqst;
                println!("⚡ Received DCDC2LL: LS VMin={:.1}V, VMax={:.1}V", self.dcdc.dcdc2ll_low_side_voltage_min_limit, self.dcdc.dcdc2ll_low_side_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2LL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2T - DC/DC Converter 2 Temperature
    pub(crate) fn handle_dcdc2t(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2T::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2t_converter_temperature = msg.dc_dc_2_converter_temperature;
                self.dcdc.dcdc2t_electronic_filter_temperature = msg.dd_2_cnvrtr_eltrn_fltr_tmprtr;
                self.dcdc.dcdc2t_power_electronics_temperature = msg.dd_2_pwr_eltrns_tmprtr;
                self.dcdc.dcdc2t_coolant_in_temperature = msg.dc_dc_2_coolant_in_temperature;
                self.dcdc.dcdc2t_coolant_out_temperature = msg.dc_dc_2_coolant_out_temperature;
                println!("🌡️  Received DCDC2T: Conv={:.1}°C", self.dcdc.dcdc2t_converter_temperature);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2T: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2V - DC/DC Converter 2 Voltage
    pub(crate) fn handle_dcdc2v(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2V::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2v_ignition_voltage = msg.dd_2_cntrllr_inpt_igntn_vltg;
                self.dcdc.dcdc2v_unswitched_sli_voltage = msg.dd_2_cntrllr_inpt_unswthd_sl_vltg;
                println!("⚡ Received DCDC2V: Ign={:.1}V, Unswitched={:.1}V", self.dcdc.dcdc2v_ignition_voltage, self.dcdc.dcdc2v_unswitched_sli_voltage);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2V: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2VC - DC/DC Converter 2 Voltage/Current
    pub(crate) fn handle_dcdc2vc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2VC::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2vc_low_side_voltage = msg.dc_dc_2_low_side_voltage;
                self.dcdc.dcdc2vc_low_side_current = msg.dc_dc_2_low_side_current;
                self.dcdc.dcdc2vc_high_side_voltage = msg.dc_dc_2_high_side_voltage;
                self.dcdc.dcdc2vc_high_side_current = msg.dc_dc_2_high_side_current;
                println!("⚡ Received DCDC2VC: LS={:.1}V/{:.1}A, HS={:.1}V/{:.1}A", self.dcdc.dcdc2vc_low_side_voltage, self.dcdc.dcdc2vc_low_side_current, self.dcdc.dcdc2vc_high_side_voltage, self.dcdc.dcdc2vc_high_side_current);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2VC: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2LD - DC/DC Converter 2 Lifetime Data
    pub(crate) fn handle_dcdc2ld(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2LD::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2ld_total_high_side_energy = msg.dc_dc_2_total_high_side_energy;
                self.dcdc.dcdc2ld_total_low_side_energy = msg.dc_dc_2_total_low_side_energy;
                self.dcdc.dcdc2ld_total_high_side_charge = msg.dc_dc_2_total_high_side_charge;
                self.dcdc.dcdc2ld_total_low_side_charge = msg.dc_dc_2_total_low_side_charge;
                println!("⚡ Received DCDC2LD: HS Energy={:.0}", self.dcdc.dcdc2ld_total_high_side_energy);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2LD: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2SBL - DC/DC Converter 2 SLI Battery Limits
    pub(crate) fn handle_dcdc2sbl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2SBL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2sbl_voltage_max_limit = msg.dd_2_sl_bttr_trmnl_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc2sbl_current_max_limit = msg.dd_2_s_btt_tc_ct_mx_lt_rqst;
                self.dcdc.dcdc2sbl_temperature_max_limit = msg.dd_2_sl_bttr_tmprtr_mxmm_lmt_rqst;
                println!("⚡ Received DCDC2SBL: VMax={:.1}V", self.dcdc.dcdc2sbl_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2SBL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC2CFG1 - DC/DC Converter 2 Configuration 1
    pub(crate) fn handle_dcdc2cfg1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC2CFG1::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc2cfg1_hs_voltage_min_limit = msg.dd_2_hgh_sd_vltg_mnmm_lmt_sttng;
                self.dcdc.dcdc2cfg1_hs_voltage_max_limit = msg.dd_2_hgh_sd_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc2cfg1_hs_current_max_limit = msg.dd_2_hgh_sd_crrnt_mxmm_lmt_sttng;
                self.dcdc.dcdc2cfg1_ls_voltage_min_limit = msg.dd_2_lw_sd_vltg_mnmm_lmt_sttng;
                self.dcdc.dcdc2cfg1_ls_voltage_max_limit = msg.dd_2_lw_sd_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc2cfg1_ls_current_max_limit = msg.dd_2_lw_sd_crrnt_mxmm_lmt_sttng;
                self.dcdc.dcdc2cfg1_sli_voltage_max_limit = msg.dd_2_sl_bttr_trmnl_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc2cfg1_sli_current_max_limit = msg.dd_2_s_btt_tc_ct_mx_lt_stt;
                self.dcdc.dcdc2cfg1_sli_temperature_max_limit = msg.dd_2_sl_bttr_tmprtr_mxmm_lmt_sttng;
                self.dcdc.dcdc2cfg1_ls_voltage_buck_default = msg.dd_2_lw_sd_vltg_bk_dflt_sttng;
                self.dcdc.dcdc2cfg1_ls_current_min_limit = msg.dd_2_lw_sd_crrnt_mnmm_lmt_sttng;
                self.dcdc.dcdc2cfg1_hs_current_min_limit = msg.dd_2_hgh_sd_crrnt_mnmm_lmt_sttng;
                println!("⚙️  Received DCDC2CFG1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC2CFG1: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3C - DC/DC Converter 3 Control
    pub(crate) fn handle_dcdc3c(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3C::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3c_operational_command = msg.dc_dc_3_operational_command;
                self.dcdc.dcdc3c_control_counter = msg.dc_dc_3_control_counter;
                self.dcdc.dcdc3c_low_side_voltage_buck_setpoint = msg.dc_dc_3_low_side_voltage_buck_setpoint;
                self.dcdc.dcdc3c_high_side_voltage_boost_setpoint = msg.dd_3_hgh_sd_vltg_bst_stpnt;
                self.dcdc.dcdc3c_low_side_voltage_buck_default_setpoint = msg.dd_3_lw_sd_vltg_bk_dflt_stpnt;
                self.dcdc.dcdc3c_control_crc = msg.dc_dc_3_control_crc;
                println!("⚡ Received DCDC3C: Op={}, LS V={:.1}V", self.dcdc.dcdc3c_operational_command, self.dcdc.dcdc3c_low_side_voltage_buck_setpoint);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3C: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3OS - DC/DC Converter 3 Operating Status
    pub(crate) fn handle_dcdc3os(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3OS::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3os_operational_status = msg.dc_dc_3_operational_status;
                self.dcdc.dcdc3os_hvil_status = msg.dc_dc_3_hvil_status;
                self.dcdc.dcdc3os_loadshed_request = msg.dc_dc_3_loadshed_request;
                self.dcdc.dcdc3os_power_limit_high_side_current = msg.dd_3_pwr_lmt_dt_hgh_sd_crrnt;
                self.dcdc.dcdc3os_power_limit_low_side_current = msg.dd_3_pwr_lmt_dt_lw_sd_crrnt;
                self.dcdc.dcdc3os_power_limit_high_side_voltage_min = msg.dd_3_pwr_lmt_dt_hgh_sd_vltg_mnmm;
                self.dcdc.dcdc3os_power_limit_high_side_voltage_max = msg.dd_3_pwr_lmt_dt_hgh_sd_vltg_mxmm;
                self.dcdc.dcdc3os_power_limit_low_side_voltage_min = msg.dd_3_pwr_lmt_dt_lw_sd_vltg_mnmm;
                self.dcdc.dcdc3os_power_limit_low_side_voltage_max = msg.dd_3_pwr_lmt_dt_lw_sd_vltg_mxmm;
                self.dcdc.dcdc3os_power_limit_converter_temperature = msg.dd_3_pwr_lmt_dt_cnvrtr_tmprtr;
                self.dcdc.dcdc3os_power_limit_electronic_filter_temperature = msg.dd_3_pwr_lmt_dt_eltrn_fltr_tmprtr;
                self.dcdc.dcdc3os_power_limit_power_electronics_temperature = msg.dd_3_pwr_lmt_dt_pwr_eltrns_tmprtr;
                self.dcdc.dcdc3os_power_limit_sli_battery_terminal_voltage = msg.dd_3_pwr_lmt_dt_sl_bttr_trmnl_vltg;
                self.dcdc.dcdc3os_power_limit_sli_battery_terminal_current = msg.dd_3_pwr_lmt_dt_sl_bttr_trmnl_crrnt;
                self.dcdc.dcdc3os_power_limit_sli_battery_terminal_temperature = msg.dd_3_pwr_lmt_dt_sl_bttr_trmnl_tmprtr;
                self.dcdc.dcdc3os_power_limit_undefined_reason = msg.dd_3_pwr_lmt_dt_undfnd_rsn;
                self.dcdc.dcdc3os_operating_status_counter = msg.dc_dc_3_operating_status_counter;
                self.dcdc.dcdc3os_operating_status_crc = msg.dc_dc_3_operating_status_crc;
                println!("⚡ Received DCDC3OS: Op={}, HVIL={}", self.dcdc.dcdc3os_operational_status, self.dcdc.dcdc3os_hvil_status);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3OS: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3S2 - DC/DC Converter 3 Status 2
    pub(crate) fn handle_dcdc3s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3S2::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3s2_low_side_power = msg.dc_dc_3_low_side_power;
                self.dcdc.dcdc3s2_high_side_power = msg.dc_dc_3_high_side_power;
                self.dcdc.dcdc3s2_high_side_ground_voltage = msg.dd_3_hgh_sd_ngtv_t_chsss_grnd_vltg;
                println!("⚡ Received DCDC3S2: HS={:.1}kW, LS={:.1}kW", self.dcdc.dcdc3s2_high_side_power, self.dcdc.dcdc3s2_low_side_power);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3S2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3SBS - DC/DC Converter 3 SLI Battery Status
    pub(crate) fn handle_dcdc3sbs(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3SBS::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3sbs_terminal_voltage = msg.dc_dc_3_sli_battery_terminal_voltage;
                self.dcdc.dcdc3sbs_terminal_current = msg.dc_dc_3_sli_battery_terminal_current;
                self.dcdc.dcdc3sbs_terminal_temperature = msg.dd_3_sl_bttr_trmnl_tmprtr;
                println!("🔋 Received DCDC3SBS: V={:.1}V, I={:.1}A", self.dcdc.dcdc3sbs_terminal_voltage, self.dcdc.dcdc3sbs_terminal_current);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3SBS: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3T - DC/DC Converter 3 Temperature
    pub(crate) fn handle_dcdc3t(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3T::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3t_converter_temperature = msg.dc_dc_3_converter_temperature;
                self.dcdc.dcdc3t_electronic_filter_temperature = msg.dd_3_cnvrtr_eltrn_fltr_tmprtr;
                self.dcdc.dcdc3t_power_electronics_temperature = msg.dd_3_pwr_eltrns_tmprtr;
                self.dcdc.dcdc3t_coolant_in_temperature = msg.dc_dc_3_coolant_in_temperature;
                self.dcdc.dcdc3t_coolant_out_temperature = msg.dc_dc_3_coolant_out_temperature;
                println!("🌡️  Received DCDC3T: Conv={:.1}°C", self.dcdc.dcdc3t_converter_temperature);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3T: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3V - DC/DC Converter 3 Voltage
    pub(crate) fn handle_dcdc3v(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3V::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3v_ignition_voltage = msg.dd_3_cntrllr_inpt_igntn_vltg;
                self.dcdc.dcdc3v_unswitched_sli_voltage = msg.dd_3_cntrllr_inpt_unswthd_sl_vltg;
                println!("⚡ Received DCDC3V: Ign={:.1}V, Unswitched={:.1}V", self.dcdc.dcdc3v_ignition_voltage, self.dcdc.dcdc3v_unswitched_sli_voltage);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3V: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3VC - DC/DC Converter 3 Voltage/Current
    pub(crate) fn handle_dcdc3vc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3VC::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3vc_low_side_voltage = msg.dc_dc_3_low_side_voltage;
                self.dcdc.dcdc3vc_low_side_current = msg.dc_dc_3_low_side_current;
                self.dcdc.dcdc3vc_high_side_voltage = msg.dc_dc_3_high_side_voltage;
                self.dcdc.dcdc3vc_high_side_current = msg.dc_dc_3_high_side_current;
                println!("⚡ Received DCDC3VC: LS={:.1}V/{:.1}A, HS={:.1}V/{:.1}A", self.dcdc.dcdc3vc_low_side_voltage, self.dcdc.dcdc3vc_low_side_current, self.dcdc.dcdc3vc_high_side_voltage, self.dcdc.dcdc3vc_high_side_current);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3VC: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3SBL - DC/DC Converter 3 SLI Battery Limits
    pub(crate) fn handle_dcdc3sbl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3SBL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3sbl_voltage_max_limit = msg.dd_3_sl_bttr_trmnl_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc3sbl_current_max_limit = msg.dd_3_s_btt_tc_ct_mx_lt_rqst;
                self.dcdc.dcdc3sbl_temperature_max_limit = msg.dd_3_sl_bttr_tmprtr_mxmm_lmt_rqst;
                println!("⚡ Received DCDC3SBL: VMax={:.1}V", self.dcdc.dcdc3sbl_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3SBL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3LL - DC/DC Converter 3 Low Side Limits
    pub(crate) fn handle_dcdc3ll(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3LL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3ll_low_side_voltage_min_limit = msg.dd_3_lw_sd_vltg_mnmm_lmt_rqst;
                self.dcdc.dcdc3ll_low_side_voltage_max_limit = msg.dd_3_lw_sd_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc3ll_low_side_current_max_limit = msg.dd_3_lw_sd_crrnt_mxmm_lmt_rqst;
                self.dcdc.dcdc3ll_low_side_current_min_limit = msg.dd_3_lw_sd_crrnt_mnmm_lmt_rqst;
                println!("⚡ Received DCDC3LL: LS VMin={:.1}V, VMax={:.1}V", self.dcdc.dcdc3ll_low_side_voltage_min_limit, self.dcdc.dcdc3ll_low_side_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3LL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3HL - DC/DC Converter 3 High Side Limits
    pub(crate) fn handle_dcdc3hl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3HL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3hl_high_side_voltage_min_limit = msg.dd_3_hgh_sd_vltg_mnmm_lmt_rqst;
                self.dcdc.dcdc3hl_high_side_voltage_max_limit = msg.dd_3_hgh_sd_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc3hl_high_side_current_max_limit = msg.dd_3_hgh_sd_crrnt_mxmm_lmt_rqst;
                self.dcdc.dcdc3hl_high_side_current_min_limit = msg.dd_3_hgh_sd_crrnt_mnmm_lmt_rqst;
                println!("⚡ Received DCDC3HL: HS VMin={:.1}V, VMax={:.1}V", self.dcdc.dcdc3hl_high_side_voltage_min_limit, self.dcdc.dcdc3hl_high_side_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3HL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3LD - DC/DC Converter 3 Lifetime Data
    pub(crate) fn handle_dcdc3ld(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3LD::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3ld_total_high_side_energy = msg.dc_dc_3_total_high_side_energy;
                self.dcdc.dcdc3ld_total_low_side_energy = msg.dc_dc_3_total_low_side_energy;
                self.dcdc.dcdc3ld_total_high_side_charge = msg.dc_dc_3_total_high_side_charge;
                self.dcdc.dcdc3ld_total_low_side_charge = msg.dc_dc_3_total_low_side_charge;
                println!("⚡ Received DCDC3LD: HS Energy={:.0}", self.dcdc.dcdc3ld_total_high_side_energy);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3LD: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC3CFG1 - DC/DC Converter 3 Configuration 1
    pub(crate) fn handle_dcdc3cfg1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC3CFG1::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc3cfg1_hs_voltage_min_limit = msg.dd_3_hgh_sd_vltg_mnmm_lmt_sttng;
                self.dcdc.dcdc3cfg1_hs_voltage_max_limit = msg.dd_3_hgh_sd_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc3cfg1_hs_current_max_limit = msg.dd_3_hgh_sd_crrnt_mxmm_lmt_sttng;
                self.dcdc.dcdc3cfg1_ls_voltage_min_limit = msg.dd_3_lw_sd_vltg_mnmm_lmt_sttng;
                self.dcdc.dcdc3cfg1_ls_voltage_max_limit = msg.dd_3_lw_sd_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc3cfg1_ls_current_max_limit = msg.dd_3_lw_sd_crrnt_mxmm_lmt_sttng;
                self.dcdc.dcdc3cfg1_sli_voltage_max_limit = msg.dd_3_sl_bttr_trmnl_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc3cfg1_sli_current_max_limit = msg.dd_3_s_btt_tc_ct_mx_lt_stt;
                self.dcdc.dcdc3cfg1_sli_temperature_max_limit = msg.dd_3_sl_bttr_tmprtr_mxmm_lmt_sttng;
                self.dcdc.dcdc3cfg1_ls_voltage_buck_default = msg.dd_3_lw_sd_vltg_bk_dflt_sttng;
                self.dcdc.dcdc3cfg1_ls_current_min_limit = msg.dd_3_lw_sd_crrnt_mnmm_lmt_sttng;
                self.dcdc.dcdc3cfg1_hs_current_min_limit = msg.dd_3_hgh_sd_crrnt_mnmm_lmt_sttng;
                println!("⚙️  Received DCDC3CFG1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC3CFG1: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4C - DC/DC Converter 4 Control
    pub(crate) fn handle_dcdc4c(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4C::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4c_operational_command = msg.dc_dc_4_operational_command;
                self.dcdc.dcdc4c_control_counter = msg.dc_dc_4_control_counter;
                self.dcdc.dcdc4c_low_side_voltage_buck_setpoint = msg.dc_dc_4_low_side_voltage_buck_setpoint;
                self.dcdc.dcdc4c_high_side_voltage_boost_setpoint = msg.dd_4_hgh_sd_vltg_bst_stpnt;
                self.dcdc.dcdc4c_low_side_voltage_buck_default_setpoint = msg.dd_4_lw_sd_vltg_bk_dflt_stpnt;
                self.dcdc.dcdc4c_control_crc = msg.dc_dc_4_control_crc;
                println!("⚡ Received DCDC4C: Op={}, LS V={:.1}V", self.dcdc.dcdc4c_operational_command, self.dcdc.dcdc4c_low_side_voltage_buck_setpoint);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4C: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4OS - DC/DC Converter 4 Operating Status
    pub(crate) fn handle_dcdc4os(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4OS::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4os_operational_status = msg.dc_dc_4_operational_status;
                self.dcdc.dcdc4os_hvil_status = msg.dc_dc_4_hvil_status;
                self.dcdc.dcdc4os_loadshed_request = msg.dc_dc_4_loadshed_request;
                self.dcdc.dcdc4os_power_limit_high_side_current = msg.dd_4_pwr_lmt_dt_hgh_sd_crrnt;
                self.dcdc.dcdc4os_power_limit_low_side_current = msg.dd_4_pwr_lmt_dt_lw_sd_crrnt;
                self.dcdc.dcdc4os_power_limit_high_side_voltage_min = msg.dd_4_pwr_lmt_dt_hgh_sd_vltg_mnmm;
                self.dcdc.dcdc4os_power_limit_high_side_voltage_max = msg.dd_4_pwr_lmt_dt_hgh_sd_vltg_mxmm;
                self.dcdc.dcdc4os_power_limit_low_side_voltage_min = msg.dd_4_pwr_lmt_dt_lw_sd_vltg_mnmm;
                self.dcdc.dcdc4os_power_limit_low_side_voltage_max = msg.dd_4_pwr_lmt_dt_lw_sd_vltg_mxmm;
                self.dcdc.dcdc4os_power_limit_converter_temperature = msg.dd_4_pwr_lmt_dt_cnvrtr_tmprtr;
                self.dcdc.dcdc4os_power_limit_electronic_filter_temperature = msg.dd_4_pwr_lmt_dt_eltrn_fltr_tmprtr;
                self.dcdc.dcdc4os_power_limit_power_electronics_temperature = msg.dd_4_pwr_lmt_dt_pwr_eltrns_tmprtr;
                self.dcdc.dcdc4os_power_limit_sli_battery_terminal_voltage = msg.dd_4_pwr_lmt_dt_sl_bttr_trmnl_vltg;
                self.dcdc.dcdc4os_power_limit_sli_battery_terminal_current = msg.dd_4_pwr_lmt_dt_sl_bttr_trmnl_crrnt;
                self.dcdc.dcdc4os_power_limit_sli_battery_terminal_temperature = msg.dd_4_pwr_lmt_dt_sl_bttr_trmnl_tmprtr;
                self.dcdc.dcdc4os_power_limit_undefined_reason = msg.dd_4_pwr_lmt_dt_undfnd_rsn;
                self.dcdc.dcdc4os_operating_status_counter = msg.dc_dc_4_operating_status_counter;
                self.dcdc.dcdc4os_operating_status_crc = msg.dc_dc_4_operating_status_crc;
                println!("⚡ Received DCDC4OS: Op={}, HVIL={}", self.dcdc.dcdc4os_operational_status, self.dcdc.dcdc4os_hvil_status);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4OS: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4S2 - DC/DC Converter 4 Status 2
    pub(crate) fn handle_dcdc4s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4S2::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4s2_low_side_power = msg.dc_dc_4_low_side_power;
                self.dcdc.dcdc4s2_high_side_power = msg.dc_dc_4_high_side_power;
                self.dcdc.dcdc4s2_high_side_ground_voltage = msg.dd_4_hgh_sd_ngtv_t_chsss_grnd_vltg;
                println!("⚡ Received DCDC4S2: HS={:.1}kW, LS={:.1}kW", self.dcdc.dcdc4s2_high_side_power, self.dcdc.dcdc4s2_low_side_power);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4S2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4SBS - DC/DC Converter 4 SLI Battery Status
    pub(crate) fn handle_dcdc4sbs(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4SBS::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4sbs_terminal_voltage = msg.dc_dc_4_sli_battery_terminal_voltage;
                self.dcdc.dcdc4sbs_terminal_current = msg.dc_dc_4_sli_battery_terminal_current;
                self.dcdc.dcdc4sbs_terminal_temperature = msg.dd_4_sl_bttr_trmnl_tmprtr;
                println!("🔋 Received DCDC4SBS: V={:.1}V, I={:.1}A", self.dcdc.dcdc4sbs_terminal_voltage, self.dcdc.dcdc4sbs_terminal_current);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4SBS: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4T - DC/DC Converter 4 Temperature
    pub(crate) fn handle_dcdc4t(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4T::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4t_converter_temperature = msg.dc_dc_4_converter_temperature;
                self.dcdc.dcdc4t_electronic_filter_temperature = msg.dd_4_cnvrtr_eltrn_fltr_tmprtr;
                self.dcdc.dcdc4t_power_electronics_temperature = msg.dd_4_pwr_eltrns_tmprtr;
                self.dcdc.dcdc4t_coolant_in_temperature = msg.dc_dc_4_coolant_in_temperature;
                self.dcdc.dcdc4t_coolant_out_temperature = msg.dc_dc_4_coolant_out_temperature;
                println!("🌡️  Received DCDC4T: Conv={:.1}°C", self.dcdc.dcdc4t_converter_temperature);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4T: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4V - DC/DC Converter 4 Voltage
    pub(crate) fn handle_dcdc4v(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4V::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4v_ignition_voltage = msg.dd_4_cntrllr_inpt_igntn_vltg;
                self.dcdc.dcdc4v_unswitched_sli_voltage = msg.dd_4_cntrllr_inpt_unswthd_sl_vltg;
                println!("⚡ Received DCDC4V: Ign={:.1}V, Unswitched={:.1}V", self.dcdc.dcdc4v_ignition_voltage, self.dcdc.dcdc4v_unswitched_sli_voltage);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4V: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4VC - DC/DC Converter 4 Voltage/Current
    pub(crate) fn handle_dcdc4vc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4VC::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4vc_low_side_voltage = msg.dc_dc_4_low_side_voltage;
                self.dcdc.dcdc4vc_low_side_current = msg.dc_dc_4_low_side_current;
                self.dcdc.dcdc4vc_high_side_voltage = msg.dc_dc_4_high_side_voltage;
                self.dcdc.dcdc4vc_high_side_current = msg.dc_dc_4_high_side_current;
                println!("⚡ Received DCDC4VC: LS={:.1}V/{:.1}A, HS={:.1}V/{:.1}A", self.dcdc.dcdc4vc_low_side_voltage, self.dcdc.dcdc4vc_low_side_current, self.dcdc.dcdc4vc_high_side_voltage, self.dcdc.dcdc4vc_high_side_current);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4VC: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4SBL - DC/DC Converter 4 SLI Battery Limits
    pub(crate) fn handle_dcdc4sbl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4SBL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4sbl_voltage_max_limit = msg.dd_4_sl_bttr_trmnl_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc4sbl_current_max_limit = msg.dd_4_s_btt_tc_ct_mx_lt_rqst;
                self.dcdc.dcdc4sbl_temperature_max_limit = msg.dd_4_sl_bttr_tmprtr_mxmm_lmt_rqst;
                println!("⚡ Received DCDC4SBL: VMax={:.1}V", self.dcdc.dcdc4sbl_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4SBL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4LL - DC/DC Converter 4 Low Side Limits
    pub(crate) fn handle_dcdc4ll(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4LL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4ll_low_side_voltage_min_limit = msg.dd_4_lw_sd_vltg_mnmm_lmt_rqst;
                self.dcdc.dcdc4ll_low_side_voltage_max_limit = msg.dd_4_lw_sd_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc4ll_low_side_current_max_limit = msg.dd_4_lw_sd_crrnt_mxmm_lmt_rqst;
                self.dcdc.dcdc4ll_low_side_current_min_limit = msg.dd_4_lw_sd_crrnt_mnmm_lmt_rqst;
                println!("⚡ Received DCDC4LL: LS VMin={:.1}V, VMax={:.1}V", self.dcdc.dcdc4ll_low_side_voltage_min_limit, self.dcdc.dcdc4ll_low_side_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4LL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4HL - DC/DC Converter 4 High Side Limits
    pub(crate) fn handle_dcdc4hl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4HL::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4hl_high_side_voltage_min_limit = msg.dd_4_hgh_sd_vltg_mnmm_lmt_rqst;
                self.dcdc.dcdc4hl_high_side_voltage_max_limit = msg.dd_4_hgh_sd_vltg_mxmm_lmt_rqst;
                self.dcdc.dcdc4hl_high_side_current_max_limit = msg.dd_4_hgh_sd_crrnt_mxmm_lmt_rqst;
                self.dcdc.dcdc4hl_high_side_current_min_limit = msg.dd_4_hgh_sd_crrnt_mnmm_lmt_rqst;
                println!("⚡ Received DCDC4HL: HS VMin={:.1}V, VMax={:.1}V", self.dcdc.dcdc4hl_high_side_voltage_min_limit, self.dcdc.dcdc4hl_high_side_voltage_max_limit);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4HL: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4LD - DC/DC Converter 4 Lifetime Data
    pub(crate) fn handle_dcdc4ld(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4LD::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4ld_total_high_side_energy = msg.dc_dc_4_total_high_side_energy;
                self.dcdc.dcdc4ld_total_low_side_energy = msg.dc_dc_4_total_low_side_energy;
                self.dcdc.dcdc4ld_total_high_side_charge = msg.dc_dc_4_total_high_side_charge;
                self.dcdc.dcdc4ld_total_low_side_charge = msg.dc_dc_4_total_low_side_charge;
                println!("⚡ Received DCDC4LD: HS Energy={:.0}", self.dcdc.dcdc4ld_total_high_side_energy);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4LD: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle DCDC4CFG1 - DC/DC Converter 4 Configuration 1
    pub(crate) fn handle_dcdc4cfg1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match DCDC4CFG1::decode(can_id, data) {
            Ok(msg) => {
                self.dcdc.dcdc4cfg1_hs_voltage_min_limit = msg.dd_4_hgh_sd_vltg_mnmm_lmt_sttng;
                self.dcdc.dcdc4cfg1_hs_voltage_max_limit = msg.dd_4_hgh_sd_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc4cfg1_hs_current_max_limit = msg.dd_4_hgh_sd_crrnt_mxmm_lmt_sttng;
                self.dcdc.dcdc4cfg1_ls_voltage_min_limit = msg.dd_4_lw_sd_vltg_mnmm_lmt_sttng;
                self.dcdc.dcdc4cfg1_ls_voltage_max_limit = msg.dd_4_lw_sd_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc4cfg1_ls_current_max_limit = msg.dd_4_lw_sd_crrnt_mxmm_lmt_sttng;
                self.dcdc.dcdc4cfg1_sli_voltage_max_limit = msg.dd_4_sl_bttr_trmnl_vltg_mxmm_lmt_sttng;
                self.dcdc.dcdc4cfg1_sli_current_max_limit = msg.dd_4_s_btt_tc_ct_mx_lt_stt;
                self.dcdc.dcdc4cfg1_sli_temperature_max_limit = msg.dd_4_sl_bttr_tmprtr_mxmm_lmt_sttng;
                self.dcdc.dcdc4cfg1_ls_voltage_buck_default = msg.dd_4_lw_sd_vltg_bk_dflt_sttng;
                self.dcdc.dcdc4cfg1_ls_current_min_limit = msg.dd_4_lw_sd_crrnt_mnmm_lmt_sttng;
                self.dcdc.dcdc4cfg1_hs_current_min_limit = msg.dd_4_hgh_sd_crrnt_mnmm_lmt_sttng;
                println!("⚙️  Received DCDC4CFG1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode DCDC4CFG1: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }
}
