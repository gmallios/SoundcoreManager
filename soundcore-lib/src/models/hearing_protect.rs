use derive_more::From;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(
    Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default, Hash, From,
)]
#[typeshare]
pub struct HearingProtect {
    pub enabled: bool,
    pub db: u8,
    pub freq: u8,
}
