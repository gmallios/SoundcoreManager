#[derive(Debug, PartialEq, Eq)]
pub enum ResponseStateUpdatePackets {
    A3951(crate::devices::a3951::packets::state_update::StateUpdatePacketResponse),
}
