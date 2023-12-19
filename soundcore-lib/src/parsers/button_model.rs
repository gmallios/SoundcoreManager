use nom::combinator::map_opt;
use nom::error::context;
use nom::number::complete::le_u8;

use crate::models::Action;
use crate::parsers::{ParseError, ParseResult};

pub fn parse_button_model_action<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<Action, E> {
    context(
        "parse_button_model_action",
        map_opt(le_u8, Action::from_repr),
    )(bytes)
}
