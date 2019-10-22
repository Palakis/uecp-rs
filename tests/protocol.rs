use uecp_rs::protocol::*;

#[test]
fn test_compute_crc() {
    let source_bytes = "2D111234010105ABCD123F0XXXX11069212491000320066".as_bytes();
    let result = Frame::compute_crc16_genibus(&source_bytes);
    assert_eq!(result, 0x9723);
}
