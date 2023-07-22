use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;

/// This struct contains the wasm-compatible state of the device
/// This will be used by the UI and must implement From/To soundcore_lib::api::SoundcoreDeviceState
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[wasm_bindgen]
pub struct SoundcoreDeviceState {
    pub sound_mode: SoundMode,
    pub eq: EQValues,
    pub battery_level: BatteryLevel,
    pub charging_status: ChargingStatus,
}

#[wasm_bindgen]
impl SoundcoreDeviceState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            sound_mode: SoundMode::new("NormalMode"),
            eq: EQValues::new(&[0, 0, 0, 0, 0, 0, 0, 0]),
            battery_level: Default::default(),
            charging_status: Default::default(),
        }
    }
}

impl From<soundcore_lib::api::SoundcoreDeviceState> for SoundcoreDeviceState {
    fn from(value: soundcore_lib::api::SoundcoreDeviceState) -> Self {
        Self {
            sound_mode: value.sound_mode.into(),
            eq: value.eq.into(),
            battery_level: BatteryLevel {
                left: value.battery_level.left,
                right: value.battery_level.right,
            },
            charging_status: ChargingStatus {
                left: value.charging_status.left,
                right: value.charging_status.right,
            },
        }
    }
}

impl Into<soundcore_lib::api::SoundcoreDeviceState> for SoundcoreDeviceState {
    fn into(self) -> soundcore_lib::api::SoundcoreDeviceState {
        soundcore_lib::api::SoundcoreDeviceState {
            sound_mode: self.sound_mode.into(),
            eq: self.eq.into(),
            charging_status: soundcore_lib::api::ChargingStatus {
                left: self.charging_status.left,
                right: self.charging_status.right,
            },
            battery_level: soundcore_lib::api::BatteryLevel {
                left: self.battery_level.left,
                right: self.battery_level.right,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, Eq, PartialEq)]
#[wasm_bindgen]
pub struct BatteryLevel {
    pub left: u8,
    pub right: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, Eq, PartialEq)]
#[wasm_bindgen]
pub struct ChargingStatus {
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[wasm_bindgen]
pub struct EQValues {
    inner: soundcore_lib::api::EQValues,
}

#[wasm_bindgen]
impl EQValues {
    #[wasm_bindgen(constructor)]
    pub fn new(values: &[i8]) -> Self {
        Self {
            inner: soundcore_lib::api::EQValues::new(values.try_into().unwrap()),
        }
    }

    #[wasm_bindgen(getter = values)]
    pub fn values(&self) -> Vec<i8> {
        self.inner.values().to_vec()
    }
}

impl From<soundcore_lib::api::EQValues> for EQValues {
    fn from(value: soundcore_lib::api::EQValues) -> Self {
        Self { inner: value }
    }
}

impl From<EQValues> for soundcore_lib::api::EQValues {
    fn from(value: EQValues) -> Self {
        value.inner
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[wasm_bindgen]
pub struct SoundMode {
    inner: soundcore_lib::api::SoundMode,
}

#[wasm_bindgen]
impl SoundMode {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &str) -> Self {
        Self {
            inner: soundcore_lib::api::SoundMode::from_str(value).unwrap(),
        }
    }

    #[wasm_bindgen(getter = value)]
    pub fn value(&self) -> String {
        self.inner.to_string()
    }
}

impl SoundMode {
    pub fn inner(&self) -> soundcore_lib::api::SoundMode {
        self.inner
    }
}

impl From<soundcore_lib::api::SoundMode> for SoundMode {
    fn from(value: soundcore_lib::api::SoundMode) -> Self {
        Self { inner: value }
    }
}

impl From<SoundMode> for soundcore_lib::api::SoundMode {
    fn from(value: SoundMode) -> Self {
        value.inner
    }
}

#[cfg(test)]
mod state_tests {

    #[test]
    fn creates_sound_mode() {
        let mode = super::SoundMode::new("NormalMode");
        assert_eq!(mode.value(), "NormalMode");
        let mode = super::SoundMode::new("NoiseCancelling(Outdoor)");
        assert_eq!(mode.value(), "NoiseCancelling(Outdoor)");
    }
}
