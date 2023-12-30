use btleplug::api::{Central, Peripheral as _};
use btleplug::platform::{Adapter, Peripheral};

use crate::ble::BLEDeviceDescriptor;
use crate::btaddr::BluetoothAdrr;
use crate::error::SoundcoreLibError;
use crate::{
    ble::{btleplug::connection::BtlePlugConnection, BLEConnectionUuidSet},
    error::SoundcoreLibResult,
};

pub struct BtlePlugConnectionFactory {}

impl BtlePlugConnectionFactory {
    async fn find_peripheral(
        adapter: &Adapter,
        addr: &BluetoothAdrr,
    ) -> SoundcoreLibResult<Option<Peripheral>> {
        for periph in adapter.peripherals().await? {
            if periph.address() == addr.to_owned().into() {
                return Ok(Some(periph));
            }
        }
        Ok(None)
    }

    pub async fn connect(
        adapters: Vec<Adapter>,
        descriptor: BLEDeviceDescriptor,
        uuid_set: Option<BLEConnectionUuidSet>,
    ) -> SoundcoreLibResult<BtlePlugConnection> {
        let mut adapter_and_peripheral = None;
        for ad in adapters {
            let _peripherals = ad.peripherals().await?;
            let peripheral = Self::find_peripheral(&ad, &descriptor.addr).await?;
            if peripheral.is_some() {
                adapter_and_peripheral = Some((ad, peripheral.unwrap()));
                break;
            }
        }

        match adapter_and_peripheral {
            Some((_adapter, peripheral)) => {
                Ok(BtlePlugConnection::new(peripheral, uuid_set, descriptor).await?)
            }
            None => Err(SoundcoreLibError::DeviceNotFound),
        }
    }
}
