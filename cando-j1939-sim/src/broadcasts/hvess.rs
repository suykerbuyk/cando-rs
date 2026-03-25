use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_hvess_frames(&self, frames: &mut Vec<CanFrame>, device_id: DeviceId) {
        // HVESSD1 - High Voltage Energy Storage System Data 1 (Power Monitoring)
        let hvessd1 = HVESSD1 {
            device_id,
            hvess_available_discharge_power: self.hvess.hvess_discharge_power,
            hvess_available_charge_power: self.hvess.hvess_charge_power,
            hvess_voltage_level: self.hvess.hvess_voltage_level,
            hvess_current: self.hvess.hvess_current_level,
        };

        if let Ok((can_id, data)) = hvessd1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD6 - High Voltage Energy Storage System Data 6 (Voltage/Temperature Monitoring)
        let hvessd6 = HVESSD6 {
            device_id,
            hvess_bus_voltage: self.hvess.hvess_bus_voltage,
            hvess_ignition_voltage: self.hvess.hvess_ignition_voltage,
            hvess_intake_coolant_temperature: self.hvess.hvess_coolant_temp,
            hvess_outlet_coolant_temperature: self.hvess.hvess_coolant_temp + 5.0, // Slight delta
            hvess_electronics_temperature: self.hvess.hvess_electronics_temp,
            hvess_temperature: self.hvess.hvess_electronics_temp,
        };

        if let Ok((can_id, data)) = hvessd6.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD2 - HVESS Data 2: Cell Voltage & State of Charge Monitoring
        let hvessd2 = HVESSD2 {
            device_id,
            hvess_fast_update_state_of_charge: self.hvess.hvess_fast_update_state_of_charge,
            hvess_highest_cell_voltage: self.hvess.hvess_highest_cell_voltage,
            hvess_lowest_cell_voltage: self.hvess.hvess_lowest_cell_voltage,
            hvss_cll_vltg_dffrntl_stts: (self.hvess.hvess_cell_voltage_differential_status % 256)
                as u8,
        };

        if let Ok((can_id, data)) = hvessd2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD3 - HVESS Data 3: Cell Temperature Monitoring
        let hvessd3 = HVESSD3 {
            device_id,
            hvess_highest_cell_temperature: self.hvess.hvess_highest_cell_temperature,
            hvess_lowest_cell_temperature: self.hvess.hvess_lowest_cell_temperature,
            hvess_average_cell_temperature: self.hvess.hvess_average_cell_temperature,
            hvss_cll_tmprtr_dffrntl_stts: (self.hvess.hvess_cell_temp_differential_status % 256)
                as u8,
        };

        if let Ok((can_id, data)) = hvessd3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSFS1 - HVESS Fan Status 1: Fan Monitoring & Feedback
        let hvessfs1 = HVESSFS1 {
            device_id,
            hvess_fan_speed_status: (self.hvess.hvess_fan_speed_status % 256) as u8,
            hvess_fan_status_reason_code: (self.hvess.hvess_fan_status_reason_code % 256) as u8,
            hvess_fan_command_status: (self.hvess.hvess_fan_command_status % 256) as u8,
            hvess_fan_speed: self.hvess.hvess_fan_speed,
            hvess_fan_medium_temperature: self.hvess.hvess_fan_medium_temperature,
            hvess_fan_power: self.hvess.hvess_fan_power,
            hvess_fan_service_indicator: (self.hvess.hvess_fan_service_indicator % 256) as u8,
            hvess_fan_operating_status: (self.hvess.hvess_fan_operating_status % 256) as u8,
            hvess_fan_status_1_instance: (self.hvess.hvess_fan_status1_instance % 256) as u8,
        };

        if let Ok((can_id, data)) = hvessfs1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // Batch 7: Extended HVESS Broadcasts
        // ============================================================================

        // HVESSD4 - High Voltage Energy Storage System Data 4 (Capacity)
        let hvessd4 = HVESSD4 {
            device_id,
            hvess_discharge_capacity: self.hvess.hvessd4_discharge_capacity,
            hvess_charge_capacity: self.hvess.hvessd4_charge_capacity,
            hvess_cell_balancing_count: self.hvess.hvessd4_cell_balancing_count,
        };
        if let Ok((can_id, data)) = hvessd4.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD5 - High Voltage Energy Storage System Data 5 (Current Limits)
        let hvessd5 = HVESSD5 {
            device_id,
            hvss_mxmm_instntns_dshrg_crrnt_lmt: self.hvess.hvessd5_max_discharge_current_limit,
            hvss_mxmm_instntns_chrg_crrnt_lmt: self.hvess.hvessd5_max_charge_current_limit,
            hvess_minimum_cell_state_of_charge: self.hvess.hvessd5_min_cell_soc,
            hvess_maximum_cell_state_of_charge: self.hvess.hvessd5_max_cell_soc,
        };
        if let Ok((can_id, data)) = hvessd5.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD7 - High Voltage Energy Storage System Data 7 (Energy Capacity)
        let hvessd7 = HVESSD7 {
            device_id,
            hvess_discharge_energy_capacity: self.hvess.hvessd7_discharge_energy_capacity,
            hvess_charge_energy_capacity: self.hvess.hvessd7_charge_energy_capacity,
            hvess_maximum_charge_voltage_limit: self.hvess.hvessd7_max_charge_voltage_limit,
        };
        if let Ok((can_id, data)) = hvessd7.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD8 - High Voltage Energy Storage System Data 8 (Cell Voltage Location)
        let hvessd8 = HVESSD8 {
            device_id,
            hvss_hghst_cll_vltg_mdl_nmr: self.hvess.hvessd8_highest_cell_voltage_module,
            hvss_hghst_cll_vltg_cll_nmr: self.hvess.hvessd8_highest_cell_voltage_cell,
            hvss_lwst_cll_vltg_mdl_nmr: self.hvess.hvessd8_lowest_cell_voltage_module,
            hvss_lwst_cll_vltg_cll_nmr: self.hvess.hvessd8_lowest_cell_voltage_cell,
            hvess_average_cell_voltage: self.hvess.hvessd8_average_cell_voltage,
        };
        if let Ok((can_id, data)) = hvessd8.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD9 - High Voltage Energy Storage System Data 9 (Cell Temp Location)
        let hvessd9 = HVESSD9 {
            device_id,
            hvss_hghst_cll_tmprtr_mdl_nmr: self.hvess.hvessd9_highest_cell_temp_module,
            hvss_hghst_cll_tmprtr_cll_nmr: self.hvess.hvessd9_highest_cell_temp_cell,
            hvss_lwst_cll_tmprtr_mdl_nmr: self.hvess.hvessd9_lowest_cell_temp_module,
            hvss_lwst_cll_tmprtr_cll_nmr: self.hvess.hvessd9_lowest_cell_temp_cell,
            hvess_thermal_event_detected: self.hvess.hvessd9_thermal_event_detected,
            hvss_dt_9_emddd_intgrt_spprt: 0,
            hvess_data_9_counter: self.hvess.hvessd9_counter,
            hvess_data_9_crc: 0,
        };
        if let Ok((can_id, data)) = hvessd9.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD10 - High Voltage Energy Storage System Data 10 (Cell SOC Location)
        let hvessd10 = HVESSD10 {
            device_id,
            hvss_hghst_cll_stt_of_chrg_mdl_nmr: self.hvess.hvessd10_highest_cell_soc_module,
            hvss_hghst_cll_stt_of_chrg_cll_nmr: self.hvess.hvessd10_highest_cell_soc_cell,
            hvss_lwst_cll_stt_of_chrg_mdl_nmr: self.hvess.hvessd10_lowest_cell_soc_module,
            hvss_lwst_cll_stt_of_chrg_cll_nmr: self.hvess.hvessd10_lowest_cell_soc_cell,
            hvss_hgh_vltg_bs_atv_isltn_tst_rslts: self.hvess.hvessd10_active_isolation_test,
            hvss_hgh_vltg_bs_pssv_isltn_tst_rslts: self.hvess.hvessd10_passive_isolation_test,
        };
        if let Ok((can_id, data)) = hvessd10.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD11 - High Voltage Energy Storage System Data 11
        let hvessd11 = HVESSD11 {
            device_id,
            hvss_bs_vltg_ngtv_t_chsss_grnd_vltg: self.hvess.hvessd11_bus_voltage_neg_to_chassis,
            hvss_vltg_lvl_ngtv_t_chsss_grnd_vltg: self.hvess.hvessd11_voltage_neg_to_chassis,
            hvess_actual_charge_rate: self.hvess.hvessd11_actual_charge_rate,
            hvss_ttl_strd_enrg_sr_lvl: self.hvess.hvessd11_total_stored_energy,
            hvss_pwr_mdl_eltrns_tmprtr: self.hvess.hvessd11_power_module_electronics_temp,
        };
        if let Ok((can_id, data)) = hvessd11.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD12 - High Voltage Energy Storage System Data 12
        let hvessd12 = HVESSD12 {
            device_id,
            hvess_intake_coolant_pressure: self.hvess.hvessd12_intake_coolant_pressure,
            hvss_estmtd_dshrg_tm_rmnng: self.hvess.hvessd12_estimated_discharge_time,
            hvss_estmtd_chrg_tm_rmnng: self.hvess.hvessd12_estimated_charge_time,
            hvss_hgh_vltg_expsr_indtr: self.hvess.hvessd12_hv_exposure_indicator,
            hvess_power_hold_relay_status: self.hvess.hvessd12_power_hold_relay_status,
            hvss_hgh_vltg_bs_pstv_pr_chrg_rl_stt: self.hvess.hvessd12_positive_precharge_relay,
            hvss_hgh_vltg_bs_ngtv_pr_chrg_rl_stt: self.hvess.hvessd12_negative_precharge_relay,
        };
        if let Ok((can_id, data)) = hvessd12.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD13 - High Voltage Energy Storage System Data 13 (Extended Range)
        let hvessd13 = HVESSD13 {
            device_id,
            hvss_avll_dshrg_pwr_extndd_rng: self.hvess.hvessd13_discharge_power_extended,
            hvss_avll_chrg_pwr_extndd_rng: self.hvess.hvessd13_charge_power_extended,
            hvess_voltage_level_extended_range: self.hvess.hvessd13_voltage_extended,
            hvess_current_extended_range: self.hvess.hvessd13_current_extended,
        };
        if let Ok((can_id, data)) = hvessd13.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD14 - High Voltage Energy Storage System Data 14 (Extended Range Current)
        let hvessd14 = HVESSD14 {
            device_id,
            hvss_mx_istts_ds_ct_lt_extdd_r: self.hvess.hvessd14_max_discharge_current_extended,
            hvss_mx_istts_c_ct_lt_extdd_r: self.hvess.hvessd14_max_charge_current_extended,
            hvess_bus_voltage_extended_range: self.hvess.hvessd14_bus_voltage_extended,
            hvss_mnmm_dshrg_vltg_lmt: self.hvess.hvessd14_min_discharge_voltage_limit,
        };
        if let Ok((can_id, data)) = hvessd14.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSD15 - High Voltage Energy Storage System Data 15 (Nominal Current)
        let hvessd15 = HVESSD15 {
            device_id,
            hvss_nmnl_dshrg_crrnt_lmt: self.hvess.hvessd15_nominal_discharge_current_limit,
            hvess_nominal_charge_current_limit: self.hvess.hvessd15_nominal_charge_current_limit,
        };
        if let Ok((can_id, data)) = hvessd15.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSIS1 - HVESS Internal Segment 1 (voltage/current pairs 1-2)
        let hvessis1 = HVESSIS1 {
            device_id,
            hvss_hgh_vltg_intrnl_vltg_lvl_1: self.hvess.hvessis1_internal_voltage_1,
            hvss_hgh_vltg_intrnl_crrnt_1: self.hvess.hvessis1_internal_current_1,
            hvss_hgh_vltg_intrnl_vltg_lvl_2: self.hvess.hvessis1_internal_voltage_2,
            hvss_hgh_vltg_intrnl_crrnt_2: self.hvess.hvessis1_internal_current_2,
        };
        if let Ok((can_id, data)) = hvessis1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSIS2 - HVESS Internal Segment 2 (voltage/current pairs 3-4)
        let hvessis2 = HVESSIS2 {
            device_id,
            hvss_hgh_vltg_intrnl_vltg_lvl_3: self.hvess.hvessis2_internal_voltage_3,
            hvss_hgh_vltg_intrnl_crrnt_3: self.hvess.hvessis2_internal_current_3,
            hvss_hgh_vltg_intrnl_vltg_lvl_4: self.hvess.hvessis2_internal_voltage_4,
            hvss_hgh_vltg_intrnl_crrnt_4: self.hvess.hvessis2_internal_current_4,
        };
        if let Ok((can_id, data)) = hvessis2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSIS3 - HVESS Internal Segment 3 (voltage/current pairs 5-6)
        let hvessis3 = HVESSIS3 {
            device_id,
            hvss_hgh_vltg_intrnl_vltg_lvl_5: self.hvess.hvessis3_internal_voltage_5,
            hvss_hgh_vltg_intrnl_crrnt_5: self.hvess.hvessis3_internal_current_5,
            hvss_hgh_vltg_intrnl_vltg_lvl_6: self.hvess.hvessis3_internal_voltage_6,
            hvss_hgh_vltg_intrnl_crrnt_6: self.hvess.hvessis3_internal_current_6,
        };
        if let Ok((can_id, data)) = hvessis3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSIS4 - HVESS Internal Segment 4 (voltage/current pairs 7-8)
        let hvessis4 = HVESSIS4 {
            device_id,
            hvss_hgh_vltg_intrnl_vltg_lvl_7: self.hvess.hvessis4_internal_voltage_7,
            hvss_hgh_vltg_intrnl_crrnt_7: self.hvess.hvessis4_internal_current_7,
            hvss_hgh_vltg_intrnl_vltg_lvl_8: self.hvess.hvessis4_internal_voltage_8,
            hvss_hgh_vltg_intrnl_crrnt_8: self.hvess.hvessis4_internal_current_8,
        };
        if let Ok((can_id, data)) = hvessis4.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSIS5 - HVESS Internal Segment 5 (contactor/relay status 1-2)
        let hvessis5 = HVESSIS5 {
            device_id,
            hvss_hgh_vltg_intrnl_pstv_cnttr_1_stt: self.hvess.hvessis5_positive_contactor_1_state,
            hvss_hgh_vltg_intrnl_ngtv_cnttr_1_stt: self.hvess.hvessis5_negative_contactor_1_state,
            hvss_hgh_vltg_intrnl_prhrg_rl_1_stt: self.hvess.hvessis5_precharge_relay_1_state,
            hvss_t_mt_sst_it_ht_1_stts: self.hvess.hvessis5_inline_heater_1_status,
            hvss_hgh_vltg_intrnl_bs_vltg_lvl_1: self.hvess.hvessis5_bus_voltage_1,
            hvss_hgh_vltg_intrnl_pstv_cnttr_2_stt: self.hvess.hvessis5_positive_contactor_2_state,
            hvss_hgh_vltg_intrnl_ngtv_cnttr_2_stt: self.hvess.hvessis5_negative_contactor_2_state,
            hvss_hgh_vltg_intrnl_prhrg_rl_2_stt: self.hvess.hvessis5_precharge_relay_2_state,
            hvss_t_mt_sst_it_ht_2_stts: self.hvess.hvessis5_inline_heater_2_status,
            hvss_hgh_vltg_intrnl_bs_vltg_lvl_2: self.hvess.hvessis5_bus_voltage_2,
        };
        if let Ok((can_id, data)) = hvessis5.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSIS6 - HVESS Internal Segment 6 (contactor/relay status 3-4)
        let hvessis6 = HVESSIS6 {
            device_id,
            hvss_hgh_vltg_intrnl_pstv_cnttr_3_stt: self.hvess.hvessis6_positive_contactor_3_state,
            hvss_hgh_vltg_intrnl_ngtv_cnttr_3_stt: self.hvess.hvessis6_negative_contactor_3_state,
            hvss_hgh_vltg_intrnl_prhrg_rl_3_stt: self.hvess.hvessis6_precharge_relay_3_state,
            hvss_t_mt_sst_it_ht_3_stts: self.hvess.hvessis6_inline_heater_3_status,
            hvss_hgh_vltg_intrnl_bs_vltg_lvl_3: self.hvess.hvessis6_bus_voltage_3,
            hvss_hgh_vltg_intrnl_pstv_cnttr_4_stt: self.hvess.hvessis6_positive_contactor_4_state,
            hvss_hgh_vltg_intrnl_ngtv_cnttr_4_stt: self.hvess.hvessis6_negative_contactor_4_state,
            hvss_hgh_vltg_intrnl_prhrg_rl_4_stt: self.hvess.hvessis6_precharge_relay_4_state,
            hvss_t_mt_sst_it_ht_4_stts: self.hvess.hvessis6_inline_heater_4_status,
            hvss_hgh_vltg_intrnl_bs_vltg_lvl_4: self.hvess.hvessis6_bus_voltage_4,
        };
        if let Ok((can_id, data)) = hvessis6.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSIS7 - HVESS Internal Segment 7
        let hvessis7 = HVESSIS7 {
            device_id,
            hvss_nmr_of_intrnl_crts_rd: self.hvess.hvessis7_number_of_internal_circuits,
        };
        if let Ok((can_id, data)) = hvessis7.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSMS1 - HVESS Module Status 1 (modules 1-32)
        let hvessms1 = HVESSMS1 {
            device_id,
            hvess_module_1_operational_status: self.hvess.hvessms1_module_status[0],
            hvess_module_2_operational_status: self.hvess.hvessms1_module_status[1],
            hvess_module_3_operational_status: self.hvess.hvessms1_module_status[2],
            hvess_module_4_operational_status: self.hvess.hvessms1_module_status[3],
            hvess_module_5_operational_status: self.hvess.hvessms1_module_status[4],
            hvess_module_6_operational_status: self.hvess.hvessms1_module_status[5],
            hvess_module_7_operational_status: self.hvess.hvessms1_module_status[6],
            hvess_module_8_operational_status: self.hvess.hvessms1_module_status[7],
            hvess_module_9_operational_status: self.hvess.hvessms1_module_status[8],
            hvess_module_10_operational_status: self.hvess.hvessms1_module_status[9],
            hvess_module_11_operational_status: self.hvess.hvessms1_module_status[10],
            hvess_module_12_operational_status: self.hvess.hvessms1_module_status[11],
            hvess_module_13_operational_status: self.hvess.hvessms1_module_status[12],
            hvess_module_14_operational_status: self.hvess.hvessms1_module_status[13],
            hvess_module_15_operational_status: self.hvess.hvessms1_module_status[14],
            hvess_module_16_operational_status: self.hvess.hvessms1_module_status[15],
            hvess_module_17_operational_status: self.hvess.hvessms1_module_status[16],
            hvess_module_18_operational_status: self.hvess.hvessms1_module_status[17],
            hvess_module_19_operational_status: self.hvess.hvessms1_module_status[18],
            hvess_module_20_operational_status: self.hvess.hvessms1_module_status[19],
            hvess_module_21_operational_status: self.hvess.hvessms1_module_status[20],
            hvess_module_22_operational_status: self.hvess.hvessms1_module_status[21],
            hvess_module_23_operational_status: self.hvess.hvessms1_module_status[22],
            hvess_module_24_operational_status: self.hvess.hvessms1_module_status[23],
            hvess_module_25_operational_status: self.hvess.hvessms1_module_status[24],
            hvess_module_26_operational_status: self.hvess.hvessms1_module_status[25],
            hvess_module_27_operational_status: self.hvess.hvessms1_module_status[26],
            hvess_module_28_operational_status: self.hvess.hvessms1_module_status[27],
            hvess_module_29_operational_status: self.hvess.hvessms1_module_status[28],
            hvess_module_30_operational_status: self.hvess.hvessms1_module_status[29],
            hvess_module_31_operational_status: self.hvess.hvessms1_module_status[30],
            hvess_module_32_operational_status: self.hvess.hvessms1_module_status[31],
        };
        if let Ok((can_id, data)) = hvessms1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSMS2 - HVESS Module Status 2 (modules 33-64)
        let hvessms2 = HVESSMS2 {
            device_id,
            hvess_module_33_operational_status: self.hvess.hvessms2_module_status[0],
            hvess_module_34_operational_status: self.hvess.hvessms2_module_status[1],
            hvess_module_35_operational_status: self.hvess.hvessms2_module_status[2],
            hvess_module_36_operational_status: self.hvess.hvessms2_module_status[3],
            hvess_module_37_operational_status: self.hvess.hvessms2_module_status[4],
            hvess_module_38_operational_status: self.hvess.hvessms2_module_status[5],
            hvess_module_39_operational_status: self.hvess.hvessms2_module_status[6],
            hvess_module_40_operational_status: self.hvess.hvessms2_module_status[7],
            hvess_module_41_operational_status: self.hvess.hvessms2_module_status[8],
            hvess_module_42_operational_status: self.hvess.hvessms2_module_status[9],
            hvess_module_43_operational_status: self.hvess.hvessms2_module_status[10],
            hvess_module_44_operational_status: self.hvess.hvessms2_module_status[11],
            hvess_module_45_operational_status: self.hvess.hvessms2_module_status[12],
            hvess_module_46_operational_status: self.hvess.hvessms2_module_status[13],
            hvess_module_47_operational_status: self.hvess.hvessms2_module_status[14],
            hvess_module_48_operational_status: self.hvess.hvessms2_module_status[15],
            hvess_module_49_operational_status: self.hvess.hvessms2_module_status[16],
            hvess_module_50_operational_status: self.hvess.hvessms2_module_status[17],
            hvess_module_51_operational_status: self.hvess.hvessms2_module_status[18],
            hvess_module_52_operational_status: self.hvess.hvessms2_module_status[19],
            hvess_module_53_operational_status: self.hvess.hvessms2_module_status[20],
            hvess_module_54_operational_status: self.hvess.hvessms2_module_status[21],
            hvess_module_55_operational_status: self.hvess.hvessms2_module_status[22],
            hvess_module_56_operational_status: self.hvess.hvessms2_module_status[23],
            hvess_module_57_operational_status: self.hvess.hvessms2_module_status[24],
            hvess_module_58_operational_status: self.hvess.hvessms2_module_status[25],
            hvess_module_59_operational_status: self.hvess.hvessms2_module_status[26],
            hvess_module_60_operational_status: self.hvess.hvessms2_module_status[27],
            hvess_module_61_operational_status: self.hvess.hvessms2_module_status[28],
            hvess_module_62_operational_status: self.hvess.hvessms2_module_status[29],
            hvess_module_63_operational_status: self.hvess.hvessms2_module_status[30],
            hvess_module_64_operational_status: self.hvess.hvessms2_module_status[31],
        };
        if let Ok((can_id, data)) = hvessms2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSMS3 - HVESS Module Status 3 (modules 65-96)
        let hvessms3 = HVESSMS3 {
            device_id,
            hvess_module_65_operational_status: self.hvess.hvessms3_module_status[0],
            hvess_module_66_operational_status: self.hvess.hvessms3_module_status[1],
            hvess_module_67_operational_status: self.hvess.hvessms3_module_status[2],
            hvess_module_68_operational_status: self.hvess.hvessms3_module_status[3],
            hvess_module_69_operational_status: self.hvess.hvessms3_module_status[4],
            hvess_module_70_operational_status: self.hvess.hvessms3_module_status[5],
            hvess_module_71_operational_status: self.hvess.hvessms3_module_status[6],
            hvess_module_72_operational_status: self.hvess.hvessms3_module_status[7],
            hvess_module_73_operational_status: self.hvess.hvessms3_module_status[8],
            hvess_module_74_operational_status: self.hvess.hvessms3_module_status[9],
            hvess_module_75_operational_status: self.hvess.hvessms3_module_status[10],
            hvess_module_76_operational_status: self.hvess.hvessms3_module_status[11],
            hvess_module_77_operational_status: self.hvess.hvessms3_module_status[12],
            hvess_module_78_operational_status: self.hvess.hvessms3_module_status[13],
            hvess_module_79_operational_status: self.hvess.hvessms3_module_status[14],
            hvess_module_80_operational_status: self.hvess.hvessms3_module_status[15],
            hvess_module_81_operational_status: self.hvess.hvessms3_module_status[16],
            hvess_module_82_operational_status: self.hvess.hvessms3_module_status[17],
            hvess_module_83_operational_status: self.hvess.hvessms3_module_status[18],
            hvess_module_84_operational_status: self.hvess.hvessms3_module_status[19],
            hvess_module_85_operational_status: self.hvess.hvessms3_module_status[20],
            hvess_module_86_operational_status: self.hvess.hvessms3_module_status[21],
            hvess_module_87_operational_status: self.hvess.hvessms3_module_status[22],
            hvess_module_88_operational_status: self.hvess.hvessms3_module_status[23],
            hvess_module_89_operational_status: self.hvess.hvessms3_module_status[24],
            hvess_module_90_operational_status: self.hvess.hvessms3_module_status[25],
            hvess_module_91_operational_status: self.hvess.hvessms3_module_status[26],
            hvess_module_92_operational_status: self.hvess.hvessms3_module_status[27],
            hvess_module_93_operational_status: self.hvess.hvessms3_module_status[28],
            hvess_module_94_operational_status: self.hvess.hvessms3_module_status[29],
            hvess_module_95_operational_status: self.hvess.hvessms3_module_status[30],
            hvess_module_96_operational_status: self.hvess.hvessms3_module_status[31],
        };
        if let Ok((can_id, data)) = hvessms3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSS1 - HVESS System Status 1
        let hvesss1 = HVESSS1 {
            device_id,
            hvss_hgh_vltg_bs_pstv_cnttr_stt: self.hvess.hvesss1_positive_contactor_state,
            hvss_hgh_vltg_bs_ngtv_cnttr_stt: self.hvess.hvesss1_negative_contactor_state,
            hvss_hgh_vltg_bs_dsnnt_frwrnng: self.hvess.hvesss1_disconnect_forewarning,
            hvss_hgh_vltg_bs_pr_chrg_rl_stt: self.hvess.hvesss1_precharge_relay_state,
            hvess_center_of_pack_contactor_state: self.hvess.hvesss1_center_of_pack_contactor,
            hvss_hgh_vltg_bs_atv_isltn_tst_stts: self.hvess.hvesss1_active_isolation_test_status,
            hvss_hgh_vltg_bs_pssv_isltn_tst_stts: self.hvess.hvesss1_passive_isolation_test_status,
            hvess_hvil_status: self.hvess.hvesss1_hvil_status,
            hvess_inertia_switch_status: self.hvess.hvesss1_inertia_switch_status,
            hvess_state_of_charge_status: self.hvess.hvesss1_soc_status,
            hvess_cell_balance_status: self.hvess.hvesss1_cell_balance_status,
            hvess_cell_balancing_active: self.hvess.hvesss1_cell_balancing_active,
            hvess_internal_charger_status: self.hvess.hvesss1_internal_charger_status,
            hvess_status_1_counter: self.hvess.hvesss1_counter,
            hvss_hgh_vltg_bs_cnntn_stts: self.hvess.hvesss1_bus_connection_status,
            hvess_operational_status: self.hvess.hvesss1_operational_status,
            hvess_number_of_hvesps_ready: self.hvess.hvesss1_num_packs_ready,
            hvess_status_1_crc: self.hvess.hvesss1_crc,
        };
        if let Ok((can_id, data)) = hvesss1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSS2 - HVESS System Status 2 (Power Limit Derating)
        let hvesss2 = HVESSS2 {
            device_id,
            hvss_dshrg_pwr_lmt_dt_stt_of_chrg: self.hvess.hvesss2_discharge_limit_soc,
            hvss_dshrg_pwr_lmt_dt_bttr_tmprtr: self.hvess.hvesss2_discharge_limit_temp,
            hvss_dshrg_pwr_lmt_dt_bttr_dgnst_cndtn: self.hvess.hvesss2_discharge_limit_diag,
            hvss_dshrg_pwr_lmt_dt_bttr_or_cll_vltg: self.hvess.hvesss2_discharge_limit_voltage,
            hvss_dshrg_pwr_lmt_dt_bttr_crrnt: self.hvess.hvesss2_discharge_limit_current,
            hvss_dshrg_pwr_lmt_dt_an_undfnd_cs: self.hvess.hvesss2_discharge_limit_undefined,
            hvss_ds_pw_lt_dt_pw_md_ets_tpt: self.hvess.hvesss2_discharge_limit_electronics_temp,
            hvss_chrg_pwr_lmt_dt_stt_of_chrg: self.hvess.hvesss2_charge_limit_soc,
            hvss_chrg_pwr_lmt_dt_bttr_tmprtr: self.hvess.hvesss2_charge_limit_temp,
            hvss_chrg_pwr_lmt_dt_bttr_dgnst_cndtn: self.hvess.hvesss2_charge_limit_diag,
            hvss_chrg_pwr_lmt_dt_bttr_or_cll_vltg: self.hvess.hvesss2_charge_limit_voltage,
            hvss_chrg_pwr_lmt_dt_bttr_crrnt: self.hvess.hvesss2_charge_limit_current,
            hvss_chrg_pwr_lmt_dt_an_undfnd_cs: self.hvess.hvesss2_charge_limit_undefined,
            hvss_c_pw_lt_dt_pw_md_ets_tpt: self.hvess.hvesss2_charge_limit_electronics_temp,
        };
        if let Ok((can_id, data)) = hvesss2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSFS2 - HVESS Fan Status 2
        let hvessfs2 = HVESSFS2 {
            device_id,
            hvess_fan_voltage: self.hvess.hvessfs2_fan_voltage,
            hvess_fan_current: self.hvess.hvessfs2_fan_current,
            hvess_fan_hvil_status: self.hvess.hvessfs2_fan_hvil_status,
            hvess_fan_status_2_instance: self.hvess.hvessfs2_fan_status_2_instance,
            hvess_fan_percent_speed_status: self.hvess.hvessfs2_fan_percent_speed_status,
            hvess_fan_percent_speed: self.hvess.hvessfs2_fan_percent_speed,
        };
        if let Ok((can_id, data)) = hvessfs2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSFC - HVESS Fan Command
        let hvessfc = HVESSFC {
            device_id,
            hvess_fan_enable_command: self.hvess.hvessfc_fan_enable_command,
            hvess_fan_power_hold: self.hvess.hvessfc_fan_power_hold,
            hvess_fan_speed_command: self.hvess.hvessfc_fan_speed_command,
            hvess_fan_percent_speed_command: self.hvess.hvessfc_fan_percent_speed_command,
            hvess_fan_instance_1: 1,
            hvess_fan_instance_2: 0,
            hvess_fan_instance_3: 0,
            hvess_fan_instance_4: 0,
            hvess_fan_instance_5: 0,
            hvess_fan_instance_6: 0,
            hvess_fan_instance_7: 0,
            hvess_fan_instance_8: 0,
            hvess_fan_instance_9: 0,
            hvess_fan_instance_10: 0,
            hvess_fan_instance_11: 0,
            hvess_fan_instance_12: 0,
            hvess_fan_instance_13: 0,
            hvess_fan_instance_14: 0,
            hvess_fan_instance_15: 0,
            h_vt_e_st_sst_f_cds_eddd_itt_sppt: 0,
            hgh_vltg_enrg_strg_sstm_fn_cmmnds_cntr: 0,
            hgh_vltg_enrg_strg_sstm_fn_cmmnds_cr: 0,
        };
        if let Ok((can_id, data)) = hvessfc.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSCFG - HVESS Configuration
        let hvesscfg = HVESSCFG {
            device_id,
            hvess_nominal_voltage: self.hvess.hvesscfg_nominal_voltage,
            hvss_rmmndd_mnmm_oprtng_vltg: self.hvess.hvesscfg_min_operating_voltage,
            hvss_rmmndd_mxmm_oprtng_vltg: self.hvess.hvesscfg_max_operating_voltage,
            hvss_rmmndd_mnmm_stt_of_chrg: self.hvess.hvesscfg_min_soc,
            hvss_rmmndd_mxmm_stt_of_chrg: self.hvess.hvesscfg_max_soc,
            hvss_rmmndd_mxmm_oprtng_tmprtr: self.hvess.hvesscfg_max_operating_temp,
            hvss_rmmndd_mnmm_oprtng_tmprtr: self.hvess.hvesscfg_min_operating_temp,
            hvess_cell_maximum_voltage_limit: self.hvess.hvesscfg_cell_max_voltage,
            hvess_cell_minimum_voltage_limit: self.hvess.hvesscfg_cell_min_voltage,
            hvess_number_of_hvesps_configured: self.hvess.hvesscfg_num_packs,
            hvess_nominal_rated_capacity: self.hvess.hvesscfg_nominal_capacity,
        };
        if let Ok((can_id, data)) = hvesscfg.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSCP1C - HVESS Coolant Pump 1 Command
        let hvesscp1c = HVESSCP1C {
            device_id,
            hvess_coolant_pump_1_enable_command: self.hvess.hvesscp1c_enable_command,
            hvess_coolant_pump_1_power_hold: self.hvess.hvesscp1c_power_hold,
            hvess_coolant_pump_1_speed_command: self.hvess.hvesscp1c_speed_command,
            hvss_clnt_pmp_1_prnt_spd_cmmnd: self.hvess.hvesscp1c_percent_speed_command,
            hvss_ct_pp_1_cd_eddd_itt_sppt: 0,
            hvess_coolant_pump_1_command_counter: 0,
            hvess_coolant_pump_1_command_crc: 0,
        };
        if let Ok((can_id, data)) = hvesscp1c.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSCP1S1 - HVESS Coolant Pump 1 Status 1
        let hvesscp1s1 = HVESSCP1S1 {
            device_id,
            hvss_clnt_pmp_1_mtr_spd_stts: self.hvess.hvesscp1s1_motor_speed_status,
            hvss_clnt_pmp_1_cntrllr_stts_rsn_cd: self.hvess.hvesscp1s1_controller_status_reason,
            hvss_clnt_pmp_1_cntrllr_cmmnd_stts: self.hvess.hvesscp1s1_controller_command_status,
            hvess_coolant_pump_1_motor_speed: self.hvess.hvesscp1s1_motor_speed,
            hvss_clnt_pmp_1_cntrl_tmprtr: self.hvess.hvesscp1s1_control_temperature,
            hvess_coolant_pump_1_power: self.hvess.hvesscp1s1_power,
            hvss_clnt_pmp_1_srv_indtr: self.hvess.hvesscp1s1_service_indicator,
            hvss_clnt_pmp_1_oprtng_stts: self.hvess.hvesscp1s1_operating_status,
        };
        if let Ok((can_id, data)) = hvesscp1s1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSCP1S2 - HVESS Coolant Pump 1 Status 2
        let hvesscp1s2 = HVESSCP1S2 {
            device_id,
            hvess_coolant_pump_1_voltage: self.hvess.hvesscp1s2_voltage,
            hvess_coolant_pump_1_current: self.hvess.hvesscp1s2_current,
            hvess_coolant_pump_1_hvil_status: self.hvess.hvesscp1s2_hvil_status,
            hvss_clnt_pmp_1_prnt_spd_stts: self.hvess.hvesscp1s2_percent_speed_status,
            hvess_coolant_pump_1_percent_speed: self.hvess.hvesscp1s2_percent_speed,
        };
        if let Ok((can_id, data)) = hvesscp1s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSCP2C - HVESS Coolant Pump 2 Command
        let hvesscp2c = HVESSCP2C {
            device_id,
            hvess_coolant_pump_2_enable_command: self.hvess.hvesscp2c_enable_command,
            hvess_coolant_pump_2_power_hold: self.hvess.hvesscp2c_power_hold,
            hvess_coolant_pump_2_speed_command: self.hvess.hvesscp2c_speed_command,
            hvss_clnt_pmp_2_prnt_spd_cmmnd: self.hvess.hvesscp2c_percent_speed_command,
            hvss_ct_pp_2_cd_eddd_itt_sppt: 0,
            hvess_coolant_pump_2_command_counter: 0,
            hvess_coolant_pump_2_command_crc: 0,
        };
        if let Ok((can_id, data)) = hvesscp2c.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSCP2S1 - HVESS Coolant Pump 2 Status 1
        let hvesscp2s1 = HVESSCP2S1 {
            device_id,
            hvss_clnt_pmp_2_mtr_spd_stts: self.hvess.hvesscp2s1_motor_speed_status,
            hvss_clnt_pmp_2_cntrllr_stts_rsn_cd: self.hvess.hvesscp2s1_controller_status_reason,
            hvss_clnt_pmp_2_cntrllr_cmmnd_stts: self.hvess.hvesscp2s1_controller_command_status,
            hvess_coolant_pump_2_motor_speed: self.hvess.hvesscp2s1_motor_speed,
            hvss_clnt_pmp_2_cntrl_tmprtr: self.hvess.hvesscp2s1_control_temperature,
            hvess_coolant_pump_2_power: self.hvess.hvesscp2s1_power,
            hvss_clnt_pmp_2_srv_indtr: self.hvess.hvesscp2s1_service_indicator,
            hvss_clnt_pmp_2_oprtng_stts: self.hvess.hvesscp2s1_operating_status,
        };
        if let Ok((can_id, data)) = hvesscp2s1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSCP2S2 - HVESS Coolant Pump 2 Status 2
        let hvesscp2s2 = HVESSCP2S2 {
            device_id,
            hvess_coolant_pump_2_voltage: self.hvess.hvesscp2s2_voltage,
            hvess_coolant_pump_2_current: self.hvess.hvesscp2s2_current,
            hvess_coolant_pump_2_hvil_status: self.hvess.hvesscp2s2_hvil_status,
            hvss_clnt_pmp_2_prnt_spd_stts: self.hvess.hvesscp2s2_percent_speed_status,
            hvess_coolant_pump_2_percent_speed: self.hvess.hvesscp2s2_percent_speed,
        };
        if let Ok((can_id, data)) = hvesscp2s2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSTCH1 - HVESS Thermal Channel 1
        let hvesstch1 = HVESSTCH1 {
            device_id,
            hvss_t_mt_sst_c_1_cpss_ds_ast_pss: self.hvess.hvesstch1_compressor_discharge_abs_pressure,
            hvss_t_mt_sst_c_1_cpss_st_ast_pss: self.hvess.hvesstch1_compressor_suction_abs_pressure,
            hvss_t_mt_sst_c_1_ott_ct_tpt: self.hvess.hvesstch1_outlet_coolant_temp,
            hvss_t_mt_sst_c_1_c_vv_pst: self.hvess.hvesstch1_condenser_valve_position,
        };
        if let Ok((can_id, data)) = hvesstch1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSTCH2 - HVESS Thermal Channel 2
        let hvesstch2 = HVESSTCH2 {
            device_id,
            hvss_t_mt_sst_c_2_cpss_ds_ast_pss: self.hvess.hvesstch2_compressor_discharge_abs_pressure,
            hvss_t_mt_sst_c_2_cpss_st_ast_pss: self.hvess.hvesstch2_compressor_suction_abs_pressure,
            hvss_t_mt_sst_c_2_ott_ct_tpt: self.hvess.hvesstch2_outlet_coolant_temp,
            hvss_t_mt_sst_c_2_c_vv_pst: self.hvess.hvesstch2_condenser_valve_position,
        };
        if let Ok((can_id, data)) = hvesstch2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSTCH3 - HVESS Thermal Channel 3
        let hvesstch3 = HVESSTCH3 {
            device_id,
            hvss_t_mt_sst_c_3_cpss_ds_ast_pss: self.hvess.hvesstch3_compressor_discharge_abs_pressure,
            hvss_t_mt_sst_c_3_cpss_st_ast_pss: self.hvess.hvesstch3_compressor_suction_abs_pressure,
            hvss_t_mt_sst_c_3_ott_ct_tpt: self.hvess.hvesstch3_outlet_coolant_temp,
            hvss_t_mt_sst_c_3_c_vv_pst: self.hvess.hvesstch3_condenser_valve_position,
        };
        if let Ok((can_id, data)) = hvesstch3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // HVESSHIST - HVESS History/Lifetime Data
        let hvesshist = HVESSHIST {
            device_id,
            hvess_state_of_health: self.hvess.hvesshist_state_of_health,
            hvss_cnttr_opn_undr_ld_cnt: self.hvess.hvesshist_contactor_open_under_load,
            hvess_total_energy_throughput: self.hvess.hvesshist_total_energy_throughput,
            hvess_total_accumulated_charge: self.hvess.hvesshist_total_accumulated_charge,
            hvess_total_lifetime_energy_input: self.hvess.hvesshist_lifetime_energy_input,
            hvess_total_lifetime_energy_output: self.hvess.hvesshist_lifetime_energy_output,
        };
        if let Ok((can_id, data)) = hvesshist.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
