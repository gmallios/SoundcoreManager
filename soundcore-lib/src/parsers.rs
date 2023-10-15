use nom::{
    error::{ContextError, ParseError},
    IResult,
};

pub use a3909_button_model::*;
pub use age::*;
pub use auto_power::*;
pub use base::*;
pub use battery::*;
pub use checksum::*;
pub use eq::*;
pub use eq_configuration::*;
pub use game_mode::*;
pub use gender::*;
pub use hearid::*;
pub use packet_header::*;
pub use serial::*;
pub use sound_mode::*;
pub use touch_tone::*;
pub use wear_detection::*;

mod a3909_button_model;
mod age;
mod auto_power;
mod base;
mod battery;
mod checksum;
mod eq;
mod eq_configuration;
mod game_mode;
mod gender;
mod hearid;
mod packet_header;
mod serial;
mod side_tone;
mod sound_mode;
mod touch_tone;
mod wear_detection;

pub type SoundcoreParseResult<'a, T, E> = IResult<&'a [u8], T, E>;

pub trait SoundcoreParseError<'a>: ParseError<&'a [u8]> + ContextError<&'a [u8]> {}

impl<'a> SoundcoreParseError<'a> for nom::error::VerboseError<&'a [u8]> {}
