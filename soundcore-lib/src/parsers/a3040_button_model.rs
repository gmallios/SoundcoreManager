use nom::combinator::map;
use nom::error::context;
use nom::sequence::tuple;

use crate::models::A3040ButtonModel;
use crate::parsers::button_model::parse_button_model_action;
use crate::parsers::{ParseError, ParseResult};

pub fn parse_a3040_button_model<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<A3040ButtonModel, E> {
    context(
        "parse_a3040_button_model",
        map(
            tuple((parse_button_model_action, parse_button_model_action)),
            |(single_click, double_click)| A3040ButtonModel {
                single_click,
                double_click,
            },
        ),
    )(bytes)
}
