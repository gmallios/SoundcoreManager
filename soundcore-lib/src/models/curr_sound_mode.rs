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
pub enum CurrentSoundMode {
    ANC = 0,
    Transparency = 1,
    #[default]
    Normal = 2,
}

impl CurrentSoundMode {
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
        assert_eq!(CurrentSoundMode::from_u8(0), Some(CurrentSoundMode::ANC));
        assert_eq!(
            CurrentSoundMode::from_u8(1),
            Some(CurrentSoundMode::Transparency)
        );
        assert_eq!(CurrentSoundMode::from_u8(2), Some(CurrentSoundMode::Normal));
    }

    #[test]
    fn init_from_u8_invalid() {
        assert_eq!(CurrentSoundMode::from_u8(10), None);
    }

    #[test]
    fn returns_value() {
        assert_eq!(CurrentSoundMode::ANC.as_u8(), 0);
        assert_eq!(CurrentSoundMode::Transparency.as_u8(), 1);
        assert_eq!(CurrentSoundMode::Normal.as_u8(), 2);
    }
}
