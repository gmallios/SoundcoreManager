#![allow(warnings, unused)]
use core::time;
use std::{fmt::Write, num::ParseIntError, ops::Add, thread, time::Duration};

use crate::devices::A3951;

use crate::types::{EQWave, RecvFnType, SendFnType};

mod base;
mod cli;
mod devices;
mod error;
mod statics;
mod types;
mod utils;

// Modes
// const TRANSPORT_MODE: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x01\x01\x01\x00\x8e";
// const NORMAL_MODE: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x02\x01\x01\x00\x8f";
// const ANC_INDOOR: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x02\x01\x00\x8e";
// const ANC_OUTDOOR: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x01\x01\x00\x8d";
// const ANC_TRANSPORT: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x00\x01\x00\x8c";
const BYTE_OFF: i32 = -1;

const OPCODE_BAT: [u8; 7] = [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x05];

fn main() {
    // let mut device = A3951Device::new().unwrap();
    // device
    //     .connect_uuid("AC:12:2F:6A:D2:07", known_uuids[0])
    //     .unwrap();
    // let info = device.get_info().unwrap();
    // println!(
    //     "SN: {}, Right FW: {}, Left FW: {}",
    //     info.sn, info.right_fw, info.left_fw
    // );
    // let status = device.get_status().unwrap();
    // println!(
    //     "Left Battery: {:X?}, LeftCharging: {:X?}, Right Battery: {:X?}, RightCharging: {:X?},",
    //     status.battery_level.left, status.battery_charging.left, status.battery_level.right, status.battery_charging.right,
    // );

    // println!("Left EQ: {:X?}", status.left_eq);
    // println!("Right EQ: {:X?}", status.right_eq);
    // println!("ANC Status: {:?}", status.anc_status);
    // println!("ANC Status from get_anc: {:?}", device.get_anc().unwrap());
    // println!("LDAC Status: {}", device.get_ldac_status().unwrap());

    // //device.set_anc(A3951DeviceANC::ANC_INDOOR_MODE).unwrap();

    // let wave = EQWave {
    //     pos0: 14.5,
    //     pos1: 13.0,
    //     pos2: 12.0,
    //     pos3: 12.0,
    //     pos4: 12.0,
    //     pos5: 12.0,
    //     pos6: 12.0,
    //     pos7: 12.0,
    //     pos8: 12.0,
    //     pos9: 12.0,
    // };

    // device.set_eq(wave);
    // let status1 = device.get_status().unwrap();
    // println!("Left EQ: {:X?}", status1.left_eq);
    // println!("Right EQ: {:X?}", status1.right_eq);
}

// // https://stackoverflow.com/questions/52987181/how-can-i-convert-a-hex-string-to-a-u8-slice
// pub fn decode_hex(s: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
//     (0..s.len())
//         .step_by(2)
//         .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
//         .collect()
// }

// pub fn encode_hex(bytes: &[u8]) -> String {
//     let mut s = String::with_capacity(bytes.len() * 2);
//     for &b in bytes {
//         write!(&mut s, "{:02x}", b).unwrap();
//     }
//     s
// }

// pub fn encode_hex_to_cmd_str(bytes: &[u8]) -> String {
//     let mut s = String::with_capacity(bytes.len() * 2);
//     for &b in bytes {
//         let hexString = format!("{:X}", b);
//         if (hexString.len() < 2) {
//             write!(&mut s, "0").unwrap();
//         }
//         write!(&mut s, "{}", hexString).unwrap();
//     }
//     s
// }

// // TODO: Remove unwraps and return Result
// pub fn build_command_array_with_options(bArr: &[u8], bArr2: Option<&[u8]>) -> Vec<u8> {
//     if (bArr2.is_none()) {
//         return Vec::from(bArr);
//     }
//     let mut result = Vec::new();
//     let length = bArr.len() + 2;
//     let length2 = bArr2.unwrap().len() + length;
//     result.copy_from_slice(&bArr);
//     result[bArr.len()] = (length2 % 256) as u8;
//     result[bArr.len() + 1] = (length2 % 256) as u8;
//     result[..length].copy_from_slice(&bArr2.unwrap());
//     return result;
// }
