use std::sync::Arc;

use crate::api::{DeviceFeatureSet, EqualizerFeatures, FeatureFlags, SoundModeFeatures};

pub fn a3951_features() -> DeviceFeatureSet {
    DeviceFeatureSet {
        sound_mode_features: Some(
            SoundModeFeatures::scene_based_customizable_anc_non_customizable_transparency(),
        ),
        equalizer_features: Some(EqualizerFeatures {
            bands: 8,
            channels: 2,
        }),
        flags: Arc::new([
            FeatureFlags::CUSTOM_BUTTONS,
            FeatureFlags::DRC,
            FeatureFlags::HEARID,
            FeatureFlags::TOUCH_TONE,
            FeatureFlags::WEAR_DETECTION,
        ]),
    }
}
