use std::sync::Arc;

use crate::api::{DeviceFeatureSet, EqualizerFeatures, FeatureFlags, SoundModeFeatures};

pub fn a3029_features() -> DeviceFeatureSet {
    DeviceFeatureSet {
        sound_mode_features: Some(
            SoundModeFeatures::scene_based_non_customizable_anc_non_customizable_transparency(),
        ),
        equalizer_features: Some(EqualizerFeatures {
            bands: 8,
            channels: 1,
        }),
        flags: Arc::new([]),
    }
}
