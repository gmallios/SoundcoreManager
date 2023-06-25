use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::api::{BatteryLevel, ChargingStatus, EQValues, SoundMode};

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub struct SoundcoreDeviceState {
    pub eq: EQValues,
    pub sound_mode: SoundMode,
    pub charging_status: ChargingStatus,
    pub battery_level: BatteryLevel,
}
