use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};

use crate::models::{AgeRange, Battery, ButtonModel, CustomHearID, FirmwareVer, HearID, SerialNumber, SideTone, SoundcoreFeatureFlags, SoundMode, TwsStatus, WearDetection};

/// This is a generalized version of the state for all devices
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash, Default)]
pub struct SoundcoreDeviceState {
    pub feature_flags: BitFlags<SoundcoreFeatureFlags>,
    pub battery: Battery,
    pub sound_mode: SoundMode,
    pub serial: Option<SerialNumber>,
    pub fw: Option<FirmwareVer>,
    pub drc_fw: Option<FirmwareVer>,
    pub host_device: Option<u8>,
    pub tws_status: Option<TwsStatus>,
    pub button_model: Option<ButtonModel>,
    pub side_tone: Option<SideTone>,
    pub hearid_eq_preset: Option<u16>,
    pub wear_detection: Option<WearDetection>,
    pub hear_id: Option<HearID>,
    pub age_range: Option<AgeRange>,
}
