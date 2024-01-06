use enumflags2::{make_bitflags, BitFlags};
use nom::{
    combinator::{all_consuming, opt},
    error::context,
    sequence::tuple,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        AgeRange, BaseHearID, DeviceFirmware, EQConfiguration, Gender, HearID, SerialNumber,
        SingleBattery, SoundMode, SoundcoreFeatureFlags, StereoEQConfiguration,
        TwsStatus, WearDetection,
    },
    parsers::{
        parse_base_hear_id, parse_bool, parse_dual_fw, parse_serial_number,
        parse_single_battery, u8_parser,
    },
};

use crate::parsers::{bool_parser, parse_gender, parse_sound_mode, parse_stereo_eq_configuration, ParseError, TaggedData, TaggedParseResult};
use crate::types::SupportedModels;

use super::DeviceStateResponse;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct A3027StateResponse {
    pub tws_status: TwsStatus,
    pub battery: SingleBattery,
    pub eq: StereoEQConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BaseHearID,
    pub sound_mode: SoundMode,
    pub fw: DeviceFirmware,
    pub sn: SerialNumber,
    pub wear_detection: WearDetection,
    pub touch_func: bool,
}

const A3027_FEATURE_FLAGS: BitFlags<SoundcoreFeatureFlags> = make_bitflags!(SoundcoreFeatureFlags::{
    SOUND_MODE
    | ANC_MODE
    | TRANS_MODE
    | CUSTOM_ANC
    | WEAR_DETECTION
    | EQ
    | STEREO_EQ
    | HEARID
});

impl From<A3027StateResponse> for DeviceStateResponse {
    fn from(value: A3027StateResponse) -> Self {
        DeviceStateResponse {
            feature_flags: A3027_FEATURE_FLAGS,
            battery: value.battery.into(),
            sound_mode: value.sound_mode,
            eq: value.eq.into(),
            tws_status: value.tws_status.into(),
            wear_detection: value.wear_detection.into(),
            hear_id: Some(HearID::Base(value.hear_id)),
            age_range: value.age_range.into(),
            ..Default::default()
        }
    }
}

pub fn parse_a3027_state_response<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> TaggedParseResult<A3027StateResponse, E> {
    context(
        "a3027_state_response",
        all_consuming(|bytes| {
            let (
                bytes,
                (battery, eq, gender, age_range, hear_id, sound_mode, fw, sn, wear_detection),
            ) = tuple((
                parse_single_battery,
                parse_stereo_eq_configuration(8),
                parse_gender,
                u8_parser::<AgeRange, E>,
                parse_base_hear_id(8),
                parse_sound_mode,
                parse_dual_fw,
                parse_serial_number,
                bool_parser::<WearDetection, E>,
            ))(bytes)?;

            // Optional Fields
            let (bytes, touch_func) = opt(parse_bool)(bytes)?;

            Ok((
                bytes,
                TaggedData {
                    tag: SupportedModels::A3027,
                    data: A3027StateResponse {
                        tws_status: TwsStatus(true),
                        battery,
                        eq,
                        gender,
                        age_range,
                        hear_id,
                        sound_mode,
                        wear_detection,
                        fw: DeviceFirmware::DUAL(fw.0, fw.1),
                        sn,
                        touch_func: touch_func.unwrap_or(false),
                    },
                }
            ))
        }),
    )(bytes)
}

#[cfg(test)]
mod a3027_state {
    // TODO
}
