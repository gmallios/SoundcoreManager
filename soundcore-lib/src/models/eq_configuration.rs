use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{EQProfile, MonoEQ, StereoEQ};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
#[typeshare]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum EQConfiguration {
    Stereo(StereoEQConfiguration),
    Mono(MonoEQConfiguration),
}

impl EQConfiguration {
    pub fn set_profile(&mut self, profile: EQProfile) {
        match self {
            EQConfiguration::Stereo(config) => {
                config.profile = profile;
                config.eq.left = profile.eq();
                config.eq.right = profile.eq();
            },
            EQConfiguration::Mono(config) => {
                config.profile = profile;
                config.eq = profile.eq();
            },
        }
    }

    pub fn stereo_with_profile(profile: EQProfile) -> Self {
        EQConfiguration::Stereo(StereoEQConfiguration {
            eq: StereoEQ {
                left: profile.eq(),
                right: profile.eq(),
            },
            profile,
        })
    }

    pub fn mono_custom(eq: MonoEQ) -> Self {
        EQConfiguration::Mono(MonoEQConfiguration {
            eq,
            profile: EQProfile::Custom,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
#[typeshare]
pub struct StereoEQConfiguration {
    pub eq: StereoEQ,
    pub profile: EQProfile,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
#[typeshare]
pub struct MonoEQConfiguration {
    pub eq: MonoEQ,
    pub profile: EQProfile,
}

impl Default for EQConfiguration {
    fn default() -> Self {
        EQConfiguration::Stereo(StereoEQConfiguration::default())
    }
}

impl From<StereoEQConfiguration> for EQConfiguration {
    fn from(config: StereoEQConfiguration) -> Self {
        EQConfiguration::Stereo(config)
    }
}

impl From<MonoEQConfiguration> for EQConfiguration {
    fn from(config: MonoEQConfiguration) -> Self {
        EQConfiguration::Mono(config)
    }
}

impl From<MonoEQConfiguration> for StereoEQConfiguration {
    fn from(config: MonoEQConfiguration) -> Self {
        Self {
            eq: config.eq.into(),
            profile: config.profile,
        }
    }
}

impl From<StereoEQConfiguration> for MonoEQConfiguration {
    fn from(config: StereoEQConfiguration) -> Self {
        Self {
            eq: config.eq.into(),
            profile: config.profile,
        }
    }
}

impl From<EQConfiguration> for StereoEQConfiguration {
    fn from(config: EQConfiguration) -> Self {
        match config {
            EQConfiguration::Stereo(config) => config,
            EQConfiguration::Mono(config) => config.into(),
        }
    }
}

impl From<EQConfiguration> for MonoEQConfiguration {
    fn from(config: EQConfiguration) -> Self {
        match config {
            EQConfiguration::Stereo(config) => config.into(),
            EQConfiguration::Mono(config) => config,
        }
    }
}
