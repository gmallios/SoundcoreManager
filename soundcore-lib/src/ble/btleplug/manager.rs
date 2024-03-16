use std::sync::{Arc, Weak};
use std::time::Duration;

use async_trait::async_trait;
use btleplug::api::{Central as _, CentralEvent};
use btleplug::{
    api::Manager as _,
    platform::{Adapter, Manager},
};
use futures::StreamExt;
use log::warn;
use tokio::sync::RwLock;
use weak_table::weak_value_hash_map::Entry;
use weak_table::WeakValueHashMap;

use crate::ble::btleplug::connection::BtlePlugConnection;
use crate::ble::{
    BLEConnectionFactory, BLEConnectionManager, BLEConnectionUuidSet, BLEDeviceDescriptor,
    BLEAdapterEvent, BLEDeviceScanner,
};
use crate::btaddr::BluetoothAdrr;
use crate::error::SoundcoreLibResult;

use super::{connection_factory::BtlePlugConnectionFactory, scanner::BtlePlugScanner};

pub struct BtlePlugBLEManager {
    manager: Manager,
    /// We need to store the adapters since calling manager.adapters()
    /// will create a new Vec<Adapter> every time and those adapters
    /// will have emtpy peripherals, even after scanning on the previous
    /// vec.
    adapters: Vec<Adapter>,
    scanner: BtlePlugScanner,
    connection_factory: BtlePlugConnectionFactory,
    open_connections: RwLock<WeakValueHashMap<BluetoothAdrr, Weak<BtlePlugConnection>>>,
}

impl BtlePlugBLEManager {
    pub async fn new() -> SoundcoreLibResult<Self> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        let connection_factory =
            BtlePlugConnectionFactory::new(manager.to_owned(), adapters.to_owned())?;
        let scanner = BtlePlugScanner::new(adapters.to_owned());
        Ok(Self {
            adapters,
            manager,
            scanner,
            connection_factory,
            open_connections: RwLock::new(WeakValueHashMap::new()),
        })
    }
}

#[async_trait]
impl BLEConnectionManager for BtlePlugBLEManager {
    type Scanner = BtlePlugScanner;
    type ConnectionFactory = BtlePlugConnectionFactory;
    type Connection = BtlePlugConnection;

    fn scanner(&self) -> Self::Scanner {
        todo!()
    }

    fn connection_factory(&self) -> Self::ConnectionFactory {
        todo!()
    }

    async fn scan(
        &self,
        duration: Option<Duration>,
    ) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>> {
        self.scanner.scan(duration).await
    }

    async fn connect(
        &self,
        descriptor: BLEDeviceDescriptor,
        uuid_set: Option<BLEConnectionUuidSet>,
    ) -> SoundcoreLibResult<Arc<Self::Connection>> {
        match self
            .open_connections
            .write()
            .await
            .entry(descriptor.addr.to_owned())
        {
            Entry::Occupied(e) => Ok(e.get().to_owned()),
            Entry::Vacant(e) => {
                let connection = self
                    .connection_factory
                    .connect(descriptor, uuid_set)
                    .await?;
                let new_conn = Arc::new(connection);
                e.insert(new_conn.clone());
                Ok(new_conn)
            }
        }
    }

    async fn adapter_events(
        &self,
    ) -> SoundcoreLibResult<tokio::sync::mpsc::Receiver<BLEAdapterEvent>> {
        let (tx, rx) = tokio::sync::mpsc::channel::<BLEAdapterEvent>(255);

        for adapter in self.adapters.clone() {
            let tx_clone = tx.clone();
            let mut adapter_events = adapter.events().await.unwrap();
            tokio::spawn(async move {
                while let Some(evt) = adapter_events.next().await {
                 let event: Option<BLEAdapterEvent> = evt.try_into().ok();
                    if let Some(event) = event {
                        tx_clone.send(event).await.unwrap();
                    }
                }
            });
        }
        Ok(rx)
    }
}


impl TryInto<BLEAdapterEvent> for CentralEvent {
    type Error = ();

    fn try_into(self) -> Result<BLEAdapterEvent, Self::Error> {
        match self {
            CentralEvent::DeviceDisconnected(id) => Ok(BLEAdapterEvent::DeviceDisconnected(id.into())),
            CentralEvent::DeviceConnected(id) => Ok(BLEAdapterEvent::DeviceConnected(id.into())),
            _ => {
                warn!("Unhandled CentralEvent: {:?}", self);
                Err(())
            }
        }
    }
}