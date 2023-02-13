use crate::{
    frontend_types::{ANCModes, DeviceSelection},
    utils::{anc_mode_to_profile, anc_profile_to_mode},
    AppState,
};
use soundcore_lib::{
    base::SoundcoreDevice,
    devices::{A3951, A3027},
    types::{BatteryCharging, BatteryLevel, DeviceInfo, DeviceStatus, EQWave},
    BluetoothAdrr,
};
use tauri::State;

#[tauri::command]
pub(crate) async fn is_connected(state: State<'_, AppState>) -> Result<bool, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let _status = device.get_info().await.map_err(|e| e.to_string())?;
    Ok(true)
}

#[tauri::command]
pub(crate) async fn close(state: State<'_, AppState>) -> Result<(), String> {
    let mut device = state.device.lock().await;
    if device.is_some() {
        device.as_ref().unwrap().close().await.unwrap();
    }
    *device = None;
    Ok(())
}

#[tauri::command]
pub(crate) async fn connect(
    state: State<'_, AppState>,
    selection: DeviceSelection,
    addr: String,
) -> Result<(), String> {
    /* Check if device is connected */
    {
        let mut device = state.device.lock().await;
        if device.is_some() {
            device.as_ref().unwrap().close().await.unwrap();
            *device = None;
        }
    }
    match selection {
        DeviceSelection::A3951 => {
            let device = A3951::default()
                .init(BluetoothAdrr::from(addr))
                .await
                .map_err(|e| e.to_string())?;
            let mut a = state.device.lock().await;
            *a = Some(device);
        },
        DeviceSelection::A3027 => {
            let device = A3027::default()
                .init(BluetoothAdrr::from(addr))
                .await
                .map_err(|e| e.to_string())?;
            let mut a = state.device.lock().await;
            *a = Some(device);
        },
        DeviceSelection::None => return Err("No device selected".to_string()),
    };
    Ok(())
}

#[tauri::command]
pub(crate) async fn get_info(state: State<'_, AppState>) -> Result<DeviceInfo, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let info = device.get_info().await.map_err(|e| e.to_string())?;
    Ok(info)
}

#[tauri::command]
pub(crate) async fn get_status(state: State<'_, AppState>) -> Result<DeviceStatus, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let status = device.get_status().await.map_err(|e| e.to_string())?;
    Ok(status)
}

#[tauri::command]
pub(crate) async fn get_battery_level(state: State<'_, AppState>) -> Result<BatteryLevel, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let battery_level = device
        .get_battery_level()
        .await
        .map_err(|e| e.to_string())?;
    Ok(battery_level)
}

#[tauri::command]
pub(crate) async fn get_battery_charging(
    state: State<'_, AppState>,
) -> Result<BatteryCharging, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let battery_charging = device
        .get_battery_charging()
        .await
        .map_err(|e| e.to_string())?;
    Ok(battery_charging)
}

#[tauri::command]
pub(crate) async fn get_anc(state: State<'_, AppState>) -> Result<ANCModes, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let anc = device.get_anc().await.map_err(|e| e.to_string())?;
    Ok(anc_profile_to_mode(anc))
}

#[tauri::command]
pub(crate) async fn set_anc(state: State<'_, AppState>, mode: ANCModes) -> Result<(), String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    device
        .set_anc(anc_mode_to_profile(mode))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub(crate) async fn get_eq(state: State<'_, AppState>) -> Result<EQWave, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let eq = device.get_eq().await.map_err(|e| e.to_string())?;
    Ok(eq)
}

#[tauri::command]
pub(crate) async fn set_eq(state: State<'_, AppState>, eq: EQWave) -> Result<(), String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    device.set_eq(eq).await.map_err(|e| e.to_string())?;
    Ok(())
}
