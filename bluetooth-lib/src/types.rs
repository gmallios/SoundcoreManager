use std::fmt::{Display, Debug};

use async_trait::async_trait;
use crate::BthError;

#[cfg(target_os = "windows")]
use windows::Win32::Devices::Bluetooth::{BLUETOOTH_ADDRESS_STRUCT, BLUETOOTH_DEVICE_INFO_STRUCT};


#[async_trait]
pub trait Scanner {
    fn new() -> Self;
    async fn scan(&mut self) -> Vec<BluetoothDevice>;
}

#[async_trait]
pub trait RFCOMMClient: Sized {
    async fn new() -> Result<Self, BthError>;
    async fn connect_uuid(&mut self, bt_addr: BluetoothAdrr, uuid: &str) -> Result<(), BthError>;
    async fn connect_port(&mut self, address: BluetoothAdrr, port: u32) -> Result<(), BthError>;
    async fn send(&self, data: &[u8]) -> Result<(), BthError>;
    async fn recv(&self, num_of_bytes: usize) -> Result<Vec<u8>, BthError>;
    async fn close(&self);
}

#[derive(Clone, Eq, PartialEq)]
pub struct BluetoothDevice {
    pub name: String,
    pub address: BluetoothAdrr,
    pub connected: bool,
    pub remembered: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BluetoothAdrr {
    pub address: [u8; 6],
}

impl From<String> for BluetoothAdrr {
    fn from(address: String) -> Self {
        if address.contains(':') {
            let address = address
                .split(':')
                .map(|x| u8::from_str_radix(x, 16).unwrap())
                .collect::<Vec<u8>>();
            Self {
                address: address.try_into().unwrap(),
            }
        } else {
            let address = address
                .split('-')
                .map(|x| u8::from_str_radix(x, 16).unwrap())
                .collect::<Vec<u8>>();
            Self {
                address: address.try_into().unwrap(),
            }
        }
    }
}

impl From<&str> for BluetoothAdrr {
    fn from(address: &str) -> Self {
        if address.contains(':') {
            let address = address
                .split(':')
                .map(|x| u8::from_str_radix(x, 16).unwrap())
                .collect::<Vec<u8>>();
            Self {
                address: address.try_into().unwrap(),
            }
        } else {
            let address = address
                .split('-')
                .map(|x| u8::from_str_radix(x, 16).unwrap())
                .collect::<Vec<u8>>();
            Self {
                address: address.try_into().unwrap(),
            }
        }
    }
}

#[cfg(target_os = "windows")]
impl From<BLUETOOTH_DEVICE_INFO_STRUCT> for BluetoothDevice {
    fn from(device_info: BLUETOOTH_DEVICE_INFO_STRUCT) -> BluetoothDevice {
        BluetoothDevice {
            name: String::from_utf16_lossy(&device_info.szName).replace('\0', ""),
            address: BluetoothAdrr::from(device_info.Address),
            connected: device_info.fConnected.as_bool(),
            remembered: device_info.fRemembered.as_bool(),
        }
    }
}

#[cfg(target_os = "windows")]
impl From<BLUETOOTH_ADDRESS_STRUCT> for BluetoothAdrr {
    fn from(address: BLUETOOTH_ADDRESS_STRUCT) -> BluetoothAdrr {
        let mut bytes;
        /* Safety: Union type defined by Microsoft docs */
        unsafe {
            bytes = address.Anonymous.rgBytes;
        }
        bytes.reverse();
        BluetoothAdrr { address: bytes }
    }
}

impl Display for BluetoothDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "- Name: {}\n\tAddress: {} Connected: {} Remembered: {}",
            self.name,
            self.address,
            self.connected,
            self.remembered
        )
    }
}

impl Debug for BluetoothDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "- Name: {}\n\tAddress: {} Connected: {} Remembered: {}",
            self.name,
            self.address,
            self.connected,
            self.remembered
        )
    }
}


impl Display for BluetoothAdrr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.address[0],
            self.address[1],
            self.address[2],
            self.address[3],
            self.address[4],
            self.address[5]
        )
    }
}
