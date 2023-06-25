mod state;
pub use state::*;

use crate::api::{ResponsePackets, SoundcoreDeviceStateTransformer};

pub fn packet_to_transformer(
    packet: ResponsePackets,
) -> Option<Box<dyn SoundcoreDeviceStateTransformer + Send + Sync>> {
    match packet {
        ResponsePackets::StateUpdate(packet) => match packet {
            crate::api::ResponseStateUpdatePackets::A3951(packet) => Some(Box::new(packet)),
        },
        _ => None,
    }
}
