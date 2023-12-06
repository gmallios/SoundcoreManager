use super::{base::parse_bool, ParseError, ParseResult};
use crate::models::TouchTone;
use nom::{combinator::map, error::context};

pub fn parse_touch_tone<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<TouchTone, E> {
    context("parse_touch_tone", map(parse_bool, TouchTone::from))(bytes)
}
