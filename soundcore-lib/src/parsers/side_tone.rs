use nom::combinator::map;
use nom::error::context;

use crate::models::SideTone;

use super::{parse_bool, SoundcoreParseError, SoundcoreParseResult};

pub fn parse_side_tone<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<SideTone, E> {
    context("parse_side_tone", map(parse_bool, SideTone::from))(bytes)
}
