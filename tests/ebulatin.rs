extern crate uecp_rs;

#[test]
fn test_to_e1_simple() {
    let source_str = "test";
    let result = uecp_rs::ebulatin::to_e1(source_str);

    let result_bytes = result.to_bytes();
    assert_eq!(result_bytes[0], 0x74);
    assert_eq!(result_bytes[1], 0x65);
    assert_eq!(result_bytes[2], 0x73);
    assert_eq!(result_bytes[3], 0x74);
}

#[test]
fn test_to_e1_special_chars() {
    let source_str = "éàçüö#$£";
    let result = uecp_rs::ebulatin::to_e1(source_str);

    let result_bytes = result.to_bytes();
    assert_eq!(result_bytes[0], 0x82);
    assert_eq!(result_bytes[1], 0x81);
    assert_eq!(result_bytes[2], 0x9B);
    assert_eq!(result_bytes[3], 0x99);
    assert_eq!(result_bytes[4], 0x97);
    assert_eq!(result_bytes[5], 0x23);
    assert_eq!(result_bytes[6], 0xAB);
    assert_eq!(result_bytes[7], 0xAA);
}
