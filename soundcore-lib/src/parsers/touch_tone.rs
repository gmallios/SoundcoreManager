use super::{base::parse_bool, SoundcoreParseError, SoundcoreParseResult};
use crate::models::TouchTone;
use nom::{combinator::map, error::context};

pub fn parse_touch_tone<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<TouchTone, E> {
    context("parse_touch_tone", map(parse_bool, TouchTone::from))(bytes)
}
