use std::sync::Arc;

use async_trait::async_trait;

use crate::{bt::ble::BLEConnectionRegistry, error::SoundcoreResult};

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
        name: &str,
        mac_addr: &str,
    ) -> SoundcoreResult<Option<Arc<SoundcoreDevices<RegistryType::ConnType>>>>;
}
