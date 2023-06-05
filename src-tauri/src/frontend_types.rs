use bluetooth_lib::BluetoothDevice;
use serde::{Deserialize, Serialize};
use soundcore_lib::types::{BatteryCharging, BatteryLevel, SupportedModels};
use typeshare::typeshare;



#[typeshare]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(tag = "mode", content = "value")]
pub(crate) enum ANCModes {
    NormalMode,
    AncTransportMode,
    AncOutdoorMode,
    AncIndoorMode,
    AncCustomValue(u8),
    TransparencyFullyTransparentMode,
    TransparencyVocalMode,
}


#[typeshare]
#[derive(Serialize, Deserialize)]
pub(crate) struct BthScanResult {
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) is_connected: bool,
    pub(crate) modelid: SupportedModels,
}

impl From<(bluetooth_lib::BluetoothDevice, SupportedModels)> for BthScanResult {
    fn from(args: (BluetoothDevice, SupportedModels)) -> Self {
        Self {
            name: args.0.name,
            address: args.0.address.to_string(),
            is_connected: args.0.connected,
            modelid: args.1,
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewTrayDeviceStatus {
    pub is_connected: bool,
    pub charging: BatteryCharging,
    pub level: BatteryLevel,
    pub anc_mode: ANCModes,
}