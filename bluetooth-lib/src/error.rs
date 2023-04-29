use thiserror::Error;

#[derive(Debug, Error)]
pub enum BthError {
    #[error("Unknown error")]
    Unknown,
    #[error("FdInitError")]
    FdInitError,
    #[error("TryConnectError")]
    TryConnectError,
    #[error("SendError")]
    SendError,
    #[error("RecvError")]
    RecvError,
    #[error("ConversionError")]
    ConversionError,
    #[error("InvalidSocketError")]
    InvalidSocketError,
    #[error("Conversion error")]
    ParseInt {
        #[from]
        source: std::num::ParseIntError,
    },
    #[error("Windows error")]
    Windows {
        #[from]
        source: windows::core::Error,
    },
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Rfcomm service not found")]
    RfcommServiceNotFound,
}
