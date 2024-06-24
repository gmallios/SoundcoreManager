use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
#[typeshare]
pub struct EqualizerFeatures {
    pub bands: u8,
    pub channels: u8,
    pub has_bass_up: bool, // We want to hide Bass Booster EQ Profile is this is true
}
