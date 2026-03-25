use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle HVESSC1 - High Voltage Energy Storage System Control 1
    pub(crate) fn handle_hvessc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // HVESSC1 - High Voltage Energy Storage System Control 1
        match HVESSC1::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvess_power_down_command = msg.hvess_power_down_command > 0;
                self.hvess.hvess_cell_balancing_command = msg.hvess_cell_balancing_command > 0;
                println!(
                    "🔋 Received HVESSC1: Power down = {}, Cell balancing = {}",
                    self.hvess.hvess_power_down_command, self.hvess.hvess_cell_balancing_command
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVESSC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    // ============================================================================
    // Batch 7: Extended HVESS Handlers
    // ============================================================================

    pub(crate) fn handle_hvessd4(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD4::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd4_discharge_capacity = msg.hvess_discharge_capacity;
                self.hvess.hvessd4_charge_capacity = msg.hvess_charge_capacity;
                self.hvess.hvessd4_cell_balancing_count = msg.hvess_cell_balancing_count;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd5(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD5::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd5_max_discharge_current_limit = msg.hvss_mxmm_instntns_dshrg_crrnt_lmt;
                self.hvess.hvessd5_max_charge_current_limit = msg.hvss_mxmm_instntns_chrg_crrnt_lmt;
                self.hvess.hvessd5_min_cell_soc = msg.hvess_minimum_cell_state_of_charge;
                self.hvess.hvessd5_max_cell_soc = msg.hvess_maximum_cell_state_of_charge;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd7(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD7::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd7_discharge_energy_capacity = msg.hvess_discharge_energy_capacity;
                self.hvess.hvessd7_charge_energy_capacity = msg.hvess_charge_energy_capacity;
                self.hvess.hvessd7_max_charge_voltage_limit = msg.hvess_maximum_charge_voltage_limit;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd8(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD8::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd8_highest_cell_voltage_module = msg.hvss_hghst_cll_vltg_mdl_nmr;
                self.hvess.hvessd8_highest_cell_voltage_cell = msg.hvss_hghst_cll_vltg_cll_nmr;
                self.hvess.hvessd8_lowest_cell_voltage_module = msg.hvss_lwst_cll_vltg_mdl_nmr;
                self.hvess.hvessd8_lowest_cell_voltage_cell = msg.hvss_lwst_cll_vltg_cll_nmr;
                self.hvess.hvessd8_average_cell_voltage = msg.hvess_average_cell_voltage;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd9(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD9::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd9_highest_cell_temp_module = msg.hvss_hghst_cll_tmprtr_mdl_nmr;
                self.hvess.hvessd9_highest_cell_temp_cell = msg.hvss_hghst_cll_tmprtr_cll_nmr;
                self.hvess.hvessd9_lowest_cell_temp_module = msg.hvss_lwst_cll_tmprtr_mdl_nmr;
                self.hvess.hvessd9_lowest_cell_temp_cell = msg.hvss_lwst_cll_tmprtr_cll_nmr;
                self.hvess.hvessd9_thermal_event_detected = msg.hvess_thermal_event_detected;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd10(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD10::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd10_highest_cell_soc_module = msg.hvss_hghst_cll_stt_of_chrg_mdl_nmr;
                self.hvess.hvessd10_highest_cell_soc_cell = msg.hvss_hghst_cll_stt_of_chrg_cll_nmr;
                self.hvess.hvessd10_lowest_cell_soc_module = msg.hvss_lwst_cll_stt_of_chrg_mdl_nmr;
                self.hvess.hvessd10_lowest_cell_soc_cell = msg.hvss_lwst_cll_stt_of_chrg_cll_nmr;
                self.hvess.hvessd10_active_isolation_test = msg.hvss_hgh_vltg_bs_atv_isltn_tst_rslts;
                self.hvess.hvessd10_passive_isolation_test = msg.hvss_hgh_vltg_bs_pssv_isltn_tst_rslts;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd11(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD11::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd11_bus_voltage_neg_to_chassis = msg.hvss_bs_vltg_ngtv_t_chsss_grnd_vltg;
                self.hvess.hvessd11_voltage_neg_to_chassis = msg.hvss_vltg_lvl_ngtv_t_chsss_grnd_vltg;
                self.hvess.hvessd11_actual_charge_rate = msg.hvess_actual_charge_rate;
                self.hvess.hvessd11_total_stored_energy = msg.hvss_ttl_strd_enrg_sr_lvl;
                self.hvess.hvessd11_power_module_electronics_temp = msg.hvss_pwr_mdl_eltrns_tmprtr;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd12(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD12::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd12_intake_coolant_pressure = msg.hvess_intake_coolant_pressure;
                self.hvess.hvessd12_estimated_discharge_time = msg.hvss_estmtd_dshrg_tm_rmnng;
                self.hvess.hvessd12_estimated_charge_time = msg.hvss_estmtd_chrg_tm_rmnng;
                self.hvess.hvessd12_hv_exposure_indicator = msg.hvss_hgh_vltg_expsr_indtr;
                self.hvess.hvessd12_power_hold_relay_status = msg.hvess_power_hold_relay_status;
                self.hvess.hvessd12_positive_precharge_relay = msg.hvss_hgh_vltg_bs_pstv_pr_chrg_rl_stt;
                self.hvess.hvessd12_negative_precharge_relay = msg.hvss_hgh_vltg_bs_ngtv_pr_chrg_rl_stt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd13(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD13::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd13_discharge_power_extended = msg.hvss_avll_dshrg_pwr_extndd_rng;
                self.hvess.hvessd13_charge_power_extended = msg.hvss_avll_chrg_pwr_extndd_rng;
                self.hvess.hvessd13_voltage_extended = msg.hvess_voltage_level_extended_range;
                self.hvess.hvessd13_current_extended = msg.hvess_current_extended_range;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd14(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD14::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd14_max_discharge_current_extended = msg.hvss_mx_istts_ds_ct_lt_extdd_r;
                self.hvess.hvessd14_max_charge_current_extended = msg.hvss_mx_istts_c_ct_lt_extdd_r;
                self.hvess.hvessd14_bus_voltage_extended = msg.hvess_bus_voltage_extended_range;
                self.hvess.hvessd14_min_discharge_voltage_limit = msg.hvss_mnmm_dshrg_vltg_lmt;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessd15(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSD15::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessd15_nominal_discharge_current_limit = msg.hvss_nmnl_dshrg_crrnt_lmt;
                self.hvess.hvessd15_nominal_charge_current_limit = msg.hvess_nominal_charge_current_limit;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessis1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSIS1::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessis1_internal_voltage_1 = msg.hvss_hgh_vltg_intrnl_vltg_lvl_1;
                self.hvess.hvessis1_internal_current_1 = msg.hvss_hgh_vltg_intrnl_crrnt_1;
                self.hvess.hvessis1_internal_voltage_2 = msg.hvss_hgh_vltg_intrnl_vltg_lvl_2;
                self.hvess.hvessis1_internal_current_2 = msg.hvss_hgh_vltg_intrnl_crrnt_2;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessis2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSIS2::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessis2_internal_voltage_3 = msg.hvss_hgh_vltg_intrnl_vltg_lvl_3;
                self.hvess.hvessis2_internal_current_3 = msg.hvss_hgh_vltg_intrnl_crrnt_3;
                self.hvess.hvessis2_internal_voltage_4 = msg.hvss_hgh_vltg_intrnl_vltg_lvl_4;
                self.hvess.hvessis2_internal_current_4 = msg.hvss_hgh_vltg_intrnl_crrnt_4;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessis3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSIS3::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessis3_internal_voltage_5 = msg.hvss_hgh_vltg_intrnl_vltg_lvl_5;
                self.hvess.hvessis3_internal_current_5 = msg.hvss_hgh_vltg_intrnl_crrnt_5;
                self.hvess.hvessis3_internal_voltage_6 = msg.hvss_hgh_vltg_intrnl_vltg_lvl_6;
                self.hvess.hvessis3_internal_current_6 = msg.hvss_hgh_vltg_intrnl_crrnt_6;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessis4(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSIS4::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessis4_internal_voltage_7 = msg.hvss_hgh_vltg_intrnl_vltg_lvl_7;
                self.hvess.hvessis4_internal_current_7 = msg.hvss_hgh_vltg_intrnl_crrnt_7;
                self.hvess.hvessis4_internal_voltage_8 = msg.hvss_hgh_vltg_intrnl_vltg_lvl_8;
                self.hvess.hvessis4_internal_current_8 = msg.hvss_hgh_vltg_intrnl_crrnt_8;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessis5(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSIS5::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessis5_positive_contactor_1_state = msg.hvss_hgh_vltg_intrnl_pstv_cnttr_1_stt;
                self.hvess.hvessis5_negative_contactor_1_state = msg.hvss_hgh_vltg_intrnl_ngtv_cnttr_1_stt;
                self.hvess.hvessis5_precharge_relay_1_state = msg.hvss_hgh_vltg_intrnl_prhrg_rl_1_stt;
                self.hvess.hvessis5_inline_heater_1_status = msg.hvss_t_mt_sst_it_ht_1_stts;
                self.hvess.hvessis5_bus_voltage_1 = msg.hvss_hgh_vltg_intrnl_bs_vltg_lvl_1;
                self.hvess.hvessis5_positive_contactor_2_state = msg.hvss_hgh_vltg_intrnl_pstv_cnttr_2_stt;
                self.hvess.hvessis5_negative_contactor_2_state = msg.hvss_hgh_vltg_intrnl_ngtv_cnttr_2_stt;
                self.hvess.hvessis5_precharge_relay_2_state = msg.hvss_hgh_vltg_intrnl_prhrg_rl_2_stt;
                self.hvess.hvessis5_inline_heater_2_status = msg.hvss_t_mt_sst_it_ht_2_stts;
                self.hvess.hvessis5_bus_voltage_2 = msg.hvss_hgh_vltg_intrnl_bs_vltg_lvl_2;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessis6(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSIS6::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessis6_positive_contactor_3_state = msg.hvss_hgh_vltg_intrnl_pstv_cnttr_3_stt;
                self.hvess.hvessis6_negative_contactor_3_state = msg.hvss_hgh_vltg_intrnl_ngtv_cnttr_3_stt;
                self.hvess.hvessis6_precharge_relay_3_state = msg.hvss_hgh_vltg_intrnl_prhrg_rl_3_stt;
                self.hvess.hvessis6_inline_heater_3_status = msg.hvss_t_mt_sst_it_ht_3_stts;
                self.hvess.hvessis6_bus_voltage_3 = msg.hvss_hgh_vltg_intrnl_bs_vltg_lvl_3;
                self.hvess.hvessis6_positive_contactor_4_state = msg.hvss_hgh_vltg_intrnl_pstv_cnttr_4_stt;
                self.hvess.hvessis6_negative_contactor_4_state = msg.hvss_hgh_vltg_intrnl_ngtv_cnttr_4_stt;
                self.hvess.hvessis6_precharge_relay_4_state = msg.hvss_hgh_vltg_intrnl_prhrg_rl_4_stt;
                self.hvess.hvessis6_inline_heater_4_status = msg.hvss_t_mt_sst_it_ht_4_stts;
                self.hvess.hvessis6_bus_voltage_4 = msg.hvss_hgh_vltg_intrnl_bs_vltg_lvl_4;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessis7(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSIS7::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessis7_number_of_internal_circuits = msg.hvss_nmr_of_intrnl_crts_rd;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessms1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSMS1::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessms1_module_status[0] = msg.hvess_module_1_operational_status;
                self.hvess.hvessms1_module_status[1] = msg.hvess_module_2_operational_status;
                self.hvess.hvessms1_module_status[2] = msg.hvess_module_3_operational_status;
                self.hvess.hvessms1_module_status[3] = msg.hvess_module_4_operational_status;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessms2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSMS2::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessms2_module_status[0] = msg.hvess_module_33_operational_status;
                self.hvess.hvessms2_module_status[1] = msg.hvess_module_34_operational_status;
                self.hvess.hvessms2_module_status[2] = msg.hvess_module_35_operational_status;
                self.hvess.hvessms2_module_status[3] = msg.hvess_module_36_operational_status;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessms3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSMS3::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessms3_module_status[0] = msg.hvess_module_65_operational_status;
                self.hvess.hvessms3_module_status[1] = msg.hvess_module_66_operational_status;
                self.hvess.hvessms3_module_status[2] = msg.hvess_module_67_operational_status;
                self.hvess.hvessms3_module_status[3] = msg.hvess_module_68_operational_status;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesss1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSS1::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesss1_positive_contactor_state = msg.hvss_hgh_vltg_bs_pstv_cnttr_stt;
                self.hvess.hvesss1_negative_contactor_state = msg.hvss_hgh_vltg_bs_ngtv_cnttr_stt;
                self.hvess.hvesss1_hvil_status = msg.hvess_hvil_status;
                self.hvess.hvesss1_soc_status = msg.hvess_state_of_charge_status;
                self.hvess.hvesss1_operational_status = msg.hvess_operational_status;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesss2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSS2::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesss2_discharge_limit_soc = msg.hvss_dshrg_pwr_lmt_dt_stt_of_chrg;
                self.hvess.hvesss2_discharge_limit_temp = msg.hvss_dshrg_pwr_lmt_dt_bttr_tmprtr;
                self.hvess.hvesss2_charge_limit_soc = msg.hvss_chrg_pwr_lmt_dt_stt_of_chrg;
                self.hvess.hvesss2_charge_limit_temp = msg.hvss_chrg_pwr_lmt_dt_bttr_tmprtr;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessfs2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSFS2::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessfs2_fan_voltage = msg.hvess_fan_voltage;
                self.hvess.hvessfs2_fan_current = msg.hvess_fan_current;
                self.hvess.hvessfs2_fan_hvil_status = msg.hvess_fan_hvil_status;
                self.hvess.hvessfs2_fan_percent_speed = msg.hvess_fan_percent_speed;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvessfc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // HVESSFC - Fan Command: update fan command state, physics will respond
        match HVESSFC::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvessfc_fan_enable_command = msg.hvess_fan_enable_command;
                self.hvess.hvessfc_fan_power_hold = msg.hvess_fan_power_hold;
                self.hvess.hvessfc_fan_speed_command = msg.hvess_fan_speed_command;
                self.hvess.hvessfc_fan_percent_speed_command = msg.hvess_fan_percent_speed_command;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesscfg(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSCFG::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesscfg_nominal_voltage = msg.hvess_nominal_voltage;
                self.hvess.hvesscfg_min_operating_voltage = msg.hvss_rmmndd_mnmm_oprtng_vltg;
                self.hvess.hvesscfg_max_operating_voltage = msg.hvss_rmmndd_mxmm_oprtng_vltg;
                self.hvess.hvesscfg_nominal_capacity = msg.hvess_nominal_rated_capacity;
                self.hvess.hvesscfg_num_packs = msg.hvess_number_of_hvesps_configured;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesscp1c(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // HVESSCP1C - Coolant Pump 1 Command: physics will respond with lag
        match HVESSCP1C::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesscp1c_enable_command = msg.hvess_coolant_pump_1_enable_command;
                self.hvess.hvesscp1c_power_hold = msg.hvess_coolant_pump_1_power_hold;
                self.hvess.hvesscp1c_speed_command = msg.hvess_coolant_pump_1_speed_command;
                self.hvess.hvesscp1c_percent_speed_command = msg.hvss_clnt_pmp_1_prnt_spd_cmmnd;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesscp1s1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSCP1S1::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesscp1s1_motor_speed = msg.hvess_coolant_pump_1_motor_speed;
                self.hvess.hvesscp1s1_power = msg.hvess_coolant_pump_1_power;
                self.hvess.hvesscp1s1_motor_speed_status = msg.hvss_clnt_pmp_1_mtr_spd_stts;
                self.hvess.hvesscp1s1_operating_status = msg.hvss_clnt_pmp_1_oprtng_stts;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesscp1s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSCP1S2::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesscp1s2_voltage = msg.hvess_coolant_pump_1_voltage;
                self.hvess.hvesscp1s2_current = msg.hvess_coolant_pump_1_current;
                self.hvess.hvesscp1s2_hvil_status = msg.hvess_coolant_pump_1_hvil_status;
                self.hvess.hvesscp1s2_percent_speed = msg.hvess_coolant_pump_1_percent_speed;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesscp2c(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // HVESSCP2C - Coolant Pump 2 Command: physics will respond with lag
        match HVESSCP2C::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesscp2c_enable_command = msg.hvess_coolant_pump_2_enable_command;
                self.hvess.hvesscp2c_power_hold = msg.hvess_coolant_pump_2_power_hold;
                self.hvess.hvesscp2c_speed_command = msg.hvess_coolant_pump_2_speed_command;
                self.hvess.hvesscp2c_percent_speed_command = msg.hvss_clnt_pmp_2_prnt_spd_cmmnd;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesscp2s1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSCP2S1::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesscp2s1_motor_speed = msg.hvess_coolant_pump_2_motor_speed;
                self.hvess.hvesscp2s1_power = msg.hvess_coolant_pump_2_power;
                self.hvess.hvesscp2s1_motor_speed_status = msg.hvss_clnt_pmp_2_mtr_spd_stts;
                self.hvess.hvesscp2s1_operating_status = msg.hvss_clnt_pmp_2_oprtng_stts;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesscp2s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSCP2S2::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesscp2s2_voltage = msg.hvess_coolant_pump_2_voltage;
                self.hvess.hvesscp2s2_current = msg.hvess_coolant_pump_2_current;
                self.hvess.hvesscp2s2_hvil_status = msg.hvess_coolant_pump_2_hvil_status;
                self.hvess.hvesscp2s2_percent_speed = msg.hvess_coolant_pump_2_percent_speed;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesstch1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSTCH1::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesstch1_compressor_discharge_abs_pressure = msg.hvss_t_mt_sst_c_1_cpss_ds_ast_pss;
                self.hvess.hvesstch1_compressor_suction_abs_pressure = msg.hvss_t_mt_sst_c_1_cpss_st_ast_pss;
                self.hvess.hvesstch1_outlet_coolant_temp = msg.hvss_t_mt_sst_c_1_ott_ct_tpt;
                self.hvess.hvesstch1_condenser_valve_position = msg.hvss_t_mt_sst_c_1_c_vv_pst;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesstch2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSTCH2::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesstch2_compressor_discharge_abs_pressure = msg.hvss_t_mt_sst_c_2_cpss_ds_ast_pss;
                self.hvess.hvesstch2_compressor_suction_abs_pressure = msg.hvss_t_mt_sst_c_2_cpss_st_ast_pss;
                self.hvess.hvesstch2_outlet_coolant_temp = msg.hvss_t_mt_sst_c_2_ott_ct_tpt;
                self.hvess.hvesstch2_condenser_valve_position = msg.hvss_t_mt_sst_c_2_c_vv_pst;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesstch3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSTCH3::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesstch3_compressor_discharge_abs_pressure = msg.hvss_t_mt_sst_c_3_cpss_ds_ast_pss;
                self.hvess.hvesstch3_compressor_suction_abs_pressure = msg.hvss_t_mt_sst_c_3_cpss_st_ast_pss;
                self.hvess.hvesstch3_outlet_coolant_temp = msg.hvss_t_mt_sst_c_3_ott_ct_tpt;
                self.hvess.hvesstch3_condenser_valve_position = msg.hvss_t_mt_sst_c_3_c_vv_pst;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }

    pub(crate) fn handle_hvesshist(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVESSHIST::decode(can_id, data) {
            Ok(msg) => {
                self.hvess.hvesshist_state_of_health = msg.hvess_state_of_health;
                self.hvess.hvesshist_contactor_open_under_load = msg.hvss_cnttr_opn_undr_ld_cnt;
                self.hvess.hvesshist_total_energy_throughput = msg.hvess_total_energy_throughput;
                self.hvess.hvesshist_total_accumulated_charge = msg.hvess_total_accumulated_charge;
                self.hvess.hvesshist_lifetime_energy_input = msg.hvess_total_lifetime_energy_input;
                self.hvess.hvesshist_lifetime_energy_output = msg.hvess_total_lifetime_energy_output;
                Ok(MessageStatus::Recognized)
            }
            Err(_) => Ok(MessageStatus::DecodeFailed),
        }
    }
}
