use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::StereoEQ;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
#[typeshare]
pub struct HearIDType(pub u8); // TODO: Move to enum?

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
#[typeshare]
pub struct HearIDMusicType(pub u8); // TODO: Move to enum?

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
#[typeshare]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum HearID {
    Base(BaseHearID),
    Custom(CustomHearID),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
#[typeshare]
pub struct BaseHearID {
    pub enabled: bool,
    pub values: StereoEQ,
    pub time: i32,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
#[typeshare]
pub struct CustomHearID {
    pub base: BaseHearID,
    pub hearid_type: HearIDType,
    pub hearid_music_type: HearIDMusicType,
    pub custom_values: StereoEQ,
    pub has_set_custom_values: bool,
    pub hear_id_eq_index: Option<u16>,
}
