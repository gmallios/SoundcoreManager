use nom::{
    bytes::complete::take,
    combinator::map,
    error::context,
    number::complete::{le_i32, le_u8},
    sequence::tuple,
};

use crate::models::{BaseHearID, CustomHearID, HearIDMusicType, HearIDType, MonoEQ, StereoEQ};

use super::{parse_bool, parse_mono_eq, parse_stereo_eq, ParseError, ParseResult};

pub fn parse_base_hear_id<'a, E: ParseError<'a>>(
    eq_bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<BaseHearID, E> {
    move |bytes| {
        context(
            "parse_base_hear_id",
            map(
                tuple((parse_bool, parse_stereo_eq(eq_bands), le_i32)),
                |(enabled, eq, time)| BaseHearID {
                    enabled,
                    values: eq,
                    time,
                },
            ),
        )(bytes)
    }
}
pub fn parse_custom_hear_id<'a, E: ParseError<'a>>(
    eq_bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<CustomHearID, E> {
    move |bytes| {
        context(
            "parse_custom_hear_id",
            map(
                tuple((
                    parse_base_hear_id(eq_bands),
                    parse_type,
                    parse_music_type,
                    take(8usize),
                    parse_mono_eq(eq_bands),
                )),
                |(base, hearid_type, hearid_music_type, l_eq, r_eq)| {
                    let eq = match l_eq[0] {
                        255 => None,
                        _ => Some(StereoEQ {
                            left: MonoEQ::from_bytes(l_eq.try_into().unwrap()),
                            right: r_eq,
                        }),
                    };
                    CustomHearID {
                        base,
                        hearid_type,
                        hearid_music_type,
                        custom_values: eq,
                    }
                },
            ),
        )(bytes)
    }
}

fn parse_type<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<HearIDType, E> {
    context("parse_hear_id_type", map(le_u8, HearIDType))(bytes)
}

fn parse_music_type<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<HearIDMusicType, E> {
    context("parse_hear_id_music_type", map(le_u8, HearIDMusicType))(bytes)
}
