use windows::Win32::Devices::Bluetooth::{BLUETOOTH_ADDRESS_STRUCT, BLUETOOTH_DEVICE_INFO_STRUCT};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BluetoothDevice {
    name: String,
    address: BluetoothAdrr,
    connected: bool,
    remembered: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BluetoothAdrr {
    address: [u8; 6],
}

impl From<BLUETOOTH_DEVICE_INFO_STRUCT> for BluetoothDevice {
    fn from(device_info: BLUETOOTH_DEVICE_INFO_STRUCT) -> BluetoothDevice {
        BluetoothDevice {
            name: String::from_utf16_lossy(&device_info.szName).replace("\0", ""),
            address: BluetoothAdrr::from(device_info.Address),
            connected: device_info.fConnected.as_bool(),
            remembered: device_info.fRemembered.as_bool(),
        }
    }
}

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



impl std::fmt::Display for BluetoothDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "- Name: {}\n\tAddress: {:X?} Connected: {} Remembered: {}",
            self.name, self.address, self.connected, self.remembered
        )
    }
}

impl ToString for BluetoothAdrr {
    fn to_string(&self) -> String {
        format!(
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