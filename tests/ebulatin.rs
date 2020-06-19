extern crate uecp_rs;

#[test]
fn test_to_e1_simple() {
    let source_str = "test";
    let result = uecp_rs::ebulatin::to_e1(source_str);

    assert_eq!(result, vec![0x74, 0x65, 0x73, 0x74]);
}

#[test]
fn test_to_e1_special_chars() {
    let source_str = "éàçüö#$£";
    let result = uecp_rs::ebulatin::to_e1(source_str);

    assert_eq!(result, vec![0x82, 0x81, 0x9B, 0x99, 0x97, 0x23, 0xAB, 0xAA]);
}
