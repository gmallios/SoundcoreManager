use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{api::DeviceFeatureSet, models::{AgeRange, Battery, ButtonModel, CustomHearID, EQConfiguration, FirmwareVer, HearID, SerialNumber, SideTone, SoundMode, SoundcoreFeatureFlags, TwsStatus, WearDetection}};

/// This is a generalized version of the state for all devices
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash, Default)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct SoundcoreDeviceState {
    pub feature_set: DeviceFeatureSet,
    pub battery: Battery,
    pub sound_mode: SoundMode,
    pub eq_configuration: EQConfiguration,
    pub serial: Option<SerialNumber>,
    pub fw: Option<FirmwareVer>,
    pub host_device: Option<u8>,
    pub tws_status: Option<TwsStatus>,
    pub button_model: Option<ButtonModel>,
    pub side_tone: Option<SideTone>,
    pub hearid_eq_preset: Option<u16>,
    pub wear_detection: Option<WearDetection>,
    pub hear_id: Option<HearID>,
    pub age_range: Option<AgeRange>,
}

