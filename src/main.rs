#![allow(warnings, unused)]
use core::time;
use std::{fmt::Write, num::ParseIntError, ops::Add, thread, time::Duration};

use A3951::A3951Device;

use windows::{
    self,
    core::HSTRING,
    Win32::{
        Devices::Bluetooth::{AF_BTH, BTHPROTO_RFCOMM, SOCKADDR_BTH, SOL_RFCOMM},
        Networking::WinSock::{
            closesocket, setsockopt, WSACleanup, WSAGetLastError, WSAStartup, SOCKADDR, SOCKET,
            SOCKET_ERROR, SOCK_STREAM, SO_RCVTIMEO, SO_SNDTIMEO, TIMEVAL, WSADATA, WSA_ERROR,
        },
    },
};

use crate::A3951::A3951DeviceANC;

mod A3951;
mod utils;

// Modes
// const TRANSPORT_MODE: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x01\x01\x01\x00\x8e";
// const NORMAL_MODE: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x02\x01\x01\x00\x8f";
// const ANC_INDOOR: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x02\x01\x00\x8e";
// const ANC_OUTDOOR: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x01\x01\x00\x8d";
// const ANC_TRANSPORT: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x00\x01\x00\x8c";
const known_uuids: [&str; 4] = [
    "00001101-0000-1000-8000-00805F9B34FB",
    "66666666-6666-6666-6666-666666666666",
    "77777777-7777-7777-7777-777777777777",
    "00002902-0000-1000-8000-00805f9b34fb",
];

const BYTE_OFF: i32 = -1;

const OPCODE_BAT: [u8; 7] = [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x05];

fn main() {
    let mut device = A3951Device::new().unwrap();
    device
        .connect_uuid("AC:12:2F:6A:D2:07", known_uuids[0])
        .unwrap();
    let info = device.get_info().unwrap();
    println!(
        "SN: {}, Right FW: {}, Left FW: {}",
        info.sn, info.right_fw, info.left_fw
    );
    let status = device.get_status().unwrap();
    println!(
        "Left Battery: {:X?}, LeftCharging: {:X?}, Right Battery: {:X?}, RightCharging: {:X?},",
        status.left_battery_level, status.left_battery_charging, status.right_battery_level, status.right_battery_charging
    );
    println!("ANC Status: {:?}", status.anc_status);

    println!("LDAC Status: {}", device.get_ldac_status().unwrap());
    
    device.set_anc(A3951DeviceANC::ANC_INDOOR_MODE).unwrap();
    


    // let signed2: Vec<i8> = vec![8, -18, 0, 0, 0, 1, 1];
    // let bytes = utils::i8vec_to_u8vec(signed2);
    // println!("{:?}", bytes);

    // let inp: &[u8] = &decode_hex("08ee0000000101").unwrap();
    // println!("inp: {:?}", inp);
    // println!(
    //     "out: {:?}",
    //     build_command_array_with_options_toggle_enabled(inp, None)
    // );

    // // Liberty Air 2 Pro working uuid known_uuids[0]
    // let status = try_connect_uuid(known_uuids[0]);
    // println!("Connect status: {}", status.0);
    // match status.1 {
    //     Some(socket) => {
    //         unsafe {
    //             loop {
    //                 get_A3195_info(socket);
    //                 // let UKNOWN_OPCODE = b"\x08\xee\x00\x00\x00\x01\x01\n\x00\x02";
    //                 // let AUKNOWN_OPCODE = b"\x08\xff\xff\xff\xee\x00\x00\x00\x01\x03";
    //                 // let buf = decode_hex("08ee00000001010a0002").unwrap();
    //                 // let ret = windows::Win32::Networking::WinSock::send(
    //                 //     socket,
    //                 //     &buf,
    //                 //     windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
    //                 // );
    //                 // println!("send status: {}", ret);
    //                 // if (ret == SOCKET_ERROR) {
    //                 //     println!("send error code: {:?}", WSAGetLastError());
    //                 // }
    //                 // // println!("send bytes: {:?}", b"\x08\xee\x00\x00\x00\x01\x01\n\x00\x02");
    //                 // let buf: &mut [u8; 300] = &mut [0; 300];
    //                 // //thread::sleep(Duration::from_millis(500));

    //                 // windows::Win32::Networking::WinSock::recv(
    //                 //     socket,
    //                 //     buf,
    //                 //     windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
    //                 // );
    //                 // println!("Recived from socket {:X?}", buf);
    //                 // println!("Battery: {:X?}", buf[11]);
    //                 // println!("Battery: {:X?}", buf[12]);
    //                 // // println!("Charging: {}", buf[10]);
    //                 // // println!("ANC Mode: {:?}", &buf[96]);
    //                 // // println!("Case battery: {:X?}", &buf[96]);
    //                 // thread::sleep(Duration::from_millis(500));
    //                 // //let _ = send_rfcomm(socket, ANC_INDOOR);
    //                 print!("\x1B[2J\x1B[1;1H");
    //             }
    //             closesocket(socket);
    //         }
    //     }
    //     None => (),
    // }

    //  unsafe {
    //      // 1-31
    //      for port in 1..32 {
    //          println!("Trying port: {}", port);
    //          let status = try_connect(port);
    //          println!("Connect status: {}", status);
    //          thread::sleep(Duration::from_secs(1));
    //      }
    //  }
}

// fn get_A3195_info(sock: SOCKET) {
//     let cmd = build_command_array_with_options_toggle_enabled(
//         &A3951::create_A3951_command(A3951::CMD_DEVICE_INFO),
//         None,
//     );

//     send_rfcomm(sock, &cmd);

//     thread::sleep(Duration::from_millis(100));

//     unsafe {
//         let buf = recv_rfcomm(sock, 100).unwrap();
//         // ParseA3951Info Method from A3951DeviceManager

//         let HOST_DEVICE = buf[9];
//         let TWS_STATUS = buf[10].eq(&1);
//         let LEFT_BATTERY = buf[11]; // 0-5
//         let RIGHT_BATTERY = buf[12]; // 0-5
//         let LEFT_CHARGING: bool = buf[13].eq(&1);
//         let RIGHT_CHARGING: bool = buf[14].eq(&1);
//         let LEFT_BATTERY = buf[11];
//         let RIGHT_BATTERY = buf[12];
//         let mOption = &buf[86]; // Clamped to 2
//         let mANCOption = &buf[87]; // 0-3
//         let mTransOption = &buf[88]; // 0-2
//         let mANCCustom = &buf[89]; // 0-10 or 255
//         let SIDE_TONE = &buf[90].eq(&1);
//         let WEAR_DETECTION = &buf[91].eq(&1);
//         let TOUCH_TONE = &buf[92].eq(&1);
//         //print bat
//         println!(
//             "Left Battery: {:X?}, LeftCharging: {:X?}, Right Battery: {:X?}, RightCharging: {:X?},",
//             LEFT_BATTERY, LEFT_CHARGING, RIGHT_BATTERY, RIGHT_CHARGING
//         );

//         //print anc
//         println!(
//             "Option: {:X?}, ANCOption: {:X?}, ANCCustom: {:X?}, TransOption: {:X?}",
//             mOption, mANCOption, mANCCustom, mTransOption
//         );
//         //println!("Left  charging: {}", LEFT_CHARGING);
//         //println!("Right charging: {}", RIGHT_CHARGING);
//         //println!("Left  battery: {}", LEFT_BATTERY);
//         //println!("Right battery: {}", RIGHT_BATTERY);
//         println!("Recived from socket {:X?}", &buf);
//     }
// }

// fn get_info(sock: SOCKET) {
//     // Get Info Command ( SN, FW Version )
//     let buf = decode_hex("08ee00000001050a0006").unwrap();
//     unsafe {
//         let ret = windows::Win32::Networking::WinSock::send(
//             sock,
//             &buf,
//             windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
//         );
//         if (ret == SOCKET_ERROR) {
//             println!("send error code: {:?}", WSAGetLastError());
//             return;
//         }

//         let buf: &mut [u8; 50] = &mut [0; 50];
//         thread::sleep(Duration::from_millis(100));

//         windows::Win32::Networking::WinSock::recv(
//             sock,
//             buf,
//             windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
//         );
//         let SN = std::str::from_utf8(&buf[19..35]).unwrap();
//         let RIGHT_FW = std::str::from_utf8(&buf[14..19]).unwrap();
//         let LEFT_FW = std::str::from_utf8(&buf[9..14]).unwrap();
//         println!("SN: {}, Right FW: {}, Left FW: {}", SN, RIGHT_FW, LEFT_FW);

//         //println!("Recived from socket {:?}",  &buf);
//     }
// }

// https://stackoverflow.com/questions/52987181/how-can-i-convert-a-hex-string-to-a-u8-slice
pub fn decode_hex(s: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn encode_hex_to_cmd_str(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        let hexString = format!("{:X}", b);
        if (hexString.len() < 2) {
            write!(&mut s, "0").unwrap();
        }
        write!(&mut s, "{}", hexString).unwrap();
    }
    s
}

// TODO: Remove unwraps and return Result
pub fn build_command_array_with_options(bArr: &[u8], bArr2: Option<&[u8]>) -> Vec<u8> {
    if (bArr2.is_none()) {
        return Vec::from(bArr);
    }
    let mut result = Vec::new();
    let length = bArr.len() + 2;
    let length2 = bArr2.unwrap().len() + length;
    result.copy_from_slice(&bArr);
    result[bArr.len()] = (length2 % 256) as u8;
    result[bArr.len() + 1] = (length2 % 256) as u8;
    result[..length].copy_from_slice(&bArr2.unwrap());
    return result;
}




