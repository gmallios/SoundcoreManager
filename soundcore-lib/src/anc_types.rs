use serde::{Deserialize, Serialize};
use strum::EnumIter;
use typeshare::typeshare;
use crate::utils::Clamp;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum SoundcoreSoundModeIdentifier {
    ANC(ANCModeIdentifier),
    TRANS(TransModeIdentifier),
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, EnumIter)]
pub enum ANCModeIdentifier {
    AncTransportMode,
    AncOutdoorMode,
    AncIndoorMode,
    AncCustomValue,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, EnumIter)]
pub enum TransModeIdentifier {
    TransparencyFullyTransparentMode,
    TransparencyVocalMode,
}


#[typeshare]
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct ANCRawData {
    pub option: u8,
    pub anc_option: u8,
    pub transparency_option: u8,
    pub anc_custom: u8,
}

// NOTE: We don't know if the profiles are the same for all devices!
// TODO: Pre-defined ANCModes should carry the saved CustomValue as well in order to send it
impl ANCRawData {
    pub const NORMAL_MODE: ANCRawData = ANCRawData {
        option: 2,
        anc_option: 0,
        transparency_option: 0,
        anc_custom: 6,
    };

    pub const ANC_TRANSPORT_MODE: ANCRawData = ANCRawData {
        option: 0,
        anc_option: 0,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const ANC_OUTDOOR_MODE: ANCRawData = ANCRawData {
        option: 0,
        anc_option: 1,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const ANC_INDOOR_MODE: ANCRawData = ANCRawData {
        option: 0,
        anc_option: 2,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const TRANSPARENCY_FULLY_TRANSPARENT_MODE: ANCRawData = ANCRawData {
        option: 1,
        anc_option: 0,
        transparency_option: 0,
        anc_custom: 6,
    };

    pub const TRANSPARENCY_VOCAL_MODE: ANCRawData = ANCRawData {
        option: 1,
        anc_option: 0,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub fn anc_custom_value(val: u8) -> ANCRawData {
        ANCRawData {
            option: 0,
            anc_option: 3,
            transparency_option: 1,
            anc_custom: Clamp::clamp(val, 0, 10),
        }
    }

    pub fn decode(arr: &[u8]) -> Result<ANCRawData, std::string::FromUtf8Error> {
        let anc_custom: u8;
        
        if arr[3] == 255 {
            anc_custom = 255;
        } else {
            anc_custom = Clamp::clamp(arr[3], 0, 10);
        }

        Ok(ANCRawData {
            option: Clamp::clamp(arr[0], 0, 2),
            anc_option: Clamp::clamp(arr[1], 0, 3),
            transparency_option: arr[2],
            anc_custom,
        })
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let anc_custom: u8;

        if self.anc_custom == 255 {
            anc_custom = 255;
        } else {
            anc_custom = Clamp::clamp(self.anc_custom, 0, 10);
        }

        [
            Clamp::clamp(self.option, 0, 2),
            Clamp::clamp(self.anc_option, 0, 3),
            self.transparency_option,
            anc_custom,
        ]
    }
}
