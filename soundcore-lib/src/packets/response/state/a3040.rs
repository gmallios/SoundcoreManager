use std::num::NonZeroUsize;

use enumflags2::{make_bitflags, BitFlags};
use nom::{bits, combinator::all_consuming, error::context, sequence::tuple};
use nom::{bytes::complete::take, number::complete::le_u8};
use serde::{Deserialize, Serialize};

use crate::models::{A3040ButtonModel, ButtonModel, PromptLanguage, TwsStatus, EQConfiguration, StereoEQConfiguration};
use crate::packets::DeviceStateResponse;
use crate::parsers::{
    bool_parser, parse_a3040_button_model, parse_auto_power_off_on, parse_fw,
    parse_hearing_protect, parse_mono_eq, parse_prompt_language, parse_single_battery,
    parse_sound_mode, parse_stereo_eq, u8_parser, TaggedData, TaggedParseResult, parse_stereo_eq_configuration,
};
use crate::types::SupportedModels;
use crate::{
    models::{
        AmbientSoundNotice, AutoPowerOff, BassUp, DeviceColor, FirmwareVer, HearingProtect,
        InEarBeep, PowerOnBatteryNotice, SerialNumber, SingleBattery, SoundcoreFeatureFlags,
        SupportTwoCnn, ThreeDimensionalEffect, TouchTone, WearDetection, LDAC,
    },
    parsers::{parse_serial_number, ParseError},
};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct A3040StateResponse {
    pub feature_flags: BitFlags<SoundcoreFeatureFlags>,
    pub battery: SingleBattery,
    pub fw: FirmwareVer,
    pub sn: SerialNumber,
    pub eq: StereoEQConfiguration,
    pub touch_tone: TouchTone,
    pub wear_detection: WearDetection,
    pub three_dimensional_effect: ThreeDimensionalEffect,
    pub button_model: A3040ButtonModel,
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
    | POWER_ON_BATTERY_NOTICE
    | AMBIENT_SOUND_NOTICE
    | HEARING_PROTECTION
    | LANG_PROMPT
    | IN_EAR_BEEP
    | SUPPORT_TWO_CONNECTIONS
});

// TODO: Figure out what the bytes remaining are so this can be all_consuming
pub fn parse_a3040_state_response<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> TaggedParseResult<A3040StateResponse, E> {
    context("a3040_state_response", |bytes: &'a [u8]| {
        let (bytes, (battery, fw, sn,eq, offset, button_model)) =
            tuple((
                parse_single_battery,
                parse_fw,
                parse_serial_number,
                parse_stereo_eq_configuration(10), // TODO: We have mono eq here, but it appears to be duplicated (bytes are not 1-1, DRC?)
                le_u8,
                parse_a3040_button_model,
            ))(bytes)?;

        let (bytes, _ignored) = take(offset as usize - 1)(bytes)?;
        let (
            bytes,
            (
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

        println!("Remaining bytes: {:x?}", bytes);
        Ok((
            bytes,
            TaggedData {
                tag: SupportedModels::A3040,
                data: A3040StateResponse {
                    feature_flags: A3040_FEATURE_FLAGS,
                    battery,
                    fw,
                    sn,
                    eq,
                    touch_tone,
                    wear_detection,
                    button_model,
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
            },
        ))
    })(&bytes)
}

impl From<A3040StateResponse> for DeviceStateResponse {
    fn from(value: A3040StateResponse) -> Self {
        Self {
            feature_flags: value.feature_flags,
            battery: value.battery.into(),
            fw: value.fw.into(),
            sn: Some(value.sn),
            touch_tone: Some(value.touch_tone),
            wear_detection: Some(value.wear_detection),
            three_dimensional_effect: Some(value.three_dimensional_effect),
            bass_up: Some(value.bass_up),
            device_color: Some(value.device_color),
            ldac: Some(value.ldac),
            support_two_cnn: Some(value.support_two_cnn),
            in_ear_beep: Some(value.in_ear_beep),
            prompt_language: Some(value.prompt_language),
            auto_power_off: Some(value.auto_power_off),
            hearing_protect: Some(value.hearing_protect),
            ambient_sound_notice: Some(value.ambient_sound_notice),
            power_on_battery_notice: Some(value.power_on_battery_notice),
            button_model: Some(ButtonModel::A3040(value.button_model)),
            tws_status: Some(TwsStatus(true)),
            eq: value.eq.into(),
            ..Default::default()
        }
    }
}
