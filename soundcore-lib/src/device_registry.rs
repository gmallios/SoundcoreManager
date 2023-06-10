use crate::devices::api::device::SoundcoreDevice;

pub struct SoundcoreDeviceRegistry {
    devices: Vec<Box<dyn SoundcoreDevice>>,
}
