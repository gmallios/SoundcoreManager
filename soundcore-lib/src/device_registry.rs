use std::collections::HashMap;

use crate::{
    bt::{
        ble::{BLEConnectionRegistry, BLEConnectionUuidSet},
        windows::registry::WindowsBLEConnectionRegistry,
    },
    devices::api::device::SoundcoreDevice,
    error::SoundcoreResult,
};

pub async fn create_soundcore_device_registry() -> impl BLEConnectionRegistry {
    #[cfg(target_os = "windows")]
    {
        WindowsBLEConnectionRegistry::new()
    }
    // TODO: Add macOS and Linux
}

pub struct SoundcoreDeviceRegistry<Registry>
where
    Registry: BLEConnectionRegistry + Send + Sync,
{
    registry: Registry,
    devices: tokio::sync::Mutex<HashMap<String, Box<dyn SoundcoreDevice>>>,
}

impl<Registry> SoundcoreDeviceRegistry<Registry>
where
    Registry: BLEConnectionRegistry + Send + Sync,
{
    pub fn new(registry: Registry) -> Self {
        Self {
            registry,
            devices: tokio::sync::Mutex::new(HashMap::new()),
        }
    }

    async fn new_device(
        &self,
        mac_addr: &str,
        uuid_set: BLEConnectionUuidSet,
    ) -> SoundcoreResult<Option<Box<dyn SoundcoreDevice>>> {
        match self.registry.connection(mac_addr, uuid_set).await? {
            Some(_conn) => {
                /* Create a device usign the connection */
                todo!()
            }
            None => Ok(None),
        }
    }
}
