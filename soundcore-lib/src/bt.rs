#[cfg(all(any(target_os = "macos", target_os = "linux"), feature = "bluetooth"))]
pub(crate) mod btleplug;

#[cfg(all(target_os = "windows", feature = "bluetooth"))]
pub(crate) mod windows;

pub mod ble;
