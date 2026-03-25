use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle EVDCS1 - EV DC Charging Status 1
    pub(crate) fn handle_evdcs1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVDCS1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evdcs1_cabin_conditioning_flag = msg.ev_cabin_conditioning_flag;
                self.ev_charging.evdcs1_ress_conditioning_flag = msg.ev_ress_conditioning_flag;
                self.ev_charging.evdcs1_error_code = msg.ev_error_code;
                println!(
                    "🔌 Received EVDCS1: Cabin={}, RESS={}, Error={}",
                    self.ev_charging.evdcs1_cabin_conditioning_flag,
                    self.ev_charging.evdcs1_ress_conditioning_flag,
                    self.ev_charging.evdcs1_error_code
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVDCS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVDCTGT - DC Charging Target
    pub(crate) fn handle_evdctgt(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVDCTGT::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evdctgt_target_voltage = msg.dc_target_charging_voltage;
                self.ev_charging.evdctgt_target_current = msg.dc_target_charging_current;
                println!(
                    "🔌 Received EVDCTGT: Voltage={:.1}V, Current={:.1}A",
                    self.ev_charging.evdctgt_target_voltage,
                    self.ev_charging.evdctgt_target_current
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVDCTGT: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVDCLIM1 - EV DC Charging Limits 1
    pub(crate) fn handle_evdclim1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVDCLIM1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evdclim1_max_voltage = msg.hrd_or_ev_d_chrgng_vltg_mxmm;
                self.ev_charging.evdclim1_max_current = msg.hrd_or_ev_d_chrgng_crrnt_mxmm;
                self.ev_charging.evdclim1_max_power = msg.hrd_or_ev_d_chrgng_pwr_mxmm;
                self.ev_charging.evdclim1_energy_transfer_type = msg.hrd_or_ev_rqstd_enrg_trnsfr_tp;
                println!(
                    "🔌 Received EVDCLIM1: MaxV={:.1}V, MaxA={:.1}A, MaxP={:.1}kW",
                    self.ev_charging.evdclim1_max_voltage,
                    self.ev_charging.evdclim1_max_current,
                    self.ev_charging.evdclim1_max_power
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVDCLIM1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVDCLIM2 - EV DC Charging Limits 2
    pub(crate) fn handle_evdclim2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVDCLIM2::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evdclim2_bulk_soc = msg.ev_bulk_state_of_charge;
                self.ev_charging.evdclim2_full_soc = msg.ev_full_state_of_charge;
                self.ev_charging.evdclim2_energy_capacity = msg.ev_energy_capacity;
                self.ev_charging.evdclim2_energy_requested = msg.ev_energy_requested;
                println!(
                    "🔌 Received EVDCLIM2: BulkSOC={:.1}%, FullSOC={:.1}%, Cap={} kWh",
                    self.ev_charging.evdclim2_bulk_soc,
                    self.ev_charging.evdclim2_full_soc,
                    self.ev_charging.evdclim2_energy_capacity
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVDCLIM2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVDCCIP - EV DC Charging In-Progress
    pub(crate) fn handle_evdccip(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVDCCIP::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evdccip_bulk_charging_complete = msg.ev_bulk_charging_complete;
                self.ev_charging.evdccip_full_charging_complete = msg.ev_full_charging_complete;
                self.ev_charging.evdccip_bulk_charge_time_remaining = msg.ev_bulk_charge_time_remaining;
                self.ev_charging.evdccip_full_charge_time_remaining = msg.ev_full_charge_time_remaining;
                self.ev_charging.evdccip_departure_time = msg.ev_departure_time;
                println!(
                    "🔌 Received EVDCCIP: Bulk={}, Full={}, TimeRemaining={:.0}s",
                    self.ev_charging.evdccip_bulk_charging_complete,
                    self.ev_charging.evdccip_full_charging_complete,
                    self.ev_charging.evdccip_full_charge_time_remaining
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVDCCIP: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVSE1CS1 - EV Supply Equipment 1 Contactor Status 1
    pub(crate) fn handle_evse1cs1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVSE1CS1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evse1cs1_contactor_input_voltage = msg.hrd_or_ev_d_cnttr_inpt_vltg;
                self.ev_charging.evse1cs1_charging_bus_voltage = msg.hybrid_or_ev_dc_charging_bus_voltage;
                self.ev_charging.evse1cs1_contactor_1_state = msg.hrd_or_ev_d_inlt_cnttr_1_stt;
                println!(
                    "🔌 Received EVSE1CS1: InputV={:.1}V, BusV={:.1}V, Contactor={}",
                    self.ev_charging.evse1cs1_contactor_input_voltage,
                    self.ev_charging.evse1cs1_charging_bus_voltage,
                    self.ev_charging.evse1cs1_contactor_1_state
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVSE1CS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVSE1CC1 - EV Supply Equipment 1 Contactor Command 1
    pub(crate) fn handle_evse1cc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVSE1CC1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evse1cc1_contactor_1_command = msg.hrd_or_ev_d_inlt_cnttr_1_cmmnd;
                println!(
                    "🔌 Received EVSE1CC1: Contactor command={}",
                    self.ev_charging.evse1cc1_contactor_1_command
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVSE1CC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVSEC1 - EVSE Control 1
    pub(crate) fn handle_evsec1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVSEC1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evsec1_connector_lock_request = msg.evse_connector_lock_request;
                self.ev_charging.evsec1_dc_stage_request = msg.evse_dc_stage_request;
                self.ev_charging.evsec1_ev_ready = msg.ev_ready;
                self.ev_charging.evsec1_contactor_command = msg.evse_contactor_command;
                println!(
                    "🔌 Received EVSEC1: Lock={}, Stage={}, Ready={}, Contactor={}",
                    self.ev_charging.evsec1_connector_lock_request,
                    self.ev_charging.evsec1_dc_stage_request,
                    self.ev_charging.evsec1_ev_ready,
                    self.ev_charging.evsec1_contactor_command
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVSEC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVSEDCS1 - EVSE DC Charging Status 1
    pub(crate) fn handle_evsedcs1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVSEDCS1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evsedcs1_dc_charging_state = msg.evse_dc_charging_state;
                self.ev_charging.evsedcs1_isolation_status = msg.evse_isolation_status;
                self.ev_charging.evsedcs1_present_voltage = msg.evse_present_dc_charging_voltage;
                self.ev_charging.evsedcs1_present_current = msg.evse_present_dc_charging_current;
                self.ev_charging.evsedcs1_voltage_limit_achieved = msg.evse_voltage_limit_achieved;
                self.ev_charging.evsedcs1_current_limit_achieved = msg.evse_current_limit_achieved;
                self.ev_charging.evsedcs1_power_limit_achieved = msg.evse_power_limit_achieved;
                self.ev_charging.evsedcs1_processing_state = msg.evse_processing_state;
                self.ev_charging.evsedcs1_status = msg.evse_status;
                self.ev_charging.evsedcs1_response_code = msg.evse_response_code;
                println!(
                    "🔌 Received EVSEDCS1: State={}, Voltage={:.1}V, Current={:.1}A, Status={}",
                    self.ev_charging.evsedcs1_dc_charging_state,
                    self.ev_charging.evsedcs1_present_voltage,
                    self.ev_charging.evsedcs1_present_current,
                    self.ev_charging.evsedcs1_status
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVSEDCS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVSES1 - EVSE Status 1
    pub(crate) fn handle_evses1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVSES1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evses1_connector_release_latch = msg.evse_connector_release_latch;
                self.ev_charging.evses1_manual_override = msg.evse_manual_override;
                self.ev_charging.evses1_connector_lock_state = msg.evse_connector_lock_state;
                self.ev_charging.evses1_connector_lock_permission = msg.evse_connector_lock_permission;
                self.ev_charging.evses1_inlet_contactor_state = msg.inlet_contactor_state;
                self.ev_charging.evses1_inlet_state = msg.evse_inlet_state;
                self.ev_charging.evses1_connection_type = msg.evse_connection_type;
                self.ev_charging.evses1_communications_physical_layer = msg.evse_communications_physical_layer;
                println!(
                    "🔌 Received EVSES1: Inlet={}, Lock={}, Connection={}",
                    self.ev_charging.evses1_inlet_state,
                    self.ev_charging.evses1_connector_lock_state,
                    self.ev_charging.evses1_connection_type
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVSES1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVSES2 - EVSE Status 2
    pub(crate) fn handle_evses2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVSES2::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evses2_temp_sensor_type = msg.evse_temp_sensor_type;
                self.ev_charging.evses2_connector_temperature_status = msg.evse_connector_temperature_status;
                self.ev_charging.evses2_inlet_connector_temperature = msg.ev_chrgng_inlt_cnntr_tmprtr;
                self.ev_charging.evses2_temp_sensor_resistance = msg.ev_c_it_ct_tpt_ss_rsst;
                println!(
                    "🔌 Received EVSES2: Temp={:.1}°C, Status={}, Resistance={}",
                    self.ev_charging.evses2_inlet_connector_temperature,
                    self.ev_charging.evses2_connector_temperature_status,
                    self.ev_charging.evses2_temp_sensor_resistance
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVSES2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVC - Engine Valve Control
    pub(crate) fn handle_evc(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVC::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evc_valve_control_modules[0] = msg.engn_vlv_cntrl_mdl_1_prlmnr_fm;
                self.ev_charging.evc_valve_control_modules[1] = msg.engn_vlv_cntrl_mdl_2_prlmnr_fm;
                self.ev_charging.evc_valve_control_modules[2] = msg.engn_vlv_cntrl_mdl_3_prlmnr_fm;
                self.ev_charging.evc_valve_control_modules[3] = msg.engn_vlv_cntrl_mdl_4_prlmnr_fm;
                self.ev_charging.evc_valve_control_modules[4] = msg.engn_vlv_cntrl_mdl_5_prlmnr_fm;
                self.ev_charging.evc_valve_control_modules[5] = msg.engn_vlv_cntrl_mdl_6_prlmnr_fm;
                self.ev_charging.evc_valve_control_modules[6] = msg.engn_vlv_cntrl_mdl_7_prlmnr_fm;
                self.ev_charging.evc_valve_control_modules[7] = msg.engn_vlv_cntrl_mdl_8_prlmnr_fm;
                println!("🔧 Received EVC: Valve control modules updated");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVC: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVEI - EV Energy Info
    pub(crate) fn handle_evei(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVEI::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evei_total_trip_energy_consumed = msg.total_trip_energy_consumed;
                self.ev_charging.evei_trip_drive_energy_economy = msg.trip_drive_energy_economy;
                println!(
                    "🔌 Received EVEI: Energy={:.1}kWh, Economy={:.2}kWh/km",
                    self.ev_charging.evei_total_trip_energy_consumed,
                    self.ev_charging.evei_trip_drive_energy_economy
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVEI: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVOI1 - EV Operating Info 1
    pub(crate) fn handle_evoi1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVOI1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evoi1_estimated_remaining_distance = msg.hvess_estimated_remaining_distance;
                println!(
                    "🔌 Received EVOI1: Remaining distance={:.1}km",
                    self.ev_charging.evoi1_estimated_remaining_distance
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVOI1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle HVBCS1 - HV Bus Contactor Status 1
    pub(crate) fn handle_hvbcs1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVBCS1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.hvbcs1_positive_contactor_states[0] = msg.hgh_vltg_bs_intrf_1_pstv_cnttr_stt;
                self.ev_charging.hvbcs1_negative_contactor_states[0] = msg.hgh_vltg_bs_intrf_1_ngtv_cnttr_stt;
                self.ev_charging.hvbcs1_positive_contactor_states[1] = msg.hgh_vltg_bs_intrf_2_pstv_cnttr_stt;
                self.ev_charging.hvbcs1_negative_contactor_states[1] = msg.hgh_vltg_bs_intrf_2_ngtv_cnttr_stt;
                self.ev_charging.hvbcs1_embedded_integrity_support = msg.hvbcs_1_embedded_integrity_support;
                self.ev_charging.hvbcs1_counter = msg.hvbcs_1_counter;
                println!(
                    "🔌 Received HVBCS1: IF1+={}, IF1-={}, IF2+={}, IF2-={}",
                    self.ev_charging.hvbcs1_positive_contactor_states[0],
                    self.ev_charging.hvbcs1_negative_contactor_states[0],
                    self.ev_charging.hvbcs1_positive_contactor_states[1],
                    self.ev_charging.hvbcs1_negative_contactor_states[1]
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVBCS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle HVBCS2 - HV Bus Contactor Status 2
    pub(crate) fn handle_hvbcs2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVBCS2::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.hvbcs2_embedded_integrity_support = msg.hvbcs_2_embedded_integrity_support;
                self.ev_charging.hvbcs2_counter = msg.hvbcs_2_counter;
                println!("🔌 Received HVBCS2: Counter={}", self.ev_charging.hvbcs2_counter);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVBCS2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle HVBCS3 - HV Bus Contactor Status 3
    pub(crate) fn handle_hvbcs3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVBCS3::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.hvbcs3_embedded_integrity_support = msg.hvbcs_3_embedded_integrity_support;
                self.ev_charging.hvbcs3_counter = msg.hvbcs_3_counter;
                println!("🔌 Received HVBCS3: Counter={}", self.ev_charging.hvbcs3_counter);
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVBCS3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle HVBCC1 - HV Bus Contactor Command 1
    pub(crate) fn handle_hvbcc1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVBCC1::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.hvbcc1_connect_commands[0] = msg.hgh_vltg_bs_intrf_1_cnnt_cmmnd;
                self.ev_charging.hvbcc1_connect_commands[1] = msg.hgh_vltg_bs_intrf_2_cnnt_cmmnd;
                self.ev_charging.hvbcc1_connect_commands[2] = msg.hgh_vltg_bs_intrf_3_cnnt_cmmnd;
                self.ev_charging.hvbcc1_connect_commands[3] = msg.hgh_vltg_bs_intrf_4_cnnt_cmmnd;
                self.ev_charging.hvbcc1_embedded_integrity_support = msg.hvbcc_1_embedded_integrity_support;
                self.ev_charging.hvbcc1_counter = msg.hvbcc_1_counter;
                println!(
                    "🔌 Received HVBCC1: IF1={}, IF2={}, IF3={}, IF4={}",
                    self.ev_charging.hvbcc1_connect_commands[0],
                    self.ev_charging.hvbcc1_connect_commands[1],
                    self.ev_charging.hvbcc1_connect_commands[2],
                    self.ev_charging.hvbcc1_connect_commands[3]
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVBCC1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle HVBCC2 - HV Bus Contactor Command 2
    pub(crate) fn handle_hvbcc2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVBCC2::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.hvbcc2_connect_commands[0] = msg.hgh_vltg_bs_intrf_25_cnnt_cmmnd;
                self.ev_charging.hvbcc2_connect_commands[1] = msg.hgh_vltg_bs_intrf_26_cnnt_cmmnd;
                self.ev_charging.hvbcc2_connect_commands[2] = msg.hgh_vltg_bs_intrf_27_cnnt_cmmnd;
                self.ev_charging.hvbcc2_connect_commands[3] = msg.hgh_vltg_bs_intrf_28_cnnt_cmmnd;
                self.ev_charging.hvbcc2_embedded_integrity_support = msg.hvbcc_2_embedded_integrity_support;
                self.ev_charging.hvbcc2_counter = msg.hvbcc_2_counter;
                println!(
                    "🔌 Received HVBCC2: IF25={}, IF26={}, IF27={}, IF28={}",
                    self.ev_charging.hvbcc2_connect_commands[0],
                    self.ev_charging.hvbcc2_connect_commands[1],
                    self.ev_charging.hvbcc2_connect_commands[2],
                    self.ev_charging.hvbcc2_connect_commands[3]
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVBCC2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle HVBI - HV Bus Info
    pub(crate) fn handle_hvbi(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match HVBI::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.hvbi_dc_bus_availability = msg.high_voltage_dc_bus_availability;
                self.ev_charging.hvbi_driveline_availability = msg.hgh_vltg_bs_drvln_avllt;
                self.ev_charging.hvbi_auxiliaries_availability = msg.hgh_vltg_bs_axlrs_avllt;
                self.ev_charging.hvbi_epto_availability = msg.high_voltage_bus_epto_availability;
                self.ev_charging.hvbi_on_board_charger_availability = msg.hgh_vltg_bs_on_brd_chrgr_avllt;
                self.ev_charging.hvbi_off_board_charger_availability = msg.hgh_vltg_bs_off_brd_chrgr_avllt;
                println!(
                    "🔌 Received HVBI: DCBus={}, Driveline={}, Aux={}, Charger={}",
                    self.ev_charging.hvbi_dc_bus_availability,
                    self.ev_charging.hvbi_driveline_availability,
                    self.ev_charging.hvbi_auxiliaries_availability,
                    self.ev_charging.hvbi_off_board_charger_availability
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode HVBI: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle EVVT - Engine VVT
    pub(crate) fn handle_evvt(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match EVVT::decode(can_id, data) {
            Ok(msg) => {
                self.ev_charging.evvt_intake_commanded_offset = msg.e_vvt_it_vv_cst_cdd_t_ost;
                self.ev_charging.evvt_intake_offset_position = msg.e_vvt_it_vv_cst_t_ost_pst;
                self.ev_charging.evvt_exhaust_commanded_offset = msg.e_vvt_exst_vv_cst_cdd_t_ost;
                self.ev_charging.evvt_exhaust_offset_position = msg.e_vvt_exst_vv_cst_t_ost_pst;
                println!(
                    "🔧 Received EVVT: IntakeCmd={:.1}°, IntakePos={:.1}°, ExhaustCmd={:.1}°, ExhaustPos={:.1}°",
                    self.ev_charging.evvt_intake_commanded_offset,
                    self.ev_charging.evvt_intake_offset_position,
                    self.ev_charging.evvt_exhaust_commanded_offset,
                    self.ev_charging.evvt_exhaust_offset_position
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode EVVT: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }
}
