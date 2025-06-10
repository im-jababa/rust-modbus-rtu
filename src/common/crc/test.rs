use super::*;


#[test]
fn test_generate_modbus16_normal() {
    let bytes: [u8; 8] = [0x01, 0x06, 0x12, 0x34, 0x56, 0x78, 0x00, 0x00];
    let crc_bytes: u16 = generate_modbus16(&bytes[..6]);
    assert_eq!(crc_bytes, 0xFEF2);
}


#[test]
fn test_generate_modbus16_empty() {
    let bytes: [u8; 0] = [];
    let crc_bytes: u16 = generate_modbus16(&bytes);
    assert_eq!(crc_bytes, 0xFFFF);
}


#[test]
fn test_validate_ok() {
    let bytes: [u8; 8] = [0x01, 0x06, 0x12, 0x34, 0x56, 0x78, 0xF2, 0xFE];
    assert!(validate(&bytes).is_ok());
}


#[test]
fn test_validate_err() {
    let bytes: [u8; 8] = [0x01, 0x06, 0x12, 0x34, 0x56, 0x78, 0xF7, 0x2D];
    assert!(validate(&bytes).is_err());
}
