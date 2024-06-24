use std::sync::Arc;

use crate::api::{DeviceFeatureSet, EqualizerFeatures, SoundModeFeatures};

pub fn a3028_features() -> DeviceFeatureSet {
    DeviceFeatureSet {
        sound_mode_features: Some(
            SoundModeFeatures::scene_based_non_customizable_anc_non_customizable_transparency(),
        ),
        equalizer_features: Some(EqualizerFeatures {
            bands: 8,
            channels: 1,
            has_bass_up: false
        }),
        flags: Arc::new([]),
    }
}
