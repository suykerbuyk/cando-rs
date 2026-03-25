use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_ev_charging_frames(
        &self,
        frames: &mut Vec<CanFrame>,
        device_id: DeviceId,
    ) {
        // EVDCS1 - EV DC Charging Status 1
        let evdcs1 = EVDCS1 {
            device_id,
            ev_cabin_conditioning_flag: self.ev_charging.evdcs1_cabin_conditioning_flag,
            ev_ress_conditioning_flag: self.ev_charging.evdcs1_ress_conditioning_flag,
            ev_error_code: self.ev_charging.evdcs1_error_code,
        };
        if let Ok((can_id, data)) = evdcs1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVDCTGT - DC Charging Target
        let evdctgt = EVDCTGT {
            device_id,
            dc_charging_target_crc: ((self.ev_charging.evdctgt_counter as u64 * 7) % 250) as u8,
            dc_charging_target_sequence_counter: self.ev_charging.evdctgt_counter,
            dc_target_charging_voltage: self.ev_charging.evdctgt_target_voltage,
            dc_target_charging_current: self.ev_charging.evdctgt_target_current,
        };
        if let Ok((can_id, data)) = evdctgt.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVDCLIM1 - EV DC Charging Limits 1
        let evdclim1 = EVDCLIM1 {
            device_id,
            hrd_or_ev_d_chrgng_vltg_mxmm: self.ev_charging.evdclim1_max_voltage,
            hrd_or_ev_d_chrgng_crrnt_mxmm: self.ev_charging.evdclim1_max_current,
            hrd_or_ev_d_chrgng_pwr_mxmm: self.ev_charging.evdclim1_max_power,
            hrd_or_ev_rqstd_enrg_trnsfr_tp: self.ev_charging.evdclim1_energy_transfer_type,
        };
        if let Ok((can_id, data)) = evdclim1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVDCLIM2 - EV DC Charging Limits 2
        let evdclim2 = EVDCLIM2 {
            device_id,
            ev_bulk_state_of_charge: self.ev_charging.evdclim2_bulk_soc,
            ev_full_state_of_charge: self.ev_charging.evdclim2_full_soc,
            ev_energy_capacity: self.ev_charging.evdclim2_energy_capacity,
            ev_energy_requested: self.ev_charging.evdclim2_energy_requested,
        };
        if let Ok((can_id, data)) = evdclim2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVDCCIP - EV DC Charging In-Progress
        let evdccip = EVDCCIP {
            device_id,
            ev_bulk_charging_complete: self.ev_charging.evdccip_bulk_charging_complete,
            ev_full_charging_complete: self.ev_charging.evdccip_full_charging_complete,
            ev_bulk_charge_time_remaining: self.ev_charging.evdccip_bulk_charge_time_remaining,
            ev_full_charge_time_remaining: self.ev_charging.evdccip_full_charge_time_remaining,
            ev_departure_time: self.ev_charging.evdccip_departure_time,
        };
        if let Ok((can_id, data)) = evdccip.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVSE1CS1 - EV Supply Equipment 1 Contactor Status 1
        let evse1cs1 = EVSE1CS1 {
            device_id,
            hrd_or_ev_d_cnttr_inpt_vltg: self.ev_charging.evse1cs1_contactor_input_voltage,
            hybrid_or_ev_dc_charging_bus_voltage: self.ev_charging.evse1cs1_charging_bus_voltage,
            hrd_or_ev_d_inlt_cnttr_1_stt: self.ev_charging.evse1cs1_contactor_1_state,
        };
        if let Ok((can_id, data)) = evse1cs1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVSE1CC1 - EV Supply Equipment 1 Contactor Command 1
        let evse1cc1 = EVSE1CC1 {
            device_id,
            hrd_or_ev_d_inlt_cnttr_1_cmmnd: self.ev_charging.evse1cc1_contactor_1_command,
        };
        if let Ok((can_id, data)) = evse1cc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVSEC1 - EVSE Control 1
        let evsec1 = EVSEC1 {
            device_id,
            evse_connector_lock_request: self.ev_charging.evsec1_connector_lock_request,
            evse_dc_stage_request: self.ev_charging.evsec1_dc_stage_request,
            ev_ready: self.ev_charging.evsec1_ev_ready,
            evse_contactor_command: self.ev_charging.evsec1_contactor_command,
        };
        if let Ok((can_id, data)) = evsec1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVSEDCS1 - EVSE DC Charging Status 1
        let evsedcs1 = EVSEDCS1 {
            device_id,
            evse_dc_charging_state: self.ev_charging.evsedcs1_dc_charging_state,
            evse_isolation_status: self.ev_charging.evsedcs1_isolation_status,
            evse_present_dc_charging_voltage: self.ev_charging.evsedcs1_present_voltage,
            evse_present_dc_charging_current: self.ev_charging.evsedcs1_present_current,
            evse_voltage_limit_achieved: self.ev_charging.evsedcs1_voltage_limit_achieved,
            evse_current_limit_achieved: self.ev_charging.evsedcs1_current_limit_achieved,
            evse_power_limit_achieved: self.ev_charging.evsedcs1_power_limit_achieved,
            evse_processing_state: self.ev_charging.evsedcs1_processing_state,
            evse_status: self.ev_charging.evsedcs1_status,
            evse_response_code: self.ev_charging.evsedcs1_response_code,
        };
        if let Ok((can_id, data)) = evsedcs1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVSES1 - EVSE Status 1
        let evses1 = EVSES1 {
            device_id,
            evse_connector_release_latch: self.ev_charging.evses1_connector_release_latch,
            evse_manual_override: self.ev_charging.evses1_manual_override,
            evse_connector_lock_state: self.ev_charging.evses1_connector_lock_state,
            evse_connector_lock_permission: self.ev_charging.evses1_connector_lock_permission,
            inlet_contactor_state: self.ev_charging.evses1_inlet_contactor_state,
            evse_inlet_state: self.ev_charging.evses1_inlet_state,
            evse_connection_type: self.ev_charging.evses1_connection_type,
            evse_communications_physical_layer: self.ev_charging.evses1_communications_physical_layer,
        };
        if let Ok((can_id, data)) = evses1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVSES2 - EVSE Status 2
        let evses2 = EVSES2 {
            device_id,
            evse_temp_sensor_type: self.ev_charging.evses2_temp_sensor_type,
            evse_connector_temperature_status: self.ev_charging.evses2_connector_temperature_status,
            ev_chrgng_inlt_cnntr_tmprtr: self.ev_charging.evses2_inlet_connector_temperature,
            ev_c_it_ct_tpt_ss_rsst: self.ev_charging.evses2_temp_sensor_resistance,
        };
        if let Ok((can_id, data)) = evses2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVC - Engine Valve Control
        let evc = EVC {
            device_id,
            engn_vlv_cntrl_mdl_1_prlmnr_fm: self.ev_charging.evc_valve_control_modules[0],
            engn_vlv_cntrl_mdl_2_prlmnr_fm: self.ev_charging.evc_valve_control_modules[1],
            engn_vlv_cntrl_mdl_3_prlmnr_fm: self.ev_charging.evc_valve_control_modules[2],
            engn_vlv_cntrl_mdl_4_prlmnr_fm: self.ev_charging.evc_valve_control_modules[3],
            engn_vlv_cntrl_mdl_5_prlmnr_fm: self.ev_charging.evc_valve_control_modules[4],
            engn_vlv_cntrl_mdl_6_prlmnr_fm: self.ev_charging.evc_valve_control_modules[5],
            engn_vlv_cntrl_mdl_7_prlmnr_fm: self.ev_charging.evc_valve_control_modules[6],
            engn_vlv_cntrl_mdl_8_prlmnr_fm: self.ev_charging.evc_valve_control_modules[7],
        };
        if let Ok((can_id, data)) = evc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVEI - EV Energy Info
        let evei = EVEI {
            device_id,
            total_trip_energy_consumed: self.ev_charging.evei_total_trip_energy_consumed,
            trip_drive_energy_economy: self.ev_charging.evei_trip_drive_energy_economy,
        };
        if let Ok((can_id, data)) = evei.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVOI1 - EV Operating Info 1
        let evoi1 = EVOI1 {
            device_id,
            hvess_estimated_remaining_distance: self.ev_charging.evoi1_estimated_remaining_distance,
        };
        if let Ok((can_id, data)) = evoi1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVBCS1 - HV Bus Contactor Status 1
        let hvbcs1 = HVBCS1 {
            device_id,
            hgh_vltg_bs_intrf_1_pstv_cnttr_stt: self.ev_charging.hvbcs1_positive_contactor_states[0],
            hgh_vltg_bs_intrf_1_ngtv_cnttr_stt: self.ev_charging.hvbcs1_negative_contactor_states[0],
            hgh_vltg_bs_intrf_2_pstv_cnttr_stt: self.ev_charging.hvbcs1_positive_contactor_states[1],
            hgh_vltg_bs_intrf_2_ngtv_cnttr_stt: self.ev_charging.hvbcs1_negative_contactor_states[1],
            hgh_vltg_bs_intrf_3_pstv_cnttr_stt: self.ev_charging.hvbcs1_positive_contactor_states[2],
            hgh_vltg_bs_intrf_3_ngtv_cnttr_stt: self.ev_charging.hvbcs1_negative_contactor_states[2],
            hgh_vltg_bs_intrf_4_pstv_cnttr_stt: self.ev_charging.hvbcs1_positive_contactor_states[3],
            hgh_vltg_bs_intrf_4_ngtv_cnttr_stt: self.ev_charging.hvbcs1_negative_contactor_states[3],
            hgh_vltg_bs_intrf_5_pstv_cnttr_stt: self.ev_charging.hvbcs1_positive_contactor_states[4],
            hgh_vltg_bs_intrf_5_ngtv_cnttr_stt: self.ev_charging.hvbcs1_negative_contactor_states[4],
            hgh_vltg_bs_intrf_6_pstv_cnttr_stt: self.ev_charging.hvbcs1_positive_contactor_states[5],
            hgh_vltg_bs_intrf_6_ngtv_cnttr_stt: self.ev_charging.hvbcs1_negative_contactor_states[5],
            hgh_vltg_bs_intrf_7_pstv_cnttr_stt: self.ev_charging.hvbcs1_positive_contactor_states[6],
            hgh_vltg_bs_intrf_7_ngtv_cnttr_stt: self.ev_charging.hvbcs1_negative_contactor_states[6],
            hgh_vltg_bs_intrf_8_pstv_cnttr_stt: self.ev_charging.hvbcs1_positive_contactor_states[7],
            hgh_vltg_bs_intrf_8_ngtv_cnttr_stt: self.ev_charging.hvbcs1_negative_contactor_states[7],
            hvbcs_1_embedded_integrity_support: self.ev_charging.hvbcs1_embedded_integrity_support,
            hvbcs_1_counter: self.ev_charging.hvbcs1_counter,
            hvbcs_1_crc: ((self.ev_charging.hvbcs1_counter as u64 * 7) % 250) as u8,
        };
        if let Ok((can_id, data)) = hvbcs1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVBCS2 - HV Bus Contactor Status 2
        let hvbcs2 = HVBCS2 {
            device_id,
            hvbcs_2_embedded_integrity_support: self.ev_charging.hvbcs2_embedded_integrity_support,
            hvbcs_2_counter: self.ev_charging.hvbcs2_counter,
            hvbcs_2_crc: ((self.ev_charging.hvbcs2_counter as u64 * 7) % 250) as u8,
        };
        if let Ok((can_id, data)) = hvbcs2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVBCS3 - HV Bus Contactor Status 3
        let hvbcs3 = HVBCS3 {
            device_id,
            hvbcs_3_embedded_integrity_support: self.ev_charging.hvbcs3_embedded_integrity_support,
            hvbcs_3_counter: self.ev_charging.hvbcs3_counter,
            hvbcs_3_crc: ((self.ev_charging.hvbcs3_counter as u64 * 7) % 250) as u8,
        };
        if let Ok((can_id, data)) = hvbcs3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVBCC1 - HV Bus Contactor Command 1
        let hvbcc1 = HVBCC1 {
            device_id,
            hgh_vltg_bs_intrf_1_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[0],
            hgh_vltg_bs_intrf_2_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[1],
            hgh_vltg_bs_intrf_3_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[2],
            hgh_vltg_bs_intrf_4_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[3],
            hgh_vltg_bs_intrf_5_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[4],
            hgh_vltg_bs_intrf_6_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[5],
            hgh_vltg_bs_intrf_7_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[6],
            hgh_vltg_bs_intrf_8_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[7],
            hgh_vltg_bs_intrf_9_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[8],
            hgh_vltg_bs_intrf_10_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[9],
            hgh_vltg_bs_intrf_11_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[10],
            hgh_vltg_bs_intrf_12_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[11],
            hgh_vltg_bs_intrf_13_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[12],
            hgh_vltg_bs_intrf_14_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[13],
            hgh_vltg_bs_intrf_15_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[14],
            hgh_vltg_bs_intrf_16_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[15],
            hgh_vltg_bs_intrf_17_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[16],
            hgh_vltg_bs_intrf_18_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[17],
            hgh_vltg_bs_intrf_19_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[18],
            hgh_vltg_bs_intrf_20_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[19],
            hgh_vltg_bs_intrf_21_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[20],
            hgh_vltg_bs_intrf_22_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[21],
            hgh_vltg_bs_intrf_23_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[22],
            hgh_vltg_bs_intrf_24_cnnt_cmmnd: self.ev_charging.hvbcc1_connect_commands[23],
            hvbcc_1_embedded_integrity_support: self.ev_charging.hvbcc1_embedded_integrity_support,
            hvbcc_1_counter: self.ev_charging.hvbcc1_counter,
            hvbcc_1_crc: ((self.ev_charging.hvbcc1_counter as u64 * 7) % 250) as u8,
        };
        if let Ok((can_id, data)) = hvbcc1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVBCC2 - HV Bus Contactor Command 2
        let hvbcc2 = HVBCC2 {
            device_id,
            hgh_vltg_bs_intrf_25_cnnt_cmmnd: self.ev_charging.hvbcc2_connect_commands[0],
            hgh_vltg_bs_intrf_26_cnnt_cmmnd: self.ev_charging.hvbcc2_connect_commands[1],
            hgh_vltg_bs_intrf_27_cnnt_cmmnd: self.ev_charging.hvbcc2_connect_commands[2],
            hgh_vltg_bs_intrf_28_cnnt_cmmnd: self.ev_charging.hvbcc2_connect_commands[3],
            hgh_vltg_bs_intrf_29_cnnt_cmmnd: self.ev_charging.hvbcc2_connect_commands[4],
            hgh_vltg_bs_intrf_30_cnnt_cmmnd: self.ev_charging.hvbcc2_connect_commands[5],
            hgh_vltg_bs_intrf_31_cnnt_cmmnd: self.ev_charging.hvbcc2_connect_commands[6],
            hgh_vltg_bs_intrf_32_cnnt_cmmnd: self.ev_charging.hvbcc2_connect_commands[7],
            hvbcc_2_embedded_integrity_support: self.ev_charging.hvbcc2_embedded_integrity_support,
            hvbcc_2_counter: self.ev_charging.hvbcc2_counter,
            hvbcc_2_crc: ((self.ev_charging.hvbcc2_counter as u64 * 7) % 250) as u8,
        };
        if let Ok((can_id, data)) = hvbcc2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVBI - HV Bus Info
        let hvbi = HVBI {
            device_id,
            high_voltage_dc_bus_availability: self.ev_charging.hvbi_dc_bus_availability,
            hgh_vltg_bs_drvln_avllt: self.ev_charging.hvbi_driveline_availability,
            hgh_vltg_bs_axlrs_avllt: self.ev_charging.hvbi_auxiliaries_availability,
            high_voltage_bus_epto_availability: self.ev_charging.hvbi_epto_availability,
            hgh_vltg_bs_on_brd_chrgr_avllt: self.ev_charging.hvbi_on_board_charger_availability,
            hgh_vltg_bs_off_brd_chrgr_avllt: self.ev_charging.hvbi_off_board_charger_availability,
        };
        if let Ok((can_id, data)) = hvbi.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // EVVT - Engine VVT
        let evvt = EVVT {
            device_id,
            e_vvt_it_vv_cst_cdd_t_ost: self.ev_charging.evvt_intake_commanded_offset,
            e_vvt_it_vv_cst_t_ost_pst: self.ev_charging.evvt_intake_offset_position,
            e_vvt_exst_vv_cst_cdd_t_ost: self.ev_charging.evvt_exhaust_commanded_offset,
            e_vvt_exst_vv_cst_t_ost_pst: self.ev_charging.evvt_exhaust_offset_position,
        };
        if let Ok((can_id, data)) = evvt.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
