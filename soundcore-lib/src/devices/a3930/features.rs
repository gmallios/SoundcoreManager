use std::sync::Arc;

use crate::api::{DeviceFeatureSet, EqualizerFeatures, SoundModeFeatures};

pub fn a3930_features() -> DeviceFeatureSet {
    DeviceFeatureSet {
        // A3030 Seems to have no sound modes
        sound_mode_features: Some(SoundModeFeatures::new(&[], &[], true)),
        equalizer_features: Some(EqualizerFeatures {
            bands: 8,
            channels: 1,
            has_bass_up: false,
        }),
        flags: Arc::new([]),
    }
}
