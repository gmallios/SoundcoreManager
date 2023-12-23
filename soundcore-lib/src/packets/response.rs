use nom::error::VerboseError;

pub use info::*;
pub use sound_mode::*;
pub use state::*;

use crate::{
    models::ResponsePacketKind,
    parsers::{parse_and_check_checksum, parse_packet_header},
};
use crate::parsers::TaggedData;

#[derive(Debug)]
pub enum ResponsePacket {
    DeviceState(TaggedData<DeviceStateResponse>),
    SoundModeUpdate(SoundModeUpdateResponse),
    DeviceInfo(DeviceInfoResponse),
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
            ResponsePacketKind::InfoUpdate => Self::DeviceInfo(parse_device_info_packet(bytes)?.1),
            _ => unimplemented!(),
        })
    }
}

mod battery;
mod info;
mod sound_mode;
mod state;

#[cfg(test)]
mod response_test {
    use test_data::a3951::*;

    use super::ResponsePacket;

    #[test]
    fn sound_mode_update() {
        let packet = ResponsePacket::from_bytes(&A3951_SOUND_MODE_UPDATE_BYTES).unwrap();
        assert!(matches!(packet, ResponsePacket::SoundModeUpdate(_)));
    }

    #[test]
    fn a3951_state_update() {
        let packet = ResponsePacket::from_bytes(&A3951_STATE_UPDATE_BYTES).unwrap();
        assert!(matches!(packet, ResponsePacket::DeviceState(_)));
    }

    #[test]
    fn a3951_info_update() {
        let packet = ResponsePacket::from_bytes(&A3951_INFO_UPDATE_BYTES).unwrap();
        assert!(matches!(packet, ResponsePacket::DeviceInfo(_)));
    }
}
