use nom::{combinator::map, error::context, number::complete::le_u8, sequence::tuple};

use crate::models::AutoPowerOff;

use super::{parse_bool, ParseError, ParseResult};

pub fn parse_auto_power_off_on<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<AutoPowerOff, E> {
    context(
        "parse_auto_power_off_on",
        map(tuple((parse_bool, le_u8)), |(enabled, index)| {
            AutoPowerOff { enabled, index: index.clamp(0, 3) }
        }),
    )(bytes)
}
