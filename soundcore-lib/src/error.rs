pub type SoundcoreResult<E> = std::result::Result<E, SoundcoreError>;
type ErrorWrapper = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum SoundcoreError {
    #[error("Device not found")]
    DeviceNotFound { source: ErrorWrapper },
    #[error("Device not connected")]
    NotConnected { source: ErrorWrapper },
    #[error("BLE Service not found")]
    BLEServiceNotFound { uuid: String },
    #[error("BLE Characteristic not found")]
    BLECharacteristicNotFound { uuid: String },
    #[error("Invalid MAC Address")]
    InvalidMACAddress { addr: String },
    #[cfg(target_os = "windows")]
    #[error("Unknown Windows Error")]
    UnknownWindowsError {
        #[from]
        source: windows::core::Error,
    },
    #[error("Unknown Error")]
    UnknownError { source: ErrorWrapper },
}
