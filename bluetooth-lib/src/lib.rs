pub mod error;
mod types;


#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod win32;


pub mod platform;
pub use {error::BthError, types::{BluetoothAdrr, RFCOMMClient, Scanner}, types::BluetoothDevice};