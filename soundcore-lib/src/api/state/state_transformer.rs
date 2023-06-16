use super::device_state::SoundcoreDeviceState;

trait SoundcoreDeviceStateTransformer {
    fn transform(&self, state: &SoundcoreDeviceState) -> SoundcoreDeviceState;
}
