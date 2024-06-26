use serde::Serialize;

use soundcore_lib::api::SoundcoreDeviceState;
use soundcore_lib::ble::BLEAdapterEvent;
use soundcore_lib::btaddr::BluetoothAdrr;
use soundcore_lib::device_manager::DiscoveredDevice;
use typeshare::typeshare;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "kind", content = "payload")]
#[typeshare]
pub enum BridgeResponse {
    ScanResult(Vec<DiscoveredDevice>),
    ConnectionEstablished(TaggedStateResponse),
    NewState(TaggedStateResponse),
    Disconnected(BluetoothAdrr),
    DisconnectedAll,
    AdapterEvent(BLEAdapterEvent),
    ConnectionFailed(ConnectionFailedResponse),
    GenericError(String),
    DeviceNotFound(BluetoothAdrr),
    SoundModeUpdated(BluetoothAdrr),
    EqualizerUpdated(BluetoothAdrr),
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct TaggedStateResponse {
    pub addr: BluetoothAdrr,
    pub state: SoundcoreDeviceState,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct ConnectionFailedResponse {
    pub addr: BluetoothAdrr,
    pub reason: String,
}
