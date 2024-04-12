use serde::{Deserialize, Serialize};
use strum::{Display, FromRepr};
use typeshare::typeshare;

#[repr(u8)]
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Clone,
    Copy,
    Default,
    FromRepr,
    Display,
    Hash,
)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub enum CustomizableTransparencyMode {
    #[default]
    TalkMode = 0,
    Custom = 1,
}

impl CustomizableTransparencyMode {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}
