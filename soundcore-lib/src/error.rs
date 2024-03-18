use thiserror::Error;

pub type SoundcoreLibResult<E> = std::result::Result<E, SoundcoreLibError>;
type ErrorWrapper = Box<dyn std::error::Error + Send + Sync>;

#[derive(Error, Debug)]
pub enum SoundcoreLibError {
    #[error("Unknown error")]
    Unknown,
    #[error("Connection error")]
    ConnectionError,
    #[error("Not connected")]
    NotConnected,
    #[error("Device Not Found")]
    DeviceNotFound,
    #[error("Parse error")]
    ParseError,
    #[error("Response checksum error")]
    ResponseChecksumError,
    #[error("Send error")]
    SendError,
    #[error("Recv error")]
    RecvError,
    #[error("Feature not supported/implemented: {0}")]
    FeatureNotSupported(String),
    #[error("Missing service: {0}")]
    MissingService(String),
    #[error("Cannot find UUID set for device: {0}")]
    MissingUUIDSet(String),
    #[error("Missing characteristic: {0}")]
    MissingCharacteristic(String),
    #[error("Unable to retrieve initial state for device: {0}")]
    MissingInitialState(String),
    #[error("Win32 error: {0}")]
    WinError(String),
    #[error("IO error")]
    IO {
        #[from]
        source: std::io::Error,
    },
    #[error("ParseInt error")]
    ParseInt {
        #[from]
        source: std::num::ParseIntError,
    },
    #[error("FromUtf8 error")]
    FromUtf8 {
        #[from]
        source: std::string::FromUtf8Error,
    },
    #[error("Bluetooth Error")]
    BthError {
        #[from]
        source: bluetooth_lib::BthError,
    },
    #[error("Invalid arguments")]
    InvalidArguments,
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Invalid response length (expected {expected}, got {got}, data: {data:?})")]
    InvalidResponseLength {
        expected: usize,
        got: usize,
        data: Vec<u8>,
    },
    #[error("Nom Parsing error")]
    NomParseError { error: String },
    #[error("Incompatible response")]
    IncompatibleResponse,
    #[error("Invalid MAC address: {addr}")]
    InvalidMACAddress { addr: String },
    // TODO: Remove btleplug-backend feature when device name resolution is fixed *see scanner.rs*
    #[cfg(all(target_os = "windows", any(feature = "btleplug-backend", feature = "winrt-backend")))]
    #[error("Unknown Windows Error")]
    UnknownWindowsError {
        #[from]
        source: windows::core::Error,
    },
    #[error("Tokio Error")]
    TokioError {
        #[from]
        source: tokio::task::JoinError,
    },
    #[cfg(feature = "btleplug-backend")]
    #[error("Btleplug Error")]
    BtleplugError {
        #[from]
        source: btleplug::Error,
    },
}

impl From<nom::Err<nom::error::VerboseError<&[u8]>>> for SoundcoreLibError {
    fn from(error: nom::Err<nom::error::VerboseError<&[u8]>>) -> Self {
        SoundcoreLibError::NomParseError {
            error: format!("{:?}", error),
        }
    }
}