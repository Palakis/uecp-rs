extern crate uecp_rs;
extern crate bytebuffer;

use bytebuffer::*;

#[test]
fn test_to_e1() {
    let source_str = "hey";
    let result = uecp_rs::ebulatin::to_e1(source_str);
}

#[test]
fn test_from_e1() {
    let source_buffer = ByteBuffer::from_bytes(&vec![]);
    let result = uecp_rs::ebulatin::from_e1(&source_buffer);
}