use crate::{
    bt::ble::{BLEConnectionRegistry, BLEConnectionUuidSet},
    bt::windows::{connection::WindowsBLEConnection, descriptor::WindowsBLEDescriptor},
    error::{SoundcoreError, SoundcoreResult},
    mac::BluetoothAdrr,
};
use async_trait::async_trait;
use log::{debug, warn};
use std::sync::Mutex;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};
use windows::{
    Devices::{
        Bluetooth::{
            Advertisement::{
                BluetoothLEAdvertisementReceivedEventArgs, BluetoothLEAdvertisementWatcher,
            },
            BluetoothDevice,
        },
        Enumeration::DeviceInformation,
    },
    Foundation::TypedEventHandler,
    Storage::Streams::DataReader,
};

const WATCH_DURATION: u64 = 10;

pub struct WindowsBLEConnectionRegistry {}

impl WindowsBLEConnectionRegistry {
    pub fn new() -> Self {
        WindowsBLEConnectionRegistry {}
    }
}

#[async_trait]
impl BLEConnectionRegistry for WindowsBLEConnectionRegistry {
    type ConnType = WindowsBLEConnection;
    type DescType = WindowsBLEDescriptor;

    async fn descriptors(&self) -> SoundcoreResult<HashSet<Self::DescType>> {
        tokio::task::spawn_blocking(move || {
            /* Refer to A3951__BLE.md to read why this is needed */
            let swap_map: Arc<Mutex<HashMap<BluetoothAdrr, BluetoothAdrr>>> =
                Arc::new(Mutex::new(HashMap::new()));
            let swap_mac_clone = swap_map.clone();
            let ble_watcher = BluetoothLEAdvertisementWatcher::new()?;

            let handler = TypedEventHandler::new(
                move |_, args: &Option<BluetoothLEAdvertisementReceivedEventArgs>| {
                    if let Some(args) = args {
                        let mac = BluetoothAdrr::from(args.BluetoothAddress()?);
                        let dev = BluetoothDevice::FromBluetoothAddressAsync(mac.into())?.get()?;
                        let m = BluetoothAdrr::from(dev.BluetoothAddress()?);
                        if m.is_soundcore_mac() {
                            debug!("Got Soudncore MAC from BLE scan: {:?}", m.to_string());
                            args.Advertisement()?.DataSections()?.into_iter().for_each(
                                |data_section| {
                                    let data_buf = data_section.Data().unwrap();
                                    let data_reader = DataReader::FromBuffer(&data_buf).unwrap();
                                    let mut output_buf =
                                        vec![0_u8; data_buf.Length().unwrap() as usize];
                                    data_reader.ReadBytes(&mut output_buf).unwrap();
                                    debug!("Got Advertisement data: {:X?}", output_buf);
                                    if output_buf.starts_with(&[0xAC, 0x12]) {
                                        // This is the device's "regular" MAC address
                                        let addr_to_swap =
                                            BluetoothAdrr::from_bytes(&output_buf[0..6]).unwrap();
                                        if m != addr_to_swap {
                                            swap_mac_clone
                                                .lock()
                                                .unwrap()
                                                .insert(addr_to_swap, m.clone());
                                        }
                                    }
                                },
                            );
                        }
                    }
                    Ok(())
                },
            );

            ble_watcher.Received(&handler)?;
            ble_watcher.Start()?;
            std::thread::sleep(Duration::from_secs(WATCH_DURATION));
            ble_watcher.Stop()?;

            let scan_result =
                DeviceInformation::FindAllAsyncAqsFilter(&BluetoothDevice::GetDeviceSelector()?)?
                    .get()?;

            let descriptors = scan_result
                .into_iter()
                .map(|dev| BluetoothDevice::FromIdAsync(&dev.Id()?)?.get())
                .filter_map(|res| match res {
                    Ok(dev) => match dev.BluetoothAddress() {
                        Ok(addr) => match BluetoothAdrr::from(addr).is_soundcore_mac() {
                            true => Some(dev),
                            false => None,
                        },
                        Err(e) => {
                            warn!("Error while getting Bluetooth address: {:?}", e);
                            None
                        }
                    },
                    Err(e) => {
                        warn!("Error while creating BLE device: {:?}", e);
                        None
                    }
                })
                .map(|device| {
                    let mut addr = BluetoothAdrr::from(device.BluetoothAddress()?);
                    match swap_map.lock().unwrap().get(&addr) {
                        Some(new_addr) => {
                            debug!(
                                "Swapping MAC address {:?} -> {:?}",
                                addr.to_string(),
                                new_addr.to_string()
                            );
                            addr = new_addr.clone();
                        }
                        None => {}
                    }
                    Ok(WindowsBLEDescriptor::new(
                        device.Name()?.to_string(),
                        addr.to_string(),
                    )) as SoundcoreResult<WindowsBLEDescriptor>
                })
                .filter_map(|res| match res {
                    Ok(desc) => Some(desc),
                    Err(e) => {
                        warn!("Error while creating BLE descriptor: {:?}", e);
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
