use std::sync::{Arc, RwLock};

use uuid::Uuid;
use windows::{
    core::HRESULT,
    Devices::Bluetooth::{
        BluetoothLEDevice,
        GenericAttributeProfile::{GattCharacteristic, GattDeviceService},
    },
    Foundation::EventRegistrationToken,
};

use crate::{
    bt::ble::BLEConnectionUuidSet,
    error::{SoundcoreError, SoundcoreResult},
};

pub struct WindowsBLEConnection {
    device: BluetoothLEDevice,
    read_characteristic: GattCharacteristic,
    write_characteristic: GattCharacteristic,
    value_changed_token: Arc<RwLock<Option<EventRegistrationToken>>>,
}

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
        _svc: &GattDeviceService,
        _uuid: &Uuid,
    ) -> SoundcoreResult<GattCharacteristic> {
        todo!()
    }

    pub fn service(device: &BluetoothLEDevice, uuid: &Uuid) -> SoundcoreResult<GattDeviceService> {
        device
            .GetGattServicesAsync()?
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
}
