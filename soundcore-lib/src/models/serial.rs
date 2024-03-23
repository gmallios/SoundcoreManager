use std::sync::Arc;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::types::SupportedModels;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
#[typeshare]
pub struct SerialNumber {
    value: Arc<str>,
    model: Option<SupportedModels>,
}

impl SerialNumber {
    pub fn to_str(&self) -> &str {
        &self.value
    }

    pub fn to_model(&self) -> Option<SupportedModels> {
        self.model
    }

    fn extract_model(value: &str) -> Option<SupportedModels> {
        match value.get(0..4) {
            Some(v) => ("A".to_owned() + v).parse().ok(),
            None => None,
        }
    }
}

impl Default for SerialNumber {
    fn default() -> Self {
        Self {
            value: "XXXXXXXXXXXXXXXX".into(),
            model: None,
        }
    }
}

impl From<&str> for SerialNumber {
    fn from(s: &str) -> Self {
        SerialNumber {
            value: s.into(),
            model: SerialNumber::extract_model(s),
        }
    }
}

#[cfg(test)]
mod serial_number {
    use super::*;

    #[test]
    fn returns_correct_model() {
        let serial = SerialNumber::from("3040EAC356CCEEE8");
        assert_eq!(serial.to_model(), Some(SupportedModels::A3040));
    }

    #[test]
    fn handles_unsupported_model() {
        let serial = SerialNumber::from("3333EAC356CCEEE8");
        assert_eq!(serial.to_model(), None);
    }

    #[test]
    fn handles_invalid_serial() {
        let serial = SerialNumber::default();
        assert_eq!(serial.to_model(), None);
    }
}
