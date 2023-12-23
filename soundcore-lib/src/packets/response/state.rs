use enumflags2::BitFlags;
use nom::{branch::alt, combinator::map, error::context};
use serde::{Deserialize, Serialize};

use a3027::*;
use a3028::*;
use a3029::*;
use a3040::*;
use a3930::*;
use a3951::*;

use crate::models::{
    AmbientSoundNotice, AutoPowerOff, BassUp, DeviceColor, FirmwareVer, HearingProtect, InEarBeep,
    PowerOnBatteryNotice, PromptLanguage, SerialNumber, SupportTwoCnn, ThreeDimensionalEffect,
    LDAC,
};
use crate::parsers::{TaggedData, TaggedParseResult};
use crate::{
    models::{
        AgeRange, Battery, ButtonModel, EQConfiguration, HearID, SideTone, SoundMode,
        SoundcoreFeatureFlags, TouchTone, TwsStatus, WearDetection,
    },
    parsers::ParseError,
};

/// This is a generalized version of the state responses for all devices
/// All device-specific state responses should be able to be converted to this type
/// Also, this must be impl Into<SoundcoreDeviceState>
/// TODO: Split this into multiple (feature) structs?
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash, Default)]
pub struct DeviceStateResponse {
    pub feature_flags: BitFlags<SoundcoreFeatureFlags>,
    pub battery: Battery,
    pub sound_mode: SoundMode,
    pub eq: EQConfiguration,
    pub sn: Option<SerialNumber>,
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

// TODO: Add more parsers
pub fn parse_state_update_packet<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> TaggedParseResult<DeviceStateResponse, E> {
    context("parse_state_update", |bytes| {
        alt((
            map(parse_a3027_state_response::<'a, E>, |result| TaggedData {
                data: DeviceStateResponse::from(result.data),
                tag: result.tag,
            }),
            map(parse_a3028_state_response::<'a, E>, |result| TaggedData {
                data: DeviceStateResponse::from(result.data),
                tag: result.tag,
            }),
            map(parse_a3029_state_response::<'a, E>, |result| TaggedData {
                data: DeviceStateResponse::from(result.data),
                tag: result.tag,
            }),
            map(parse_a3930_state_response::<'a, E>, |result| TaggedData {
                data: DeviceStateResponse::from(result.data),
                tag: result.tag,
            }),
            map(parse_a3040_state_response::<'a, E>, |result| TaggedData {
                data: DeviceStateResponse::from(result.data),
                tag: result.tag,
            }),
            map(parse_a3951_state_response::<'a, E>, |result| TaggedData {
                data: DeviceStateResponse::from(result.data),
                tag: result.tag,
            }),
        ))(bytes)
    })(bytes)
}

mod a3027;
mod a3028;
mod a3029;
mod a3040;
mod a3930;
mod a3951;
