use nom::{error::context, combinator::map_opt, number::complete::le_u8};

use crate::models::PromptLanguage;

use super::{ParseError, ParseResult};

pub fn parse_prompt_language<'a, E: ParseError<'a>>(bytes: &'a [u8]) -> ParseResult<'a, PromptLanguage, E> {
    context(
        "parse_prompt_language",
        map_opt(le_u8, |prompt_language| {
            PromptLanguage::from_repr(prompt_language.into())
        }),
    )(bytes)
}