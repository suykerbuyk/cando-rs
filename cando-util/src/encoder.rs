//! Message encoder for cando-util
//!
//! This module provides the infrastructure for encoding CAN messages from field assignments.
//! It handles dynamic message type selection, field mapping, and device ID embedding.
//!
//! Currently supports J1939 messages with implemented encode methods:
//! - J1939: CN, WAND, LDISP, EEC12, ETC5, AEBS2, ALTC, GC2, and many more

use anyhow::{Result, anyhow};
use cando_messages::common::DeviceId;
use cando_messages::j1939::{self, *};
use std::collections::HashMap;

/// Encoded CAN message with metadata
#[derive(Debug, Clone)]
pub struct EncodedMessage {
    /// CAN identifier (with device ID embedded)
    pub can_id: u32,
    /// Encoded message data bytes
    pub data: Vec<u8>,
    /// Message name
    pub message_name: String,
    /// Protocol name (EMP or HVPC)
    pub protocol: String,
}

/// Encode a CAN message from field assignments
///
/// This function attempts to find the message in all supported protocols
/// and encode it with the given field values.
pub fn encode_message(
    message_name: &str,
    device_id: DeviceId,
    field_map: &HashMap<String, f64>,
) -> Result<EncodedMessage> {
    // Try J1939 protocol
    if let Some(result) = try_encode_j1939(message_name, device_id, field_map)? {
        return Ok(result);
    }

    Err(anyhow!(
        "Message '{}' not found in any supported protocol or not yet supported for encoding",
        message_name
    ))
}

/// Attempt to encode a J1939 protocol message
///
/// Returns None if the message name doesn't match any J1939 message.
fn try_encode_j1939(
    message_name: &str,
    device_id: DeviceId,
    field_map: &HashMap<String, f64>,
) -> Result<Option<EncodedMessage>> {
    match message_name {
        "CN" => {
            let mut msg = CN {
                device_id,
                crash_checksum: 0,
                crash_counter: 0,
                crash_type: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "crash_checksum" => {
                        msg.crash_checksum = *value as u8;
                    }
                    "crash_counter" => {
                        msg.crash_counter = *value as u8;
                    }
                    "crash_type" => {
                        msg.crash_type = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "WAND" => {
            let mut msg = WAND {
                device_id,
                wand_angle: 0.0,
                wand_sensor_figure_of_merit: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "wand_angle" => {
                        msg.wand_angle = *value;
                    }
                    "wand_sensor_figure_of_merit" => {
                        msg.wand_sensor_figure_of_merit = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "LDISP" => {
            let mut msg = LDISP {
                device_id,
                measured_linear_displacement: 0.0,
                lnr_dsplmnt_snsr_snsr_fgr_of_mrt: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "measured_linear_displacement" => {
                        msg.measured_linear_displacement = *value;
                    }
                    "lnr_dsplmnt_snsr_snsr_fgr_of_mrt" | "quality" => {
                        msg.lnr_dsplmnt_snsr_snsr_fgr_of_mrt = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC12" => {
            let mut msg = EEC12 {
                device_id,
                engn_exhst_1_gs_snsr_1_pwr_sppl: 0,
                aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl: 0,
                engn_exhst_2_gs_snsr_1_pwr_sppl: 0,
                aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl: 0,
                engn_exhst_1_gs_snsr_2_pwr_sppl: 0,
                aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_exhst_1_gs_snsr_1_pwr_sppl" => {
                        msg.engn_exhst_1_gs_snsr_1_pwr_sppl = *value as u8;
                    }
                    "aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl" => {
                        msg.aftrtrtmnt_1_otlt_1_gs_snsr_pwr_sppl = *value as u8;
                    }
                    "engn_exhst_2_gs_snsr_1_pwr_sppl" => {
                        msg.engn_exhst_2_gs_snsr_1_pwr_sppl = *value as u8;
                    }
                    "aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl" => {
                        msg.aftrtrtmnt_2_otlt_1_gs_snsr_pwr_sppl = *value as u8;
                    }
                    "engn_exhst_1_gs_snsr_2_pwr_sppl" => {
                        msg.engn_exhst_1_gs_snsr_2_pwr_sppl = *value as u8;
                    }
                    "aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl" => {
                        msg.aftrtrtmnt_1_otlt_2_gs_snsr_pwr_sppl = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "ETC5" => {
            let mut msg = ETC5 {
                device_id,
                trnsmssn_hgh_rng_sns_swth: 0,
                transmission_low_range_sense_switch: 0,
                transmission_splitter_position: 0,
                trnsmssn_rvrs_drtn_swth: 0,
                transmission_neutral_switch: 0,
                trnsmssn_frwrd_drtn_swth: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "trnsmssn_hgh_rng_sns_swth" => {
                        msg.trnsmssn_hgh_rng_sns_swth = *value as u8;
                    }
                    "transmission_low_range_sense_switch" => {
                        msg.transmission_low_range_sense_switch = *value as u8;
                    }
                    "transmission_splitter_position" => {
                        msg.transmission_splitter_position = *value as u8;
                    }
                    "trnsmssn_rvrs_drtn_swth" => {
                        msg.trnsmssn_rvrs_drtn_swth = *value as u8;
                    }
                    "transmission_neutral_switch" => {
                        msg.transmission_neutral_switch = *value as u8;
                    }
                    "trnsmssn_frwrd_drtn_swth" => {
                        msg.trnsmssn_frwrd_drtn_swth = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "AEBS2" => {
            let mut msg = AEBS2 {
                device_id,
                dv_atvt_dd_f_advd_eb_sst: 0,
                aebs_2_message_counter: 0,
                aebs_2_message_checksum: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "dv_atvt_dd_f_advd_eb_sst" => {
                        msg.dv_atvt_dd_f_advd_eb_sst = *value as u8;
                    }
                    "aebs_2_message_counter" => {
                        msg.aebs_2_message_counter = *value as u8;
                    }
                    "aebs_2_message_checksum" => {
                        msg.aebs_2_message_checksum = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "ALTC" => {
            let mut msg = ALTC {
                device_id,
                altrntr_stpnt_vltg_cmmnd: 0.0,
                altrntr_exttn_mxmm_crrnt_lmt: 0.0,
                alternator_torque_ramp_time_command: 0.0,
                altrntr_trq_rmp_mxmm_spd_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "altrntr_stpnt_vltg_cmmnd" => {
                        msg.altrntr_stpnt_vltg_cmmnd = *value;
                    }
                    "altrntr_exttn_mxmm_crrnt_lmt" => {
                        msg.altrntr_exttn_mxmm_crrnt_lmt = *value;
                    }
                    "alternator_torque_ramp_time_command" => {
                        msg.alternator_torque_ramp_time_command = *value;
                    }
                    "altrntr_trq_rmp_mxmm_spd_cmmnd" => {
                        msg.altrntr_trq_rmp_mxmm_spd_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "GC2" => {
            let mut msg = GC2 {
                device_id,
                engine_load_setpoint_request: 0.0,
                engine_self_induced_derate_inhibit: 0,
                generator_governing_bias: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engine_load_setpoint_request" => {
                        msg.engine_load_setpoint_request = *value;
                    }
                    "engine_self_induced_derate_inhibit" => {
                        msg.engine_self_induced_derate_inhibit = *value as u8;
                    }
                    "generator_governing_bias" => {
                        msg.generator_governing_bias = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DCACAI1S2" => {
            let mut msg = DCACAI1S2 {
                device_id,
                da_assr_invrtr_1_d_sd_pwr: 0.0,
                da_assr_invrtr_1_d_sd_vltg: 0.0,
                da_assr_invrtr_1_d_sd_crrnt: 0.0,
                da_ass_ivt_1_d_sd_ntv_t_csss_gd_vt: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "da_assr_invrtr_1_d_sd_pwr" => {
                        msg.da_assr_invrtr_1_d_sd_pwr = *value;
                    }
                    "da_assr_invrtr_1_d_sd_vltg" => {
                        msg.da_assr_invrtr_1_d_sd_vltg = *value;
                    }
                    "da_assr_invrtr_1_d_sd_crrnt" => {
                        msg.da_assr_invrtr_1_d_sd_crrnt = *value;
                    }
                    "da_ass_ivt_1_d_sd_ntv_t_csss_gd_vt" => {
                        msg.da_ass_ivt_1_d_sd_ntv_t_csss_gd_vt = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DCDC1OS" => {
            let mut msg = DCDC1OS {
                device_id,
                dc_dc_1_hvil_status: 0,
                dc_dc_1_loadshed_request: 0,
                dc_dc_1_operational_status: 0,
                dc_dc_1_operating_status_counter: 0,
                dc_dc_1_operating_status_crc: 0,
                dd_1_pwr_lmt_dt_hgh_sd_crrnt: 0,
                dd_1_pwr_lmt_dt_lw_sd_crrnt: 0,
                dd_1_pwr_lmt_dt_hgh_sd_vltg_mnmm: 0,
                dd_1_pwr_lmt_dt_hgh_sd_vltg_mxmm: 0,
                dd_1_pwr_lmt_dt_lw_sd_vltg_mnmm: 0,
                dd_1_pwr_lmt_dt_lw_sd_vltg_mxmm: 0,
                dd_1_pwr_lmt_dt_cnvrtr_tmprtr: 0,
                dd_1_pwr_lmt_dt_eltrn_fltr_tmprtr: 0,
                dd_1_pwr_lmt_dt_pwr_eltrns_tmprtr: 0,
                dd_1_pwr_lmt_dt_sl_bttr_trmnl_vltg: 0,
                dd_1_pwr_lmt_dt_sl_bttr_trmnl_crrnt: 0,
                dd_1_pwr_lmt_dt_sl_bttr_trmnl_tmprtr: 0,
                dd_1_pwr_lmt_dt_undfnd_rsn: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "dc_dc_1_hvil_status" => {
                        msg.dc_dc_1_hvil_status = *value as u8;
                    }
                    "dc_dc_1_loadshed_request" => {
                        msg.dc_dc_1_loadshed_request = *value as u8;
                    }
                    "dc_dc_1_operational_status" => {
                        msg.dc_dc_1_operational_status = *value as u8;
                    }
                    "dc_dc_1_operating_status_counter" => {
                        msg.dc_dc_1_operating_status_counter = *value as u8;
                    }
                    "dc_dc_1_operating_status_crc" => {
                        msg.dc_dc_1_operating_status_crc = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_hgh_sd_crrnt" => {
                        msg.dd_1_pwr_lmt_dt_hgh_sd_crrnt = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_lw_sd_crrnt" => {
                        msg.dd_1_pwr_lmt_dt_lw_sd_crrnt = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_hgh_sd_vltg_mnmm" => {
                        msg.dd_1_pwr_lmt_dt_hgh_sd_vltg_mnmm = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_hgh_sd_vltg_mxmm" => {
                        msg.dd_1_pwr_lmt_dt_hgh_sd_vltg_mxmm = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_lw_sd_vltg_mnmm" => {
                        msg.dd_1_pwr_lmt_dt_lw_sd_vltg_mnmm = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_lw_sd_vltg_mxmm" => {
                        msg.dd_1_pwr_lmt_dt_lw_sd_vltg_mxmm = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_cnvrtr_tmprtr" => {
                        msg.dd_1_pwr_lmt_dt_cnvrtr_tmprtr = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_eltrn_fltr_tmprtr" => {
                        msg.dd_1_pwr_lmt_dt_eltrn_fltr_tmprtr = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_pwr_eltrns_tmprtr" => {
                        msg.dd_1_pwr_lmt_dt_pwr_eltrns_tmprtr = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_sl_bttr_trmnl_vltg" => {
                        msg.dd_1_pwr_lmt_dt_sl_bttr_trmnl_vltg = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_sl_bttr_trmnl_crrnt" => {
                        msg.dd_1_pwr_lmt_dt_sl_bttr_trmnl_crrnt = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_sl_bttr_trmnl_tmprtr" => {
                        msg.dd_1_pwr_lmt_dt_sl_bttr_trmnl_tmprtr = *value as u8;
                    }
                    "dd_1_pwr_lmt_dt_undfnd_rsn" => {
                        msg.dd_1_pwr_lmt_dt_undfnd_rsn = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DCDC1SBS" => {
            let mut msg = DCDC1SBS {
                device_id,
                dc_dc_1_sli_battery_terminal_current: 0.0,
                dc_dc_1_sli_battery_terminal_voltage: 0.0,
                dd_1_sl_bttr_trmnl_tmprtr: 0.0,
            };

            // Apply field values
            for (field_name, value) in field_map.iter() {
                match field_name.as_str() {
                    "dc_dc_1_sli_battery_terminal_current" => {
                        msg.dc_dc_1_sli_battery_terminal_current = *value;
                    }
                    "dc_dc_1_sli_battery_terminal_voltage" => {
                        msg.dc_dc_1_sli_battery_terminal_voltage = *value;
                    }
                    "dd_1_sl_bttr_trmnl_tmprtr" => {
                        msg.dd_1_sl_bttr_trmnl_tmprtr = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DCDC1S2" => {
            let mut msg = DCDC1S2 {
                device_id,
                dc_dc_1_high_side_power: 0.0,
                dc_dc_1_low_side_power: 0.0,
                dd_1_hgh_sd_ngtv_t_chsss_grnd_vltg: 0.0,
            };

            // Apply field values
            for (field_name, value) in field_map.iter() {
                match field_name.as_str() {
                    "dc_dc_1_high_side_power" => {
                        msg.dc_dc_1_high_side_power = *value;
                    }
                    "dc_dc_1_low_side_power" => {
                        msg.dc_dc_1_low_side_power = *value;
                    }
                    "dd_1_hgh_sd_ngtv_t_chsss_grnd_vltg" => {
                        msg.dd_1_hgh_sd_ngtv_t_chsss_grnd_vltg = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DCACAI1V" => {
            let mut msg = DCACAI1V {
                device_id,
                da_assr_invrtr_1_igntn_vltg: 0.0,
                da_assr_invrtr_1_unswthd_sl_vltg: 0.0,
            };

            // Apply field values
            for (field_name, value) in field_map.iter() {
                match field_name.as_str() {
                    "da_assr_invrtr_1_igntn_vltg" => {
                        msg.da_assr_invrtr_1_igntn_vltg = *value;
                    }
                    "da_assr_invrtr_1_unswthd_sl_vltg" => {
                        msg.da_assr_invrtr_1_unswthd_sl_vltg = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "GTRACE" => {
            let mut msg = GTRACE {
                device_id,
                generator_trip_kw_hours_export: 0,
                generator_trip_kvar_hours_export: 0,
            };

            // Apply field values
            for (field_name, value) in field_map.iter() {
                match field_name.as_str() {
                    "generator_trip_kw_hours_export" => {
                        msg.generator_trip_kw_hours_export = *value as u32;
                    }
                    "generator_trip_kvar_hours_export" => {
                        msg.generator_trip_kvar_hours_export = *value as u32;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DCDC2SBS" => {
            let mut msg = DCDC2SBS {
                device_id,
                dc_dc_2_sli_battery_terminal_voltage: 0.0,
                dc_dc_2_sli_battery_terminal_current: 0.0,
                dd_2_sl_bttr_trmnl_tmprtr: 0.0,
            };

            // Apply field values
            for (field_name, value) in field_map.iter() {
                match field_name.as_str() {
                    "dc_dc_2_sli_battery_terminal_voltage" => {
                        msg.dc_dc_2_sli_battery_terminal_voltage = *value;
                    }
                    "dc_dc_2_sli_battery_terminal_current" => {
                        msg.dc_dc_2_sli_battery_terminal_current = *value;
                    }
                    "dd_2_sl_bttr_trmnl_tmprtr" => {
                        msg.dd_2_sl_bttr_trmnl_tmprtr = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DCDC2S2" => {
            let mut msg = DCDC2S2 {
                device_id,
                dc_dc_2_high_side_power: 0.0,
                dc_dc_2_low_side_power: 0.0,
                dd_2_hgh_sd_ngtv_t_chsss_grnd_vltg: 0.0,
            };

            // Apply field values
            for (field_name, value) in field_map.iter() {
                match field_name.as_str() {
                    "dc_dc_2_high_side_power" => {
                        msg.dc_dc_2_high_side_power = *value;
                    }
                    "dc_dc_2_low_side_power" => {
                        msg.dc_dc_2_low_side_power = *value;
                    }
                    "dd_2_hgh_sd_ngtv_t_chsss_grnd_vltg" => {
                        msg.dd_2_hgh_sd_ngtv_t_chsss_grnd_vltg = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC22" => {
            let mut msg = EEC22 {
                device_id,
                engn_exhst_gs_rrltn_1_clr_intk_prssr: 0.0,
                ttl_nmr_of_crnk_attmpts_drng_engn_lf: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_exhst_gs_rrltn_1_clr_intk_prssr" => {
                        msg.engn_exhst_gs_rrltn_1_clr_intk_prssr = *value;
                    }
                    "ttl_nmr_of_crnk_attmpts_drng_engn_lf" => {
                        msg.ttl_nmr_of_crnk_attmpts_drng_engn_lf = *value as u32;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC23" => {
            let mut msg = EEC23 {
                device_id,
                engn_crnks_prssr_cntrl_attr_1_cmmnd: 0.0,
                engn_crnks_prssr_cntrl_attr_2_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_crnks_prssr_cntrl_attr_1_cmmnd" => {
                        msg.engn_crnks_prssr_cntrl_attr_1_cmmnd = *value;
                    }
                    "engn_crnks_prssr_cntrl_attr_2_cmmnd" => {
                        msg.engn_crnks_prssr_cntrl_attr_2_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGF2" => {
            let mut msg = EGF2 {
                device_id,
                engn_trhrgr_1_cmprssr_intk_flw_rt: 0.0,
                engn_trhrgr_2_cmprssr_intk_flw_rt: 0.0,
                engn_intk_ar_mss_flw_rt_extndd_rng: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_trhrgr_1_cmprssr_intk_flw_rt" => {
                        msg.engn_trhrgr_1_cmprssr_intk_flw_rt = *value;
                    }
                    "engn_trhrgr_2_cmprssr_intk_flw_rt" => {
                        msg.engn_trhrgr_2_cmprssr_intk_flw_rt = *value;
                    }
                    "engn_intk_ar_mss_flw_rt_extndd_rng" => {
                        msg.engn_intk_ar_mss_flw_rt_extndd_rng = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "ETC12" => {
            let mut msg = ETC12 {
                device_id,
                trnsmssn_hdrstt_lp_1_prssr: 0,
                trnsmssn_hdrstt_lp_2_prssr: 0,
                trnsmssn_drtnl_otpt_shft_spd: 0.0,
                trnsmssn_intrmdt_shft_spd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "trnsmssn_hdrstt_lp_1_prssr" => {
                        msg.trnsmssn_hdrstt_lp_1_prssr = *value as u16;
                    }
                    "trnsmssn_hdrstt_lp_2_prssr" => {
                        msg.trnsmssn_hdrstt_lp_2_prssr = *value as u16;
                    }
                    "trnsmssn_drtnl_otpt_shft_spd" => {
                        msg.trnsmssn_drtnl_otpt_shft_spd = *value;
                    }
                    "trnsmssn_intrmdt_shft_spd" => {
                        msg.trnsmssn_intrmdt_shft_spd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGF1" => {
            let mut msg = EGF1 {
                device_id,
                engn_exhst_gs_rrltn_1_mss_flw_rt: 0.0,
                engine_intake_air_mass_flow_rate: 0.0,
                engn_exhst_gs_rrltn_2_mss_flw_rt: 0.0,
                target_fresh_air_mass_flow: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_exhst_gs_rrltn_1_mss_flw_rt" => {
                        msg.engn_exhst_gs_rrltn_1_mss_flw_rt = *value;
                    }
                    "engine_intake_air_mass_flow_rate" => {
                        msg.engine_intake_air_mass_flow_rate = *value;
                    }
                    "engn_exhst_gs_rrltn_2_mss_flw_rt" => {
                        msg.engn_exhst_gs_rrltn_2_mss_flw_rt = *value;
                    }
                    "target_fresh_air_mass_flow" => {
                        msg.target_fresh_air_mass_flow = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVDC1" => {
            let mut msg = EGFVDC1 {
                device_id,
                engn_gss_fl_vlv_1_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_2_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_3_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_4_drtn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_1_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_1_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_2_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_2_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_3_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_3_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_4_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_4_drtn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVDC2" => {
            let mut msg = EGFVDC2 {
                device_id,
                engn_gss_fl_vlv_5_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_6_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_7_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_8_drtn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_5_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_5_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_6_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_6_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_7_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_7_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_8_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_8_drtn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVDC3" => {
            let mut msg = EGFVDC3 {
                device_id,
                engn_gss_fl_vlv_9_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_10_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_11_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_12_drtn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_9_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_9_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_10_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_10_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_11_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_11_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_12_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_12_drtn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVDC4" => {
            let mut msg = EGFVDC4 {
                device_id,
                engn_gss_fl_vlv_13_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_14_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_15_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_16_drtn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_13_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_13_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_14_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_14_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_15_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_15_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_16_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_16_drtn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVDC5" => {
            let mut msg = EGFVDC5 {
                device_id,
                engn_gss_fl_vlv_17_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_18_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_19_drtn_cmmnd: 0.0,
                engn_gss_fl_vlv_20_drtn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_17_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_17_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_18_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_18_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_19_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_19_drtn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_20_drtn_cmmnd" => {
                        msg.engn_gss_fl_vlv_20_drtn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVSOAC1" => {
            let mut msg = EGFVSOAC1 {
                device_id,
                engn_gss_fl_vlv_1_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_2_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_3_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_4_strt_of_attn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_1_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_1_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_2_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_2_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_3_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_3_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_4_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_4_strt_of_attn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVSOAC2" => {
            let mut msg = EGFVSOAC2 {
                device_id,
                engn_gss_fl_vlv_5_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_6_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_7_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_8_strt_of_attn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_5_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_5_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_6_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_6_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_7_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_7_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_8_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_8_strt_of_attn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVSOAC3" => {
            let mut msg = EGFVSOAC3 {
                device_id,
                engn_gss_fl_vlv_9_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_10_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_11_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_12_strt_of_attn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_9_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_9_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_10_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_10_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_11_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_11_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_12_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_12_strt_of_attn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVSOAC4" => {
            let mut msg = EGFVSOAC4 {
                device_id,
                engn_gss_fl_vlv_13_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_14_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_15_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_16_strt_of_attn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_13_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_13_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_14_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_14_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_15_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_15_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_16_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_16_strt_of_attn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EGFVSOAC5" => {
            let mut msg = EGFVSOAC5 {
                device_id,
                engn_gss_fl_vlv_17_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_18_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_19_strt_of_attn_cmmnd: 0.0,
                engn_gss_fl_vlv_20_strt_of_attn_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_gss_fl_vlv_17_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_17_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_18_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_18_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_19_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_19_strt_of_attn_cmmnd = *value;
                    }
                    "engn_gss_fl_vlv_20_strt_of_attn_cmmnd" => {
                        msg.engn_gss_fl_vlv_20_strt_of_attn_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC18" => {
            let mut msg = EEC18 {
                device_id,
                engine_intake_air_source_valve: 0,
                engn_clndr_hd_bpss_attr_1_cmmnd: 0.0,
                engn_exhst_gs_rstrtn_vlv_pstn: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engine_intake_air_source_valve" => {
                        msg.engine_intake_air_source_valve = *value as u8;
                    }
                    "engn_clndr_hd_bpss_attr_1_cmmnd" => {
                        msg.engn_clndr_hd_bpss_attr_1_cmmnd = *value;
                    }
                    "engn_exhst_gs_rstrtn_vlv_pstn" => {
                        msg.engn_exhst_gs_rstrtn_vlv_pstn = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC21" => {
            let mut msg = EEC21 {
                device_id,
                engn_exhst_mnfld_aslt_prssr_1: 0.0,
                engn_exhst_mnfld_aslt_prssr_2: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_exhst_mnfld_aslt_prssr_1" => {
                        msg.engn_exhst_mnfld_aslt_prssr_1 = *value;
                    }
                    "engn_exhst_mnfld_aslt_prssr_2" => {
                        msg.engn_exhst_mnfld_aslt_prssr_2 = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "ETCC2" => {
            let mut msg = ETCC2 {
                device_id,
                engn_stgd_trhrgr_slnd_stts: 0.0,
                nmr_of_engn_trhrgrs_cmmndd: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_stgd_trhrgr_slnd_stts" => {
                        msg.engn_stgd_trhrgr_slnd_stts = *value;
                    }
                    "nmr_of_engn_trhrgrs_cmmndd" => {
                        msg.nmr_of_engn_trhrgrs_cmmndd = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "ETCC4" => {
            let mut msg = ETCC4 {
                device_id,
                engn_trhrgr_wstgt_attr_3_cmmnd: 0.0,
                engn_trhrgr_wstgt_attr_4_cmmnd: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_trhrgr_wstgt_attr_3_cmmnd" => {
                        msg.engn_trhrgr_wstgt_attr_3_cmmnd = *value;
                    }
                    "engn_trhrgr_wstgt_attr_4_cmmnd" => {
                        msg.engn_trhrgr_wstgt_attr_4_cmmnd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "HVESSC1" => {
            let mut msg = HVESSC1 {
                device_id,
                hvess_power_down_command: 0,
                hvess_cell_balancing_command: 0,
                hvess_operation_consent: 0,
                hvess_control_1_counter: 0,
                hvess_control_1_crc: 0,
                hvss_hgh_vltg_bs_cnnt_cmmnd: 0,
                hvss_hgh_vltg_bs_atv_isltn_tst_cmmnd: 0,
                hvss_hgh_vltg_bs_pssv_isltn_tst_cmmnd: 0,
                hvss_enl_intrnl_chrgr_cmmnd: 0,
                hvss_hgh_vltg_bs_hgh_sd_rsstr_cnnt_rqst: 0,
                hvss_hgh_vltg_bs_lw_sd_rsstr_cnnt_rqst: 0,
                hvss_thrml_mngmnt_mntnn_rqst: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "hvess_power_down_command" => {
                        msg.hvess_power_down_command = *value as u8;
                    }
                    "hvess_cell_balancing_command" => {
                        msg.hvess_cell_balancing_command = *value as u8;
                    }
                    "hvess_operation_consent" => {
                        msg.hvess_operation_consent = *value as u8;
                    }
                    "hvess_control_1_counter" => {
                        msg.hvess_control_1_counter = *value as u8;
                    }
                    "hvess_control_1_crc" => {
                        msg.hvess_control_1_crc = *value as u8;
                    }
                    "hvss_hgh_vltg_bs_cnnt_cmmnd" => {
                        msg.hvss_hgh_vltg_bs_cnnt_cmmnd = *value as u8;
                    }
                    "hvss_hgh_vltg_bs_atv_isltn_tst_cmmnd" => {
                        msg.hvss_hgh_vltg_bs_atv_isltn_tst_cmmnd = *value as u8;
                    }
                    "hvss_hgh_vltg_bs_pssv_isltn_tst_cmmnd" => {
                        msg.hvss_hgh_vltg_bs_pssv_isltn_tst_cmmnd = *value as u8;
                    }
                    "hvss_enl_intrnl_chrgr_cmmnd" => {
                        msg.hvss_enl_intrnl_chrgr_cmmnd = *value as u8;
                    }
                    "hvss_hgh_vltg_bs_hgh_sd_rsstr_cnnt_rqst" => {
                        msg.hvss_hgh_vltg_bs_hgh_sd_rsstr_cnnt_rqst = *value as u8;
                    }
                    "hvss_hgh_vltg_bs_lw_sd_rsstr_cnnt_rqst" => {
                        msg.hvss_hgh_vltg_bs_lw_sd_rsstr_cnnt_rqst = *value as u8;
                    }
                    "hvss_thrml_mngmnt_mntnn_rqst" => {
                        msg.hvss_thrml_mngmnt_mntnn_rqst = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "HVESSD1" => {
            let mut msg = HVESSD1 {
                device_id,
                hvess_available_discharge_power: 0.0,
                hvess_available_charge_power: 0.0,
                hvess_voltage_level: 0.0,
                hvess_current: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "hvess_available_discharge_power" => {
                        msg.hvess_available_discharge_power = *value;
                    }
                    "hvess_available_charge_power" => {
                        msg.hvess_available_charge_power = *value;
                    }
                    "hvess_voltage_level" => {
                        msg.hvess_voltage_level = *value;
                    }
                    "hvess_current" => {
                        msg.hvess_current = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "HVESSD2" => {
            let mut msg = HVESSD2 {
                device_id,
                hvess_fast_update_state_of_charge: 0.0,
                hvess_highest_cell_voltage: 0.0,
                hvess_lowest_cell_voltage: 0.0,
                hvss_cll_vltg_dffrntl_stts: 0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "hvess_fast_update_state_of_charge"
                    | "stateofcharge"
                    | "soc"
                    | "charge"
                    | "batterylevel" => {
                        msg.hvess_fast_update_state_of_charge = *value;
                    }
                    "hvess_highest_cell_voltage"
                    | "highestvoltagage"
                    | "maxvoltage"
                    | "highvoltage"
                    | "maxcellvoltage" => {
                        msg.hvess_highest_cell_voltage = *value;
                    }
                    "hvess_lowest_cell_voltage"
                    | "lowestvoltage"
                    | "minvoltage"
                    | "lowvoltage"
                    | "mincellvoltage" => {
                        msg.hvess_lowest_cell_voltage = *value;
                    }
                    "hvss_cll_vltg_dffrntl_stts"
                    | "voltagestatus"
                    | "voltagedifferential"
                    | "differentialstatus"
                    | "cellvoltagestatus" => {
                        msg.hvss_cll_vltg_dffrntl_stts = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for HVESSD2 message",
                            field_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "HVESSD3" => {
            let mut msg = HVESSD3 {
                device_id,
                hvess_highest_cell_temperature: 0.0,
                hvess_lowest_cell_temperature: 0.0,
                hvess_average_cell_temperature: 0.0,
                hvss_cll_tmprtr_dffrntl_stts: 0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "hvess_highest_cell_temperature"
                    | "highesttemperature"
                    | "maxtemperature"
                    | "highestcelltemp"
                    | "maxcelltemp" => {
                        msg.hvess_highest_cell_temperature = *value;
                    }
                    "hvess_lowest_cell_temperature"
                    | "lowesttemperature"
                    | "mintemperature"
                    | "lowestcelltemp"
                    | "mincelltemp" => {
                        msg.hvess_lowest_cell_temperature = *value;
                    }
                    "hvess_average_cell_temperature"
                    | "averagetemperature"
                    | "avgtemperature"
                    | "averagecelltemp"
                    | "avgcelltemp" => {
                        msg.hvess_average_cell_temperature = *value;
                    }
                    "hvss_cll_tmprtr_dffrntl_stts"
                    | "temperaturestatus"
                    | "tempstatus"
                    | "differentialstatus"
                    | "celltempstatus" => {
                        msg.hvss_cll_tmprtr_dffrntl_stts = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for HVESSD3 message",
                            field_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "HVESSFS1" => {
            let mut msg = HVESSFS1 {
                device_id,
                hvess_fan_speed_status: 0,
                hvess_fan_status_reason_code: 0,
                hvess_fan_command_status: 0,
                hvess_fan_speed: 0.0,
                hvess_fan_medium_temperature: 0.0,
                hvess_fan_power: 0.0,
                hvess_fan_service_indicator: 0,
                hvess_fan_operating_status: 0,
                hvess_fan_status_1_instance: 0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "hvess_fan_speed" | "fan_speed" | "speed" | "rpm" | "fanrpm" => {
                        msg.hvess_fan_speed = *value;
                    }
                    "hvess_fan_power" | "fanpower" | "power" | "consumption"
                    | "powerconsumption" => {
                        msg.hvess_fan_power = *value;
                    }
                    "hvess_fan_medium_temperature"
                    | "temperature"
                    | "temp"
                    | "mediumtemperature"
                    | "mediumtemp" => {
                        msg.hvess_fan_medium_temperature = *value;
                    }
                    "hvess_fan_speed_status" | "speedstatus" | "fanspeedstatus" | "speedstate" => {
                        msg.hvess_fan_speed_status = *value as u8;
                    }
                    "hvess_fan_status_reason_code" | "reasoncode" | "statusreason" | "reason" => {
                        msg.hvess_fan_status_reason_code = *value as u8;
                    }
                    "hvess_fan_command_status" | "command_status" | "command" | "cmdstatus" => {
                        msg.hvess_fan_command_status = *value as u8;
                    }
                    "hvess_fan_service_indicator"
                    | "service_indicator"
                    | "service"
                    | "maintenance" => {
                        msg.hvess_fan_service_indicator = *value as u8;
                    }
                    "hvess_fan_operating_status" | "operatingstatus" | "operating" | "opstatus" => {
                        msg.hvess_fan_operating_status = *value as u8;
                    }
                    "hvess_fan_status_1_instance"
                    | "instance"
                    | "faninstance"
                    | "statusinstance" => {
                        msg.hvess_fan_status_1_instance = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for HVESSFS1 message",
                            field_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "HVESSD6" => {
            let mut msg = HVESSD6 {
                device_id,
                hvess_bus_voltage: 0.0,
                hvess_ignition_voltage: 0.0,
                hvess_intake_coolant_temperature: 0.0,
                hvess_outlet_coolant_temperature: 0.0,
                hvess_electronics_temperature: 0.0,
                hvess_temperature: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "hvess_bus_voltage" => {
                        msg.hvess_bus_voltage = *value;
                    }
                    "hvess_ignition_voltage" => {
                        msg.hvess_ignition_voltage = *value;
                    }
                    "hvess_intake_coolant_temperature" => {
                        msg.hvess_intake_coolant_temperature = *value;
                    }
                    "hvess_outlet_coolant_temperature" => {
                        msg.hvess_outlet_coolant_temperature = *value;
                    }
                    "hvess_electronics_temperature" => {
                        msg.hvess_electronics_temperature = *value;
                    }
                    "hvess_temperature" => {
                        msg.hvess_temperature = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "MG1IC" => {
            let mut msg = MG1IC {
                device_id,
                mtr_gnrtr_1_invrtr_cntrl_cntr: 0,
                mt_gt_1_ivt_ct_lts_rqst_ovd_md: 0,
                mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst: 0,
                mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst: 0.0,
                mtr_gnrtr_1_invrtr_cntrl_cr: 0,
                mt_gt_1_ivt_ct_lts_rqst_ovd_mx: 0.0,
                mt_gt_1_ivt_ct_lts_rqst_ovd_m: 0.0,
                mg_1_rotor_position_sensing_request: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "mtr_gnrtr_1_invrtr_cntrl_cntr" => {
                        msg.mtr_gnrtr_1_invrtr_cntrl_cntr = *value as u8;
                    }
                    "mt_gt_1_ivt_ct_lts_rqst_ovd_md" => {
                        msg.mt_gt_1_ivt_ct_lts_rqst_ovd_md = *value as u8;
                    }
                    "mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst" => {
                        msg.mtr_gnrtr_1_invrtr_cntrl_stpnt_md_rqst = *value as u8;
                    }
                    "mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst" => {
                        msg.mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst = *value;
                    }
                    "mtr_gnrtr_1_invrtr_cntrl_cr" => {
                        msg.mtr_gnrtr_1_invrtr_cntrl_cr = *value as u8;
                    }
                    "mt_gt_1_ivt_ct_lts_rqst_ovd_mx" => {
                        msg.mt_gt_1_ivt_ct_lts_rqst_ovd_mx = *value;
                    }
                    "mt_gt_1_ivt_ct_lts_rqst_ovd_m" => {
                        msg.mt_gt_1_ivt_ct_lts_rqst_ovd_m = *value;
                    }
                    "mg_1_rotor_position_sensing_request" => {
                        msg.mg_1_rotor_position_sensing_request = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DCDC1C" => {
            let mut msg = DCDC1C {
                device_id,
                dc_dc_1_low_side_voltage_buck_setpoint: 0.0,
                dc_dc_1_operational_command: 0,
                dc_dc_1_control_counter: 0,
                dc_dc_1_control_crc: 0,
                dd_1_hgh_sd_vltg_bst_stpnt: 0.0,
                dd_1_lw_sd_vltg_bk_dflt_stpnt: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "dc_dc_1_low_side_voltage_buck_setpoint" => {
                        msg.dc_dc_1_low_side_voltage_buck_setpoint = *value;
                    }
                    "dc_dc_1_operational_command" => {
                        msg.dc_dc_1_operational_command = *value as u8;
                    }
                    "dc_dc_1_control_counter" => {
                        msg.dc_dc_1_control_counter = *value as u8;
                    }
                    "dc_dc_1_control_crc" => {
                        msg.dc_dc_1_control_crc = *value as u8;
                    }
                    "dd_1_hgh_sd_vltg_bst_stpnt" => {
                        msg.dd_1_hgh_sd_vltg_bst_stpnt = *value;
                    }
                    "dd_1_lw_sd_vltg_bk_dflt_stpnt" => {
                        msg.dd_1_lw_sd_vltg_bk_dflt_stpnt = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "MG2IC" => {
            let mut msg = MG2IC {
                device_id,
                mtr_gnrtr_2_invrtr_cntrl_cntr: 0,
                mt_gt_2_ivt_ct_lts_rqst_ovd_md: 0,
                mtr_gnrtr_2_invrtr_cntrl_stpnt_md_rqst: 0,
                mtr_gnrtr_2_invrtr_cntrl_stpnt_rqst: 0.0,
                mtr_gnrtr_2_invrtr_cntrl_cr: 0,
                mt_gt_2_ivt_ct_lts_rqst_ovd_mx: 0.0,
                mt_gt_2_ivt_ct_lts_rqst_ovd_m: 0.0,
                mg_2_rotor_position_sensing_request: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "mtr_gnrtr_2_invrtr_cntrl_cntr" => {
                        msg.mtr_gnrtr_2_invrtr_cntrl_cntr = *value as u8;
                    }
                    "mt_gt_2_ivt_ct_lts_rqst_ovd_md" => {
                        msg.mt_gt_2_ivt_ct_lts_rqst_ovd_md = *value as u8;
                    }
                    "mtr_gnrtr_2_invrtr_cntrl_stpnt_md_rqst" => {
                        msg.mtr_gnrtr_2_invrtr_cntrl_stpnt_md_rqst = *value as u8;
                    }
                    "mtr_gnrtr_2_invrtr_cntrl_stpnt_rqst" => {
                        msg.mtr_gnrtr_2_invrtr_cntrl_stpnt_rqst = *value;
                    }
                    "mtr_gnrtr_2_invrtr_cntrl_cr" => {
                        msg.mtr_gnrtr_2_invrtr_cntrl_cr = *value as u8;
                    }
                    "mt_gt_2_ivt_ct_lts_rqst_ovd_mx" => {
                        msg.mt_gt_2_ivt_ct_lts_rqst_ovd_mx = *value;
                    }
                    "mt_gt_2_ivt_ct_lts_rqst_ovd_m" => {
                        msg.mt_gt_2_ivt_ct_lts_rqst_ovd_m = *value;
                    }
                    "mg_2_rotor_position_sensing_request" => {
                        msg.mg_2_rotor_position_sensing_request = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "MG1IS1" => {
            let mut msg = MG1IS1 {
                device_id,
                mtr_gnrtr_1_invrtr_stts_1_cntr: 0,
                motor_generator_1_speed: 0.0,
                mtr_gnrtr_1_invrtr_stts_1_cr: 0,
                mtr_gnrtr_1_invrtr_d_sd_crrnt: 0.0,
                mtr_gnrtr_1_invrtr_d_sd_vltg: 0.0,
                motor_generator_1_net_rotor_torque: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "mtr_gnrtr_1_invrtr_stts_1_cntr" => {
                        msg.mtr_gnrtr_1_invrtr_stts_1_cntr = *value as u8;
                    }
                    "motor_generator_1_speed" => {
                        msg.motor_generator_1_speed = *value;
                    }
                    "mtr_gnrtr_1_invrtr_stts_1_cr" => {
                        msg.mtr_gnrtr_1_invrtr_stts_1_cr = *value as u8;
                    }
                    "mtr_gnrtr_1_invrtr_d_sd_crrnt" => {
                        msg.mtr_gnrtr_1_invrtr_d_sd_crrnt = *value;
                    }
                    "mtr_gnrtr_1_invrtr_d_sd_vltg" => {
                        msg.mtr_gnrtr_1_invrtr_d_sd_vltg = *value;
                    }
                    "motor_generator_1_net_rotor_torque" => {
                        msg.motor_generator_1_net_rotor_torque = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "MG2IS1" => {
            let mut msg = MG2IS1 {
                device_id,
                mtr_gnrtr_2_invrtr_stts_1_cntr: 0,
                motor_generator_2_speed: 0.0,
                mtr_gnrtr_2_invrtr_stts_1_cr: 0,
                mtr_gnrtr_2_invrtr_d_sd_crrnt: 0.0,
                mtr_gnrtr_2_invrtr_d_sd_vltg: 0.0,
                motor_generator_2_net_rotor_torque: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "mtr_gnrtr_2_invrtr_stts_1_cntr" => {
                        msg.mtr_gnrtr_2_invrtr_stts_1_cntr = *value as u8;
                    }
                    "motor_generator_2_speed" => {
                        msg.motor_generator_2_speed = *value;
                    }
                    "mtr_gnrtr_2_invrtr_stts_1_cr" => {
                        msg.mtr_gnrtr_2_invrtr_stts_1_cr = *value as u8;
                    }
                    "mtr_gnrtr_2_invrtr_d_sd_crrnt" => {
                        msg.mtr_gnrtr_2_invrtr_d_sd_crrnt = *value;
                    }
                    "mtr_gnrtr_2_invrtr_d_sd_vltg" => {
                        msg.mtr_gnrtr_2_invrtr_d_sd_vltg = *value;
                    }
                    "motor_generator_2_net_rotor_torque" => {
                        msg.motor_generator_2_net_rotor_torque = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "MG1IS2" => {
            let mut msg = MG1IS2 {
                device_id,
                mtr_gnrtr_1_invrtr_stts_2_cntr: 0,
                mtr_gnrtr_1_avll_mxmm_trq: 0.0,
                mtr_gnrtr_1_avll_mnmm_trq: 0.0,
                mtr_gnrtr_1_invrtr_stts_2_cr: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "mtr_gnrtr_1_invrtr_stts_2_cntr" => {
                        msg.mtr_gnrtr_1_invrtr_stts_2_cntr = *value as u8;
                    }
                    "mtr_gnrtr_1_avll_mxmm_trq" => {
                        msg.mtr_gnrtr_1_avll_mxmm_trq = *value;
                    }
                    "mtr_gnrtr_1_avll_mnmm_trq" => {
                        msg.mtr_gnrtr_1_avll_mnmm_trq = *value;
                    }
                    "mtr_gnrtr_1_invrtr_stts_2_cr" => {
                        msg.mtr_gnrtr_1_invrtr_stts_2_cr = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC19" => {
            let mut msg = EEC19 {
                device_id,
                total_engine_energy: 0,
                engn_exhst_flw_rt_extndd_rng: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "total_engine_energy" => {
                        msg.total_engine_energy = *value as u32;
                    }
                    "engn_exhst_flw_rt_extndd_rng" => {
                        msg.engn_exhst_flw_rt_extndd_rng = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "ETC4" => {
            let mut msg = ETC4 {
                device_id,
                trnsmssn_snhrnzr_clth_vl: 0.0,
                trnsmssn_snhrnzr_brk_vl: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "trnsmssn_snhrnzr_clth_vl" => {
                        msg.trnsmssn_snhrnzr_clth_vl = *value;
                    }
                    "trnsmssn_snhrnzr_brk_vl" => {
                        msg.trnsmssn_snhrnzr_brk_vl = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DPFC2" => {
            let mut msg = DPFC2 {
                device_id,
                atttt_1_ds_ptt_ft_it_tpt_st_pt: 0.0,
                engine_unburned_fuel_percentage: 0.0,
                aftertreatment_1_fuel_mass_rate: 0.0,
                aftertreatment_2_fuel_mass_rate: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "atttt_1_ds_ptt_ft_it_tpt_st_pt" => {
                        msg.atttt_1_ds_ptt_ft_it_tpt_st_pt = *value;
                    }
                    "engine_unburned_fuel_percentage" => {
                        msg.engine_unburned_fuel_percentage = *value;
                    }
                    "aftertreatment_1_fuel_mass_rate" => {
                        msg.aftertreatment_1_fuel_mass_rate = *value;
                    }
                    "aftertreatment_2_fuel_mass_rate" => {
                        msg.aftertreatment_2_fuel_mass_rate = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC7" => {
            let mut msg = EEC7 {
                device_id,
                engn_exhst_gs_rrltn_1_vlv_pstn: 0.0,
                engn_exhst_gs_rrltn_1_vlv_2_pstn: 0.0,
                engn_crnks_brthr_ol_sprtr_spd: 0,
                engn_intk_mnfld_cmmndd_prssr: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_exhst_gs_rrltn_1_vlv_pstn" => {
                        msg.engn_exhst_gs_rrltn_1_vlv_pstn = *value;
                    }
                    "engn_exhst_gs_rrltn_1_vlv_2_pstn" => {
                        msg.engn_exhst_gs_rrltn_1_vlv_2_pstn = *value;
                    }
                    "engn_crnks_brthr_ol_sprtr_spd" => {
                        msg.engn_crnks_brthr_ol_sprtr_spd = *value as u16;
                    }
                    "engn_intk_mnfld_cmmndd_prssr" => {
                        msg.engn_intk_mnfld_cmmndd_prssr = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC11" => {
            let mut msg = EEC11 {
                device_id,
                engn_exhst_gs_rrltn_2_vlv_1_cntrl: 0.0,
                engn_exhst_gs_rrltn_2_vlv_2_cntrl: 0.0,
                engn_exhst_gs_rrltn_2_vlv_1_pstn_errr: 0.0,
                engn_exhst_gs_rrltn_2_vlv_2_pstn_errr: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_exhst_gs_rrltn_2_vlv_1_cntrl" => {
                        msg.engn_exhst_gs_rrltn_2_vlv_1_cntrl = *value;
                    }
                    "engn_exhst_gs_rrltn_2_vlv_2_cntrl" => {
                        msg.engn_exhst_gs_rrltn_2_vlv_2_cntrl = *value;
                    }
                    "engn_exhst_gs_rrltn_2_vlv_1_pstn_errr" => {
                        msg.engn_exhst_gs_rrltn_2_vlv_1_pstn_errr = *value;
                    }
                    "engn_exhst_gs_rrltn_2_vlv_2_pstn_errr" => {
                        msg.engn_exhst_gs_rrltn_2_vlv_2_pstn_errr = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC20" => {
            let mut msg = EEC20 {
                device_id,
                esttd_e_pst_lsss_pt_tq_h_rst: 0.0,
                atl_mxmm_avll_engn_prnt_fl: 0.0,
                nmnl_frtn_prnt_trq_hgh_rsltn: 0.0,
                aslt_engn_ld_prnt_ar_mss: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "esttd_e_pst_lsss_pt_tq_h_rst" => {
                        msg.esttd_e_pst_lsss_pt_tq_h_rst = *value;
                    }
                    "atl_mxmm_avll_engn_prnt_fl" => {
                        msg.atl_mxmm_avll_engn_prnt_fl = *value;
                    }
                    "nmnl_frtn_prnt_trq_hgh_rsltn" => {
                        msg.nmnl_frtn_prnt_trq_hgh_rsltn = *value;
                    }
                    "aslt_engn_ld_prnt_ar_mss" => {
                        msg.aslt_engn_ld_prnt_ar_mss = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "ETC9" => {
            let mut msg = ETC9 {
                device_id,
                dl_clth_trnsmssn_crrnt_pr_sltn_gr: 0.0,
                dl_clth_trnsmssn_inpt_shft_1_spd: 0.0,
                dl_clth_trnsmssn_inpt_shft_2_spd: 0.0,
                dl_clth_trnsmssn_sltd_pr_sltn_gr: 0.0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "dl_clth_trnsmssn_crrnt_pr_sltn_gr" => {
                        msg.dl_clth_trnsmssn_crrnt_pr_sltn_gr = *value;
                    }
                    "dl_clth_trnsmssn_inpt_shft_1_spd" => {
                        msg.dl_clth_trnsmssn_inpt_shft_1_spd = *value;
                    }
                    "dl_clth_trnsmssn_inpt_shft_2_spd" => {
                        msg.dl_clth_trnsmssn_inpt_shft_2_spd = *value;
                    }
                    "dl_clth_trnsmssn_sltd_pr_sltn_gr" => {
                        msg.dl_clth_trnsmssn_sltd_pr_sltn_gr = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "HVESSTS1" => {
            let mut msg = HVESSTS1 {
                device_id,
                hvss_thrml_mngmnt_sstm_sl_inpt_pwr: 0.0,
                hvss_t_mt_sst_h_vt_ipt_pw: 0.0,
                hvss_thrml_mngmnt_sstm_cmprssr_spd: 0.0,
                hvss_thrml_mngmnt_sstm_rltv_hmdt: 0.0,
                hvss_thrml_mngmnt_sstm_htr_stts: 0,
                hvss_thrml_mngmnt_sstm_hvl_stts: 0,
                hvss_thrml_mngmnt_sstm_md: 0,
                hvss_thrml_mngmnt_sstm_clnt_lvl: 0,
                hvss_thrml_mngmnt_sstm_clnt_lvl_fll: 0,
            };

            // Populate fields from field_map
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "hvss_thrml_mngmnt_sstm_sl_inpt_pwr" | "systeminputpower" => {
                        msg.hvss_thrml_mngmnt_sstm_sl_inpt_pwr = *value;
                    }
                    "hvss_t_mt_sst_h_vt_ipt_pw" | "hvinputpower" => {
                        msg.hvss_t_mt_sst_h_vt_ipt_pw = *value;
                    }
                    "hvss_thrml_mngmnt_sstm_cmprssr_spd" | "compressorspeed" => {
                        msg.hvss_thrml_mngmnt_sstm_cmprssr_spd = *value;
                    }
                    "hvss_thrml_mngmnt_sstm_rltv_hmdt" | "relative_humidity" => {
                        msg.hvss_thrml_mngmnt_sstm_rltv_hmdt = *value;
                    }
                    "hvss_thrml_mngmnt_sstm_htr_stts" | "heaterstatus" => {
                        msg.hvss_thrml_mngmnt_sstm_htr_stts = *value as u8;
                    }
                    "hvss_thrml_mngmnt_sstm_hvl_stts" | "hvilstatus" => {
                        msg.hvss_thrml_mngmnt_sstm_hvl_stts = *value as u8;
                    }
                    "hvss_thrml_mngmnt_sstm_md" | "systemmode" => {
                        msg.hvss_thrml_mngmnt_sstm_md = *value as u8;
                    }
                    "hvss_thrml_mngmnt_sstm_clnt_lvl" | "coolantlevel" => {
                        msg.hvss_thrml_mngmnt_sstm_clnt_lvl = *value as u8;
                    }
                    "hvss_thrml_mngmnt_sstm_clnt_lvl_fll" | "coolantlevelfull" => {
                        msg.hvss_thrml_mngmnt_sstm_clnt_lvl_fll = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for message '{}'",
                            field_name,
                            message_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "HVESSTC1" => {
            let mut msg = HVESSTC1 {
                device_id,
                hvss_t_mt_sst_it_ct_tpt_rqst: 0.0,
                hvss_t_mt_sst_ott_ct_tpt_rqst: 0.0,
                hvss_t_mt_sst_ct_fw_rt_rqst: 0.0,
                hvss_thrml_mngmnt_sstm_htr_enl_cmmnd: 0,
                hvss_t_mt_sst_ct_pp_e_cd: 0,
                hvss_t_mt_sst_cpss_e_cd: 0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "hvss_t_mt_sst_it_ct_tpt_rqst"
                    | "intaketemperature"
                    | "inlettemperature"
                    | "intakecoolanttemp" => {
                        msg.hvss_t_mt_sst_it_ct_tpt_rqst = *value;
                    }
                    "hvss_t_mt_sst_ott_ct_tpt_rqst"
                    | "outlettemperature"
                    | "exittemperature"
                    | "outletcoolanttemp" => {
                        msg.hvss_t_mt_sst_ott_ct_tpt_rqst = *value;
                    }
                    "hvss_t_mt_sst_ct_fw_rt_rqst"
                    | "flowrate"
                    | "coolantflowrate"
                    | "flowraterequest" => {
                        msg.hvss_t_mt_sst_ct_fw_rt_rqst = *value;
                    }
                    "hvss_thrml_mngmnt_sstm_htr_enl_cmmnd"
                    | "heaterenable"
                    | "heatercommand"
                    | "heatercontrol" => {
                        msg.hvss_thrml_mngmnt_sstm_htr_enl_cmmnd = *value as u8;
                    }
                    "hvss_t_mt_sst_ct_pp_e_cd"
                    | "pumpenable"
                    | "coolantpumpenable"
                    | "pumpcontrol" => {
                        msg.hvss_t_mt_sst_ct_pp_e_cd = *value as u8;
                    }
                    "hvss_t_mt_sst_cpss_e_cd"
                    | "compressorenable"
                    | "compressorcommand"
                    | "compressorcontrol" => {
                        msg.hvss_t_mt_sst_cpss_e_cd = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for HVESSTC1 message",
                            field_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "HVESSTC2" => {
            let mut msg = HVESSTC2 {
                device_id,
                hvss_t_mt_sst_ct_pp_spd_cd: 0.0,
                hvss_t_mt_sst_ct_pp_pt_spd_cd: 0.0,
                hvss_t_mt_sst_cpss_spd_cd: 0.0,
                hvss_t_mt_sst_cpss_pt_spd_cd: 0.0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "hvss_t_mt_sst_ct_pp_spd_cd" | "pumpspeed" | "pumpspeedcommand" | "pumprpm" => {
                        msg.hvss_t_mt_sst_ct_pp_spd_cd = *value;
                    }
                    "hvss_t_mt_sst_ct_pp_pt_spd_cd"
                    | "pumpspeedpercent"
                    | "pumppercentage"
                    | "pumppercent" => {
                        msg.hvss_t_mt_sst_ct_pp_pt_spd_cd = *value;
                    }
                    "hvss_t_mt_sst_cpss_spd_cd"
                    | "compressorspeed"
                    | "compressorspeedcommand"
                    | "compressorrpm" => {
                        msg.hvss_t_mt_sst_cpss_spd_cd = *value;
                    }
                    "hvss_t_mt_sst_cpss_pt_spd_cd"
                    | "compressorspeedpercent"
                    | "compressorpercentage"
                    | "compressorpercent" => {
                        msg.hvss_t_mt_sst_cpss_pt_spd_cd = *value;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for HVESSTC2 message",
                            field_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "ETCC3" => {
            let mut msg = ETCC3 {
                device_id,
                et_cpss_bw_att_1_mt_ct_ds: 0,
                engn_trhrgr_wstgt_attr_1_mtr_crrnt_dsl: 0,
                engn_clndr_hd_bpss_attr_1_mtr_crrnt_dsl: 0,
                engn_thrttl_vlv_1_mtr_crrnt_dsl: 0,
                et_cpss_bpss_att_1_mt_ct_ds: 0,
                et_cpss_bpss_att_2_mt_ct_ds: 0,
                engn_trhrgr_wstgt_attr_2_mtr_crrnt_dsl: 0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "et_cpss_bw_att_1_mt_ct_ds" | "bypass1" | "etcbypass" | "bypass" => {
                        msg.et_cpss_bw_att_1_mt_ct_ds = *value as u8;
                    }
                    "engn_trhrgr_wstgt_attr_1_mtr_crrnt_dsl"
                    | "wastegate1"
                    | "turbo1"
                    | "wastegate" => {
                        msg.engn_trhrgr_wstgt_attr_1_mtr_crrnt_dsl = *value as u8;
                    }
                    "engn_clndr_hd_bpss_attr_1_mtr_crrnt_dsl"
                    | "cylinderhead"
                    | "head"
                    | "headbypass" => {
                        msg.engn_clndr_hd_bpss_attr_1_mtr_crrnt_dsl = *value as u8;
                    }
                    "engn_thrttl_vlv_1_mtr_crrnt_dsl" | "throttle" | "throttlevalve" | "valve" => {
                        msg.engn_thrttl_vlv_1_mtr_crrnt_dsl = *value as u8;
                    }
                    "et_cpss_bpss_att_1_mt_ct_ds" | "etcbypass1" | "etcpass1" | "pass1" => {
                        msg.et_cpss_bpss_att_1_mt_ct_ds = *value as u8;
                    }
                    "et_cpss_bpss_att_2_mt_ct_ds" | "etcbypass2" | "etcpass2" | "pass2" => {
                        msg.et_cpss_bpss_att_2_mt_ct_ds = *value as u8;
                    }
                    "engn_trhrgr_wstgt_attr_2_mtr_crrnt_dsl" | "wastegate2" | "turbo2" => {
                        msg.engn_trhrgr_wstgt_attr_2_mtr_crrnt_dsl = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!("Unknown field '{}' for ETCC3 message", field_name));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "DM01" => {
            let mut msg = j1939::DM01 {
                device_id,
                // Initialize lamp status to off
                protect_lamp_status: 0,
                amber_warning_lamp_status: 0,
                red_stop_lamp_status: 0,
                malfunction_indicator_lamp_status: 0,
                flash_protect_lamp: 0,
                flash_amber_warning_lamp: 0,
                flash_red_stop_lamp: 0,
                flash_malfunc_indicator_lamp: 0,
                // Initialize first DTC to not available
                dm01_01spn: 0xFFFF,
                dm01_01spn_high: 0.0,
                dm01_01fmi: 0xFF,
                dm01_01oc: 0xFF,
                dm01_01cm: 0xFF,
                // Initialize all remaining DTCs to not available
                dm01_02spn: 0xFFFF,
                dm01_02spn_high: 0.0,
                dm01_02fmi: 0xFF,
                dm01_02oc: 0xFF,
                dm01_02cm: 0xFF,
                dm01_03spn: 0xFFFF,
                dm01_03spn_high: 0.0,
                dm01_03fmi: 0xFF,
                dm01_03oc: 0xFF,
                dm01_03cm: 0xFF,
                dm01_04spn: 0xFFFF,
                dm01_04spn_high: 0.0,
                dm01_04fmi: 0xFF,
                dm01_04oc: 0xFF,
                dm01_04cm: 0xFF,
                dm01_05spn: 0xFFFF,
                dm01_05spn_high: 0.0,
                dm01_05fmi: 0xFF,
                dm01_05oc: 0xFF,
                dm01_05cm: 0xFF,
                dm01_06spn: 0xFFFF,
                dm01_06spn_high: 0.0,
                dm01_06fmi: 0xFF,
                dm01_06oc: 0xFF,
                dm01_06cm: 0xFF,
                dm01_07spn: 0xFFFF,
                dm01_07spn_high: 0.0,
                dm01_07fmi: 0xFF,
                dm01_07oc: 0xFF,
                dm01_07cm: 0xFF,
                dm01_08spn: 0xFFFF,
                dm01_08spn_high: 0.0,
                dm01_08fmi: 0xFF,
                dm01_08oc: 0xFF,
                dm01_08cm: 0xFF,
                dm01_09spn: 0xFFFF,
                dm01_09spn_high: 0.0,
                dm01_09fmi: 0xFF,
                dm01_09oc: 0xFF,
                dm01_09cm: 0xFF,
                dm01_10spn: 0xFFFF,
                dm01_10spn_high: 0.0,
                dm01_10fmi: 0xFF,
                dm01_10oc: 0xFF,
                dm01_10cm: 0xFF,
                dm01_11spn: 0xFFFF,
                dm01_11spn_high: 0.0,
                dm01_11fmi: 0xFF,
                dm01_11oc: 0xFF,
                dm01_11cm: 0xFF,
                dm01_12spn: 0xFFFF,
                dm01_12spn_high: 0.0,
                dm01_12fmi: 0xFF,
                dm01_12oc: 0xFF,
                dm01_12cm: 0xFF,
                dm01_13spn: 0xFFFF,
                dm01_13spn_high: 0.0,
                dm01_13fmi: 0xFF,
                dm01_13oc: 0xFF,
                dm01_13cm: 0xFF,
                dm01_14spn: 0xFFFF,
                dm01_14spn_high: 0.0,
                dm01_14fmi: 0xFF,
                dm01_14oc: 0xFF,
                dm01_14cm: 0xFF,
                dm01_15spn: 0xFFFF,
                dm01_15spn_high: 0.0,
                dm01_15fmi: 0xFF,
                dm01_15oc: 0xFF,
                dm01_15cm: 0xFF,
                dm01_16spn: 0xFFFF,
                dm01_16spn_high: 0.0,
                dm01_16fmi: 0xFF,
                dm01_16oc: 0xFF,
                dm01_16cm: 0xFF,
                dm01_17spn: 0xFFFF,
                dm01_17spn_high: 0.0,
                dm01_17fmi: 0xFF,
                dm01_17oc: 0xFF,
                dm01_17cm: 0xFF,
                dm01_18spn: 0xFFFF,
                dm01_18spn_high: 0.0,
                dm01_18fmi: 0xFF,
                dm01_18oc: 0xFF,
                dm01_18cm: 0xFF,
                dm01_19spn: 0xFFFF,
                dm01_19spn_high: 0.0,
                dm01_19fmi: 0xFF,
                dm01_19oc: 0xFF,
                dm01_19cm: 0xFF,
            };

            // Populate fields from field_map with diagnostic-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    // Lamp status fields
                    "protect_lamp_status" | "protect_lamp" => {
                        msg.protect_lamp_status = *value as u8;
                    }
                    "amber_warning_lamp_status" | "amber_warning_lamp" | "amber_lamp" => {
                        msg.amber_warning_lamp_status = *value as u8;
                    }
                    "red_stop_lamp_status" | "red_stop_lamp" | "stop_lamp" => {
                        msg.red_stop_lamp_status = *value as u8;
                    }
                    "malfunction_indicator_lamp_status" | "mil" | "malfunction_lamp" => {
                        msg.malfunction_indicator_lamp_status = *value as u8;
                    }
                    // Flash lamp controls
                    "flash_protect_lamp" | "flash_protect" => {
                        msg.flash_protect_lamp = *value as u8;
                    }
                    "flash_amber_warning_lamp" | "flash_amber" => {
                        msg.flash_amber_warning_lamp = *value as u8;
                    }
                    "flash_red_stop_lamp" | "flash_red_stop" => {
                        msg.flash_red_stop_lamp = *value as u8;
                    }
                    "flash_malfunc_indicator_lamp" | "flash_mil" => {
                        msg.flash_malfunc_indicator_lamp = *value as u8;
                    }
                    // First active DTC fields (most common)
                    "dm01_01spn" | "spn" | "suspect_parameter" => {
                        msg.dm01_01spn = *value as u16;
                    }
                    "dm01_01spn_high" | "spn_high" => {
                        msg.dm01_01spn_high = *value;
                    }
                    "dm01_01fmi" | "fmi" | "failure_mode" => {
                        msg.dm01_01fmi = *value as u8;
                    }
                    "dm01_01oc" | "oc" | "occurrence_count" => {
                        msg.dm01_01oc = *value as u8;
                    }
                    "dm01_01cm" | "cm" | "conversion_method" => {
                        msg.dm01_01cm = *value as u8;
                    }
                    _ => {
                        return Err(anyhow!(
                            "Unknown field '{}' for DM01 message. Supported: protect_lamp, amber_lamp, red_stop_lamp, mil, spn, fmi, oc",
                            field_name
                        ));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939-73".to_string(),
            }))
        }

        "ETC2" => {
            let mut msg = ETC2 {
                device_id,
                transmission_selected_gear: 0.0,
                transmission_actual_gear_ratio: 0.0,
                transmission_current_gear: 0.0,
                transmission_requested_range: 0,
                transmission_current_range: 0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "transmission_selected_gear" | "selectedgear" | "target" => {
                        msg.transmission_selected_gear = *value;
                    }
                    "transmission_actual_gear_ratio" | "gearratio" | "ratio" => {
                        msg.transmission_actual_gear_ratio = *value;
                    }
                    "transmission_current_gear" | "currentgear" | "gear" => {
                        msg.transmission_current_gear = *value;
                    }
                    "transmission_requested_range" | "requestedrange" | "reqrange" => {
                        msg.transmission_requested_range = *value as u16;
                    }
                    "transmission_current_range" | "currentrange" | "range" => {
                        msg.transmission_current_range = *value as u16;
                    }
                    _ => {
                        return Err(anyhow!("Unknown field '{}' for ETC2 message", field_name));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC8" => {
            let mut msg = EEC8 {
                device_id,
                engn_exhst_gs_rrltn_1_vlv_2_cntrl: 0.0,
                engn_exhst_gs_rrltn_1_clr_intk_tmprtr: 0.0,
                e_exst_gs_rt_1_c_it_ast_pss: 0.0,
                engn_exhst_gs_rrltn_1_clr_effn: 0.0,
                e_exst_gs_rt_at_it_ct_tpt: 0.0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_exhst_gs_rrltn_1_vlv_2_cntrl" | "egrvalve" | "valve" => {
                        msg.engn_exhst_gs_rrltn_1_vlv_2_cntrl = *value;
                    }
                    "engn_exhst_gs_rrltn_1_clr_intk_tmprtr" | "coolertemp" | "temp" => {
                        msg.engn_exhst_gs_rrltn_1_clr_intk_tmprtr = *value;
                    }
                    "e_exst_gs_rt_1_c_it_ast_pss" | "pressure" | "coolerpressure" => {
                        msg.e_exst_gs_rt_1_c_it_ast_pss = *value;
                    }
                    "engn_exhst_gs_rrltn_1_clr_effn" | "efficiency" | "coolerefficiency" => {
                        msg.engn_exhst_gs_rrltn_1_clr_effn = *value;
                    }
                    "e_exst_gs_rt_at_it_ct_tpt" | "targettemp" | "target" => {
                        msg.e_exst_gs_rt_at_it_ct_tpt = *value;
                    }
                    _ => {
                        return Err(anyhow!("Unknown field '{}' for EEC8 message", field_name));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC15" => {
            let mut msg = EEC15 {
                device_id,
                accelerator_pedal_1_channel_2: 0.0,
                accelerator_pedal_1_channel_3: 0.0,
                accelerator_pedal_2_channel_2: 0.0,
                accelerator_pedal_2_channel_3: 0.0,
                engn_exhst_gs_rstrtn_vlv_cntrl: 0.0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "accelerator_pedal_1_channel_2" | "pedal1ch2" | "pedal1" => {
                        msg.accelerator_pedal_1_channel_2 = *value;
                    }
                    "accelerator_pedal_1_channel_3" | "pedal1ch3" => {
                        msg.accelerator_pedal_1_channel_3 = *value;
                    }
                    "accelerator_pedal_2_channel_2" | "pedal2ch2" | "pedal2" => {
                        msg.accelerator_pedal_2_channel_2 = *value;
                    }
                    "accelerator_pedal_2_channel_3" | "pedal2ch3" => {
                        msg.accelerator_pedal_2_channel_3 = *value;
                    }
                    "engn_exhst_gs_rstrtn_vlv_cntrl" | "restrictionvalve" | "valve" => {
                        msg.engn_exhst_gs_rstrtn_vlv_cntrl = *value;
                    }
                    _ => {
                        return Err(anyhow!("Unknown field '{}' for EEC15 message", field_name));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }
        "ETCC1" => {
            let mut msg = ETCC1 {
                device_id,
                engn_trhrgr_wstgt_attr_1_cmmnd: 0.0,
                engn_trhrgr_wstgt_attr_2_cmmnd: 0.0,
                e_exst_b_1_pss_rt_ct_cd: 0.0,
                et_cpss_bw_att_1_cd: 0.0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "engn_trhrgr_wstgt_attr_1_cmmnd" | "wastegate1" | "turbo1" => {
                        msg.engn_trhrgr_wstgt_attr_1_cmmnd = *value;
                    }
                    "engn_trhrgr_wstgt_attr_2_cmmnd" | "wastegate2" | "turbo2" => {
                        msg.engn_trhrgr_wstgt_attr_2_cmmnd = *value;
                    }
                    "e_exst_b_1_pss_rt_ct_cd" | "exhaustpressure" | "pressure" => {
                        msg.e_exst_b_1_pss_rt_ct_cd = *value;
                    }
                    "et_cpss_bw_att_1_cd" | "blowoff" | "compressorblowoff" => {
                        msg.et_cpss_bw_att_1_cd = *value;
                    }
                    _ => {
                        return Err(anyhow!("Unknown field '{}' for ETCC1 message", field_name));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "EEC17" => {
            let mut msg = EEC17 {
                device_id,
                pems_engine_fuel_mass_flow_rate: 0.0,
                vehicle_fuel_rate: 0.0,
                engine_exhaust_flow_rate: 0.0,
                cylinder_fuel_rate: 0.0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "pems_engine_fuel_mass_flow_rate" | "pemsfuel" | "pems" => {
                        msg.pems_engine_fuel_mass_flow_rate = *value;
                    }
                    "vehicle_fuel_rate" | "vehiclefuel" | "fuelrate" => {
                        msg.vehicle_fuel_rate = *value;
                    }
                    "engine_exhaust_flow_rate" | "exhaustflow" | "exhaust" => {
                        msg.engine_exhaust_flow_rate = *value;
                    }
                    "cylinder_fuel_rate" | "cylinderfuel" | "cylinder" => {
                        msg.cylinder_fuel_rate = *value;
                    }
                    _ => {
                        return Err(anyhow!("Unknown field '{}' for EEC17 message", field_name));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }

        "ETC6" => {
            let mut msg = ETC6 {
                device_id,
                recommended_gear: 0.0,
                lowest_possible_gear: 0.0,
                highest_possible_gear: 0.0,
                clutch_life_remaining: 0.0,
            };

            // Populate fields from field_map with user-friendly aliases
            for (field_name, value) in field_map {
                let normalized = normalize_field_name(field_name);
                match normalized.as_str() {
                    "recommended_gear" | "recommended" | "gear" => {
                        msg.recommended_gear = *value;
                    }
                    "lowest_possible_gear" | "lowest" | "mingear" => {
                        msg.lowest_possible_gear = *value;
                    }
                    "highest_possible_gear" | "highest" | "maxgear" => {
                        msg.highest_possible_gear = *value;
                    }
                    "clutch_life_remaining" | "clutchlife" | "clutch" => {
                        msg.clutch_life_remaining = *value;
                    }
                    _ => {
                        return Err(anyhow!("Unknown field '{}' for ETC6 message", field_name));
                    }
                }
            }

            // Encode using the real encoder
            let (can_id, data) = msg.encode()?;

            Ok(Some(EncodedMessage {
                can_id,
                data: data.to_vec(),
                message_name: message_name.to_string(),
                protocol: "J1939".to_string(),
            }))
        }


        _ => Ok(None), // Message not found in J1939 protocol
    }
}

/// Normalize field name for matching
///
/// Converts DBC-style names (e.g., "MCM_MotorSpeedCommand") to struct field names
/// (e.g., "mcm_motorspeedcommand") by converting to lowercase.
fn normalize_field_name(name: &str) -> String {
    name.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_field_name() {
        assert_eq!(
            normalize_field_name("MCM_MotorSpeedCommand"),
            "mcm_motorspeedcommand"
        );
    }

    #[test]
    fn test_encode_unknown_message() {
        let device_id = DeviceId::new(0x42);
        let field_map = HashMap::new();

        let result = encode_message("UnknownMessage", device_id, &field_map);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("not found in any supported protocol")
        );
    }
}
