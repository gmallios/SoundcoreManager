use async_trait::async_trait;
use btleplug::api::{CharPropFlags, Characteristic, Peripheral as _, Service};
use btleplug::platform::Peripheral;
use futures::StreamExt;
use log::{error, trace, warn};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Receiver;
use tokio::task;
use uuid::{uuid, Uuid};

use crate::ble::{BLEConnection, BLEConnectionUuidSet, BLEDeviceDescriptor, WriteType};
use crate::error::{SoundcoreLibError, SoundcoreLibResult};

static EXCLUDED_SERVICE_UUIDS: [Uuid; 4] = [
    uuid!("00001800-0000-1000-8000-00805f9b34fb"),
    uuid!("00001801-0000-1000-8000-00805f9b34fb"),
    uuid!("86868686-8686-8686-8686-868686868686"),
    uuid!("66666666-6666-6666-6666-666666666666"),
];

pub struct BtlePlugConnection {
    peripheral: Peripheral,
    uuid_set: BLEConnectionUuidSet,
    descriptor: BLEDeviceDescriptor,
    read_characteristic: Characteristic,
    write_characteristic: Characteristic,
}

impl BtlePlugConnection {
    pub async fn new(
        peripheral: Peripheral,
        uuid_set: Option<BLEConnectionUuidSet>,
        descriptor: BLEDeviceDescriptor,
    ) -> SoundcoreLibResult<Self> {
        task::spawn(async move {
            peripheral.connect().await?;
            peripheral.discover_services().await?;

            let uuid_set = match uuid_set {
                Some(uuid_set) => uuid_set,
                None => Self::find_soundcore_uuid_set(&peripheral).await?.ok_or(
                    SoundcoreLibError::MissingUUIDSet(peripheral.address().to_string()),
                )?,
            };

            let service = Self::find_service_by_uuid(&peripheral, uuid_set.service_uuid).ok_or(
                SoundcoreLibError::MissingService(uuid_set.service_uuid.to_string()),
            )?;
            let read_characteristic =
                Self::find_characteristic_by_uuid(&service, uuid_set.read_uuid).ok_or(
                    SoundcoreLibError::MissingCharacteristic(uuid_set.read_uuid.to_string()),
                )?;
            let write_characteristic =
                Self::find_characteristic_by_uuid(&service, uuid_set.write_uuid).ok_or(
                    SoundcoreLibError::MissingCharacteristic(uuid_set.write_uuid.to_string()),
                )?;
            peripheral.subscribe(&read_characteristic).await?;
            Ok(BtlePlugConnection {
                peripheral,
                uuid_set,
                descriptor,
                read_characteristic,
                write_characteristic,
            })
        })
        .await?
    }

    async fn find_soundcore_uuid_set(
        peripheral: &Peripheral,
    ) -> SoundcoreLibResult<Option<BLEConnectionUuidSet>> {
        let services = peripheral
            .services()
            .to_owned()
            .into_iter()
            .filter(|svc| svc.characteristics.len() >= 2)
            .filter(|svc| !EXCLUDED_SERVICE_UUIDS.contains(&svc.uuid))
            .collect::<Vec<_>>();

        for service in services.iter() {
            trace!("Inspecting Service: {:#?}", service);
            let characteristics = service.characteristics.to_owned();
            let read_characteristic = characteristics.to_owned().into_iter().find(|c| {
                c.properties.contains(CharPropFlags::NOTIFY)
                    && c.properties.contains(CharPropFlags::READ)
            });

            let write_characteristic = characteristics.into_iter().find(|c| {
                c.properties.contains(CharPropFlags::WRITE)
                    && c.properties.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE)
            });

            if let (Some(read_characteristic), Some(write_characteristic)) =
                (read_characteristic, write_characteristic)
            {
                return Ok(Some(BLEConnectionUuidSet {
                    service_uuid: service.uuid,
                    read_uuid: read_characteristic.uuid,
                    write_uuid: write_characteristic.uuid,
                }));
            }
        }

        trace!(
            "No suitable service found for device: {:?}",
            peripheral.address()
        );
        trace!("Available services: {:#?}", services);
        Ok(None)
    }

    fn find_service_by_uuid(peripheral: &Peripheral, uuid: Uuid) -> Option<Service> {
        peripheral.services().into_iter().find(|s| s.uuid == uuid)
    }

    fn find_characteristic_by_uuid(service: &Service, uuid: Uuid) -> Option<Characteristic> {
        service
            .characteristics
            .clone()
            .into_iter()
            .find(|c| c.uuid == uuid)
    }
}

impl BLEConnection for BtlePlugConnection {
    fn descriptor(&self) -> BLEDeviceDescriptor {
        self.descriptor.to_owned()
    }

    async fn byte_channel(&self) -> SoundcoreLibResult<Receiver<Vec<u8>>> {
        let (tx, rx) = mpsc::channel(255);
        let (peripheral, read_characteristic_uuid) = (
            self.peripheral.clone(),
            self.read_characteristic.clone().uuid,
        );
        let mut notifications = tokio::spawn(async move { peripheral.notifications().await })
            .await?
            .unwrap();

        tokio::spawn(async move {
            while let Some(notification) = notifications.next().await {
                match notification.uuid == read_characteristic_uuid {
                    true => match tx.try_send(notification.value.to_vec()) {
                        Ok(_) => {
                            trace!(
                                "Sent notification to channel, data: {:#X?}",
                                notification.value
                            );
                        }
                        Err(TrySendError::Closed(_)) => {
                            warn!("Channel closed");
                            break;
                        }
                        Err(e) => {
                            error!(
                                "Failed to send notification to byte_channel, error: {:?}",
                                e
                            );
                        }
                    },
                    false => {
                        warn!("Got unrelated notification: {:?}", notification);
                    }
                }
            }
        });
        Ok(rx)
    }

    async fn write(&self, bytes: &[u8], write_type: WriteType) -> SoundcoreLibResult<()> {
        let (peripheral, writer_characteristic, bytes) = (
            self.peripheral.clone(),
            self.write_characteristic.clone(),
            bytes.to_owned(),
        );
        trace!(
            "Writing bytes: {:#X?} to characteristic: {:?}",
            bytes,
            self.write_characteristic
        );
        tokio::spawn(async move {
            peripheral
                .write(&writer_characteristic, &bytes, write_type.into())
                .await
        })
        .await
        .unwrap()?;
        Ok(())
    }
}

impl Drop for BtlePlugConnection {
    fn drop(&mut self) {
        let peripheral = self.peripheral.clone();
        task::spawn(async move {
            peripheral.disconnect().await.unwrap();
        });
    }
}
