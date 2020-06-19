#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DecodeError {
    UnknownElementType,
    CRCError,
    MessageTooLarge
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EncodeError {
    ElementTooLarge,
    MessageTooLarge,
    InvalidSiteAddress,
    InvalidEncoderAddress
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResponseCode {
    Ok = 0,
    CRCError = 1,
    NotReceived = 2,
    UnknownMessage = 3,
    DSNError = 4,
    PSNError = 5,
    ParameterOutOfRange = 6,
    ElementLengthError = 7,
    FieldLengthError = 8,
    NotAcceptable = 9,
    EndMessageMissing = 10,
    BufferOverflow = 11,
    BadStuffing = 12,
    UnexpectedEndOfMessage = 13,
    NotInterpreted = 14
}

impl ResponseCode {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum PTY {
    None = 0,
    News = 1,
    CurrentAffairs = 2,
    Information = 3,
    Sport = 4,
    Education = 5,
    Drama = 6,
    Culture = 7,
    Science = 8,
    Varied = 9,
    PopMusic = 10,
    RockMusic = 11,
    EasyListening = 12,
    LightClassical = 13,
    SeriousClassical = 14,
    OtherMusic = 15,
    Weather = 16,
    Finance = 17,
    Children = 18,
    SocialAffairs = 19,
    Religion = 20,
    PhoneIn = 21,
    Travel = 22,
    Leisure = 23,
    JazzMusic = 24,
    CountryMusic = 25,
    NationalMusic = 26,
    OldiesMusic = 27,
    FolkMusic = 28,
    Documentary = 29,
    AlarmTest = 30,
    Alarm = 31
}

impl PTY {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LengthType {
    FixedLength(usize),
    VariableLength
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DSNPSNType {
    None,
    DSNOnly,
    All
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MessageElementType {
    pub code: u8,
    pub dsn_psn_type: DSNPSNType,
    pub length_type: LengthType
}

impl MessageElementType {
    pub fn new(code: u8, dsn_psn_type: DSNPSNType, length_type: LengthType) -> Self {
        Self {
            code, dsn_psn_type, length_type
        }
    }

    pub fn get_next_element_length(bytes: &[u8]) -> Option<usize> {
        let mut result: usize = 1;
        
        let element_type = element_types::from_code(bytes[0])?;

        result += match element_type.dsn_psn_type {
            DSNPSNType::None => 0,
            DSNPSNType::DSNOnly => 1,
            DSNPSNType::All => 2
        };

        result += match element_type.length_type {
            LengthType::FixedLength(x) => x,
            LengthType::VariableLength => 1 + bytes[result] as usize
        };

        Some(result)
    }
}

pub mod element_types {
    use crate::defs::{ LengthType, DSNPSNType, MessageElementType };

    macro_rules! register_element_types {
        (
            $(
                { $name:ident, $code:expr, $dsnpsn:expr, $lengthtype:expr }
            ),*
        ) => {
            $(
                pub const $name: MessageElementType = MessageElementType {
                    code: $code,
                    dsn_psn_type: $dsnpsn,
                    length_type: $lengthtype
                };
            )*

            static ELEMENT_CODE_TO_MAP: phf::Map<u8, MessageElementType> = phf_map! {
                $($code => $name),*
            };
        };
    }

    register_element_types! {
        { PI, 0x01u8, DSNPSNType::All, LengthType::FixedLength(2) },
        { PS, 0x02u8, DSNPSNType::All, LengthType::FixedLength(8) },
        { PIN, 0x06u8, DSNPSNType::All, LengthType::FixedLength(2) },
        { DI, 0x04u8, DSNPSNType::All, LengthType::FixedLength(1) },
        { TA_TP, 0x03u8, DSNPSNType::All, LengthType::FixedLength(1) },
        { MS, 0x05u8, DSNPSNType::All, LengthType::FixedLength(1) },
        { PTY, 0x07u8, DSNPSNType::All, LengthType::FixedLength(1) },
        { PTYN, 0x3Eu8, DSNPSNType::All, LengthType::FixedLength(8) },
        { RT, 0x0Au8, DSNPSNType::All, LengthType::VariableLength },
        { AF, 0x13u8, DSNPSNType::All, LengthType::VariableLength },
        { EON_AF, 0x14u8, DSNPSNType::All, LengthType::VariableLength },
        { SLOW_LABELING, 0x1Au8, DSNPSNType::DSNOnly, LengthType::FixedLength(2) },
        { LINKAGE_INFO, 0x2Eu8, DSNPSNType::All, LengthType::FixedLength(2) },

        { ODA_CONFIG, 0x40u8, DSNPSNType::None, LengthType::FixedLength(7) },
        { ODA_IDENT, 0x41u8, DSNPSNType::DSNOnly, LengthType::VariableLength },
        { ODA_FREE_FORMAT, 0x42u8, DSNPSNType::None, LengthType::FixedLength(7) },
        { ODA_PRIORITY, 0x43u8, DSNPSNType::None, LengthType::VariableLength },
        { ODA_BURST_MODE, 0x44u8, DSNPSNType::None, LengthType::FixedLength(2) },
        { ODA_SPINNING_WHEEL, 0x45u8, DSNPSNType::None, LengthType::FixedLength(4) },
        { ODA_DATA, 0x46u8, DSNPSNType::None, LengthType::VariableLength },
        { ODA_DATA_ACL, 0x47u8, DSNPSNType::None, LengthType::FixedLength(4) },

        { TDC, 0x26u8, DSNPSNType::None, LengthType::VariableLength },
        { EWS, 0x2Bu8, DSNPSNType::None, LengthType::FixedLength(5) },
        { IH, 0x25u8, DSNPSNType::None, LengthType::FixedLength(6) },
        { TMC, 0x30u8, DSNPSNType::None, LengthType::VariableLength },
        { FREE_FORMAT, 0x24u8, DSNPSNType::None, LengthType::FixedLength(6) },

        { RTC, 0x0Du8, DSNPSNType::None, LengthType::FixedLength(8) },
        { RTC_CORRECTION, 0x09u8, DSNPSNType::None, LengthType::FixedLength(2) },
        { CT_ON_OFF, 0x19u8, DSNPSNType::None, LengthType::FixedLength(1) },

        { RDS_ON_OFF, 0x1Eu8, DSNPSNType::None, LengthType::FixedLength(1) },
        { RDS_PHASE, 0x22u8, DSNPSNType::None, LengthType::FixedLength(2) },
        { RDS_LEVEL, 0x0Eu8, DSNPSNType::None, LengthType::FixedLength(2) }, // todo recheck from spec

        { SITE_ADDRESS, 0x23u8, DSNPSNType::None, LengthType::FixedLength(3) },
        { ENCODER_ADDRESS, 0x27u8, DSNPSNType::None, LengthType::FixedLength(2) },
        { MAKE_PSN_LIST, 0x28u8, DSNPSNType::DSNOnly, LengthType::VariableLength },
        { PSN_TOGGLE, 0x0Bu8, DSNPSNType::DSNOnly, LengthType::VariableLength },
        { EON_ELEMENTS_TOGGLE, 0x3Fu8, DSNPSNType::All, LengthType::FixedLength(1) },
        { COMM_MODE, 0x2Cu8, DSNPSNType::None, LengthType::FixedLength(1) },
        { TA_CONTROL, 0x2Au8, DSNPSNType::None, LengthType::FixedLength(2) },
        { EON_TA_CONTROL, 0x15u8, DSNPSNType::None, LengthType::FixedLength(2) },
        { REFERENCE_INPUT_SELECT, 0x1Du8, DSNPSNType::None, LengthType::FixedLength(1) },
        { DATASET_SELECT, 0x1Cu8, DSNPSNType::None, LengthType::FixedLength(1) },
        { GROUP_SEQUENCE, 0x16u8, DSNPSNType::DSNOnly, LengthType::VariableLength },
        { EXTENDED_GROUP_SEQUENCE, 0x38u8, DSNPSNType::DSNOnly, LengthType::VariableLength },
        { GROUP_VARIANT_CODE_SEQUENCE, 0x29u8, DSNPSNType::DSNOnly, LengthType::VariableLength },
        { ENCODER_ACCESS_RIGHT, 0x3Au8, DSNPSNType::None, LengthType::FixedLength(3) },
        { COMM_PORT_MODE, 0x3Bu8, DSNPSNType::None, LengthType::FixedLength(2) },
        { COMM_PORT_SPEED, 0x3Cu8, DSNPSNType::None, LengthType::FixedLength(2) },
        { COMM_PORT_TIMEOUT, 0x3Du8, DSNPSNType::None, LengthType::FixedLength(2) },

        { UECP_ACK, 0x18u8, DSNPSNType::None, LengthType::FixedLength(2) },
        { UECP_REQUEST, 0x17u8, DSNPSNType::None, LengthType::VariableLength },

        { SPECIFIC_COMMAND, 0x2Du8, DSNPSNType::None, LengthType::VariableLength },
        { DAB_DL_MESSAGE, 0xAAu8, DSNPSNType::None, LengthType::VariableLength },
        { DAB_DL_COMMAND, 0x48u8, DSNPSNType::None, LengthType::VariableLength }
    }

    pub fn from_code(code: u8) -> Option<MessageElementType> {
        ELEMENT_CODE_TO_MAP.get(&code).map(|x| *x)
    }
}
