use serde::{Deserialize, Serialize};

use super::StereoEQ;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct HearIDType(pub u8); // TODO: Move to enum?

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct HearIDMusicType(pub u8); // TODO: Move to enum?

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum HearID {
    Base(BaseHearID),
    Custom(CustomHearID),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct BaseHearID {
    pub enabled: bool,
    pub values: StereoEQ,
    pub time: i32,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct CustomHearID {
    pub base: BaseHearID,
    pub hearid_type: HearIDType,
    pub hearid_music_type: HearIDMusicType,
    pub custom_values: Option<StereoEQ>,
}
