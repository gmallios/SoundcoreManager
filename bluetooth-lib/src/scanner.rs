use windows::Win32::{
    Devices::Bluetooth::{
        BluetoothFindFirstDevice, BluetoothFindNextDevice, BLUETOOTH_ADDRESS_STRUCT,
        BLUETOOTH_DEVICE_INFO_STRUCT, BLUETOOTH_DEVICE_SEARCH_PARAMS,
    },
    Foundation::{BOOL, HANDLE},
    Networking::WinSock::{WSACleanup, WSAStartup, WSADATA},
};

use crate::{util::init_winsock, types::{ BluetoothDevice }};

pub struct BthScanner {
    params: BLUETOOTH_DEVICE_SEARCH_PARAMS,
    device_info: BLUETOOTH_DEVICE_INFO_STRUCT,
}

impl BthScanner {
    pub fn new() -> BthScanner {
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
                device_info: BLUETOOTH_DEVICE_INFO_STRUCT {
                    dwSize: std::mem::size_of::<BLUETOOTH_DEVICE_INFO_STRUCT>() as u32,
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

    pub fn scan(&mut self) -> Vec<BluetoothDevice> {
        let mut devices = Vec::new();
        unsafe {
            let h_device: isize = BluetoothFindFirstDevice(&self.params, &mut self.device_info);
            if h_device != 0 {
                devices.push(BluetoothDevice::from(self.device_info.clone()));
                while BluetoothFindNextDevice(h_device, &mut self.device_info) == BOOL::from(true) {
                    devices.push(BluetoothDevice::from(self.device_info.clone()));
                }
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




