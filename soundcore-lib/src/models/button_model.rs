use serde::{Deserialize, Serialize};
use strum::FromRepr;

use super::{A3909ButtonModel, A3040ButtonModel};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonModel {
    A3909(A3909ButtonModel),
    A3040(A3040ButtonModel),
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, FromRepr,
)]
#[repr(u8)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    VolumeUp = 0,
    VolumeDown = 1,
    PreviousSong = 2,
    NextSong = 3,
    AmbientSound = 4,
    VoiceAssistant = 5,
    PlayPause = 6,
    BassUpToggle = 7,
    Null = 8,
    ControlThreeDimensionalEffect = 9, /* Assumption, most likely incorrect  */
}

impl Action {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}
