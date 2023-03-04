use log::debug;

use soundcore_lib::types::SupportedModels;
use tauri::{
    AppHandle, CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu,
};

use crate::{
    frontend_types::{ANCModes, BatteryStatus, NewTrayDeviceStatus},
    SoundcoreAppState,
};

/* Sets the tray menu to either the basic or the extended one */
#[tauri::command]
pub(crate) async fn set_tray_menu(app_handle: AppHandle, is_connected: bool) {
    debug!("Setting tray menu to connected status: {}", is_connected);
    let tray_handle = app_handle.tray_handle();
    /* Set appropriate menu */
    match is_connected {
        true => {
            tray_handle.set_menu(build_extended_menu(None)).unwrap();
        }
        false => {
            tray_handle.set_menu(build_base_tray_menu()).unwrap();
        }
    }
}

/* Handles tray menu updates coming from frontend for now */
/* When the event system menu is done updates should be emmited from the backend to avoid unnecessary overhead */
/* Also fixes the issue where when the window is hidden the setIntervals from the FE are not timed properly  */
#[tauri::command]
pub(crate) async fn set_tray_device_status(app_handle: AppHandle, status: NewTrayDeviceStatus) {
    debug!("Updating tray menu: {:?}", status);
    let tray_handle = app_handle.tray_handle();
    let state: State<SoundcoreAppState> = app_handle.state();
    /* Try to fix PoisonError bug which occurs randomly while refreshing the app */
    /* Remove set_tray_menu and use only this command? */
    match status.is_connected {
        true => {
            let model = state.model.read().await;
            tray_handle
                .set_menu(build_extended_menu(model.as_ref()))
                .unwrap();
        }
        false => {
            tray_handle.set_menu(build_base_tray_menu()).unwrap();
            return;
        }
    }
    /* Update menu items */
    let conn_status = tray_handle.get_item("conn_status");
    let charging_status = tray_handle.get_item("batt_charging_status");
    let battery_level = tray_handle.get_item("batt_level_status");
    match status.is_connected {
        true => {
            /* When connected the menu should be the extended one already */
            conn_status.set_title("Connected").unwrap();
            battery_level
                .set_title(format!(
                    "Battery Level: L: {}% R: {}%",
                    status.left_status.battery_level * 2 * 10,
                    status.right_status.battery_level * 2 * 10
                ))
                .unwrap()
        }
        false => {
            conn_status.set_title("Disconnected").unwrap();
        }
    }

    let anc_items = [
        "anc_sub_normal_mode",
        "anc_sub_transport_mode",
        "anc_sub_outdoor_mode",
        "anc_sub_indoor_mode",
        "anc_sub_fully_transparent_mode",
        "anc_sub_vocal_mode",
    ];

    let idx_to_enable = match status.anc_mode {
        ANCModes::NormalMode => 0,
        ANCModes::AncTransportMode => 1,
        ANCModes::AncOutdoorMode => 2,
        ANCModes::AncIndoorMode => 3,
        ANCModes::TransparencyFullyTransparentMode => 4,
        ANCModes::TransparencyVocalMode => 5,
        _ => 0,
    };

    anc_items.iter().enumerate().for_each(|(idx, item)| {
        let item = tray_handle.get_item(item);
        if idx == idx_to_enable {
            item.set_selected(true).unwrap();
        } else {
            item.set_selected(false).unwrap();
        }
    });

    match status {
        NewTrayDeviceStatus {
            is_connected: true,
            left_status: BatteryStatus {
                is_charging: true, ..
            },
            right_status: BatteryStatus {
                is_charging: true, ..
            },
            ..
        } => {
            // Both Charging
            charging_status.set_title("Both Charging").unwrap();
        }
        NewTrayDeviceStatus {
            is_connected: true,
            left_status: BatteryStatus {
                is_charging: true, ..
            },
            right_status: BatteryStatus {
                is_charging: false, ..
            },
            ..
        } => {
            // Left Charging
            charging_status.set_title("Left Charging").unwrap();
        }
        NewTrayDeviceStatus {
            is_connected: true,
            left_status: BatteryStatus {
                is_charging: false, ..
            },
            right_status: BatteryStatus {
                is_charging: true, ..
            },
            ..
        } => {
            // Right Charging
            charging_status.set_title("Right Charging").unwrap();
        }
        NewTrayDeviceStatus {
            is_connected: true,
            left_status: BatteryStatus {
                is_charging: false, ..
            },
            right_status: BatteryStatus {
                is_charging: false, ..
            },
            ..
        } => {
            // Not Charging
            charging_status.set_title("Not Charging").unwrap();
        }
        _ => {
            charging_status.set_title("Not Charging").unwrap();
        }
    }
}

/* Menu used while disconnected */
fn build_base_tray_menu() -> SystemTrayMenu {
    let conn_status = CustomMenuItem::new("conn_status".to_string(), "Disconnected").disabled();
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    SystemTrayMenu::new()
        .add_item(conn_status)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(quit)
}

fn build_anc_menu(model: &SupportedModels) -> SystemTrayMenu {
    let normal_mode = CustomMenuItem::new("anc_sub_normal_mode".to_string(), "Normal Mode");
    let transport_mode =
        CustomMenuItem::new("anc_sub_transport_mode".to_string(), "ANC: Transport Mode");
    let outdoor_mode = CustomMenuItem::new("anc_sub_outdoor_mode".to_string(), "ANC: Outdoor Mode");
    let indoor_mode = CustomMenuItem::new("anc_sub_indoor_mode".to_string(), "ANC: Indoor Mode");
    let fully_transparent = CustomMenuItem::new(
        "anc_sub_fully_transparent_mode".to_string(),
        "Transparency: Fully Transparent Mode",
    );
    let vocal_mode =
        CustomMenuItem::new("anc_sub_vocal_mode".to_string(), "Transparency: Vocal Mode");

    match model {
        SupportedModels::A3951 => SystemTrayMenu::new()
            .add_item(indoor_mode)
            .add_item(outdoor_mode)
            .add_item(transport_mode)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(normal_mode)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(fully_transparent)
            .add_item(vocal_mode),
        SupportedModels::A3027 | SupportedModels::A3028 | SupportedModels::A3029 => {
            SystemTrayMenu::new()
                .add_item(indoor_mode)
                .add_item(outdoor_mode)
                .add_item(transport_mode)
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(normal_mode)
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(fully_transparent)
        }
        _ => SystemTrayMenu::new()
            .add_item(CustomMenuItem::new("unsoported".to_string(), "Not supported"))
    }
}

/* Menu used while connected */
fn build_extended_menu(model: Option<&SupportedModels>) -> SystemTrayMenu {
    let conn_status = CustomMenuItem::new("conn_status".to_string(), "Disconnected").disabled();
    let anc_submenu = match model {
        Some(model) => SystemTraySubmenu::new("ANC Profiles", build_anc_menu(model)),
        None => SystemTraySubmenu::new("ANC Profiles", SystemTrayMenu::new()),
    };
    let batt_charging_status = CustomMenuItem::new(
        "batt_charging_status".to_string(),
        "Battery: Is it charging?",
    )
    .disabled();
    let batt_level =
        CustomMenuItem::new("batt_level_status".to_string(), "Battery: L:?% R:?%").disabled();
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    SystemTrayMenu::new()
        .add_item(conn_status)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(anc_submenu)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(batt_charging_status)
        .add_item(batt_level)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(quit)
}

/* Gets called on init, we are disconnected so return base menu */
pub(crate) fn get_system_tray() -> SystemTray {
    SystemTray::new().with_menu(build_base_tray_menu())
}

fn handle_anc_submenu(app: &AppHandle, id: String) {
    let anc_mode = match id.as_str() {
        "anc_sub_normal_mode" => ANCModes::NormalMode,
        "anc_sub_transport_mode" => ANCModes::AncTransportMode,
        "anc_sub_outdoor_mode" => ANCModes::AncOutdoorMode,
        "anc_sub_indoor_mode" => ANCModes::AncIndoorMode,
        "anc_sub_fully_transparent_mode" => ANCModes::TransparencyFullyTransparentMode,
        "anc_sub_vocal_mode" => ANCModes::TransparencyVocalMode,
        _ => ANCModes::NormalMode,
    };
    app.emit_all("anc_sub_change", anc_mode).unwrap();
}

/* Tray event handler */
pub(crate) fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            debug!("Menu item clicked: {}", id);
            if id.starts_with("anc_sub") {
                handle_anc_submenu(&app.clone(), id);
                return;
            }
            match id.as_str() {
                "hide" => {
                    let window = app.get_window("main").unwrap_or_else(|| panic!("Could not get window"));
                    match window.is_visible() {
                        Ok(true) => {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        }
                        Ok(false) => {
                            window.show().unwrap();
                            item_handle.set_title("Hide").unwrap();
                        },
                        Err(e) => {
                            panic!("Could not get window visibility: {}", e);
                        }
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        }
        SystemTrayEvent::LeftClick { .. } => {
            let window = app.get_window("main").unwrap();
            app.tray_handle()
                .get_item("hide")
                .set_title("Hide")
                .unwrap();
            window.show().unwrap();
            window.set_focus().unwrap();
        }
        _ => {}
    }
}
