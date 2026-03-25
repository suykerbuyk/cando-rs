use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle ALTC - Alternator Control
    pub(crate) fn handle_altc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // ALTC - Alternator Control
        match ALTC::decode(can_id, data) {
            Ok(msg) => {
                self.power_supply.altc_setpoint_voltage = msg.altrntr_stpnt_vltg_cmmnd;
                self.power_supply.altc_excitation_current_limit = msg.altrntr_exttn_mxmm_crrnt_lmt;
                self.power_supply.altc_torque_ramp_time = msg.alternator_torque_ramp_time_command;
                self.power_supply.altc_torque_ramp_max_speed = msg.altrntr_trq_rmp_mxmm_spd_cmmnd;
                println!(
                    "⚡ Received ALTC: Voltage = {:.3}V, Current = {:.1}A, Ramp time = {:.1}s, Max speed = {:.0} rpm",
                    self.power_supply.altc_setpoint_voltage,
                    self.power_supply.altc_excitation_current_limit,
                    self.power_supply.altc_torque_ramp_time,
                    self.power_supply.altc_torque_ramp_max_speed
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode ALTC: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle GC2 - Generator Control 2 (Phase 1 Power Supply)
    pub(crate) fn handle_gc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // GC2 - Generator Control 2 (Phase 1 Power Supply)
        match GC2::decode(can_id, data) {
            Ok(msg) => {
                self.power_supply.gc2_engine_load_setpoint = msg.engine_load_setpoint_request;
                self.power_supply.gc2_derate_inhibit = msg.engine_self_induced_derate_inhibit;
                self.power_supply.gc2_governing_bias = msg.generator_governing_bias;
                println!(
                    "⚡ Received GC2: Load setpoint = {:.1}kW, Derate inhibit = {}, Governing bias = {:.3}%",
                    self.power_supply.gc2_engine_load_setpoint,
                    self.power_supply.gc2_derate_inhibit,
                    self.power_supply.gc2_governing_bias
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode GC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DCACAI1S2 - DC/AC Accessory Inverter 1 Status 2 (Phase 1 Power Supply)
    pub(crate) fn handle_dcacai1s2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DCACAI1S2 - DC/AC Accessory Inverter 1 Status 2 (Phase 1 Power Supply)
        match DCACAI1S2::decode(can_id, data) {
            Ok(msg) => {
                self.power_supply.dcacai1s2_desired_power = msg.da_assr_invrtr_1_d_sd_pwr;
                self.power_supply.dcacai1s2_desired_voltage = msg.da_assr_invrtr_1_d_sd_vltg;
                self.power_supply.dcacai1s2_desired_current = msg.da_assr_invrtr_1_d_sd_crrnt;
                self.power_supply.dcacai1s2_desired_ground_voltage =
                    msg.da_ass_ivt_1_d_sd_ntv_t_csss_gd_vt;
                println!(
                    "⚡ Received DCACAI1S2: Power = {:.1}kW, Voltage = {:.1}V, Current = {:.1}A, Ground = {:.1}V",
                    self.power_supply.dcacai1s2_desired_power,
                    self.power_supply.dcacai1s2_desired_voltage,
                    self.power_supply.dcacai1s2_desired_current,
                    self.power_supply.dcacai1s2_desired_ground_voltage
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DCACAI1S2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle DCACAI1V - DC/AC Accessory Inverter 1 Voltage (Phase 1 Power Supply)
    pub(crate) fn handle_dcacai1v(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // DCACAI1V - DC/AC Accessory Inverter 1 Voltage (Phase 1 Power Supply)
        match DCACAI1V::decode(can_id, data) {
            Ok(msg) => {
                self.power_supply.dcacai1v_ignition_voltage = msg.da_assr_invrtr_1_igntn_vltg;
                self.power_supply.dcacai1v_unswitched_voltage = msg.da_assr_invrtr_1_unswthd_sl_vltg;
                println!(
                    "🔌 Received DCACAI1V: Ignition={:.1}V, Unswitched={:.1}V",
                    self.power_supply.dcacai1v_ignition_voltage, self.power_supply.dcacai1v_unswitched_voltage
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode DCACAI1V: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle GTRACE - Generator Trip Energy (Phase 1 Power Supply)
    pub(crate) fn handle_gtrace(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // GTRACE - Generator Trip Energy (Phase 1 Power Supply)
        match GTRACE::decode(can_id, data) {
            Ok(msg) => {
                self.power_supply.gtrace_kwh_export = msg.generator_trip_kw_hours_export;
                self.power_supply.gtrace_kvarh_export = msg.generator_trip_kvar_hours_export;
                println!(
                    "⚡ Received GTRACE: kWh Export={} kWh, kVArh Export={} kVArh",
                    self.power_supply.gtrace_kwh_export, self.power_supply.gtrace_kvarh_export
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode GTRACE: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle GC1 - Generator Control 1 (Batch 9 Power Supply)
    pub(crate) fn handle_gc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match GC1::decode(can_id, data) {
            Ok(msg) => {
                self.power_supply.gc1_requested_engine_control_mode = msg.requested_engine_control_mode;
                self.power_supply.gc1_not_in_auto_start_state = msg.gnrtr_cntrl_nt_in_atmt_strt_stt;
                self.power_supply.gc1_not_ready_to_parallel_state = msg.gnrtr_nt_rd_t_atmtll_prlll_stt;
                self.power_supply.gc1_alternator_efficiency = msg.generator_alternator_efficiency;
                self.power_supply.gc1_governing_speed_command = msg.generator_governing_speed_command;
                self.power_supply.gc1_frequency_selection = msg.generator_frequency_selection;
                self.power_supply.gc1_speed_governor_gain_adjust = msg.engine_speed_governor_gain_adjust;
                self.power_supply.gc1_speed_governor_droop = msg.engine_speed_governor_droop;
                println!("⚡ Received GC1: Mode={}, Efficiency={:.1}%", self.power_supply.gc1_requested_engine_control_mode, self.power_supply.gc1_alternator_efficiency);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode GC1: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle GTRACE2 - Generator Trip Energy 2 (Batch 9 Power Supply)
    pub(crate) fn handle_gtrace2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match GTRACE2::decode(can_id, data) {
            Ok(msg) => {
                self.power_supply.gtrace2_kvarh_import = msg.generator_trip_kvar_hours_import;
                println!("⚡ Received GTRACE2: kVArh Import={}", self.power_supply.gtrace2_kvarh_import);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode GTRACE2: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }

    /// Handle GAAC - Generator Average AC (Batch 9 Power Supply)
    pub(crate) fn handle_gaac(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match GAAC::decode(can_id, data) {
            Ok(msg) => {
                self.power_supply.gaac_avg_line_line_voltage = msg.gnrtr_avrg_ln_ln_a_rms_vltg;
                self.power_supply.gaac_avg_line_neutral_voltage = msg.gnrtr_avrg_ln_ntrl_a_rms_vltg;
                self.power_supply.gaac_avg_frequency = msg.generator_average_ac_frequency;
                self.power_supply.gaac_avg_rms_current = msg.generator_average_ac_rms_current;
                println!("⚡ Received GAAC: V_LL={}, Freq={:.1}Hz, I={}", self.power_supply.gaac_avg_line_line_voltage, self.power_supply.gaac_avg_frequency, self.power_supply.gaac_avg_rms_current);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => { println!("⚠️ Failed to decode GAAC: {}", e); Ok(MessageStatus::DecodeFailed) }
        }
    }
}
