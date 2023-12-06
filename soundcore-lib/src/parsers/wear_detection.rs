use nom::{combinator::map, error::context};

use crate::models::WearDetection;

use super::{base::parse_bool, ParseError, ParseResult};

pub fn parse_wear_detection<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<WearDetection, E> {
    context("parse_wear_detection", map(parse_bool, WearDetection::from))(bytes)
}
