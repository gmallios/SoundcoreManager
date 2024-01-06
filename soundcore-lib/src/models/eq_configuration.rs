

use super::{EQProfile, MonoEQ, StereoEQ};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum EQConfiguration {
    Stereo(StereoEQConfiguration),
    Mono(MonoEQConfiguration),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
pub struct StereoEQConfiguration {
    pub eq: StereoEQ,
    pub profile: EQProfile,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
pub struct MonoEQConfiguration {
    pub eq: MonoEQ,
    pub profile: EQProfile,
}

impl Default for EQConfiguration {
    fn default() -> Self {
        EQConfiguration::Stereo(StereoEQConfiguration::default())
    }
}

impl Into<EQConfiguration> for StereoEQConfiguration {
    fn into(self) -> EQConfiguration {
        EQConfiguration::Stereo(self)
    }
}

impl Into<EQConfiguration> for MonoEQConfiguration {
    fn into(self) -> EQConfiguration {
        EQConfiguration::Mono(self)
    }
}