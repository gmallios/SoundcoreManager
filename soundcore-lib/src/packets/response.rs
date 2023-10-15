use crate::{
    models::ResponsePacketKind,
    parsers::{parse_and_check_checksum, parse_packet_header},
};

use nom::error::{VerboseError};

pub enum ResponsePacket {
    DeviceState(DeviceStateResponse),
    DeviceInfo, // TODO
}

impl ResponsePacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, nom::Err<VerboseError<&[u8]>>> {
        let bytes = parse_and_check_checksum(bytes)?.0;
        let (bytes, packet_header) = parse_packet_header(bytes)?;
        Ok(match packet_header.kind {
            ResponsePacketKind::StateUpdate => {
                Self::DeviceState(parse_state_update_packet(bytes)?.1)
            }
            _ => unimplemented!(),
        })
    }
}

mod info;
mod state;

pub use info::*;
pub use state::*;
