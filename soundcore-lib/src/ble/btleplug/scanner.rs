use std::time::Duration;

use async_trait::async_trait;
use btleplug::api::{Central, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Peripheral};
use futures::{stream, StreamExt};
use log::{debug, info, warn};

use crate::{ble::BLEDeviceDescriptor, error::SoundcoreLibResult};
use crate::ble::BLEDeviceScanner;
use crate::btaddr::BluetoothAdrr;

static DEFAULT_SCAN_DURATION: Duration = Duration::from_secs(5);

pub struct BtlePlugScanner {
    adapters: Vec<Adapter>,
}

impl BtlePlugScanner {
    pub fn new(adapters: Vec<Adapter>) -> Self {
        Self { adapters }
    }

    pub async fn scan(
        adapters: Vec<Adapter>,
        duration: Option<Duration>,
    ) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>> {
        tokio::spawn(async move {
            stream::iter(&adapters)
                .for_each_concurrent(2, |adapter| async move {
                    adapter.start_scan(ScanFilter::default()).await.unwrap();
                    tokio::time::sleep(duration.unwrap_or(DEFAULT_SCAN_DURATION)).await;
                    adapter.stop_scan().await.unwrap();
                })
                .await;

            let peripherals = stream::iter(adapters)
                .filter_map(|d| async move { Self::extract_peripherals(d).await })
                .flatten()
                .map(|(_a, p)| p)
                .filter_map(|p| async move { Self::connected_peripherals(p).await })
                .filter_map(|p| async move { Self::peripheral_to_descriptor(p).await })
                .filter_map(|d| async move { Self::resolve_name_for_descriptor(d).await.ok() })
                .collect::<Vec<_>>()
                .await;

            Ok(peripherals)
        })
            .await
            .unwrap()
    }

    // TODO: Remove this when https://github.com/deviceplug/btleplug/issues/267 is fixed
    #[cfg(target_os = "windows")]
    async fn resolve_name_for_descriptor(
        mut descriptor: BLEDeviceDescriptor,
    ) -> SoundcoreLibResult<BLEDeviceDescriptor> {
        if !descriptor.name.is_empty() {
            return Ok(descriptor);
        }

        use windows::Devices::Bluetooth::BluetoothLEDevice;

        descriptor.name =
            BluetoothLEDevice::FromBluetoothAddressAsync(descriptor.addr.clone().into())?
                .get()?
                .Name()?
                .to_string_lossy();
        Ok(descriptor)
    }

    // TODO: Remove this when https://github.com/deviceplug/btleplug/issues/267 is fixed
    #[cfg(not(target_os = "windows"))]
    async fn resolve_name_for_descriptor(
        mut descriptor: BLEDeviceDescriptor,
    ) -> SoundcoreLibResult<BLEDeviceDescriptor> {
        Ok(descriptor)
    }

    async fn connected_peripherals(peripheral: Peripheral) -> Option<Peripheral> {
        match tokio::time::timeout(tokio::time::Duration::from_secs(5), peripheral.connect()).await {
            Ok(Ok(_)) => Some(peripheral),
            Ok(Err(err)) => {
                warn!(
                    "Errored out determining if peripheral {:?} is connected, err: {err}",
                    peripheral
                );
                None
            }
            Err(err) => {
                warn!(
                    "Timed out determining if peripheral {:?} is connected, err: {err}",
                    peripheral
                );
                None
            }
        }
    }

    async fn peripheral_to_descriptor(peripheral: Peripheral) -> Option<BLEDeviceDescriptor> {
        let name = match peripheral.properties().await {
            Ok(Some(props)) => props.local_name.unwrap_or_default(),
            Err(err) => {
                warn!(
                    "Error getting properties for peripheral {:?} err: {err}",
                    peripheral
                );
                "".to_string()
            }
            _ => "".to_string(),
        };

        Some(BLEDeviceDescriptor::new(
            BluetoothAdrr::try_from(peripheral.address()).ok()?,
            peripheral.id(),
            name,
        ))
    }

    async fn extract_peripherals(
        adapter: Adapter,
    ) -> Option<impl stream::Stream<Item=(Adapter, Peripheral)>> {
        match adapter.peripherals().await {
            Ok(peripherals) => {
                Some(stream::iter(peripherals).map(move |p| (adapter.to_owned(), p)))
            }
            Err(err) => {
                warn!(
                    "Error getting peripherals for adapter: {:?} err: {err}",
                    adapter
                );
                None
            }
        }
    }
}

#[async_trait]
impl BLEDeviceScanner for BtlePlugScanner {
    async fn scan(
        &self,
        duration: Option<Duration>,
    ) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>> {
        Self::scan(self.adapters.to_owned(), duration).await
    }
}
