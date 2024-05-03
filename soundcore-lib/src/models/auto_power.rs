use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[serde(rename_all = "camelCase", tag = "type")]
#[typeshare]
pub struct AutoPowerOff {
    pub enabled: bool,
    pub index: u8, //TODO: Search for possible values and map to enum
}
