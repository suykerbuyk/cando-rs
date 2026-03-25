use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_power_supply_frames(
        &self,
        frames: &mut Vec<CanFrame>,
        device_id: DeviceId,
    ) {
        // ALTC - Alternator Control
        let altc = ALTC {
            device_id,
            altrntr_stpnt_vltg_cmmnd: self.power_supply.altc_setpoint_voltage,
            altrntr_exttn_mxmm_crrnt_lmt: self.power_supply.altc_excitation_current_limit,
            alternator_torque_ramp_time_command: self.power_supply.altc_torque_ramp_time,
            altrntr_trq_rmp_mxmm_spd_cmmnd: self.power_supply.altc_torque_ramp_max_speed,
        };

        if let Ok((can_id, data)) = altc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // GC2 - Generator Control 2 (Phase 1 Power Supply)
        let gc2 = GC2 {
            device_id,
            engine_load_setpoint_request: self.power_supply.gc2_engine_load_setpoint,
            engine_self_induced_derate_inhibit: self.power_supply.gc2_derate_inhibit,
            generator_governing_bias: self.power_supply.gc2_governing_bias,
        };

        if let Ok((can_id, data)) = gc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCACAI1S2 - DC/AC Accessory Inverter 1 Status 2 (Phase 1 Power Supply)
        let dcacai1s2 = DCACAI1S2 {
            device_id,
            da_assr_invrtr_1_d_sd_pwr: self.power_supply.dcacai1s2_desired_power,
            da_assr_invrtr_1_d_sd_vltg: self.power_supply.dcacai1s2_desired_voltage,
            da_assr_invrtr_1_d_sd_crrnt: self.power_supply.dcacai1s2_desired_current,
            da_ass_ivt_1_d_sd_ntv_t_csss_gd_vt: self
                .power_supply
                .dcacai1s2_desired_ground_voltage,
        };

        if let Ok((can_id, data)) = dcacai1s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // DCACAI1V - DC/AC Accessory Inverter 1 Voltage (Phase 1 Power Supply)
        let dcacai1v = DCACAI1V {
            device_id,
            da_assr_invrtr_1_igntn_vltg: self.power_supply.dcacai1v_ignition_voltage,
            da_assr_invrtr_1_unswthd_sl_vltg: self.power_supply.dcacai1v_unswitched_voltage,
        };

        if let Ok((can_id, data)) = dcacai1v.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // GTRACE - Generator Trip Energy (Phase 1 Power Supply)
        let gtrace = GTRACE {
            device_id,
            generator_trip_kw_hours_export: self.power_supply.gtrace_kwh_export,
            generator_trip_kvar_hours_export: self.power_supply.gtrace_kvarh_export,
        };

        if let Ok((can_id, data)) = gtrace.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // GC1 - Generator Control 1 (Batch 9 Power Supply)
        let gc1 = GC1 {
            device_id,
            requested_engine_control_mode: self.power_supply.gc1_requested_engine_control_mode,
            gnrtr_cntrl_nt_in_atmt_strt_stt: self.power_supply.gc1_not_in_auto_start_state,
            gnrtr_nt_rd_t_atmtll_prlll_stt: self.power_supply.gc1_not_ready_to_parallel_state,
            generator_alternator_efficiency: self.power_supply.gc1_alternator_efficiency,
            generator_governing_speed_command: self.power_supply.gc1_governing_speed_command,
            generator_frequency_selection: self.power_supply.gc1_frequency_selection,
            engine_speed_governor_gain_adjust: self.power_supply.gc1_speed_governor_gain_adjust,
            engine_speed_governor_droop: self.power_supply.gc1_speed_governor_droop,
        };
        if let Ok((can_id, data)) = gc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // GTRACE2 - Generator Trip Energy 2 (Batch 9 Power Supply)
        let gtrace2 = GTRACE2 {
            device_id,
            generator_trip_kvar_hours_import: self.power_supply.gtrace2_kvarh_import,
        };
        if let Ok((can_id, data)) = gtrace2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // GAAC - Generator Average AC (Batch 9 Power Supply)
        let gaac = GAAC {
            device_id,
            gnrtr_avrg_ln_ln_a_rms_vltg: self.power_supply.gaac_avg_line_line_voltage,
            gnrtr_avrg_ln_ntrl_a_rms_vltg: self.power_supply.gaac_avg_line_neutral_voltage,
            generator_average_ac_frequency: self.power_supply.gaac_avg_frequency,
            generator_average_ac_rms_current: self.power_supply.gaac_avg_rms_current,
        };
        if let Ok((can_id, data)) = gaac.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
