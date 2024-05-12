use nom::{
    combinator::map,
    error::context,
    number::complete::{le_i32, le_u8},
    sequence::tuple,
};
use nom::number::complete::le_u16;

use crate::models::{BaseHearID, CustomHearID, HearIDMusicType, HearIDType};

use super::{parse_bool, parse_stereo_eq, ParseError, ParseResult};

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
                    parse_stereo_eq(eq_bands),
                )),
                |(base, hearid_type, hearid_music_type, eq)| {
                    let has_set_custom_values = *eq.left.values.get(0).unwrap() == 255;
                    CustomHearID {
                        base,
                        hearid_type,
                        hearid_music_type,
                        custom_values: eq,
                        has_set_custom_values,
                        hear_id_eq_index: None,
                    }
                },
            ),
        )(bytes)
    }
}

pub fn parse_custom_hear_id_with_eq_index<'a, E: ParseError<'a>>(
    eq_bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<CustomHearID, E> {
    move |bytes| {
        context(
            "parse_custom_hear_id_with_eq_index",
            map(
                tuple((parse_custom_hear_id(eq_bands), le_u16)),
                |(mut custom_hear_id, hear_id_eq_index)| {
                    custom_hear_id.hear_id_eq_index = Some(hear_id_eq_index);
                    custom_hear_id
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
