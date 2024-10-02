use serde::{Deserialize, Serialize};
use std::fmt::Display;
use typeshare::typeshare;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[typeshare]
pub struct DeviceFirmware {
    primary: FirmwareVer,
    secondary: Option<FirmwareVer>,
}

impl DeviceFirmware {
    pub fn new(primary: FirmwareVer, secondary: Option<FirmwareVer>) -> Self {
        Self { primary, secondary }
    }

    pub fn primary(&self) -> FirmwareVer {
        self.primary
    }

    pub fn secondary(&self) -> Option<FirmwareVer> {
        self.secondary
    }
}

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
#[typeshare]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct FirmwareVer {
    major: u8,
    minor: u8,
}

impl FirmwareVer {
    pub fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }

    pub fn major(&self) -> u8 {
        self.major
    }

    pub fn minor(&self) -> u8 {
        self.minor
    }
}

impl Display for FirmwareVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.{:02}", self.major, self.minor)
    }
}

impl From<FirmwareVer> for Option<DeviceFirmware> {
    fn from(val: FirmwareVer) -> Self {
        Some(DeviceFirmware::new(val, None))
    }
}

#[cfg(test)]
mod fw_tests {
    use super::*;

    #[test]
    fn simple_init() {
        let fw = FirmwareVer::new(1, 2);
        assert_eq!(fw.major(), 1);
        assert_eq!(fw.minor(), 2);
        assert_eq!(fw.to_string(), "01.02");
    }

    #[test]
    fn comparison() {
        let new = FirmwareVer::new(1, 22);
        let old = FirmwareVer::new(1, 21);
        assert!(new > old);
    }
}
