use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::btaddr::BluetoothAdrr;
use crate::error::SoundcoreLibResult;

#[cfg(feature = "btleplug-backend")]
pub mod btleplug;
#[cfg(feature = "winrt-backend")]
pub mod windows;

#[async_trait]
pub trait BLEConnectionManager {
    type Scanner: BLEDeviceScanner + Send + Sync;
    type ConnectionFactory: BLEConnectionFactory + Send + Sync;
    type Connection: BLEConnection + Send + Sync;

    fn scanner(&self) -> Self::Scanner;
    fn connection_factory(&self) -> Self::ConnectionFactory;
    async fn scan(
        &self,
        duration: Option<Duration>,
    ) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>>;

    async fn connect(
        &self,
        descriptor: BLEDeviceDescriptor,
        uuid_set: Option<BLEConnectionUuidSet>,
    ) -> SoundcoreLibResult<Arc<Self::Connection>>;
}

#[async_trait]
pub trait BLEConnection {
    fn descriptor(&self) -> BLEDeviceDescriptor;
    async fn byte_channel(&self) -> SoundcoreLibResult<tokio::sync::mpsc::Receiver<Vec<u8>>>;
    async fn write(&self, bytes: &[u8], write_type: WriteType) -> SoundcoreLibResult<()>;
}

#[async_trait]
pub trait BLEConnectionFactory {
    type Connection: BLEConnection + Send + Sync;
    async fn connect(
        &self,
        descriptor: BLEDeviceDescriptor,
        uuid_set: Option<BLEConnectionUuidSet>,
    ) -> SoundcoreLibResult<Self::Connection>;
}

#[async_trait]
pub trait BLEDeviceScanner {
    async fn scan(
        &self,
        duration: Option<Duration>,
    ) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>>;
}

pub trait DeviceDescriptor {
    fn mac_addr(&self) -> BluetoothAdrr;
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BLEDeviceDescriptor {
    pub addr: BluetoothAdrr,
    pub name: String,
}

impl BLEDeviceDescriptor {
    pub fn new(mac_addr: impl Into<BluetoothAdrr>, name: impl Into<String>) -> Self {
        Self {
            addr: mac_addr.into(),
            name: name.into(),
        }
    }
}

impl DeviceDescriptor for BLEDeviceDescriptor {
    fn mac_addr(&self) -> BluetoothAdrr {
        self.addr.clone()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub enum WriteType {
    WithResponse,
    WithoutResponse,
}

#[cfg(all(target_os = "windows", feature = "winrt-backend"))]
impl From<WriteType> for ::windows::Devices::Bluetooth::GenericAttributeProfile::GattWriteOption {
    fn from(val: WriteType) -> Self {
        match val {
            WriteType::WithResponse =>  ::windows::Devices::Bluetooth::GenericAttributeProfile::GattWriteOption::WriteWithResponse,
            WriteType::WithoutResponse =>  ::windows::Devices::Bluetooth::GenericAttributeProfile::GattWriteOption::WriteWithoutResponse,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BLEConnectionUuidSet {
    pub service_uuid: uuid::Uuid,
    pub read_uuid: uuid::Uuid,
    pub write_uuid: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConnectionEvent {
    Connected(String),
    Disconnected(String),
    DataReceived(Vec<u8>),
}
