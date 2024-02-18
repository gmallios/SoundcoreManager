use crate::error::SoundcoreLibError;
use crate::models::{
    Battery, EQConfiguration, MonoEQ, SideTone, SoundMode, TouchTone, TwsStatus, WearDetection,
};
use crate::packets::ResponsePacket;
use crate::types::{ANCProfile, BatteryCharging, BatteryLevel, DeviceStatus, EQWave};

impl TryInto<DeviceStatus> for ResponsePacket {
    type Error = SoundcoreLibError;
    fn try_into(self) -> Result<DeviceStatus, Self::Error> {
        match self {
            ResponsePacket::DeviceState(state) => {
                let state = state.data;
                let (battery_level, battery_charging) = state.battery.into();
                let (left_eq, right_eq) = state.eq.into();
                Ok(DeviceStatus {
                    host_device: state.host_device.unwrap_or(0),
                    tws_status: state.tws_status.unwrap_or(TwsStatus(false)).0,
                    battery_level,
                    battery_charging,
                    anc_status: ANCProfile::from(state.sound_mode),
                    side_tone_enabled: state.side_tone.unwrap_or(SideTone(false)).0,
                    wear_detection_enabled: state.wear_detection.unwrap_or(WearDetection(false)).0,
                    touch_tone_enabled: state.touch_tone.unwrap_or(TouchTone(false)).0,
                    left_eq,
                    right_eq,
                    // TODO: Use actual HearID data
                    hearid_enabled: false,
                    left_hearid: EQWave::default(),
                    right_hearid: EQWave::default(),
                    left_hearid_customdata: EQWave::default(),
                    right_hearid_customdata: EQWave::default(),
                })
            }
            _ => Err(SoundcoreLibError::IncompatibleResponse),
        }
    }
}

impl From<SoundMode> for ANCProfile {
    fn from(mode: SoundMode) -> Self {
        let bytes = mode.to_bytes();

        ANCProfile {
            option: bytes[0],
            anc_option: bytes[1],
            transparency_option: bytes[2],
            anc_custom: bytes[3],
        }
    }
}

impl From<Battery> for (BatteryLevel, BatteryCharging) {
    fn from(val: Battery) -> Self {
        match val {
            Battery::Single(batt) => {
                let left = BatteryLevel {
                    left: batt.level,
                    right: 0,
                };
                let right = BatteryCharging {
                    left: batt.charging,
                    right: false,
                };
                (left, right)
            }
            Battery::Dual(batt) => {
                let left = BatteryLevel {
                    left: batt.left.level,
                    right: batt.right.level,
                };
                let right = BatteryCharging {
                    left: batt.left.charging,
                    right: batt.right.charging,
                };
                (left, right)
            }
        }
    }
}

// Maybe figure out how to do this better and test it
impl From<MonoEQ> for EQWave {
    fn from(val: MonoEQ) -> Self {
        EQWave::decode(&val.to_bytes(8usize)).unwrap()
    }
}

impl From<EQConfiguration> for (EQWave, EQWave) {
    fn from(val: EQConfiguration) -> Self {
        match val {
            EQConfiguration::Stereo(stereo) => {
                let left = stereo.eq.left.into();
                let right = stereo.eq.right.into();
                (left, right)
            }
            EQConfiguration::Mono(mono) => {
                let left = mono.eq.into();
                let right = EQWave::default();
                (left, right)
            }
        }
    }
}
