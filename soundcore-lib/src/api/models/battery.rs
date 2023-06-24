use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub struct BatteryLevel {
    left: u8,
    right: u8,
}

impl BatteryLevel {
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        Self {
            left: bytes[0].clamp(0, 5),
            right: bytes[1].clamp(0, 5),
        }
    }
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub struct ChargingStatus {
    left: bool,
    right: bool,
}

impl ChargingStatus {
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        Self {
            left: bytes[0] == 1,
            right: bytes[1] == 1,
        }
    }
}

#[cfg(test)]
mod battery_tests {
    use crate::api::BatteryLevel;
    use crate::api::ChargingStatus;

    #[test]
    fn level_from_bytes() {
        let bytes = [0, 5];
        let level = BatteryLevel::from_bytes(bytes);
        assert_eq!(level, BatteryLevel { left: 0, right: 5 });
    }

    #[test]
    fn clamps_level_from_bytes() {
        let bytes = [6, 6];
        let level = BatteryLevel::from_bytes(bytes);
        assert_eq!(level, BatteryLevel { left: 5, right: 5 });
    }

    #[test]
    fn status_from_bytes() {
        let bytes = [1, 0];
        let status = ChargingStatus::from_bytes(bytes);
        assert_eq!(
            status,
            ChargingStatus {
                left: true,
                right: false
            }
        );
    }
}
