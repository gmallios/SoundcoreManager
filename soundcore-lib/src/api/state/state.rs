use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::models::{
    AmbientSoundNotice, AutoPowerOff, BassUp, DeviceColor, HearingProtect, InEarBeep,
    PowerOnBatteryNotice, PromptLanguage, SupportTwoCnn, ThreeDimensionalEffect, TouchTone, LDAC,
};
use crate::{
    api::DeviceFeatureSet,
    models::{
        AgeRange, Battery, ButtonModel, EQConfiguration, FirmwareVer, HearID, SerialNumber,
        SideTone, SoundMode, TwsStatus, WearDetection,
    },
};

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
    pub button_model: Option<ButtonModel>,
    pub host_device: Option<u8>,
    pub side_tone: Option<SideTone>,
    pub age_range: Option<AgeRange>,
    /// HearID
    pub hearid_eq_preset: Option<u16>,
    pub hear_id: Option<HearID>,
    pub hear_id_has_data: Option<bool>,
    // Toggles
    pub touch_tone: Option<TouchTone>,
    pub tws_status: Option<TwsStatus>,
    pub wear_detection: Option<WearDetection>,
    pub bass_up: Option<BassUp>,
    pub auto_power_off: Option<AutoPowerOff>,
    pub support_two_cnn: Option<SupportTwoCnn>,
    pub in_ear_beep: Option<InEarBeep>,
    pub ambient_sound_notice: Option<AmbientSoundNotice>,
    pub power_on_battery_notice: Option<PowerOnBatteryNotice>,
    // Other
    pub three_dimensional_effect: Option<ThreeDimensionalEffect>,
    pub device_color: Option<DeviceColor>,
    pub ldac: Option<LDAC>,
    pub prompt_language: Option<PromptLanguage>,
    pub hearing_protect: Option<HearingProtect>,
}
