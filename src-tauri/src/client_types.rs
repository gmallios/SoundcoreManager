use ts_rs::TS;
use serde::{Serialize, Deserialize};
// Run cargo test inside src-tauri to generate the typescript definitions
#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "../src/bindings/DeviceSelection.d.ts")]
pub(crate) enum DeviceSelection {
    A3951,
    None
}


#[derive(TS, Serialize, Deserialize)]
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


