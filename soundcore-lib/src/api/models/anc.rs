use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString, IntoEnumIterator};
use typeshare::typeshare;

/// Sound Mode byte alignment
/// | Byte | Description           | Values |
/// |------|-----------------------|--------|
/// | 0    | Mode                  | 0 for ANC, 1 for Transparency, 2 for Normal |
/// | 1    | ANC Sub-mode          | 0/1/2/3 - Transport,Outdoor,Indoor,Custom  |
/// | 2    | Transparency Sub-mode | 0 for Fully Transparent 1 for Vocal for Transparency |
/// | 3    | Sub-mode value        | Used only in the Custom mode |

#[typeshare]
#[derive(
    Debug,
    Default,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    AsRefStr,
    EnumIter,
)]
#[serde(tag = "type", content = "mode")]
pub enum SoundMode {
    #[default]
    NormalMode,
    NoiseCancelling(ANCModes),
    TransparencyMode(TransparencyModes),
}

// Custom Display implementation to print the inner enum value
// in case of NoiseCancelling and TransparencyMode
impl std::fmt::Display for SoundMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl SoundMode {
    pub fn from_bytes(bytes: [u8; 4]) -> Option<Self> {
        match bytes {
            [0, 0, _, _] => Some(Self::NoiseCancelling(ANCModes::Transport)),
            [0, 1, _, _] => Some(Self::NoiseCancelling(ANCModes::Outdoor)),
            [0, 2, _, _] => Some(Self::NoiseCancelling(ANCModes::Indoor)),
            [0, 3, _, _] => Some(Self::NoiseCancelling(ANCModes::Custom(bytes[3]))),
            [1, _, 0, _] => Some(Self::TransparencyMode(TransparencyModes::FullyTransparent)),
            [1, _, 1, _] => Some(Self::TransparencyMode(TransparencyModes::Vocal)),
            [2, _, _, _] => Some(Self::NormalMode),
            _ => None,
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let bytes = match self {
            Self::NoiseCancelling(mode) => match mode {
                ANCModes::Transport => [0, 0, 0, 0],
                ANCModes::Outdoor => [0, 1, 0, 0],
                ANCModes::Indoor => [0, 2, 0, 0],
                ANCModes::Custom(_value) => [0, 3, 0, 0],
            },
            Self::TransparencyMode(mode) => match mode {
                TransparencyModes::FullyTransparent => [1, 0, 0, 0],
                TransparencyModes::Vocal => [1, 0, 1, 0],
            },
            Self::NormalMode => [2, 0, 0, 0],
        };
        bytes.to_vec()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "NormalMode" => Some(SoundMode::NormalMode),
            s if s.starts_with("NoiseCancelling(") && s.ends_with(")") => {
                let inner_str = &s[16..s.len() - 1]; // Extract the inner string
                let inner_mode = ANCModes::from_str(inner_str);
                if inner_mode.is_err() {
                    return None;
                }
                Some(SoundMode::NoiseCancelling(inner_mode.unwrap()))
            }
            s if s.starts_with("TransparencyMode(") && s.ends_with(")") => {
                let inner_str = &s[16..s.len() - 1]; // Extract the inner string
                let inner_mode = TransparencyModes::from_str(inner_str);
                if inner_mode.is_err() {
                    return None;
                }
                Some(SoundMode::TransparencyMode(inner_mode.unwrap()))
            }
            _ => None,
        }
    }
}

#[typeshare]
#[derive(
    Debug,
    Default,
    Display,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    EnumString,
    AsRefStr,
)]
#[serde(tag = "type", content = "value")]
pub enum ANCModes {
    #[default] // TODO: Find the default value which works across most devices
    Outdoor,
    Indoor,
    Custom(u8),
    Transport,
}

#[typeshare]
#[derive(
    Debug,
    Default,
    Display,
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    EnumString,
)]
pub enum TransparencyModes {
    #[default]
    Vocal,
    FullyTransparent,
}

#[cfg(test)]
mod soundmode {
    use crate::api::SoundMode;

    #[test]
    fn normal_mode() {
        let bytes = [2, 0, 0, 0];
        let mode = SoundMode::from_bytes(bytes);
        assert_eq!(mode, Some(SoundMode::NormalMode));
    }

    #[test]
    fn anc_transport() {
        let bytes = [0, 0, 0, 0];
        let mode = SoundMode::from_bytes(bytes);
        assert_eq!(
            mode,
            Some(SoundMode::NoiseCancelling(crate::api::ANCModes::Transport))
        );
    }

    #[test]
    fn anc_outdoor() {
        let bytes = [0, 1, 0, 0];
        let mode = SoundMode::from_bytes(bytes);
        assert_eq!(
            mode,
            Some(SoundMode::NoiseCancelling(crate::api::ANCModes::Outdoor))
        );
    }

    #[test]
    fn anc_indoor() {
        let bytes = [0, 2, 0, 0];
        let mode = SoundMode::from_bytes(bytes);
        assert_eq!(
            mode,
            Some(SoundMode::NoiseCancelling(crate::api::ANCModes::Indoor))
        );
    }

    #[test]
    fn anc_custom() {
        let bytes = [0, 3, 0, 5];
        let mode = SoundMode::from_bytes(bytes);
        assert_eq!(
            mode,
            Some(SoundMode::NoiseCancelling(crate::api::ANCModes::Custom(5)))
        );
    }

    #[test]
    fn transparency_fully_transparent() {
        let bytes = [1, 0, 0, 0];
        let mode = SoundMode::from_bytes(bytes);
        assert_eq!(
            mode,
            Some(SoundMode::TransparencyMode(
                crate::api::TransparencyModes::FullyTransparent
            ))
        );
    }

    #[test]
    fn transparency_vocal() {
        let bytes = [1, 0, 1, 0];
        let mode = SoundMode::from_bytes(bytes);
        assert_eq!(
            mode,
            Some(SoundMode::TransparencyMode(
                crate::api::TransparencyModes::Vocal
            ))
        );
    }
}
