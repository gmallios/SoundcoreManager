use std::num::ParseIntError;

use crate::{utils::i8_to_u8vec, build_command_array_with_options_toggle_enabled};


use windows::{
    self,
    core::HSTRING,
    Win32::{
        Devices::Bluetooth::{AF_BTH, BTHPROTO_RFCOMM, SOCKADDR_BTH, SOL_RFCOMM},
        Networking::WinSock::{
            closesocket, setsockopt, WSACleanup, WSAGetLastError, WSAStartup, SOCKADDR, SOCKET,
            SOCKET_ERROR, SOCK_STREAM, SO_RCVTIMEO, SO_SNDTIMEO, TIMEVAL, WSADATA, WSA_ERROR, send, SEND_RECV_FLAGS, recv,
        },
    },
};


//TODO: More error types and rewrite winerrors 

#[derive(Debug)]
pub enum A3951Error {
    Unknown
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
            //A3951Error::Errno(_, ref message) => message.as_str(),
        }
    }
}

impl From<std::io::Error> for A3951Error {
    fn from(error: std::io::Error) -> Self {
        A3951Error::Unknown
    }
}

impl From<std::num::ParseIntError> for A3951Error {
    fn from(error: std::num::ParseIntError) -> Self {
        A3951Error::Unknown
    }
}


impl From<windows::core::Error> for A3951Error {
    fn from(error: windows::core::Error) -> Self {
        A3951Error::Unknown
    }
}




pub(crate) struct A3951Device {
    sock: SOCKET,
    pub state: i32
}

static CMD_DEVICE_INFO: [i8; 7] = [8,-18,0,0,0,1,1];
pub const WINAPI_FLAG: SEND_RECV_FLAGS = windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0);

impl A3951Device {
   

    pub fn new() -> Result<A3951Device, A3951Error> {
        unsafe {
            if init_winsock() != 0 {
                return Err(A3951Error::from(windows::core::Error::new(
                    windows::core::HRESULT(0),
                    HSTRING::from("winsock init error"),
                )));
            }
        }
        Ok(A3951Device {
            sock: create_bt_sock()?,
            state: 0
        })
    }

    pub fn connect_uuid(
        &mut self,
        mac_addr: &str,
        uuid: &str,
    ) -> Result<(), A3951Error> {
        self.sock = try_connect_uuid(self.sock, mac_addr, uuid)?;
        self.state = 1;
        Ok(())
    }

    pub fn example_get_info(&self) {
        let cmd = &Self::create_cmd(CMD_DEVICE_INFO);
        println!("CMD: {:?}", cmd);
        self.send(cmd);
        std::thread::sleep(std::time::Duration::from_millis(100));
        let resp = self.recv(100).unwrap();
        println!("resp: {:?}", resp);
    }

    pub fn create_cmd(inp: [i8; 7]) -> Vec<u8>{
        return build_command_array_with_options_toggle_enabled(&i8_to_u8vec(&inp), None);
    }


    fn send(&self, data: &[u8]) -> Result<(), A3951Error> {
        let mut bytes_sent = 0;
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

#[derive(Default)]
pub(crate) struct A3951DeviceInfo {
    name: String,
    mac_address: String,
    status: A3951_STATUS,
    anc: A3951_ANC,
}

#[derive(Default)]
pub(crate) struct A3951_STATUS {
    HOST_DEVICE: u8,
    TWS_STATUS: bool,
    LEFT_BATTERY: u8,
    RIGHT_BATTERY: u8,
    LEFT_CHARGING: bool,
    RIGHT_CHARGING: bool,
}
#[derive(Default)]
pub(crate) struct A3951_ANC {
    Option: u8,
    ANCOption: u8,
    TransOption: u8,
    ANCCustom: u8,
}

fn try_connect_uuid(
    sock: SOCKET,
    addr: &str,
    uuid: &str,
) -> Result<SOCKET, A3951Error> {
    unsafe {
        let mut saddr: SOCKADDR_BTH = SOCKADDR_BTH {
            addressFamily: AF_BTH,
            btAddr: crate::utils::mac_str_to_u64(addr)?, // set your bt mac 0xAC122F6AD207
            serviceClassId: windows::core::GUID::from(uuid),
            port: 0,
        };

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

        return Ok(sock);
    }
}

fn create_bt_sock() -> Result<SOCKET, A3951Error> {
    unsafe {
        let mut sock = windows::Win32::Networking::WinSock::INVALID_SOCKET;
        sock = windows::Win32::Networking::WinSock::socket(
            AF_BTH.into(),
            SOCK_STREAM.into(),
            BTHPROTO_RFCOMM.try_into().unwrap(),
        );
        if (sock == windows::Win32::Networking::WinSock::INVALID_SOCKET) {
            return Err(A3951Error::from(windows::core::Error::new(
                windows::core::HRESULT(0),
                HSTRING::from("failed creating socket"),
            )));
        }
        return Ok(sock);
    }
}

pub(crate) unsafe fn init_winsock() -> i32 {
    let wsaData = Box::into_raw(Box::new(WSADATA::default()));
    let i_result: i32;
    i_result = WSAStartup(0x0202, wsaData);
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
