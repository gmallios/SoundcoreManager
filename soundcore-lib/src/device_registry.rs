use std::sync::{Arc, Weak};

use async_trait::async_trait;
use weak_table::{weak_value_hash_map::Entry, WeakValueHashMap};

use crate::{
    api::{DeviceRegistry, SoundcoreDevice, SoundcoreDevices},
    bt::{ble::BLEConnectionRegistry, windows::registry::WindowsBLEConnectionRegistry},
    device_descriptor::SoundcoreDeviceDescriptor,
    devices::{
        a3951::device::A3951, match_model_id_to_uuid_set, match_name_to_model_id, SupportedModelIDs,
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
    devices: tokio::sync::Mutex<WeakValueHashMap<String, Weak<SoundcoreDevices<R::ConnType>>>>,
}

impl<R> SoundcoreDeviceRegistry<R>
where
    R: BLEConnectionRegistry + Send + Sync,
{
    pub fn new(registry: R) -> Self {
        Self {
            registry,
            devices: tokio::sync::Mutex::new(WeakValueHashMap::new()),
        }
    }

    async fn new_device(
        &self,
        name: &str,
        mac_addr: &str,
    ) -> SoundcoreResult<Option<SoundcoreDevices<R::ConnType>>> {
        let device_model = match_name_to_model_id(name);
        if device_model.is_none() {
            return Ok(None);
        }
        let uuid_set = match_model_id_to_uuid_set(&device_model.unwrap());
        if uuid_set.is_none() {
            return Ok(None);
        }
        let conn = match self
            .registry
            .connection(mac_addr, uuid_set.unwrap())
            .await?
        {
            Some(conn) => conn,
            None => return Ok(None),
        };
        match device_model.unwrap() {
            SupportedModelIDs::A3951 => {
                let device = A3951::new(conn).await?;
                Ok(Some(SoundcoreDevices::A3951(device)))
            }
            _ => todo!(),
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
        name: &str,
        mac_addr: &str,
    ) -> SoundcoreResult<Option<Arc<SoundcoreDevices<R::ConnType>>>> {
        match self.devices.lock().await.entry(mac_addr.to_owned()) {
            Entry::Occupied(e) => Ok(Some(e.get().to_owned())),
            Entry::Vacant(e) => {
                if let Some(device) = self.new_device(name, mac_addr).await? {
                    let device = Arc::new(device);
                    e.insert(device.to_owned());
                    Ok(Some(device))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
