use uecp_rs::defs::*;
use uecp_rs::protocol::*;

#[test]
fn test_compute_crc() {
    let source_bytes = "2D111234010105ABCD123F0XXXX11069212491000320066".as_bytes();
    let result = Frame::compute_crc16_genibus(&source_bytes);
    assert_eq!(result, 0x9723);
}

#[test]
fn test_apply_byte_stuffing() {
    let source = vec![0x12, 0xFD, 0x15, 0xFF, 0xAB, 0xFE, 0x26];
    let result = Frame::apply_byte_stuffing(&source);
    assert_eq!(result, vec![0x12, 0xFD, 0x00, 0x15, 0xFD, 0x02, 0xAB, 0xFD, 0x01, 0x26]);
}
