use crate::SimulatorState;
use cando_messages::common::DeviceId;
use cando_messages::j1939::*;
use cando_simulator_common::{create_can_frame, FrameType};
use socketcan::CanFrame;

impl SimulatorState {
    pub(crate) fn generate_motor_frames(&self, frames: &mut Vec<CanFrame>, device_id: DeviceId) {
        // MG1IS1 - Motor/Generator 1 Inverter Status 1 (Primary Motor Status)
        let mg1is1 = MG1IS1 {
            device_id,
            mtr_gnrtr_1_invrtr_stts_1_cntr: (self.motor.mg1_status_counter % 256) as u8,
            motor_generator_1_speed: self.motor.mg1_actual_speed,
            mtr_gnrtr_1_invrtr_stts_1_cr: ((self.motor.mg1_status_counter * 7) % 250) as u8, // Simple CRC
            mtr_gnrtr_1_invrtr_d_sd_crrnt: self.motor.mg1_current,
            mtr_gnrtr_1_invrtr_d_sd_vltg: self.motor.mg1_voltage,
            motor_generator_1_net_rotor_torque: self.motor.mg1_actual_torque,
        };

        if let Ok((can_id, data)) = mg1is1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG2IS1 - Motor/Generator 2 Inverter Status 1 (Secondary Motor Status)
        let mg2is1 = MG2IS1 {
            device_id,
            mtr_gnrtr_2_invrtr_stts_1_cntr: (self.motor.mg2_status_counter % 256) as u8,
            motor_generator_2_speed: self.motor.mg2_actual_speed,
            mtr_gnrtr_2_invrtr_stts_1_cr: ((self.motor.mg2_status_counter * 7) % 250) as u8, // Simple CRC
            mtr_gnrtr_2_invrtr_d_sd_crrnt: self.motor.mg2_current,
            mtr_gnrtr_2_invrtr_d_sd_vltg: self.motor.mg2_voltage,
            motor_generator_2_net_rotor_torque: self.motor.mg2_actual_torque,
        };

        if let Ok((can_id, data)) = mg2is1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG1IS2 - Motor/Generator 1 Inverter Status 2 (Advanced Diagnostics)
        let mg1is2 = MG1IS2 {
            device_id,
            mtr_gnrtr_1_invrtr_stts_2_cntr: (self.motor.mg1_status_counter % 256) as u8,
            mtr_gnrtr_1_avll_mxmm_trq: self.motor.mg1_max_torque,
            mtr_gnrtr_1_avll_mnmm_trq: self.motor.mg1_min_torque,
            mtr_gnrtr_1_invrtr_stts_2_cr: ((self.motor.mg1_status_counter * 11) % 250) as u8,
        };

        if let Ok((can_id, data)) = mg1is2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // ============================================================================
        // Batch 8: Extended Motor/Generator Broadcast Frames
        // ============================================================================

        // MG2IS2 - Motor/Generator 2 Inverter Status 2
        let mg2is2 = MG2IS2 {
            device_id,
            mtr_gnrtr_2_invrtr_stts_2_cntr: (self.motor.mg2_status_counter % 256) as u8,
            mtr_gnrtr_2_avll_mxmm_trq: self.motor.mg2_max_torque,
            mtr_gnrtr_2_avll_mnmm_trq: self.motor.mg2_min_torque,
            mtr_gnrtr_2_invrtr_stts_2_cr: ((self.motor.mg2_status_counter * 11) % 250) as u8,
        };
        if let Ok((can_id, data)) = mg2is2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG1IS3 - Motor/Generator 1 Inverter Control Status 3 (Motor Angle)
        let mg1is3 = MG1IS3 {
            device_id,
            mtr_gnrtr_1_invrtr_cntrl_stts_3_cr: ((self.motor.mg1_status_counter * 13) % 250) as u8,
            mtr_gnrtr_1_invrtr_cntrl_stts_3_cntr: (self.motor.mg1_status_counter % 16) as u8,
            motor_generator_1_motor_angle: self.motor.mg1_motor_angle,
        };
        if let Ok((can_id, data)) = mg1is3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG2IS3 - Motor/Generator 2 Inverter Control Status 3 (Motor Angle)
        let mg2is3 = MG2IS3 {
            device_id,
            mtr_gnrtr_2_invrtr_cntrl_stts_3_cr: ((self.motor.mg2_status_counter * 13) % 250) as u8,
            mtr_gnrtr_2_invrtr_cntrl_stts_3_cntr: (self.motor.mg2_status_counter % 16) as u8,
            motor_generator_2_motor_angle: self.motor.mg2_motor_angle,
        };
        if let Ok((can_id, data)) = mg2is3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG1IT - Motor/Generator 1 Inverter Temperature
        let mg1it = MG1IT {
            device_id,
            mtr_gnrtr_1_invrtr_tmprtr_1: self.motor.mg1_inverter_temp1,
            mtr_gnrtr_1_invrtr_tmprtr_2: self.motor.mg1_inverter_temp2,
            mtr_gnrtr_1_invrtr_tmprtr_3: self.motor.mg1_inverter_temp3,
            mtr_gnrtr_1_invrtr_tmprtr_4: self.motor.mg1_inverter_temp4,
            mtr_gnrtr_1_invrtr_tmprtr_5: self.motor.mg1_inverter_temp5,
        };
        if let Ok((can_id, data)) = mg1it.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG2IT - Motor/Generator 2 Inverter Temperature
        let mg2it = MG2IT {
            device_id,
            mtr_gnrtr_2_invrtr_tmprtr_1: self.motor.mg2_inverter_temp1,
            mtr_gnrtr_2_invrtr_tmprtr_2: self.motor.mg2_inverter_temp2,
            mtr_gnrtr_2_invrtr_tmprtr_3: self.motor.mg2_inverter_temp3,
            mtr_gnrtr_2_invrtr_tmprtr_4: self.motor.mg2_inverter_temp4,
            mtr_gnrtr_2_invrtr_tmprtr_5: self.motor.mg2_inverter_temp5,
        };
        if let Ok((can_id, data)) = mg2it.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG1II - Motor/Generator 1 Inverter Isolation Integrity
        let mg1ii = MG1II {
            device_id,
            mtr_gnrtr_1_invrtr_isltn_intgrt_cr: ((self.motor.mg1_status_counter * 17) % 250) as u8,
            mtr_gnrtr_1_invrtr_isltn_intgrt_cntr: (self.motor.mg1_status_counter % 16) as u8,
            mt_gt_1_ivt_d_sd_ntv_t_csss_gd_vt: self.motor.mg1_isolation_neg_voltage,
            mt_gt_1_ivt_h_vt_bs_atv_ist_tst_stts: 0,
            mt_gt_1_ivt_h_vt_bs_pssv_ist_tst_stts: 0,
            mt_gt_1_ivt_h_vt_bs_atv_ist_tst_rsts: 100.0,
            mt_gt_1_ivt_h_vt_bs_pssv_ist_tst_rsts: 100.0,
        };
        if let Ok((can_id, data)) = mg1ii.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG2II - Motor/Generator 2 Inverter Isolation Integrity
        let mg2ii = MG2II {
            device_id,
            mtr_gnrtr_2_invrtr_isltn_intgrt_cr: ((self.motor.mg2_status_counter * 17) % 250) as u8,
            mtr_gnrtr_2_invrtr_isltn_intgrt_cntr: (self.motor.mg2_status_counter % 16) as u8,
            mt_gt_2_ivt_d_sd_ntv_t_csss_gd_vt: self.motor.mg2_isolation_neg_voltage,
            mt_gt_2_ivt_h_vt_bs_atv_ist_tst_stts: 0,
            mt_gt_2_ivt_h_vt_bs_pssv_ist_tst_stts: 0,
            mt_gt_2_ivt_h_vt_bs_atv_ist_tst_rsts: 100.0,
            mt_gt_2_ivt_h_vt_bs_pssv_ist_tst_rsts: 100.0,
        };
        if let Ok((can_id, data)) = mg2ii.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG1IR1 - Motor/Generator 1 Inverter Reference 1
        let mg1ir1 = MG1IR1 {
            device_id,
            mtr_gnrtr_1_invrtr_rfrn_1_cr: ((self.motor.mg1_status_counter * 19) % 250) as u8,
            mtr_gnrtr_1_invrtr_rfrn_1_cntr: (self.motor.mg1_status_counter % 16) as u8,
            mtr_gnrtr_1_invrtr_rfrn_trq: self.motor.mg1_ref_torque,
            mtr_gnrtr_1_invrtr_rfrn_spd: self.motor.mg1_ref_speed,
            mtr_gnrtr_1_invrtr_rfrn_pwr: self.motor.mg1_ref_power,
        };
        if let Ok((can_id, data)) = mg1ir1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG1IR2 - Motor/Generator 1 Inverter Reference 2
        let mg1ir2 = MG1IR2 {
            device_id,
            mtr_gnrtr_1_invrtr_rfrn_2_cr: ((self.motor.mg1_status_counter * 23) % 250) as u8,
            mtr_gnrtr_1_invrtr_rfrn_2_cntr: (self.motor.mg1_status_counter % 16) as u8,
            mtr_gnrtr_1_invrtr_rfrn_crrnt: self.motor.mg1_ref_current,
            mtr_gnrtr_1_invrtr_rfrn_vltg: self.motor.mg1_ref_voltage,
        };
        if let Ok((can_id, data)) = mg1ir2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG2IR1 - Motor/Generator 2 Inverter Reference 1
        let mg2ir1 = MG2IR1 {
            device_id,
            mtr_gnrtr_2_invrtr_rfrn_1_cr: ((self.motor.mg2_status_counter * 19) % 250) as u8,
            mtr_gnrtr_2_invrtr_rfrn_1_cntr: (self.motor.mg2_status_counter % 16) as u8,
            mtr_gnrtr_2_invrtr_rfrn_trq: self.motor.mg2_ref_torque,
            mtr_gnrtr_2_invrtr_rfrn_spd: self.motor.mg2_ref_speed,
            mtr_gnrtr_2_invrtr_rfrn_pwr: self.motor.mg2_ref_power,
        };
        if let Ok((can_id, data)) = mg2ir1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG2IR2 - Motor/Generator 2 Inverter Reference 2
        let mg2ir2 = MG2IR2 {
            device_id,
            mtr_gnrtr_2_invrtr_rfrn_2_cr: ((self.motor.mg2_status_counter * 23) % 250) as u8,
            mtr_gnrtr_2_invrtr_rfrn_2_cntr: (self.motor.mg2_status_counter % 16) as u8,
            mtr_gnrtr_2_invrtr_rfrn_crrnt: self.motor.mg2_ref_current,
            mtr_gnrtr_2_invrtr_rfrn_vltg: self.motor.mg2_ref_voltage,
        };
        if let Ok((can_id, data)) = mg2ir2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG1IRP - Motor/Generator 1 Inverter Limits Request Power
        let mg1irp = MG1IRP {
            device_id,
            mtr_gnrtr_1_invrtr_lmts_rqst_pwr_cr: ((self.motor.mg1_status_counter * 29) % 250) as u8,
            mtr_gnrtr_1_invrtr_lmts_rqst_pwr_cntr: (self.motor.mg1_status_counter % 16) as u8,
            mt_gt_1_ivt_lts_rqst_m_pw_mx: self.motor.mg1_power_limit_mech_max,
            mt_gt_1_ivt_lts_rqst_m_pw_m: self.motor.mg1_power_limit_mech_min,
            mt_gt_1_ivt_lts_rqst_d_sd_pw_mx: self.motor.mg1_power_limit_dc_max,
            mt_gt_1_ivt_lts_rqst_d_sd_pw_m: self.motor.mg1_power_limit_dc_min,
        };
        if let Ok((can_id, data)) = mg1irp.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG2IRP - Motor/Generator 2 Inverter Limits Request Power
        let mg2irp = MG2IRP {
            device_id,
            mtr_gnrtr_2_invrtr_lmts_rqst_pwr_cr: ((self.motor.mg2_status_counter * 29) % 250) as u8,
            mtr_gnrtr_2_invrtr_lmts_rqst_pwr_cntr: (self.motor.mg2_status_counter % 16) as u8,
            mt_gt_2_ivt_lts_rqst_m_pw_mx: self.motor.mg2_power_limit_mech_max,
            mt_gt_2_ivt_lts_rqst_m_pw_m: self.motor.mg2_power_limit_mech_min,
            mt_gt_2_ivt_lts_rqst_d_sd_pw_mx: self.motor.mg2_power_limit_dc_max,
            mt_gt_2_ivt_lts_rqst_d_sd_pw_m: self.motor.mg2_power_limit_dc_min,
        };
        if let Ok((can_id, data)) = mg2irp.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG1IAPL - Motor/Generator 1 Inverter Active Power Limits
        let mg1iapl = MG1IAPL {
            device_id,
            mt_gt_1_ivt_pw_ltd_dt_udd_rs: 0,
            mt_gt_1_ivt_pw_ltd_dtd_sd_ct_mx: 0,
            mt_gt_1_ivt_pw_ltd_dtd_sd_ct_m: 0,
            mt_gt_1_ivt_pw_ltd_dtd_sd_vt_mx: 0,
            mt_gt_1_ivt_pw_ltd_dtd_sd_vt_m: 0,
            mt_gt_1_ivt_pw_ltd_dtm_pw_mx: 0,
            mt_gt_1_ivt_pw_ltd_dtm_pw_m: 0,
            mt_gt_1_ivt_pw_ltd_dtd_sd_pw_mx: 0,
            mt_gt_1_ivt_pw_ltd_dtd_sd_pw_m: 0,
            mtr_gnrtr_1_invrtr_pwr_lmtd_dt_trq_mxmm: 0,
            mtr_gnrtr_1_invrtr_pwr_lmtd_dt_trq_mnmm: 0,
            mtr_gnrtr_1_invrtr_pwr_lmtd_dt_spd_mxmm: 0,
            mtr_gnrtr_1_invrtr_pwr_lmtd_dt_spd_mnmm: 0,
            mt_gt_1_ivt_pw_ltd_dt_ivt_tpt: if self.motor.mg1_inverter_temp1 > 150.0 { 1 } else { 0 },
            mt_gt_1_ivt_pw_ltd_dt_mt_tpt: 0,
            mt_gt_1_ivt_pw_ltd_dt_ft_cdt: 0,
            mt_gt_1_ivt_pw_ltd_dt_atv_tq_rt_lt: 0,
            mt_gt_1_ivt_pw_ltd_dt_atv_spd_rt_lt: 0,
            mt_gt_1_ivt_tq_ltd_dtm_ctsts: 0,
        };
        if let Ok((can_id, data)) = mg1iapl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG2IAPL - Motor/Generator 2 Inverter Active Power Limits
        let mg2iapl = MG2IAPL {
            device_id,
            mt_gt_2_ivt_pw_ltd_dt_udd_rs: 0,
            mt_gt_2_ivt_pw_ltd_dtd_sd_ct_mx: 0,
            mt_gt_2_ivt_pw_ltd_dtd_sd_ct_m: 0,
            mt_gt_2_ivt_pw_ltd_dtd_sd_vt_mx: 0,
            mt_gt_2_ivt_pw_ltd_dtd_sd_vt_m: 0,
            mt_gt_2_ivt_pw_ltd_dtm_pw_mx: 0,
            mt_gt_2_ivt_pw_ltd_dtm_pw_m: 0,
            mt_gt_2_ivt_pw_ltd_dtd_sd_pw_mx: 0,
            mt_gt_2_ivt_pw_ltd_dtd_sd_pw_m: 0,
            mtr_gnrtr_2_invrtr_pwr_lmtd_dt_trq_mxmm: 0,
            mtr_gnrtr_2_invrtr_pwr_lmtd_dt_trq_mnmm: 0,
            mtr_gnrtr_2_invrtr_pwr_lmtd_dt_spd_mxmm: 0,
            mtr_gnrtr_2_invrtr_pwr_lmtd_dt_spd_mnmm: 0,
            mt_gt_2_ivt_pw_ltd_dt_ivt_tpt: if self.motor.mg2_inverter_temp1 > 150.0 { 1 } else { 0 },
            mt_gt_2_ivt_pw_ltd_dt_mt_tpt: 0,
            mt_gt_2_ivt_pw_ltd_dt_ft_cdt: 0,
            mt_gt_2_ivt_pw_ltd_dt_atv_tq_rt_lt: 0,
            mt_gt_2_ivt_pw_ltd_dt_atv_spd_rt_lt: 0,
            mt_gt_2_ivt_tq_ltd_dtm_ctsts: 0,
        };
        if let Ok((can_id, data)) = mg2iapl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG1IMF1 - Motor/Generator 1 Inverter Mode Feedback 1
        let mg1imf1 = MG1IMF1 {
            device_id,
            mtr_gnrtr_1_invrtr_md_fdk_1_cr: ((self.motor.mg1_status_counter * 31) % 250) as u8,
            mtr_gnrtr_1_invrtr_md_fdk_1_cntr: (self.motor.mg1_status_counter % 16) as u8,
            mtr_gnrtr_1_invrtr_cntrl_lmts_ovrrd_md: 0,
            mtr_gnrtr_1_invrtr_cntrl_stpnt_md: 3,
            mtr_gnrtr_1_invrtr_hvl_stts: 0,
            mg_1_rotor_position_sensing_status: 1,
        };
        if let Ok((can_id, data)) = mg1imf1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG2IMF1 - Motor/Generator 2 Inverter Mode Feedback 1
        let mg2imf1 = MG2IMF1 {
            device_id,
            mtr_gnrtr_2_invrtr_md_fdk_1_cr: ((self.motor.mg2_status_counter * 31) % 250) as u8,
            mtr_gnrtr_2_invrtr_md_fdk_1_cntr: (self.motor.mg2_status_counter % 16) as u8,
            mtr_gnrtr_2_invrtr_cntrl_lmts_ovrrd_md: 0,
            mtr_gnrtr_2_invrtr_cntrl_stpnt_md: 3,
            mtr_gnrtr_2_invrtr_hvl_stts: 0,
            mg_2_rotor_position_sensing_status: 1,
        };
        if let Ok((can_id, data)) = mg2imf1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3IS1 - Motor/Generator 3 Inverter Status 1
        let mg3is1 = MG3IS1 {
            device_id,
            mtr_gnrtr_3_invrtr_stts_1_cntr: (self.motor.mg3_status_counter % 256) as u8,
            motor_generator_3_speed: self.motor.mg3_actual_speed,
            mtr_gnrtr_3_invrtr_stts_1_cr: ((self.motor.mg3_status_counter * 7) % 250) as u8,
            mtr_gnrtr_3_invrtr_d_sd_crrnt: self.motor.mg3_current,
            mtr_gnrtr_3_invrtr_d_sd_vltg: self.motor.mg3_voltage,
            motor_generator_3_net_rotor_torque: self.motor.mg3_actual_torque,
        };
        if let Ok((can_id, data)) = mg3is1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3IS2 - Motor/Generator 3 Inverter Status 2
        let mg3is2 = MG3IS2 {
            device_id,
            mtr_gnrtr_3_invrtr_stts_2_cntr: (self.motor.mg3_status_counter % 256) as u8,
            mtr_gnrtr_3_avll_mxmm_trq: self.motor.mg3_max_torque,
            mtr_gnrtr_3_avll_mnmm_trq: self.motor.mg3_min_torque,
            mtr_gnrtr_3_invrtr_stts_2_cr: ((self.motor.mg3_status_counter * 11) % 250) as u8,
        };
        if let Ok((can_id, data)) = mg3is2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3IS3 - Motor/Generator 3 Inverter Control Status 3 (Motor Angle)
        let mg3is3 = MG3IS3 {
            device_id,
            mtr_gnrtr_3_invrtr_cntrl_stts_3_cr: ((self.motor.mg3_status_counter * 13) % 250) as u8,
            mtr_gnrtr_3_invrtr_cntrl_stts_3_cntr: (self.motor.mg3_status_counter % 16) as u8,
            motor_generator_3_motor_angle: self.motor.mg3_motor_angle,
        };
        if let Ok((can_id, data)) = mg3is3.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3IT - Motor/Generator 3 Inverter Temperature
        let mg3it = MG3IT {
            device_id,
            mtr_gnrtr_3_invrtr_tmprtr_1: self.motor.mg3_inverter_temp1,
            mtr_gnrtr_3_invrtr_tmprtr_2: self.motor.mg3_inverter_temp2,
            mtr_gnrtr_3_invrtr_tmprtr_3: self.motor.mg3_inverter_temp3,
            mtr_gnrtr_3_invrtr_tmprtr_4: self.motor.mg3_inverter_temp4,
            mtr_gnrtr_3_invrtr_tmprtr_5: self.motor.mg3_inverter_temp5,
        };
        if let Ok((can_id, data)) = mg3it.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3II - Motor/Generator 3 Inverter Isolation Integrity
        let mg3ii = MG3II {
            device_id,
            mtr_gnrtr_3_invrtr_isltn_intgrt_cr: ((self.motor.mg3_status_counter * 17) % 250) as u8,
            mtr_gnrtr_3_invrtr_isltn_intgrt_cntr: (self.motor.mg3_status_counter % 16) as u8,
            mt_gt_3_ivt_d_sd_ntv_t_csss_gd_vt: self.motor.mg3_isolation_neg_voltage,
            mt_gt_3_ivt_h_vt_bs_atv_ist_tst_stts: 0,
            mt_gt_3_ivt_h_vt_bs_pssv_ist_tst_stts: 0,
            mt_gt_3_ivt_h_vt_bs_atv_ist_tst_rsts: 100.0,
            mt_gt_3_ivt_h_vt_bs_pssv_ist_tst_rsts: 100.0,
        };
        if let Ok((can_id, data)) = mg3ii.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3IR1 - Motor/Generator 3 Inverter Reference 1
        let mg3ir1 = MG3IR1 {
            device_id,
            mtr_gnrtr_3_invrtr_rfrn_1_cr: ((self.motor.mg3_status_counter * 19) % 250) as u8,
            mtr_gnrtr_3_invrtr_rfrn_1_cntr: (self.motor.mg3_status_counter % 16) as u8,
            mtr_gnrtr_3_invrtr_rfrn_trq: self.motor.mg3_ref_torque,
            mtr_gnrtr_3_invrtr_rfrn_spd: self.motor.mg3_ref_speed,
            mtr_gnrtr_3_invrtr_rfrn_pwr: self.motor.mg3_ref_power,
        };
        if let Ok((can_id, data)) = mg3ir1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3IR2 - Motor/Generator 3 Inverter Reference 2
        let mg3ir2 = MG3IR2 {
            device_id,
            mtr_gnrtr_3_invrtr_rfrn_2_cr: ((self.motor.mg3_status_counter * 23) % 250) as u8,
            mtr_gnrtr_3_invrtr_rfrn_2_cntr: (self.motor.mg3_status_counter % 16) as u8,
            mtr_gnrtr_3_invrtr_rfrn_crrnt: self.motor.mg3_ref_current,
            mtr_gnrtr_3_invrtr_rfrn_vltg: self.motor.mg3_ref_voltage,
        };
        if let Ok((can_id, data)) = mg3ir2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3IRP - Motor/Generator 3 Inverter Limits Request Power
        let mg3irp = MG3IRP {
            device_id,
            mtr_gnrtr_3_invrtr_lmts_rqst_pwr_cr: ((self.motor.mg3_status_counter * 29) % 250) as u8,
            mtr_gnrtr_3_invrtr_lmts_rqst_pwr_cntr: (self.motor.mg3_status_counter % 16) as u8,
            mt_gt_3_ivt_lts_rqst_m_pw_mx: self.motor.mg3_power_limit_mech_max,
            mt_gt_3_ivt_lts_rqst_m_pw_m: self.motor.mg3_power_limit_mech_min,
            mt_gt_3_ivt_lts_rqst_d_sd_pw_mx: self.motor.mg3_power_limit_dc_max,
            mt_gt_3_ivt_lts_rqst_d_sd_pw_m: self.motor.mg3_power_limit_dc_min,
        };
        if let Ok((can_id, data)) = mg3irp.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3IAPL - Motor/Generator 3 Inverter Active Power Limits
        let mg3iapl = MG3IAPL {
            device_id,
            mt_gt_3_ivt_pw_ltd_dt_udd_rs: 0,
            mt_gt_3_ivt_pw_ltd_dtd_sd_ct_mx: 0,
            mt_gt_3_ivt_pw_ltd_dtd_sd_ct_m: 0,
            mt_gt_3_ivt_pw_ltd_dtd_sd_vt_mx: 0,
            mt_gt_3_ivt_pw_ltd_dtd_sd_vt_m: 0,
            mt_gt_3_ivt_pw_ltd_dtm_pw_mx: 0,
            mt_gt_3_ivt_pw_ltd_dtm_pw_m: 0,
            mt_gt_3_ivt_pw_ltd_dtd_sd_pw_mx: 0,
            mt_gt_3_ivt_pw_ltd_dtd_sd_pw_m: 0,
            mtr_gnrtr_3_invrtr_pwr_lmtd_dt_trq_mxmm: 0,
            mtr_gnrtr_3_invrtr_pwr_lmtd_dt_trq_mnmm: 0,
            mtr_gnrtr_3_invrtr_pwr_lmtd_dt_spd_mxmm: 0,
            mtr_gnrtr_3_invrtr_pwr_lmtd_dt_spd_mnmm: 0,
            mt_gt_3_ivt_pw_ltd_dt_ivt_tpt: if self.motor.mg3_inverter_temp1 > 150.0 { 1 } else { 0 },
            mt_gt_3_ivt_pw_ltd_dt_mt_tpt: 0,
            mt_gt_3_ivt_pw_ltd_dt_ft_cdt: 0,
            mt_gt_3_ivt_pw_ltd_dt_atv_tq_rt_lt: 0,
            mt_gt_3_ivt_pw_ltd_dt_atv_spd_rt_lt: 0,
            mt_gt_3_ivt_tq_ltd_dtm_ctsts: 0,
        };
        if let Ok((can_id, data)) = mg3iapl.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG3IMF1 - Motor/Generator 3 Inverter Mode Feedback 1
        let mg3imf1 = MG3IMF1 {
            device_id,
            mtr_gnrtr_3_invrtr_md_fdk_1_cr: ((self.motor.mg3_status_counter * 31) % 250) as u8,
            mtr_gnrtr_3_invrtr_md_fdk_1_cntr: (self.motor.mg3_status_counter % 16) as u8,
            mtr_gnrtr_3_invrtr_cntrl_lmts_ovrrd_md: 0,
            mtr_gnrtr_3_invrtr_cntrl_stpnt_md: 3,
            mtr_gnrtr_3_invrtr_hvl_stts: 0,
            mg_3_rotor_position_sensing_status: 1,
        };
        if let Ok((can_id, data)) = mg3imf1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG4IS1 - Motor/Generator 4 Inverter Status 1
        let mg4is1 = MG4IS1 {
            device_id,
            mtr_gnrtr_4_invrtr_stts_1_cntr: (self.motor.mg4_status_counter % 256) as u8,
            motor_generator_4_speed: self.motor.mg4_actual_speed,
            mtr_gnrtr_4_invrtr_stts_1_cr: ((self.motor.mg4_status_counter * 7) % 250) as u8,
            mtr_gnrtr_4_invrtr_d_sd_crrnt: self.motor.mg4_current,
            mtr_gnrtr_4_invrtr_d_sd_vltg: self.motor.mg4_voltage,
            motor_generator_4_net_rotor_torque: self.motor.mg4_actual_torque,
        };
        if let Ok((can_id, data)) = mg4is1.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }

        // MG4IS2 - Motor/Generator 4 Inverter Status 2
        let mg4is2 = MG4IS2 {
            device_id,
            mtr_gnrtr_4_invrtr_stts_2_cntr: (self.motor.mg4_status_counter % 256) as u8,
            mtr_gnrtr_4_avll_mxmm_trq: self.motor.mg4_max_torque,
            mtr_gnrtr_4_avll_mnmm_trq: self.motor.mg4_min_torque,
            mtr_gnrtr_4_invrtr_stts_2_cr: ((self.motor.mg4_status_counter * 11) % 250) as u8,
        };
        if let Ok((can_id, data)) = mg4is2.encode()
            && let Ok(frame) = create_can_frame(can_id, &data, FrameType::Extended)
        {
            frames.push(frame);
        }
    }
}
