use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::j1939::*;

impl SimulatorState {
    /// Handle MG1IC - Motor/Generator 1 Inverter Control
    pub(crate) fn handle_mg1ic(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // MG1IC - Motor/Generator 1 Inverter Control
        match MG1IC::decode(can_id, data) {
            Ok(msg) => {
                self.motor.mg1_speed_setpoint = msg.mtr_gnrtr_1_invrtr_cntrl_stpnt_rqst;
                // Update other control parameters if needed
                println!(
                    "🔧 Received MG1IC: Speed setpoint = {:.1}%",
                    self.motor.mg1_speed_setpoint
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode MG1IC: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG2IC - Motor/Generator 2 Inverter Control
    pub(crate) fn handle_mg2ic(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // MG2IC - Motor/Generator 2 Inverter Control
        match MG2IC::decode(can_id, data) {
            Ok(msg) => {
                self.motor.mg2_speed_setpoint = msg.mtr_gnrtr_2_invrtr_cntrl_stpnt_rqst;
                println!(
                    "🔧 Received MG2IC: Speed setpoint = {:.1}%",
                    self.motor.mg2_speed_setpoint
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("⚠️ Failed to decode MG2IC: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    // ============================================================================
    // Batch 8: Extended Motor/Generator Message Handlers
    // ============================================================================

    /// Handle MG3IC - Motor/Generator 3 Inverter Control
    pub(crate) fn handle_mg3ic(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IC::decode(can_id, data) {
            Ok(msg) => {
                self.motor.mg3_speed_setpoint = msg.mtr_gnrtr_3_invrtr_cntrl_stpnt_rqst;
                println!(
                    "Received MG3IC: Speed setpoint = {:.1}%",
                    self.motor.mg3_speed_setpoint
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IC: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG4IC - Motor/Generator 4 Inverter Control
    pub(crate) fn handle_mg4ic(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG4IC::decode(can_id, data) {
            Ok(msg) => {
                self.motor.mg4_speed_setpoint = msg.mtr_gnrtr_4_invrtr_cntrl_stpnt_rqst;
                println!(
                    "Received MG4IC: Speed setpoint = {:.1}%",
                    self.motor.mg4_speed_setpoint
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG4IC: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG1IS3 - Motor/Generator 1 Inverter Control Status 3
    pub(crate) fn handle_mg1is3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG1IS3::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG1IS3");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG1IS3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG2IS3 - Motor/Generator 2 Inverter Control Status 3
    pub(crate) fn handle_mg2is3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG2IS3::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG2IS3");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG2IS3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG1IT - Motor/Generator 1 Inverter Temperature
    pub(crate) fn handle_mg1it(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG1IT::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG1IT");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG1IT: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG2IT - Motor/Generator 2 Inverter Temperature
    pub(crate) fn handle_mg2it(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG2IT::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG2IT");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG2IT: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG1II - Motor/Generator 1 Inverter Isolation Integrity
    pub(crate) fn handle_mg1ii(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG1II::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG1II");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG1II: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG2II - Motor/Generator 2 Inverter Isolation Integrity
    pub(crate) fn handle_mg2ii(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG2II::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG2II");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG2II: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG1IR1 - Motor/Generator 1 Inverter Reference 1
    pub(crate) fn handle_mg1ir1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG1IR1::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG1IR1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG1IR1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG1IR2 - Motor/Generator 1 Inverter Reference 2
    pub(crate) fn handle_mg1ir2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG1IR2::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG1IR2");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG1IR2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG2IR1 - Motor/Generator 2 Inverter Reference 1
    pub(crate) fn handle_mg2ir1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG2IR1::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG2IR1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG2IR1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG2IR2 - Motor/Generator 2 Inverter Reference 2
    pub(crate) fn handle_mg2ir2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG2IR2::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG2IR2");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG2IR2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG1IRP - Motor/Generator 1 Inverter Limits Request Power
    pub(crate) fn handle_mg1irp(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG1IRP::decode(can_id, data) {
            Ok(msg) => {
                self.motor.mg1_power_limit_mech_max = msg.mt_gt_1_ivt_lts_rqst_m_pw_mx;
                self.motor.mg1_power_limit_mech_min = msg.mt_gt_1_ivt_lts_rqst_m_pw_m;
                self.motor.mg1_power_limit_dc_max = msg.mt_gt_1_ivt_lts_rqst_d_sd_pw_mx;
                self.motor.mg1_power_limit_dc_min = msg.mt_gt_1_ivt_lts_rqst_d_sd_pw_m;
                println!(
                    "Received MG1IRP: mech max={:.1}%",
                    self.motor.mg1_power_limit_mech_max
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG1IRP: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG2IRP - Motor/Generator 2 Inverter Limits Request Power
    pub(crate) fn handle_mg2irp(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG2IRP::decode(can_id, data) {
            Ok(msg) => {
                self.motor.mg2_power_limit_mech_max = msg.mt_gt_2_ivt_lts_rqst_m_pw_mx;
                self.motor.mg2_power_limit_mech_min = msg.mt_gt_2_ivt_lts_rqst_m_pw_m;
                self.motor.mg2_power_limit_dc_max = msg.mt_gt_2_ivt_lts_rqst_d_sd_pw_mx;
                self.motor.mg2_power_limit_dc_min = msg.mt_gt_2_ivt_lts_rqst_d_sd_pw_m;
                println!(
                    "Received MG2IRP: mech max={:.1}%",
                    self.motor.mg2_power_limit_mech_max
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG2IRP: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG1IAPL - Motor/Generator 1 Inverter Active Power Limits
    pub(crate) fn handle_mg1iapl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG1IAPL::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG1IAPL");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG1IAPL: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG2IAPL - Motor/Generator 2 Inverter Active Power Limits
    pub(crate) fn handle_mg2iapl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG2IAPL::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG2IAPL");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG2IAPL: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG1IMF1 - Motor/Generator 1 Inverter Mode Feedback 1
    pub(crate) fn handle_mg1imf1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG1IMF1::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG1IMF1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG1IMF1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG2IMF1 - Motor/Generator 2 Inverter Mode Feedback 1
    pub(crate) fn handle_mg2imf1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG2IMF1::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG2IMF1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG2IMF1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3IS1 - Motor/Generator 3 Inverter Status 1
    pub(crate) fn handle_mg3is1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IS1::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG3IS1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3IS2 - Motor/Generator 3 Inverter Status 2
    pub(crate) fn handle_mg3is2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IS2::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG3IS2");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IS2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3IS3 - Motor/Generator 3 Inverter Control Status 3
    pub(crate) fn handle_mg3is3(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IS3::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG3IS3");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IS3: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3IT - Motor/Generator 3 Inverter Temperature
    pub(crate) fn handle_mg3it(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IT::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG3IT");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IT: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3II - Motor/Generator 3 Inverter Isolation Integrity
    pub(crate) fn handle_mg3ii(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3II::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG3II");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3II: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3IR1 - Motor/Generator 3 Inverter Reference 1
    pub(crate) fn handle_mg3ir1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IR1::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG3IR1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IR1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3IR2 - Motor/Generator 3 Inverter Reference 2
    pub(crate) fn handle_mg3ir2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IR2::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG3IR2");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IR2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3IRP - Motor/Generator 3 Inverter Limits Request Power
    pub(crate) fn handle_mg3irp(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IRP::decode(can_id, data) {
            Ok(msg) => {
                self.motor.mg3_power_limit_mech_max = msg.mt_gt_3_ivt_lts_rqst_m_pw_mx;
                self.motor.mg3_power_limit_mech_min = msg.mt_gt_3_ivt_lts_rqst_m_pw_m;
                self.motor.mg3_power_limit_dc_max = msg.mt_gt_3_ivt_lts_rqst_d_sd_pw_mx;
                self.motor.mg3_power_limit_dc_min = msg.mt_gt_3_ivt_lts_rqst_d_sd_pw_m;
                println!(
                    "Received MG3IRP: mech max={:.1}%",
                    self.motor.mg3_power_limit_mech_max
                );
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IRP: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3IAPL - Motor/Generator 3 Inverter Active Power Limits
    pub(crate) fn handle_mg3iapl(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IAPL::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG3IAPL");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IAPL: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG3IMF1 - Motor/Generator 3 Inverter Mode Feedback 1
    pub(crate) fn handle_mg3imf1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG3IMF1::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG3IMF1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG3IMF1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG4IS1 - Motor/Generator 4 Inverter Status 1
    pub(crate) fn handle_mg4is1(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG4IS1::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG4IS1");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG4IS1: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }

    /// Handle MG4IS2 - Motor/Generator 4 Inverter Status 2
    pub(crate) fn handle_mg4is2(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        match MG4IS2::decode(can_id, data) {
            Ok(_msg) => {
                println!("Received MG4IS2");
                Ok(MessageStatus::Recognized)
            }
            Err(e) => {
                println!("Failed to decode MG4IS2: {}", e);
                Ok(MessageStatus::DecodeFailed)
            }
        }
    }
}
