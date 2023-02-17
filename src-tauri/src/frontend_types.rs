use serde::{Deserialize, Serialize};
use ts_rs::TS;
use typeshare::typeshare;
// Run cargo test inside src-tauri to generate the typescript definitions
#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/DeviceSelection.d.ts")]
pub(crate) enum DeviceSelection {
    A3951,
    A3027,
    None,
}

#[derive(TS, Serialize, Deserialize, Clone, Copy, Debug)]
#[ts(export, export_to = "../src/bindings/ANCModes.d.ts")]
pub(crate) enum ANCModes {
    NormalMode,
    AncTransportMode,
    AncOutdoorMode,
    AncIndoorMode,
    AncCustomValue(u8),
    TransparencyFullyTransparentMode,
    TransparencyVocalMode,
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "../src/bindings/ScanResult.d.ts")]
pub(crate) struct BthScanResult {
    name: String,
    address: String,
    is_connected: bool,
}

impl From<bluetooth_lib::BluetoothDevice> for BthScanResult {
    fn from(device: bluetooth_lib::BluetoothDevice) -> Self {
        Self {
            name: device.name,
            address: device.address.to_string(),
            is_connected: device.connected,
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewTrayDeviceStatus {
    pub is_connected: bool,
    pub left_status: BatteryStatus,
    pub right_status: BatteryStatus,
    pub anc_mode: ANCModes,
}

#[derive(TS, Serialize, Deserialize, Debug)]
#[ts(export, export_to = "../src/bindings/BatteryStatus.d.ts")]
pub(crate) struct BatteryStatus {
    pub is_charging: bool,
    pub battery_level: u8,
}

// #[derive(TS, Serialize, Deserialize)]
// #[ts(export, export_to = "../src/bindings/Result.d.ts")]
// pub(crate) enum Result {
//     Ok,
//     Error
// }

// #[derive(TS, Serialize, Deserialize)]
// #[ts(export, export_to = "../src/bindings/BatteryInfo.d.ts")]
// pub(crate) struct BatteryLevelResponse {
//     pub(crate) level: A3951BatteryLevel,
//     pub(crate) charging: A3951BatteryCharging
// }

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct DeviceFeatures {
    pub profiles: Vec<SupportedANCProfiles>,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub enum SupportedANCProfiles {
    Normal,
    AncTransportMode,
    AncOutdoorMode,
    AncIndoorMode,
    AncCustomValue,
    TransparencyFullyTransparentMode,
    TransparencyVocalMode,
}
