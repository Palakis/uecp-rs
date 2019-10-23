#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[allow(non_camel_case_types)]
pub enum ElementCode {
    RDS_PI = 0x01,
    RDS_PS = 0x02,
    RDS_PIN = 0x06,
    RDS_DI = 0x04,
    RDS_TA_TP = 0x03,
    RDS_MS = 0x05,
    RDS_PTY = 0x07,
    RDS_PTYN = 0x3E,
    RDS_RT = 0x0A,
    RDS_AF = 0x13,
    RDS_EON_AF = 0x14,
    RDS_SLOW_LABELING = 0x1A,
    RDS_LINKAGE_INFO = 0x2E,

    ODA_CONFIG = 0x40,
    ODA_IDENT = 0x41,
    ODA_FREE_FORMAT = 0x42,
    ODA_PRIORITY = 0x43,
    ODA_BURST_MODE = 0x44,
    ODA_SPIN_WHEEL = 0x45,
    ODA_DATA = 0x46,
    ODA_DATA_ACL = 0x47,

    TDC = 0x26,
    EWS = 0x2B,
    IH = 0x25,
    TMC = 0x30,
    FREE_FORMAT = 0x24,

    RTC = 0x0D,
    RTC_CORRECTION = 0x09,
    CT_ON_OFF = 0x19,

    RDS_ON_OFF = 0x1E,
    RDS_PHASE = 0x22,
    RDS_LEVEL = 0x00,

    DAB_DL_MESSAGE = 0xAA,
    DAB_DL_COMMAND = 0x48,

    UECP_ACK = 0x18,
    UECP_REQUEST = 0x17
}

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ODABufferConfig {
    TransmitOnce,
    AddToCyclic,
    ClearCyclic
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ODATransmitMode {
    Normal,
    Burst,
    SpinningWheel
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ODAConfigKind {
    Data,
    ShortMessage
}

pub mod element_code_rules {
    use num_traits::FromPrimitive;
    use crate::defs::ElementCode;

    pub fn include_length_field(value: &ElementCode) -> bool {
        static HAS_MEL: &'static [ElementCode] = &[
            ElementCode::RDS_RT,
            ElementCode::RDS_AF,
            ElementCode::RDS_EON_AF,
            ElementCode::ODA_IDENT,
            ElementCode::ODA_PRIORITY,
            ElementCode::ODA_DATA,
            ElementCode::TDC,
            ElementCode::TMC,
            ElementCode::UECP_REQUEST,
            ElementCode::DAB_DL_MESSAGE,
            ElementCode::DAB_DL_COMMAND
        ];

        HAS_MEL.contains(value)
    }

    pub fn include_dsn_psn_fields(value: &ElementCode) -> bool {
        static HAS_DSN_PSN: &'static [ElementCode] = &[
            ElementCode::RDS_PI,
            ElementCode::RDS_PS,
            ElementCode::RDS_PIN,
            ElementCode::RDS_TA_TP,
            ElementCode::RDS_MS,
            ElementCode::RDS_PTY,
            ElementCode::RDS_PTYN,
            ElementCode::RDS_RT,
            ElementCode::RDS_AF,
            ElementCode::RDS_EON_AF,
            ElementCode::RDS_SLOW_LABELING,
            ElementCode::RDS_LINKAGE_INFO,
            ElementCode::ODA_IDENT
        ];

        HAS_DSN_PSN.contains(value)
    }

    pub fn exclude_psn_field(value: &ElementCode) -> bool {
        static EXCLUDE_PSN: &'static [ElementCode] = &[
            ElementCode::RDS_SLOW_LABELING,
            ElementCode::ODA_IDENT
        ];

        EXCLUDE_PSN.contains(value)
    }

    pub fn get_next_element_length(bytes: &[u8]) -> usize {
        let mut result: usize = 1;
        
        let element_code = ElementCode::from_u8(bytes[0]).unwrap();
        if include_dsn_psn_fields(&element_code) {
            result += 1;
            
            if !exclude_psn_field(&element_code) {
                result += 1;
            }
        }

        if include_length_field(&element_code) {
            result += (1 + (bytes[result] as usize));
        } else {
            // TODO fixed sizes dictionary
        }

        result
    }
}
