use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[typeshare]
pub struct EqualizerFeatures {
    pub bands: u8,
    pub channels: u8,
}
