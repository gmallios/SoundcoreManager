use std::sync::Arc;

use crate::api::{DeviceFeatureSet, EqualizerFeatures, FeatureFlags, SoundModeFeatures};

pub fn a3040_features() -> DeviceFeatureSet {
    DeviceFeatureSet {
        sound_mode_features: Some(
            SoundModeFeatures::adaptive_customizable_anc_customizable_transparency(),
        ),
        equalizer_features: Some(EqualizerFeatures {
            bands: 8,
            channels: 2,
        }),
        flags: Arc::new([
            FeatureFlags::CUSTOM_BUTTONS,
            FeatureFlags::AUTO_POWER_OFF_ON,
            FeatureFlags::POWER_ON_BATTERY_NOTICE,
            FeatureFlags::MULTIPLE_DEVICE_LIST,
            FeatureFlags::HEARING_PROTECTION,
            FeatureFlags::AMBIENT_SOUND_NOTICE,
        ]),
    }
}
