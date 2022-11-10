#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Mutex};


use client_types::{DeviceSelection, ANCModes};
#[cfg(target_os = "windows")]
use soundcore_lib::A3951::A3951Device;
use soundcore_lib::types::{BatteryLevel, BatteryCharging, DeviceStatus, DeviceInfo, ANCProfile, EQWave};
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
    let state = state.device.lock().unwrap();
    let device_state = state.as_ref();
    match device_state{
        Some(selected_device) => {
            let mut guard = selected_device.lock().unwrap();
            let selected_device = &mut *guard;
            match selected_device {
                SupportedDevices::A3951(selected_device) => {
                    match selected_device.connect_uuid(&mac_addr, &uuid) {
                        Ok(_) => {
                            Ok(())
                        },
                        Err(_) => {
                            Err("Failed to connect to device".to_string())
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


// Generalizing this requires generalizing soundcore-lib types
#[tauri::command]
fn get_battery_level(state: State<DeviceState>) -> Result<BatteryLevel, String> {
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
fn get_battery_charging(state: State<DeviceState>) -> Result<BatteryCharging, String> {
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

#[tauri::command]
fn get_device_status(state: State<DeviceState>) -> Result<DeviceStatus, String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let device_status = device.get_status();
                    match device_status {
                        Ok(device_status) => {
                            Ok(device_status)
                        },
                        Err(_) => {
                            Err("Failed to get device status".to_string())
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
fn get_device_info(state: State<DeviceState>) -> Result<DeviceInfo, String> {
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    let device_info = device.get_info();
                    match device_info {
                        Ok(device_info) => {
                            Ok(device_info)
                        },
                        Err(_) => {
                            Err("Failed to get device info".to_string())
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
fn get_anc_mode(state: State<DeviceState>) -> Result<ANCModes, String> {
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
                                ANCProfile::TRANSPARENCY_FULLY_TRANSPARENT_MODE => ANCModes::TransparencyFullyTransparentMode,
                                ANCProfile::TRANSPARENCY_VOCAL_MODE => ANCModes::TransparencyVocalMode,
                                custom_val => {
                                    ANCModes::AncCustomValue(custom_val.anc_custom)
                                }
                            };
                            return Ok(anc_mode_selected);
                        },
                        Err(_) => {
                            return Err("Failed to get anc mode".to_string());
                        }
                    }
                }
            }
        },
        None => {
            return Err("Device not initialized".to_string());
        }
    }
}

#[tauri::command]
fn set_anc_mode(state: State<DeviceState>, mode: ANCModes) -> Result<(), String> {
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
                        ANCModes::TransparencyFullyTransparentMode => ANCProfile::TRANSPARENCY_FULLY_TRANSPARENT_MODE,
                        ANCModes::TransparencyVocalMode => ANCProfile::TRANSPARENCY_VOCAL_MODE,
                    };
                    match device.set_anc(mode_to_send) {
                        Ok(_) => {
                            Ok(())
                        },
                        Err(_) => {
                            Err("Failed to set ANC mode".to_string())
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
fn set_eq_wave(state: State<DeviceState>, eq: EQWave) -> Result<(), String>{
    let device_state = state.device.lock().unwrap();
    match &*device_state {
        Some(device) => {
            let device = device.lock().unwrap();
            match &*device {
                SupportedDevices::A3951(device) => {
                    match device.set_eq(eq) {
                        Ok(_) => {
                            Ok(())
                        },
                        Err(_) => {
                            Err("Failed to set EQ wave".to_string())
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
        .invoke_handler(tauri::generate_handler![
            init_device, connect_uuid,
            get_device_status, get_device_info,
            get_battery_level, get_battery_charging, 
            set_anc_mode, get_anc_mode, set_eq_wave
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
