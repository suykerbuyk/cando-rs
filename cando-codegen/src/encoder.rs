//! Bit-level encoder/decoder implementations for CAN messages
//!
//! This module provides the actual encode/decode logic that will replace the
//! stub implementations in the generated message types.

/// Extract bits from a byte array at a specific position
pub fn extract_signal(data: &[u8], start_bit: usize, length: usize) -> Result<u64, String> {
    if data.len() < (start_bit + length + 7) / 8 {
        return Err("Insufficient data for signal extraction".to_string());
    }

    let mut value = 0u64;

    for i in 0..length {
        let bit_pos = start_bit + i;
        let byte_idx = bit_pos / 8;
        let bit_in_byte = bit_pos % 8;

        if byte_idx >= data.len() {
            return Err("Bit position exceeds data length".to_string());
        }

        let bit = (data[byte_idx] >> bit_in_byte) & 1;
        value |= (bit as u64) << i;
    }

    Ok(value)
}

/// Pack bits into a byte array at a specific position
pub fn pack_signal(
    data: &mut [u8],
    start_bit: usize,
    length: usize,
    value: u64,
) -> Result<(), String> {
    if data.len() < (start_bit + length + 7) / 8 {
        return Err("Insufficient data for signal packing".to_string());
    }

    let mask = (1u64 << length) - 1;
    let masked_value = value & mask;

    for i in 0..length {
        let bit_pos = start_bit + i;
        let byte_idx = bit_pos / 8;
        let bit_in_byte = bit_pos % 8;

        if byte_idx >= data.len() {
            return Err("Bit position exceeds data length".to_string());
        }

        let bit = (masked_value >> i) & 1;
        if bit != 0 {
            data[byte_idx] |= 1 << bit_in_byte;
        } else {
            data[byte_idx] &= !(1 << bit_in_byte);
        }
    }

    Ok(())
}

/// Apply DBC scaling to convert from raw to engineering units
pub fn apply_scaling(
    raw_value: u64,
    factor: f64,
    offset: f64,
    is_signed: bool,
    bit_length: usize,
) -> f64 {
    let value = if is_signed && bit_length > 0 {
        let sign_bit = 1u64 << (bit_length - 1);
        if raw_value & sign_bit != 0 {
            // Negative number in two's complement
            let max_val = 1u64 << bit_length;
            (raw_value as i64 - max_val as i64) as f64
        } else {
            raw_value as f64
        }
    } else {
        raw_value as f64
    };

    value * factor + offset
}

/// Apply inverse DBC scaling to convert from engineering to raw units
pub fn apply_inverse_scaling(eng_value: f64, factor: f64, offset: f64, bit_length: usize) -> u64 {
    let scaled = ((eng_value - offset) / factor).round();
    let max_val = (1u64 << bit_length) - 1;

    if scaled < 0.0 {
        0
    } else if scaled as u64 > max_val {
        max_val
    } else {
        scaled as u64
    }
}

/// Extract device ID from CAN ID (assumes lower 8 bits contain device ID)
pub fn extract_device_id(can_id: u32) -> u8 {
    (can_id & 0xFF) as u8
}

/// Embed device ID into CAN ID
pub fn embed_device_id(base_can_id: u32, device_id: u8) -> u32 {
    (base_can_id & 0xFFFFFF00) | (device_id as u32)
}

/// MCM_MotorCommandMessage decoder implementation
pub fn decode_mcm_motor_command_message(
    can_id: u32,
    data: &[u8],
) -> Result<(u8, u64, u64, f64, f64), String> {
    if data.len() < 8 {
        return Err("Invalid data length for MCM_MotorCommandMessage".to_string());
    }

    let device_id = extract_device_id(can_id);

    // MCM_OnOffDirectionCommand: bit 0, length 2
    let on_off_direction = extract_signal(data, 0, 2)?;

    // MCM_PowerHoldCommand: bit 2, length 2
    let power_hold = extract_signal(data, 2, 2)?;

    // MCM_MotorSpeedCommand: bit 8, length 16, factor 0.5
    let motor_speed_raw = extract_signal(data, 8, 16)?;
    let motor_speed = apply_scaling(motor_speed_raw, 0.5, 0.0, false, 16);

    // MCM_PercentMotorSpeedCommand: bit 24, length 8, factor 0.5
    let percent_speed_raw = extract_signal(data, 24, 8)?;
    let percent_speed = apply_scaling(percent_speed_raw, 0.5, 0.0, false, 8);

    Ok((
        device_id,
        on_off_direction,
        power_hold,
        motor_speed,
        percent_speed,
    ))
}

/// MSM2_MotorStatusMessage2 encoder implementation
pub fn encode_msm2_motor_status_message2(
    device_id: u8,
    on_off_direction: u64,
    controller_status: u64,
    command_status: u64,
    motor_speed: f64,
    external_temp: f64,
    motor_power: f64,
    service_indicator: u64,
    operation_status: u64,
) -> [u8; 8] {
    let mut data = [0u8; 8];

    // MSM2_OnOffDirectionStatus: bit 0, length 2
    let _ = pack_signal(&mut data, 0, 2, on_off_direction);

    // MSM2_ControllerStatus: bit 2, length 4
    let _ = pack_signal(&mut data, 2, 4, controller_status);

    // MSM2_CommandStatus: bit 6, length 2
    let _ = pack_signal(&mut data, 6, 2, command_status);

    // MSM2_MeasuredMotorSpeed: bit 8, length 16, factor 0.5
    let motor_speed_raw = apply_inverse_scaling(motor_speed, 0.5, 0.0, 16);
    let _ = pack_signal(&mut data, 8, 16, motor_speed_raw);

    // MSM2_MeasuredExternalTemp: bit 24, length 16, factor 0.03125, offset -273
    let temp_raw = apply_inverse_scaling(external_temp, 0.03125, -273.0, 16);
    let _ = pack_signal(&mut data, 24, 16, temp_raw);

    // MSM2_MeasuredMotorPower: bit 40, length 16, factor 0.5
    let power_raw = apply_inverse_scaling(motor_power, 0.5, 0.0, 16);
    let _ = pack_signal(&mut data, 40, 16, power_raw);

    // MSM2_ServiceIndicator: bit 56, length 2
    let _ = pack_signal(&mut data, 56, 2, service_indicator);

    // MSM2_OperationStatus: bit 58, length 2
    let _ = pack_signal(&mut data, 58, 2, operation_status);

    data
}

/// MET_MeasuredExternalTemperature encoder implementation
pub fn encode_met_external_temperature(device_id: u8, temperature: f64) -> [u8; 8] {
    let mut data = [0u8; 8];

    // MET_ExternalTemperatureMeasured: bit 0, length 16, factor 0.03125, offset -273
    let temp_raw = apply_inverse_scaling(temperature, 0.03125, -273.0, 16);
    let _ = pack_signal(&mut data, 0, 16, temp_raw);

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_signal() {
        let data = [0b10101100, 0b11010011]; // Example data

        // Extract 2 bits starting at bit 0 (should be 0b00)
        assert_eq!(extract_signal(&data, 0, 2).unwrap(), 0b00);

        // Extract 2 bits starting at bit 2 (should be 0b11)
        assert_eq!(extract_signal(&data, 2, 2).unwrap(), 0b11);

        // Extract 4 bits starting at bit 4 (should be 0b1010)
        assert_eq!(extract_signal(&data, 4, 4).unwrap(), 0b1010);
    }

    #[test]
    fn test_pack_signal() {
        let mut data = [0u8; 2];

        // Pack 2 bits at position 0
        pack_signal(&mut data, 0, 2, 0b11).unwrap();
        assert_eq!(data[0] & 0b11, 0b11);

        // Pack 2 bits at position 2
        pack_signal(&mut data, 2, 2, 0b01).unwrap();
        assert_eq!((data[0] >> 2) & 0b11, 0b01);
    }

    #[test]
    fn test_scaling() {
        // Test factor 0.5, offset 0
        assert_eq!(apply_scaling(100, 0.5, 0.0, false, 16), 50.0);
        assert_eq!(apply_inverse_scaling(50.0, 0.5, 0.0, 16), 100);

        // Test factor 0.03125, offset -273 (temperature)
        let temp_c = apply_scaling(10000, 0.03125, -273.0, false, 16);
        assert!((temp_c - 39.5).abs() < 0.01); // 10000 * 0.03125 - 273 = 39.5
    }

    #[test]
    fn test_device_id_operations() {
        let base_id = 0x12345600;
        let device_id = 0x8A;

        let combined = embed_device_id(base_id, device_id);
        assert_eq!(combined, 0x1234568A);

        let extracted = extract_device_id(combined);
        assert_eq!(extracted, device_id);
    }

    #[test]
    fn test_mcm_decode() {
        // Create test data with known values
        let mut data = [0u8; 8];

        // Set MCM_OnOffDirectionCommand to 1 (bits 0-1)
        data[0] |= 1;

        // Set MCM_PowerHoldCommand to 1 (bits 2-3)
        data[0] |= 1 << 2;

        // Set MCM_MotorSpeedCommand to 2000 raw (1000 RPM at 0.5 factor)
        // Bits 8-23
        let speed_raw = 2000u16;
        data[1] = speed_raw as u8;
        data[2] = (speed_raw >> 8) as u8;

        let can_id = 0x1234568A; // Base + device ID 0x8A

        let (device_id, on_off, power_hold, motor_speed, _percent_speed) =
            decode_mcm_motor_command_message(can_id, &data).unwrap();

        assert_eq!(device_id, 0x8A);
        assert_eq!(on_off, 1);
        assert_eq!(power_hold, 1);
        assert_eq!(motor_speed, 1000.0); // 2000 * 0.5
    }
}
