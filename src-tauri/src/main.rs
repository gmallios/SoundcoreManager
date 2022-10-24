#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Mutex, RwLock};


use client_types::{DeviceSelection};
use soundcore_lib::A3951::{A3951BatteryLevel, A3951BatteryCharging};
#[cfg(target_os = "windows")]
use soundcore_lib::A3951::A3951Device;
use tauri::State;

mod client_types;



enum SupportedDevices {
    A3951(A3951Device),
}



#[tauri::command]
fn init_device(state: State<DeviceState>, device: DeviceSelection) -> Result<String, String> {
    let mut device_state = state.device.lock().unwrap();
    match device {
        DeviceSelection::A3951 => {
            let device = A3951Device::new();
            match device {
                Ok(device) => {
                    *device_state = Some(Mutex::new(SupportedDevices::A3951(device)));
                    *state.initialized.lock().unwrap() = true;
                    println!("Device initialized");
                    Ok("A3951 Device initialized".to_string())
                }
                Err(_) => {
                    *device_state = None;
                    *state.initialized.lock().unwrap() = false;
                    Err("Failed to initialize device".to_string())
                },
            }
        },
        DeviceSelection::None => {
            *device_state = None;
            *state.initialized.lock().unwrap() = false;
            Err("Nothing to initialize".to_string())
        }
    }
}

#[tauri::command]
fn connect_uuid(state: State<DeviceState>, mac_addr: String, uuid: String) -> Result<(), String>{
    let mut state = state.device.lock().unwrap();
    let device_state = state.as_ref();
    match device_state{
        Some(selected_device) => {
            let mut guard = selected_device.lock().unwrap();
            let mut selected_device = &mut *guard;
            match selected_device {
                SupportedDevices::A3951(selected_device) => {
                    match selected_device.connect_uuid(&mac_addr, &uuid) {
                        Ok(_) => {
                            println!("Connected to device");
                            Ok(())
                        },
                        Err(_) => {
                            println!("Failed to connect to device");
                            Err("Failed to connect to device".to_string())
                        }
                    }
                }
            }
        },
        None => {
            println!("Device not initialized");
            Err("Device not initialized".to_string())
        }
    }
}


// Generalizing this requires generalizing soundcore-lib types
#[tauri::command]
fn get_battery_level(state: State<DeviceState>) -> Result<A3951BatteryLevel, String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let battery_level = device.get_battery_level();
                    match battery_level {
                        Ok(battery_level) => {
                            Ok(battery_level)
                        },
                        Err(_) => {
                            Err("Failed to get battery info".to_string())
                        }
                    }
                }
            }
        },
        None => {
            Err("Device not initialized".to_string())
        }
    }
}

#[tauri::command]
fn get_battery_charging(state: State<DeviceState>) -> Result<A3951BatteryCharging, String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let battery_charging = device.get_battery_charging();
                    match battery_charging {
                        Ok(battery_charging) => {
                            Ok(battery_charging)
                        },
                        Err(_) => {
                            Err("Failed to get battery charging".to_string())
                        }
                    }
                }
            }
        },
        None => {
            Err("Device not initialized".to_string())
        }
    }
}


struct DeviceState {
    device: Mutex<Option<Mutex<SupportedDevices>>>,
    initialized: Mutex<bool>,
}



fn main() {
    tauri::Builder::default()
        .manage(DeviceState { device: Mutex::new(None), initialized: Mutex::new(false) })
        .invoke_handler(tauri::generate_handler![init_device, connect_uuid, get_battery_level, get_battery_charging])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
