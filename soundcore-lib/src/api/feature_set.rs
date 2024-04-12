use std::sync::Arc;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

mod equalizer_features;
mod flags;
mod sound_mode_features;

pub use equalizer_features::*;
pub use flags::*;
pub use sound_mode_features::*;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[typeshare]
#[serde(rename_all = "camelCase")]
pub struct DeviceFeatureSet {
    pub sound_mode_features: Option<SoundModeFeatures>,
    pub equalizer_features: Option<EqualizerFeatures>,
    pub flags: Arc<[FeatureFlags]>,
}

impl Default for DeviceFeatureSet {
    fn default() -> Self {
        Self {
            sound_mode_features: None,
            equalizer_features: None,
            flags: Arc::new([]),
        }
    }
}
