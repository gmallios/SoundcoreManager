use nom::{combinator::map, error::context, number::complete::le_u8, sequence::tuple};

use crate::models::HearingProtect;

use super::{parse_bool, ParseError, ParseResult};

pub fn parse_hearing_protect<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<'a, HearingProtect, E> {
    context(
        "parse_hearing_protect",
        map(tuple((parse_bool, le_u8, le_u8)), |(enabled, db, freq)| {
            HearingProtect { enabled, db, freq }
        }),
    )(bytes)
}
