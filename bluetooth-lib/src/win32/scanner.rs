use async_trait::async_trait;
use windows::Win32::{
    Devices::Bluetooth::{
        BluetoothFindFirstDevice, BluetoothFindNextDevice, BLUETOOTH_DEVICE_INFO,
        BLUETOOTH_DEVICE_SEARCH_PARAMS,
    },
    Foundation::{BOOL, HANDLE},
    Networking::WinSock::WSACleanup,
};

use crate::{
    types::{BluetoothDevice, Scanner},
    win32::util::init_winsock,
};

pub struct BthScanner {
    params: BLUETOOTH_DEVICE_SEARCH_PARAMS,
    device_info: BLUETOOTH_DEVICE_INFO,
}

#[async_trait]
impl Scanner for BthScanner {
    fn new() -> BthScanner {
        init_winsock();
        /* Safety: Zeroed Memory */
        unsafe {
            BthScanner {
                params: BLUETOOTH_DEVICE_SEARCH_PARAMS {
                    dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32,
                    fReturnAuthenticated: BOOL::from(true),
                    fReturnRemembered: BOOL::from(true),
                    fReturnUnknown: BOOL::from(true),
                    fReturnConnected: BOOL::from(true),
                    fIssueInquiry: BOOL::from(true),
                    cTimeoutMultiplier: 1,
                    hRadio: HANDLE::default(),
                },
                device_info: BLUETOOTH_DEVICE_INFO {
                    dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32,
                    Address: std::mem::zeroed(),
                    ulClassofDevice: std::mem::zeroed(),
                    fConnected: std::mem::zeroed(),
                    fRemembered: std::mem::zeroed(),
                    fAuthenticated: std::mem::zeroed(),
                    stLastSeen: std::mem::zeroed(),
                    stLastUsed: std::mem::zeroed(),
                    szName: std::mem::zeroed(),
                },
            }
        }
    }

    async fn scan(&mut self) -> Vec<BluetoothDevice> {
        let mut devices = Vec::new();
        unsafe {
            let h_device = BluetoothFindFirstDevice(&self.params, &mut self.device_info).unwrap();
            devices.push(BluetoothDevice::from(self.device_info));
            while BluetoothFindNextDevice(h_device, &mut self.device_info) == BOOL::from(true) {
                devices.push(BluetoothDevice::from(self.device_info));
            }
        }
        devices
    }
}

impl Drop for BthScanner {
    fn drop(&mut self) {
        unsafe {
            WSACleanup();
        }
    }
}
