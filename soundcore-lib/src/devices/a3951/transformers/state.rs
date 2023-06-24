use crate::{
    api::{SoundcoreDeviceState, SoundcoreDeviceStateTransformer},
    devices::a3951::packets::state_update::StateUpdatePacketResponse,
};

impl SoundcoreDeviceStateTransformer for StateUpdatePacketResponse {
    fn transform(&self, _state: &SoundcoreDeviceState) -> SoundcoreDeviceState {
        self.into()
    }
}

impl From<&StateUpdatePacketResponse> for SoundcoreDeviceState {
    fn from(value: &StateUpdatePacketResponse) -> Self {
        Self {
            eq: value.eq.0,
            sound_mode: value.sound_mode,
            charging_status: value.charging_status,
            battery_level: value.battery_level,
        }
    }
}

impl From<StateUpdatePacketResponse> for SoundcoreDeviceState {
    fn from(packet: StateUpdatePacketResponse) -> Self {
        (&packet).into()
    }
}
