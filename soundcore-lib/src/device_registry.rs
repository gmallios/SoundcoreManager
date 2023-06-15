use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    bt::{
        ble::{BLEConnectionRegistry, BLEConnectionUuidSet},
        windows::registry::WindowsBLEConnectionRegistry,
    },
    device_descriptor::SoundcoreDeviceDescriptor,
    devices::{
        api::{
            device::{SoundcoreDevice, SoundcoreDevices},
            device_registry::DeviceRegistry,
        },
        SupportedModelIDs,
    },
    error::SoundcoreResult,
};

pub async fn create_soundcore_device_registry() -> impl BLEConnectionRegistry {
    #[cfg(target_os = "windows")]
    {
        WindowsBLEConnectionRegistry::new()
    }
    // TODO: Add macOS and Linux
}

pub struct SoundcoreDeviceRegistry<R>
where
    R: BLEConnectionRegistry + Send + Sync,
{
    registry: R,
    devices: tokio::sync::Mutex<HashMap<String, SoundcoreDevices<R::ConnType>>>,
}

impl<R> SoundcoreDeviceRegistry<R>
where
    R: BLEConnectionRegistry + Send + Sync,
{
    pub fn new(registry: R) -> Self {
        Self {
            registry,
            devices: tokio::sync::Mutex::new(HashMap::new()),
        }
    }

    async fn new_device(
        &self,
        device_model: SupportedModelIDs,
        mac_addr: &str,
        uuid_set: BLEConnectionUuidSet,
    ) -> SoundcoreResult<Option<Arc<SoundcoreDevices<R::ConnType>>>> {
        match self.registry.connection(mac_addr, uuid_set).await? {
            Some(_conn) => match device_model {
                SupportedModelIDs::A3951 => {
                    todo!()
                }
                _ => todo!(),
            },
            None => Ok(None),
        }
    }
}

#[async_trait]
impl<R> DeviceRegistry<R> for SoundcoreDeviceRegistry<R>
where
    R: BLEConnectionRegistry + Send + Sync,
{
    type DescriptorType = SoundcoreDeviceDescriptor<R::DescType>;

    async fn descriptors(&self) -> SoundcoreResult<Vec<Self::DescriptorType>> {
        Ok(self
            .registry
            .descriptors()
            .await?
            .into_iter()
            .map(SoundcoreDeviceDescriptor::new)
            .collect::<Vec<_>>())
    }

    async fn device(
        &self,
        device_model: SupportedModelIDs,
        mac_addr: &str,
    ) -> SoundcoreResult<Option<SoundcoreDevices<R::ConnType>>> {
        todo!()
    }
}
