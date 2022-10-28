use soundcore_lib::A3951::{A3951BatteryLevel, A3951BatteryCharging};
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


