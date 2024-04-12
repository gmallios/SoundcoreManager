use async_trait::async_trait;
use btleplug::api::{Central, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};

use crate::ble::{BLEConnectionFactory, BLEDeviceDescriptor};
use crate::btaddr::BluetoothAdrr;
use crate::error::SoundcoreLibError;
use crate::{
    ble::{btleplug::connection::BtlePlugConnection, BLEConnectionUuidSet},
    error::SoundcoreLibResult,
};

pub struct BtlePlugConnectionFactory {
    manager: Manager,
    adapters: Vec<Adapter>,
}

impl BtlePlugConnectionFactory {
    pub fn new(manager: Manager, adapters: Vec<Adapter>) -> SoundcoreLibResult<Self> {
        Ok(Self { manager, adapters })
    }

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
}

#[async_trait]
impl BLEConnectionFactory for BtlePlugConnectionFactory {
    type Connection = BtlePlugConnection;
    async fn connect(
        &self,
        descriptor: BLEDeviceDescriptor,
        uuid_set: Option<BLEConnectionUuidSet>,
    ) -> SoundcoreLibResult<Self::Connection> {
        let mut adapter_and_peripheral = None;
        let adapters = self.adapters.to_owned();
        for ad in adapters {
            let _peripherals: Vec<Peripheral> = ad.peripherals().await?;
            let peripheral: Option<Peripheral> =
                Self::find_peripheral(&ad, &descriptor.addr).await?;
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
