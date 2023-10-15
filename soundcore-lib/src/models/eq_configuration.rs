use super::{EQProfile, MonoEQ, StereoEQ};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum EQConfiguration {
    Stereo(StereoEQConfiguration),
    Mono(MonoEQConfiguration),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct StereoEQConfiguration {
    pub eq: StereoEQ,
    pub profile: EQProfile,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct MonoEQConfiguration {
    pub eq: MonoEQ,
    pub profile: EQProfile,
}
