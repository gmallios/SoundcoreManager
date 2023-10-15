use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
// TODO: Create parser
pub enum DeviceFirmware {
    DUAL(FirmwareVer, FirmwareVer),
    SINGLE(FirmwareVer),
}

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash,
)]
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

impl ToString for FirmwareVer {
    fn to_string(&self) -> String {
        format!("{:02}.{:02}", self.major, self.minor)
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
