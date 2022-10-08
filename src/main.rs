use core::time;
use std::{thread, time::Duration};

use windows::{
    self,
    Win32::{
        Devices::Bluetooth::{AF_BTH, BTHPROTO_RFCOMM, SOCKADDR_BTH, SOL_RFCOMM},
        Networking::WinSock::{
            closesocket, WSACleanup, WSAGetLastError, WSAStartup, SOCKADDR, SOCKET, SOCKET_ERROR,
            SOCK_STREAM, WSADATA, setsockopt, TIMEVAL, SO_RCVTIMEO, SO_SNDTIMEO,
        },
    },
};

fn main() {
    let res = init_winsock();
    if (res != 0) {
        println!("Error init winsock");
    }
    let known_uuids = ["00001101-0000-1000-8000-00805F9B34FB", "66666666-6666-6666-6666-666666666666", "77777777-7777-7777-7777-777777777777", "00002902-0000-1000-8000-00805f9b34fb"];
    let status = try_connect_uuid(known_uuids[0]);
    println!("Connect status: {}", status.0);
    match status.1 {
        Some(socket) => {
         unsafe{
            let ret = windows::Win32::Networking::WinSock::send(socket, b"\x08\xee\x00\x00\x00\x01\x01\n\x00\x02", windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0));
            println!("send status: {}", ret);
            if(ret == SOCKET_ERROR){
                  println!("send error code: {:?}", WSAGetLastError());
            }
            println!("send bytes: {:?}", b"\x08\xee\x00\x00\x00\x01\x01\n\x00\x02");
            let buf: &mut [u8; 1024] = &mut [0; 1024];
            thread::sleep(Duration::from_millis(100));
            windows::Win32::Networking::WinSock::recv(socket, buf, windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0));
            println!("Recived from socket {:?}", buf);
            //sleep 1 sec
            thread::sleep(Duration::from_millis(500));
            let TRANSPORT_MODE = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x01\x01\x01\x00\x8e";
            let NORMAL_MODE = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x02\x01\x01\x00\x8f";
            let ANC_INDOOR = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x02\x01\x00\x8e";
            let ANC_OUTDOOR = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x01\x01\x00\x8d";
            let ANC_TRANSPORT = b"\x08\xee\x00\x00\x00\x06\x81\x0e\x00\x00\x00\x01\x00\x8c";
            let _ = windows::Win32::Networking::WinSock::send(socket, ANC_INDOOR, windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0));
         }
        },
        None => (),
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

fn try_connect(port: u32) -> i32 {
    unsafe {
        let sock = create_bt_sock();

        if sock == windows::Win32::Networking::WinSock::INVALID_SOCKET {
            println!("Error create sock");
            WSACleanup();
            return -1;
        } else {
            println!("Socket created...");

 
            let set_result = setsockopt(sock, SOL_RFCOMM.try_into().unwrap(), SO_SNDTIMEO.try_into().unwrap(), Some(&[1,0]));
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




fn try_connect_uuid(uuid: &str) -> (i32, Option<SOCKET>) {
   unsafe {
       let sock = create_bt_sock();

       if sock == windows::Win32::Networking::WinSock::INVALID_SOCKET {
           println!("Error create sock");
           WSACleanup();
           return (-1, None);
       } else {
           println!("Socket created...");


           let set_result = setsockopt(sock, SOL_RFCOMM.try_into().unwrap(), SO_SNDTIMEO.try_into().unwrap(), Some(&[1,0]));
           println!("Set timeout: {}", set_result);

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
