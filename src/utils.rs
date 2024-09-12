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

pub fn vec_bool_to_vec_u8(vec: &Vec<bool>) -> Vec<u8> {
    let mut result = Vec::new();
    
    // 8개씩 묶어서 처리
    for chunk in vec.chunks(8) {
        let mut byte = 0u8;
        
        for (i, &b) in chunk.iter().enumerate() {
            if b {
                byte |= 1 << i; // i번째 비트를 1로 설정
            }
        }
        
        result.push(byte);
    }
    
    result
}