use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Sound Mode byte alignment
/// | Byte | Description           | Values |
/// |------|-----------------------|--------|
/// | 0    | Mode                  | 0 for ANC, 1 for Transparency, 2 for Normal |
/// | 1    | ANC Sub-mode          | 0/1/2/3 - Transport,Outdoor,Indoor,Custom  |
/// | 2    | Transparency Sub-mode | 0 for Fully Transparent 1 for Vocal for Transparency |
/// | 3    | Sub-mode value        | Used only in the Custom mode |

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[serde(tag = "type", content = "mode")]
pub enum SoundMode {
    NoiseCancelling(ANCModes),
    NormalMode,
    TransparencyMode(TransparencyModes),
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
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[serde(tag = "type", content = "value")]
pub enum ANCModes {
    Outdoor,
    Indoor,
    Custom(u8),
    Transport,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum TransparencyModes {
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
