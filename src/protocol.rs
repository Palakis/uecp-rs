use crate::defs::ElementCode;
use crate::defs::element_code_rules::*;
use bytebuffer::ByteBuffer;

pub struct MessageElement {
    pub element_code: ElementCode,
    pub dataset_number: u8,
    pub program_service_number: u8,
    pub data: Vec<u8>
}

pub struct Frame {
    pub sequence_counter: u8,
    pub site_address: u16,
    pub encoder_address: u16,
    pub elements: Vec<MessageElement>
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageElement {
    pub fn new(element_code: ElementCode, data: Vec<u8>) -> MessageElement {
        MessageElement {
            element_code: element_code,
            dataset_number: 0,
            program_service_number: 0,
            data: data
        }
    }

    pub fn into_bytes(&self) -> Result<Vec<u8>, &'static str> {
        if self.data.len() > 254 {
            return Err("Element data too large")
        }

        let mut buffer = ByteBuffer::new();

        // MEC field
        buffer.write_u8(self.element_code as u8);

        if include_dsn_psn_fields(&self.element_code) {
            // DSN field
            buffer.write_u8(self.dataset_number);

            if !exclude_psn_field(&self.element_code) {
                // PSN field
                buffer.write_u8(self.program_service_number);
            }
        }

        // MEL (length) field
        if include_length_field(&self.element_code) {
            buffer.write_u8(self.data.len() as u8);
        }

        // Element data
        buffer.write_bytes(&self.data);

        Ok(buffer.to_bytes())
    }
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            sequence_counter: 1,
            site_address: 0,
            encoder_address: 0,
            elements: vec![]
        }
    }

    pub fn into_bytes(&self) -> Result<Vec<u8>, &'static str> {
        // Gather all message elements into a single byte array
        let mut message_bytes: Vec<u8> = vec![];
        for element in self.elements.iter() {
            let mut element_bytes = element.into_bytes().unwrap();
            message_bytes.append(&mut element_bytes);
        }

        if message_bytes.len() > 255 {
            return Err("Message too large")
        }

        // Calculate the two-bytes ADD field
        let addr_field: u16 = Frame::get_address_field(self.site_address, self.encoder_address)
                                    .unwrap();
        let addr_first_byte: u8 = (addr_field & 0xFF) as u8;
        let addr_second_byte: u8 = ((addr_field & 0xFF00) >> 8) as u8;

        // Start building the UECP frame
        let mut frame = ByteBuffer::new();
        frame.write_u8(addr_first_byte); // ADD
        frame.write_u8(addr_second_byte); // ADD
        frame.write_u8(self.sequence_counter); // SEQ
        frame.write_u8(message_bytes.len() as u8); // MEL (message element length)
        frame.write_bytes(&message_bytes); // Message

        // Calculate the CRC value mid-way
        let crc = Frame::compute_crc16_genibus(&frame.to_bytes());
        frame.write_u16(crc);
        
        // Apply stuffing
        let stuffed_frame = Frame::apply_byte_stuffing(&frame.to_bytes());

        // Build the final frame
        let mut final_frame = ByteBuffer::new();
        final_frame.write_u8(0xFE);
        final_frame.write_bytes(&stuffed_frame);
        final_frame.write_u8(0xFF);

        // And voilà
        Ok(final_frame.to_bytes()) // TODO
    }

    fn get_address_field(site_address: u16, encoder_address: u16) -> Result<u16, &'static str> {
        if site_address > 1023 {
            return Err("Invalid site address")
        }

        if encoder_address > 64 {
            return Err("Invalid encoder address")
        }
        
        let mut address: u16;
        address = (site_address & 0x3FF) << 6;
        address |= encoder_address & 0x3F;

        Ok(address)
    }

    pub fn compute_crc16_genibus(data: &[u8]) -> u16 {
        let mut crc: u16 = 0xFFFF;

        // I have not idea what's happening here
        // Took from https://github.com/UoC-Radio/rds-control/blob/master/uecp.c#L50-L65
        for c in data {
            crc = (crc >> 8) | (crc << 8);
            crc ^= *c as u16;
            crc ^= (crc & 0xFF) >> 4;
            crc ^= (crc << 8) << 4;
            crc ^= ((crc & 0xFF) << 4) << 1;
        }

        (crc ^ 0xFFFF)
    }

    pub fn apply_byte_stuffing(data: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        
        for c in data {
            let current = *c;
            result.push(current);

            if current >= 0xFD {
                result.push(current - 0xFD);
            }
        }

        result
    }
}