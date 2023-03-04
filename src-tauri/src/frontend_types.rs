use serde::{Deserialize, Serialize};
use soundcore_lib::types::{BatteryCharging, BatteryLevel};
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
    pub charging: BatteryCharging,
    pub level: BatteryLevel,
    pub anc_mode: ANCModes,
}


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
