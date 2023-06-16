#[derive(Debug, PartialEq, Eq)]
pub enum ResponseStateUpdatePackets {
    A3951(crate::devices::a3951::packets::state_update::StateUpdatePacketResponse),
}

pub trait ResponseStateUpdatePacket {
    fn from_bytes(bytes: &[u8]) -> Option<Self>
    where
        Self: Sized;
}
