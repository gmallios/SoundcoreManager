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
    fn from(_value: &StateUpdatePacketResponse) -> Self {
        Self {
            eq: todo!(),
            anc_mode: todo!(),
            charging_status: todo!(),
            battery_level: todo!(),
            ldac_status: todo!(),
        }
    }
}
