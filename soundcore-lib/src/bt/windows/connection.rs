use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use tokio::sync::mpsc;
use uuid::Uuid;
use windows::{
    core::HRESULT,
    Devices::{
        self,
        Bluetooth::{
            BluetoothLEDevice,
            GenericAttributeProfile::{
                GattCharacteristic, GattClientCharacteristicConfigurationDescriptorValue,
                GattDeviceService, GattValueChangedEventArgs, GattWriteOption,
            },
        },
    },
    Foundation::{EventRegistrationToken, TypedEventHandler},
    Storage::Streams::{DataReader, DataWriter},
};

use crate::{
    bt::ble::{BLEConnection, BLEConnectionUuidSet, InternalWriteType},
    error::{SoundcoreError, SoundcoreResult},
};

pub struct WindowsBLEConnection {
    device: BluetoothLEDevice,
    read_characteristic: GattCharacteristic,
    write_characteristic: GattCharacteristic,
    value_changed_token: Arc<RwLock<Option<EventRegistrationToken>>>,
}

/* Implements the actual Windows API  */
impl WindowsBLEConnection {
    pub async fn new(addr: u64, uuid_set: BLEConnectionUuidSet) -> SoundcoreResult<Option<Self>> {
        tokio::task::spawn_blocking(move || {
            let device = BluetoothLEDevice::FromBluetoothAddressAsync(addr)?
                .get()
                .map_err(|err| match HRESULT::is_ok(err.code()) {
                    true => SoundcoreError::DeviceNotFound {
                        source: Box::new(err),
                    },
                    false => err.into(),
                })?;
            println!(
                "Found device: {:?}",
                device.GetGattServicesAsync()?.get()?.Services()?.Size()
            );
            let ble_svc = Self::service(&device, &uuid_set.service_uuid)?;
            let read_characteristic = Self::characteristic(&ble_svc, &uuid_set.read_uuid)?;
            let write_characteristic = Self::characteristic(&ble_svc, &uuid_set.write_uuid)?;
            Ok(Some(Self {
                device,
                read_characteristic,
                write_characteristic,
                value_changed_token: Arc::new(RwLock::new(None)),
            }))
        })
        .await
        .map_err(|err| SoundcoreError::UnknownError {
            source: Box::new(err),
        })?
    }

    pub fn characteristic(
        svc: &GattDeviceService,
        uuid: &Uuid,
    ) -> SoundcoreResult<GattCharacteristic> {
        svc.GetCharacteristicsAsync()?
            .get()?
            .Characteristics()?
            .into_iter()
            .find(|c| match c.Uuid() {
                Ok(u) => u.to_u128() == uuid.as_u128(),
                Err(_e) => false,
            })
            .ok_or(SoundcoreError::BLECharacteristicNotFound {
                uuid: uuid.to_string(),
            })
    }

    pub fn service(device: &BluetoothLEDevice, uuid: &Uuid) -> SoundcoreResult<GattDeviceService> {
        device
            .GetGattServicesWithCacheModeAsync(Devices::Bluetooth::BluetoothCacheMode::Uncached)?
            .get()?
            .Services()?
            .into_iter()
            .find(|s| match s.Uuid() {
                Ok(u) => u.to_u128() == uuid.as_u128(),
                Err(_e) => false,
            })
            .ok_or(SoundcoreError::BLEServiceNotFound {
                uuid: uuid.to_string(),
            })
    }

    pub fn write_to_characteristic(
        characteristic: &GattCharacteristic,
        data: &[u8],
        write_opt: GattWriteOption,
    ) -> SoundcoreResult<()> {
        let dw = DataWriter::new()?;
        dw.WriteBytes(data)?;
        let buf = dw.DetachBuffer()?;
        characteristic
            .WriteValueWithOptionAsync(&buf, write_opt)?
            .get()?;
        Ok(())
    }
}

/* Implements the mapping to our internal API */
#[async_trait]
impl BLEConnection for WindowsBLEConnection {
    async fn name(&self) -> SoundcoreResult<String> {
        Ok(self.device.Name()?.to_string())
    }

    async fn mac(&self) -> SoundcoreResult<String> {
        Ok(self.device.BluetoothAddress()?.to_string())
    }

    async fn write(&self, data: &[u8], write_type: InternalWriteType) -> SoundcoreResult<()> {
        let characteristic = self.write_characteristic.to_owned();
        let data = data.to_owned();
        tokio::task::spawn_blocking(move || {
            Self::write_to_characteristic(&characteristic, &data, write_type.into())
        })
        .await
        .map_err(|err| SoundcoreError::UnknownError {
            source: Box::new(err),
        })?
    }

    async fn receive_channel(&self) -> SoundcoreResult<mpsc::Receiver<Vec<u8>>> {
        self.read_characteristic
            .WriteClientCharacteristicConfigurationDescriptorAsync(
                GattClientCharacteristicConfigurationDescriptorValue::Notify,
            )?
            .await?;

        let (tx, rx) = mpsc::channel::<Vec<u8>>(100);
        let vct = self.value_changed_token.clone();
        let token = self
            .read_characteristic
            .ValueChanged(&TypedEventHandler::new(
                move |characteristic: &Option<GattCharacteristic>,
                      args: &Option<GattValueChangedEventArgs>| {
                    if let (Some(characteristic), Some(args)) = (characteristic, args) {
                        let val = args.CharacteristicValue()?;
                        let dr = DataReader::FromBuffer(&val)?;
                        let mut buf: Vec<u8> = vec![0; dr.UnconsumedBufferLength()? as usize];
                        dr.ReadBytes(&mut buf)?;
                        match tx.try_send(buf) {
                            Ok(_) => (),
                            Err(mpsc::error::TrySendError::Closed(_)) => {
                                let lock = vct.read().unwrap_or_else(|err| {
                                    log::debug!("Could not acquire read lock, error: {:?}", err);
                                    err.into_inner()
                                });
                                if let Some(token) = *lock {
                                    characteristic.RemoveValueChanged(token)?;
                                }
                            }
                            Err(err) => {
                                log::error!(
                                    "Could not send data to mpsc channel, error: {:?}",
                                    err
                                );
                            }
                        }
                    }
                    Ok(())
                },
            ))?;
        let mut token_lock = match self.value_changed_token.write() {
            Ok(lock) => lock,
            Err(err) => {
                log::debug!("Could not acquire write lock on vct, error: {:?}", err);
                err.into_inner()
            }
        };
        if let Some(token) = *token_lock {
            self.read_characteristic.RemoveValueChanged(token)?;
        }
        *token_lock = Some(token);
        Ok(rx)
    }
}

impl Drop for WindowsBLEConnection {
    fn drop(&mut self) {
        let lock = self.value_changed_token.read().unwrap_or_else(|err| {
            log::debug!("Could not acquire vct read lock, error: {:?}", err);
            err.into_inner()
        });
        if let Some(token) = *lock {
            if let Err(err) = self.read_characteristic.RemoveValueChanged(token) {
                log::error!("Could not remove value changed, error: {:?}", err);
            }
        }
    }
}

impl From<InternalWriteType> for GattWriteOption {
    fn from(val: InternalWriteType) -> Self {
        match val {
            InternalWriteType::WithResponse => GattWriteOption::WriteWithResponse,
            InternalWriteType::WithoutResponse => GattWriteOption::WriteWithoutResponse,
        }
    }
}
