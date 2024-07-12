use serde::Deserialize;
use typeshare::typeshare;

use soundcore_lib::btaddr::BluetoothAdrr;
use soundcore_lib::device_manager::DiscoveredDevice;
use soundcore_lib::models::{EQProfile, MonoEQ, SoundMode};

#[typeshare]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase", tag = "command", content = "payload")]
pub enum BridgeCommand {
    Scan,
    Connect(DiscoveredDevice),
    Disconnect(BluetoothAdrr),
    DisconnectAll,
    SetSoundMode(AddrWrappedPayload<SoundMode>),
    SetEqualizer(AddrWrappedPayload<SetEqualizerPayload>),
}
#[derive(Debug, Deserialize, Clone)]
#[typeshare]
#[serde(rename_all = "camelCase")]
pub struct AddrWrappedPayload<T> {
    pub addr: BluetoothAdrr,
    pub payload: T,
}

#[derive(Debug, Deserialize, Clone)]
#[typeshare]
#[serde(rename_all = "camelCase", tag = "command", content = "payload")]
pub enum SetEqualizerPayload {
    SetCustomEqualizer(Vec<i8>),
    SetEqualizerPreset(EQProfile),
}
