#![allow(non_camel_case_types)]
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use typeshare::typeshare;

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, EnumIter,
)]
#[typeshare]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeatureFlags {
    DRC,
    HEARID,
    WEAR_DETECTION,
    CUSTOM_BUTTONS,
    TOUCH_TONE,
    GAME_MODE,
    AUTO_POWER_OFF_ON,
    IN_EAR_BEEP,
    LANG_PROMPT,
    HEARING_PROTECTION,
    AMBIENT_SOUND_NOTICE,
    POWER_ON_BATTERY_NOTICE,
    SUPPORT_TWO_CONNECTIONS,
    MULTIPLE_DEVICE_LIST,
}
