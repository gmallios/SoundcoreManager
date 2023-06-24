use crate::{api::DeviceDescriptor, bt::ble::BLEDeviceDescriptor};

#[derive(Debug, Clone)]
pub struct SoundcoreDeviceDescriptor<T>
where
    T: BLEDeviceDescriptor + Send + Sync + Clone,
{
    inner: T,
}

impl<T> SoundcoreDeviceDescriptor<T>
where
    T: BLEDeviceDescriptor + Send + Sync + Clone,
{
    pub fn new(t: T) -> Self {
        Self { inner: t }
    }
}

impl<T> DeviceDescriptor for SoundcoreDeviceDescriptor<T>
where
    T: BLEDeviceDescriptor + Send + Sync + Clone,
{
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn mac_address(&self) -> &str {
        self.inner.mac()
    }
}
