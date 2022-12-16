#[cfg(target_os = "macos")]
pub use crate::macos::{scanner::BthScanner, rfcomm::RFCOMM};

#[cfg(target_os = "windows")]
pub use crate::win32::{scanner::BthScanner, rfcomm::RFCOMM};


