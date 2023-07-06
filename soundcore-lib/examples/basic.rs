use soundcore_lib::{
    api::{DeviceDescriptor, DeviceRegistry},
    device_registry::{create_soundcore_device_registry, SoundcoreDeviceRegistry},
};
use std::{error::Error, sync::Arc};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let reg = create_soundcore_device_registry().await;
    let registry = SoundcoreDeviceRegistry::new(reg);
    let desc = registry.descriptors().await?;
    let desc = desc.get(0).unwrap();
    let dev = registry
        .device(desc.name(), desc.mac_address())
        .await?
        .unwrap();
    // Create thread to listen for state changes
    let device = Arc::new(dev.clone());
    let device_clone = device.clone();
    let rx = tokio::spawn(async move {
        println!("subscribing to state changes");
        let mut state_receiver = device_clone.to_device().unwrap().subscribe_state();
        while let Ok(state) = state_receiver.recv().await {
            println!("new_state: {:?}", state);
        }
    });
    // Create thread to request state update
    let device_clone = device.clone();
    let tx = tokio::spawn(async move {
        loop {
            println!("requesting state update");
            let _ = device_clone.to_device().unwrap().refresh_state().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
    // Await both threads
    let _ = tokio::join!(rx, tx);
    Ok(())
}

fn calc_and_resize_checksum(cmd: &[u8]) -> Vec<u8> {
    let mut res = vec![0; cmd.len() + 1];
    res[..cmd.len()].copy_from_slice(cmd);
    res[cmd.len()] = calculate_checksum_byte(cmd);
    res
}

fn calculate_checksum_byte(cmd: &[u8]) -> u8 {
    if cmd.is_empty() {
        return 0;
    }
    let mut i = 0;
    for byte in cmd {
        i += *byte as i32;
    }
    (i & 0xFF).try_into().unwrap()
}

fn build_arr(cmd: &[u8], optional_data: Option<&[u8]>) -> Vec<u8> {
    let length = cmd.len() + 2;
    let length2 = (if optional_data.is_some() {
        optional_data.unwrap().len()
    } else {
        0
    }) + length
        + 1;

    let mut output_arr = vec![0; length2 - 1];
    output_arr[..cmd.len()].copy_from_slice(cmd);
    let len_bytes = int_to_byte_array(length2 as i32);
    output_arr[cmd.len()] = len_bytes[3];
    output_arr[cmd.len() + 1] = len_bytes[2];
    if let Some(data) = optional_data {
        output_arr[length..].copy_from_slice(data);
    }

    calc_and_resize_checksum(&output_arr)
}

fn int_to_byte_array(i: i32) -> [u8; 4] {
    let mut bytes = [0; 4];
    bytes[0] = (i & 0xFF) as u8;
    bytes[1] = ((i >> 8) & 0xFF) as u8;
    bytes[2] = ((i >> 16) & 0xFF) as u8;
    bytes[3] = ((i >> 24) & 0xFF) as u8;
    bytes
}
