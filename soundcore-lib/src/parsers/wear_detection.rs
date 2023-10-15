use nom::{combinator::map, error::context};

use crate::models::WearDetection;

use super::{base::parse_bool, SoundcoreParseError, SoundcoreParseResult};

pub fn parse_wear_detection<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<WearDetection, E> {
    context("parse_wear_detection", map(parse_bool, WearDetection::from))(bytes)
}
