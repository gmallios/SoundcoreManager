use enumflags2::BitFlags;
use nom::{combinator::map, error::context};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        AgeRange, Battery, ButtonModel, CustomHearID, EQConfiguration, SideTone, SoundMode,
        SoundcoreFeatureFlags, TouchTone, TwsStatus, WearDetection,
    },
    parsers::{SoundcoreParseError, SoundcoreParseResult},
};

/// This is a generalized version of the state responses for all devices
/// All device-specific state responses should be able to be converted to this type
/// Also, this must be impl Into<SoundcoreDeviceState>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub struct DeviceStateResponse {
    pub feature_flags: BitFlags<SoundcoreFeatureFlags>,
    pub battery: Battery,
    pub sound_mode: SoundMode,
    pub eq: EQConfiguration,
    pub host_device: Option<u8>,
    pub tws_status: Option<TwsStatus>,
    pub button_model: Option<ButtonModel>,
    pub side_tone: Option<SideTone>,
    pub hearid_eq_preset: Option<u16>,
    pub wear_detection: Option<WearDetection>,
    pub hear_id: Option<CustomHearID>,
    pub age_range: Option<AgeRange>,
    pub touch_tone: Option<TouchTone>,
}

// TODO: Add more parsers
pub fn parse_state_update_packet<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<DeviceStateResponse, E> {
    context("parse_state_update", |bytes| {
        map(
            parse_a3951_state_response::<'a, E>,
            DeviceStateResponse::from,
        )(bytes)
    })(bytes)
}

mod a3951;
pub use a3951::*;
