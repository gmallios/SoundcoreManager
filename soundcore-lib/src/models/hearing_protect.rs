use serde::{Serialize, Deserialize};
use derive_more::From;

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct HearingProtect {
    pub enabled: bool,
    pub db: u8,
    pub freq: u8,
}