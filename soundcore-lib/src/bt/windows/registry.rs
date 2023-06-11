use std::{collections::HashSet, sync::Arc, time::Duration};

use async_trait::async_trait;
use windows::{
    core::HSTRING,
    Devices::{
        Bluetooth::{self, BluetoothConnectionStatus},
        Enumeration::{DeviceInformation, DeviceInformationKind},
    },
    Foundation::{Collections::IVectorView, TypedEventHandler},
};

use crate::{
    bt::ble::{BLEConnectionRegistry, BLEConnectionUuidSet},
    error::{SoundcoreError, SoundcoreResult},
    mac::BluetoothAdrr,
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
            Self::ble_scan()?;
            let devices = DeviceInformation::FindAllAsyncAqsFilter(
                &Bluetooth::BluetoothDevice::GetDeviceSelectorFromConnectionStatus(
                    BluetoothConnectionStatus::Connected,
                )?,
            )?
            .get()?;
            let descriptors = devices
                .into_iter()
                .map(|device| {
                    let device_id = device.Id()?;
                    let bt_device = Bluetooth::BluetoothDevice::FromIdAsync(&device_id)?.get()?;
                    Ok(WindowsBLEDescriptor::new(
                        device.Name()?.to_string(),
                        BluetoothAdrr::from(bt_device.BluetoothAddress()?).to_string(),
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
        Ok(
            WindowsBLEConnection::new(BluetoothAdrr::from_str(mac_addr)?.into(), uuid_set)
                .await?
                .map(Arc::new),
        )
    }
}
