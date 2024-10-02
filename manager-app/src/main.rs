#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;
use std::sync::Arc;

use log::trace;
use mpsc::channel;
use tauri::async_runtime::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_log::LogTarget;
use tokio::sync::mpsc;

use soundcore_lib::api::SoundcoreDeviceState;
use soundcore_lib::btaddr::BluetoothAdrr;

use crate::async_bridge::{async_bridge, BridgeCommand, BridgeResponse};

pub(crate) mod async_bridge;
// Remove for now since this uses legacy code
// TODO: Migrate it to the new async system
// mod tray;

struct SoundcoreAppState {
    bridge_tx: Mutex<mpsc::Sender<BridgeCommand>>,
    scan_in_progress: Arc<Mutex<bool>>,
    last_states: Arc<Mutex<HashMap<BluetoothAdrr, SoundcoreDeviceState>>>,
}

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

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
        // .system_tray(tray::get_system_tray())
        // .on_system_tray_event(tray::handle_tray_event)
        .manage(SoundcoreAppState {
            bridge_tx: Mutex::new(input_tx),
            scan_in_progress: Arc::new(Mutex::new(false)),
            last_states: Arc::new(Mutex::new(HashMap::new()))
        })
        .invoke_handler(tauri::generate_handler![
            // tray::set_tray_device_status,
            // tray::set_tray_menu,
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
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let win = app_handle.get_window(label.as_str()).unwrap();
                        win.hide().unwrap();
                        /* Fix show/hide tray item */
                        let item = app_handle.tray_handle().get_item("hide");
                        item.set_title("Show").unwrap();
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
    } else if let BridgeResponse::NewState(new_state) = resp.clone() {
        let state = manager.state::<SoundcoreAppState>();
        let mut device_states = state.last_states.lock().await;
        let last_state = device_states.insert(new_state.addr, new_state.state.clone());
        handle_state_update(last_state, new_state.state, manager.app_handle());
    } else if let BridgeResponse::ConnectionEstablished(conn) = resp.clone() {
        // This is the initial state so we don't want to show anything to the user
        let state = manager.state::<SoundcoreAppState>();
        let mut device_states = state.last_states.lock().await;
        device_states.insert(conn.addr, conn.state.clone());
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

fn handle_state_update<R: tauri::Runtime>(
    last_state: Option<SoundcoreDeviceState>,
    new_state: SoundcoreDeviceState,
    _app_handle: AppHandle<R>,
) {
    if let Some(last_state) = last_state {
        if last_state == new_state {}
        // This is a bit too verbose for now
        // TODO: Add a setting to enable/disable this
        // TODO: Improve the notification message based on the values changed
        // TODO: Don't show this if we are the originator of the change
        // Notification::new("soundcore-manager")
        //     .title("Device state update")
        //     .body(format!(
        //         "Device state update: {:?} -> {:?}",
        //         last_state, new_state
        //     ))
        //     .show()
        //     .expect("Failed to show notification");
    }
}
