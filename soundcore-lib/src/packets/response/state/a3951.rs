use enumflags2::{make_bitflags, BitFlags};
use nom::{
    combinator::{all_consuming, opt},
    error::context,
    number::complete::{le_u16, le_u8},
    sequence::tuple,
};
use serde::{Deserialize, Serialize};

use crate::models::{
    A3909ButtonModel, AgeRange, Battery, ButtonModel, CustomHearID, DualBattery, Gender, SideTone,
    SoundMode, SoundcoreFeatureFlags, StereoEQConfiguration, TouchTone, TwsStatus, WearDetection,
};

use crate::parsers::{
    bool_parser, parse_a3909_button_model, parse_age_range, parse_custom_hear_id,
    parse_dual_battery, parse_gender, parse_sound_mode, parse_stereo_eq_configuration,
    SoundcoreParseError, SoundcoreParseResult,
};

use super::DeviceStateResponse;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct A3951StateResponse {
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
    pub wear_detection: WearDetection,
    pub touch_tone: TouchTone,
    pub hearid_eq_preset: Option<u16>,
    pub new_battery: Option<(u8, u8)>,
}

const A3951_FEATURE_FLAGS: BitFlags<SoundcoreFeatureFlags> = make_bitflags!(SoundcoreFeatureFlags::{
    SOUND_MODE
    | ANC_MODE
    | TRANS_MODE
    | CUSTOM_ANC
    | CUSTOM_BUTTONS
    | WEAR_DETECTION
    | EQ
    | STEREO_EQ
    | DRC
    | HEARID
    | TOUCH_TONE
});

impl From<A3951StateResponse> for DeviceStateResponse {
    fn from(value: A3951StateResponse) -> Self {
        DeviceStateResponse {
            feature_flags: A3951_FEATURE_FLAGS,
            battery: Battery::Dual(value.battery),
            sound_mode: value.sound_mode,
            host_device: Some(value.host_device),
            tws_status: Some(value.tws_status),
            button_model: Some(ButtonModel::A3909(value.button_model)),
            side_tone: Some(value.side_tone),
            hearid_eq_preset: value.hearid_eq_preset,
            wear_detection: Some(value.wear_detection),
            hear_id: Some(value.hear_id),
            age_range: Some(value.age_range),
            touch_tone: Some(value.touch_tone),
            eq: crate::models::EQConfiguration::Stereo(value.eq),
        }
    }
}

pub fn parse_a3951_state_response<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<A3951StateResponse, E> {
    context(
        "a3951_state_response",
        all_consuming(|bytes| {
            let (
                bytes,
                (
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
                    wear_detection,
                    touch_tone,
                ),
            ) = tuple((
                le_u8,
                bool_parser::<TwsStatus, E>,
                parse_dual_battery,
                parse_stereo_eq_configuration,
                parse_gender,
                parse_age_range,
                parse_custom_hear_id,
                parse_a3909_button_model,
                parse_sound_mode,
                bool_parser::<SideTone, E>,
                bool_parser::<WearDetection, E>,
                bool_parser::<TouchTone, E>,
            ))(bytes)?;

            // Optional Fields
            let (bytes, hearid_eq_preset) = opt(le_u16)(bytes)?;
            let (bytes, new_battery) = opt(tuple((le_u8, le_u8)))(bytes)?;
            let (bytes, unknown) = opt(le_u8)(bytes)?; // TODO: Unknown field

            Ok((
                bytes,
                A3951StateResponse {
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
                    wear_detection,
                    touch_tone,
                    hearid_eq_preset,
                    new_battery,
                },
            ))
        }),
    )(bytes)
}

#[cfg(test)]
mod a3951_state {

    use super::*;

    const RESP_BYTES: [u8; 86] = [
        1, 1, 5, 5, 1, 0, 254, 254, 160, 150, 130, 120, 120, 120, 120, 120, 160, 150, 130, 120,
        120, 120, 120, 120, 255, 255, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 1, 99, 1, 84, 1, 102, 1, 84, 0, 1, 0, 0, 0, 1, 1, 6, 0,
        1, 0, 0, 0,
    ];
    const ORIG_RESP_BYTES: [u8; 97] = [
        9, 255, 0, 0, 1, 1, 1, 97, 0, 1, 1, 5, 5, 1, 0, 254, 254, 160, 150, 130, 120, 120, 120,
        120, 120, 160, 150, 130, 120, 120, 120, 120, 120, 255, 255, 0, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 99, 1, 84, 1, 102, 1,
        84, 0, 1, 0, 0, 0, 1, 1, 6, 0, 1, 0, 0, 0, 0, 242,
    ];

    #[test]
    fn should_parse_state_same_as_old() {
        let (remaining, state) =
            parse_a3951_state_response::<nom::error::VerboseError<&[u8]>>(&RESP_BYTES).unwrap();
        let old_state = crate::devices::A3951::device_status(&ORIG_RESP_BYTES);

        assert!(remaining.is_empty());

        assert_eq!(state.host_device, old_state.host_device);
        assert_eq!(state.tws_status.0, old_state.tws_status);
        assert_eq!(state.battery.left.charging, old_state.battery_charging.left);
        assert_eq!(state.battery.left.level, old_state.battery_level.left);
        assert_eq!(
            state.battery.right.charging,
            old_state.battery_charging.right
        );
        assert_eq!(state.battery.right.level, old_state.battery_level.right);
        assert_eq!(
            state.sound_mode.anc_mode.as_u8(),
            old_state.anc_status.anc_option
        );
        assert_eq!(
            state.sound_mode.custom_anc.as_u8(),
            old_state.anc_status.anc_custom
        );
        assert_eq!(
            state.sound_mode.anc_mode.as_u8(),
            old_state.anc_status.anc_option
        );
        assert_eq!(
            state.sound_mode.trans_mode.as_u8(),
            old_state.anc_status.transparency_option
        );
    }
}
