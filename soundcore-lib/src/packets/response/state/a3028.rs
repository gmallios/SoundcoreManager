use enumflags2::{make_bitflags, BitFlags};
use nom::{
    combinator::{all_consuming, opt},
    error::context,
    number::complete::{le_u16, le_u8},
    sequence::tuple,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        A3909ButtonModel, AgeRange, BaseHearID, Battery, ButtonModel, CustomHearID, DeviceFirmware,
        DualBattery, EQConfiguration, FirmwareVer, Gender, HearID, SerialNumber, SideTone,
        SingleBattery, SoundMode, SoundcoreFeatureFlags, StereoEQConfiguration, TouchTone,
        TwsStatus, WearDetection,
    },
    parsers::{
        parse_base_hear_id, parse_bool, parse_dual_fw, parse_fw, parse_serial_number,
        parse_single_battery,
    },
};

use crate::parsers::{
    bool_parser, parse_a3909_button_model, parse_age_range, parse_custom_hear_id,
    parse_dual_battery, parse_gender, parse_sound_mode, parse_stereo_eq_configuration, ParseError,
    ParseResult,
};

use super::DeviceStateResponse;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct A3028StateResponse {
    pub tws_status: TwsStatus,
    pub battery: SingleBattery,
    pub eq: StereoEQConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BaseHearID,
    pub sound_mode: SoundMode,
    pub fw: DeviceFirmware,
    pub sn: SerialNumber,
}

const A3028_FEATURE_FLAGS: BitFlags<SoundcoreFeatureFlags> = make_bitflags!(SoundcoreFeatureFlags::{
    SOUND_MODE
    | ANC_MODE
    | TRANS_MODE
    | EQ
    | STEREO_EQ
    | HEARID
});

impl From<A3028StateResponse> for DeviceStateResponse {
    fn from(value: A3028StateResponse) -> Self {
        DeviceStateResponse {
            feature_flags: A3028_FEATURE_FLAGS,
            battery: value.battery.into(),
            sound_mode: value.sound_mode,
            eq: EQConfiguration::Stereo(value.eq).into(),
            tws_status: value.tws_status.into(),
            hear_id: Some(HearID::Base(value.hear_id)),
            age_range: value.age_range.into(),
            ..Default::default()
        }
    }
}

pub fn parse_a3028_state_response<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<A3028StateResponse, E> {
    context(
        "a3028_state_response",
        all_consuming(|bytes| {
            let (bytes, (battery, eq, gender, age_range, hear_id, sound_mode, fw, sn)) =
                tuple((
                    parse_single_battery,
                    parse_stereo_eq_configuration,
                    parse_gender,
                    parse_age_range,
                    parse_base_hear_id,
                    parse_sound_mode,
                    parse_dual_fw,
                    parse_serial_number,
                ))(bytes)?;

            Ok((
                bytes,
                A3028StateResponse {
                    tws_status: TwsStatus(true),
                    battery,
                    eq,
                    gender,
                    age_range,
                    hear_id,
                    sound_mode,
                    fw: DeviceFirmware::DUAL(fw.0, fw.1),
                    sn,
                },
            ))
        }),
    )(bytes)
}

#[cfg(test)]
mod a3028_state {
    // TODO
}
