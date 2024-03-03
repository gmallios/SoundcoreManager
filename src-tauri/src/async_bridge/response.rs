use serde::Serialize;

use soundcore_lib::api::SoundcoreDeviceState;
use soundcore_lib::btaddr::BluetoothAdrr;
use soundcore_lib::device_manager::DiscoveredDevice;
use typeshare::typeshare;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "kind", content = "payload")]
#[typeshare]
pub enum BridgeResponse {
    ScanResult(Vec<DiscoveredDevice>),
    ConnectionEstablished(BluetoothAdrr),
    NewState(NewStateResponse),
    Disconnected(BluetoothAdrr),
    Error(String),
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct NewStateResponse {
    pub addr: BluetoothAdrr,
    pub state: SoundcoreDeviceState,
}