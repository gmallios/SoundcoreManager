use nom::combinator::map;
use nom::error::context;

use crate::api::SoundcoreDeviceState;
use crate::devices::parse_a3040_eq_info_update;
use serde::{Deserialize, Serialize};

use crate::models::EQProfile;
use crate::packets::StateTransformationPacket;
use crate::parsers::{ParseError, ParseResult};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct EqInfoUpdate(pub EQProfile);

pub fn parse_eq_info_update<'a, E: ParseError<'a>>(
    bytes: &'a [u8],
) -> ParseResult<EqInfoUpdate, E> {
    context(
        "parse_eq_info_update",
        map(parse_a3040_eq_info_update, EqInfoUpdate),
    )(bytes)
}

impl StateTransformationPacket for EqInfoUpdate {
    fn transform_state(self, state: &SoundcoreDeviceState) -> SoundcoreDeviceState {
        let mut state = state.clone();
        state.eq_configuration.set_profile(self.0);
        state
    }
}
