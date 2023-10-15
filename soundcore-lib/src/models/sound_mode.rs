use serde::{Deserialize, Serialize};

use super::{ANCMode, CurrentSoundMode, CustomANC, TransparencyMode};

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct SoundMode {
    pub current: CurrentSoundMode,
    pub anc_mode: ANCMode,
    pub trans_mode: TransparencyMode,
    pub custom_anc: CustomANC,
}

impl SoundMode {
    pub fn to_bytes(&self) -> [u8; 4] {
        [
            self.current.as_u8(),
            self.anc_mode.as_u8(),
            self.trans_mode.as_u8(),
            self.custom_anc.as_u8(),
        ]
    }
}
