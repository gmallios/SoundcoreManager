#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use log::{info, trace};
use mpsc::channel;
use tauri::async_runtime::{Mutex, RwLock};
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use tokio::sync::mpsc;

use bluetooth_lib::platform::BthScanner;
use bluetooth_lib::Scanner;
use frontend_types::BthScanResult;
use soundcore_lib::base::SoundcoreDevice;
use soundcore_lib::types::{SupportedModels, SOUNDCORE_NAME_MODEL_MAP};

use crate::async_bridge::{async_bridge, BridgeCommand, BridgeResponse};

pub(crate) mod async_bridge;
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
        if !btdevice.connected
            || !SOUNDCORE_NAME_MODEL_MAP
                .keys()
                .any(|name| btdevice.name.contains(name))
        {
            return;
        }
        scan_res.push(BthScanResult::from(btdevice));
    });
    scan_res
}

struct SoundcoreAppState {
    device: Arc<Mutex<Option<Box<dyn SoundcoreDevice>>>>,
    model: Arc<RwLock<Option<SupportedModels>>>,
    bridge_tx: Mutex<mpsc::Sender<BridgeCommand>>,
    scan_in_progress: Arc<Mutex<bool>>,
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    #[cfg(target_os = "macos")]
    server::launch_server();

    // builder()
    //     .filter(None, log::LevelFilter::Debug)
    //     .filter_module("h2", log::LevelFilter::Off)
    //     .filter_module("hyper", log::LevelFilter::Off)
    //     .filter_module("tower", log::LevelFilter::Off)
    //     .init();
    let (input_tx, input_rx) = channel(255);
    let (output_tx, mut output_rx) = channel(255);

    tauri::Builder::default()
        .setup(|app| {
            tokio::spawn(async_bridge(input_rx, output_tx));

            let app_handle = app.handle();
            tokio::spawn(async move {
                loop {
                   if let Some(resp) = output_rx.recv().await {
                       handle_bridge_output(resp, &app_handle).await;
                   }
                }
            });
            Ok(())
        })
        .system_tray(tray::get_system_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .manage(SoundcoreAppState {
            device: Arc::new(Mutex::new(None)),
            model: Arc::new(RwLock::new(None)),
            bridge_tx: Mutex::new(input_tx),
            scan_in_progress: Arc::new(Mutex::new(false)),
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
            send_bridge_command
        ])
        .plugin(tauri_plugin_log::Builder::default().targets([
            LogTarget::LogDir,
            LogTarget::Stdout,
            LogTarget::Webview
        ]).build())
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

async fn handle_bridge_output<R: tauri::Runtime>(resp: BridgeResponse, manager: &impl Manager<R>) {
    trace!("Received response from bridge, emitting event...");
    trace!("Response: {:?}", resp);
    if let BridgeResponse::ScanResult(_) = resp {
        let state = manager.state::<SoundcoreAppState>();
        let mut scan_in_progress = state.scan_in_progress.lock().await;
        *scan_in_progress = false;
    }
    manager.emit_all("async-bridge-event", resp).unwrap();
}

#[tauri::command]
async fn send_bridge_command(
    app_state: tauri::State<'_, SoundcoreAppState>,
    payload: BridgeCommand,
) -> Result<(), String> {
    if let BridgeCommand::Scan = payload {
        let mut scan_in_progress = app_state.scan_in_progress.lock().await;
        if *scan_in_progress {
            return Err("Scan already in progress".to_string());
        }
        *scan_in_progress = true;
    }

    let tx = app_state.bridge_tx.lock().await;
    tx.send(payload).await.map_err(|e| e.to_string())
}
