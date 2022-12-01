use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

use crate::client_types::{TrayDeviceStatus, BatteryStatus};

/* Sets the tray menu to either the basic or the extended one */
#[tauri::command]
pub(crate) async fn set_tray_menu(app_handle: AppHandle, is_connected: bool) {
    let tray_handle = app_handle.tray_handle();
    /* Set appropriate menu */
    match is_connected {
        true => {
            tray_handle.set_menu(build_extended_menu()).unwrap();
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
pub(crate) async fn set_tray_device_status(app_handle: AppHandle, status: TrayDeviceStatus) {
    let tray_handle = app_handle.tray_handle();
    /* Update menu items */
    let conn_status = tray_handle.get_item("conn_status");
    let anc_status = tray_handle.get_item("anc_status");
    let charging_status = tray_handle.get_item("batt_charging_status");
    let battery_level = tray_handle.get_item("batt_level_status");
    match status.is_connected {
        true => {
            /* When connected the menu should be the extended one already */
            conn_status.set_title("Connected").unwrap();
            battery_level.set_title(format!("Battery Level: L: {}% R: {}%", status.left_status.battery_level*2*10, status.right_status.battery_level*2*10)).unwrap()
        }
        false => {
            conn_status.set_title("Disconnected").unwrap();
        }
    }
    let anc_text: String = match status.anc_mode {
        crate::client_types::ANCModes::NormalMode => "ANC: Normal Mode".to_string(),
        crate::client_types::ANCModes::AncTransportMode => "ANC: Transport Mode".to_string(),
        crate::client_types::ANCModes::AncOutdoorMode => "ANC: Outdoor Mode".to_string(),
        crate::client_types::ANCModes::AncIndoorMode => "ANC: Indoor Mode".to_string(),
        crate::client_types::ANCModes::AncCustomValue(val) => format!("ANC: Custom Value {}", val),
        crate::client_types::ANCModes::TransparencyFullyTransparentMode => "Transparency: Fully Transparent Mode".to_string(),
        crate::client_types::ANCModes::TransparencyVocalMode => "Transparency: Vocal Mode".to_string(),
    };
    anc_status.set_title(anc_text).unwrap();

    match status {
        TrayDeviceStatus {
            is_connected: true,
            left_status: BatteryStatus {
                is_charging: true,
                ..
            },
            right_status: BatteryStatus {
                is_charging: true,
                ..
            },
            ..
        } => {
            // Both Charging
            charging_status.set_title("Both Charging").unwrap();
        },
        TrayDeviceStatus {
            is_connected: true,
            left_status: BatteryStatus {
                is_charging: true,
                ..
            },
            right_status: BatteryStatus {
                is_charging: false,
                ..
            },
            ..
        } => {
            // Left Charging
            charging_status.set_title("Left Charging").unwrap();
        },
        TrayDeviceStatus {
            is_connected: true,
            left_status: BatteryStatus {
                is_charging: false,
                ..
            },
            right_status: BatteryStatus {
                is_charging: true,
                ..
            },
            ..
        } => {
            // Right Charging
            charging_status.set_title("Right Charging").unwrap();
        },
        TrayDeviceStatus {
            is_connected: true,
            left_status: BatteryStatus {
                is_charging: false,
                ..
            },
            right_status: BatteryStatus {
                is_charging: false,
                ..
            },
            ..
        } => {
            // Not Charging
            charging_status.set_title("Not Charging").unwrap();
        },
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

/* Menu used while connected */
fn build_extended_menu() -> SystemTrayMenu {
    let conn_status = CustomMenuItem::new("conn_status".to_string(), "Disconnected").disabled();
    let anc_status = CustomMenuItem::new("anc_status".to_string(), "ANC: Off").disabled();
    let batt_charging_status = CustomMenuItem::new("batt_charging_status".to_string(), "Battery: Is it charging?").disabled();
    let batt_level = CustomMenuItem::new("batt_level_status".to_string(), "Battery: L:?% R:?%").disabled();
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    SystemTrayMenu::new()
        .add_item(conn_status)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(anc_status)
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

/* Tray event handler */
pub(crate) fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    match window.is_visible().unwrap() {
                        true => {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        }
                        false => {
                            window.show().unwrap();
                            item_handle.set_title("Hide").unwrap();
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
        }
        _ => {}
    }
}
