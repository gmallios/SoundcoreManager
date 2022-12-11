use thiserror::Error;


#[derive(Error, Debug)]
pub enum SoundcoreError {
    #[error("Unknown error")]
    Unknown,
    #[error("Connection error")]
    ConnectionError,
    #[error("Not connected")]
    NotConnected,
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
        source: bluetooth_lib::BthError
    },
}

// impl std::fmt::Display for SoundcoreError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             SoundcoreError::Unknown => write!(f, "Unknown error"),
//             SoundcoreError::ParseError => write!(f, "Parse error"),
//             SoundcoreError::ResponseChecksumError => write!(f, "Response checksum error"),
//             SoundcoreError::SendError => write!(f, "Send error"),
//             SoundcoreError::RecvError => write!(f, "Recv error"),
//             SoundcoreError::FeatureNotSupported(feat) => write!(f, "Feature not supported/implemented: {}", feat),
//             SoundcoreError::WinError(e) => write!(f, "Win32 error: {}", e),
//         }
//     }
// }

// impl std::error::Error for SoundcoreError {}

// impl From<std::io::Error> for SoundcoreError {
//     fn from(_error: std::io::Error) -> Self {
//         SoundcoreError::Unknown
//     }
// }

// impl From<std::num::ParseIntError> for SoundcoreError {
//     fn from(_error: std::num::ParseIntError) -> Self {
//         SoundcoreError::Unknown
//     }
// }


// impl From<std::string::FromUtf8Error> for SoundcoreError {
//     fn from(_error: std::string::FromUtf8Error) -> Self {
//         SoundcoreError::ParseError
//     }
// }
