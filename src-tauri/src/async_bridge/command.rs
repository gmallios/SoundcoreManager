use serde::Deserialize;
use typeshare::typeshare;
use soundcore_lib::btaddr::BluetoothAdrr;

use soundcore_lib::device_manager::DiscoveredDevice;

#[typeshare]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "command", content = "payload")]
pub enum BridgeCommand {
    Scan,
    Connect(DiscoveredDevice),
    Disconnect(BluetoothAdrr),
}
