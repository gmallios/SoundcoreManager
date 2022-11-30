#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex, RwLock};

use bluetooth_lib::{BluetoothAdrr, RFCOMM};
use client_types::{ANCModes, BthScanResult, DeviceSelection};
use serde::Serialize;
use soundcore_lib::types::{
    ANCProfile, BatteryCharging, BatteryLevel, DeviceInfo, DeviceStatus, EQWave,
};
#[cfg(target_os = "windows")]
use soundcore_lib::A3951::A3951Device;
use tauri::{CustomMenuItem, State, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

mod client_types;
mod tray;

enum SupportedDevices<'a> {
    A3951(A3951Device<'a>),
}

fn send_rfcomm(data: &[u8]) -> Result<(), soundcore_lib::error::SoundcoreError> {
    let comm = RFCOMM_STATE.lock().unwrap();
    match comm.send(data) {
        Ok(_) => Ok(()),
        Err(_e) => Err(soundcore_lib::error::SoundcoreError::SendError),
    }
}
fn recv_rfcomm(num_of_bytes: usize) -> Result<Vec<u8>, soundcore_lib::error::SoundcoreError> {
    let comm = RFCOMM_STATE.lock().unwrap();
    match comm.recv(num_of_bytes) {
        Ok(data) => Ok(data),
        Err(_e) => Err(soundcore_lib::error::SoundcoreError::RecvError),
    }
}

#[tauri::command]
fn init_device(state: State<DeviceState>, device: DeviceSelection) -> Result<String, String> {
    let mut device_state = state.device.lock().unwrap();
    let mut comm = RFCOMM_STATE.lock().unwrap();
    match comm.create_rfcomm_socket() {
        Ok(c) => *comm = c,
        Err(e) => return Err(e.to_string()),
    }
    match device {
        DeviceSelection::A3951 => {
            let device = A3951Device::new(&send_rfcomm, &recv_rfcomm);
            match device {
                Ok(device) => {
                    *device_state = Some(Mutex::new(SupportedDevices::A3951(device)));
                    *state.initialized.lock().unwrap() = true;
                    Ok("A3951 Device initialized".to_string())
                }
                Err(_) => {
                    *device_state = None;
                    *state.initialized.lock().unwrap() = false;
                    Err("Failed to initialize device".to_string())
                }
            }
        }
        DeviceSelection::None => {
            *device_state = None;
            *state.initialized.lock().unwrap() = false;
            Err("Nothing to initialize".to_string())
        }
    }
}

#[tauri::command]
fn connect_uuid(mac_addr: String, uuid: String) -> Result<(), String> {
    let mut comm = RFCOMM_STATE.lock().unwrap();
    match comm.connect_uuid(BluetoothAdrr::from(mac_addr), &uuid) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

// Generalizing this requires generalizing soundcore-lib types
#[tauri::command]
async fn get_battery_level(state: State<'_, DeviceState<'_>>) -> Result<BatteryLevel, String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let battery_level = device.get_battery_level();
                    match battery_level {
                        Ok(battery_level) => Ok(battery_level),
                        Err(_) => Err("Failed to get battery info".to_string()),
                    }
                }
            }
        }
        None => Err("Device not initialized".to_string()),
    }
}

#[tauri::command]
async fn get_battery_charging(
    state: State<'_, DeviceState<'_>>,
) -> Result<BatteryCharging, String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let battery_charging = device.get_battery_charging();
                    match battery_charging {
                        Ok(battery_charging) => Ok(battery_charging),
                        Err(_) => Err("Failed to get battery charging".to_string()),
                    }
                }
            }
        }
        None => Err("Device not initialized".to_string()),
    }
}

#[tauri::command]
async fn get_device_status(state: State<'_, DeviceState<'_>>) -> Result<DeviceStatus, String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let device_status = device.get_status();
                    match device_status {
                        Ok(device_status) => Ok(device_status),
                        Err(_) => Err("Failed to get device status".to_string()),
                    }
                }
            }
        }
        None => Err("Device not initialized".to_string()),
    }
}

#[tauri::command]
async fn get_device_info(state: State<'_, DeviceState<'_>>) -> Result<DeviceInfo, String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let device_info = device.get_info();
                    match device_info {
                        Ok(device_info) => Ok(device_info),
                        Err(_) => Err("Failed to get device info".to_string()),
                    }
                }
            }
        }
        None => Err("Device not initialized".to_string()),
    }
}

#[tauri::command]
async fn get_anc_mode(state: State<'_, DeviceState<'_>>) -> Result<ANCModes, String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let anc_mode = device.get_anc();
                    match anc_mode {
                        Ok(anc_mode) => {
                            let anc_mode_selected: ANCModes = match anc_mode {
                                ANCProfile::ANC_INDOOR_MODE => ANCModes::AncIndoorMode,
                                ANCProfile::ANC_OUTDOOR_MODE => ANCModes::AncOutdoorMode,
                                ANCProfile::ANC_TRANSPORT_MODE => ANCModes::AncTransportMode,
                                ANCProfile::NORMAL_MODE => ANCModes::NormalMode,
                                ANCProfile::TRANSPARENCY_FULLY_TRANSPARENT_MODE => {
                                    ANCModes::TransparencyFullyTransparentMode
                                }
                                ANCProfile::TRANSPARENCY_VOCAL_MODE => {
                                    ANCModes::TransparencyVocalMode
                                }
                                custom_val => ANCModes::AncCustomValue(custom_val.anc_custom),
                            };
                            return Ok(anc_mode_selected);
                        }
                        Err(_) => {
                            return Err("Failed to get anc mode".to_string());
                        }
                    }
                }
            }
        }
        None => {
            return Err("Device not initialized".to_string());
        }
    }
}

#[tauri::command]
async fn set_anc_mode(state: State<'_, DeviceState<'_>>, mode: ANCModes) -> Result<(), String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let mode_to_send = match mode {
                        ANCModes::NormalMode => ANCProfile::NORMAL_MODE,
                        ANCModes::AncTransportMode => ANCProfile::ANC_TRANSPORT_MODE,
                        ANCModes::AncOutdoorMode => ANCProfile::ANC_OUTDOOR_MODE,
                        ANCModes::AncIndoorMode => ANCProfile::ANC_INDOOR_MODE,
                        ANCModes::AncCustomValue(value) => ANCProfile::anc_custom_value(value),
                        ANCModes::TransparencyFullyTransparentMode => {
                            ANCProfile::TRANSPARENCY_FULLY_TRANSPARENT_MODE
                        }
                        ANCModes::TransparencyVocalMode => ANCProfile::TRANSPARENCY_VOCAL_MODE,
                    };
                    match device.set_anc(mode_to_send) {
                        Ok(_) => Ok(()),
                        Err(_) => Err("Failed to set ANC mode".to_string()),
                    }
                }
            }
        }
        None => Err("Device not initialized".to_string()),
    }
}

#[tauri::command]
fn set_eq_wave(state: State<DeviceState>, eq: EQWave) -> Result<(), String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => match device.set_eq(eq) {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Failed to set EQ wave".to_string()),
                },
            }
        }
        None => Err("Device not initialized".to_string()),
    }
}

#[tauri::command]
fn scan_for_devices() -> Vec<BthScanResult> {
    let res = bluetooth_lib::BthScanner::new().scan();
    let mut scan_res: Vec<BthScanResult> = vec![];
    res.into_iter().for_each(|x| {
        scan_res.push(BthScanResult::from(x));
    });
    scan_res
}

struct DeviceState<'a> {
    device: Arc<Mutex<Option<Mutex<SupportedDevices<'a>>>>>,
    initialized: Mutex<bool>,
}

lazy_static! {
    static ref RFCOMM_STATE: Arc<Mutex<RFCOMM>> = Arc::new(Mutex::new(RFCOMM::new()));
}

fn main() {
    tauri::Builder::default()
        .system_tray(tray::get_system_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .manage(DeviceState {
            device: Arc::new(Mutex::new(None)),
            initialized: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            scan_for_devices,
            init_device,
            connect_uuid,
            get_device_status,
            get_device_info,
            get_battery_level,
            get_battery_charging,
            set_anc_mode,
            get_anc_mode,
            set_eq_wave,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
