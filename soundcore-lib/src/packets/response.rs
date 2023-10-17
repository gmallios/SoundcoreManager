use crate::{
    models::ResponsePacketKind,
    parsers::{parse_and_check_checksum, parse_packet_header},
};

use nom::error::VerboseError;

#[derive(Debug)]
pub enum ResponsePacket {
    DeviceState(DeviceStateResponse),
    SoundModeUpdate(SoundModeUpdateResponse),
    DeviceInfo, // TODO
}

impl ResponsePacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, nom::Err<VerboseError<&[u8]>>> {
        let bytes = parse_and_check_checksum(bytes)?.0;
        let (bytes, packet_header) = parse_packet_header(bytes)?;
        println!("Packet header: {:?}", packet_header);
        Ok(match packet_header.kind {
            ResponsePacketKind::StateUpdate => {
                Self::DeviceState(parse_state_update_packet(bytes)?.1)
            }
            ResponsePacketKind::SoundModeUpdate => {
                Self::SoundModeUpdate(parse_sound_mode_update_packet(bytes)?.1)
            }
            _ => unimplemented!(),
        })
    }
}

mod info;
mod sound_mode;
mod state;

pub use info::*;
pub use sound_mode::*;
pub use state::*;

#[cfg(test)]
mod response_test {
    use super::ResponsePacket;

    const A3951_STATE_UPDATE_BYTES: [u8; 97] = [
        9, 255, 0, 0, 1, 1, 1, 97, 0, 1, 1, 5, 5, 1, 0, 254, 254, 160, 150, 130, 120, 120, 120,
        120, 120, 160, 150, 130, 120, 120, 120, 120, 120, 255, 255, 0, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 99, 1, 84, 1, 102, 1,
        84, 0, 1, 0, 0, 0, 1, 1, 6, 0, 1, 0, 0, 0, 0, 242,
    ];

    #[test]
    fn sound_mode_update() {
        let bytes = [
            0x09, 0xFF, 0x00, 0x00, 0x01, 0x06, 0x01, 0x0E, 0x00, 0x00, 0x01, 0x01, 0x06, 0x26,
        ];

        let packet = ResponsePacket::from_bytes(&bytes).unwrap();
        assert!(matches!(packet, ResponsePacket::SoundModeUpdate(_)));
    }

    #[test]
    fn a3951_state_update() {
        let packet = ResponsePacket::from_bytes(&A3951_STATE_UPDATE_BYTES).unwrap();
        assert!(matches!(packet, ResponsePacket::DeviceState(_)));
    }
}
