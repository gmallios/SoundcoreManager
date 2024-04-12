use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
pub struct HearingProtect {
    pub enabled: bool,
    pub db: u8,
    pub freq: u8,
}
