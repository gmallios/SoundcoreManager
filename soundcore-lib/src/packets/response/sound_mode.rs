use nom::{combinator::map, error::context};
use serde::{Deserialize, Serialize};

use crate::api::SoundcoreDeviceState;
use crate::packets::StateTransformationPacket;
use crate::{
    models::SoundMode,
    parsers::{parse_sound_mode, ParseError, ParseResult},
};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SoundModeUpdateResponse(pub SoundMode);

pub fn parse_sound_mode_update_packet<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<SoundModeUpdateResponse, E> {
    println!("Bytes: {:?}", bytes);
    context(
        "parse_sound_mode_update",
        map(parse_sound_mode, SoundModeUpdateResponse),
    )(bytes)
}

impl StateTransformationPacket for SoundModeUpdateResponse {
    fn transform_state(self, state: &SoundcoreDeviceState) -> SoundcoreDeviceState {
        let mut state = state.clone();
        state.sound_mode = self.0;
        state
    }
}
