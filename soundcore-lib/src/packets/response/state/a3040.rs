use enumflags2::{make_bitflags, BitFlags};
use nom::number::complete::le_u8;
use nom::{combinator::all_consuming, error::context, sequence::tuple};
use serde::{Deserialize, Serialize};

use crate::models::PromptLanguage;
use crate::parsers::{
    bool_parser, parse_a3040_button_model, parse_auto_power_off_on, parse_fw,
    parse_hearing_protect, parse_prompt_language, parse_single_battery, parse_sound_mode,
    parse_stereo_eq, u8_parser,
};
use crate::{
    models::{
        AmbientSoundNotice, AutoPowerOff, BassUp, DeviceColor, FirmwareVer, HearingProtect,
        InEarBeep, PowerOnBatteryNotice, SerialNumber, SingleBattery, SoundcoreFeatureFlags,
        SupportTwoCnn, ThreeDimensionalEffect, TouchTone, WearDetection, LDAC,
    },
    parsers::{parse_serial_number, ParseError, ParseResult},
};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct A3040StateResponse {
    pub battery: SingleBattery,
    pub fw: FirmwareVer,
    pub sn: SerialNumber,
    pub touch_tone: TouchTone,
    pub wear_detection: WearDetection,
    pub three_dimensional_effect: ThreeDimensionalEffect,
    // TODO: Extract type?
    pub charging_case_battery: u8,
    pub bass_up: BassUp,
    pub device_color: DeviceColor,
    pub ldac: LDAC,
    pub support_two_cnn: SupportTwoCnn,
    pub in_ear_beep: InEarBeep,
    pub prompt_language: PromptLanguage,
    pub auto_power_off: AutoPowerOff,
    pub hearing_protect: HearingProtect,
    pub ambient_sound_notice: AmbientSoundNotice,
    pub power_on_battery_notice: PowerOnBatteryNotice,
}

const A3040_FEATURE_FLAGS: BitFlags<SoundcoreFeatureFlags> = make_bitflags!(SoundcoreFeatureFlags::{
    SOUND_MODE
    | ANC_MODE
    | CUSTOM_BUTTONS
    | WEAR_DETECTION
    | EQ
    | STEREO_EQ
    // | DRC - Unknown
    // | HEARID - Unknown
    // | CUSTOM_ANC - Unknown
    | TOUCH_TONE
    | GAME_MODE
    | AUTO_POWER_OFF_ON
    | PowerOnBatteryNotice
    | AmbientSoundNotice
    | HearingProtect
    | PromptLang
    | InEarBeep
    | SupportTwoCnn
});

pub fn parse_a3040_state_response<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<A3040StateResponse, E> {
    context(
        "a3040_state_response",
        all_consuming(|bytes| {
            // The A3040 state response seems to be composed of 3 parts:
            // 1. The first part where some state data exists along with an offset
            // 2. An empty? part which length is determined by the offset
            // 3. The 3rd part where the rest of the state data exists and is read based on the offset

            let (bytes, (battery, fw, sn, _eq_idx_0, _eq_idx_1, _eq)) = tuple((
                parse_single_battery,
                parse_fw,
                parse_serial_number,
                le_u8,
                le_u8,
                parse_stereo_eq,
            ))(bytes)?;

            let (bytes, _offset) = (le_u8)(bytes)?;

            let (
                bytes,
                (
                    _custom_btn_model,
                    _sound_mode,
                    touch_tone,
                    wear_detection,
                    three_dimensional_effect,
                    charging_case_battery,
                    bass_up,
                    device_color,
                    ldac,
                    support_two_cnn,
                    in_ear_beep,
                    prompt_language,
                    auto_power_off,
                    hearing_protect,
                    ambient_sound_notice,
                    power_on_battery_notice,
                ),
            ) = tuple((
                parse_a3040_button_model,
                parse_sound_mode,
                bool_parser::<TouchTone, E>,
                bool_parser::<WearDetection, E>,
                bool_parser::<ThreeDimensionalEffect, E>,
                le_u8,
                bool_parser::<BassUp, E>,
                u8_parser::<DeviceColor, E>,
                bool_parser::<LDAC, E>,
                bool_parser::<SupportTwoCnn, E>,
                bool_parser::<InEarBeep, E>,
                parse_prompt_language,
                parse_auto_power_off_on,
                parse_hearing_protect,
                bool_parser::<AmbientSoundNotice, E>,
                bool_parser::<PowerOnBatteryNotice, E>,
            ))(bytes)?;

            Ok((
                bytes,
                A3040StateResponse {
                    battery,
                    fw,
                    sn,
                    touch_tone,
                    wear_detection,
                    three_dimensional_effect,
                    charging_case_battery,
                    bass_up,
                    device_color,
                    ldac,
                    support_two_cnn,
                    in_ear_beep,
                    prompt_language,
                    auto_power_off,
                    hearing_protect,
                    ambient_sound_notice,
                    power_on_battery_notice,
                },
            ))
        }),
    )(bytes)
}
