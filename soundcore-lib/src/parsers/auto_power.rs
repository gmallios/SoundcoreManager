use nom::{combinator::map, error::context, number::complete::le_u8, sequence::tuple};

use crate::models::AutoPowerOffOn;

use super::{parse_bool, SoundcoreParseError, SoundcoreParseResult};

pub fn parse_auto_power_off_on<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<AutoPowerOffOn, E> {
    context(
        "parse_auto_power_off_on",
        map(tuple((parse_bool, le_u8)), |(enabled, index)| {
            AutoPowerOffOn { enabled, index }
        }),
    )(bytes)
}
