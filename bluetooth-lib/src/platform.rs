#[cfg(target_os = "macos")]
pub use crate::macos::{rfcomm::RFCOMM, scanner::BthScanner};

#[cfg(target_os = "windows")]
pub use crate::{win32::scanner::BthScanner, winrt::rfcomm::RFCOMM};
