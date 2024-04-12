use nom::{
    combinator::{all_consuming, opt},
    error::context,
    number::complete::le_u8,
    sequence::tuple,
};
use serde::{Deserialize, Serialize};

use crate::{
    devices::a3930_features,
    models::{
        A3909ButtonModel, AgeRange, Battery, ButtonModel, CustomHearID, DualBattery, Gender,
        HearID, SideTone, SoundMode, StereoEQConfiguration, TwsStatus,
    },
    parsers::u8_parser,
};

use crate::parsers::{
    bool_parser, parse_a3909_button_model, parse_custom_hear_id, parse_dual_battery, parse_gender,
    parse_sound_mode, parse_stereo_eq_configuration, ParseError, TaggedData, TaggedParseResult,
};
use crate::types::SupportedModels;

use super::DeviceStateResponse;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct A3930StateResponse {
    pub host_device: u8,
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub eq: StereoEQConfiguration,
    pub gender: Gender,
    pub age_range: AgeRange,
    pub hear_id: CustomHearID,
    pub button_model: A3909ButtonModel,
    pub sound_mode: SoundMode,
    pub side_tone: SideTone,
    pub hear_id_has_custom_data: bool,
    pub hear_id_eq_index: Option<(u8, u8)>, // TODO: Parse this correctly
}

impl From<A3930StateResponse> for DeviceStateResponse {
    fn from(value: A3930StateResponse) -> Self {
        DeviceStateResponse {
            feature_set: a3930_features(),
            battery: Battery::Dual(value.battery),
            sound_mode: value.sound_mode,
            host_device: Some(value.host_device),
            tws_status: Some(value.tws_status),
            button_model: Some(ButtonModel::A3909(value.button_model)),
            side_tone: Some(value.side_tone),
            hear_id: Some(HearID::Custom(value.hear_id)),
            age_range: Some(value.age_range),
            eq: value.eq.into(),
            ..Default::default()
        }
    }
}

pub fn parse_a3930_state_response<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> TaggedParseResult<A3930StateResponse, E> {
    context(
        "a3930_state_response",
        all_consuming(|bytes| {
            let (
                bytes,
                (
                    host_device,
                    tws_status,
                    battery,
                    eq,
                    gender,
                    hear_id_has_custom_data,
                    age_range,
                    hear_id,
                    button_model,
                    sound_mode,
                    side_tone,
                ),
            ) = tuple((
                le_u8,
                bool_parser::<TwsStatus, E>,
                parse_dual_battery,
                parse_stereo_eq_configuration(8),
                parse_gender,
                le_u8,
                u8_parser::<AgeRange, E>,
                parse_custom_hear_id(8),
                parse_a3909_button_model,
                parse_sound_mode,
                bool_parser::<SideTone, E>,
            ))(bytes)?;

            let hear_id_has_custom_data = hear_id_has_custom_data == 255;
            // Optional
            let (bytes, hear_id_eq_index) = opt(tuple((le_u8, le_u8)))(bytes)?;

            Ok((
                bytes,
                TaggedData {
                    tag: SupportedModels::A3930,
                    data: A3930StateResponse {
                        host_device,
                        tws_status,
                        battery,
                        eq,
                        gender,
                        age_range,
                        hear_id,
                        button_model,
                        sound_mode,
                        side_tone,
                        hear_id_has_custom_data,
                        hear_id_eq_index,
                    },
                },
            ))
        }),
    )(bytes)
}

#[cfg(test)]
mod a3930_state {
    // TODO
}
