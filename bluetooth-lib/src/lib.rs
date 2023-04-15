pub mod error;
mod types;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
mod winrt;

pub mod platform;
pub use {
    error::BthError,
    types::BluetoothDevice,
    types::{BluetoothAdrr, RFCOMMClient, Scanner},
};
