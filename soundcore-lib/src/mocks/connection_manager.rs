use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{mpsc, Mutex};

use crate::ble::BLEDeviceScanner;
use crate::{
    ble::{BLEConnectionManager, BLEConnectionUuidSet, BLEDeviceDescriptor},
    error::SoundcoreLibResult,
};

use super::{MockBLEConnection, MockBLEConnectionFactory, MockBLEScanner};

pub struct MockBLEConnectionManager {
    adapter_event_sender: Arc<Mutex<Option<mpsc::Sender<crate::ble::BLEAdapterEvent>>>>,
}

impl MockBLEConnectionManager {
    pub fn new() -> Self {
        Self {
            adapter_event_sender: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl BLEConnectionManager for MockBLEConnectionManager {
    type Scanner = MockBLEScanner;
    type ConnectionFactory = MockBLEConnectionFactory;
    type Connection = MockBLEConnection;

    fn scanner(&self) -> Self::Scanner {
        MockBLEScanner {}
    }

    fn connection_factory(&self) -> Self::ConnectionFactory {
        unimplemented!()
    }

    async fn scan(
        &self,
        duration: Option<std::time::Duration>,
    ) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>> {
        let scanner = MockBLEScanner {};
        scanner.scan(duration).await
    }

    async fn connect(
        &self,
        descriptor: BLEDeviceDescriptor,
        uuid_set: Option<BLEConnectionUuidSet>,
    ) -> SoundcoreLibResult<Arc<Self::Connection>> {
        let conn = MockBLEConnection::new_with_empty_channel();
        Ok(Arc::new(conn))
    }

    async fn adapter_events(
        &self,
    ) -> SoundcoreLibResult<mpsc::Receiver<crate::ble::BLEAdapterEvent>> {
        let (tx, rx) = mpsc::channel(1);
        *self.adapter_event_sender.lock().await = Some(tx);
        Ok(rx)
    }
}

impl Default for MockBLEConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}