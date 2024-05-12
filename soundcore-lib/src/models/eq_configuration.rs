use super::{EQProfile, MonoEQ, StereoEQ};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

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
            EQConfiguration::Stereo(config) => config.profile = profile,
            EQConfiguration::Mono(config) => config.profile = profile,
        }
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
