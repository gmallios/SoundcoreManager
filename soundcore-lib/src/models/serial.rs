use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct SerialNumber(pub Arc<str>);

impl SerialNumber {
    pub fn as_str(&self) -> &str {
        &self.0
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
