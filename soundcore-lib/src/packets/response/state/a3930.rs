use enumflags2::{make_bitflags, BitFlags};
use nom::{
    combinator::{all_consuming, opt},
    error::context,
    number::complete::le_u8,
    sequence::tuple,
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        A3909ButtonModel, AgeRange, Battery, ButtonModel, CustomHearID, DualBattery,
        EQConfiguration, Gender, HearID, SideTone, SoundMode, SoundcoreFeatureFlags,
        StereoEQConfiguration, TwsStatus,
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

const A3930_FEATURE_FLAGS: BitFlags<SoundcoreFeatureFlags> = make_bitflags!(SoundcoreFeatureFlags::{
    // TODO: Check if these are correct
    SOUND_MODE
    | ANC_MODE
    | TRANS_MODE
    | CUSTOM_ANC
    | CUSTOM_BUTTONS
    | EQ
    | STEREO_EQ
    | DRC
    | HEARID
});

impl From<A3930StateResponse> for DeviceStateResponse {
    fn from(value: A3930StateResponse) -> Self {
        DeviceStateResponse {
            feature_flags: A3930_FEATURE_FLAGS,
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
