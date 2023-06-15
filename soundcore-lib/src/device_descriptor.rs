use crate::{bt::ble::BLEDeviceDescriptor, devices::api::device_descriptor::DeviceDescriptor};

pub struct SoundcoreDeviceDescriptor<T>
where
    T: BLEDeviceDescriptor + Send + Sync,
{
    inner: T,
}

impl<T> SoundcoreDeviceDescriptor<T>
where
    T: BLEDeviceDescriptor + Send + Sync,
{
    pub fn new(t: T) -> Self {
        Self { inner: t }
    }
}

impl<T> DeviceDescriptor for SoundcoreDeviceDescriptor<T>
where
    T: BLEDeviceDescriptor + Send + Sync,
{
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn mac_address(&self) -> &str {
        self.inner.mac()
    }
}
