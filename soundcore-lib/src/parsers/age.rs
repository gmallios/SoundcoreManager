use nom::{combinator::map, error::context, number::complete::le_u8};

use crate::models::AgeRange;

use super::{ParseError, ParseResult};

pub fn parse_age_range<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<AgeRange, E> {
    context("parse_age_range", map(le_u8, AgeRange::from_u8))(bytes)
}
