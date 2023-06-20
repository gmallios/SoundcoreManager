use super::device_state::SoundcoreDeviceState;

pub trait SoundcoreDeviceStateTransformer {
    fn transform(&self, state: &SoundcoreDeviceState) -> SoundcoreDeviceState;
}
