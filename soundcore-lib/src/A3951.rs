use crate::{utils::{
    build_command_array_with_options_toggle_enabled, i8_to_u8vec, verify_resp, Clamp,
}, types::{BatteryLevel, BatteryCharging, ANCProfile, EQWave, EQWaveInt, DeviceInfo}, error::SoundcoreError};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use windows::{
    self,
    core::HSTRING,
    Win32::{
        Devices::Bluetooth::{AF_BTH, BTHPROTO_RFCOMM, SOCKADDR_BTH},
        Networking::WinSock::{
            closesocket, recv, send, WSACleanup, WSAGetLastError, WSAStartup,
            SEND_RECV_FLAGS, SOCKADDR, SOCKET, SOCKET_ERROR, SOCK_STREAM, WSADATA, 
        },
    },
};

static CMD_DEVICE_STATUS: [i8; 7] = [8, -18, 0, 0, 0, 1, 1];
static CMD_DEVICE_INFO: [i8; 7] = [8, -18, 0, 0, 0, 1, 5];
static CMD_DEVICE_BATTERYLEVEL: [i8; 7] = [8, -18, 0, 0, 0, 1, 3];
static CMD_DEVICE_BATTERYCHARGING: [i8; 7] = [8, -18, 0, 0, 0, 1, 4];
static CMD_DEVICE_LDAC: [i8; 7] = [8, -18, 0, 0, 0, 1, 127]; // NOTE: Last byte is Byte.MAX_VALUE from java. Im not sure about the value
static CMD_DEVICE_GETEQ: [i8; 7] = [8, -18, 0, 0, 0, 2, 1]; // Not tested yet.
static CMD_DEVICE_SETEQ_DRC: [i8; 7] = [8, -18, 0, 0, 0, 3, -121]; // This gets used when DRC is supported/enabled.
static CMD_DEVICE_SETEQ_NODRC: [i8; 7] = [8, -18, 0, 0, 0, 3, -122]; // This gets used when DRC is not supported/enabled.
static CMD_DEVICE_GETANC: [i8; 7] = [8, -18, 0, 0, 0, 6, 1];
static CMD_DEVICE_SETANC: [i8; 7] = [8, -18, 0, 0, 0, 6, -127];

static SLEEP_DURATION: Duration = std::time::Duration::from_millis(30);

pub const WINAPI_FLAG: SEND_RECV_FLAGS = windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0);

pub struct A3951Device {
    sock: SOCKET,
    //connected: bool,
    // For future use. Not implemented yet. It will enable us to not handle socket/sending/receiving.
    // send_fn: Box<dyn FnMut(&[u8]) -> Result<(), A3951Error>>,
    // recv_fn: Box<dyn FnMut(usize) -> Result<Vec<u8>, Box<dyn std::error::Error>>>,
}

impl A3951Device {
    pub fn new() -> Result<A3951Device, SoundcoreError> {
        if init_winsock() != 0 {
            return Err(SoundcoreError::from(windows::core::Error::new(
                windows::core::HRESULT(0),
                HSTRING::from("winsock init error"),
            )));
        }

        Ok(A3951Device {
            sock: create_bt_sock()?,
            //connected: false,
        })
    }

    pub fn connect_uuid(&mut self, mac_addr: &str, uuid: &str) -> Result<(), SoundcoreError> {
        self.sock = try_connect_uuid(self.sock, mac_addr, uuid)?;
        Ok(())
    }

    //TODO: Check for command in response ( 2 bytes )
    pub fn get_info(&self) -> Result<DeviceInfo, SoundcoreError> {
        let cmd = &Self::create_cmd(CMD_DEVICE_INFO);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(36)?;
        if !verify_resp(&resp) {
            return Err(SoundcoreError::ResponseChecksumError);
        }
        Ok(DeviceInfo::from_bytes(&resp)?)
    }

    pub fn get_status(&self) -> Result<A3951DeviceStatus, SoundcoreError> {
        let cmd = &Self::create_cmd(CMD_DEVICE_STATUS);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(97)?;
        if !verify_resp(&resp) {
            return Err(SoundcoreError::ResponseChecksumError);
        }
        Ok(A3951DeviceStatus::from_bytes(&resp)?)
    }

    pub fn get_battery_level(&self) -> Result<BatteryLevel, SoundcoreError> {
        let cmd = &Self::create_cmd(CMD_DEVICE_BATTERYLEVEL);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(100)?;

        if !verify_resp(&resp[0..12]) {
            return Err(SoundcoreError::ResponseChecksumError);
        }

        if resp[6] == 4 {
            println!("Device level blink: {:?}", resp);
            // Case battery level. Ignore for now, more debugging needed.
            // Battery charging "blinks" when this event is triggered.
            return Err(SoundcoreError::Unknown);
        }

        Ok(BatteryLevel::from_bytes(&resp[9..11])?)
    }

    pub fn get_battery_charging(&self) -> Result<BatteryCharging, SoundcoreError> {
        let cmd = &Self::create_cmd(CMD_DEVICE_BATTERYCHARGING);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(100)?;

        if !verify_resp(&resp[0..12]) {
            return Err(SoundcoreError::ResponseChecksumError);
        }
        // https://prnt.sc/yze5IvvUtYlq Case battery "blink"
        if resp[13] == 255 {
            println!("Device charging blink: {:?}", resp);
            // When "blinking" resp[13] is 255 afaik.
            return Err(SoundcoreError::Unknown);
        }

        Ok(BatteryCharging::from_bytes(&resp[9..11])?)
    }

    pub fn get_ldac_status(&self) -> Result<bool, SoundcoreError> {
        let cmd = &Self::create_cmd(CMD_DEVICE_LDAC);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(11)?;
        if !verify_resp(&resp) {
            return Err(SoundcoreError::ResponseChecksumError);
        }
        Ok(resp[9] == 1)
    }

    pub fn get_anc(&self) -> Result<ANCProfile, SoundcoreError> {
        let cmd = &Self::create_cmd(CMD_DEVICE_GETANC);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(14)?;
        if !verify_resp(&resp) {
            return Err(SoundcoreError::ResponseChecksumError);
        }
        Ok(ANCProfile::from_bytes(&resp[9..13])?)
    }

    pub fn set_anc(&self, anc_profile: ANCProfile) -> Result<(), SoundcoreError> {
        let cmd = &Self::create_cmd_with_data(CMD_DEVICE_SETANC, anc_profile.to_bytes().to_vec());
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        // Validate resp??
        let _resp = self.recv(10)?;
        Ok(())
    }


    pub fn set_eq(&self, eq_wave: EQWave) -> Result<(), SoundcoreError> {
        let drc_supported = true;
        let eq_index: i32 = 65278; /* Custom EQ Index */
        let eq_hindex = 0; /* I don't know what this is, doesn't seem to change across EQ Indexes and EQ values and is constant */
        let arr_len = match drc_supported {
            /* 76: DRC 74: No DRC */
            true => 76,
            false => 74,
        };
        let drc_offset = match drc_supported {
            true => 4,
            false => 2,
        };
        let mut output_arr: Vec<u8> = vec![0; arr_len];

        output_arr[0] = eq_index as u8 & 0xFF;
        output_arr[1] = ((eq_index >> 8) & 0xFF) as u8;

        if drc_supported {
            /* hindex is used on DRC models */
            output_arr[2] = eq_hindex as u8 & 0xFF;
            output_arr[3] = ((eq_hindex >> 8) & 0xFF) as u8;
        }

        /* used for both left and right EQs */
        let corrected_eq_wave = EQWave::transform_to_realeq(eq_wave);
        let eq_wave_bytes = EQWaveInt::from_eq_wave(eq_wave).to_bytes(); 
        let corrected_eq_wave_bytes = EQWaveInt::from_eq_wave(corrected_eq_wave).to_bytes(); 
        let hearid_wave_bytes = EQWaveInt::from_eq_wave(EQWave::HEARD_ID_DEFAULT).to_bytes();

        /* drc_offset - drc_offset + 16 EQ Wave */
        output_arr[drc_offset..drc_offset+8].copy_from_slice(&eq_wave_bytes[0..8]);
        output_arr[drc_offset+8..drc_offset+16].copy_from_slice(&eq_wave_bytes[0..8]);
        /* Straight from Soundcore spaghetti */
        output_arr[drc_offset+16] = ((-1 & -1) & 255) as u8; 
        output_arr[drc_offset+17] = ((-1 & -1) & 255) as u8; 
        output_arr[drc_offset+18] = (0 & 255) as u8;
        /* drc_offset + 19-35 HearID EQ Wave */
        output_arr[drc_offset+19..drc_offset+27].copy_from_slice(&hearid_wave_bytes[0..8]);
        output_arr[drc_offset+27..drc_offset+35].copy_from_slice(&hearid_wave_bytes[0..8]);

        output_arr[drc_offset+35..drc_offset+39].copy_from_slice(&[0, 0, 0, 0]);
        output_arr[drc_offset+39] = (0 & 255) as u8; /* HearID type */

        /* drc_offset + 40-56 HearID Customer EQ Wave (IDK what this means, hearid data is not reversed atm) */
        output_arr[drc_offset+40..drc_offset+48].copy_from_slice(&hearid_wave_bytes[0..8]);
        output_arr[drc_offset+48..drc_offset+56].copy_from_slice(&hearid_wave_bytes[0..8]);

        /* drc_offset + 56-72 "Corrected" EQ Wave */
        output_arr[drc_offset+56..drc_offset+64].copy_from_slice(&corrected_eq_wave_bytes[0..8]);
        output_arr[drc_offset+64..drc_offset+72].copy_from_slice(&corrected_eq_wave_bytes[0..8]);
        let cmd = Self::create_cmd_with_data(CMD_DEVICE_SETEQ_DRC, output_arr);
        self.send(&cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let _resp = self.recv(100)?;
        Ok(())
    }

    pub fn create_cmd(inp: [i8; 7]) -> Vec<u8> {
        return build_command_array_with_options_toggle_enabled(&i8_to_u8vec(&inp), None);
    }

    pub fn create_cmd_with_data(inp: [i8; 7], data: Vec<u8>) -> Vec<u8> {
        return build_command_array_with_options_toggle_enabled(&i8_to_u8vec(&inp), Some(&data));
    }

    fn send(&self, data: &[u8]) -> Result<(), SoundcoreError> {
        unsafe {
            if send(self.sock, data, WINAPI_FLAG) == SOCKET_ERROR {
                return Err(SoundcoreError::from(windows::core::Error::new(
                    windows::core::HRESULT(0),
                    HSTRING::from("send error"),
                )));
            }
        }
        Ok(())
    }

    fn recv(&self, num_of_bytes: usize) -> Result<Vec<u8>, SoundcoreError> {
        let mut resp: Vec<u8> = vec![0; num_of_bytes];
        unsafe {
            if recv(self.sock, &mut resp, WINAPI_FLAG) == SOCKET_ERROR {
                return Err(SoundcoreError::from(windows::core::Error::new(
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

impl DeviceInfo {
    fn from_bytes(arr: &[u8]) -> Result<DeviceInfo, std::string::FromUtf8Error> {
        Ok(DeviceInfo {
            left_fw: String::from_utf8(arr[9..14].to_vec())?,
            right_fw: String::from_utf8(arr[14..19].to_vec())?,
            sn: String::from_utf8(arr[19..35].to_vec())?,
        })
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct A3951DeviceStatus {
    pub host_device: u8,
    pub tws_status: bool,
    pub battery_level: BatteryLevel,
    pub battery_charging: BatteryCharging,
    pub anc_status: ANCProfile,
    pub side_tone_enabled: bool,
    pub wear_detection_enabled: bool,
    pub touch_tone_enabled: bool,
    pub left_eq: EQWave,
    pub right_eq: EQWave,
    pub hearid_enabled: bool,
    pub left_hearid: EQWave,
    pub right_hearid: EQWave,
    pub left_hearid_customdata: EQWave,
    pub right_hearid_customdata: EQWave,
}

impl A3951DeviceStatus {
    fn from_bytes(arr: &[u8]) -> Result<A3951DeviceStatus, SoundcoreError> {
        if arr.len() < 93 {
            return Err(SoundcoreError::Unknown);
        }

        Ok(A3951DeviceStatus {
            host_device: arr[9],
            tws_status: arr[10] == 1,
            battery_level: BatteryLevel::from_bytes(&arr[11..13])?,
            battery_charging: BatteryCharging::from_bytes(&arr[13..15])?,
            left_eq: EQWave::from_bytes(&arr[17..25])?,
            right_eq: EQWave::from_bytes(&arr[25..33])?,
            hearid_enabled: arr[35] == 1,
            left_hearid: EQWave::from_bytes(&arr[36..44])?,
            right_hearid: EQWave::from_bytes(&arr[44..52])?,
            left_hearid_customdata: EQWave::from_bytes(&arr[58..66])?,
            right_hearid_customdata: EQWave::from_bytes(&arr[66..74])?,
            anc_status: ANCProfile::from_bytes(&arr[86..90])?,
            side_tone_enabled: arr[90] == 1,
            wear_detection_enabled: arr[91] == 1,
            touch_tone_enabled: arr[92] == 1,
        })
    }
}


impl BatteryLevel {
    fn from_bytes(arr: &[u8]) -> Result<BatteryLevel, SoundcoreError> {
        if arr.len() < 2 {
            return Err(SoundcoreError::Unknown);
        }

        Ok(BatteryLevel {
            left: Clamp::clamp(arr[0], 0, 5),
            right: Clamp::clamp(arr[1], 0, 5),
        })
    }
}

impl BatteryCharging {
    fn from_bytes(arr: &[u8]) -> Result<BatteryCharging, SoundcoreError> {
        if arr.len() < 2 {
            return Err(SoundcoreError::Unknown);
        }

        Ok(BatteryCharging {
            left: arr[0] == 1,
            right: arr[1] == 1,
        })
    }
}

impl ANCProfile {
    pub const NORMAL_MODE: ANCProfile = ANCProfile {
        option: 2,
        anc_option: 0,
        transparency_option: 0,
        anc_custom: 6,
    };

    pub const ANC_TRANSPORT_MODE: ANCProfile = ANCProfile {
        option: 0,
        anc_option: 0,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const ANC_OUTDOOR_MODE: ANCProfile = ANCProfile {
        option: 0,
        anc_option: 1,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const ANC_INDOOR_MODE: ANCProfile = ANCProfile {
        option: 0,
        anc_option: 2,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub const TRANSPARENCY_FULLY_TRANSPARENT_MODE: ANCProfile = ANCProfile {
        option: 1,
        anc_option: 0,
        transparency_option: 0,
        anc_custom: 6,
    };

    pub const TRANSPARENCY_VOCAL_MODE: ANCProfile = ANCProfile {
        option: 1,
        anc_option: 0,
        transparency_option: 1,
        anc_custom: 6,
    };

    pub fn anc_custom_value(val: u8) -> ANCProfile {
        ANCProfile {
            option: 0,
            anc_option: 3,
            transparency_option: 1,
            anc_custom: Clamp::clamp(val, 0, 10),
        }
    }

    fn from_bytes(arr: &[u8]) -> Result<ANCProfile, std::string::FromUtf8Error> {
        let anc_custom: u8;

        if arr[3] == 255 {
            anc_custom = 255;
        } else {
            anc_custom = Clamp::clamp(arr[3], 0, 10);
        }

        Ok(ANCProfile {
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


fn try_connect_uuid(sock: SOCKET, addr: &str, uuid: &str) -> Result<SOCKET, SoundcoreError> {
    let saddr: SOCKADDR_BTH = SOCKADDR_BTH {
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
        if status == SOCKET_ERROR {
            let err = WSAGetLastError();
            println!("Error connect socket: {:?}", err);
            closesocket(sock);
            return Err(SoundcoreError::from(windows::core::Error::new(
                windows::core::HRESULT(0),
                HSTRING::from("error connecting to socket"),
            )));
        }
    }

    return Ok(sock);
}

fn create_bt_sock() -> Result<SOCKET, SoundcoreError> {
    let sock;
    unsafe {
        sock = windows::Win32::Networking::WinSock::socket(
            AF_BTH.into(),
            SOCK_STREAM.into(),
            BTHPROTO_RFCOMM.try_into().unwrap(),
        );
    }
    if sock == windows::Win32::Networking::WinSock::INVALID_SOCKET {
        return Err(SoundcoreError::from(windows::core::Error::new(
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
