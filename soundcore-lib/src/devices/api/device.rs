use crate::devices::SupportedDevices;

use super::device_state::SoundcoreDeviceState;

pub trait SoundcoreDevice {
    /* TODO: Add Get/Set ANC,EQ,LDAC */
    fn get_supported_device(&self) -> SupportedDevices;
    fn get_device_name(&self) -> String;
    fn subscribe_state(&self) -> tokio::sync::broadcast::Receiver<SoundcoreDeviceState>;
}
