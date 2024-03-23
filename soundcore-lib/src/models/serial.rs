use std::sync::Arc;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::types::SupportedModels;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
#[typeshare]
pub struct SerialNumber(pub Arc<str>);

impl SerialNumber {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_model(&self) -> Option<SupportedModels> {
        match self.as_str().get(0..4) {
            Some(v) => {
                ("A".to_owned() + v).parse().ok()
            }
            None => None,
        }
    }
}

impl Default for SerialNumber {
    fn default() -> Self {
        Self("XXXXXXXXXXXXXXXX".into())
    }
}

impl From<&str> for SerialNumber {
    fn from(s: &str) -> Self {
        SerialNumber(s.into())
    }
}


#[cfg(test)]
mod serial_number {
    use super::*;

    #[test]
    fn returns_correct_model() {
        let serial = SerialNumber::from("3040EAC356CCEEE8");
        assert_eq!(serial.as_model(), Some(SupportedModels::A3040));
    }

    #[test]
    fn handles_unsupported_model() {
        let serial = SerialNumber::from("3333EAC356CCEEE8");
        assert_eq!(serial.as_model(), None);
    }

    #[test]
    fn handles_invalid_serial() {
        let serial = SerialNumber::default();
        assert_eq!(serial.as_model(), None);
    }
}