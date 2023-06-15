use async_trait::async_trait;

use crate::{
    bt::ble::{BLEConnectionRegistry, BLEDeviceDescriptor},
    devices::SupportedModelIDs,
    error::SoundcoreResult,
};

use super::{device::SoundcoreDevices, device_descriptor::DeviceDescriptor};

#[async_trait]
pub trait DeviceRegistry<RegistryType>
where
    RegistryType: BLEConnectionRegistry + Send + Sync,
{
    type DescriptorType: DeviceDescriptor + Send + Sync;

    async fn descriptors(&self) -> SoundcoreResult<Vec<Self::DescriptorType>>;
    async fn device(
        &self,
        device_mode: SupportedModelIDs,
        mac_addr: &str,
    ) -> SoundcoreResult<Option<SoundcoreDevices<RegistryType::ConnType>>>;
}
