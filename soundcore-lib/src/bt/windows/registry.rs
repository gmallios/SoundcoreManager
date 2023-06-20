use std::{collections::HashSet, sync::Arc, time::Duration};

use crate::{
    bt::ble::{BLEConnectionRegistry, BLEConnectionUuidSet},
    error::{SoundcoreError, SoundcoreResult},
    mac::BluetoothAdrr,
};
use async_trait::async_trait;
use std::sync::Mutex;
use windows::{
    core::HSTRING,
    Devices::{
        Bluetooth::{
            self,
            Advertisement::{
                BluetoothLEAdvertisementReceivedEventArgs, BluetoothLEAdvertisementWatcher,
            },
            BluetoothConnectionStatus,
        },
        Enumeration::{DeviceInformation, DeviceInformationKind},
    },
    Foundation::{Collections::IVectorView, TypedEventHandler},
};

use super::{connection::WindowsBLEConnection, descriptor::WindowsBLEDescriptor};

const WATCH_DURATION: u64 = 2;

pub struct WindowsBLEConnectionRegistry {}

impl WindowsBLEConnectionRegistry {
    /* This could be extended to include Classic Bluetooth devices using a standard scan fn */
    pub fn new() -> Self {
        WindowsBLEConnectionRegistry {}
    }

    fn ble_scan() -> SoundcoreResult<()> {
        /* We shouldn't need it since actual BLE scanning takes place in descriptors fn */
        let props = IVectorView::try_from(vec![
            HSTRING::from("System.Devices.Aep.DeviceAddress"),
            HSTRING::from("System.Devices.Aep.IsConnected"),
            HSTRING::from("System.Devices.Aep.Bluetooth.Le.IsConnectable"),
        ])?;
        let device_watcher =
            DeviceInformation::CreateWatcherWithKindAqsFilterAndAdditionalProperties(
                &HSTRING::from(
                    "(System.Devices.Aep.ProtocolId:=\"{bb7bb05e-5972-42b5-94fc-76eaa7084d49}\")",
                ),
                &props,
                DeviceInformationKind::AssociationEndpoint,
            )?;
        // Workaround from
        // https://github.com/Oppzippy/OpenSCQ30/blob/7fb236c7c3e41180a83a4abc8561e5cc36d0bc8c/lib/src/q30/connection/windows/windows_connection_registry.rs#L48
        device_watcher.Added(&TypedEventHandler::new(
            |_, device_info: &Option<DeviceInformation>| {
                if let Some(device_info) = device_info {
                    log::debug!("Found device {:?}", device_info);
                }
                Ok(())
            },
        ))?;
        device_watcher.Start()?;
        std::thread::sleep(Duration::from_secs(WATCH_DURATION));
        device_watcher.Stop()?;
        Ok(())
    }
}

#[async_trait]
impl BLEConnectionRegistry for WindowsBLEConnectionRegistry {
    type ConnType = WindowsBLEConnection;
    type DescType = WindowsBLEDescriptor;

    async fn descriptors(&self) -> SoundcoreResult<HashSet<Self::DescType>> {
        tokio::task::spawn_blocking(move || {
            let discovered_mac_addresses: Arc<Mutex<HashSet<String>>> =
                Arc::new(Mutex::new(HashSet::new()));
            let discovered_mac_addresses_clone = Arc::clone(&discovered_mac_addresses);
            let ble_watcher = BluetoothLEAdvertisementWatcher::new()?;

            let handler = TypedEventHandler::new(
                move |_, args: &Option<BluetoothLEAdvertisementReceivedEventArgs>| {
                    if let Some(args) = args {
                        let mac = BluetoothAdrr::from(args.BluetoothAddress()?);
                        log::trace!(
                            "Discovered BLE device with MAC {:?}",
                            BluetoothAdrr::from(args.BluetoothAddress()?).to_string()
                        );
                        if mac.is_soundcore_mac() {
                            log::debug!("Discovered Soundcore device {:?}", mac);
                            discovered_mac_addresses_clone
                                .lock()
                                .unwrap()
                                .insert(mac.to_string());
                        }
                    }
                    Ok(())
                },
            );

            ble_watcher.Received(&handler).unwrap();
            ble_watcher.Start().unwrap();
            std::thread::sleep(Duration::from_secs(WATCH_DURATION));
            ble_watcher.Stop().unwrap();

            let descriptors = discovered_mac_addresses
                .lock()
                .map_err(|_| SoundcoreError::MutexLockError {})?
                .iter()
                .map(|device| {
                    let bt_device = Bluetooth::BluetoothDevice::FromBluetoothAddressAsync(
                        BluetoothAdrr::from_str(&device)?.into(),
                    )?
                    .get()?;
                    Ok(WindowsBLEDescriptor::new(
                        bt_device.Name()?.to_string(),
                        device,
                    )) as SoundcoreResult<WindowsBLEDescriptor>
                })
                .filter_map(|res| match res {
                    Ok(desc) => Some(desc),
                    Err(e) => {
                        log::warn!("Error while creating BLE descriptor: {:?}", e);
                        None
                    }
                })
                .collect();
            Ok(descriptors) as SoundcoreResult<HashSet<WindowsBLEDescriptor>>
        })
        .await
        .map_err(|e| SoundcoreError::UnknownError {
            source: Box::new(e),
        })?
    }

    async fn connection(
        &self,
        mac_addr: &str,
        uuid_set: BLEConnectionUuidSet,
    ) -> SoundcoreResult<Option<Arc<Self::ConnType>>> {
        println!("Connecting to {:?}", mac_addr);
        Ok(
            WindowsBLEConnection::new(BluetoothAdrr::from_str(mac_addr)?.into(), uuid_set)
                .await?
                .map(Arc::new),
        )
    }
}
