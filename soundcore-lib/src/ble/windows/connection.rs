use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use log::{debug, error};
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;
use windows::Devices::Bluetooth::GenericAttributeProfile::{
    GattCharacteristic, GattClientCharacteristicConfigurationDescriptorValue, GattDeviceService,
    GattValueChangedEventArgs,
};
use windows::Devices::Bluetooth::{BluetoothCacheMode, BluetoothLEDevice};
use windows::Foundation::{EventRegistrationToken, TypedEventHandler};
use windows::Storage::Streams::{DataReader, DataWriter};

use crate::ble::{BLEConnection, BLEConnectionUuidSet, BLEDeviceDescriptor, WriteType};
use crate::btaddr::BluetoothAdrr;
use crate::error::{SoundcoreLibError, SoundcoreLibResult};

pub struct WindowsBLEConnection {
    device: BluetoothLEDevice,
    read_charateristic: GattCharacteristic,
    write_charateristic: GattCharacteristic,
    value_changed: Arc<RwLock<Option<EventRegistrationToken>>>,
}

impl WindowsBLEConnection {
    pub async fn new(
        _addr: BluetoothAdrr,
        _uuid_set: Option<BLEConnectionUuidSet>,
    ) -> SoundcoreLibResult<Option<Self>> {
        tokio::task::spawn_blocking(move || {
            // TODO: Handle errors better
            let ble_device = BluetoothLEDevice::FromBluetoothAddressAsync(
                BluetoothAdrr::from_str("72:19:23:4E:B4:97").unwrap().into(),
            )?
            .get()
            .map_err(|_| SoundcoreLibError::DeviceNotFound)?;
            debug!(
                "Connected to device: {:?} with number of services: {:?}",
                ble_device,
                ble_device
                    .GetGattServicesAsync()?
                    .get()?
                    .Services()?
                    .Size()?
            );
            let _set = Self::find_soundcore_uuid_set(&ble_device)?;

            Ok(None)
        })
        .await?
    }

    fn find_soundcore_uuid_set(
        device: &BluetoothLEDevice,
    ) -> SoundcoreLibResult<Option<BLEConnectionUuidSet>> {
        let services = device
            .GetGattServicesWithCacheModeAsync(BluetoothCacheMode::Uncached)?
            .get()?
            .Services()?;

        if services.Size()? == 0 {
            return Ok(None);
        }

        let service = services.GetAt(0)?;
        let mut characteristics = service
            .GetCharacteristicsWithCacheModeAsync(BluetoothCacheMode::Uncached)?
            .get()?
            .Characteristics()?
            .into_iter();

        let _read_characteristic = characteristics.find(|c| {
            let uuid = Uuid::from_u128(c.Uuid().unwrap().to_u128());
            println!("{:?}", uuid);
            uuid == Uuid::parse_str("8888").unwrap()
        });

        todo!()
    }

    fn find_service_by_uuid(
        device: &BluetoothLEDevice,
        uuid: &Uuid,
    ) -> SoundcoreLibResult<Option<GattDeviceService>> {
        Ok(device
            .GetGattServicesWithCacheModeAsync(BluetoothCacheMode::Uncached)?
            .get()?
            .Services()?
            .into_iter()
            .find(|s| s.Uuid().unwrap().to_u128() == uuid.as_u128()))
    }

    fn find_characteristic_by_uuid(
        service: &GattDeviceService,
        uuid: &Uuid,
    ) -> SoundcoreLibResult<Option<GattCharacteristic>> {
        Ok(service
            .GetCharacteristicsWithCacheModeAsync(BluetoothCacheMode::Uncached)?
            .get()?
            .Characteristics()?
            .into_iter()
            .find(|c| c.Uuid().unwrap().to_u128() == uuid.as_u128()))
    }

    fn write_to_characteristic(
        characteristic: &GattCharacteristic,
        bytes: &[u8],
        write_type: WriteType,
    ) -> SoundcoreLibResult<()> {
        let data_writer = DataWriter::new()?;
        data_writer.WriteBytes(bytes)?;
        let buf = data_writer.DetachBuffer()?;
        characteristic
            .WriteValueWithOptionAsync(&buf, write_type.into())?
            .get()?;
        Ok(())
    }

    fn value_changed_handler(
        characteristic: &Option<GattCharacteristic>,
        args: &Option<GattValueChangedEventArgs>,
        tx: &tokio::sync::mpsc::Sender<Vec<u8>>,
        value_changed: &Arc<RwLock<Option<EventRegistrationToken>>>,
    ) -> windows::core::Result<()> {
        if let (Some(characteristic), Some(args)) = (characteristic, args) {
            let characteristic_value = args.CharacteristicValue()?;
            let reader = DataReader::FromBuffer(&characteristic_value)?;
            let mut buf = vec![0u8; reader.UnconsumedBufferLength()? as usize];
            reader.ReadBytes(&mut buf)?;

            match tx.try_send(buf) {
                Ok(_) => {}
                Err(tokio::sync::mpsc::error::TrySendError::Closed(e)) => {
                    let lock = value_changed.read().unwrap();
                    if let Some(token) = *lock {
                        characteristic.RemoveValueChanged(token)?;
                    }
                    error!("Error sending data over mpsc channel, error: {:?}", e);
                }
                Err(e) => {
                    error!("TX mpsc channel error: {:?}", e);
                }
            }
        }
        Ok(())
    }
}

impl BLEConnection for WindowsBLEConnection {
    async fn byte_channel(&self) -> SoundcoreLibResult<Receiver<Vec<u8>>> {
        self.read_charateristic
            .WriteClientCharacteristicConfigurationDescriptorAsync(
                GattClientCharacteristicConfigurationDescriptorValue::Notify,
            )?
            .await?;

        let (tx, rx) = tokio::sync::mpsc::channel::<Vec<u8>>(255);
        let value_changed = self.value_changed.clone();
        let handler = TypedEventHandler::new(move |c, args| {
            Self::value_changed_handler(c, args, &tx, &value_changed)
        });
        let event_token = self.read_charateristic.ValueChanged(&handler)?;
        let mut token_lock = match self.value_changed.write() {
            Ok(lock) => lock,
            Err(_) => return Err(SoundcoreLibError::Unknown),
        };

        if let Some(token) = *token_lock {
            self.read_charateristic.RemoveValueChanged(token)?;
        }

        *token_lock = Some(event_token);
        Ok(rx)
    }

    async fn write(&self, bytes: &[u8], write_type: WriteType) -> SoundcoreLibResult<()> {
        let characteristic = self.write_charateristic.to_owned();
        let data = bytes.to_owned();
        tokio::task::spawn_blocking(move || {
            Self::write_to_characteristic(&characteristic, &data, write_type)
        })
        .await?
    }

    fn descriptor(&self) -> BLEDeviceDescriptor {
        todo!()
    }
}
