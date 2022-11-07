use crate::utils::{
    build_command_array_with_options_toggle_enabled, i8_to_u8vec, verify_resp, Clamp,
};
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
        let resp = self.recv(36)?;
        if !verify_resp(&resp) {
            return Err(A3951Error::ResponseChecksumError);
        }
        Ok(A3951DeviceInfo::from_bytes(&resp)?)
    }

    pub fn get_status(&self) -> Result<A3951DeviceStatus, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_STATUS);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(97)?;
        if !verify_resp(&resp) {
            return Err(A3951Error::ResponseChecksumError);
        }
        Ok(A3951DeviceStatus::from_bytes(&resp)?)
    }

    pub fn get_battery_level(&self) -> Result<A3951BatteryLevel, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_BATTERYLEVEL);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(100)?;

        if !verify_resp(&resp[0..12]) {
            return Err(A3951Error::ResponseChecksumError);
        }

        if resp[6] == 4 {
            println!("Device level blink: {:?}", resp);
            // Case battery level. Ignore for now, more debugging needed.
            // Battery charging "blinks" when this event is triggered.
            return Err(A3951Error::Unknown);
        }

        Ok(A3951BatteryLevel::from_bytes(&resp[9..11])?)
    }

    pub fn get_battery_charging(&self) -> Result<A3951BatteryCharging, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_BATTERYCHARGING);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(100)?;

        if !verify_resp(&resp[0..12]) {
            return Err(A3951Error::ResponseChecksumError);
        }
        // https://prnt.sc/yze5IvvUtYlq Case battery "blink"
        if resp[13] == 255 {
            println!("Device charging blink: {:?}", resp);
            // When "blinking" resp[13] is 255 afaik.
            return Err(A3951Error::Unknown);
        }

        Ok(A3951BatteryCharging::from_bytes(&resp[9..11])?)
    }

    pub fn get_ldac_status(&self) -> Result<bool, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_LDAC);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(11)?;
        if !verify_resp(&resp) {
            return Err(A3951Error::ResponseChecksumError);
        }
        Ok(resp[9] == 1)
    }

    pub fn get_anc(&self) -> Result<A3951DeviceANC, A3951Error> {
        let cmd = &Self::create_cmd(CMD_DEVICE_GETANC);
        self.send(cmd)?;
        std::thread::sleep(SLEEP_DURATION);
        let resp = self.recv(14)?;
        if !verify_resp(&resp) {
            return Err(A3951Error::ResponseChecksumError);
        }
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


    pub fn set_eq(&self, eq_wave: EQWave) -> Result<(), A3951Error> {
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

#[derive(Default, Debug, Serialize, Deserialize)]
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct A3951DeviceStatus {
    pub host_device: u8,
    pub tws_status: bool,
    pub battery_level: A3951BatteryLevel,
    pub battery_charging: A3951BatteryCharging,
    pub anc_status: A3951DeviceANC,
    pub side_tone_enabled: bool,
    pub wear_detection_enabled: bool,
    pub touch_tone_enabled: bool,
    pub left_eq: EQWave,
    pub right_eq: EQWave,
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
            left_eq: EQWave::from_bytes(&arr[17..25])?,
            right_eq: EQWave::from_bytes(&arr[25..33])?,
            anc_status: A3951DeviceANC::from_bytes(&arr[86..90])?,
            side_tone_enabled: arr[90] == 1,
            wear_detection_enabled: arr[91] == 1,
            touch_tone_enabled: arr[92] == 1,
        })
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
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

#[derive(Default, Debug, Serialize, Deserialize)]
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

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

    pub fn anc_custom_value(val: u8) -> A3951DeviceANC {
        A3951DeviceANC {
            option: 0,
            anc_option: 3,
            transparency_option: 1,
            anc_custom: Clamp::clamp(val, 0, 10),
        }
    }

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

/* This gets sent to the device. EQWave is transformed into this. */
#[derive(Default, Debug)]
pub struct EQWaveInt {
    pos0: i16,
    pos1: i16,
    pos2: i16,
    pos3: i16,
    pos4: i16,
    pos5: i16,
    pos6: i16,
    pos7: i16,
    pos8: i16,
    pos9: i16,
}

impl EQWaveInt {
    fn from_eq_wave(eq: EQWave) -> EQWaveInt {
        const F: f32 = 10.0; /* Constant derived from method usage in the Soundcore App */
        EQWaveInt {
            pos0: (eq.pos0 * F).round() as i16,
            pos1: (eq.pos1 * F).round() as i16,
            pos2: (eq.pos2 * F).round() as i16,
            pos3: (eq.pos3 * F).round() as i16,
            pos4: (eq.pos4 * F).round() as i16,
            pos5: (eq.pos5 * F).round() as i16,
            pos6: (eq.pos6 * F).round() as i16,
            pos7: (eq.pos7 * F).round() as i16,
            pos8: (eq.pos8 * F).round() as i16,
            pos9: (eq.pos9 * F).round() as i16,
        }
    }

    fn to_bytes(&self) -> [u8; 10] {
        [
            (self.pos0 as u8) & 255,
            (self.pos1 as u8) & 255,
            (self.pos2 as u8) & 255,
            (self.pos3 as u8) & 255,
            (self.pos4 as u8) & 255,
            (self.pos5 as u8) & 255,
            (self.pos6 as u8) & 255,
            (self.pos7 as u8) & 255,
            (self.pos8 as u8) & 255,
            (self.pos9 as u8) & 255,
        ]
    }
}

/* This gets received from the device and is used to create the EQ to send. */
#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct EQWave {
    pub pos0: f32,
    pub pos1: f32,
    pub pos2: f32,
    pub pos3: f32,
    pub pos4: f32,
    pub pos5: f32,
    pub pos6: f32,
    pub pos7: f32,
    pub pos8: f32,
    pub pos9: f32,
}

impl EQWave {

    pub const HEARD_ID_DEFAULT: EQWave = EQWave {
        pos0: 25.5,
        pos1: 25.5,
        pos2: 25.5,
        pos3: 25.5,
        pos4: 25.5,
        pos5: 25.5,
        pos6: 25.5,
        pos7: 25.5,
        pos8: 25.5,
        pos9: 25.5,
    };

    fn from_bytes(arr: &[u8]) -> Result<EQWave, A3951Error> {
        if arr.len() < 8 {
            return Err(A3951Error::Unknown);
        }

        let results = Self::eq_int_to_float(arr);
        Ok(EQWave {
            pos0: arr[0] as f32 / 10.0, //6.0 - 18.0
            pos1: results[1],
            pos2: results[2],
            pos3: results[3],
            pos4: results[4],
            pos5: results[5],
            pos6: results[6],
            pos7: results[7],
            /* Since A3951 uses 8-band eq these are constant */
            pos8: 12.0,
            pos9: 0.0,
        })
    }

    fn eq_int_to_float(arr: &[u8]) -> Vec<f32> {
        let mut eq: Vec<f32> = Vec::new();
        let max_val: f32 = (12.0 + 7.0) - 1.0;
        let min_val: f32 = (12.0 - 7.0) + 1.0;
        for i in arr {
            let f: f32 = *i as f32 / 10.0;
            if f > max_val {
                eq.push(max_val);
            } else if f < min_val {
                eq.push(min_val);
            } else {
                eq.push(f);
            }
        }
        eq
    }

    /* A3951 "Needs" drc, other devices might not (see m10061y0 in jadx) */
    fn transform_to_realeq(input_wave: EQWave) -> EQWave {
        Self::transform_addsub(
            Self::apply_drc(Self::transform_addsub(input_wave, false, 12.0)),
            true,
            12.0,
        )
    }

    fn apply_drc(mut input_wave: EQWave) -> EQWave {
        /* Spaghetti code, ported straight from Soundcore App */
        const EQCONST_A: f32 = 0.85;
        const EQCONST_B: f32 = 0.95;
        let (d, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12) = (
            input_wave.pos0 as f64,
            input_wave.pos1 as f64,
            EQCONST_A as f64,
            input_wave.pos2 as f64,
            input_wave.pos3 as f64,
            input_wave.pos4 as f64,
            input_wave.pos5 as f64,
            input_wave.pos6 as f64,
            input_wave.pos7 as f64,
            EQCONST_B as f64,
            (input_wave.pos2 * 0.81 * EQCONST_A) as f64,
            (input_wave.pos5 * 0.81 * EQCONST_A) as f64,
        );
        input_wave.pos0 = ((((((((1.26 * d) - ((d2 * 0.71) * d3)) + (d4 * 0.177))
            - (d5 * 0.0494))
            + (d6 * 0.0345))
            - (d7 * 0.0197))
            + (d8 * 0.0075))
            - (0.00217 * d9)) as f32;
        input_wave.pos1 = ((((((((((-0.71) * d) * d3) + ((d2 * 1.73) * d10)) - d11)
            + (d5 * 0.204))
            - (d6 * 0.068))
            + (d7 * 0.045))
            - (d8 * 0.0235))
            + (d9 * 0.0075)) as f32;
        input_wave.pos2 = ((((((((d * 0.177) - ((d2 * 0.81) * d3)) + ((d4 * 1.73) * d10))
            - ((d5 * 0.81) * d3))
            + (d6 * 0.208))
            - (d7 * 0.07))
            + (d8 * 0.045))
            - (d9 * 0.0197)) as f32;
        input_wave.pos3 = (((((((((-0.0494) * d) + (d2 * 0.204)) - d11) + ((d5 * 1.73) * d10))
            - ((d6 * 0.82) * d3))
            + (d7 * 0.208))
            - (d8 * 0.068))
            + (d9 * 0.0345)) as f32;
        input_wave.pos4 = ((((((((d * 0.0345) - (d2 * 0.068)) + (d4 * 0.208))
            - ((0.82 * d5) * d3))
            + ((d6 * 1.73) * d10))
            - d12)
            + (d8 * 0.204))
            - (d9 * 0.0494)) as f32;
        input_wave.pos5 = (((((((((-0.0197) * d) + (d2 * 0.045)) - (0.07 * d4)) + (0.208 * d5))
            - ((d6 * 0.81) * d3))
            + ((1.73 * d7) * d10))
            - ((0.81 * d8) * d3))
            + (d9 * 0.177)) as f32;
        input_wave.pos6 = ((((((((d * 0.0075) - (d2 * 0.0235)) + (0.045 * d4)) - (d5 * 0.068))
            + (0.204 * d6))
            - d12)
            + ((1.83 * d8) * d10))
            - ((d9 * 0.71) * d3)) as f32;
        input_wave.pos7 = ((((((((d * (-0.00217)) + (d2 * 0.0075)) - (d4 * 0.0197))
            + (d5 * 0.0345))
            - (d6 * 0.0494))
            + (d7 * 0.177))
            - ((d8 * 0.71) * d3))
            + (d9 * 1.5)) as f32;
        Self::transform_multdiv(input_wave, false, 10.0)
    }

    fn transform_multdiv(mut input_wave: EQWave, should_mult: bool, factor: f32) -> EQWave {
        if should_mult {
            input_wave.pos0 *= factor;
            input_wave.pos1 *= factor;
            input_wave.pos2 *= factor;
            input_wave.pos3 *= factor;
            input_wave.pos4 *= factor;
            input_wave.pos5 *= factor;
            input_wave.pos6 *= factor;
            input_wave.pos7 *= factor;
        } else {
            input_wave.pos0 /= factor;
            input_wave.pos1 /= factor;
            input_wave.pos2 /= factor;
            input_wave.pos3 /= factor;
            input_wave.pos4 /= factor;
            input_wave.pos5 /= factor;
            input_wave.pos6 /= factor;
            input_wave.pos7 /= factor;
        }
        input_wave
    }

    fn transform_addsub(mut input_wave: EQWave, should_add: bool, offset: f32) -> EQWave {
        if should_add {
            input_wave.pos0 += offset;
            input_wave.pos1 += offset;
            input_wave.pos2 += offset;
            input_wave.pos3 += offset;
            input_wave.pos4 += offset;
            input_wave.pos5 += offset;
            input_wave.pos6 += offset;
            input_wave.pos7 += offset;
        } else {
            input_wave.pos0 -= offset;
            input_wave.pos1 -= offset;
            input_wave.pos2 -= offset;
            input_wave.pos3 -= offset;
            input_wave.pos4 -= offset;
            input_wave.pos5 -= offset;
            input_wave.pos6 -= offset;
            input_wave.pos7 -= offset;
        }
        input_wave
    }
}

fn try_connect_uuid(sock: SOCKET, addr: &str, uuid: &str) -> Result<SOCKET, A3951Error> {
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
            return Err(A3951Error::from(windows::core::Error::new(
                windows::core::HRESULT(0),
                HSTRING::from("error connecting to socket"),
            )));
        }
    }

    return Ok(sock);
}

fn create_bt_sock() -> Result<SOCKET, A3951Error> {
    let sock;
    unsafe {
        sock = windows::Win32::Networking::WinSock::socket(
            AF_BTH.into(),
            SOCK_STREAM.into(),
            BTHPROTO_RFCOMM.try_into().unwrap(),
        );
    }
    if sock == windows::Win32::Networking::WinSock::INVALID_SOCKET {
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
    ResponseChecksumError,
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
            A3951Error::ResponseChecksumError => "Response Checksum Error",
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
