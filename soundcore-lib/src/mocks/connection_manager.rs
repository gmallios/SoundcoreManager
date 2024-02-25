use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    ble::{BLEConnectionManager, BLEConnectionUuidSet, BLEDeviceDescriptor},
    error::SoundcoreLibResult,
};
use crate::ble::BLEDeviceScanner;

use super::{MockBLEConnection, MockBLEConnectionFactory, MockBLEScanner};

pub struct MockBLEConnectionManager;

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
}
