use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub struct BatteryLevel {
    left: u8,
    right: u8,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub struct ChargingStatus {
    left: bool,
    right: bool,
}
