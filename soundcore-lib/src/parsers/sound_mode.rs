use nom::{
    combinator::{map, map_opt},
    error::context,
    number::complete::le_u8,
    sequence::tuple,
};

use crate::models::{ANCMode, CurrentSoundMode, CustomANC, SoundMode, TransparencyMode};

use super::{ParseError, ParseResult};

pub fn parse_sound_mode<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<SoundMode, E> {
    context(
        "parse_sound_mode",
        map(
            tuple((
                parse_curr_sound_mode,
                parse_anc_mode,
                parse_trans_mode,
                parse_custom_anc,
            )),
            |(current, anc_mode, trans_mode, custom_anc)| SoundMode {
                current,
                anc_mode,
                trans_mode,
                custom_anc,
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

fn parse_anc_mode<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<ANCMode, E> {
    context("parse_anc_mode", map_opt(le_u8, ANCMode::from_u8))(bytes)
}

fn parse_trans_mode<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<TransparencyMode, E> {
    context(
        "parse_trans_mode",
        map_opt(le_u8, TransparencyMode::from_u8),
    )(bytes)
}

fn parse_custom_anc<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<CustomANC, E> {
    context("parse_custom_anc", map(le_u8, CustomANC::from_u8))(bytes)
}

#[cfg(test)]
mod anc_parser {
    use super::*;
    use crate::models::SoundMode;

    #[test]
    fn parses_valid_bytes() {
        let input = SoundMode {
            current: CurrentSoundMode::ANC,
            anc_mode: ANCMode::Outdoor,
            trans_mode: TransparencyMode::Vocal,
            custom_anc: CustomANC::from_u8(0x10),
        }
        .to_bytes();

        let expected = Ok((
            &b""[..],
            SoundMode {
                current: CurrentSoundMode::ANC,
                anc_mode: ANCMode::Outdoor,
                trans_mode: TransparencyMode::Vocal,
                custom_anc: CustomANC::from_u8(0x10),
            },
        ));
        let output = parse_sound_mode::<nom::error::VerboseError<&[u8]>>(&input);

        assert_eq!(expected, output);
    }
}
