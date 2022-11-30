#[derive(Debug)]
pub enum SoundcoreError {
    Unknown,
    ParseError,
    ResponseChecksumError,
    SendError,
    RecvError,
    WinError(String),
}

impl std::fmt::Display for SoundcoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:}", std::error::Error::description(self))
    }
}

impl std::error::Error for SoundcoreError {
    fn description(&self) -> &str {
        match self {
            SoundcoreError::Unknown => "Unknown Error",
            SoundcoreError::ParseError => "Parse Error",
            SoundcoreError::ResponseChecksumError => "Response Checksum Error",
            SoundcoreError::WinError(ref message) => message.as_str(),
            SoundcoreError::SendError => "Send Error",
            SoundcoreError::RecvError => "Recv Error",
        }
    }
}

impl From<std::io::Error> for SoundcoreError {
    fn from(_error: std::io::Error) -> Self {
        SoundcoreError::Unknown
    }
}

impl From<std::num::ParseIntError> for SoundcoreError {
    fn from(_error: std::num::ParseIntError) -> Self {
        SoundcoreError::Unknown
    }
}


impl From<std::string::FromUtf8Error> for SoundcoreError {
    fn from(_error: std::string::FromUtf8Error) -> Self {
        SoundcoreError::ParseError
    }
}
