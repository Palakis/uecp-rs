use crate::defs::{ LengthType, DSNPSNType, MessageElementType, element_types };
use bytebuffer::ByteBuffer;

pub struct MessageElement {
    pub element_type: MessageElementType,
    pub dataset_number: u8,
    pub program_service_number: u8,
    pub data: Vec<u8>
}

pub struct Frame {
    pub sequence_counter: u8,
    pub site_address: u16,
    pub encoder_address: u8,
    pub elements: Vec<MessageElement>
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageElement {
    pub fn new(element_type: MessageElementType, data: &[u8]) -> MessageElement {
        MessageElement {
            element_type,
            dataset_number: 0,
            program_service_number: 0,
            data: data.to_vec()
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> MessageElement {
        let mut buffer = ByteBuffer::from_bytes(bytes);

        let element_type = element_types::from_code(buffer.read_u8()).unwrap();

        let dataset_number: u8 = match element_type.dsn_psn_type {
            DSNPSNType::DSNOnly | DSNPSNType::All => buffer.read_u8(),
            _ => 0
        };

        let program_service_number: u8 = match element_type.dsn_psn_type {
            DSNPSNType::All => buffer.read_u8(),
            _ => 0
        };;

        match element_type.length_type {
            LengthType::VariableLength => {
                // Skip MEL field
                buffer.read_u8();
            },
            LengthType::FixedLength(_) => {}
        }

        let data: Vec<u8> = buffer.read_bytes(buffer.len() - buffer.get_rpos());

        MessageElement {
            element_type,
            dataset_number,
            program_service_number,
            data
        }
    }

    pub fn into_bytes(&self) -> Result<Vec<u8>, &'static str> {
        if self.data.len() > 254 {
            return Err("Element data too large")
        }

        let mut buffer = ByteBuffer::new();

        // MEC field
        buffer.write_u8(self.element_type.code as u8);

        // DSN and PSN fields
        match self.element_type.dsn_psn_type {
            DSNPSNType::None => {},
            DSNPSNType::DSNOnly => {
                buffer.write_u8(self.dataset_number);
            },
            DSNPSNType::All => {
                buffer.write_u8(self.dataset_number);
                buffer.write_u8(self.program_service_number);
            }
        }

        // MEL (length) field
        match self.element_type.length_type {
            LengthType::VariableLength => {
                buffer.write_u8(self.data.len() as u8);
            }
            LengthType::FixedLength(_) => {}
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

    pub fn from_bytes(bytes: &[u8]) -> Result<Frame, &'static str> {
        // Remove byte stuffing before passing the bytes to the bytebuffer
        let last_index = bytes.len() - 1;
        let unstuffed_bytes = Frame::revert_byte_stuffing(&bytes[1..last_index]); // STA and STP are ignored
        let mut buffer = ByteBuffer::from_bytes(&unstuffed_bytes);

        // Fetch data needed by the CRC computation (from the beginning to before the CRC field)
        let crc_data = buffer.read_bytes(buffer.len() - 2);
        let frame_crc = buffer.read_u16();

        // Compute CRC
        let computed_crc = Frame::compute_crc16_genibus(&crc_data);
        if computed_crc != frame_crc {
            return Err("CRC error - frame likely corrupted");
        }

        // Then come back to the beginning
        buffer.set_rpos(0);

        let address = buffer.read_u16();
        let site_address: u16 = (address & 0xFFC0) >> 6 as u16;
        let encoder_address: u8 = (address & 0x3F) as u8;

        let sequence_counter = buffer.read_u8();
        let message_length = buffer.read_u8() as usize;
        if message_length > 255 {
            return Err("Message too large");
        }

        let message = buffer.read_bytes(message_length);

        Ok(Frame {
            sequence_counter,
            site_address,
            encoder_address,
            elements: Frame::decode_message_field(&message)
        })
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
        let addr_first_byte: u8 = ((addr_field & 0xFF00) >> 8) as u8;
        let addr_second_byte: u8 = (addr_field & 0xFF) as u8;

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

    pub fn get_address_field(site_address: u16, encoder_address: u8) -> Result<u16, &'static str> {
        if site_address > 1023 {
            return Err("Invalid site address")
        }

        if encoder_address > 64 {
            return Err("Invalid encoder address")
        }
        
        let mut address: u16;
        address = (site_address & 0x3FF) << 6;
        address |= (encoder_address & 0x3F) as u16;

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

            if current >= 0xFD {
                result.push(0xFD);
                result.push(current - 0xFD);
            } else {
                result.push(current);
            }
        }

        result
    }

    pub fn revert_byte_stuffing(data: &[u8]) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        for (index, c) in data.iter().enumerate() {
            if index > 0 && data[index-1] == 0xFD {
                continue;
            }

            let current = *c;
            if current == 0xFD {
                result.push(0xFD + data[index+1]);
            } else {
                result.push(current);
            }
        }

        result
    }

    fn decode_message_field(bytes: &[u8]) -> Vec<MessageElement> {
        let mut result: Vec<MessageElement> = vec![];

        let last_index = bytes.len();
        let mut i: usize = 0;
        while i < bytes.len() {
            let readable_bytes = &bytes[i..last_index];
            let element_length = MessageElementType::get_next_element_length(readable_bytes).unwrap();
            result.push(
                MessageElement::from_bytes(readable_bytes)
            );

            i += element_length;
        }

        result
    }
}