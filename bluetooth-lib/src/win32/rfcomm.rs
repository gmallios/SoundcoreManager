use async_trait::async_trait;
use log::trace;
use windows::{
    self,
    Win32::{
        Devices::Bluetooth::{AF_BTH, BTHPROTO_RFCOMM, SOCKADDR_BTH},
        Networking::WinSock::{
            closesocket, connect, recv, send, socket, WSACleanup,
            INVALID_SOCKET, SEND_RECV_FLAGS, SOCKADDR, SOCKET, SOCKET_ERROR, SOCK_STREAM,
        },
    },
};

use crate::{types::{BluetoothAdrr, RFCOMMClient}, error::BthError, win32::util::init_winsock};

#[derive(Debug)]
pub struct RFCOMM {
    fd: SOCKET,
    connected: bool,
}

#[async_trait]
impl RFCOMMClient for RFCOMM {
    fn new() -> Result<RFCOMM, BthError> {
        init_winsock();
        let fd: SOCKET;
        unsafe {
            fd = socket(
                AF_BTH.into(),
                SOCK_STREAM.into(),
                BTHPROTO_RFCOMM.try_into().map_err(|_| BthError::FdInitError)?,
            );
        }
        if fd == INVALID_SOCKET {
            return Err(BthError::FdInitError);
        }
        trace!("RFCOMM win32 socket created");
        Ok(RFCOMM {
            fd,
            connected: false,
        })
    }

    // fn create_rfcomm_socket(&self) -> Result<RFCOMM, BthError> {
    //     let mut fd: SOCKET;
    //     unsafe {
    //         fd = socket(
    //             AF_BTH.into(),
    //             SOCK_STREAM.into(),
    //             BTHPROTO_RFCOMM.try_into().unwrap(),
    //         );
    //     }
    //     if fd == INVALID_SOCKET {
    //         fd = SOCKET::default();
    //         return Err(BthError::FdInitError);
    //     }
    //     Ok(RFCOMM {
    //         fd: fd,
    //         connected: false,
    //     })
    // }

    async fn connect_uuid(&mut self, bt_addr: BluetoothAdrr, uuid: &str) -> Result<(), BthError> {
        if self.fd == INVALID_SOCKET {
            return Err(BthError::InvalidSocketError);
        }

        let s_addr: SOCKADDR_BTH = SOCKADDR_BTH {
            addressFamily: AF_BTH,
            btAddr: u64::from_str_radix(&bt_addr.to_string().replace(':', ""), 16)?,
            serviceClassId: windows::core::GUID::from(uuid),
            port: 0, // When using uuid, port is 0
        };

        unsafe {
            let ret = connect(
                self.fd,
                &s_addr as *const SOCKADDR_BTH as *const SOCKADDR,
                std::mem::size_of::<SOCKADDR_BTH>() as i32,
            );
            if ret == SOCKET_ERROR {
                closesocket(self.fd);
                return Err(BthError::TryConnectError);
            }
        }
        self.connected = true;
        trace!("Connected to uuid: {}", uuid);
        return Ok(());
    }

    async fn connect_port(&mut self, bt_addr: BluetoothAdrr, port: u32) -> Result<(), BthError>{
        if self.fd == INVALID_SOCKET {
            return Err(BthError::InvalidSocketError);
        }

        
        unsafe {
            let s_addr: SOCKADDR_BTH = SOCKADDR_BTH {
                addressFamily: AF_BTH,
                btAddr: u64::from_str_radix(&bt_addr.to_string().replace(':', ""), 16)?,
                serviceClassId: std::mem::zeroed(),
                port,
            };

            let ret = connect(
                self.fd,
                &s_addr as *const SOCKADDR_BTH as *const SOCKADDR,
                std::mem::size_of::<SOCKADDR_BTH>() as i32,
            );
            if ret == SOCKET_ERROR {
                closesocket(self.fd);
                return Err(BthError::TryConnectError);
            }
        }
        trace!("Connected to port: {}", port);
        self.connected = true;
        return Ok(());
    }

    async fn send(&self, data: &[u8]) -> Result<(), BthError> {
        if !self.connected || self.fd == INVALID_SOCKET {
            return Err(BthError::InvalidSocketError);
        }
        unsafe {
            if send(self.fd, data, windows::Win32::Networking::WinSock::SEND_RECV_FLAGS(0)) == SOCKET_ERROR {
                return Err(BthError::SendError);
            }
        }
        trace!("Sent data: {:?}", data);
        Ok(())
    }

    async fn recv(&self, num_of_bytes: usize) -> Result<Vec<u8>, BthError> {
        if !self.connected || self.fd == INVALID_SOCKET {
            return Err(BthError::InvalidSocketError);
        }
        let mut data: Vec<u8> = vec![0; num_of_bytes];
        unsafe {
            /* Safety Warning: Could result in buffer overflow!! */
            /* I think excess bytes are discarded... */
            if recv(self.fd, &mut data, SEND_RECV_FLAGS(0)) == SOCKET_ERROR {
                return Err(BthError::RecvError);
            }
        }
        trace!("Received data: {:?}", data);
        Ok(data)
    }

    fn close(&self) {
        unsafe {
            WSACleanup();
            closesocket(self.fd);
        }
    }
}

impl Drop for RFCOMM {
    fn drop(&mut self) {
        unsafe {
            closesocket(self.fd);
            // WSACleanup();
        }
    }
}
