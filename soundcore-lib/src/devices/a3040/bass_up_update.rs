use nom::{combinator::map, error::context};

use crate::{
    models::BassUp,
    parsers::{parse_bool, ParseError, ParseResult},
};

pub fn parse_a3040_bass_up_update<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<BassUp, E> {
    context("parse_a3040_bass_up_update", map(parse_bool, BassUp))(bytes)
}
