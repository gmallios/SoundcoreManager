use std::{
    sync::{Arc, Weak},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use weak_table::{weak_value_hash_map::Entry, WeakValueHashMap};

use crate::{
    ble::{btleplug::manager::BtlePlugBLEManager, BLEConnectionManager, BLEDeviceDescriptor},
    btaddr::BluetoothAdrr,
    device::SoundcoreBLEDevice,
    error::SoundcoreLibResult,
    types::{SupportedModels, SOUNDCORE_NAME_MODEL_MAP},
};

pub struct DeviceManager<B>
where
    B: BLEConnectionManager,
{
    ble_manager: B,
    ble_devices: RwLock<WeakValueHashMap<BluetoothAdrr, Weak<SoundcoreBLEDevice<B::Connection>>>>,
}

impl<B> DeviceManager<B>
where
    B: BLEConnectionManager,
{
    pub async fn new(ble_manager: B) -> Self {
        Self {
            ble_manager,
            ble_devices: RwLock::new(WeakValueHashMap::new()),
        }
    }

    pub async fn connect(
        &self,
        device: DiscoveredDevice,
    ) -> SoundcoreLibResult<Arc<SoundcoreBLEDevice<B::Connection>>> {
        match self
            .ble_devices
            .write()
            .await
            .entry(device.descriptor.addr.clone())
        {
            Entry::Occupied(e) => Ok(e.get().to_owned()),
            Entry::Vacant(ve) => {
                // TODO: Check UUID sets based on resolved model
                let connection = self.ble_manager.connect(device.descriptor, None).await?;
                let device = Arc::new(SoundcoreBLEDevice::new(connection).await?);
                ve.insert(device.clone());
                Ok(device)
            }
        }
    }

    pub async fn ble_scan(
        &self,
        duration: Option<Duration>,
    ) -> SoundcoreLibResult<Vec<DiscoveredDevice>> {
        Ok(self
            .ble_manager
            .scan(duration)
            .await?
            .iter()
            .map(Self::map_descriptor_to_discovered_device)
            .map(|d| Self::resolve_model(d))
            .collect::<Vec<_>>())
    }

    fn map_descriptor_to_discovered_device(descriptor: &BLEDeviceDescriptor) -> DiscoveredDevice {
        DiscoveredDevice {
            descriptor: descriptor.to_owned(),
            model: None,
        }
    }

    fn resolve_model(discovered_device: DiscoveredDevice) -> DiscoveredDevice {
        match SOUNDCORE_NAME_MODEL_MAP
            .into_iter()
            .find(|(k, _v)| discovered_device.descriptor.name.contains(**k))
        {
            Some((_k, v)) => DiscoveredDevice {
                model: Some(v.to_owned()),
                ..discovered_device
            },
            None => discovered_device,
        }
    }
}

/// A discovered BLE device. The DiscoveredDevice can be upgraded to a SoundcoreBLEDevice.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiscoveredDevice {
    /// The BLE device descriptor.
    pub descriptor: BLEDeviceDescriptor,
    /// The model of the device, resolved using the device's advertised name.
    pub model: Option<SupportedModels>,
}

#[cfg(all(feature = "btleplug-backend", not(feature = "winrt-backend")))]
pub async fn create_device_manager() -> DeviceManager<BtlePlugBLEManager> {
    let manager = BtlePlugBLEManager::new().await.unwrap();
    DeviceManager::new(manager).await
}
