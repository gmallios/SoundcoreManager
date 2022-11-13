use std::error::Error;

#[derive(Debug)]
pub enum BthError {
    Unknown,
    FdInitError,
    TryConnectError,
    SendError,
    RecvError,
    ConversionError,
    InvalidSocketError
}

impl Error for BthError {
    fn description(&self) -> &str {
        match self {
            BthError::Unknown => "Unknown Error",
            BthError::FdInitError => "Failed to initialize file descriptor",
            BthError::TryConnectError => "Failed to connect to device",
            BthError::SendError => "Failed to send data",
            BthError::RecvError => "Failed to receive data",
            BthError::ConversionError => "Failed to convert data",
            BthError::InvalidSocketError => "Invalid socket"
        }
    }
}

impl std::fmt::Display for BthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BthError::Unknown => write!(f, "Unknown Error"),
            BthError::FdInitError => write!(f, "Failed to initialize fd"),
            BthError::TryConnectError => write!(f, "Failed to connect to device"),
            BthError::SendError => write!(f, "Failed to send data"),
            BthError::RecvError => write!(f, "Failed to receive data"),
            BthError::ConversionError => write!(f, "Failed to convert data"),
            BthError::InvalidSocketError => write!(f, "Invalid socket")
        }
    }
}

impl From<std::num::ParseIntError> for BthError {
    fn from(_error: std::num::ParseIntError) -> Self {
        BthError::ConversionError
    }
}