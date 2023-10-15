use nom::{
    combinator::{map, map_opt},
    error::context,
    number::complete::le_u16,
    sequence::pair,
};

use crate::models::{EQProfile, MonoEQConfiguration, StereoEQ, StereoEQConfiguration};

use super::{parse_mono_eq, parse_stereo_eq, SoundcoreParseError, SoundcoreParseResult};

pub fn parse_stereo_eq_configuration<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<StereoEQConfiguration, E> {
    context(
        "parse_stereo_eq_config",
        map(
            pair(parse_eq_profile, parse_stereo_eq),
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

pub fn parse_mono_eq_configuration<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<MonoEQConfiguration, E> {
    context(
        "parse_mono_eq_config",
        map(
            pair(parse_eq_profile, parse_mono_eq),
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

fn parse_eq_profile<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<EQProfile, E> {
    context("parse_mono_eq_config", map_opt(le_u16, EQProfile::from_id))(bytes)
}
