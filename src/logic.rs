use std::ops::Fn;
use crate::defs::{ ResponseCode, MessageElementType, element_types };
use crate::protocol::{ Frame, MessageElement };

pub fn process_incoming_frame<F>(request: &Frame, cb: F) -> Option<Frame>
    where F: Fn(&Frame) -> ResponseCode
{
    let response_code = cb(request);

    let mut ack_data: Vec<u8> = vec![response_code.as_u8()];
    if response_code != ResponseCode::Ok {
        ack_data.push(request.sequence_counter);
    }

    let mut response = create_command_frame(element_types::UECP_ACK, &ack_data);
    response.set_addresses(request.site_address, request.encoder_address);
    Some(response)
}

pub fn create_command_frame(element_type: MessageElementType, data: &[u8]) -> Frame {
    let mut frame = Frame::new();
    frame.elements.push(
        MessageElement::new(element_type, data)
    );
    frame
}