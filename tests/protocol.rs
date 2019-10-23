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

#[test]
fn test_get_address_field() {
    let site_address: u16 = 341;
    let encoder_address: u8 = 21;
    let result = Frame::get_address_field(site_address, encoder_address).unwrap();
    assert_eq!(result, 0x5555);
}

#[test]
fn test_build_pi_frame() {
    let mut frame = Frame::new();
    frame.sequence_counter = 12;
    frame.site_address = 62;
    frame.encoder_address = 14;
    frame.elements.push(
        MessageElement::new(ElementCode::RDS_PI, &[0xC2, 0x01])
    );

    let result = frame.into_bytes().unwrap();
    assert_eq!(result, vec![
        0xFE, // Start
        0x0F, 0x8E, // Address
        0x0C, // Sequence counter
        0x05, 0x01, 0x00, 0x00, 0xC2, 0x01, // Message data
        0xC7, 0x65, // CRC
        0xFF // Stop
    ]);
}

#[test]
fn test_build_rt_frame() {
    let mut frame = Frame::new();
    frame.sequence_counter = 13;
    frame.site_address = 62;
    frame.encoder_address = 14;
    frame.elements.push(
        MessageElement::new(ElementCode::RDS_RT, &[0x00, 0x68, 0x65, 0x6C, 0x6C, 0x6F])
    );

    let result = frame.into_bytes().unwrap();
    assert_eq!(result, vec![
        0xFE, // Start
        0x0F, 0x8E, // Address
        0x0D, // Sequence counter
        0x0A, 0x0A, 0x00, 0x00, 0x06, 0x00, 0x68, 0x65, 0x6C, 0x6C, 0x6F,  // Message data
        0x1D, 0xDB, // CRC
        0xFF // Stop
    ]);
}

#[test]
fn test_message_element_encode_and_decode() {
    let source = MessageElement::new(ElementCode::RDS_PI, &[0xAB, 0xCD]);
    let encoded = source.into_bytes().unwrap();
    let decoded = MessageElement::from_bytes(&encoded);
    assert_eq!(decoded.element_code, ElementCode::RDS_PI);
    assert_eq!(decoded.data, &[0xAB, 0xCD]);
}
