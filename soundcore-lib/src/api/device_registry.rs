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

    /// Returns a list of Device descriptors after scanning.
    async fn descriptors(&self) -> SoundcoreResult<Vec<Self::DescriptorType>>;
    /// Attempts to connect to a Device with the given MAC address and name.
    /// The name is used to match to a model ID.
    async fn device(
        &self,
        name: &str,
        mac_addr: &str,
    ) -> SoundcoreResult<Option<Arc<SoundcoreDevices<RegistryType::ConnType>>>>;
}
