mod aftertreatment;
mod braking;
mod crash;
mod dcdc;
mod diagnostics;
mod engine;
mod ev_charging;
mod hvess;
mod motor;
mod power_supply;
mod sensors;
mod thermal;
mod transmission;
mod vehicle;

use anyhow::Result;
use crate::{MessageStatus, SimulatorState};
use cando_messages::common::CAN_BASE_ID_MASK;
use cando_messages::j1939::*;
use cando_simulator_common::should_ignore_message;
use cando_simulator_common::ReceivedMessage;

impl SimulatorState {
    /// Process incoming CAN message and update state accordingly
    ///
    /// Returns:
    /// - `Ok(MessageStatus::Recognized)` if message was decoded and processed
    /// - `Ok(MessageStatus::Unrecognized)` if message ID is not recognized
    /// - `Ok(MessageStatus::DecodeFailed)` if message ID is recognized but decode failed
    /// - `Ok(MessageStatus::Ignored)` if message is from this simulator (self-reception)
    pub fn process_incoming_message(&mut self, can_id: u32, data: &[u8]) -> Result<MessageStatus> {
        // Ignore messages from ourselves (prevent self-reception loop)
        // Real CAN devices ignore their own echoed transmissions
        if should_ignore_message(can_id, self.device_id as u8) {
            return Ok(MessageStatus::Ignored);
        }

        // Record message reception for test verification
        let timestamp_ms = self.simulator_start_time.elapsed().as_millis() as u64;

        self.recent_messages.push_back(ReceivedMessage {
            can_id,
            timestamp_ms,
            processed: false, // Will be updated based on decode success
        });

        // Keep only last 100 messages to prevent unbounded growth
        if self.recent_messages.len() > 100 {
            self.recent_messages.pop_front();
        }

        // Extract base message ID (mask device ID)
        let base_id = can_id & CAN_BASE_ID_MASK;

        let result = match base_id {
            // ============================================================================
            // EMP Motor Control Messages (Commands)
            // ============================================================================
            MG1IC::BASE_CAN_ID => self.handle_mg1ic(can_id, data),
            MG2IC::BASE_CAN_ID => self.handle_mg2ic(can_id, data),
            // Batch 8: Extended Motor/Generator Messages
            MG3IC::BASE_CAN_ID => self.handle_mg3ic(can_id, data),
            MG4IC::BASE_CAN_ID => self.handle_mg4ic(can_id, data),
            MG1IS3::BASE_CAN_ID => self.handle_mg1is3(can_id, data),
            MG2IS3::BASE_CAN_ID => self.handle_mg2is3(can_id, data),
            MG1IT::BASE_CAN_ID => self.handle_mg1it(can_id, data),
            MG2IT::BASE_CAN_ID => self.handle_mg2it(can_id, data),
            MG1II::BASE_CAN_ID => self.handle_mg1ii(can_id, data),
            MG2II::BASE_CAN_ID => self.handle_mg2ii(can_id, data),
            MG1IR1::BASE_CAN_ID => self.handle_mg1ir1(can_id, data),
            MG1IR2::BASE_CAN_ID => self.handle_mg1ir2(can_id, data),
            MG2IR1::BASE_CAN_ID => self.handle_mg2ir1(can_id, data),
            MG2IR2::BASE_CAN_ID => self.handle_mg2ir2(can_id, data),
            MG1IRP::BASE_CAN_ID => self.handle_mg1irp(can_id, data),
            MG2IRP::BASE_CAN_ID => self.handle_mg2irp(can_id, data),
            MG1IAPL::BASE_CAN_ID => self.handle_mg1iapl(can_id, data),
            MG2IAPL::BASE_CAN_ID => self.handle_mg2iapl(can_id, data),
            MG1IMF1::BASE_CAN_ID => self.handle_mg1imf1(can_id, data),
            MG2IMF1::BASE_CAN_ID => self.handle_mg2imf1(can_id, data),
            MG3IS1::BASE_CAN_ID => self.handle_mg3is1(can_id, data),
            MG3IS2::BASE_CAN_ID => self.handle_mg3is2(can_id, data),
            MG3IS3::BASE_CAN_ID => self.handle_mg3is3(can_id, data),
            MG3IT::BASE_CAN_ID => self.handle_mg3it(can_id, data),
            MG3II::BASE_CAN_ID => self.handle_mg3ii(can_id, data),
            MG3IR1::BASE_CAN_ID => self.handle_mg3ir1(can_id, data),
            MG3IR2::BASE_CAN_ID => self.handle_mg3ir2(can_id, data),
            MG3IRP::BASE_CAN_ID => self.handle_mg3irp(can_id, data),
            MG3IAPL::BASE_CAN_ID => self.handle_mg3iapl(can_id, data),
            MG3IMF1::BASE_CAN_ID => self.handle_mg3imf1(can_id, data),
            MG4IS1::BASE_CAN_ID => self.handle_mg4is1(can_id, data),
            MG4IS2::BASE_CAN_ID => self.handle_mg4is2(can_id, data),
            HVESSC1::BASE_CAN_ID => self.handle_hvessc1(can_id, data),
            DCDC1C::BASE_CAN_ID => self.handle_dcdc1c(can_id, data),

            ALTC::BASE_CAN_ID => self.handle_altc(can_id, data),

            // ============================================================================
            // Engine Control Messages (Commands - if any)
            // ============================================================================
            // Note: Most engine messages are status/data rather than commands
            // But we can process them for completeness
            ETC9::BASE_CAN_ID => self.handle_etc9(can_id, data),

            // ============================================================================
            // Core Engine Electronics Messages (Batch 1)
            // ============================================================================
            EEC1::BASE_CAN_ID => self.handle_eec1(can_id, data),
            EEC2::BASE_CAN_ID => self.handle_eec2(can_id, data),
            EEC3::BASE_CAN_ID => self.handle_eec3(can_id, data),
            EEC4::BASE_CAN_ID => self.handle_eec4(can_id, data),
            EEC5::BASE_CAN_ID => self.handle_eec5(can_id, data),
            EEC6::BASE_CAN_ID => self.handle_eec6(can_id, data),
            EEC7::BASE_CAN_ID => self.handle_eec7(can_id, data),
            EEC9::BASE_CAN_ID => self.handle_eec9(can_id, data),
            EEC10::BASE_CAN_ID => self.handle_eec10(can_id, data),
            EEC11::BASE_CAN_ID => self.handle_eec11(can_id, data),
            EEC13::BASE_CAN_ID => self.handle_eec13(can_id, data),
            EEC14::BASE_CAN_ID => self.handle_eec14(can_id, data),
            EEC16::BASE_CAN_ID => self.handle_eec16(can_id, data),
            EEC18::BASE_CAN_ID => self.handle_eec18(can_id, data),
            EEC19::BASE_CAN_ID => self.handle_eec19(can_id, data),
            EEC20::BASE_CAN_ID => self.handle_eec20(can_id, data),
            EEC23::BASE_CAN_ID => self.handle_eec23(can_id, data),
            EEC24::BASE_CAN_ID => self.handle_eec24(can_id, data),
            EEC25::BASE_CAN_ID => self.handle_eec25(can_id, data),
            ETC1::BASE_CAN_ID => self.handle_etc1(can_id, data),

            // ============================================================================
            // Batch 2: Transmission & Drivetrain Messages
            // ============================================================================
            ETC3::BASE_CAN_ID => self.handle_etc3(can_id, data),
            ETC4::BASE_CAN_ID => self.handle_etc4(can_id, data),
            ETC7::BASE_CAN_ID => self.handle_etc7(can_id, data),
            ETC8::BASE_CAN_ID => self.handle_etc8(can_id, data),
            ETC10::BASE_CAN_ID => self.handle_etc10(can_id, data),
            ETC11::BASE_CAN_ID => self.handle_etc11(can_id, data),
            ETC12::BASE_CAN_ID => self.handle_etc12(can_id, data),
            ETC13::BASE_CAN_ID => self.handle_etc13(can_id, data),
            ETC14::BASE_CAN_ID => self.handle_etc14(can_id, data),
            ETC15::BASE_CAN_ID => self.handle_etc15(can_id, data),
            ETCC4::BASE_CAN_ID => self.handle_etcc4(can_id, data),
            ETCBI::BASE_CAN_ID => self.handle_etcbi(can_id, data),
            TC1::BASE_CAN_ID => self.handle_tc1(can_id, data),
            TC2::BASE_CAN_ID => self.handle_tc2(can_id, data),

            // ============================================================================
            // Engine Exhaust and Transmission Control Messages (Commands)
            // ============================================================================
            EEC12::BASE_CAN_ID => self.handle_eec12(can_id, data),
            ETC5::BASE_CAN_ID => self.handle_etc5(can_id, data),
            EEC22::BASE_CAN_ID => self.handle_eec22(can_id, data),
            EEC21::BASE_CAN_ID => self.handle_eec21(can_id, data),
            ETCC2::BASE_CAN_ID => self.handle_etcc2(can_id, data),
            ETCC1::BASE_CAN_ID => self.handle_etcc1(can_id, data),
            EEC17::BASE_CAN_ID => self.handle_eec17(can_id, data),
            ETC6::BASE_CAN_ID => self.handle_etc6(can_id, data),
            ETC2::BASE_CAN_ID => self.handle_etc2(can_id, data),
            EEC8::BASE_CAN_ID => self.handle_eec8(can_id, data),
            EEC15::BASE_CAN_ID => self.handle_eec15(can_id, data),

            // ============================================================================
            // Braking Safety Messages (Commands)
            // ============================================================================
            EBC1::BASE_CAN_ID => self.handle_ebc1(can_id, data),
            EBC2::BASE_CAN_ID => self.handle_ebc2(can_id, data),
            EBC3::BASE_CAN_ID => self.handle_ebc3(can_id, data),
            EBC4::BASE_CAN_ID => self.handle_ebc4(can_id, data),
            EBC5::BASE_CAN_ID => self.handle_ebc5(can_id, data),
            EBC6::BASE_CAN_ID => self.handle_ebc6(can_id, data),
            EBC7::BASE_CAN_ID => self.handle_ebc7(can_id, data),
            EBCC::BASE_CAN_ID => self.handle_ebcc(can_id, data),
            XBR::BASE_CAN_ID => self.handle_xbr(can_id, data),
            AEBS1::BASE_CAN_ID => self.handle_aebs1(can_id, data),
            ACC1::BASE_CAN_ID => self.handle_acc1(can_id, data),
            ACC2::BASE_CAN_ID => self.handle_acc2(can_id, data),
            ACCS::BASE_CAN_ID => self.handle_accs(can_id, data),
            ACCVC::BASE_CAN_ID => self.handle_accvc(can_id, data),
            ERC1::BASE_CAN_ID => self.handle_erc1(can_id, data),
            ERC2::BASE_CAN_ID => self.handle_erc2(can_id, data),
            RC::BASE_CAN_ID => self.handle_rc(can_id, data),
            LMP::BASE_CAN_ID => self.handle_lmp(can_id, data),
            // Note: AEBS2 is typically a status message, but could process commands

            // ============================================================================
            // Original Messages (Commands)
            // ============================================================================
            CN::BASE_CAN_ID => self.handle_cn(can_id, data),
            GC2::BASE_CAN_ID => self.handle_gc2(can_id, data),
            DCACAI1S2::BASE_CAN_ID => self.handle_dcacai1s2(can_id, data),
            DCDC1OS::BASE_CAN_ID => self.handle_dcdc1os(can_id, data),
            DCDC1SBS::BASE_CAN_ID => self.handle_dcdc1sbs(can_id, data),
            DCDC1S2::BASE_CAN_ID => self.handle_dcdc1s2(can_id, data),
            DCACAI1V::BASE_CAN_ID => self.handle_dcacai1v(can_id, data),
            GTRACE::BASE_CAN_ID => self.handle_gtrace(can_id, data),
            DCDC2SBS::BASE_CAN_ID => self.handle_dcdc2sbs(can_id, data),
            DCDC2S2::BASE_CAN_ID => self.handle_dcdc2s2(can_id, data),
            // Batch 9: Extended DCDC Messages
            DCDC1HL::BASE_CAN_ID => self.handle_dcdc1hl(can_id, data),
            DCDC1LL::BASE_CAN_ID => self.handle_dcdc1ll(can_id, data),
            DCDC1T::BASE_CAN_ID => self.handle_dcdc1t(can_id, data),
            DCDC1V::BASE_CAN_ID => self.handle_dcdc1v(can_id, data),
            DCDC1VC::BASE_CAN_ID => self.handle_dcdc1vc(can_id, data),
            DCDC1LD::BASE_CAN_ID => self.handle_dcdc1ld(can_id, data),
            DCDC1SBL::BASE_CAN_ID => self.handle_dcdc1sbl(can_id, data),
            DCDC1CFG1::BASE_CAN_ID => self.handle_dcdc1cfg1(can_id, data),
            DCDC2C::BASE_CAN_ID => self.handle_dcdc2c(can_id, data),
            DCDC2OS::BASE_CAN_ID => self.handle_dcdc2os(can_id, data),
            DCDC2HL::BASE_CAN_ID => self.handle_dcdc2hl(can_id, data),
            DCDC2LL::BASE_CAN_ID => self.handle_dcdc2ll(can_id, data),
            DCDC2T::BASE_CAN_ID => self.handle_dcdc2t(can_id, data),
            DCDC2V::BASE_CAN_ID => self.handle_dcdc2v(can_id, data),
            DCDC2VC::BASE_CAN_ID => self.handle_dcdc2vc(can_id, data),
            DCDC2LD::BASE_CAN_ID => self.handle_dcdc2ld(can_id, data),
            DCDC2SBL::BASE_CAN_ID => self.handle_dcdc2sbl(can_id, data),
            DCDC2CFG1::BASE_CAN_ID => self.handle_dcdc2cfg1(can_id, data),
            DCDC3C::BASE_CAN_ID => self.handle_dcdc3c(can_id, data),
            DCDC3OS::BASE_CAN_ID => self.handle_dcdc3os(can_id, data),
            DCDC3S2::BASE_CAN_ID => self.handle_dcdc3s2(can_id, data),
            DCDC3SBS::BASE_CAN_ID => self.handle_dcdc3sbs(can_id, data),
            DCDC3T::BASE_CAN_ID => self.handle_dcdc3t(can_id, data),
            DCDC3V::BASE_CAN_ID => self.handle_dcdc3v(can_id, data),
            DCDC3VC::BASE_CAN_ID => self.handle_dcdc3vc(can_id, data),
            DCDC3SBL::BASE_CAN_ID => self.handle_dcdc3sbl(can_id, data),
            DCDC3LL::BASE_CAN_ID => self.handle_dcdc3ll(can_id, data),
            DCDC3HL::BASE_CAN_ID => self.handle_dcdc3hl(can_id, data),
            DCDC3LD::BASE_CAN_ID => self.handle_dcdc3ld(can_id, data),
            DCDC3CFG1::BASE_CAN_ID => self.handle_dcdc3cfg1(can_id, data),
            DCDC4C::BASE_CAN_ID => self.handle_dcdc4c(can_id, data),
            DCDC4OS::BASE_CAN_ID => self.handle_dcdc4os(can_id, data),
            DCDC4S2::BASE_CAN_ID => self.handle_dcdc4s2(can_id, data),
            DCDC4SBS::BASE_CAN_ID => self.handle_dcdc4sbs(can_id, data),
            DCDC4T::BASE_CAN_ID => self.handle_dcdc4t(can_id, data),
            DCDC4V::BASE_CAN_ID => self.handle_dcdc4v(can_id, data),
            DCDC4VC::BASE_CAN_ID => self.handle_dcdc4vc(can_id, data),
            DCDC4SBL::BASE_CAN_ID => self.handle_dcdc4sbl(can_id, data),
            DCDC4LL::BASE_CAN_ID => self.handle_dcdc4ll(can_id, data),
            DCDC4HL::BASE_CAN_ID => self.handle_dcdc4hl(can_id, data),
            DCDC4LD::BASE_CAN_ID => self.handle_dcdc4ld(can_id, data),
            DCDC4CFG1::BASE_CAN_ID => self.handle_dcdc4cfg1(can_id, data),
            // Batch 9: Power Supply Messages
            GC1::BASE_CAN_ID => self.handle_gc1(can_id, data),
            GTRACE2::BASE_CAN_ID => self.handle_gtrace2(can_id, data),
            GAAC::BASE_CAN_ID => self.handle_gaac(can_id, data),
            HVESSTS1::BASE_CAN_ID => self.handle_hvessts1(can_id, data),
            HVESSTC1::BASE_CAN_ID => self.handle_hvesstc1(can_id, data),
            HVESSTC2::BASE_CAN_ID => self.handle_hvesstc2(can_id, data),
            ETCC3::BASE_CAN_ID => self.handle_etcc3(can_id, data),

            // ============================================================================
            // Batch 7: Extended HVESS Messages (35 new handlers)
            // ============================================================================
            HVESSD4::BASE_CAN_ID => self.handle_hvessd4(can_id, data),
            HVESSD5::BASE_CAN_ID => self.handle_hvessd5(can_id, data),
            HVESSD7::BASE_CAN_ID => self.handle_hvessd7(can_id, data),
            HVESSD8::BASE_CAN_ID => self.handle_hvessd8(can_id, data),
            HVESSD9::BASE_CAN_ID => self.handle_hvessd9(can_id, data),
            HVESSD10::BASE_CAN_ID => self.handle_hvessd10(can_id, data),
            HVESSD11::BASE_CAN_ID => self.handle_hvessd11(can_id, data),
            HVESSD12::BASE_CAN_ID => self.handle_hvessd12(can_id, data),
            HVESSD13::BASE_CAN_ID => self.handle_hvessd13(can_id, data),
            HVESSD14::BASE_CAN_ID => self.handle_hvessd14(can_id, data),
            HVESSD15::BASE_CAN_ID => self.handle_hvessd15(can_id, data),
            HVESSIS1::BASE_CAN_ID => self.handle_hvessis1(can_id, data),
            HVESSIS2::BASE_CAN_ID => self.handle_hvessis2(can_id, data),
            HVESSIS3::BASE_CAN_ID => self.handle_hvessis3(can_id, data),
            HVESSIS4::BASE_CAN_ID => self.handle_hvessis4(can_id, data),
            HVESSIS5::BASE_CAN_ID => self.handle_hvessis5(can_id, data),
            HVESSIS6::BASE_CAN_ID => self.handle_hvessis6(can_id, data),
            HVESSIS7::BASE_CAN_ID => self.handle_hvessis7(can_id, data),
            HVESSMS1::BASE_CAN_ID => self.handle_hvessms1(can_id, data),
            HVESSMS2::BASE_CAN_ID => self.handle_hvessms2(can_id, data),
            HVESSMS3::BASE_CAN_ID => self.handle_hvessms3(can_id, data),
            HVESSS1::BASE_CAN_ID => self.handle_hvesss1(can_id, data),
            HVESSS2::BASE_CAN_ID => self.handle_hvesss2(can_id, data),
            HVESSFS2::BASE_CAN_ID => self.handle_hvessfs2(can_id, data),
            HVESSFC::BASE_CAN_ID => self.handle_hvessfc(can_id, data),
            HVESSCFG::BASE_CAN_ID => self.handle_hvesscfg(can_id, data),
            HVESSCP1C::BASE_CAN_ID => self.handle_hvesscp1c(can_id, data),
            HVESSCP1S1::BASE_CAN_ID => self.handle_hvesscp1s1(can_id, data),
            HVESSCP1S2::BASE_CAN_ID => self.handle_hvesscp1s2(can_id, data),
            HVESSCP2C::BASE_CAN_ID => self.handle_hvesscp2c(can_id, data),
            HVESSCP2S1::BASE_CAN_ID => self.handle_hvesscp2s1(can_id, data),
            HVESSCP2S2::BASE_CAN_ID => self.handle_hvesscp2s2(can_id, data),
            HVESSTCH1::BASE_CAN_ID => self.handle_hvesstch1(can_id, data),
            HVESSTCH2::BASE_CAN_ID => self.handle_hvesstch2(can_id, data),
            HVESSTCH3::BASE_CAN_ID => self.handle_hvesstch3(can_id, data),
            HVESSHIST::BASE_CAN_ID => self.handle_hvesshist(can_id, data),

            DM01::BASE_CAN_ID => self.handle_dm01(can_id, data),
            DM02::BASE_CAN_ID => self.handle_dm02(can_id, data),
            DM03::BASE_CAN_ID => self.handle_dm03(can_id, data),

            // ============================================================================
            // Batch 10: Extended Diagnostics (DM04-DM35)
            // ============================================================================
            DM04::BASE_CAN_ID => self.handle_dm04(can_id, data),
            DM05::BASE_CAN_ID => self.handle_dm05(can_id, data),
            DM06::BASE_CAN_ID => self.handle_dm06(can_id, data),
            DM07::BASE_CAN_ID => self.handle_dm07(can_id, data),
            DM10::BASE_CAN_ID => self.handle_dm10(can_id, data),
            DM11::BASE_CAN_ID => self.handle_dm11(can_id, data),
            DM12::BASE_CAN_ID => self.handle_dm12(can_id, data),
            DM13::BASE_CAN_ID => self.handle_dm13(can_id, data),
            DM19::BASE_CAN_ID => self.handle_dm19(can_id, data),
            DM20::BASE_CAN_ID => self.handle_dm20(can_id, data),
            DM21::BASE_CAN_ID => self.handle_dm21(can_id, data),
            DM25::BASE_CAN_ID => self.handle_dm25(can_id, data),
            DM27::BASE_CAN_ID => self.handle_dm27(can_id, data),
            DM28::BASE_CAN_ID => self.handle_dm28(can_id, data),
            DM29::BASE_CAN_ID => self.handle_dm29(can_id, data),
            DM31::BASE_CAN_ID => self.handle_dm31(can_id, data),
            DM33::BASE_CAN_ID => self.handle_dm33(can_id, data),
            DM34::BASE_CAN_ID => self.handle_dm34(can_id, data),
            DM35::BASE_CAN_ID => self.handle_dm35(can_id, data),

            // ============================================================================
            // Batch 3: Engine Temps, Fluids & Sensors Handlers
            // ============================================================================
            ET1::BASE_CAN_ID => self.handle_et1(can_id, data),
            ET2::BASE_CAN_ID => self.handle_et2(can_id, data),
            ET3::BASE_CAN_ID => self.handle_et3(can_id, data),
            ET4::BASE_CAN_ID => self.handle_et4(can_id, data),
            ET5::BASE_CAN_ID => self.handle_et5(can_id, data),
            ET6::BASE_CAN_ID => self.handle_et6(can_id, data),
            LFE1::BASE_CAN_ID => self.handle_lfe1(can_id, data),
            LFE2::BASE_CAN_ID => self.handle_lfe2(can_id, data),
            IC1::BASE_CAN_ID => self.handle_ic1(can_id, data),
            IC2::BASE_CAN_ID => self.handle_ic2(can_id, data),
            IC3::BASE_CAN_ID => self.handle_ic3(can_id, data),
            AMB::BASE_CAN_ID => self.handle_amb(can_id, data),
            AMB2::BASE_CAN_ID => self.handle_amb2(can_id, data),
            AMB3::BASE_CAN_ID => self.handle_amb3(can_id, data),
            AMB4::BASE_CAN_ID => self.handle_amb4(can_id, data),
            FD2::BASE_CAN_ID => self.handle_fd2(can_id, data),
            DD2::BASE_CAN_ID => self.handle_dd2(can_id, data),
            DD3::BASE_CAN_ID => self.handle_dd3(can_id, data),
            HOURS::BASE_CAN_ID => self.handle_hours(can_id, data),
            HOURS2::BASE_CAN_ID => self.handle_hours2(can_id, data),
            IO::BASE_CAN_ID => self.handle_io(can_id, data),
            FL::BASE_CAN_ID => self.handle_fl(can_id, data),
            LFC1::BASE_CAN_ID => self.handle_lfc1(can_id, data),

            // ============================================================================
            // Batch 4: Vehicle Speed, Distance & Wheels Handlers
            // ============================================================================
            CCVS1::BASE_CAN_ID => self.handle_ccvs1(can_id, data),
            CCVS2::BASE_CAN_ID => self.handle_ccvs2(can_id, data),
            CCVS3::BASE_CAN_ID => self.handle_ccvs3(can_id, data),
            CCVS4::BASE_CAN_ID => self.handle_ccvs4(can_id, data),
            CCVS5::BASE_CAN_ID => self.handle_ccvs5(can_id, data),
            CCVS6::BASE_CAN_ID => self.handle_ccvs6(can_id, data),
            VD::BASE_CAN_ID => self.handle_vd(can_id, data),
            VDS::BASE_CAN_ID => self.handle_vds(can_id, data),
            VDS2::BASE_CAN_ID => self.handle_vds2(can_id, data),
            HRW::BASE_CAN_ID => self.handle_hrw(can_id, data),
            VW::BASE_CAN_ID => self.handle_vw(can_id, data),
            TIRE1::BASE_CAN_ID => self.handle_tire1(can_id, data),
            TIRE2::BASE_CAN_ID => self.handle_tire2(can_id, data),
            SSI::BASE_CAN_ID => self.handle_ssi(can_id, data),
            VEP1::BASE_CAN_ID => self.handle_vep1(can_id, data),
            VEP2::BASE_CAN_ID => self.handle_vep2(can_id, data),
            VEP3::BASE_CAN_ID => self.handle_vep3(can_id, data),
            AS1::BASE_CAN_ID => self.handle_as1(can_id, data),
            AS2::BASE_CAN_ID => self.handle_as2(can_id, data),
            EP::BASE_CAN_ID => self.handle_ep(can_id, data),
            TD::BASE_CAN_ID => self.handle_td(can_id, data),
            OEL::BASE_CAN_ID => self.handle_oel(can_id, data),
            SHUTDN::BASE_CAN_ID => self.handle_shutdn(can_id, data),
            BSA::BASE_CAN_ID => self.handle_bsa(can_id, data),
            GFI1::BASE_CAN_ID => self.handle_gfi1(can_id, data),

            // ============================================================================
            // Aftertreatment Messages (Batch 6 - Bank 1)
            // ============================================================================
            AT1S1::BASE_CAN_ID => self.handle_at1s1(can_id, data),
            AT1S2::BASE_CAN_ID => self.handle_at1s2(can_id, data),
            AT1T1I1::BASE_CAN_ID => self.handle_at1t1i1(can_id, data),
            AT1T1I2::BASE_CAN_ID => self.handle_at1t1i2(can_id, data),
            AT1TI::BASE_CAN_ID => self.handle_at1ti(can_id, data),
            AT1OG1::BASE_CAN_ID => self.handle_at1og1(can_id, data),
            AT1OG2::BASE_CAN_ID => self.handle_at1og2(can_id, data),
            AT1IG1::BASE_CAN_ID => self.handle_at1ig1(can_id, data),
            AT1IG2::BASE_CAN_ID => self.handle_at1ig2(can_id, data),
            AT1HI1::BASE_CAN_ID => self.handle_at1hi1(can_id, data),
            AT1GP::BASE_CAN_ID => self.handle_at1gp(can_id, data),
            AT1FC1::BASE_CAN_ID => self.handle_at1fc1(can_id, data),
            AT1FC2::BASE_CAN_ID => self.handle_at1fc2(can_id, data),
            AT1AC1::BASE_CAN_ID => self.handle_at1ac1(can_id, data),
            A1DOC1::BASE_CAN_ID => self.handle_a1doc1(can_id, data),
            A1DOC2::BASE_CAN_ID => self.handle_a1doc2(can_id, data),
            A1SCRAI::BASE_CAN_ID => self.handle_a1scrai(can_id, data),
            A1SCRSI1::BASE_CAN_ID => self.handle_a1scrsi1(can_id, data),
            A1SCRSI2::BASE_CAN_ID => self.handle_a1scrsi2(can_id, data),
            DPF1S::BASE_CAN_ID => self.handle_dpf1s(can_id, data),
            DPF1S2::BASE_CAN_ID => self.handle_dpf1s2(can_id, data),
            DPFC1::BASE_CAN_ID => self.handle_dpfc1(can_id, data),
            DPFC2::BASE_CAN_ID => self.handle_dpfc2(can_id, data),

            // ============================================================================
            // Aftertreatment Messages (Batch 11 - Bank 2 + EGR)
            // ============================================================================
            AT2S1::BASE_CAN_ID => self.handle_at2s1(can_id, data),
            AT2S2::BASE_CAN_ID => self.handle_at2s2(can_id, data),
            AT2OG1::BASE_CAN_ID => self.handle_at2og1(can_id, data),
            AT2IG1::BASE_CAN_ID => self.handle_at2ig1(can_id, data),
            AT2HI1::BASE_CAN_ID => self.handle_at2hi1(can_id, data),
            AT2GP::BASE_CAN_ID => self.handle_at2gp(can_id, data),
            AT2FC1::BASE_CAN_ID => self.handle_at2fc1(can_id, data),
            AT2AC1::BASE_CAN_ID => self.handle_at2ac1(can_id, data),
            A2DOC1::BASE_CAN_ID => self.handle_a2doc1(can_id, data),
            A2SCRAI::BASE_CAN_ID => self.handle_a2scrai(can_id, data),
            A2SCRSI1::BASE_CAN_ID => self.handle_a2scrsi1(can_id, data),
            A1SCRDSI1::BASE_CAN_ID => self.handle_a1scrdsi1(can_id, data),
            A1SCRDSI2::BASE_CAN_ID => self.handle_a1scrdsi2(can_id, data),
            A1SCRDSI3::BASE_CAN_ID => self.handle_a1scrdsi3(can_id, data),
            A2SCRDSI1::BASE_CAN_ID => self.handle_a2scrdsi1(can_id, data),
            A2SCRDSI2::BASE_CAN_ID => self.handle_a2scrdsi2(can_id, data),
            A2SCRDSI3::BASE_CAN_ID => self.handle_a2scrdsi3(can_id, data),
            EEGR1A::BASE_CAN_ID => self.handle_eegr1a(can_id, data),
            EEGR2A::BASE_CAN_ID => self.handle_eegr2a(can_id, data),
            DPF2S::BASE_CAN_ID => self.handle_dpf2s(can_id, data),

            // ============================================================================
            // EV Charging & HV Bus Messages (Batch 12)
            // ============================================================================
            EVDCS1::BASE_CAN_ID => self.handle_evdcs1(can_id, data),
            EVDCTGT::BASE_CAN_ID => self.handle_evdctgt(can_id, data),
            EVDCLIM1::BASE_CAN_ID => self.handle_evdclim1(can_id, data),
            EVDCLIM2::BASE_CAN_ID => self.handle_evdclim2(can_id, data),
            EVDCCIP::BASE_CAN_ID => self.handle_evdccip(can_id, data),
            EVSE1CS1::BASE_CAN_ID => self.handle_evse1cs1(can_id, data),
            EVSE1CC1::BASE_CAN_ID => self.handle_evse1cc1(can_id, data),
            EVSEC1::BASE_CAN_ID => self.handle_evsec1(can_id, data),
            EVSEDCS1::BASE_CAN_ID => self.handle_evsedcs1(can_id, data),
            EVSES1::BASE_CAN_ID => self.handle_evses1(can_id, data),
            EVSES2::BASE_CAN_ID => self.handle_evses2(can_id, data),
            EVC::BASE_CAN_ID => self.handle_evc(can_id, data),
            EVEI::BASE_CAN_ID => self.handle_evei(can_id, data),
            EVOI1::BASE_CAN_ID => self.handle_evoi1(can_id, data),
            HVBCS1::BASE_CAN_ID => self.handle_hvbcs1(can_id, data),
            HVBCS2::BASE_CAN_ID => self.handle_hvbcs2(can_id, data),
            HVBCS3::BASE_CAN_ID => self.handle_hvbcs3(can_id, data),
            HVBCC1::BASE_CAN_ID => self.handle_hvbcc1(can_id, data),
            HVBCC2::BASE_CAN_ID => self.handle_hvbcc2(can_id, data),
            HVBI::BASE_CAN_ID => self.handle_hvbi(can_id, data),
            EVVT::BASE_CAN_ID => self.handle_evvt(can_id, data),

            _ => {
                // Unknown message - log for debugging
                println!(
                    "❓ Unknown message: ID = 0x{:08X}, Base = 0x{:08X}",
                    can_id, base_id
                );
                Ok(MessageStatus::Unrecognized)
            }
        };

        // Mark message as processed if it was successfully recognized
        if let Ok(MessageStatus::Recognized) = result
            && let Some(last_msg) = self.recent_messages.back_mut()
        {
            last_msg.processed = true;
        }

        result
    }
}
