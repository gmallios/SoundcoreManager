use enumflags2::BitFlags;
use nom::{branch::alt, combinator::map, error::context};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        AgeRange, Battery, ButtonModel, CustomHearID, EQConfiguration, HearID, SideTone, SoundMode,
        SoundcoreFeatureFlags, TouchTone, TwsStatus, WearDetection,
    },
    parsers::{ParseError, ParseResult},
};

/// This is a generalized version of the state responses for all devices
/// All device-specific state responses should be able to be converted to this type
/// Also, this must be impl Into<SoundcoreDeviceState>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash, Default)]
pub struct DeviceStateResponse {
    pub feature_flags: BitFlags<SoundcoreFeatureFlags>,
    pub battery: Battery,
    pub sound_mode: SoundMode,
    pub eq: EQConfiguration,
    pub button_model: Option<ButtonModel>,
    pub host_device: Option<u8>,
    pub tws_status: Option<TwsStatus>,
    pub side_tone: Option<SideTone>,
    pub wear_detection: Option<WearDetection>,
    pub age_range: Option<AgeRange>,
    pub touch_tone: Option<TouchTone>,
    /// HearID
    pub hearid_eq_preset: Option<u16>,
    pub hear_id: Option<HearID>,
    pub hear_id_has_data: Option<bool>,
}

// TODO: Add more parsers
pub fn parse_state_update_packet<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<DeviceStateResponse, E> {
    context("parse_state_update", |bytes| {
        alt((
            map(
                parse_a3951_state_response::<'a, E>,
                DeviceStateResponse::from,
            ),
            map(
                parse_a3027_state_response::<'a, E>,
                DeviceStateResponse::from,
            ),
        ))(bytes)
    })(bytes)
}

mod a3027;
mod a3028;
mod a3029;
mod a3930;
mod a3951;
mod a3040;

use a3027::*;
use a3028::*;
use a3029::*;
use a3930::*;
use a3951::*;
use a3040::*;