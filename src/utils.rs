

/// ### crc16_modbus
/// Generate CRC16 modbus bytes from the given data.
/// ### Params
/// - `data`: The source bytes frin which to generate the CRC.
/// ### Returns
/// - 16-bit CRC as two bytes. Index 0 is the low byte, and index 1 is the high byte.
pub fn crc16_modbus(data: &[u8]) -> [u8; 2] {
    let mut crc: u16 = 0xFFFF;
    for byte in data {
        crc ^= *byte as u16;
        for _ in 0..8 {
            if (crc & 0x0001) != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc.to_le_bytes()
}

/// ### vec_bool_to_vec_u8
/// Converts a `Vec<bool>` into a `Vec<u8>`, where each `u8` contains up to 8 bits from the original `Vec<bool>`.
/// ### Params
/// - `vec`: The source `Vec<bool>` to be grouped into bytes.
/// ### Returns
/// - A `Vec<u8>`, where each byte is formed by grouping 8 consecutive boolean values from the input vector.
///   If the number of bits is not a multiple of 8, the remaining bits will be filled with 0s.
pub fn vec_bool_to_vec_u8(vec: &Vec<bool>) -> Vec<u8> {
    let mut result = Vec::new();
    for chunk in vec.chunks(8) {
        let mut byte = 0u8;
        for (i, &b) in chunk.iter().enumerate() {
            if b {
                byte |= 1 << i;
            }
        }
        result.push(byte);
    }
    result
}