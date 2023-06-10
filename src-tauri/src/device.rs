use crate::{
    frontend_types::ANCModes,
    utils::{anc_mode_to_profile, anc_profile_to_mode},
    SoundcoreAppState,
};
use log::debug;
use soundcore_lib::{
    base::SoundcoreDevice,
    devices::{A3027, A3040, A3935, A3951},
    types::{BatteryCharging, BatteryLevel, DeviceInfo, DeviceStatus, EQWave, SupportedModels},
    BluetoothAdrr,
};
use tauri::State;

#[tauri::command]
pub(crate) async fn is_connected(state: State<'_, SoundcoreAppState>) -> Result<bool, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let _status = device.get_info().await.map_err(|e| e.to_string())?;
    Ok(true)
}

#[tauri::command]
pub(crate) async fn close(state: State<'_, SoundcoreAppState>) -> Result<(), String> {
    debug!("Closing device");
    let mut device = state.device.lock().await;
    if device.is_some() {
        device.as_ref().unwrap().close().await.unwrap();
    }
    *device = None;
    *state.model.write().await = None;
    Ok(())
}

#[tauri::command]
pub(crate) async fn get_model(
    state: State<'_, SoundcoreAppState>,
) -> Result<SupportedModels, String> {
    let model = state.model.read().await;
    model.clone().ok_or("No device connected".to_string())
}

#[tauri::command]
pub(crate) async fn connect(
    app_state: State<'_, SoundcoreAppState>,
    bt_name: String,
    bt_addr: String,
) -> Result<(), String> {
    /* Check if device is connected */
    {
        let mut device = app_state.device.lock().await;
        if device.is_some() {
            device.as_ref().unwrap().close().await.unwrap();
            *device = None;
        }
    }

    let device_model = soundcore_lib::types::SOUNDCORE_NAME_MODEL_MAP
        .get(&bt_name)
        .ok_or(format!(
            "No Model ID found for given bluetooth name: {}",
            bt_name
        ))?;

    let mut device_state = app_state.device.lock().await;
    match device_model {
        SupportedModels::A3951 => {
            let device = A3951::default()
                .init(BluetoothAdrr::from(bt_addr))
                .await
                .map_err(|e| e.to_string())?;
            *device_state = Some(device);
        }
        SupportedModels::A3935 => {
            let device = A3935::default()
                .init(BluetoothAdrr::from(bt_addr))
                .await
                .map_err(|e| e.to_string())?;
            *device_state = Some(device);
        }
        SupportedModels::A3027 | SupportedModels::A3028 | SupportedModels::A3029 => {
            let device = A3027::default()
                .init(BluetoothAdrr::from(bt_addr))
                .await
                .map_err(|e| e.to_string())?;
            *device_state = Some(device);
        }
        SupportedModels::A3040 => {
            let device = A3040::default()
                .init(BluetoothAdrr::from(bt_addr))
                .await
                .map_err(|e| e.to_string())?;
            *device_state = Some(device);
        }
    };

    *app_state.model.write().await = Some(device_model.clone());
    debug!("Connected to device: {}", bt_name);
    Ok(())
}

#[tauri::command]
pub(crate) async fn get_info(state: State<'_, SoundcoreAppState>) -> Result<DeviceInfo, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let info = device.get_info().await.map_err(|e| e.to_string())?;
    Ok(info)
}

#[tauri::command]
pub(crate) async fn get_status(
    state: State<'_, SoundcoreAppState>,
) -> Result<DeviceStatus, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let status = device.get_status().await.map_err(|e| e.to_string())?;
    Ok(status)
}

#[tauri::command]
pub(crate) async fn get_battery_level(
    state: State<'_, SoundcoreAppState>,
) -> Result<BatteryLevel, String> {
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
    state: State<'_, SoundcoreAppState>,
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
pub(crate) async fn get_anc(state: State<'_, SoundcoreAppState>) -> Result<ANCModes, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let anc = device.get_anc().await.map_err(|e| e.to_string())?;
    Ok(anc_profile_to_mode(anc))
}

#[tauri::command]
pub(crate) async fn set_anc(
    state: State<'_, SoundcoreAppState>,
    mode: ANCModes,
) -> Result<(), String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    device
        .set_anc(anc_mode_to_profile(mode))
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub(crate) async fn get_eq(state: State<'_, SoundcoreAppState>) -> Result<EQWave, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let eq = device.get_eq().await.map_err(|e| e.to_string())?;
    Ok(eq)
}

#[tauri::command]
pub(crate) async fn set_eq(state: State<'_, SoundcoreAppState>, eq: EQWave) -> Result<(), String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    device.set_eq(eq).await.map_err(|e| e.to_string())?;
    Ok(())
}
