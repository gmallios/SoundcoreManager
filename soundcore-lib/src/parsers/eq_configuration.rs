use nom::{
    branch::alt,
    combinator::{map, map_opt},
    error::context,
    number::complete::le_u16,
    sequence::pair,
};

use crate::models::{EQProfile, MonoEQConfiguration, StereoEQ, StereoEQConfiguration};

use super::{parse_mono_eq, parse_stereo_eq, ParseError, ParseResult};

pub fn parse_stereo_eq_configuration<'a, E: ParseError<'a>>(
    eq_bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<StereoEQConfiguration, E> {
    move |bytes| {
        context(
            "parse_stereo_eq_config",
            map(
                pair(parse_eq_profile, parse_stereo_eq(eq_bands)),
                |(profile, eq)| match profile {
                    EQProfile::Custom => StereoEQConfiguration { profile, eq },
                    _ => StereoEQConfiguration {
                        profile,
                        eq: StereoEQ {
                            left: profile.eq(),
                            right: profile.eq(),
                        },
                    },
                },
            ),
        )(bytes)
    }
}

#[allow(unused)]
pub fn parse_mono_eq_configuration<'a, E: ParseError<'a>>(
    eq_bands: usize,
) -> impl Fn(&'a [u8]) -> ParseResult<MonoEQConfiguration, E> {
    move |bytes| {
        context(
            "parse_mono_eq_config",
            map(
                pair(parse_eq_profile, parse_mono_eq(eq_bands)),
                |(profile, eq)| match profile {
                    EQProfile::Custom => MonoEQConfiguration { profile, eq },
                    _ => MonoEQConfiguration {
                        profile,
                        eq: profile.eq(),
                    },
                },
            ),
        )(bytes)
    }
}

fn parse_eq_profile<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<EQProfile, E> {
    context(
        "parse_mono_eq_config",
        alt((
            // TODO: Pass a variable for the endianness at the state parser level?
            // This could be an issue if an EQProfile has 2 profiles with the same ID but different endianness (unlikely but possible)
            map_opt(le_u16, EQProfile::from_id_le),
            map_opt(le_u16, EQProfile::from_id_be),
        )),
    )(bytes)
}
