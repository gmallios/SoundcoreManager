use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct AutoPowerOffOn {
    pub enabled: bool,
    pub index: u8, //TODO: Search for possible values and map to enum
}
