#![allow(non_camel_case_types)]
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use typeshare::typeshare;

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, EnumIter,
)]
#[bitflags]
#[repr(u32)]
#[typeshare]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SoundcoreFeatureFlags {
    SOUND_MODE,
    ANC_MODE,
    TRANS_MODE,
    CUSTOM_ANC,
    EQ,
    STEREO_EQ,
    DRC,
    HEARID,
    WEAR_DETECTION,
    CUSTOM_BUTTONS,
    TOUCH_TONE,
    GAME_MODE,
    AUTO_POWER_OFF_ON,
    // TODO: Rename
    InEarBeep,
    PromptLang,
    HearingProtect,
    AmbientSoundNotice,
    PowerOnBatteryNotice,
    SupportTwoCnn,
    MultipleDeviceList
}
