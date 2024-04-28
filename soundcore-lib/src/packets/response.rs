use log::error;
use nom::error::VerboseError;

use crate::api::SoundcoreDeviceState;
use crate::parsers::TaggedData;
use crate::{
    models::ResponsePacketKind,
    parsers::{parse_and_check_checksum, parse_packet_header},
};

mod battery;
mod info;
mod sound_mode;
mod state;
mod bass_up;

pub use info::*;
pub use sound_mode::*;
pub use state::*;
pub use bass_up::*;

#[derive(Debug)]
pub enum ResponsePacket {
    DeviceState(TaggedData<DeviceStateResponse>),
    SoundModeUpdate(SoundModeUpdateResponse),
    DeviceInfo(DeviceInfoResponse),
    BassUpUpdate(BassUpUpdateResponse),
    Unknown,
}

pub trait StateTransformationPacket {
    fn transform_state(self, state: &SoundcoreDeviceState) -> SoundcoreDeviceState;
}

impl ResponsePacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, nom::Err<VerboseError<&[u8]>>> {
        let bytes = parse_and_check_checksum(bytes)?.0;
        let (bytes, packet_header) = parse_packet_header(bytes)?;

        Ok(match packet_header.kind {
            ResponsePacketKind::StateUpdate => {
                Self::DeviceState(parse_state_update_packet(bytes)?.1)
            }
            ResponsePacketKind::SoundModeUpdate => {
                Self::SoundModeUpdate(parse_sound_mode_update_packet(bytes)?.1)
            }
            ResponsePacketKind::InfoUpdate => Self::DeviceInfo(parse_device_info_packet(bytes)?.1),
            ResponsePacketKind::BassUpUpdate => Self::BassUpUpdate(parse_bass_up_update(bytes)?.1),
            _ => {
                // TODO: Have an array of Acks and handle those properly
                error!(
                    "Unexpected or unhandled packet kind {:?} and bytes {:?}",
                    packet_header.kind, bytes
                );
                ResponsePacket::Unknown
            }
        })
    }

    pub fn from_bytes_for_initial_state(
        bytes: &[u8],
    ) -> Result<Option<TaggedData<SoundcoreDeviceState>>, nom::Err<VerboseError<&[u8]>>> {
        let bytes = parse_and_check_checksum(bytes)?.0;
        let (bytes, packet_header) = parse_packet_header(bytes)?;

        Ok(match packet_header.kind {
            ResponsePacketKind::StateUpdate => {
                let tagged_state_resp = parse_state_update_packet(bytes)?.1;
                let state = ResponsePacket::DeviceState(TaggedData {
                    tag: tagged_state_resp.tag,
                    data: tagged_state_resp.data,
                })
                .transform_state(&SoundcoreDeviceState::default());
                Some(TaggedData {
                    tag: tagged_state_resp.tag,
                    data: state,
                })
            }
            _ => {
                error!(
                    "Unexpected or unhandled packet kind: {:?}",
                    packet_header.kind
                );
                None
            }
        })
    }
}

impl StateTransformationPacket for ResponsePacket {
    fn transform_state(self, state: &SoundcoreDeviceState) -> SoundcoreDeviceState {
        match self {
            ResponsePacket::SoundModeUpdate(sound_mode_update) => {
                sound_mode_update.transform_state(state)
            }
            ResponsePacket::DeviceState(state_update) => state_update.data.transform_state(state),
            ResponsePacket::BassUpUpdate(packet) => packet.transform_state(state),
            // No-op
            _ => state.clone(),
        }
    }
}

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
