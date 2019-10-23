#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum ElementCode {
    PI = 0x01,
    PS = 0x02,
    PIN = 0x06,
    DecoderInformation = 0x04,
    TrafficProgram = 0x03,
    MusicSpeech = 0x05,
    ProgramType = 0x07,
    ProgramTypeName = 0x3E,
    Radiotext = 0x0A,
    AF = 0x13,
    EonAF = 0x14,
    SlowLabeling = 0x1A,
    LinkageInfo = 0x2E,

    OdaConfig = 0x40,
    OdaIdent = 0x41,
    OdaFreeFormat = 0x42,
    OdaPriority = 0x43,
    OdaBurstMode = 0x44,
    OdaSpinningWheel = 0x45,
    OdaData = 0x46,
    OdaDataACL = 0x47,

    TDC = 0x26,
    EWS = 0x2B,
    IH = 0x25,
    TMC = 0x30,
    FreeFormat = 0x24,

    RTC = 0x0D,
    RTCCorrection = 0x09,
    CTOnOff = 0x19,

    RDSOnOff = 0x1E,
    RDSPhase = 0x22,
    RDSLevel = 0x00,

    SiteAddress = 0x23,
    EncoderAddress = 0x27,
    MakePSNList = 0x28,
    PSNToggle = 0x0B,
    EonElementsToggle = 0x3F,
    CommunicationMode = 0x2C,
    TrafficAnnouncementControl = 0x2A,
    EonTrafficAnnouncementControl = 0x15,
    ReferenceInputSelect = 0x1D,
    DatasetSelect = 0x1C,
    GroupSequence = 0x16,
    ExtendedGroupSequence = 0x38,
    GroupVariantCodeSequence = 0x29,
    EncoderAccessRights = 0x3A,
    CommunicationPortMode = 0x3B,
    CommunicationPortSpeed = 0x3C,
    CommunicationPortTimeout = 0x3D,

    DABDynamicLabelMessage = 0xAA,
    DABDynamicLabelCommand = 0x48,

    UECPAcknowledgement = 0x18,
    UECPRequest = 0x17
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

    pub fn include_length_field(ec: ElementCode) -> bool {
        match ec {
            ElementCode::Radiotext |
            ElementCode::AF |
            ElementCode::EonAF |
            ElementCode::OdaIdent |
            ElementCode::OdaPriority |
            ElementCode::OdaData |
            ElementCode::TDC |
            ElementCode::TMC |
            ElementCode::UECPRequest |
            ElementCode::DABDynamicLabelMessage |
            ElementCode::DABDynamicLabelCommand |
            ElementCode::MakePSNList |
            ElementCode::PSNToggle |
            ElementCode::GroupSequence |
            ElementCode::ExtendedGroupSequence |
            ElementCode::GroupVariantCodeSequence
            => true,

            _ => false
        }
    }

    pub fn include_dsn_psn_fields(ec: ElementCode) -> bool {
        match ec {
            ElementCode::PI |
            ElementCode::PS |
            ElementCode::PIN |
            ElementCode::TrafficProgram |
            ElementCode::MusicSpeech |
            ElementCode::ProgramType |
            ElementCode::ProgramTypeName |
            ElementCode::Radiotext |
            ElementCode::AF |
            ElementCode::EonAF |
            ElementCode::SlowLabeling |
            ElementCode::LinkageInfo |
            ElementCode::OdaIdent
            => true,

            _ => false
        }
    }

    pub fn exclude_psn_field(ec: ElementCode) -> bool {
        match ec {
            ElementCode::SlowLabeling |
            ElementCode::OdaIdent
            => true,

            _ => false
        }
    }

    pub fn get_fixed_element_length(ec: ElementCode) -> usize {
        match ec {
            ElementCode::PI |
            ElementCode::PIN |
            ElementCode::SlowLabeling |
            ElementCode::LinkageInfo |
            ElementCode::OdaBurstMode |
            ElementCode::RTCCorrection |
            ElementCode::RDSPhase |
            ElementCode::RDSLevel |
            ElementCode::EncoderAddress |
            ElementCode::TrafficAnnouncementControl |
            ElementCode::EonTrafficAnnouncementControl |
            ElementCode::CommunicationPortMode |
            ElementCode::CommunicationPortSpeed |
            ElementCode::CommunicationPortTimeout |
            ElementCode::UECPAcknowledgement
            => 2,

            ElementCode::SiteAddress |
            ElementCode::EncoderAccessRights
            => 3,

            ElementCode::OdaSpinningWheel |
            ElementCode::OdaDataACL
            => 4,
            
            ElementCode::EWS
            => 5,

            ElementCode::IH |
            ElementCode::FreeFormat
            => 6,            

            ElementCode::OdaConfig |
            ElementCode::OdaFreeFormat
            => 7,

            ElementCode::PS |
            ElementCode::ProgramTypeName |
            ElementCode::RTC
            => 8,
            
            _ => 1
        }
    }

    pub fn get_next_element_length(bytes: &[u8]) -> usize {
        let mut result: usize = 1;
        
        let element_code = ElementCode::from_u8(bytes[0]).unwrap();
        if include_dsn_psn_fields(element_code) {
            result += 1;
            
            if !exclude_psn_field(element_code) {
                result += 1;
            }
        }

        if include_length_field(element_code) {
            result += 1 + (bytes[result] as usize);
        } else {
            result += get_fixed_element_length(element_code);
        }

        result
    }
}
