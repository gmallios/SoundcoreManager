use async_trait::async_trait;
use btleplug::api::{Central, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral, PeripheralId};

use crate::{
    ble::{BLEConnectionUuidSet, btleplug::connection::BtlePlugConnection},
    error::SoundcoreLibResult,
};
use crate::ble::{BLEConnectionFactory, BLEDeviceDescriptor};
use crate::error::SoundcoreLibError;

pub struct BtlePlugConnectionFactory {
    _manager: Manager,
    adapters: Vec<Adapter>,
}

impl BtlePlugConnectionFactory {
    pub fn new(manager: Manager, adapters: Vec<Adapter>) -> SoundcoreLibResult<Self> {
        Ok(Self { _manager: manager, adapters })
    }

    async fn find_peripheral(
        adapter: &Adapter,
        id: PeripheralId,
    ) -> SoundcoreLibResult<Option<Peripheral>> {
        for peripheral in adapter.peripherals().await? {
            if id == peripheral.id() {
                return Ok(Some(peripheral));
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
                Self::find_peripheral(&ad, descriptor.id.clone()).await?;
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
