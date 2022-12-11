#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use std::sync::{Arc};
use bluetooth_lib::{Scanner};
use bluetooth_lib::platform::BthScanner;
use frontend_types::{BthScanResult};
use soundcore_lib::{base::SoundcoreDevice};
use tauri::{async_runtime::Mutex};

pub(crate) mod frontend_types;
pub(crate) mod utils;
mod tray;
mod device;



// #[tauri::command]
// fn close_all(state: State<DeviceState>) -> Result<(), ()> {
//     let mut device_state = state.device.lock().map_err(|_| ())?;
//     *device_state = None;
//     let rfcomm = RFCOMM_STATE.lock().map_err(|_| ())?;
//     rfcomm.close();
//     Ok(())
// }

#[tauri::command]
async fn scan_for_devices() -> Vec<BthScanResult> {
    let res = BthScanner::new().scan().await;
    let mut scan_res: Vec<BthScanResult> = vec![];
    res.into_iter().for_each(|x| {
        scan_res.push(BthScanResult::from(x));
    });
    scan_res
}



struct AppState {
    device: Arc<Mutex<Option<Box<dyn SoundcoreDevice>>>>,
}


fn main() {
    tauri::Builder::default()
        .system_tray(tray::get_system_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .manage(AppState {
            device: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            tray::set_tray_device_status,
            tray::set_tray_menu,
            device::connect,
            device::close,
            device::is_connected,
            device::get_info,
            device::get_status,
            device::get_battery_level,
            device::get_battery_charging,
            device::set_anc,
            device::get_anc,
            device::set_eq,
            device::get_eq,
            scan_for_devices,
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
