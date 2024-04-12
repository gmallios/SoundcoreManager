use nom::{combinator::all_consuming, error::context, number::complete::le_u8, sequence::tuple};
use serde::{Deserialize, Serialize};

use crate::devices::a3029_features;
use crate::parsers::{
    parse_gender, parse_sound_mode, parse_stereo_eq_configuration, ParseError, TaggedData,
    TaggedParseResult,
};
use crate::types::SupportedModels;
use crate::{
    models::{
        AgeRange, BaseHearID, DeviceFirmware, Gender, HearID, SerialNumber, SingleBattery,
        SoundMode, StereoEQConfiguration, TwsStatus,
    },
    parsers::{
        parse_base_hear_id, parse_dual_fw, parse_serial_number, parse_single_battery, u8_parser,
    },
};

use super::DeviceStateResponse;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct A3029StateResponse {
    pub tws_status: TwsStatus,
    pub battery: SingleBattery,
    pub eq: StereoEQConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: BaseHearID,
    pub sound_mode: SoundMode,
    pub fw: DeviceFirmware,
    pub sn: SerialNumber,
    pub hear_id_has_data: bool,
}

impl From<A3029StateResponse> for DeviceStateResponse {
    fn from(value: A3029StateResponse) -> Self {
        DeviceStateResponse {
            feature_set: a3029_features(),
            battery: value.battery.into(),
            sound_mode: value.sound_mode,
            eq: value.eq.into(),
            tws_status: value.tws_status.into(),
            hear_id: Some(HearID::Base(value.hear_id)),
            age_range: value.age_range.into(),
            hear_id_has_data: Some(value.hear_id_has_data),
            ..Default::default()
        }
    }
}

pub fn parse_a3029_state_response<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> TaggedParseResult<A3029StateResponse, E> {
    context(
        "a3029_state_response",
        all_consuming(|bytes| {
            let (
                bytes,
                (battery, eq, gender, hear_id_has_data, age_range, hear_id, sound_mode, fw, sn),
            ) = tuple((
                parse_single_battery,
                parse_stereo_eq_configuration(8),
                parse_gender,
                le_u8,
                u8_parser::<AgeRange, E>,
                parse_base_hear_id(8),
                parse_sound_mode,
                parse_dual_fw,
                parse_serial_number,
            ))(bytes)?;

            let hear_id_has_data = hear_id_has_data == 255;

            Ok((
                bytes,
                TaggedData {
                    tag: SupportedModels::A3029,
                    data: A3029StateResponse {
                        tws_status: TwsStatus(true),
                        battery,
                        eq,
                        gender,
                        age_range,
                        hear_id,
                        sound_mode,
                        fw: DeviceFirmware::DUAL(fw.0, fw.1),
                        sn,
                        hear_id_has_data,
                    },
                },
            ))
        }),
    )(bytes)
}

#[cfg(test)]
mod a3029_state {
    // TODO
}
