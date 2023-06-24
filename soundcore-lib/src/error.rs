use crate::devices::SupportedModelIDs;

pub type SoundcoreResult<E> = std::result::Result<E, SoundcoreError>;
type ErrorWrapper = Box<dyn std::error::Error + Send + Sync>;

#[derive(thiserror::Error, Debug)]
pub enum SoundcoreError {
    #[error("Device not found")]
    DeviceNotFound { source: ErrorWrapper },
    #[error("No UUID set found for device model")]
    NoUUIDSetFoundForDeviceModel { model_id: SupportedModelIDs },
    #[error("Device is not supported")]
    DeviceNotSupported { name: String },
    #[error("Device not connected")]
    NotConnected { source: ErrorWrapper },
    #[error("BLE Service not found")]
    BLEServiceNotFound { uuid: String },
    #[error("BLE Characteristic not found")]
    BLECharacteristicNotFound { uuid: String },
    #[error("Invalid MAC Address")]
    InvalidMACAddress { addr: String },
    #[error("No response from device")]
    NoResponse,
    #[cfg(target_os = "windows")]
    #[error("Unknown Windows Error")]
    UnknownWindowsError {
        #[from]
        source: windows::core::Error,
    },
    #[error("Unknown Error")]
    UnknownError { source: ErrorWrapper },
    #[error("Mutex Lock Error")]
    MutexLockError {},
}
