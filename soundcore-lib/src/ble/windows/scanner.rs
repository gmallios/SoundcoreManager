use collections::HashMap;
use std::{collections, sync};
use sync::{Arc, Mutex};

use async_trait::async_trait;
use log::trace;
use tokio::task::spawn_blocking;
use windows::Devices::Bluetooth::Advertisement::{
    BluetoothLEAdvertisementReceivedEventArgs, BluetoothLEAdvertisementWatcher,
};
use windows::Devices::Bluetooth::BluetoothDevice;
use windows::Devices::Enumeration::DeviceInformation;
use windows::Foundation::TypedEventHandler;
use windows::Storage::Streams::DataReader;

use crate::ble::{BLEDeviceDescriptor, BLEDeviceScanner};
use crate::btaddr::BluetoothAdrr;
use crate::error::{SoundcoreLibError, SoundcoreLibResult};

const WATCH_DURATION: u64 = 10;

pub struct WindowsBLEDeviceScanner {}

impl WindowsBLEDeviceScanner {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl BLEDeviceScanner for WindowsBLEDeviceScanner {
    // type Descriptor = WindowsBLEDescriptor;

    async fn scan(&self) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>> {
        spawn_blocking(move || {
            let addr_swap_map =
                Arc::new(Mutex::new(HashMap::<BluetoothAdrr, BluetoothAdrr>::new()));

            let device_watcher = BluetoothLEAdvertisementWatcher::new()?;
            let handler = TypedEventHandler::new(
                move |_sender: &Option<BluetoothLEAdvertisementWatcher>,
                      args: &Option<BluetoothLEAdvertisementReceivedEventArgs>|
                      -> Result<(), windows::core::Error> {
                    event_handler(addr_swap_map.clone(), _sender, args)
                },
            );

            // Register the event handler
            device_watcher.Received(&handler)?;

            // Scan for devices
            device_watcher.Start()?;
            std::thread::sleep(std::time::Duration::from_secs(WATCH_DURATION));
            device_watcher.Stop()?;

            let scan_result =
                DeviceInformation::FindAllAsyncAqsFilter(&BluetoothDevice::GetDeviceSelector()?)?
                    .get()?;

            Ok(scan_result
                .into_iter()
                .map(|info| BluetoothDevice::FromIdAsync(&info.Id()?)?.get())
                .filter_map(|device| device.ok())
                .map(|device| {
                    let mut addr = BluetoothAdrr::from(device.BluetoothAddress()?);
                    match addr_swap_map.lock().unwrap().get(&addr) {
                        Some(new_addr) => {
                            trace!("Swapping MAC address {:?} with {:?}", addr, new_addr);
                            addr = new_addr.clone();
                        }
                        None => {}
                    }
                    Ok(BLEDeviceDescriptor::new(
                        device.Name()?.to_string(),
                        addr.to_string(),
                    )) as SoundcoreLibResult<BLEDeviceDescriptor>
                })
                .filter_map(|descriptor_result| descriptor_result.ok())
                .collect::<Vec<BLEDeviceDescriptor>>())
        })
        .await
        .map_err(|_e| SoundcoreLibError::Unknown)?
    }
}

/// This is a hack to replace the address with the one that is in the BLE advertisment
/// frames and not the one return by the device information.
/// This HashMap has the original address as the key and the new address as the value.
fn event_handler(
    swap_map: Arc<Mutex<HashMap<BluetoothAdrr, BluetoothAdrr>>>,
    _sender: &Option<BluetoothLEAdvertisementWatcher>,
    args: &Option<BluetoothLEAdvertisementReceivedEventArgs>,
) -> Result<(), windows::core::Error> {
    if let Some(args) = args {
        let addr = BluetoothAdrr::from(
            BluetoothDevice::FromBluetoothAddressAsync(args.BluetoothAddress()?)?
                .get()?
                .BluetoothAddress()?,
        );
        let mut swap_map = swap_map.lock().unwrap();
        if addr.is_soundcore_mac() {
            trace!(
                "Found candidate device {:?} for swapping MACs, checking advertisement data sections...",
                addr
            );
            let data_sections = args.Advertisement()?.DataSections()?.into_iter();

            for section in data_sections {
                let data_buf = section.Data()?;
                let data_reader = DataReader::FromBuffer(&data_buf)?;
                let mut data = vec![0_u8; data_buf.Length()? as usize];
                data_reader.ReadBytes(&mut data)?;
                trace!("Found advertisement data section: {:?}", data);

                match BluetoothAdrr::SOUNDCORE_MAC_PREFIXES
                    .iter()
                    .any(|prefix| data.starts_with(prefix))
                {
                    true => {
                        let addr_to_swap = BluetoothAdrr::from_bytes(&data[0..6]).unwrap();
                        if addr_to_swap != addr {
                            trace!("Found advertisement data section with MAC address, swapping {:?} with {:?}", addr_to_swap, addr);
                            swap_map.insert(addr_to_swap, addr.clone());
                        }
                    }
                    false => {
                        trace!("Found advertisement data section that does not contain a MAC address, skipping...");
                    }
                }
            }
        }
        drop(swap_map);
    }
    Ok(())
}
