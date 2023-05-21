#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use bluetooth_lib::platform::BthScanner;
use bluetooth_lib::Scanner;
use frontend_types::BthScanResult;

use soundcore_lib::base::SoundcoreDevice;
use soundcore_lib::types::{SupportedModels, SOUNDCORE_NAME_MODEL_MAP};
use tauri_plugin_log::LogTarget;

use std::sync::Arc;
use tauri::async_runtime::{Mutex, RwLock};
use tauri::Manager;

mod device;
pub(crate) mod frontend_types;
mod tray;
pub(crate) mod utils;

#[cfg(target_os = "macos")]
mod server;


#[tauri::command]
async fn scan_for_devices() -> Vec<BthScanResult> {
    let res = BthScanner::new().scan().await;
    let mut scan_res: Vec<BthScanResult> = vec![];
    res.into_iter().for_each(|btdevice| {
        /* Sometimes on macOS, the app needs to open the connection in order to open the rfcomm socket */
        #[cfg(target_os = "macos")]
        if !SOUNDCORE_NAME_MODEL_MAP.contains_key(&btdevice.name) {
            return;
        }
        /* In other platform that is not the case, so we should filter not connected devices */
        #[cfg(not(target_os = "macos"))]
        if !btdevice.connected || !SOUNDCORE_NAME_MODEL_MAP.contains_key(&btdevice.name) {
            return;
        }
        scan_res.push(BthScanResult::from(btdevice));
    });
    scan_res
}

struct SoundcoreAppState {
    device: Arc<Mutex<Option<Box<dyn SoundcoreDevice>>>>,
    model: Arc<RwLock<Option<SupportedModels>>>,
}

fn main() {
    #[cfg(target_os = "macos")]
    server::launch_server();

    tauri::Builder::default()
        .system_tray(tray::get_system_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .manage(SoundcoreAppState {
            device: Arc::new(Mutex::new(None)),
            model: Arc::new(RwLock::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            tray::set_tray_device_status,
            tray::set_tray_menu,
            device::connect,
            device::close,
            device::get_model,
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
        .plugin(
            tauri_plugin_log::Builder::default().targets([
                LogTarget::LogDir,
                LogTarget::Stdout,
                LogTarget::Webview
            ])
            .level_for("h2", log::LevelFilter::Off)
            .level_for("hyper", log::LevelFilter::Off)
            .level_for("tower", log::LevelFilter::Off)
            .level_for("tracing", log::LevelFilter::Off)
            .level_for("tokio_util", log::LevelFilter::Off)
            .level_for("want", log::LevelFilter::Off)
            .level_for("tonic", log::LevelFilter::Off)
            .level_for("mio", log::LevelFilter::Off)
            .level_for("tao", log::LevelFilter::Off)
            .level_for("soundcoremanager::server", log::LevelFilter::Off)
            .build()
        )
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                if cfg!(target_os = "macos"){
                    api.prevent_exit();
                }
            },
            /* Prevent window from closing - Primarily for macOS since we rely heavily on the window */
            tauri::RunEvent::WindowEvent { label, event, .. } => {
                if cfg!(target_os = "macos"){
                    match event {
                        tauri::WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            let win = app_handle.get_window(label.as_str()).unwrap();
                            win.hide().unwrap();
                            /* Fix show/hide tray item */
                            let item = app_handle.tray_handle().get_item("hide");
                            item.set_title("Show").unwrap();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });
}
