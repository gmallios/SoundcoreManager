pub use bluetooth_lib::BluetoothAdrr;

pub mod api;
pub mod base;
pub mod ble;
pub mod btaddr;
pub mod compat;
pub mod devices;
pub mod error;
pub mod models;
pub mod packets;
mod parsers;
pub mod statics;
pub mod types;
#[allow(non_snake_case)]
mod utils;

pub mod device;
pub mod device_manager;

#[cfg(any(test, feature = "mock"))]
pub mod mocks;
