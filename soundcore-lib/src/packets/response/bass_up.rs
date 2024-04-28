use log::{debug, warn};
use nom::combinator::map;
use nom::error::context;
use serde::{Deserialize, Serialize};

use crate::api::SoundcoreDeviceState;
use crate::devices::parse_a3040_bass_up_update;
use crate::models::BassUp;
use crate::packets::StateTransformationPacket;
use crate::parsers::{ParseError, ParseResult};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct BassUpUpdateResponse(pub BassUp);
pub fn parse_bass_up_update<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<BassUpUpdateResponse, E> {
    context(
        "parse_bass_up_update",
        map(parse_a3040_bass_up_update, BassUpUpdateResponse),
    )(bytes)
}

impl StateTransformationPacket for BassUpUpdateResponse {
    fn transform_state(self, state: &SoundcoreDeviceState) -> SoundcoreDeviceState {
        let mut state = state.to_owned();
        match state.bass_up {
            Some(bass_up) => {
                debug!("Updating BassUp state from {:?} to {:?}", bass_up, self.0);
                state.bass_up = Some(self.0);
            }
            None => {
                warn!("BassUpUpdateResponse received without a previous BassUp state");
            }
        }
        state
    }
}
