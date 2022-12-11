use windows::Win32::Networking::WinSock::{WSAStartup, WSADATA};

pub(crate) fn init_winsock() -> i32 {
    let wsa_data = Box::into_raw(Box::new(WSADATA::default()));
    let i_result: i32;
    unsafe {
        i_result = WSAStartup(0x0202, wsa_data);
    }
    i_result
}