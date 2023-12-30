use ::windows::Devices::Bluetooth::GenericAttributeProfile::GattWriteOption;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::btaddr::BluetoothAdrr;
use crate::error::SoundcoreLibResult;


pub mod btleplug;
// pub mod windows;

/// The general flow should be:
/// BLEDeviceScanner -> BLEDeviceDescriptor -> BLEConnectionFactory -> BLEConnection -> SoundcoreDevice
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
    // type Descriptor: DeviceDescriptor + Clone + Send + Sync;

    async fn scan(&self) -> SoundcoreLibResult<Vec<BLEDeviceDescriptor>>;
}

pub trait DeviceDescriptor {
    fn mac_addr(&self) -> BluetoothAdrr;
    fn name(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[cfg(target_os = "windows")]
impl From<WriteType> for GattWriteOption {
    fn from(val: WriteType) -> Self {
        match val {
            WriteType::WithResponse => GattWriteOption::WriteWithResponse,
            WriteType::WithoutResponse => GattWriteOption::WriteWithoutResponse,
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
