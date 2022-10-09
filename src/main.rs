#![allow(warnings, unused)]
use core::time;
use std::{thread, time::Duration};

use std::{fmt::Write, num::ParseIntError};

use windows::{
    self,
    Win32::{
        Devices::Bluetooth::{AF_BTH, BTHPROTO_RFCOMM, SOCKADDR_BTH, SOL_RFCOMM},
        Networking::WinSock::{
            closesocket, setsockopt, WSACleanup, WSAGetLastError, WSAStartup, SOCKADDR, SOCKET,
            SOCKET_ERROR, SOCK_STREAM, SO_RCVTIMEO, SO_SNDTIMEO, TIMEVAL, WSADATA,
        },
    },
};

// Modes
const TRANSPORT_MODE: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x01\x01\x01\x00\x8e";
const NORMAL_MODE: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x02\x01\x01\x00\x8f";
const ANC_INDOOR: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x02\x01\x00\x8e";
const ANC_OUTDOOR: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x01\x01\x00\x8d";
const ANC_TRANSPORT: &[u8; 14] = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x00\x01\x00\x8c";
const known_uuids: [&str; 4] = [
    "00001101-0000-1000-8000-00805F9B34FB",
    "66666666-6666-6666-6666-666666666666",
    "77777777-7777-7777-7777-777777777777",
    "00002902-0000-1000-8000-00805f9b34fb",
];

const BYTE_OFF: i32 = -1;

const OPCODE_BAT: [u8; 7] = [0x08, 0xEE, 0x00, 0x00, 0x00, 0x01, 0x05];

fn main() {
    let res = init_winsock();
    if (res != 0) {
        println!("Error init winsock");
    }

    // Liberty Air 2 Pro working uuid known_uuids[0]
    let status = try_connect_uuid(known_uuids[0]);
    println!("Connect status: {}", status.0);
    match status.1 {
        Some(socket) => {
            unsafe {
                loop {
                    get_battery(socket);
                    // let UKNOWN_OPCODE = b"\x08\xee\x00\x00\x00\x01\x01\n\x00\x02";
                    // let AUKNOWN_OPCODE = b"\x08\xff\xff\xff\xee\x00\x00\x00\x01\x03";
                    // let buf = decode_hex("08ee00000001010a0002").unwrap();
                    // let ret = windows::Win32::Networking::WinSock::send(
                    //     socket,
                    //     &buf,
                    //     windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
                    // );
                    // println!("send status: {}", ret);
                    // if (ret == SOCKET_ERROR) {
                    //     println!("send error code: {:?}", WSAGetLastError());
                    // }
                    // // println!("send bytes: {:?}", b"\x08\xee\x00\x00\x00\x01\x01\n\x00\x02");
                    // let buf: &mut [u8; 300] = &mut [0; 300];
                    // //thread::sleep(Duration::from_millis(500));

                    // windows::Win32::Networking::WinSock::recv(
                    //     socket,
                    //     buf,
                    //     windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
                    // );
                    // println!("Recived from socket {:X?}", buf);
                    // println!("Battery: {:X?}", buf[11]);
                    // println!("Battery: {:X?}", buf[12]);
                    // // println!("Charging: {}", buf[10]);
                    // // println!("ANC Mode: {:?}", &buf[96]);
                    // // println!("Case battery: {:X?}", &buf[96]);
                    // thread::sleep(Duration::from_millis(500));
                    // //let _ = send_rfcomm(socket, ANC_INDOOR);
                }
                closesocket(socket);
            }
        }
        None => (),
    }

    unsafe {
        WSACleanup();
    }
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

fn get_battery(sock: SOCKET) {
    let buf = decode_hex("08ee00000001010a0002").unwrap();
    unsafe {
        let ret = windows::Win32::Networking::WinSock::send(
            sock,
            &buf,
            windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
        );
        if (ret == SOCKET_ERROR) {
            println!("send error code: {:?}", WSAGetLastError());
            return;
        }

        let buf: &mut [u8; 300] = &mut [0; 300];
        thread::sleep(Duration::from_millis(100));

        windows::Win32::Networking::WinSock::recv(
            sock,
            buf,
            windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
        );
        let LEFT_CHARGING: bool = buf[13].eq(&1);
        let RIGHT_CHARGING: bool = buf[14].eq(&1);
        let LEFT_BATTERY = buf[11] & 0xFF;
        let RIGHT_BATTERY = buf[12] & 0xFF;
        //println!("Left  charging: {}", LEFT_CHARGING);
        //println!("Right charging: {}", RIGHT_CHARGING);
        println!("Left  battery: {}", LEFT_BATTERY);
        println!("Right battery: {}", RIGHT_BATTERY);
        //println!("Recived from socket {:X?}", &buf[13]);
    }
}

fn get_info(sock: SOCKET) {
    // Get Info Command ( SN, FW Version )
    let buf = decode_hex("08ee00000001050a0006").unwrap();
    unsafe {
        let ret = windows::Win32::Networking::WinSock::send(
            sock,
            &buf,
            windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
        );
        if (ret == SOCKET_ERROR) {
            println!("send error code: {:?}", WSAGetLastError());
            return;
        }

        let buf: &mut [u8; 50] = &mut [0; 50];
        thread::sleep(Duration::from_millis(100));

        windows::Win32::Networking::WinSock::recv(
            sock,
            buf,
            windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
        );
        let SN = std::str::from_utf8(&buf[19..35]).unwrap();
        let RIGHT_FW = std::str::from_utf8(&buf[14..19]).unwrap();
        let LEFT_FW = std::str::from_utf8(&buf[9..14]).unwrap();
        println!("Recived from socket {:?}",  &buf);
    }
}

fn send_rfcomm(socket: SOCKET, packet: &[u8]) -> i32 {
    let res: i32;
    unsafe {
        res = windows::Win32::Networking::WinSock::send(
            socket,
            packet,
            windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0),
        );
    }
    return res;
}

fn try_connect_uuid(uuid: &str) -> (i32, Option<SOCKET>) {
    unsafe {
        let sock = create_bt_sock();

        if sock == windows::Win32::Networking::WinSock::INVALID_SOCKET {
            println!("Error create sock");
            WSACleanup();
            return (-1, None);
        } else {
            let mut sa: SOCKADDR_BTH = SOCKADDR_BTH {
                addressFamily: AF_BTH,
                btAddr: 0xAC122F6AD207, // set your bt mac
                serviceClassId: windows::core::GUID::from(uuid),
                port: 0,
            };

            let status = windows::Win32::Networking::WinSock::connect(
                sock,
                &sa as *const SOCKADDR_BTH as *const SOCKADDR,
                std::mem::size_of::<SOCKADDR_BTH>() as i32,
            );
            if (status == SOCKET_ERROR) {
                let err = WSAGetLastError();
                println!("Error connect socket: {:?}", err);
                closesocket(sock);
            }

            return (status, Some(sock));
        }
    }
}

fn create_bt_sock() -> SOCKET {
    unsafe {
        let mut sock = windows::Win32::Networking::WinSock::INVALID_SOCKET;
        sock = windows::Win32::Networking::WinSock::socket(
            AF_BTH.into(),
            SOCK_STREAM.into(),
            BTHPROTO_RFCOMM.try_into().unwrap(),
        );
        return sock;
    }
}

fn init_winsock() -> i32 {
    unsafe {
        let wsaData = Box::into_raw(Box::new(WSADATA::default()));
        let i_result: i32;
        i_result = WSAStartup(0x0202, wsaData);
        return i_result;
    }
}

// Not using this
// It takes a long time to find working port or it doesnt work at all ¯\_(ツ)_/¯
// and it requires fixing to return the socket if connection is successfull
// Maybe find a way to change timeout???
fn try_connect(port: u32) -> i32 {
    unsafe {
        let sock = create_bt_sock();

        if sock == windows::Win32::Networking::WinSock::INVALID_SOCKET {
            println!("Error create sock");
            WSACleanup();
            return -1;
        } else {
            println!("Socket created...");

            let set_result = setsockopt(
                sock,
                SOL_RFCOMM.try_into().unwrap(),
                SO_SNDTIMEO.try_into().unwrap(),
                Some(&[1, 0]),
            );
            println!("Set timeout: {}", set_result);

            let mut sa: SOCKADDR_BTH = SOCKADDR_BTH {
                addressFamily: AF_BTH,
                btAddr: 0xAC122F6AD207, // set your bt mac
                serviceClassId: std::mem::zeroed(),
                port: port,
            };

            let status = windows::Win32::Networking::WinSock::connect(
                sock,
                &sa as *const SOCKADDR_BTH as *const SOCKADDR,
                std::mem::size_of::<SOCKADDR_BTH>() as i32,
            );
            if (status == SOCKET_ERROR) {
                let err = WSAGetLastError();
                println!("Error connect socket: {:?}", err);
            }
            closesocket(sock);
            return status;
        }
    }
}

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
        let hexString = format!("{:X}", b & 0xFF);
        if (hexString.len() < 2) {
            write!(&mut s, "0").unwrap();
        }
        write!(&mut s, "{}", hexString).unwrap();
    }
    s
}
