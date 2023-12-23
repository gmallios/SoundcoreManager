use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::SoundcoreLibResult;

mod ble;
mod windows;

/// The general flow should be:
/// BLEDeviceScanner -> BLEDeviceDescriptor -> BLEConnectionFactory -> BLEConnection -> SoundcoreDevice
#[async_trait]
pub trait BLEConnection {
    async fn read_channel(&self) -> SoundcoreLibResult<tokio::sync::mpsc::Receiver<Vec<u8>>>;
    async fn write(&self, bytes: &[u8], write_type: WriteType) -> SoundcoreLibResult<()>;
}

#[async_trait]
pub trait BLEConnectionFactory {
    type Connection: BLEConnection + Send + Sync;
    async fn connect(
        &self,
        mac_addr: &str,
        uuid_set: BLEConnectionUuidSet,
    ) -> SoundcoreLibResult<Self::Connection>;
}

#[async_trait]
pub trait BLEDeviceScanner {
    // type Descriptor: DeviceDescriptor + Clone + Send + Sync;

    async fn scan(&self) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>>;
}

pub trait DeviceDescriptor {
    fn mac_addr(&self) -> &str;
    fn name(&self) -> &str;
}

pub struct BLEDeviceDescriptor {
    pub mac_addr: String,
    pub name: String,
}

impl BLEDeviceDescriptor {
    pub fn new(mac_addr: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            mac_addr: mac_addr.into(),
            name: name.into(),
        }
    }
}

impl DeviceDescriptor for BLEDeviceDescriptor {
    fn mac_addr(&self) -> &str {
        &self.mac_addr
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub enum WriteType {
    WithResponse,
    WithoutResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BLEConnectionUuidSet {
    pub service_uuid: uuid::Uuid,
    pub read_uuid: uuid::Uuid,
    pub write_uuid: uuid::Uuid,
}
