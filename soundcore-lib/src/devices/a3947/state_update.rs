use nom::bytes::complete::take;
use nom::error::context;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::tuple;
use serde::{Deserialize, Serialize};

use crate::models::{
    AgeRange, AutoPowerOff, ChargingCaseBattery, CustomButtonWearEnable, DeviceColor, DualBattery,
    FirmwareVer, HearingProtect, InEarBeep, LDAC, LeakyCompensation, MediaTone, SerialNumber,
    SideTone, SoundMode, StereoEQConfiguration, SupportTwoCnn, TouchTone, TwsStatus, WearDetection,
};
use crate::packets::DeviceStateResponse;
use crate::parsers::{
    bool_parser, parse_adaptive_sound_mode_customizable_trans, parse_auto_power_off_on,
    parse_custom_hear_id_with_eq_index, parse_dual_battery, parse_dual_fw, parse_fw,
    parse_hearing_protect, parse_serial_number, parse_stereo_eq_configuration, ParseError,
    TaggedData, TaggedParseResult, u8_parser,
};
use crate::types::KnownProductCodes;

// TODO: Implement button model, a new model is needed
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct A3947StateResponse {
    pub host_device: u8,
    pub tws_status: TwsStatus,
    pub battery: DualBattery,
    pub fw: FirmwareVer,
    pub sn: SerialNumber,
    pub charging_case_fw: Option<FirmwareVer>,
    pub eq: StereoEQConfiguration,
    pub sound_mode: SoundMode,
    pub charging_case_battery: ChargingCaseBattery,
    pub device_color: DeviceColor,
    pub leaky_compensation: LeakyCompensation,
    pub side_tone: SideTone,
    pub media_tone: MediaTone,
    pub touch_tone: TouchTone,
    pub wear_detection: WearDetection,
    pub ldac: LDAC,
    pub support_two_cnn: SupportTwoCnn,
    pub three_dimensional_effective_mode: u8, // 0-5
    pub hearing_protect: HearingProtect,
    pub in_ear_beep: InEarBeep,
    pub auto_power_off: AutoPowerOff,
    pub custom_button_wear_enable: CustomButtonWearEnable,
}
pub fn parse_a3947_state_update<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> TaggedParseResult<A3947StateResponse, E> {
    context("parse_a3947_state_update", |bytes| {
        let (bytes, (host_device, tws_status, battery, fw, sn)) = tuple((
            le_u8,
            bool_parser::<TwsStatus, E>,
            parse_dual_battery,
            parse_dual_fw,
            parse_serial_number,
        ))(bytes)?;
        let has_charging_case_fw = ((fw.0.major() + fw.0.minor()) / 100) % 2 == 0;
        let (bytes, charging_case_fw) = if has_charging_case_fw {
            let (bytes, charging_case_fw) = parse_fw(bytes)?;
            (bytes, Some(charging_case_fw))
        } else {
            (bytes, None)
        };
        let (bytes, eq) = parse_stereo_eq_configuration(10)(bytes)?;
        let (bytes, age_range) = u8_parser::<AgeRange, E>(bytes)?;
        let (bytes, hear_id) = parse_custom_hear_id_with_eq_index(10)(bytes)?;
        let (bytes, custom_btn_selected) = le_u8(bytes)?;
        let (bytes, button_model_bytes) = take(16usize)(bytes)?;
        println!("Bytes: {:X?}", bytes);
        let (bytes, sound_mode) = parse_adaptive_sound_mode_customizable_trans(bytes)?;
        let (bytes, (personal_anc_test_time, personal_anc_volume_db, personal_anc_result_index)) =
            tuple((le_u32, le_u8, le_u8))(bytes)?;

        let (
            bytes,
            (
                charging_case_battery,
                device_color,
                leaky_compensation,
                side_tone,
                media_tone,
                touch_tone,
                wear_detection,
                ldac,
                support_two_cnn,
                three_dimensional_effective_mode,
                hearing_protect,
                wear_detection_2,
                in_ear_beep,
                auto_power_off,
                custom_button_wear_enable,
            ),
        ) = tuple((
            u8_parser::<ChargingCaseBattery, E>,
            u8_parser::<DeviceColor, E>,
            bool_parser::<LeakyCompensation, E>,
            bool_parser::<SideTone, E>,
            bool_parser::<MediaTone, E>,
            bool_parser::<TouchTone, E>,
            bool_parser::<WearDetection, E>,
            bool_parser::<LDAC, E>,
            bool_parser::<SupportTwoCnn, E>,
            le_u8,
            parse_hearing_protect,
            bool_parser::<WearDetection, E>,
            bool_parser::<InEarBeep, E>,
            parse_auto_power_off_on,
            bool_parser::<CustomButtonWearEnable, E>,
        ))(bytes)?;

        Ok((
            bytes,
            TaggedData {
                tag: KnownProductCodes::A3947,
                data: A3947StateResponse {
                    host_device,
                    tws_status,
                    battery,
                    fw: fw.0,
                    sn,
                    charging_case_fw,
                    eq,
                    sound_mode,
                    charging_case_battery,
                    device_color,
                    leaky_compensation,
                    side_tone,
                    media_tone,
                    touch_tone,
                    wear_detection,
                    ldac,
                    support_two_cnn,
                    three_dimensional_effective_mode,
                    hearing_protect,
                    in_ear_beep,
                    auto_power_off,
                    custom_button_wear_enable,
                },
            },
        ))
    })(bytes)
}

impl From<A3947StateResponse> for DeviceStateResponse {
    fn from(value: A3947StateResponse) -> Self {
        DeviceStateResponse {
            // TODO: add feature set
            feature_set: Default::default(),
            battery: value.battery.into(),
            sound_mode: value.sound_mode,
            eq: value.eq.into(),
            sn: Some(value.sn),
            fw: Some(value.fw),
            button_model: None,
            host_device: Some(value.host_device),
            side_tone: Some(value.side_tone),
            age_range: None,
            hearid_eq_preset: None,
            hear_id: None,
            hear_id_has_data: None,
            touch_tone: Some(value.touch_tone),
            tws_status: Some(value.tws_status),
            wear_detection: Some(value.wear_detection),
            bass_up: None,
            auto_power_off: Some(value.auto_power_off),
            support_two_cnn: Some(value.support_two_cnn),
            in_ear_beep: Some(value.in_ear_beep),
            ambient_sound_notice: None,
            power_on_battery_notice: None,
            three_dimensional_effect: None,
            device_color: Some(value.device_color),
            ldac: Some(value.ldac),
            prompt_language: None,
            hearing_protect: Some(value.hearing_protect),
        }
    }
}
