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
pub enum SceneBasedANCMode {
    #[default]
    Transport = 0,
    Outdoor = 1,
    Indoor = 2,
    Custom = 3,
}

impl SceneBasedANCMode {
    pub fn from_u8(value: u8) -> Option<Self> {
        Self::from_repr(value)
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod scene_based_anc_mode_tests {
    use super::*;

    #[test]
    fn init_from_u8() {
        assert_eq!(
            SceneBasedANCMode::from_u8(0),
            Some(SceneBasedANCMode::Transport)
        );
        assert_eq!(
            SceneBasedANCMode::from_u8(1),
            Some(SceneBasedANCMode::Outdoor)
        );
        assert_eq!(
            SceneBasedANCMode::from_u8(2),
            Some(SceneBasedANCMode::Indoor)
        );
        assert_eq!(
            SceneBasedANCMode::from_u8(3),
            Some(SceneBasedANCMode::Custom)
        );
    }

    #[test]
    fn init_from_u8_invalid() {
        assert_eq!(SceneBasedANCMode::from_u8(10), None);
    }

    #[test]
    fn returns_value() {
        assert_eq!(SceneBasedANCMode::Transport.as_u8(), 0);
        assert_eq!(SceneBasedANCMode::Outdoor.as_u8(), 1);
        assert_eq!(SceneBasedANCMode::Indoor.as_u8(), 2);
        assert_eq!(SceneBasedANCMode::Custom.as_u8(), 3);
    }
}
