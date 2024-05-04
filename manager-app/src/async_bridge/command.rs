use serde::Deserialize;
use soundcore_lib::btaddr::BluetoothAdrr;
use soundcore_lib::models::SoundMode;
use typeshare::typeshare;

use soundcore_lib::device_manager::DiscoveredDevice;

#[typeshare]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "command", content = "payload")]
pub enum BridgeCommand {
    Scan,
    Connect(DiscoveredDevice),
    Disconnect(BluetoothAdrr),
    DisconnectAll,
    SetSoundMode(SetSoundModePayload),
}
#[derive(Debug, Deserialize, Clone)]
#[typeshare]
#[serde(rename_all = "camelCase")]
pub struct SetSoundModePayload {
    pub addr: BluetoothAdrr,
    pub sound_mode: SoundMode,
}

#[derive(Debug, Deserialize, Clone)]
#[typeshare]
#[serde(rename_all = "camelCase")]
pub enum SetEqualizerPayload {
    SetCustomEqualizer,
    SetEqualizerPreset,
}