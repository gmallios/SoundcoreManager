use crate::{
    api::{RequestPacket, SoundMode},
    utils,
};

/// Packet to set the sound mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SoundModeRequestPacket {
    mode: SoundMode,
}

impl SoundModeRequestPacket {
    pub fn new(mode: SoundMode) -> SoundModeRequestPacket {
        SoundModeRequestPacket { mode }
    }
}

impl RequestPacket for SoundModeRequestPacket {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0x08, 0xEE, 0x00, 0x00, 0x00, 0x06, 0x81, 0x00];
        bytes.append(&mut self.mode.bytes());
        bytes.push(utils::calculate_checksum_byte(&bytes));
        bytes
    }
}
