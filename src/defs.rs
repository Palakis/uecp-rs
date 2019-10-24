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

    pub fn get_next_element_length(bytes: &[u8]) -> usize {
        let mut result: usize = 1;
        
        let element_type = element_types::from_code(bytes[0]).unwrap();

        result += match element_type.dsn_psn_type {
            DSNPSNType::None => 0,
            DSNPSNType::DSNOnly => 1,
            DSNPSNType::All => 2
        };

        result += match element_type.length_type {
            LengthType::FixedLength(x) => x,
            LengthType::VariableLength => 1 + bytes[result] as usize
        };

        result
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
        { RT, 0x0Au8, DSNPSNType::All, LengthType::VariableLength }
    }

    pub fn from_code(code: u8) -> Result<MessageElementType, &'static str> {
        match ELEMENT_CODE_TO_MAP.get(&code) {
            Some(x) => Ok(*x),
            None => Err("unknown element code")
        }
    }
}

// #[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
// pub enum ElementCode {
//     PI = 0x01,
//     PS = 0x02,
//     PIN = 0x06,
//     DecoderInformation = 0x04,
//     TrafficProgram = 0x03,
//     MusicSpeech = 0x05,
//     ProgramType = 0x07,
//     ProgramTypeName = 0x3E,
//     Radiotext = 0x0A,
//     AF = 0x13,
//     EonAF = 0x14,
//     SlowLabeling = 0x1A,
//     LinkageInfo = 0x2E,

//     OdaConfig = 0x40,
//     OdaIdent = 0x41,
//     OdaFreeFormat = 0x42,
//     OdaPriority = 0x43,
//     OdaBurstMode = 0x44,
//     OdaSpinningWheel = 0x45,
//     OdaData = 0x46,
//     OdaDataACL = 0x47,

//     TDC = 0x26,
//     EWS = 0x2B,
//     IH = 0x25,
//     TMC = 0x30,
//     FreeFormat = 0x24,

//     RTC = 0x0D,
//     RTCCorrection = 0x09,
//     CTOnOff = 0x19,

//     RDSOnOff = 0x1E,
//     RDSPhase = 0x22,
//     RDSLevel = 0x00,

//     SiteAddress = 0x23,
//     EncoderAddress = 0x27,
//     MakePSNList = 0x28,
//     PSNToggle = 0x0B,
//     EonElementsToggle = 0x3F,
//     CommunicationMode = 0x2C,
//     TrafficAnnouncementControl = 0x2A,
//     EonTrafficAnnouncementControl = 0x15,
//     ReferenceInputSelect = 0x1D,
//     DatasetSelect = 0x1C,
//     GroupSequence = 0x16,
//     ExtendedGroupSequence = 0x38,
//     GroupVariantCodeSequence = 0x29,
//     EncoderAccessRights = 0x3A,
//     CommunicationPortMode = 0x3B,
//     CommunicationPortSpeed = 0x3C,
//     CommunicationPortTimeout = 0x3D,

//     SpecificCommand = 0x2D,
//     DABDynamicLabelMessage = 0xAA,
//     DABDynamicLabelCommand = 0x48,

//     UECPAcknowledgement = 0x18,
//     UECPRequest = 0x17
// }

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

// pub mod element_code_rules {
//     use num_traits::FromPrimitive;
//     use crate::defs::ElementCode;

//     pub fn include_length_field(ec: ElementCode) -> bool {
//         match ec {
//             ElementCode::Radiotext |
//             ElementCode::AF |
//             ElementCode::EonAF |
//             ElementCode::OdaIdent |
//             ElementCode::OdaPriority |
//             ElementCode::OdaData |
//             ElementCode::TDC |
//             ElementCode::TMC |
//             ElementCode::UECPRequest |
//             ElementCode::MakePSNList |
//             ElementCode::PSNToggle |
//             ElementCode::GroupSequence |
//             ElementCode::ExtendedGroupSequence |
//             ElementCode::GroupVariantCodeSequence |
//             ElementCode::SpecificCommand |
//             ElementCode::DABDynamicLabelMessage |
//             ElementCode::DABDynamicLabelCommand
//             => true,

//             _ => false
//         }
//     }

//     pub fn include_dsn_psn_fields(ec: ElementCode) -> bool {
//         match ec {
//             ElementCode::PI |
//             ElementCode::PS |
//             ElementCode::PIN |
//             ElementCode::TrafficProgram |
//             ElementCode::MusicSpeech |
//             ElementCode::ProgramType |
//             ElementCode::ProgramTypeName |
//             ElementCode::Radiotext |
//             ElementCode::AF |
//             ElementCode::EonAF |
//             ElementCode::SlowLabeling |
//             ElementCode::LinkageInfo |
//             ElementCode::OdaIdent
//             => true,

//             _ => false
//         }
//     }

//     pub fn exclude_psn_field(ec: ElementCode) -> bool {
//         match ec {
//             ElementCode::SlowLabeling |
//             ElementCode::OdaIdent
//             => true,

//             _ => false
//         }
//     }

//     pub fn get_fixed_element_length(ec: ElementCode) -> usize {
//         match ec {
//             ElementCode::PI |
//             ElementCode::PIN |
//             ElementCode::SlowLabeling |
//             ElementCode::LinkageInfo |
//             ElementCode::OdaBurstMode |
//             ElementCode::RTCCorrection |
//             ElementCode::RDSPhase |
//             ElementCode::RDSLevel |
//             ElementCode::EncoderAddress |
//             ElementCode::TrafficAnnouncementControl |
//             ElementCode::EonTrafficAnnouncementControl |
//             ElementCode::CommunicationPortMode |
//             ElementCode::CommunicationPortSpeed |
//             ElementCode::CommunicationPortTimeout |
//             ElementCode::UECPAcknowledgement
//             => 2,

//             ElementCode::SiteAddress |
//             ElementCode::EncoderAccessRights
//             => 3,

//             ElementCode::OdaSpinningWheel |
//             ElementCode::OdaDataACL
//             => 4,
            
//             ElementCode::EWS
//             => 5,

//             ElementCode::IH |
//             ElementCode::FreeFormat
//             => 6,            

//             ElementCode::OdaConfig |
//             ElementCode::OdaFreeFormat
//             => 7,

//             ElementCode::PS |
//             ElementCode::ProgramTypeName |
//             ElementCode::RTC
//             => 8,
            
//             _ => 1
//         }
//     }
// }
