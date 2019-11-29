use std::ops::Fn;
use crate::defs::{ ResponseCode, element_types };
use crate::protocol::{ Frame, MessageElement };

pub fn process_incoming_frame<F>(request: &Frame, cb: F) -> Option<Frame>
    where F: Fn(&Frame) -> ResponseCode
{
    let response_code = cb(request);

    let mut ack_data: Vec<u8> = vec![response_code as u8];
    if response_code != ResponseCode::Ok {
        ack_data.push(request.sequence_counter as u8);
    }

    let mut response = Frame::response_for(request);
    response.elements.push(
        MessageElement::new(element_types::UECP_ACK, &ack_data)
    );
    Some(response)
}