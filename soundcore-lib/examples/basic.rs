use bluetooth_lib::{RFCOMM};
use soundcore_lib::{error::SoundcoreError, devices::{A3951, A3951_RFCOMM_UUID}};

const BT_ADDR: &str = "AC:12:2F:6A:D2:07";

fn main() {
    println!("Fetching device info...");

    // Initialize the RFCOMM communcation channel
    let mut comm = RFCOMM::new().create_rfcomm_socket().unwrap();

    let addr = bluetooth_lib::BluetoothAdrr::from(BT_ADDR);
    // Connect to device UUID service
    comm.connect_uuid(addr, A3951_RFCOMM_UUID).unwrap();

    // Setup send and receive functions for soundcore-lib, we use our own RFCOMM implementation
    // Feel free to use any other implementation
    let send_fn = |data: &[u8]| match comm.send(data) {
        Ok(_) => Ok(()),
        Err(_e) => Err(SoundcoreError::SendError),
    };
    let recv_fn = |num_of_bytes: usize| match comm.recv(num_of_bytes) {
        Ok(data) => Ok(data),
        Err(_e) => Err(SoundcoreError::RecvError),
    };

    let device = A3951::new(&send_fn, &recv_fn).unwrap();

    match device.get_info() {
        Ok(info) => println!(
            "SN: {}, Right FW: {}, Left FW: {}",
            info.sn, info.right_fw, info.left_fw
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    comm.close();
}
