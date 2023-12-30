use std::time::Duration;

use btleplug::{
    api::{Manager as _},
    platform::{Adapter, Manager},
};
use futures::{StreamExt};

use crate::{
    ble::{BLEConnectionUuidSet, BLEDeviceDescriptor},
    error::SoundcoreLibResult,
};

use super::{
    connection::BtlePlugConnection,
    connection_factory::BtlePlugConnectionFactory, scanner::BtlePlugScanner,
};

static DEFAULT_SCAN_DURATION: Duration = Duration::from_secs(5);

pub struct BtlePlugDeviceRegistry {
    manager: Manager,
    /// We need to store the adapters since calling manager.adapters()
    /// will create a new Vec<Adapter> every time and those adapters
    /// will have emtpy peripherals, even after scanning on the previous
    /// vec.
    adapters: Vec<Adapter>,
}

impl BtlePlugDeviceRegistry {
    pub async fn new() -> SoundcoreLibResult<Self> {
        let manager = Manager::new().await?;
        Ok(Self {
            adapters: manager.adapters().await?,
            manager,
        })
    }

    pub async fn scan(
        &self,
        duration: Option<Duration>,
    ) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>> {
        BtlePlugScanner::scan(self.adapters.to_owned(), duration).await
    }

    pub async fn connect(
        &self,
        descriptor: BLEDeviceDescriptor,
        uuid_set: impl Into<Option<BLEConnectionUuidSet>>,
    ) -> SoundcoreLibResult<BtlePlugConnection> {
        BtlePlugConnectionFactory::connect(
            self.adapters.to_owned(),
            descriptor,
            uuid_set.into(),
        )
        .await
    }
}
