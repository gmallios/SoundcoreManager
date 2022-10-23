use std::{num::ParseIntError, str::Utf8Error, string::ParseError, time::Duration};

use crate::utils::{build_command_array_with_options_toggle_enabled, i8_to_u8vec, Clamp};

use windows::{
    self,
    core::HSTRING,
    Win32::{
        Devices::Bluetooth::{AF_BTH, BTHPROTO_RFCOMM, SOCKADDR_BTH, SOL_RFCOMM},
        Networking::WinSock::{
            closesocket, recv, send, setsockopt, WSACleanup, WSAGetLastError, WSAStartup,
            SEND_RECV_FLAGS, SOCKADDR, SOCKET, SOCKET_ERROR, SOCK_STREAM, SO_RCVTIMEO, SO_SNDTIMEO,
            TIMEVAL, WSADATA, WSA_ERROR,
        },
    },
};

static CMD_DEVICE_STATUS: [i8; 7] = [8, -18, 0, 0, 0, 1, 1];
static CMD_DEVICE_INFO: [i8; 7] = [8, -18, 0, 0, 0, 1, 5];
static CMD_DEVICE_BATTERYLEVEL: [i8; 7] = [8, -18, 0, 0, 0, 1, 3];
static CMD_DEVICE_BATTERYCHARGING: [i8; 7] = [8, -18, 0, 0, 0, 1, 4];
static CMD_DEVICE_LDAC: [i8; 7] = [8, -18, 0, 0, 0, 1, 127]; // NOTE: Last byte is Byte.MAX_VALUE from java. Im not sure about the value
static CMD_DEVICE_GETEQ: [i8; 7] = [8, -18, 0, 0, 0, 2, 1]; // Not tested yet.
static CMD_DEVICE_GETANC: [i8; 7] = [8, -18, 0, 0, 0, 6, 1];
static CMD_DEVICE_SETANC: [i8; 7] = [8, -18, 0, 0, 0, 6, -127];

static SLEEP_DURATION: Duration = std::time::Duration::from_millis(100);

pub const WINAPI_FLAG: SEND_RECV_FLAGS = windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0);

pub struct A3951Device {
    sock: SOCKET,
    //connected: bool,
    // For future use. Not implemented yet. It will enable us to not handle socket/sending/receiving.
    // send_fn: Box<dyn FnMut(&[u8]) -> Result<(), A3951Error>>,
    // recv_fn: Box<dyn FnMut(usize) -> Result<Vec<u8>, Box<dyn std::error::Error>>>,
}

impl A3951Device {
    pub fn new() -> Result<A3951Device, A3951Error> {
        if init_winsock() != 0 {
            return Err(A3951Error::from(windows::core::Error::new(
                windows::core::HRESULT(0),
                HSTRING::from("winsock init error"),
            )));
        }

        Ok(A3951Device {
            sock: create_bt_sock()?,
            //connected: false,
        })
    }

    pub fn connect_uuid(&mut self, mac_addr: &str, uuid: &str) -> Result<(), A3951Error> {
        self.sock = try_connect_uuid(self.sock, mac_addr, uuid)?;
        Ok(())
    }

    //TODO: Check for command in response ( 2 bytes )
    pub fn get_info(&self) -> Result<A3951DeviceInfo, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_INFO);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(50)?;
        Ok(A3951DeviceInfo::from_bytes(&resp)?)
    }

    pub fn get_status(&self) -> Result<A3951DeviceStatus, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_STATUS);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(100)?;
        Ok(A3951DeviceStatus::from_bytes(&resp)?)
    }

    pub fn get_battery_level(&self) -> Result<A3951BatteryLevel, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_BATTERYLEVEL);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(20)?;
        Ok(A3951BatteryLevel::from_bytes(&resp[9..11])?)
    }

    pub fn get_battery_charging(&self) -> Result<A3951BatteryCharging, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_BATTERYCHARGING);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(20)?;
        Ok(A3951BatteryCharging::from_bytes(&resp[9..11])?)
    }

    pub fn get_ldac_status(&self) -> Result<bool, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_LDAC);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(20)?;
        Ok(resp[9] == 1)
    }

    pub fn get_anc(&self) -> Result<A3951DeviceANC, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_GETANC);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(50)?;
        Ok(A3951DeviceANC::from_bytes(&resp[9..13])?)
    }

    pub fn set_anc(&self, anc_profile: A3951DeviceANC) -> Result<(), A3951Error> {
        let cmd = &Self::create_cmd_with_data(CMD_DEVICE_SETANC, anc_profile.to_bytes().to_vec());
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        // Validate resp??
        let _resp = self.recv(10)?;
        Ok(())
    }

    pub fn create_cmd(inp: [i8; 7]) -> Vec<u8> {
        return build_command_array_with_options_toggle_enabled(&i8_to_u8vec(&inp), None);
    }

    pub fn create_cmd_with_data(inp: [i8; 7], data: Vec<u8>) -> Vec<u8> {
        return build_command_array_with_options_toggle_enabled(&i8_to_u8vec(&inp), Some(&data));
    }

    fn send(&self, data: &[u8]) -> Result<(), A3951Error> {
        unsafe {
            if send(self.sock, data, WINAPI_FLAG) == SOCKET_ERROR {
                return Err(A3951Error::from(windows::core::Error::new(
                    windows::core::HRESULT(0),
                    HSTRING::from("send error"),
                )));
            }
        }
        Ok(())
    }

    fn recv(&self, num_of_bytes: usize) -> Result<Vec<u8>, A3951Error> {
        let mut resp: Vec<u8> = vec![0; num_of_bytes];
        unsafe {
            if recv(self.sock, &mut resp, WINAPI_FLAG) == SOCKET_ERROR {
                return Err(A3951Error::from(windows::core::Error::new(
                    windows::core::HRESULT(0),
                    HSTRING::from("recv error"),
                )));
            }
        }
        Ok(resp)
    }

    pub unsafe fn close(&mut self) {
        closesocket(self.sock);
        WSACleanup();
    }
}

impl Drop for A3951Device {
    fn drop(&mut self) {
        unsafe {
            closesocket(self.sock);
            WSACleanup();
        }
    }
}

#[derive(Default, Debug)]
pub struct A3951DeviceInfo {
    pub left_fw: String,
    pub right_fw: String,
    pub sn: String,
}
impl A3951DeviceInfo {
    fn from_bytes(arr: &[u8]) -> Result<A3951DeviceInfo, std::string::FromUtf8Error> {
        Ok(A3951DeviceInfo {
            left_fw: String::from_utf8(arr[9..14].to_vec())?,
            right_fw: String::from_utf8(arr[14..19].to_vec())?,
            sn: String::from_utf8(arr[19..35].to_vec())?,
        })
    }
}

#[derive(Default, Debug)]
pub struct A3951DeviceStatus {
    pub host_device: u8,
    pub tws_status: bool,
    pub battery_level: A3951BatteryLevel,
    pub battery_charging: A3951BatteryCharging,
    pub anc_status: A3951DeviceANC,
    pub side_tone_enabled: bool,
    pub wear_detection_enabled: bool,
    pub touch_tone_enabled: bool,
}

impl A3951DeviceStatus {
    fn from_bytes(arr: &[u8]) -> Result<A3951DeviceStatus, A3951Error> {
        if arr.len() < 93 {
            return Err(A3951Error::Unknown);
        }

        Ok(A3951DeviceStatus {
            host_device: arr[9],
            tws_status: arr[10] == 1,
            battery_level: A3951BatteryLevel::from_bytes(&arr[11..13])?,
            battery_charging: A3951BatteryCharging::from_bytes(&arr[13..15])?,
            anc_status: A3951DeviceANC::from_bytes(&arr[86..90])?,
            side_tone_enabled: arr[90] == 1,
            wear_detection_enabled: arr[91] == 1,
            touch_tone_enabled: arr[92] == 1,
        })
    }
}

#[derive(Default, Debug)]
pub struct A3951BatteryLevel {
    pub left: u8,
    pub right: u8,
}

impl A3951BatteryLevel {
    fn from_bytes(arr: &[u8]) -> Result<A3951BatteryLevel, A3951Error> {
        if arr.len() < 2 {
            return Err(A3951Error::Unknown);
        }

        Ok(A3951BatteryLevel {
            left: Clamp::clamp(arr[0], 0, 5),
            right: Clamp::clamp(arr[1], 0, 5),
        })
    }
}

#[derive(Default, Debug)]
pub struct A3951BatteryCharging {
    pub left: bool,
    pub right: bool,
}

impl A3951BatteryCharging {
    fn from_bytes(arr: &[u8]) -> Result<A3951BatteryCharging, A3951Error> {
        if arr.len() < 2 {
            return Err(A3951Error::Unknown);
        }

        Ok(A3951BatteryCharging {
            left: arr[0] == 1,
            right: arr[1] == 1,
        })
    }
}

#[derive(Default, Debug)]
pub struct A3951DeviceANC {
    pub option: u8,
    pub anc_option: u8,
    pub transparency_option: u8,
    pub anc_custom: u8,
}

impl A3951DeviceANC {
    pub const NORMAL_MODE: A3951DeviceANC = A3951DeviceANC {
        option: 2,
        anc_option: 0,
        transparency_option: 0,
        anc_custom: 6,
    };

    pub const ANC_TRANSPORT_MODE: A3951DeviceANC = A3951DeviceANC {
        option: 0,
        anc_option: 0,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const ANC_OUTDOOR_MODE: A3951DeviceANC = A3951DeviceANC {
        option: 0,
        anc_option: 1,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const ANC_INDOOR_MODE: A3951DeviceANC = A3951DeviceANC {
        option: 0,
        anc_option: 2,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const TRANSPARENCY_FULLY_TRANSPARENT_MODE: A3951DeviceANC = A3951DeviceANC {
        option: 1,
        anc_option: 0,
        transparency_option: 0,
        anc_custom: 6,
    };

    pub const TRANSPARENCY_VOCAL_MODE: A3951DeviceANC = A3951DeviceANC {
        option: 1,
        anc_option: 0,
        transparency_option: 1,
        anc_custom: 6,
    };

    fn from_bytes(arr: &[u8]) -> Result<A3951DeviceANC, std::string::FromUtf8Error> {
        let anc_custom: u8;

        if arr[3] == 255 {
            anc_custom = 255;
        } else {
            anc_custom = Clamp::clamp(arr[3], 0, 10);
        }

        Ok(A3951DeviceANC {
            option: Clamp::clamp(arr[0], 0, 2),
            anc_option: Clamp::clamp(arr[1], 0, 3),
            transparency_option: arr[2],
            anc_custom,
        })
    }

    fn to_bytes(&self) -> [u8; 4] {
        let anc_custom: u8;

        if self.anc_custom == 255 {
            anc_custom = 255;
        } else {
            anc_custom = Clamp::clamp(self.anc_custom, 0, 10);
        }

        [
            Clamp::clamp(self.option, 0, 2),
            Clamp::clamp(self.anc_option, 0, 3),
            self.transparency_option,
            anc_custom,
        ]
    }
}

#[derive(Default)]
pub struct EQWave {
    pos0: f32,
    pos1: f32,
    pos2: f32,
    pos3: f32,
    pos4: f32,
    pos5: f32,
    pos6: f32,
    pos7: f32,
}

impl EQWave {
    fn from_bytes(arr: &[u8]) -> Result<EQWave, A3951Error> {
        if (arr.len() < 8) {
            return Err(A3951Error::Unknown);
        }

        let i8slice = unsafe { &*(arr as *const _ as *const [i8]) };
        let results = Self::eq_int_to_float(i8slice);
        Ok(EQWave {
            pos0: results[0],
            pos1: results[1],
            pos2: results[2],
            pos3: results[3],
            pos4: results[4],
            pos5: results[5],
            pos6: results[6],
            pos7: results[7],
        })
    }

    fn eq_int_to_float(arr: &[i8]) -> Vec<f32> {
        let mut eq: Vec<f32> = Vec::new();
        let max_val: f32 = (12.0 + 7.0) - 1.0;
        let min_val: f32 = (12.0 - 7.0) + 1.0;
        for i in arr {
            let f: f32 = *i as f32 / 10.0;
            if (f > max_val) {
                eq.push(max_val);
            } else if (f < min_val) {
                eq.push(min_val);
            } else {
                eq.push(f);
            }
        }
        eq
    }
}

fn try_connect_uuid(sock: SOCKET, addr: &str, uuid: &str) -> Result<SOCKET, A3951Error> {
    let mut saddr: SOCKADDR_BTH = SOCKADDR_BTH {
        addressFamily: AF_BTH,
        btAddr: crate::utils::mac_str_to_u64(addr)?, // set your bt mac 0xAC122F6AD207
        serviceClassId: windows::core::GUID::from(uuid),
        port: 0,
    };

    unsafe {
        let status = windows::Win32::Networking::WinSock::connect(
            sock,
            &saddr as *const SOCKADDR_BTH as *const SOCKADDR,
            std::mem::size_of::<SOCKADDR_BTH>() as i32,
        );
        if (status == SOCKET_ERROR) {
            let err = WSAGetLastError();
            println!("Error connect socket: {:?}", err);
            closesocket(sock);
            return Err(A3951Error::from(windows::core::Error::new(
                windows::core::HRESULT(0),
                HSTRING::from("error connecting to socket"),
            )));
        }
    }

    return Ok(sock);
}

fn create_bt_sock() -> Result<SOCKET, A3951Error> {
    let mut sock = windows::Win32::Networking::WinSock::INVALID_SOCKET;
    unsafe {
        sock = windows::Win32::Networking::WinSock::socket(
            AF_BTH.into(),
            SOCK_STREAM.into(),
            BTHPROTO_RFCOMM.try_into().unwrap(),
        );
    }
    if (sock == windows::Win32::Networking::WinSock::INVALID_SOCKET) {
        return Err(A3951Error::from(windows::core::Error::new(
            windows::core::HRESULT(0),
            HSTRING::from("failed creating socket"),
        )));
    }
    return Ok(sock);
}

fn init_winsock() -> i32 {
    let wsa_data = Box::into_raw(Box::new(WSADATA::default()));
    let i_result: i32;
    unsafe {
        i_result = WSAStartup(0x0202, wsa_data);
    }
    return i_result;
}

// Not using this
// It takes a long time to find working port or it doesnt work at all ¯\_(ツ)_/¯
// and it requires fixing to return the socket if connection is successfull
// Maybe find a way to change timeout???
// pub(crate) fn try_connect(port: u32) -> i32 {
//     unsafe {
//         let sock = create_bt_sock();

//         if sock == windows::Win32::Networking::WinSock::INVALID_SOCKET {
//             println!("Error create sock");
//             WSACleanup();
//             return -1;
//         } else {
//             println!("Socket created...");

//             let set_result = setsockopt(
//                 sock,
//                 SOL_RFCOMM.try_into().unwrap(),
//                 SO_SNDTIMEO.try_into().unwrap(),
//                 Some(&[1, 0]),
//             );
//             println!("Set timeout: {}", set_result);

//             let mut sa: SOCKADDR_BTH = SOCKADDR_BTH {
//                 addressFamily: AF_BTH,
//                 btAddr: 0xAC122F6AD207, // set your bt mac
//                 serviceClassId: std::mem::zeroed(),
//                 port: port,
//             };

//             let status = windows::Win32::Networking::WinSock::connect(
//                 sock,
//                 &sa as *const SOCKADDR_BTH as *const SOCKADDR,
//                 std::mem::size_of::<SOCKADDR_BTH>() as i32,
//             );
//             if (status == SOCKET_ERROR) {
//                 let err = WSAGetLastError();
//                 println!("Error connect socket: {:?}", err);
//             }
//             closesocket(sock);
//             return status;
//         }
//     }
// }

//TODO: More error types and rewrite winerrors

#[derive(Debug)]
pub enum A3951Error {
    Unknown,
    ParseError,
    WinError(String),
}

impl std::fmt::Display for A3951Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:}", std::error::Error::description(self))
    }
}

impl std::error::Error for A3951Error {
    fn description(&self) -> &str {
        match self {
            A3951Error::Unknown => "Unknown Error",
            A3951Error::ParseError => "Parse Error",
            A3951Error::WinError(ref message) => message.as_str(),
        }
    }
}

impl From<std::io::Error> for A3951Error {
    fn from(_error: std::io::Error) -> Self {
        A3951Error::Unknown
    }
}

impl From<std::num::ParseIntError> for A3951Error {
    fn from(_error: std::num::ParseIntError) -> Self {
        A3951Error::Unknown
    }
}

impl From<windows::core::Error> for A3951Error {
    fn from(error: windows::core::Error) -> Self {
        A3951Error::WinError(error.to_string())
    }
}

impl From<std::string::FromUtf8Error> for A3951Error {
    fn from(_error: std::string::FromUtf8Error) -> Self {
        A3951Error::ParseError
    }
}
