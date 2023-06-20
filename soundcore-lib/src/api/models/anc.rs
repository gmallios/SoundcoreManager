use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Sound Mode byte alignment
/// | Byte | Description           | Values |
/// |------|-----------------------|--------|
/// | 1    | Mode                  | 0 for ANC, 1 for Transparency, 2 for Normal |
/// | 2    | Sub-mode              | N/A for Normal |
/// |      | ANC Sub-mode          | 0/1/2/3 - Transport/Outdoor,Indoor,Custom  |
/// |      | Transparency Sub-mode | 0 for Fully Transparent 1 for Vocal for Transparency |
/// | 3    | Sub-mode value        | Used only in the Custom mode |

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "mode")]
pub enum SoundModes {
    NoiseCancelling(ANCModes),
    NormalMode,
    TransparencyMode(TransparencyModes),
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum ANCModes {
    Outdoor,
    Indoor,
    Custom(u8),
    Transport,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransparencyModes {
    Vocal,
    FullyTransparent,
}
