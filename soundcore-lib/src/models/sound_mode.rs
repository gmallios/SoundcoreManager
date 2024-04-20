use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::models::custom_trans_value::CustomTransparencyValue;

use super::{ANCMode, CurrentSoundMode, CustomANCValue, TransparencyMode};

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
#[typeshare]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct SoundMode {
    pub current: CurrentSoundMode,
    pub anc_mode: ANCMode,
    pub trans_mode: TransparencyMode,
    pub custom_anc: CustomANCValue,
    pub custom_trans: Option<CustomTransparencyValue>,
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

    pub fn to_bytes_with_custom_transparency(&self) -> [u8; 6] {
        [
            self.current.as_u8(),
            (self.custom_anc.as_u8() << 4) | 0x01, // TODO: 0x01 is the bit for auto ANC, some new devices support it
            self.trans_mode.as_u8(),
            self.anc_mode.as_u8(),
            0x00,
            self.custom_trans.unwrap_or_default().as_u8(),
        ]
    }
}
