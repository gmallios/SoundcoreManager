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
pub enum TransparencyMode {
    #[default]
    FullyTransparent = 0,
    Vocal = 1,
}

impl TransparencyMode {
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
        assert_eq!(
            TransparencyMode::from_u8(0),
            Some(TransparencyMode::FullyTransparent)
        );
        assert_eq!(TransparencyMode::from_u8(1), Some(TransparencyMode::Vocal));
    }

    #[test]
    fn init_from_u8_invalid() {
        assert_eq!(TransparencyMode::from_u8(10), None);
    }

    #[test]
    fn returns_value() {
        assert_eq!(TransparencyMode::FullyTransparent.as_u8(), 0);
        assert_eq!(TransparencyMode::Vocal.as_u8(), 1);
    }
}
