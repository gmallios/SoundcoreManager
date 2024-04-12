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
#[typeshare]
pub enum AdaptiveANCMode {
    Custom = 0,
    #[default]
    Adaptive = 1,
}

impl AdaptiveANCMode {
    pub fn from_u8(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}
