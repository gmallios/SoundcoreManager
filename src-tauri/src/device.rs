use crate::{
    frontend_types::ANCModes,
    utils::{anc_mode_to_profile, anc_profile_to_mode},
    SoundcoreAppState,
};
use serde::Serialize;
use soundcore_lib::{
    base::SoundcoreDevice,
    devices::{A3027, A3951},
    types::{BatteryCharging, BatteryLevel, DeviceInfo, DeviceStatus, EQWave},
    BluetoothAdrr,
};
use tauri::{utils::assets::phf::phf_map, State};
use typeshare::typeshare;

#[tauri::command]
pub(crate) async fn is_connected(state: State<'_, SoundcoreAppState>) -> Result<bool, String> {
    let device = state.device.lock().await;
    let device = device.as_ref().ok_or("No device connected")?;
    let _status = device.get_info().await.map_err(|e| e.to_string())?;
    Ok(true)
}

#[tauri::command]
pub(crate) async fn close(state: State<'_, SoundcoreAppState>) -> Result<(), String> {
    let mut device = state.device.lock().await;
    if device.is_some() {
        device.as_ref().unwrap().close().await.unwrap();
    }
    *device = None;
    *state.model.write().await = None;
    Ok(())
}

#[typeshare]
#[derive(Clone, Serialize)]
pub(crate) enum SupportedModel {
    A3951,
    A3027,
    A3028,
    A3029,
}

/* Maps Bluetooth Name to SupportedModel */
static SOUNDCORE_NAME_MODEL_MAP: phf::Map<&'static str, SupportedModel> = phf_map! {
    "Soundcore Liberty Air 2 Pro" => SupportedModel::A3951,
    "Soundcore Life Q35" => SupportedModel::A3027,
    "Soundcore Q35" => SupportedModel::A3027, /* EU Variant */
    "Soundcore Life Q30" => SupportedModel::A3028,
    "Soundcore Q30" => SupportedModel::A3028, /* EU Variant */
    "Soundcore Life Tune" => SupportedModel::A3029,
};

#[tauri::command]
pub(crate) async fn get_model(
    state: State<'_, SoundcoreAppState>,
) -> Result<SupportedModel, String> {
    let model = state.model.read().await;
    model.clone().ok_or("No device connected".to_string())
}

#[tauri::command]
pub(crate) async fn connect(
    state: State<'_, SoundcoreAppState>,
    bt_name: String,
    bt_addr: String,
) -> Result<(), String> {
    /* Check if device is connected */
    {
        let mut device = state.device.lock().await;
        if device.is_some() {
            device.as_ref().unwrap().close().await.unwrap();
            *device = None;
        }
    }

    let device_model = SOUNDCORE_NAME_MODEL_MAP.get(&bt_name).ok_or(format!(
        "No Model ID found for given bluetooth name: {}",
        bt_name
    ))?;
    *state.model.write().await = Some(device_model.clone());

    match device_model {
        SupportedModel::A3951 => {
            let device = A3951::default()
                .init(BluetoothAdrr::from(bt_addr))
                .await
                .map_err(|e| e.to_string())?;
            let mut a = state.device.lock().await;
            *a = Some(device);
        }
        SupportedModel::A3027 | SupportedModel::A3028 | SupportedModel::A3029 => {
            let device = A3027::default()
                .init(BluetoothAdrr::from(bt_addr))
                .await
                .map_err(|e| e.to_string())?;
            let mut a = state.device.lock().await;
            *a = Some(device);
        }
    };
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
