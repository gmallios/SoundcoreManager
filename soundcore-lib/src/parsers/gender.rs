use nom::combinator::map;
use nom::error::context;
use nom::number::complete::le_u8;

use crate::models::Gender;

use super::{SoundcoreParseError, SoundcoreParseResult};

pub fn parse_gender<'a, E: SoundcoreParseError<'a>>(
    bytes: &'a [u8],
) -> SoundcoreParseResult<Gender, E> {
    context("parse_gender", map(le_u8, Gender::from_u8))(bytes)
}
