use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct SoundcoreDeviceState {
    eq: u8,
    anc_mode: u8,
    charging_status: u8,
    battery_level: DeviceTypeDependantState<u8>,
    ldac_status: u8,
}

#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceTypeDependantState<T> {
    Earbuds((T, T)),
    Headphones(T),
}

fn a() {
    let state = SoundcoreDeviceState {
        eq: 0,
        anc_mode: 0,
        charging_status: 0,
        battery_level: DeviceTypeDependantState::Headphones(0),
        ldac_status: 0,
    };
    let _level = state.battery_level;
}
