use serde::{Deserialize, Serialize};
use strum::{Display, FromRepr};

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
pub enum ANCMode {
    #[default]
    Transport = 0,
    Outdoor = 1,
    Indoor = 2,
    Custom = 3,
}

impl ANCMode {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }
}

#[cfg(test)]
mod anc_mode_tests {
    use super::*;

    #[test]
    fn init_from_u8() {
        assert_eq!(ANCMode::from_u8(0), Some(ANCMode::Transport));
        assert_eq!(ANCMode::from_u8(1), Some(ANCMode::Outdoor));
        assert_eq!(ANCMode::from_u8(2), Some(ANCMode::Indoor));
        assert_eq!(ANCMode::from_u8(3), Some(ANCMode::Custom));
    }

    #[test]
    fn init_from_u8_invalid() {
        assert_eq!(ANCMode::from_u8(10), None);
    }

    #[test]
    fn returns_value() {
        assert_eq!(ANCMode::Transport.as_u8(), 0);
        assert_eq!(ANCMode::Outdoor.as_u8(), 1);
        assert_eq!(ANCMode::Indoor.as_u8(), 2);
        assert_eq!(ANCMode::Custom.as_u8(), 3);
    }
}
