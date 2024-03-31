use nom::branch::alt;
use nom::{
    combinator::{map, map_opt},
    error::context,
    number::complete::le_u8,
    sequence::tuple,
};

use crate::models::{
    ANCMode, CurrentSoundMode, CustomANCValue, CustomTransparencyValue, SoundMode, TransparencyMode,
};

use super::{ParseError, ParseResult};

pub fn parse_sound_mode<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<SoundMode, E> {
    context("parse_sound_mode", |bytes| {
        alt((
            parse_adaptive_sound_mode_customizable_trans,
            parse_scene_based_sound_mode_non_customizable_trans,
        ))(bytes)
    })(bytes)
}

pub fn parse_scene_based_sound_mode_non_customizable_trans<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<SoundMode, E> {
    context(
        "parse_a3951_sound_mode",
        map(
            tuple((
                parse_curr_sound_mode,
                parse_scene_based_anc_mode,
                parse_non_customizable_trans_mode,
                parse_custom_anc,
            )),
            |(current, anc_mode, trans_mode, custom_anc)| SoundMode {
                current,
                anc_mode,
                trans_mode,
                custom_anc,
                custom_trans: None,
            },
        ),
    )(bytes)
}

pub fn parse_adaptive_sound_mode_customizable_trans<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<SoundMode, E> {
    context(
        "parse_adaptive_customizable_sound_mode",
        map(
            tuple((
                parse_curr_sound_mode,
                le_u8,
                parse_customizable_trans_mode,
                parse_adaptive_anc_mode,
                le_u8,
                parse_custom_trans,
            )),
            |(current_mode, custom_anc, trans_mode, anc_mode, _e, custom_trans)| {
                let custom_anc_value = CustomANCValue::from_u8(custom_anc << 4);
                let _unk = custom_anc & 0x0F;
                SoundMode {
                    current: current_mode,
                    anc_mode,
                    trans_mode,
                    custom_trans: Some(custom_trans),
                    custom_anc: custom_anc_value,
                }
            },
        ),
    )(bytes)
}

fn parse_curr_sound_mode<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<CurrentSoundMode, E> {
    context(
        "parse_curr_sound_mode",
        map_opt(le_u8, CurrentSoundMode::from_u8),
    )(bytes)
}

fn parse_scene_based_anc_mode<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<ANCMode, E> {
    context(
        "parse_scene_based_anc_mode",
        map_opt(le_u8, ANCMode::from_u8_scene_based),
    )(bytes)
}

fn parse_adaptive_anc_mode<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<ANCMode, E> {
    context(
        "parse_adaptive_anc_mode",
        map_opt(le_u8, ANCMode::from_u8_adaptive),
    )(bytes)
}

fn parse_non_customizable_trans_mode<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<TransparencyMode, E> {
    context(
        "parse_non_customizable_trans_mode",
        map_opt(le_u8, TransparencyMode::from_u8_non_customizable),
    )(bytes)
}

fn parse_customizable_trans_mode<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<TransparencyMode, E> {
    context(
        "parse_customizable_trans_mode",
        map_opt(le_u8, TransparencyMode::from_u8_customizable),
    )(bytes)
}

fn parse_custom_anc<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<CustomANCValue, E> {
    context("parse_custom_anc", map(le_u8, CustomANCValue::from_u8))(bytes)
}

fn parse_custom_trans<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<CustomTransparencyValue, E> {
    context(
        "parse_custom_trans",
        map(le_u8, CustomTransparencyValue::from_u8),
    )(bytes)
}

#[cfg(test)]
mod test {
    use crate::models::{
        AdaptiveANCMode, CustomizableTransparencyMode, NonCustomizableTransparencyMode,
        SceneBasedANCMode,
    };

    use super::*;

    #[test]
    fn should_parse_a3040_sound_mode() {
        let bytes = [0x0, 0x51, 0x1, 0x1, 0x0, 0x5];
        let output = super::parse_sound_mode::<nom::error::VerboseError<&[u8]>>(&bytes).unwrap();

        assert_eq!(
            SoundMode {
                current: CurrentSoundMode::ANC,
                anc_mode: ANCMode::Adaptive(AdaptiveANCMode::Adaptive),
                trans_mode: TransparencyMode::Customizable(CustomizableTransparencyMode::Custom),
                custom_anc: CustomANCValue(0xA),
                custom_trans: Some(CustomTransparencyValue(0x5)),
            },
            output.1
        );
    }

    #[test]
    fn should_parse_a3951_sound_mode() {
        let bytes = [0x00, 0x01, 0x01, 0x06];
        let output = super::parse_sound_mode::<nom::error::VerboseError<&[u8]>>(&bytes).unwrap();
        assert_eq!(
            SoundMode {
                current: CurrentSoundMode::ANC,
                anc_mode: ANCMode::SceneBased(SceneBasedANCMode::Outdoor),
                custom_anc: CustomANCValue(0x6),
                custom_trans: None,
                trans_mode: TransparencyMode::NonCustomizable(
                    NonCustomizableTransparencyMode::Vocal
                ),
            },
            output.1
        );
    }
}
