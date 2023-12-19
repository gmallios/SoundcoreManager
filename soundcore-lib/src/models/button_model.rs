use serde::{Deserialize, Serialize};
use strum::FromRepr;

use super::A3909ButtonModel;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonModel {
    A3909(A3909ButtonModel),
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
    Trans = 4,
    /* TODO: Refered to as "custom_trans" in the source, unknown action */
    VoiceAssistant = 5,
    PlayPause = 6,
}

impl Action {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}
