use nom::{number::complete::le_u8, combinator::map, sequence::tuple, error::context};

use crate::models::HearingProtect;

use super::{ParseError, parse_bool, ParseResult};

pub fn parse_hearing_protect<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<'a, HearingProtect, E> {
    context(
        "parse_hearing_protect",
        map(tuple((parse_bool, le_u8, le_u8)), |(enabled, db, freq)| {
            HearingProtect { enabled, db, freq }
        }),
    )(bytes)
}