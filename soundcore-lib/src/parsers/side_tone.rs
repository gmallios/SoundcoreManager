use nom::combinator::map;
use nom::error::context;

use crate::models::SideTone;

use super::{parse_bool, ParseError, ParseResult};

pub fn parse_side_tone<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<SideTone, E> {
    context("parse_side_tone", map(parse_bool, SideTone::from))(bytes)
}
