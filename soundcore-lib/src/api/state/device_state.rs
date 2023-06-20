use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::api::{BatteryLevel, ChargingStatus};

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SoundcoreDeviceState {
    pub eq: u8,
    pub anc_mode: u8,
    pub charging_status: ChargingStatus,
    pub battery_level: BatteryLevel,
    pub ldac_status: bool,
}
