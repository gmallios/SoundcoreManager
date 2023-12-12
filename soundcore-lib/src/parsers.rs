use nom::{
    error::{ContextError, ParseError as NomParseError},
    IResult,
};
use crate::types::SupportedModels;

pub use a3909_button_model::*;
pub use auto_power::*;
pub use base::*;
pub use battery::*;
pub use checksum::*;
pub use eq::*;
pub use eq_configuration::*;
pub use fw::*;
pub use gender::*;
pub use hearid::*;
pub use packet_header::*;
pub use serial::*;
pub use sound_mode::*;
pub use hearing_protect::*;
pub use prompt_language::*;

mod a3909_button_model;
mod auto_power;
mod base;
mod battery;
mod checksum;
mod eq;
mod eq_configuration;
mod fw;
mod gender;
mod hearid;
mod packet_header;
mod serial;
mod sound_mode;
mod hearing_protect;
mod prompt_language;

pub type ParseResult<'a, T, E> = IResult<&'a [u8], T, E>;
pub type TaggedParseResult<'a, T, E> = IResult<&'a [u8], TaggedData<T>, E>;

pub trait ParseError<'a>: NomParseError<&'a [u8]> + ContextError<&'a [u8]> {}

impl<'a> ParseError<'a> for nom::error::VerboseError<&'a [u8]> {}

/// Used when we need to know the successful parser's device model
pub struct TaggedData<T> {
    pub data: T,
    pub tag: SupportedModels,
}


#[cfg(test)]
#[allow(dead_code)]
pub type TestParserError<'a> = nom::error::VerboseError<&'a [u8]>;
