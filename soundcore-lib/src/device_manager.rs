use std::{sync::Arc, time::Duration};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use typeshare::typeshare;

use crate::{
    ble::{BLEConnectionManager, BLEDeviceDescriptor},
    btaddr::BluetoothAdrr,
    device::SoundcoreBLEDevice,
    error::SoundcoreLibResult,
    types::{KnownProductCodes, SOUNDCORE_NAME_PRODUCT_CODE_MAP},
};
use crate::ble::BLEAdapterEvent;
// TODO: Specify clippy & fmt features
#[allow(unused_imports)]
#[cfg(all(feature = "btleplug-backend", not(feature = "mock")))]
use crate::ble::btleplug::manager::BtlePlugBLEManager;
#[cfg(any(test, feature = "mock"))]
use crate::mocks::*;

pub struct DeviceManager<B>
where
    B: BLEConnectionManager,
{
    ble_manager: B,
    ble_devices: RwLock<HashMap<BluetoothAdrr, Arc<SoundcoreBLEDevice<B::Connection>>>>,
}

impl<B> DeviceManager<B>
where
    B: BLEConnectionManager,
{
    pub async fn new(ble_manager: B) -> Self {
        Self {
            ble_manager,
            ble_devices: RwLock::new(HashMap::new()),
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

    pub async fn get_device(
        &self,
        addr: BluetoothAdrr,
    ) -> Option<Arc<SoundcoreBLEDevice<B::Connection>>> {
        self.ble_devices.read().await.get(&addr).cloned()
    }

    pub async fn disconnect(&self, addr: BluetoothAdrr) -> SoundcoreLibResult<()> {
        self.ble_devices.write().await.remove(&addr);
        Ok(())
    }

    pub async fn list_open_connections(&self) -> Vec<BluetoothAdrr> {
        self.ble_devices.read().await.keys().cloned().collect()
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

    pub async fn get_event_channel(
        &self,
    ) -> SoundcoreLibResult<tokio::sync::mpsc::Receiver<BLEAdapterEvent>> {
        self.ble_manager.adapter_events().await
    }

    pub async fn disconnect_all(&self) -> SoundcoreLibResult<()> {
        let mut devices = self.ble_devices.write().await;
        devices.clear();
        Ok(())
    }

    fn map_descriptor_to_discovered_device(descriptor: &BLEDeviceDescriptor) -> DiscoveredDevice {
        DiscoveredDevice {
            descriptor: descriptor.to_owned(),
            model: None,
        }
    }

    fn resolve_model(discovered_device: DiscoveredDevice) -> DiscoveredDevice {
        match SOUNDCORE_NAME_PRODUCT_CODE_MAP
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
#[serde(rename_all = "camelCase", tag = "type")]
#[typeshare]
pub struct DiscoveredDevice {
    /// The BLE device descriptor.
    pub descriptor: BLEDeviceDescriptor,
    /// The model of the device, resolved using the device's advertised name.
    pub model: Option<KnownProductCodes>,
}

#[cfg(all(
    feature = "btleplug-backend",
    not(feature = "winrt-backend"),
    not(feature = "mock")
))]
pub async fn create_device_manager() -> DeviceManager<BtlePlugBLEManager> {
    let manager = BtlePlugBLEManager::new().await.unwrap();
    DeviceManager::new(manager).await
}

/// default-features shall be set to false
#[cfg(feature = "mock")]
pub async fn create_device_manager() -> DeviceManager<MockBLEConnectionManager> {
    DeviceManager::new(MockBLEConnectionManager::new()).await
}
