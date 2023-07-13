#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use frontend_types::BthScanResult;

use soundcore_process::{SoundcoreResponseMessage, SoundcoreRequestMessage};
use tauri::Manager;
use tauri_plugin_log::LogTarget;
use tokio::sync::mpsc;

// pub(crate) mod frontend_types;
// mod tray;
// pub(crate) mod utils;
mod soundcore_process;

struct SoundcoreAppState {
    soundcore_tx: tokio::sync::Mutex<mpsc::Sender<soundcore_process::SoundcoreRequestMessage>>
}

fn main() {
    #[cfg(target_os = "macos")]
    server::launch_server();

    // builder()
    //     .filter(None, log::LevelFilter::Debug)
    //     .filter_module("h2", log::LevelFilter::Off)
    //     .filter_module("hyper", log::LevelFilter::Off)
    //     .filter_module("tower", log::LevelFilter::Off)
    //     .init();

    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    tauri::Builder::default()
        // .system_tray(tray::get_system_tray())
        // .on_system_tray_event(tray::handle_tray_event)
        .manage(SoundcoreAppState {
            soundcore_tx: tokio::sync::Mutex::new(async_proc_input_tx) 
        })
        .invoke_handler(tauri::generate_handler![
            soundcore_command
        ])
        .setup(|app| {
            // Spawn the soundcore async process
            tauri::async_runtime::spawn(async move {
                soundcore_process::soundcore_process(async_proc_input_rx, async_proc_output_tx).await;
            });       

            //TODO: Maybe spawn a thread to poll the device periodically for battery status/level

            // Spawn the soundcore output handler
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(event) = async_proc_output_rx.recv().await {
                        handle_soundcore_event(event, &app_handle);
                    }
                }
            });
            Ok(())
        })
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

fn handle_soundcore_event<R: tauri::Runtime>(message: SoundcoreResponseMessage, manager: &impl Manager<R>){
    // TODO: Update the tray menu
    println!("Soundcore event: {:?}", message);
    manager.emit_all(SoundcoreUIEvent::SoundcoreEvent.to_str(), message).unwrap();
}

pub enum SoundcoreUIEvent {
    TrayUpdate,
    SoundcoreEvent
}

impl SoundcoreUIEvent {
    fn to_str(&self) -> &str {
        match self {
            SoundcoreUIEvent::TrayUpdate => "tray_update",
            SoundcoreUIEvent::SoundcoreEvent => "soundcore_event"
        }
    }
}

#[tauri::command]
async fn soundcore_command(message: SoundcoreRequestMessage, state: tauri::State<'_, SoundcoreAppState>) -> Result<(), String> {
    let tx = state.soundcore_tx.lock().await;
    tx.send(message).await.map_err(|e| e.to_string())
}