use serde::Serialize;

use soundcore_lib::api::SoundcoreDeviceState;
use soundcore_lib::btaddr::BluetoothAdrr;
use soundcore_lib::device_manager::DiscoveredDevice;

#[derive(Debug, Serialize, Clone)]
pub enum BridgeResponse {
    ScanResult(Vec<DiscoveredDevice>),
    ConnectionEstablished(BluetoothAdrr),
    NewState((BluetoothAdrr, SoundcoreDeviceState)),
    Disconnected(BluetoothAdrr),
    Error(String),
}

unsafe impl Send for BridgeResponse {}
unsafe impl Sync for BridgeResponse {}
