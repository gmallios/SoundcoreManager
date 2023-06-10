#[cfg(any(target_os = "macos", target_os = "linux"))]
pub(crate) mod btleplug;

#[cfg(target_os = "windows")]
pub(crate) mod windows;

pub(crate) mod ble;
