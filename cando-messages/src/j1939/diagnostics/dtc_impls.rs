//! DtcMessage trait implementations for DM01, DM02, DM06, DM12, and DM23

use super::super::{DM01, DM02, DM06, DM12, DM23};
use super::dtc_helpers::DtcMessage;
use crate::common::DeviceId;

// ============================================================================
// DM01 - Active Diagnostic Trouble Codes
// ============================================================================

impl DtcMessage for DM01 {
    fn device_id(&self) -> DeviceId {
        self.device_id
    }

    fn lamp_fields(&self) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        (
            self.malfunction_indicator_lamp_status,
            self.red_stop_lamp_status,
            self.amber_warning_lamp_status,
            self.protect_lamp_status,
            self.flash_malfunc_indicator_lamp,
            self.flash_red_stop_lamp,
            self.flash_amber_warning_lamp,
            self.flash_protect_lamp,
        )
    }

    fn dtc_slot(&self, slot: usize) -> Option<(u16, u8, f64, u8, u8)> {
        match slot {
            1 => Some((
                self.dm01_01spn,
                self.dm01_01fmi,
                self.dm01_01spn_high,
                self.dm01_01oc,
                self.dm01_01cm,
            )),
            2 => Some((
                self.dm01_02spn,
                self.dm01_02fmi,
                self.dm01_02spn_high,
                self.dm01_02oc,
                self.dm01_02cm,
            )),
            3 => Some((
                self.dm01_03spn,
                self.dm01_03fmi,
                self.dm01_03spn_high,
                self.dm01_03oc,
                self.dm01_03cm,
            )),
            4 => Some((
                self.dm01_04spn,
                self.dm01_04fmi,
                self.dm01_04spn_high,
                self.dm01_04oc,
                self.dm01_04cm,
            )),
            5 => Some((
                self.dm01_05spn,
                self.dm01_05fmi,
                self.dm01_05spn_high,
                self.dm01_05oc,
                self.dm01_05cm,
            )),
            6 => Some((
                self.dm01_06spn,
                self.dm01_06fmi,
                self.dm01_06spn_high,
                self.dm01_06oc,
                self.dm01_06cm,
            )),
            7 => Some((
                self.dm01_07spn,
                self.dm01_07fmi,
                self.dm01_07spn_high,
                self.dm01_07oc,
                self.dm01_07cm,
            )),
            8 => Some((
                self.dm01_08spn,
                self.dm01_08fmi,
                self.dm01_08spn_high,
                self.dm01_08oc,
                self.dm01_08cm,
            )),
            9 => Some((
                self.dm01_09spn,
                self.dm01_09fmi,
                self.dm01_09spn_high,
                self.dm01_09oc,
                self.dm01_09cm,
            )),
            10 => Some((
                self.dm01_10spn,
                self.dm01_10fmi,
                self.dm01_10spn_high,
                self.dm01_10oc,
                self.dm01_10cm,
            )),
            11 => Some((
                self.dm01_11spn,
                self.dm01_11fmi,
                self.dm01_11spn_high,
                self.dm01_11oc,
                self.dm01_11cm,
            )),
            12 => Some((
                self.dm01_12spn,
                self.dm01_12fmi,
                self.dm01_12spn_high,
                self.dm01_12oc,
                self.dm01_12cm,
            )),
            13 => Some((
                self.dm01_13spn,
                self.dm01_13fmi,
                self.dm01_13spn_high,
                self.dm01_13oc,
                self.dm01_13cm,
            )),
            14 => Some((
                self.dm01_14spn,
                self.dm01_14fmi,
                self.dm01_14spn_high,
                self.dm01_14oc,
                self.dm01_14cm,
            )),
            15 => Some((
                self.dm01_15spn,
                self.dm01_15fmi,
                self.dm01_15spn_high,
                self.dm01_15oc,
                self.dm01_15cm,
            )),
            16 => Some((
                self.dm01_16spn,
                self.dm01_16fmi,
                self.dm01_16spn_high,
                self.dm01_16oc,
                self.dm01_16cm,
            )),
            17 => Some((
                self.dm01_17spn,
                self.dm01_17fmi,
                self.dm01_17spn_high,
                self.dm01_17oc,
                self.dm01_17cm,
            )),
            18 => Some((
                self.dm01_18spn,
                self.dm01_18fmi,
                self.dm01_18spn_high,
                self.dm01_18oc,
                self.dm01_18cm,
            )),
            19 => Some((
                self.dm01_19spn,
                self.dm01_19fmi,
                self.dm01_19spn_high,
                self.dm01_19oc,
                self.dm01_19cm,
            )),
            _ => None,
        }
    }
}

// ============================================================================
// DM02 - Previously Active Diagnostic Trouble Codes
// ============================================================================

impl DtcMessage for DM02 {
    fn device_id(&self) -> DeviceId {
        self.device_id
    }

    fn lamp_fields(&self) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        (
            self.malfunction_indicator_lamp_status,
            self.red_stop_lamp_status,
            self.amber_warning_lamp_status,
            self.protect_lamp_status,
            self.flash_malfunc_indicator_lamp,
            self.flash_red_stop_lamp,
            self.flash_amber_warning_lamp,
            self.flash_protect_lamp,
        )
    }

    fn dtc_slot(&self, slot: usize) -> Option<(u16, u8, f64, u8, u8)> {
        match slot {
            1 => Some((
                self.dm02_01spn,
                self.dm02_01fmi,
                self.dm02_01spn_high,
                self.dm02_01oc,
                self.dm02_01cm,
            )),
            2 => Some((
                self.dm02_02spn,
                self.dm02_02fmi,
                self.dm02_02spn_high,
                self.dm02_02oc,
                self.dm02_02cm,
            )),
            3 => Some((
                self.dm02_03spn,
                self.dm02_03fmi,
                self.dm02_03spn_high,
                self.dm02_03oc,
                self.dm02_03cm,
            )),
            4 => Some((
                self.dm02_04spn,
                self.dm02_04fmi,
                self.dm02_04spn_high,
                self.dm02_04oc,
                self.dm02_04cm,
            )),
            5 => Some((
                self.dm02_05spn,
                self.dm02_05fmi,
                self.dm02_05spn_high,
                self.dm02_05oc,
                self.dm02_05cm,
            )),
            6 => Some((
                self.dm02_06spn,
                self.dm02_06fmi,
                self.dm02_06spn_high,
                self.dm02_06oc,
                self.dm02_06cm,
            )),
            7 => Some((
                self.dm02_07spn,
                self.dm02_07fmi,
                self.dm02_07spn_high,
                self.dm02_07oc,
                self.dm02_07cm,
            )),
            8 => Some((
                self.dm02_08spn,
                self.dm02_08fmi,
                self.dm02_08spn_high,
                self.dm02_08oc,
                self.dm02_08cm,
            )),
            9 => Some((
                self.dm02_09spn,
                self.dm02_09fmi,
                self.dm02_09spn_high,
                self.dm02_09oc,
                self.dm02_09cm,
            )),
            10 => Some((
                self.dm02_10spn,
                self.dm02_10fmi,
                self.dm02_10spn_high,
                self.dm02_10oc,
                self.dm02_10cm,
            )),
            11 => Some((
                self.dm02_11spn,
                self.dm02_11fmi,
                self.dm02_11spn_high,
                self.dm02_11oc,
                self.dm02_11cm,
            )),
            12 => Some((
                self.dm02_12spn,
                self.dm02_12fmi,
                self.dm02_12spn_high,
                self.dm02_12oc,
                self.dm02_12cm,
            )),
            13 => Some((
                self.dm02_13spn,
                self.dm02_13fmi,
                self.dm02_13spn_high,
                self.dm02_13oc,
                self.dm02_13cm,
            )),
            14 => Some((
                self.dm02_14spn,
                self.dm02_14fmi,
                self.dm02_14spn_high,
                self.dm02_14oc,
                self.dm02_14cm,
            )),
            15 => Some((
                self.dm02_15spn,
                self.dm02_15fmi,
                self.dm02_15spn_high,
                self.dm02_15oc,
                self.dm02_15cm,
            )),
            16 => Some((
                self.dm02_16spn,
                self.dm02_16fmi,
                self.dm02_16spn_high,
                self.dm02_16oc,
                self.dm02_16cm,
            )),
            17 => Some((
                self.dm02_17spn,
                self.dm02_17fmi,
                self.dm02_17spn_high,
                self.dm02_17oc,
                self.dm02_17cm,
            )),
            18 => Some((
                self.dm02_18spn,
                self.dm02_18fmi,
                self.dm02_18spn_high,
                self.dm02_18oc,
                self.dm02_18cm,
            )),
            19 => Some((
                self.dm02_19spn,
                self.dm02_19fmi,
                self.dm02_19spn_high,
                self.dm02_19oc,
                self.dm02_19cm,
            )),
            _ => None,
        }
    }
}

// ============================================================================
// DM06 - Emission-Related Pending Diagnostic Trouble Codes
// ============================================================================

impl DtcMessage for DM06 {
    fn device_id(&self) -> DeviceId {
        self.device_id
    }

    fn lamp_fields(&self) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        (
            self.malfunction_indicator_lamp_status,
            self.red_stop_lamp_status,
            self.amber_warning_lamp_status,
            self.protect_lamp_status,
            self.flash_malfunc_indicator_lamp,
            self.flash_red_stop_lamp,
            self.flash_amber_warning_lamp,
            self.flash_protect_lamp,
        )
    }

    fn dtc_slot(&self, slot: usize) -> Option<(u16, u8, f64, u8, u8)> {
        match slot {
            1 => Some((
                self.dm06_01spn,
                self.dm06_01fmi,
                self.dm06_01spn_high,
                self.dm06_01oc,
                self.dm06_01cm,
            )),
            2 => Some((
                self.dm06_02spn,
                self.dm06_02fmi,
                self.dm06_02spn_high,
                self.dm06_02oc,
                self.dm06_02cm,
            )),
            3 => Some((
                self.dm06_03spn,
                self.dm06_03fmi,
                self.dm06_03spn_high,
                self.dm06_03oc,
                self.dm06_03cm,
            )),
            4 => Some((
                self.dm06_04spn,
                self.dm06_04fmi,
                self.dm06_04spn_high,
                self.dm06_04oc,
                self.dm06_04cm,
            )),
            5 => Some((
                self.dm06_05spn,
                self.dm06_05fmi,
                self.dm06_05spn_high,
                self.dm06_05oc,
                self.dm06_05cm,
            )),
            6 => Some((
                self.dm06_06spn,
                self.dm06_06fmi,
                self.dm06_06spn_high,
                self.dm06_06oc,
                self.dm06_06cm,
            )),
            7 => Some((
                self.dm06_07spn,
                self.dm06_07fmi,
                self.dm06_07spn_high,
                self.dm06_07oc,
                self.dm06_07cm,
            )),
            8 => Some((
                self.dm06_08spn,
                self.dm06_08fmi,
                self.dm06_08spn_high,
                self.dm06_08oc,
                self.dm06_08cm,
            )),
            9 => Some((
                self.dm06_09spn,
                self.dm06_09fmi,
                self.dm06_09spn_high,
                self.dm06_09oc,
                self.dm06_09cm,
            )),
            10 => Some((
                self.dm06_10spn,
                self.dm06_10fmi,
                self.dm06_10spn_high,
                self.dm06_10oc,
                self.dm06_10cm,
            )),
            11 => Some((
                self.dm06_11spn,
                self.dm06_11fmi,
                self.dm06_11spn_high,
                self.dm06_11oc,
                self.dm06_11cm,
            )),
            12 => Some((
                self.dm06_12spn,
                self.dm06_12fmi,
                self.dm06_12spn_high,
                self.dm06_12oc,
                self.dm06_12cm,
            )),
            13 => Some((
                self.dm06_13spn,
                self.dm06_13fmi,
                self.dm06_13spn_high,
                self.dm06_13oc,
                self.dm06_13cm,
            )),
            14 => Some((
                self.dm06_14spn,
                self.dm06_14fmi,
                self.dm06_14spn_high,
                self.dm06_14oc,
                self.dm06_14cm,
            )),
            15 => Some((
                self.dm06_15spn,
                self.dm06_15fmi,
                self.dm06_15spn_high,
                self.dm06_15oc,
                self.dm06_15cm,
            )),
            16 => Some((
                self.dm06_16spn,
                self.dm06_16fmi,
                self.dm06_16spn_high,
                self.dm06_16oc,
                self.dm06_16cm,
            )),
            17 => Some((
                self.dm06_17spn,
                self.dm06_17fmi,
                self.dm06_17spn_high,
                self.dm06_17oc,
                self.dm06_17cm,
            )),
            18 => Some((
                self.dm06_18spn,
                self.dm06_18fmi,
                self.dm06_18spn_high,
                self.dm06_18oc,
                self.dm06_18cm,
            )),
            19 => Some((
                self.dm06_19spn,
                self.dm06_19fmi,
                self.dm06_19spn_high,
                self.dm06_19oc,
                self.dm06_19cm,
            )),
            _ => None,
        }
    }
}

// ============================================================================
// DM12 - Emission-Related MIL-On Diagnostic Trouble Codes
// ============================================================================

impl DtcMessage for DM12 {
    fn device_id(&self) -> DeviceId {
        self.device_id
    }

    fn lamp_fields(&self) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        (
            self.malfunction_indicator_lamp_status,
            self.red_stop_lamp_status,
            self.amber_warning_lamp_status,
            self.protect_lamp_status,
            self.flash_malfunc_indicator_lamp,
            self.flash_red_stop_lamp,
            self.flash_amber_warning_lamp,
            self.flash_protect_lamp,
        )
    }

    fn dtc_slot(&self, slot: usize) -> Option<(u16, u8, f64, u8, u8)> {
        match slot {
            1 => Some((
                self.dm12_01spn,
                self.dm12_01fmi,
                self.dm12_01spn_high,
                self.dm12_01oc,
                self.dm12_01cm,
            )),
            2 => Some((
                self.dm12_02spn,
                self.dm12_02fmi,
                self.dm12_02spn_high,
                self.dm12_02oc,
                self.dm12_02cm,
            )),
            3 => Some((
                self.dm12_03spn,
                self.dm12_03fmi,
                self.dm12_03spn_high,
                self.dm12_03oc,
                self.dm12_03cm,
            )),
            4 => Some((
                self.dm12_04spn,
                self.dm12_04fmi,
                self.dm12_04spn_high,
                self.dm12_04oc,
                self.dm12_04cm,
            )),
            5 => Some((
                self.dm12_05spn,
                self.dm12_05fmi,
                self.dm12_05spn_high,
                self.dm12_05oc,
                self.dm12_05cm,
            )),
            6 => Some((
                self.dm12_06spn,
                self.dm12_06fmi,
                self.dm12_06spn_high,
                self.dm12_06oc,
                self.dm12_06cm,
            )),
            7 => Some((
                self.dm12_07spn,
                self.dm12_07fmi,
                self.dm12_07spn_high,
                self.dm12_07oc,
                self.dm12_07cm,
            )),
            8 => Some((
                self.dm12_08spn,
                self.dm12_08fmi,
                self.dm12_08spn_high,
                self.dm12_08oc,
                self.dm12_08cm,
            )),
            9 => Some((
                self.dm12_09spn,
                self.dm12_09fmi,
                self.dm12_09spn_high,
                self.dm12_09oc,
                self.dm12_09cm,
            )),
            10 => Some((
                self.dm12_10spn,
                self.dm12_10fmi,
                self.dm12_10spn_high,
                self.dm12_10oc,
                self.dm12_10cm,
            )),
            11 => Some((
                self.dm12_11spn,
                self.dm12_11fmi,
                self.dm12_11spn_high,
                self.dm12_11oc,
                self.dm12_11cm,
            )),
            12 => Some((
                self.dm12_12spn,
                self.dm12_12fmi,
                self.dm12_12spn_high,
                self.dm12_12oc,
                self.dm12_12cm,
            )),
            13 => Some((
                self.dm12_13spn,
                self.dm12_13fmi,
                self.dm12_13spn_high,
                self.dm12_13oc,
                self.dm12_13cm,
            )),
            14 => Some((
                self.dm12_14spn,
                self.dm12_14fmi,
                self.dm12_14spn_high,
                self.dm12_14oc,
                self.dm12_14cm,
            )),
            15 => Some((
                self.dm12_15spn,
                self.dm12_15fmi,
                self.dm12_15spn_high,
                self.dm12_15oc,
                self.dm12_15cm,
            )),
            16 => Some((
                self.dm12_16spn,
                self.dm12_16fmi,
                self.dm12_16spn_high,
                self.dm12_16oc,
                self.dm12_16cm,
            )),
            17 => Some((
                self.dm12_17spn,
                self.dm12_17fmi,
                self.dm12_17spn_high,
                self.dm12_17oc,
                self.dm12_17cm,
            )),
            18 => Some((
                self.dm12_18spn,
                self.dm12_18fmi,
                self.dm12_18spn_high,
                self.dm12_18oc,
                self.dm12_18cm,
            )),
            19 => Some((
                self.dm12_19spn,
                self.dm12_19fmi,
                self.dm12_19spn_high,
                self.dm12_19oc,
                self.dm12_19cm,
            )),
            _ => None,
        }
    }
}

// ============================================================================
// DM23 - Emission-Related Previously MIL-On Diagnostic Trouble Codes
// ============================================================================

impl DtcMessage for DM23 {
    fn device_id(&self) -> DeviceId {
        self.device_id
    }

    fn lamp_fields(&self) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        (
            self.malfunction_indicator_lamp_status,
            self.red_stop_lamp_status,
            self.amber_warning_lamp_status,
            self.protect_lamp_status,
            self.flash_malfunc_indicator_lamp,
            self.flash_red_stop_lamp,
            self.flash_amber_warning_lamp,
            self.flash_protect_lamp,
        )
    }

    fn dtc_slot(&self, slot: usize) -> Option<(u16, u8, f64, u8, u8)> {
        match slot {
            1 => Some((
                self.dm23_01spn,
                self.dm23_01fmi,
                self.dm23_01spn_high,
                self.dm23_01oc,
                self.dm23_01cm,
            )),
            2 => Some((
                self.dm23_02spn,
                self.dm23_02fmi,
                self.dm23_02spn_high,
                self.dm23_02oc,
                self.dm23_02cm,
            )),
            3 => Some((
                self.dm23_03spn,
                self.dm23_03fmi,
                self.dm23_03spn_high,
                self.dm23_03oc,
                self.dm23_03cm,
            )),
            4 => Some((
                self.dm23_04spn,
                self.dm23_04fmi,
                self.dm23_04spn_high,
                self.dm23_04oc,
                self.dm23_04cm,
            )),
            5 => Some((
                self.dm23_05spn,
                self.dm23_05fmi,
                self.dm23_05spn_high,
                self.dm23_05oc,
                self.dm23_05cm,
            )),
            6 => Some((
                self.dm23_06spn,
                self.dm23_06fmi,
                self.dm23_06spn_high,
                self.dm23_06oc,
                self.dm23_06cm,
            )),
            7 => Some((
                self.dm23_07spn,
                self.dm23_07fmi,
                self.dm23_07spn_high,
                self.dm23_07oc,
                self.dm23_07cm,
            )),
            8 => Some((
                self.dm23_08spn,
                self.dm23_08fmi,
                self.dm23_08spn_high,
                self.dm23_08oc,
                self.dm23_08cm,
            )),
            9 => Some((
                self.dm23_09spn,
                self.dm23_09fmi,
                self.dm23_09spn_high,
                self.dm23_09oc,
                self.dm23_09cm,
            )),
            10 => Some((
                self.dm23_10spn,
                self.dm23_10fmi,
                self.dm23_10spn_high,
                self.dm23_10oc,
                self.dm23_10cm,
            )),
            11 => Some((
                self.dm23_11spn,
                self.dm23_11fmi,
                self.dm23_11spn_high,
                self.dm23_11oc,
                self.dm23_11cm,
            )),
            12 => Some((
                self.dm23_12spn,
                self.dm23_12fmi,
                self.dm23_12spn_high,
                self.dm23_12oc,
                self.dm23_12cm,
            )),
            13 => Some((
                self.dm23_13spn,
                self.dm23_13fmi,
                self.dm23_13spn_high,
                self.dm23_13oc,
                self.dm23_13cm,
            )),
            14 => Some((
                self.dm23_14spn,
                self.dm23_14fmi,
                self.dm23_14spn_high,
                self.dm23_14oc,
                self.dm23_14cm,
            )),
            15 => Some((
                self.dm23_15spn,
                self.dm23_15fmi,
                self.dm23_15spn_high,
                self.dm23_15oc,
                self.dm23_15cm,
            )),
            16 => Some((
                self.dm23_16spn,
                self.dm23_16fmi,
                self.dm23_16spn_high,
                self.dm23_16oc,
                self.dm23_16cm,
            )),
            17 => Some((
                self.dm23_17spn,
                self.dm23_17fmi,
                self.dm23_17spn_high,
                self.dm23_17oc,
                self.dm23_17cm,
            )),
            18 => Some((
                self.dm23_18spn,
                self.dm23_18fmi,
                self.dm23_18spn_high,
                self.dm23_18oc,
                self.dm23_18cm,
            )),
            19 => Some((
                self.dm23_19spn,
                self.dm23_19fmi,
                self.dm23_19spn_high,
                self.dm23_19oc,
                self.dm23_19cm,
            )),
            _ => None,
        }
    }
}
